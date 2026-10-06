//! The gate: from a boundary recording alone, rebuild every snapshot the
//! Python engine's `evaluate` saw, key for key and in its order.
//!
//! The driver is the library's, [`rebuild_snapshots`], the one the
//! differential job runs over live recordings; its module says how it
//! drives the reader. Here it runs over the committed synthetic
//! recordings, which answer every repeated read alike, except the failures
//! they stage, which come first.

use crate::support::*;
use pr_hygiene_engine::conformance::{rebuild_snapshots, Failure, Outcome, Recording};
use std::path::{Path, PathBuf};

fn conformance() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance")
}

/// Every directory under `root` that holds a recording, in a stable order.
fn recordings_under(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return found;
    };
    let mut children: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    children.sort();
    for child in children {
        if child.join("recording.json").is_file() {
            found.push(child);
        } else if child.is_dir() {
            found.extend(recordings_under(&child));
        }
    }
    found
}

fn load(dir: &Path) -> Recording {
    Recording::load_with(|name| std::fs::read_to_string(dir.join(name)))
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
}

/// Rebuild every evaluated snapshot of the recording in `dir`. Returns how
/// many were rebuilt, or every one that was not, with what stopped it.
fn rebuild(dir: &Path, recording: &Recording) -> Result<usize, String> {
    let name = dir.display();
    let (checks, missing) = rebuild_snapshots(recording);
    let mut problems = Vec::new();
    for check in &checks {
        let index = check.index;
        match &check.outcome {
            Outcome::Matched => {}
            Outcome::Differs(found) => {
                let at: Vec<String> = found
                    .iter()
                    .map(|d| format!("{} ({})", d.path, d.kind))
                    .collect();
                problems.push(format!(
                    "{name}: evaluation {index} differs at {}",
                    at.join(", ")
                ));
            }
            Outcome::Failed { failure, detail } => {
                problems.push(format!("{name}: evaluation {index}: {failure}: {detail}"))
            }
            // Only a live read explains a difference or sees a move.
            Outcome::Explained { .. }
            | Outcome::Moved
            | Outcome::Unexplained { .. }
            | Outcome::NotCompared { .. } => problems.push(format!(
                "{name}: evaluation {index}: a live outcome from a recording"
            )),
        }
    }
    if missing > 0 && problems.is_empty() {
        problems.push(format!("{name}: {missing} read(s) not in the recording"));
    }
    if problems.is_empty() {
        Ok(checks.len())
    } else {
        Err(problems.join("\n"))
    }
}

/// Rebuild every recording under `root`; the number of snapshots rebuilt.
fn rebuild_all(root: &Path, refuse_unredacted: bool) -> (usize, usize) {
    let dirs = recordings_under(root);
    let mut failures = Vec::new();
    let mut snapshots = 0;
    for dir in &dirs {
        let recording = load(dir);
        if refuse_unredacted && !recording.redacted() {
            failures.push(format!(
                "{}: an unredacted recording; redact it before it is read here (conformance/README.md)",
                dir.display()
            ));
            continue;
        }
        match rebuild(dir, &recording) {
            Ok(count) => snapshots += count,
            Err(failure) => failures.push(failure),
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
    (dirs.len(), snapshots)
}

#[test]
fn every_snapshot_python_evaluated_is_rebuilt_from_a_synthetic_recording() {
    let (recordings, snapshots) = rebuild_all(&conformance().join("synthetic"), false);
    assert!(
        recordings >= 7,
        "the committed synthetic recordings are found ({recordings})"
    );
    assert!(snapshots >= 10, "{snapshots} snapshots rebuilt");
}

#[test]
#[ignore = "reads conformance/live, which only a local redacted recording fills"]
fn every_snapshot_python_evaluated_is_rebuilt_from_a_live_recording() {
    let (recordings, snapshots) = rebuild_all(&conformance().join("live"), true);
    eprintln!("{recordings} live recording(s), {snapshots} snapshot(s) rebuilt");
}

#[test]
fn a_changed_answer_from_github_changes_the_rebuilt_snapshot() {
    // A gate that cannot fail proves nothing: the same recording with one
    // review withdrawn must no longer give the snapshot Python evaluated.
    let dir = conformance().join("synthetic/report");
    let mut recording = load(&dir);
    assert!(recording.calls.contains(r#"\"state\": \"APPROVED\""#));
    recording.calls = recording
        .calls
        .replace(r#"\"state\": \"APPROVED\""#, r#"\"state\": \"DISMISSED\""#);
    let failure = rebuild(&dir, &recording).unwrap_err();
    assert!(
        failure.contains("differs at pr.reviews[1].state (value)"),
        "{failure}"
    );
}

#[test]
fn the_same_entries_in_another_order_are_another_snapshot() {
    // The snapshot is held to Python's key order, not only its entries:
    // move one key of one comment to the end and the gate must notice.
    let dir = conformance().join("synthetic/report");
    let mut recording = load(&dir);
    let PyValue::Dict(evaluation) = &mut recording.evaluations[0] else {
        panic!("an evaluation is a dict")
    };
    let Some(PyValue::Dict(pr)) = evaluation.get_mut("pr") else {
        panic!("an evaluation holds its pr")
    };
    let Some(PyValue::List(comments)) = pr.get_mut("comments") else {
        panic!("a pr holds its comments")
    };
    let Some(PyValue::Dict(comment)) = comments.first_mut() else {
        panic!("a comment")
    };
    let user = comment.shift_remove("user").expect("a comment's user");
    comment.insert("user".into(), user);
    let failure = rebuild(&dir, &recording).unwrap_err();
    assert!(
        failure.contains("differs at pr.comments[0] (key order)"),
        "{failure}"
    );
}

#[test]
fn a_read_the_recording_lacks_is_refused_by_name() {
    let dir = conformance().join("synthetic/report");
    let mut recording = load(&dir);
    recording.calls = recording
        .calls
        .split_inclusive('\n')
        .filter(|line| !line.contains("/reviews?per_page=100"))
        .collect();
    let failure = rebuild(&dir, &recording).unwrap_err();
    assert!(
        failure.contains(
            "A read the recording does not hold: gh api --method GET repos/dashpay/platform/pulls/1/reviews?per_page=100 --paginate --slurp"
        ),
        "{failure}"
    );
    // Counted as a missing read, and named as one, without the message.
    let (checks, missing) = rebuild_snapshots(&recording);
    assert_eq!(missing, 2, "both pull requests' reviews are unrecorded");
    assert!(checks.iter().all(|check| matches!(
        check.outcome,
        Outcome::Failed {
            failure: Failure::ReadNotRecorded,
            ..
        }
    )));
}

#[test]
fn a_read_asked_more_often_than_recorded_is_told_from_one_never_recorded() {
    // The report's first snapshot reads pull request 1; asked a second time
    // after the caches are dropped, the recording has no answer left.
    let dir = conformance().join("synthetic/report");
    let mut recording = load(&dir);
    let first = recording.evaluations[0].clone();
    recording.evaluations.push(first);
    let (checks, missing) = rebuild_snapshots(&recording);
    assert_eq!(missing, 1);
    assert!(checks[..2].iter().all(|check| check.matched()));
    assert!(matches!(
        checks[2].outcome,
        Outcome::Failed {
            failure: Failure::ReadAskedAgain,
            ..
        }
    ));
}

#[test]
fn the_synthetic_recordings_walk_the_read_paths_they_are_named_for() {
    // Each recording is named for a path the reader must take; a generator
    // change that stopped walking it would leave the gate passing over
    // nothing.
    let calls = |name: &str| load(&conformance().join("synthetic").join(name)).calls;
    assert!(calls("long-history").contains("comments(first:100, after:$after)"));
    let edited = load(&conformance().join("synthetic/edited-record"));
    assert!(edited.calls.contains(r#"\"login\": \"mallory\""#));
    assert!(matches!(
        field(field(&edited.evaluations[0], "pr"), "controller_state"),
        PyValue::None
    ));
    let partial = calls("partial-answer");
    assert!(partial.contains(r#""exit":1"#) && partial.contains("NOT_FOUND"));
    let transient = calls("transient-retry");
    assert!(transient.contains("HTTP 502") && transient.contains("i/o timeout"));
    assert!(transient.contains(r#""raised":"TimeoutExpired""#));
    let rich = load(&conformance().join("synthetic/rich-evidence"));
    assert!(
        rich.evaluations.len() > rich.verdicts.len(),
        "a snapshot taken without the batched history, after the caches were dropped"
    );
}
