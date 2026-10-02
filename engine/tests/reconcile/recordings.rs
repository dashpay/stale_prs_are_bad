//! The gate: a whole recorded run of Python's engine, replayed through the
//! port from its reads alone, must come out the same — the same verdicts,
//! the same writes in the same order with the same bodies, the same text
//! for every mark a pull request would carry, the same JSON report, the
//! same outcome, and the clock read at the same places.
//!
//! The driver is the library's, [`replay_run`], beside the snapshot driver
//! the evidence gate and the differential job share; its module says how it
//! drives the run. Here it runs over the committed synthetic recordings.

use crate::support::*;
use pr_hygiene_engine::conformance::{
    replay_run, Comparison, Layer, Outcome, OwnWords, Recording, RunFiles,
};
use std::path::{Path, PathBuf};

/// Every directory under `root` holding a recording, in a stable order.
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

/// A recording and what its run put out.
struct Whole {
    recording: Recording,
    files: RunFiles,
}

fn load(dir: &Path) -> Whole {
    let read = |name: &str| std::fs::read_to_string(dir.join(name));
    Whole {
        recording: Recording::load_with(read).unwrap_or_else(|e| panic!("{}: {e}", dir.display())),
        files: RunFiles::load_with(read).unwrap_or_else(|e| panic!("{}: {e}", dir.display())),
    }
}

fn own_words() -> OwnWords {
    let source = |name: &str| {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../pr_review")
            .join(name);
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    };
    OwnWords::from_sources(&source("conformance.py"), &source("policy.py"))
        .expect("the engine's own words are found")
}

/// Every check that did not match, as `layer index: path (kind)` or
/// `layer index: failure: detail`, and any read the recording lacks.
fn found(comparison: &Comparison) -> Vec<String> {
    let mut found = Vec::new();
    for check in &comparison.checks {
        let at = format!("{} {}", check.layer.as_str(), check.index);
        match &check.outcome {
            Outcome::Matched => {}
            Outcome::Differs(differences) => {
                for d in differences {
                    found.push(format!("{at}: {} ({})", d.path, d.kind));
                }
            }
            Outcome::Failed { failure, detail } => found.push(format!("{at}: {failure}: {detail}")),
            // Only a live read explains a difference or sees a move.
            Outcome::Explained { .. } | Outcome::Moved => {
                found.push(format!("{at}: a live outcome from a recording"))
            }
        }
    }
    if comparison.missing_reads > 0 {
        found.push(format!(
            "{} read(s) not in the recording",
            comparison.missing_reads
        ));
    }
    found
}

fn compare(whole: &Whole) -> Vec<String> {
    found(&replay_run(&whole.recording, &whole.files, &own_words()))
}

fn synthetic(name: &str) -> PathBuf {
    conformance().join("synthetic").join(name)
}

#[test]
fn every_synthetic_recording_replays_to_the_same_run() {
    let dirs = recordings_under(&conformance().join("synthetic"));
    assert!(
        dirs.len() >= 13,
        "the committed synthetic recordings are found"
    );
    let mut failures = Vec::new();
    for dir in &dirs {
        let whole = load(dir);
        let comparison = replay_run(&whole.recording, &whole.files, &own_words());
        let differences = found(&comparison);
        if !differences.is_empty() {
            failures.push(format!(
                "{}:\n  {}",
                dir.display(),
                differences.join("\n  ")
            ));
        }
        // Every layer of a whole run was compared, and something was: every
        // write too, where the run wrote.
        let writes = whole.recording.calls.contains(r#""kind":"write""#);
        let layers = [Layer::Run, Layer::Clock, Layer::Call, Layer::Write];
        for layer in layers.into_iter().take(if writes { 4 } else { 3 }) {
            assert!(
                comparison.matched(layer).1 > 0,
                "{}: no {layer:?}",
                dir.display()
            );
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
#[ignore = "reads conformance/live, which only a local redacted recording fills"]
fn every_live_recording_replays_to_the_same_run() {
    let mut failures = Vec::new();
    let dirs = recordings_under(&conformance().join("live"));
    for dir in &dirs {
        let whole = load(dir);
        if !whole.recording.redacted() {
            failures.push(format!("{}: an unredacted recording", dir.display()));
            continue;
        }
        let differences = compare(&whole);
        if !differences.is_empty() {
            failures.push(format!(
                "{}:\n  {}",
                dir.display(),
                differences.join("\n  ")
            ));
        }
    }
    eprintln!("{} live recording(s)", dirs.len());
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// The differences the gate finds in `name` once `change` has been made to
/// it: a gate that cannot fail proves nothing.
fn tampered(name: &str, change: impl FnOnce(&mut Whole)) -> Vec<String> {
    let mut whole = load(&synthetic(name));
    change(&mut whole);
    compare(&whole)
}

#[test]
fn a_write_with_other_words_fails_the_gate() {
    let found = tampered("sync-pr-2", |w| {
        let calls = &mut w.recording.calls;
        assert!(calls.contains("Evaluating current review policy"));
        *calls = calls.replace("Evaluating current review policy", "Evaluating the policy");
    });
    // The write is named by its place and the field that differs, and the
    // run goes on to compare every write after it.
    assert_eq!(found, ["write 0: write.description (value)"]);
}

#[test]
fn a_write_python_did_not_make_fails_the_gate() {
    let found = tampered("sync-pr-2", |w| {
        // The last write, the status after the re-check, dropped.
        let mut lines: Vec<&str> = w.recording.calls.lines().collect();
        let last = lines
            .iter()
            .rposition(|line| line.contains(r#""kind":"write""#))
            .expect("a write");
        lines.remove(last);
        w.recording.calls = lines.iter().map(|line| format!("{line}\n")).collect();
    });
    assert!(
        found
            .iter()
            .any(|d| d
                == "write 5: write the recording does not hold: made after every recorded write"),
        "{found:?}"
    );
    assert!(
        found
            .iter()
            .any(|d| d.contains("A write the recording does not hold")),
        "{found:?}"
    );
}

#[test]
fn a_write_python_made_and_the_port_did_not_fails_the_gate() {
    let found = tampered("sync-pr-2", |w| {
        // One more write recorded after the last: the port never makes it.
        let last = w
            .recording
            .calls
            .lines()
            .rfind(|line| line.contains(r#""kind":"write""#))
            .expect("a write")
            .to_owned();
        w.recording
            .calls
            .push_str(&last.replacen(r#"{"ordinal":"#, r#"{"ordinal":9"#, 1));
        w.recording.calls.push('\n');
    });
    assert!(
        found
            .iter()
            .any(|d| d.starts_with("write 6: recorded write never made")),
        "{found:?}"
    );
}

#[test]
fn a_clock_read_somewhere_else_fails_the_gate() {
    let found = tampered("rich-evidence", |w| {
        let PyValue::Dict(meta) = &mut w.recording.meta else {
            panic!("a dict")
        };
        let Some(PyValue::List(reads)) = meta.get_mut("clock_reads") else {
            panic!("clock reads")
        };
        let Some(PyValue::Dict(last)) = reads.last_mut() else {
            panic!("a read")
        };
        last.insert("site".into(), PyValue::Str("run".into()));
    });
    assert!(
        found
            .iter()
            .any(|d| d.starts_with("clock 0: clock_reads[2][0]")),
        "{found:?}"
    );
}

#[test]
fn a_verdict_or_an_output_python_did_not_reach_fails_the_gate() {
    let found = tampered("report", |w| {
        let PyValue::Dict(row) = &mut w.recording.verdicts[0] else {
            panic!("a row")
        };
        row.insert(
            "ready_since".into(),
            PyValue::Str("2026-01-01T00:00:00Z".into()),
        );
        let PyValue::Dict(output) = &mut w.files.outputs[1] else {
            panic!("an output")
        };
        output.insert("move".into(), PyValue::None);
    });
    assert!(
        found
            .iter()
            .any(|d| d.starts_with("run verdict 0: verdict.ready_since")),
        "{found:?}"
    );
    assert!(
        found.iter().any(|d| d.starts_with("output 1: output.move")),
        "{found:?}"
    );
}

#[test]
fn a_printed_report_python_did_not_print_fails_the_gate() {
    let found = tampered("report-json", |w| {
        let printed = &mut w.files.printed;
        assert!(printed.contains(r#""author": "reviewer""#));
        *printed = printed.replace(r#""author": "reviewer""#, r#""author": "someone""#);
    });
    assert!(
        found
            .iter()
            .any(|d| d.starts_with("report 0: report.pull_requests[0].author")),
        "{found:?}"
    );
}

#[test]
fn calls_made_in_another_order_fail_the_gate() {
    // The first two calls of the report, recorded the other way round.
    let found = tampered("report", |w| {
        w.recording.calls = w
            .recording
            .calls
            .replacen(r#"{"ordinal":1,"#, r#"{"ordinal":0,"#, 1)
            .replacen(r#"{"ordinal":2,"#, r#"{"ordinal":1,"#, 1)
            .replacen(r#"{"ordinal":0,"#, r#"{"ordinal":2,"#, 1);
    });
    assert!(
        found.iter().any(|d| d.starts_with("call 0: calls[0]")),
        "{found:?}"
    );
}

#[test]
fn the_synthetic_recordings_walk_the_write_paths_they_are_named_for() {
    // Each recording is named for a path the engine must take; a generator
    // change that stopped walking it would leave the gate passing over
    // nothing.
    let writes = |name: &str| -> Vec<String> {
        load(&synthetic(name))
            .recording
            .calls
            .lines()
            .filter(|line| line.contains(r#""kind":"write""#))
            .map(str::to_owned)
            .collect()
    };
    let routed = |name: &str, route: &str| writes(name).iter().any(|w| w.contains(route));
    assert!(routed(
        "sync-pr-2",
        r#""POST","repos/dashpay/platform/issues/2/comments""#
    ));
    assert!(routed("sync-pr-2", "requested_reviewers"));
    assert!(routed(
        "rich-evidence",
        r#""DELETE","repos/dashpay/platform/issues/2/labels/waiting-bots""#
    ));
    assert!(routed(
        "rich-evidence",
        r#""DELETE","repos/dashpay/platform/issues/comments/200""#
    ));
    // A full pass: the unreadable head marked alone, a draft's block taken
    // out, two records set aside and their labels cleared.
    for route in [
        r#""POST","repos/dashpay/platform/statuses/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa""#,
        r#""PATCH","repos/dashpay/platform/pulls/5""#,
        r#""PATCH","repos/dashpay/platform/issues/comments/300""#,
        r#""DELETE","repos/dashpay/platform/issues/3/labels/waiting-bots""#,
        r#""PATCH","repos/dashpay/platform/pulls/3""#,
        r#""PATCH","repos/dashpay/platform/issues/comments/400""#,
        r#""DELETE","repos/dashpay/platform/issues/4/labels/ready-to-merge""#,
    ] {
        assert!(routed("sweep", route), "sweep: {route}");
    }
    assert!(load(&synthetic("sweep"))
        .recording
        .calls
        .contains("repos/dashpay/platform/issues/4/comments?per_page=100"));
    // The hourly batch, chosen from the clock; an announcement edited in
    // place; the success published.
    let batch = load(&synthetic("batch"));
    assert!(dump(field(&batch.recording.meta, "clock_reads")).contains("periodic_batch"));
    assert!(routed(
        "batch",
        r#""PATCH","repos/dashpay/platform/issues/comments/150""#
    ));
    assert!(writes("batch")
        .last()
        .is_some_and(|w| w.contains(r#"\"state\": \"success\""#)));
    // A nudge, with the status page read once.
    let nudge = load(&synthetic("nudge"));
    assert_eq!(dump(field(&nudge.recording.meta, "telemetry_reads")), "1");
    assert!(writes("nudge")[0].contains("pr-hygiene-nudge v1 bot=coderabbitai"));
    // A publication that fails, and a policy that does not validate.
    for (name, raised, status) in [
        (
            "failing-publish",
            "GitHubError",
            "Policy reconciliation failed",
        ),
        ("config-error", "ValueError", "Invalid policy configuration"),
    ] {
        let whole = load(&synthetic(name));
        assert_eq!(
            text(field(field(&whole.recording.meta, "outcome"), "raised")),
            raised
        );
        assert!(
            writes(name).last().is_some_and(|w| w.contains(status)),
            "{name}"
        );
    }
    // The report, filtered to one person's pull requests.
    let report = py_loads(&load(&synthetic("report-json")).files.printed).expect("a JSON report");
    assert_eq!(items(field(&report, "pull_requests")).len(), 1);
}
