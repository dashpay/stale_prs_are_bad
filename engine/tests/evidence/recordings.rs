//! The gate: from a boundary recording alone, rebuild every snapshot the
//! Python engine's `evaluate` saw, key for key and in its order.
//!
//! A recording keeps every `gh api` call Python made, and `evaluations.jsonl`
//! keeps the `pr` each `evaluate` call was given. The reader is driven over
//! a [`ReplayTransport`] as Python's run drove its own: one reader for the
//! run, with its caches; the batched history read that admission made, its
//! pull requests named by the recorded query itself; a snapshot for each
//! evaluation, the first ones (as many as there are verdicts) over that
//! history, and each later one — the last check before a write — after the
//! caches are dropped and without it. Every read must be one the recording
//! holds, asked no more often than Python asked it.
//!
//! What Python read between those snapshots and did not evaluate — the
//! re-check before a write, the admission re-read — is not asked here, so a
//! later snapshot may be answered with a recorded answer Python's earlier
//! read got. Where the same read was answered differently within one run,
//! as GitHub changing under a live run can do, a difference here is that
//! and not necessarily the port; the synthetic recordings answer every
//! repeated read alike, except the failures they stage, which come first.

use crate::support::*;
use pr_hygiene_engine::evidence::ReplayTransport;
use pr_hygiene_engine::pycompat::text::py_repr_str;
use regex::Regex;
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

struct Recording {
    meta: PyValue,
    calls: String,
    evaluations: Vec<PyValue>,
    verdicts: usize,
}

fn read(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn load(dir: &Path) -> Recording {
    let meta = py_loads(&read(dir, "recording.json")).expect("recording.json is JSON");
    let evaluations = read(dir, "evaluations.jsonl")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| py_loads(line).expect("an evaluation is JSON"))
        .collect();
    let verdicts =
        items(&py_loads(&read(dir, "verdicts.json")).expect("verdicts.json is JSON")).len();
    Recording {
        meta,
        calls: read(dir, "calls.jsonl"),
        evaluations,
        verdicts,
    }
}

/// The pull requests the first batched history query named: the one
/// admission made, whose answers the first snapshots reuse.
fn collected(calls: &str) -> Vec<PyInt> {
    let alias =
        Regex::new(r"pr([0-9]+): pullRequest\(number:([0-9]+)\)").expect("a constant pattern");
    for line in calls.split('\n').filter(|line| !line.is_empty()) {
        let entry = py_loads(line).expect("a call is JSON");
        let (PyValue::Str(kind), PyValue::Str(stdin)) =
            (field(&entry, "kind"), field(&entry, "stdin"))
        else {
            continue;
        };
        if kind != "read" {
            continue;
        }
        let Ok(document) = py_loads(stdin) else {
            continue;
        };
        let PyValue::Dict(document) = document else {
            continue;
        };
        let Some(PyValue::Str(query)) = document.get("query") else {
            continue;
        };
        if !query.contains("fragment history") {
            continue;
        }
        return alias
            .captures_iter(query)
            .map(|found| PyInt::from_decimal(&found[2]).expect("digits"))
            .collect();
    }
    Vec::new()
}

/// Where two values first differ, in reading order: a key out of place, a
/// key missing, a value unlike the other.
fn first_difference(read: &PyValue, wanted: &PyValue, at: &str) -> Option<String> {
    match (read, wanted) {
        (PyValue::Dict(a), PyValue::Dict(b)) => {
            let (keys_a, keys_b): (Vec<_>, Vec<_>) = (a.keys().collect(), b.keys().collect());
            if keys_a != keys_b {
                return Some(format!("{at}: keys {keys_a:?}, Python's {keys_b:?}"));
            }
            a.iter()
                .find_map(|(key, value)| first_difference(value, &b[key], &format!("{at}.{key}")))
        }
        (PyValue::List(a), PyValue::List(b)) => {
            if a.len() != b.len() {
                return Some(format!("{at}: {} items, Python's {}", a.len(), b.len()));
            }
            a.iter()
                .zip(b.iter())
                .enumerate()
                .find_map(|(i, (x, y))| first_difference(x, y, &format!("{at}[{i}]")))
        }
        _ => {
            let (x, y) = (shown(read), shown(wanted));
            (x != y).then(|| format!("{at}: {}, Python's {}", py_repr_str(&x), py_repr_str(&y)))
        }
    }
}

/// Rebuild every evaluated snapshot of the recording in `dir`. Returns how
/// many were rebuilt, or what went wrong first.
fn rebuild(dir: &Path, recording: &Recording) -> Result<usize, String> {
    let name = dir.display();
    let transport =
        ReplayTransport::from_calls_jsonl(&recording.calls).map_err(|e| format!("{name}: {e}"))?;
    let repository = text(field(&recording.meta, "repository"));
    let mut api = GitHub::new(repository, Client::with_sleep(transport, NoSleep))
        .map_err(|e| format!("{name}: {e}"))?;
    let numbers = collected(&recording.calls);
    let histories = if numbers.is_empty() {
        Default::default()
    } else {
        api.histories(&numbers)
            .map_err(|e| format!("{name}: the batched history read: {e}"))?
    };
    for (index, evaluation) in recording.evaluations.iter().enumerate() {
        let wanted = field(evaluation, "pr");
        let PyValue::Dict(entries) = evaluation else {
            unreachable!()
        };
        let policy = entries
            .get("policy")
            .unwrap_or_else(|| field(&recording.meta, "policy"));
        let PyValue::Int(number) = field(wanted, "number") else {
            return Err(format!(
                "{name}: evaluation {index} has no pull request number"
            ));
        };
        let read = if index < recording.verdicts {
            api.snapshot(number, policy, histories.get(number))
        } else {
            api.forget_cached_access();
            api.snapshot(number, policy, None)
        };
        let read = read.map_err(|e| format!("{name}: evaluation {index} (#{number}): {e}"))?;
        let compact =
            |value: &PyValue| py_dumps(value, false, Some((",", ":")), None).expect("no floats");
        if compact(&read) != compact(wanted) {
            let at = first_difference(&read, wanted, "pr").unwrap_or_else(|| "pr".into());
            return Err(format!(
                "{name}: evaluation {index} (#{number}) differs at {at}"
            ));
        }
    }
    Ok(recording.evaluations.len())
}

/// Rebuild every recording under `root`; the number of snapshots rebuilt.
fn rebuild_all(root: &Path, refuse_unredacted: bool) -> (usize, usize) {
    let dirs = recordings_under(root);
    let mut failures = Vec::new();
    let mut snapshots = 0;
    for dir in &dirs {
        let recording = load(dir);
        if refuse_unredacted && matches!(field(&recording.meta, "redacted"), PyValue::None) {
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
    assert!(failure.contains("differs at pr.reviews"), "{failure}");
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
        rich.evaluations.len() > rich.verdicts,
        "a snapshot taken without the batched history, after the caches were dropped"
    );
}
