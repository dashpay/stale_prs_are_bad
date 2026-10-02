//! The gate: a whole recorded run of Python's engine, replayed through the
//! port from its reads alone, must come out the same — the same verdicts,
//! the same writes in the same order with the same bodies, the same text
//! for every mark a pull request would carry, the same JSON report, the
//! same outcome, and the clock read at the same places.
//!
//! The run is driven as `pr_review.conformance` replays one: the recording's
//! policy, its clock, its status page, its `GITHUB_RUN_ID`, and the dry
//! sync walking the write path (`apply`), every write answered as the
//! recorder answered it. What Python's command line decides from its own
//! arguments is decided here from the recording's `argv`: the hourly batch
//! (`periodic_batch`, read from the clock as Python reads it) and the
//! nudge allowance of one per run.
//!
//! The port makes Python's calls in Python's order, so this holds it to
//! more than the contract asks: every recorded call made, each once, in
//! the order recorded.

use crate::support::*;
use pr_hygiene_engine::evidence::records::{state_comment_body, validate_state};
use pr_hygiene_engine::evidence::ReplayTransport;
use pr_hygiene_engine::policy::{
    diff_print, machine_author, receipt_print, validate_policy, LABEL_FOR_STATE,
};
use pr_hygiene_engine::pycompat::text::py_slice;
use pr_hygiene_engine::pycompat::PyDict;
use pr_hygiene_engine::reconcile::{
    checklist_block, context_fingerprint, diff_record, move_state, move_text, state_record,
    Command, Reconciler, Run, RunOptions, Selection, POINTER, WAIVED_LABEL,
};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Every directory under `root` holding a recording, in a stable order.
pub fn recordings_under(root: &Path) -> Vec<PathBuf> {
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

/// A recording, as `conformance.load_recording` reads one.
pub struct Recording {
    pub meta: PyValue,
    pub calls: String,
    pub verdicts: PyValue,
    pub outputs: PyValue,
    pub printed: String,
}

fn read(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

pub fn load(dir: &Path) -> Recording {
    let parse = |name: &str| {
        py_loads(&read(dir, name)).unwrap_or_else(|e| panic!("{}/{name}: {e}", dir.display()))
    };
    Recording {
        meta: parse("recording.json"),
        calls: read(dir, "calls.jsonl"),
        verdicts: parse("verdicts.json"),
        outputs: parse("outputs.json"),
        printed: read(dir, "printed.txt"),
    }
}

/// What the recorded command line asked for.
struct Argv {
    command: Command,
    pr: Option<PyInt>,
    batch_size: Option<usize>,
    user: Option<String>,
    json: bool,
}

fn argv(meta: &PyValue) -> Result<Argv, String> {
    let words: Vec<&str> = items(field(meta, "argv")).iter().map(text).collect();
    let command = match words.first() {
        Some(&"sync") => Command::Sync,
        Some(&"report") => Command::Report,
        other => return Err(format!("a command the port does not run: {other:?}")),
    };
    let mut parsed = Argv {
        command,
        pr: None,
        batch_size: None,
        user: None,
        json: false,
    };
    let mut rest = words[1..].iter();
    while let Some(word) = rest.next() {
        let mut value = || {
            rest.next()
                .copied()
                .ok_or(format!("{word} without a value"))
        };
        match *word {
            "--repo" => {
                value()?;
            }
            "--pr" => parsed.pr = Some(PyInt::from_decimal(value()?).ok_or("--pr")?),
            "--batch-size" => {
                parsed.batch_size = Some(value()?.parse().map_err(|_| "--batch-size")?)
            }
            "--user" => parsed.user = Some(value()?.to_owned()),
            "--format" => parsed.json = value()? == "json",
            other => return Err(format!("an argument the port does not take: {other}")),
        }
    }
    Ok(parsed)
}

/// `main.periodic_batch(prs, size)`: a slice of the open pull requests
/// rotating with the hour, read from the clock as Python reads it — only
/// when there is something to choose from.
fn periodic_batch(prs: &[PyValue], size: usize, clock: &LoggingClock) -> Vec<PyValue> {
    let mut ordered = prs.to_vec();
    ordered.sort_by_key(|pr| match field(pr, "number") {
        PyValue::Int(n) => n.as_i64().expect("a small number"),
        _ => panic!("a number"),
    });
    if ordered.is_empty() {
        return Vec::new();
    }
    clock.log("periodic_batch");
    let seconds = clock.at.timestamp().expect("an aware clock");
    let hour = (seconds / 3600.0).floor() as i64;
    let start = (hour * size as i64).rem_euclid(ordered.len() as i64) as usize;
    (0..size.min(ordered.len()))
        .map(|offset| ordered[(start + offset) % ordered.len()].clone())
        .collect()
}

/// `conformance.outputs_for(engine, policy, context, pr, result)`: the
/// exact text this verdict puts on GitHub, as `publish` would write it.
fn outputs_for(policy: &PyValue, context: &[PyValue], pr: &PyValue, result: &PyValue) -> PyValue {
    let state = field(result, "state");
    let context_print = context_fingerprint(context, field(pr, "author")).expect("a context");
    let record = state_record(pr, result, &context_print).expect("a record");
    let carried = diff_record(pr, result).expect("a diff record");
    let mv = move_state(state).expect("a state");
    let move_body = match mv {
        Some(mv)
            if !(machine_author(policy, pr).expect("an author") && mv == "waiting-self-review") =>
        {
            move_text(result).expect("move text")
        }
        _ => None,
    };
    let (comment, refused) = match validate_state(&record).and_then(|()| {
        state_comment_body(
            &record,
            move_body.as_deref().unwrap_or(POINTER),
            carried.as_ref(),
        )
    }) {
        Ok(comment) => (Some(comment), None),
        Err(ReadError::GitHub(why)) => (None, Some(why)),
        Err(other) => panic!("outputs: {other}"),
    };
    let markers = comment
        .as_deref()
        .map(|c| c.split_once("\n\n").map_or(c, |(m, _)| m).to_owned());
    let group = |pattern: &str| {
        let pattern = Regex::new(pattern).expect("a pattern");
        markers
            .as_deref()
            .and_then(|m| pattern.captures(m))
            .and_then(|found| found.get(1))
            .map_or(PyValue::None, |g| PyValue::Str(g.as_str().into()))
    };
    let mut labels: Vec<&str> = LABEL_FOR_STATE
        .iter()
        .filter(|(s, _)| matches!(state, PyValue::Str(t) if t == s))
        .map(|(_, label)| *label)
        .collect();
    if pr_hygiene_engine::pycompat::object::get(result, "waived")
        .expect("a dict")
        .is_some_and(PyValue::truthy)
    {
        labels.push(WAIVED_LABEL);
    }
    labels.sort();
    labels.dedup();
    let mut receipts = PyDict::new();
    let comments = match pr_hygiene_engine::pycompat::object::get(pr, "comments").expect("a dict") {
        Some(PyValue::List(comments)) => comments.to_vec(),
        _ => Vec::new(),
    };
    for comment in &comments {
        if let Some(said) = receipt_print(comment).expect("a receipt print") {
            receipts.insert(dump_key(field(comment, "id")), PyValue::Str(said));
        }
    }
    let state_text = text(state);
    let checklist = if state_text == "draft" {
        PyValue::None
    } else {
        checklist_block(result)
            .expect("a checklist")
            .map_or(PyValue::None, PyValue::Str)
    };
    let mut status = PyDict::new();
    status.insert("state".into(), field(result, "status").clone());
    status.insert("context".into(), PyValue::Str("PR Hygiene".into()));
    status.insert(
        "description".into(),
        PyValue::Str(py_slice(state_text, None, Some(140)).into()),
    );
    let optional = |text: Option<String>| text.map_or(PyValue::None, PyValue::Str);
    let mut out = PyDict::new();
    out.insert("number".into(), field(pr, "number").clone());
    out.insert("head".into(), field(pr, "head").clone());
    out.insert("status".into(), PyValue::Dict(status));
    out.insert(
        "labels".into(),
        PyValue::List(labels.iter().map(|l| PyValue::Str((*l).into())).collect()),
    );
    out.insert("checklist".into(), checklist);
    out.insert("move".into(), optional(move_body.clone()));
    out.insert(
        "record".into(),
        group(r"<!-- platform-pr-review-state-v1 (\{[^\r\n]*\}) -->"),
    );
    out.insert(
        "diff".into(),
        group(r"<!-- pr-hygiene-diff-v1 (\{[^\r\n]*\}) -->"),
    );
    out.insert("markers".into(), optional(markers.clone()));
    out.insert("comment".into(), optional(comment));
    out.insert("comment_refused".into(), optional(refused));
    out.insert(
        "diff_print".into(),
        optional(diff_print(pr).expect("a diff print")),
    );
    out.insert("receipt_prints".into(), PyValue::Dict(receipts));
    PyValue::Dict(out)
}

/// `str(item['id'])` of a comment id.
fn dump_key(id: &PyValue) -> String {
    match id {
        PyValue::Str(s) => s.clone(),
        other => dump(other),
    }
}

/// What a replay did, set against what the recording holds.
pub struct Replayed {
    pub outcome: Result<Run, ReadError>,
    pub transport: ReplayTransport,
    pub reads: Vec<(String, usize)>,
    pub telemetry_reads: usize,
    pub policy_error: bool,
}

/// Run the port over a recording's reads, at its clock.
pub fn replay(recording: &Recording) -> Result<Replayed, String> {
    let meta = &recording.meta;
    let args = argv(meta)?;
    let transport =
        ReplayTransport::from_calls_jsonl(&recording.calls).map_err(|e| e.to_string())?;
    let calls: Calls = Default::default();
    let counted = Counted {
        inner: transport,
        calls: calls.clone(),
    };
    let repository = text(field(meta, "repository"));
    let mut api =
        GitHub::new(repository, Client::with_sleep(counted, NoSleep)).map_err(|e| e.to_string())?;
    let mut clock = LoggingClock::new(text(field(meta, "clock")), calls);
    let reads = clock.reads.clone();
    let policy = field(meta, "policy").clone();
    let sync = args.command == Command::Sync;
    // Inside a recording a sync takes the write path, so that what it would
    // write is captured; the recorder sends none of it.
    let apply = sync;
    let invalid = validate_policy(&policy)
        .err()
        .map(|e| e.to_string())
        .or_else(|| {
            (!matches!(field(&policy, "repository"), PyValue::Str(r) if r == repository))
                .then(|| "Repository must match the registered policy".to_owned())
        });
    if invalid.is_some() {
        if apply {
            let run_id = match field(field(meta, "environment"), "GITHUB_RUN_ID") {
                PyValue::Str(id) if !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) => {
                    Some(format!("https://github.com/{repository}/actions/runs/{id}"))
                }
                _ => None,
            };
            let mut reconciler = Reconciler::new(&mut api, &mut clock);
            reconciler
                .mark_configuration_error(Some(&policy), args.pr.as_ref(), run_id.as_deref())
                .map_err(|e| e.to_string())?;
        }
        let transport = api.client_mut().transport_mut();
        return Ok(Replayed {
            outcome: Err(ReadError::Exception {
                class: pr_hygiene_engine::evidence::PyClass::ValueError,
                detail: invalid.unwrap_or_default(),
            }),
            transport: std::mem::take(&mut transport.inner),
            reads: reads.borrow().clone(),
            telemetry_reads: 0,
            policy_error: true,
        });
    }
    let payload = field(meta, "telemetry").clone();
    let mut telemetry_reads = 0;
    let mut telemetry = || {
        telemetry_reads += 1;
        payload.clone()
    };
    let batch_clock = LoggingClock {
        at: clock.at,
        calls: clock.calls.clone(),
        reads: clock.reads.clone(),
    };
    let mut choose =
        |prs: &[PyValue]| periodic_batch(prs, args.batch_size.unwrap_or(1), &batch_clock);
    let selection = match (&args.pr, args.batch_size) {
        (Some(n), _) => Selection::Pr(n.clone()),
        (None, Some(_)) => Selection::Batch(&mut choose),
        (None, None) => Selection::All,
    };
    let outcome = {
        let mut reconciler = Reconciler::new(&mut api, &mut clock);
        reconciler.run(
            &policy,
            RunOptions {
                command: args.command,
                selection,
                apply,
                user: args.user.clone(),
                nudges: 1,
                telemetry: &mut telemetry,
            },
        )
    };
    let transport = std::mem::take(&mut api.client_mut().transport_mut().inner);
    let reads = reads.borrow().clone();
    Ok(Replayed {
        outcome,
        transport,
        reads,
        telemetry_reads,
        policy_error: false,
    })
}

/// Every way the replay differs from the recording. Empty means the same.
pub fn compare(dir: &Path, recording: &Recording) -> Vec<String> {
    let mut differences = Vec::new();
    let replayed = match replay(recording) {
        Ok(replayed) => replayed,
        Err(why) => return vec![format!("could not replay: {why}")],
    };
    let meta = &recording.meta;
    let args = argv(meta).expect("parsed once already");
    // The outcome: whether the run failed, and with what class of error.
    let recorded = field(meta, "outcome");
    let (run, raised) = match &replayed.outcome {
        Ok(run) => match run.failure() {
            Some(_) => (Some(run), Some("GitHubError".to_owned())),
            None => (Some(run), None),
        },
        Err(ReadError::GitHub(_)) => (None, Some("GitHubError".to_owned())),
        Err(ReadError::Exception { class, .. }) => (
            None,
            Some(match class {
                pr_hygiene_engine::evidence::PyClass::ValueError => "ValueError".to_owned(),
                pr_hygiene_engine::evidence::PyClass::KeyError => "KeyError".to_owned(),
                pr_hygiene_engine::evidence::PyClass::TypeError => "TypeError".to_owned(),
                other => format!("{other:?}"),
            }),
        ),
        Err(other) => return vec![format!("the port stopped: {other}")],
    };
    let python = match recorded {
        PyValue::Dict(outcome) => outcome.get("raised").map(|r| text(r).to_owned()),
        _ => None,
    };
    if python != raised {
        differences.push(format!(
            "outcome: Python raised {python:?}, the port {raised:?}"
        ));
    }
    // The verdicts: every field of every row, rows in order.
    let python_verdicts = items(&recording.verdicts);
    let verdicts = run.map(|run| run.verdicts.as_slice()).unwrap_or_default();
    if verdicts.len() != python_verdicts.len() {
        differences.push(format!(
            "verdicts: Python {}, the port {}",
            python_verdicts.len(),
            verdicts.len()
        ));
    }
    for (index, (ours, theirs)) in verdicts.iter().zip(python_verdicts).enumerate() {
        if let Some(at) = first_difference(ours, theirs, &format!("verdict {index}")) {
            differences.push(at);
        }
    }
    // The canonical outputs, one per verdict.
    if let Some(run) = run {
        let outputs: Vec<PyValue> = run
            .snapshots
            .iter()
            .zip(&run.verdicts)
            .map(|(pr, row)| outputs_for(field(meta, "policy"), &run.context, pr, row))
            .collect();
        let python_outputs = items(&recording.outputs);
        if outputs.len() != python_outputs.len() {
            differences.push(format!(
                "outputs: Python {}, the port {}",
                python_outputs.len(),
                outputs.len()
            ));
        }
        for (index, (ours, theirs)) in outputs.iter().zip(python_outputs).enumerate() {
            if let Some(at) = first_difference(ours, theirs, &format!("output {index}")) {
                differences.push(at);
            }
        }
        // The JSON report, as a value and as the text printed.
        if args.json && run.failure().is_none() {
            match (&run.report, py_loads(&recording.printed)) {
                (Some(report), Ok(printed)) => {
                    if same_form(report) != same_form(&printed) {
                        let at = first_difference(report, &printed, "report")
                            .unwrap_or_else(|| "report".into());
                        differences.push(format!("the JSON report differs at {at}"));
                    }
                    let text = py_dumps(report, false, None, Some(2)).expect("no floats") + "\n";
                    if text != recording.printed {
                        differences.push("the JSON report's text differs".into());
                    }
                }
                (None, _) => differences.push("no report".into()),
                (_, Err(e)) => differences.push(format!("printed.txt is not JSON: {e}")),
            }
        }
    }
    // The writes were checked one by one as they were made; none may be
    // left unmade, nor any read unasked, and every call came in the order
    // recorded.
    let transport = &replayed.transport;
    if transport.unwritten() > 0 {
        differences.push(format!(
            "{} recorded write(s) never made",
            transport.unwritten()
        ));
    }
    if transport.unasked() > 0 {
        differences.push(format!(
            "{} recorded read(s) never asked",
            transport.unasked()
        ));
    }
    let served = transport.served();
    if let Some(at) = served
        .iter()
        .enumerate()
        .position(|(i, ordinal)| *ordinal != i + 1)
    {
        differences.push(format!(
            "call {} was recorded as call {}: the calls were made in another order",
            at + 1,
            served[at]
        ));
    }
    // The clock: read at the same sites, at the same points among the calls.
    let python_reads: Vec<(String, usize)> = match field(meta, "clock_reads") {
        PyValue::List(reads) => reads
            .iter()
            .map(|read| {
                let after = match field(read, "after") {
                    PyValue::Int(n) => n.as_i64().expect("small") as usize,
                    _ => panic!("an ordinal"),
                };
                (text(field(read, "site")).to_owned(), after)
            })
            .collect(),
        _ => panic!("{}: format 2 records its clock reads", dir.display()),
    };
    if replayed.reads != python_reads {
        differences.push(format!(
            "clock reads: Python {python_reads:?}, the port {:?}",
            replayed.reads
        ));
    }
    let telemetry = match field(meta, "telemetry_reads") {
        PyValue::Int(n) => n.as_i64().expect("small") as usize,
        _ => 0,
    };
    if !replayed.policy_error && replayed.telemetry_reads != telemetry {
        differences.push(format!(
            "status page read {} time(s), Python {telemetry}",
            replayed.telemetry_reads
        ));
    }
    differences
}

fn synthetic() -> Vec<PathBuf> {
    recordings_under(&conformance().join("synthetic"))
}

#[test]
fn every_synthetic_recording_replays_to_the_same_run() {
    let dirs = synthetic();
    assert!(
        dirs.len() >= 13,
        "the committed synthetic recordings are found"
    );
    let mut failures = Vec::new();
    for dir in &dirs {
        let differences = compare(dir, &load(dir));
        if !differences.is_empty() {
            failures.push(format!(
                "{}:\n  {}",
                dir.display(),
                differences.join("\n  ")
            ));
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
        let recording = load(dir);
        if matches!(field(&recording.meta, "redacted"), PyValue::None) {
            failures.push(format!("{}: an unredacted recording", dir.display()));
            continue;
        }
        let differences = compare(dir, &recording);
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
fn tampered(name: &str, change: impl FnOnce(&mut Recording)) -> Vec<String> {
    let dir = conformance().join("synthetic").join(name);
    let mut recording = load(&dir);
    change(&mut recording);
    compare(&dir, &recording)
}

#[test]
fn a_write_with_other_words_fails_the_gate() {
    let found = tampered("sync-pr-2", |r| {
        assert!(r.calls.contains("Evaluating current review policy"));
        r.calls = r
            .calls
            .replace("Evaluating current review policy", "Evaluating the policy");
    });
    assert!(
        found
            .iter()
            .any(|d| d.contains("differs from the recording in its body")),
        "{found:?}"
    );
}

#[test]
fn a_write_python_did_not_make_fails_the_gate() {
    let found = tampered("sync-pr-2", |r| {
        // The last write, the status after the re-check, dropped.
        let mut lines: Vec<&str> = r.calls.lines().collect();
        let last = lines
            .iter()
            .rposition(|line| line.contains(r#""kind":"write""#))
            .expect("a write");
        lines.remove(last);
        r.calls = lines.iter().map(|line| format!("{line}\n")).collect();
    });
    assert!(
        found
            .iter()
            .any(|d| d.contains("A write the recording does not hold")),
        "{found:?}"
    );
}

#[test]
fn a_clock_read_somewhere_else_fails_the_gate() {
    let found = tampered("rich-evidence", |r| {
        let PyValue::Dict(meta) = &mut r.meta else {
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
        found.iter().any(|d| d.starts_with("clock reads")),
        "{found:?}"
    );
}

#[test]
fn a_verdict_or_an_output_python_did_not_reach_fails_the_gate() {
    let found = tampered("report", |r| {
        let PyValue::List(rows) = &mut r.verdicts else {
            panic!("rows")
        };
        let PyValue::Dict(row) = &mut rows[0] else {
            panic!("a row")
        };
        row.insert(
            "ready_since".into(),
            PyValue::Str("2026-01-01T00:00:00Z".into()),
        );
        let PyValue::List(outputs) = &mut r.outputs else {
            panic!("outputs")
        };
        let PyValue::Dict(output) = &mut outputs[1] else {
            panic!("an output")
        };
        output.insert("move".into(), PyValue::None);
    });
    assert!(
        found.iter().any(|d| d.starts_with("verdict 0.ready_since")),
        "{found:?}"
    );
    assert!(
        found.iter().any(|d| d.starts_with("output 1.move")),
        "{found:?}"
    );
}

#[test]
fn the_synthetic_recordings_walk_the_write_paths_they_are_named_for() {
    // Each recording is named for a path the engine must take; a generator
    // change that stopped walking it would leave the gate passing over
    // nothing.
    let writes = |name: &str| -> Vec<String> {
        load(&conformance().join("synthetic").join(name))
            .calls
            .lines()
            .filter(|line| line.contains(r#""kind":"write""#))
            .map(str::to_owned)
            .collect()
    };
    let sync = writes("sync-pr-2");
    assert!(sync
        .iter()
        .any(|w| w.contains(r#""POST","repos/dashpay/platform/issues/2/comments""#)));
    assert!(sync.iter().any(|w| w.contains("requested_reviewers")));
    let rich = writes("rich-evidence");
    assert!(rich
        .iter()
        .any(|w| w.contains(r#""DELETE","repos/dashpay/platform/issues/2/labels/waiting-bots""#)));
    assert!(rich
        .iter()
        .any(|w| w.contains(r#""DELETE","repos/dashpay/platform/issues/comments/200""#)));
    let routed = |name: &str, route: &str| writes(name).iter().any(|w| w.contains(route));
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
    assert!(load(&conformance().join("synthetic/sweep"))
        .calls
        .contains("repos/dashpay/platform/issues/4/comments?per_page=100"));
    // The hourly batch, chosen from the clock; an announcement edited in
    // place; the success published.
    let batch = load(&conformance().join("synthetic/batch"));
    assert!(dump(field(&batch.meta, "clock_reads")).contains("periodic_batch"));
    assert!(routed(
        "batch",
        r#""PATCH","repos/dashpay/platform/issues/comments/150""#
    ));
    assert!(writes("batch")
        .last()
        .is_some_and(|w| w.contains(r#"\"state\": \"success\""#)));
    // A nudge, with the status page read once.
    let nudge = load(&conformance().join("synthetic/nudge"));
    assert_eq!(dump(field(&nudge.meta, "telemetry_reads")), "1");
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
        let recording = load(&conformance().join("synthetic").join(name));
        assert_eq!(
            text(field(field(&recording.meta, "outcome"), "raised")),
            raised
        );
        assert!(
            writes(name).last().is_some_and(|w| w.contains(status)),
            "{name}"
        );
    }
    // The report, filtered to one person's pull requests.
    let report = py_loads(&load(&conformance().join("synthetic/report-json")).printed)
        .expect("a JSON report");
    assert_eq!(items(field(&report, "pull_requests")).len(), 1);
}

#[test]
fn a_printed_report_python_did_not_print_fails_the_gate() {
    let found = tampered("report-json", |r| {
        assert!(r.printed.contains(r#""author": "reviewer""#));
        r.printed = r
            .printed
            .replace(r#""author": "reviewer""#, r#""author": "someone""#);
    });
    assert!(
        found
            .iter()
            .any(|d| d.starts_with("the JSON report differs")),
        "{found:?}"
    );
}

#[test]
fn calls_made_in_another_order_fail_the_gate() {
    // The first two calls of the report, recorded the other way round.
    let found = tampered("report", |r| {
        r.calls = r
            .calls
            .replacen(r#"{"ordinal":1,"#, r#"{"ordinal":0,"#, 1)
            .replacen(r#"{"ordinal":2,"#, r#"{"ordinal":1,"#, 1)
            .replacen(r#"{"ordinal":0,"#, r#"{"ordinal":2,"#, 1);
    });
    assert!(
        found
            .iter()
            .any(|d| d.contains("the calls were made in another order")),
        "{found:?}"
    );
}
