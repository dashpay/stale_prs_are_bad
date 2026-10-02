//! One recording replayed as a whole run: the port's reconcile layer driven
//! over the recorded reads, at the recorded clock, with the recorded policy,
//! status page and `GITHUB_RUN_ID`, the dry sync walking the write path as
//! Python's did, and every write answered only as the next write the
//! recording holds, with the same route and the same body as a JSON value.
//!
//! What Python's command line decided from its own arguments is decided
//! here from the recorded `argv`: the hourly batch of `--batch-size`, which
//! `periodic_batch` chooses from the clock and reads it to choose, and the
//! one nudge a run may make. The engine itself takes whatever batch and
//! allowance its caller gives.
//!
//! The port makes Python's calls in Python's order, so this holds it to
//! more than the contract asks: every recorded call made, each once, in the
//! order recorded, and the clock read at the same sites at the same points
//! among them.

use super::compare::{compare_result, read_failure, Check, Comparison, Failure, Layer};
use super::diff::{differences, Difference, Kind};
use super::exception::OwnWords;
use super::recording::{LoadError, Recording};
use crate::evidence::records::{state_comment_body, validate_state};
use crate::evidence::{
    Call, Client, GitHub, NoSleep, ReadError, ReplayTransport, Reply, Transport, TransportError,
    WriteCheck,
};
use crate::policy::{diff_print, machine_author, receipt_print, validate_policy, LABEL_FOR_STATE};
use crate::pycompat::text::py_slice;
use crate::pycompat::{py_dumps, py_loads, PyDateTime, PyDict, PyInt, PyList, PyValue};
use crate::reconcile::{
    checklist_block, context_fingerprint, diff_record, move_state, move_text, state_record, Clock,
    ClockSite, Command, Reconciler, Run, RunOptions, Selection, POINTER, WAIVED_LABEL,
};
use regex::Regex;
use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;
use std::sync::LazyLock;

static STATE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<!-- platform-pr-review-state-v1 (\{[^\r\n]*\}) -->")
        .expect("a constant pattern compiles")
});
static DIFF_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<!-- pr-hygiene-diff-v1 (\{[^\r\n]*\}) -->").expect("a constant pattern compiles")
});

/// The files of a recording a whole-run replay reads beyond [`Recording`]'s.
pub const RUN_FILES: [&str; 2] = ["outputs.json", "printed.txt"];

/// What a recorded run put out: `outputs.json`, the text each verdict puts
/// on GitHub, and `printed.txt`, the report it printed.
#[derive(Clone)]
pub struct RunFiles {
    pub outputs: Vec<PyValue>,
    pub printed: String,
}

impl RunFiles {
    /// Read them through `read`, which returns the text of the named file.
    pub fn load_with<E: fmt::Display>(
        mut read: impl FnMut(&'static str) -> Result<String, E>,
    ) -> Result<Self, LoadError> {
        let mut text = |file: &'static str| {
            read(file).map_err(|error| LoadError::Unreadable {
                file,
                problem: error.to_string(),
            })
        };
        let outputs = match py_loads(&text("outputs.json")?) {
            Ok(PyValue::List(outputs)) => outputs.into_vec(),
            _ => {
                return Err(LoadError::Shape {
                    file: "outputs.json",
                    problem: "not a JSON list",
                })
            }
        };
        Ok(RunFiles {
            outputs,
            printed: text("printed.txt")?,
        })
    }
}

fn field<'a>(value: &'a PyValue, key: &str) -> Option<&'a PyValue> {
    match value {
        PyValue::Dict(entries) => entries.get(key),
        _ => None,
    }
}

fn s(text: impl Into<String>) -> PyValue {
    PyValue::Str(text.into())
}

fn int(n: usize) -> PyValue {
    PyValue::Int(PyInt::from(i64::try_from(n).unwrap_or(i64::MAX)))
}

/// What the recorded command line asked for.
struct Argv {
    command: Command,
    pr: Option<PyInt>,
    batch_size: Option<usize>,
    user: Option<String>,
    json: bool,
}

/// The recorded `argv`, or why the port does not run it.
fn argv(meta: &PyValue) -> Result<Argv, String> {
    let Some(PyValue::List(words)) = field(meta, "argv") else {
        return Err("recording.json holds no argv".into());
    };
    let mut words = words.iter().map(|word| match word {
        PyValue::Str(word) => Ok(word.as_str()),
        _ => Err("an argument that is not text".to_owned()),
    });
    let command = match words.next().transpose()? {
        Some("sync") => Command::Sync,
        Some("report") => Command::Report,
        _ => return Err("a command the port does not run".into()),
    };
    let mut parsed = Argv {
        command,
        pr: None,
        batch_size: None,
        user: None,
        json: false,
    };
    while let Some(word) = words.next().transpose()? {
        let mut value = || -> Result<&str, String> {
            words
                .next()
                .transpose()?
                .ok_or_else(|| format!("{word} without a value"))
        };
        match word {
            "--repo" => {
                value()?;
            }
            "--pr" => {
                parsed.pr = Some(PyInt::from_decimal(value()?).ok_or("--pr is not a number")?)
            }
            "--batch-size" => {
                parsed.batch_size = Some(value()?.parse().map_err(|_| "--batch-size")?)
            }
            "--user" => parsed.user = Some(value()?.to_owned()),
            "--format" => parsed.json = value()? == "json",
            _ => return Err("an argument the port does not take".into()),
        }
    }
    Ok(parsed)
}

/// How many calls have crossed the transport, shared with the clock, which
/// logs each read against it.
type Calls = Rc<Cell<usize>>;

/// The replay transport, counting the calls made through it.
struct Counted {
    inner: ReplayTransport,
    calls: Calls,
}

impl Transport for Counted {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        self.calls.set(self.calls.get() + 1);
        self.inner.call(call)
    }
}

/// The recorded instant, logging every read as Python's recorder does: the
/// site, and the ordinal of the last call before it.
#[derive(Clone)]
struct LoggingClock {
    at: PyDateTime,
    calls: Calls,
    reads: Rc<RefCell<Vec<(String, usize)>>>,
}

impl LoggingClock {
    fn log(&self, site: &str) {
        self.reads
            .borrow_mut()
            .push((site.to_owned(), self.calls.get()));
    }
}

impl Clock for LoggingClock {
    fn now(&mut self, site: ClockSite) -> PyDateTime {
        self.log(site.as_str());
        self.at
    }
}

/// `main.periodic_batch(prs, size)`: a slice of the governed pull requests,
/// by number, rotating with the hour of the clock — which it reads only when
/// there is something to choose from.
fn periodic_batch(prs: &[PyValue], size: usize, clock: &LoggingClock) -> Vec<PyValue> {
    let number = |pr: &PyValue| match field(pr, "number") {
        Some(PyValue::Int(n)) => n.clone(),
        _ => PyInt::from(0),
    };
    let mut ordered = prs.to_vec();
    ordered.sort_by_key(number);
    if ordered.is_empty() || size == 0 {
        return Vec::new();
    }
    clock.log("periodic_batch");
    let seconds = clock.at.timestamp().unwrap_or(0.0);
    let hour = (seconds / 3600.0).floor() as i64;
    let length = ordered.len() as i64;
    let start = hour.wrapping_mul(size as i64).rem_euclid(length) as usize;
    (0..size.min(ordered.len()))
        .map(|offset| ordered[(start + offset) % ordered.len()].clone())
        .collect()
}

/// `conformance.outputs_for(engine, policy, context, pr, result)`: the
/// exact text a verdict puts on GitHub, as `publish` would write it.
fn outputs_for(
    policy: &PyValue,
    context: &[PyValue],
    pr: &PyValue,
    result: &PyValue,
) -> Result<PyValue, String> {
    let error = |e: &dyn fmt::Display| e.to_string();
    let state = field(result, "state").unwrap_or(&PyValue::None);
    let author = field(pr, "author").unwrap_or(&PyValue::None);
    let printed = context_fingerprint(context, author).map_err(|e| error(&e))?;
    let record = state_record(pr, result, &printed).map_err(|e| error(&e))?;
    let carried = diff_record(pr, result).map_err(|e| error(&e))?;
    let move_body = match move_state(state).map_err(|e| error(&e))? {
        Some(mv)
            if !(machine_author(policy, pr).map_err(|e| error(&e))?
                && mv == "waiting-self-review") =>
        {
            move_text(result).map_err(|e| error(&e))?
        }
        _ => None,
    };
    let written = validate_state(&record).and_then(|()| {
        state_comment_body(
            &record,
            move_body.as_deref().unwrap_or(POINTER),
            carried.as_ref(),
        )
    });
    let (comment, refused) = match written {
        Ok(comment) => (Some(comment), None),
        Err(ReadError::GitHub(why)) => (None, Some(why)),
        Err(other) => return Err(other.to_string()),
    };
    let markers = comment
        .as_deref()
        .map(|c| c.split_once("\n\n").map_or(c, |(m, _)| m).to_owned());
    // `STATE_PATTERN.search(markers)` and `DIFF_PATTERN.search(markers)`:
    // the JSON each marker carries. The patterns are ASCII, so the regex
    // crate's own classes read them as Python's do.
    let group = |pattern: &Regex| -> PyValue {
        markers
            .as_deref()
            .and_then(|markers| pattern.captures(markers))
            .and_then(|found| found.get(1))
            .map_or(PyValue::None, |json| s(json.as_str()))
    };
    let mut labels: Vec<&str> = LABEL_FOR_STATE
        .iter()
        .filter(|(name, _)| matches!(state, PyValue::Str(t) if t == name))
        .map(|(_, label)| *label)
        .collect();
    if field(result, "waived").is_some_and(PyValue::truthy) {
        labels.push(WAIVED_LABEL);
    }
    labels.sort();
    labels.dedup();
    let mut receipts = PyDict::new();
    if let Some(PyValue::List(comments)) = field(pr, "comments") {
        for comment in comments.iter() {
            if let Some(said) = receipt_print(comment).map_err(|e| error(&e))? {
                let id = match field(comment, "id") {
                    Some(PyValue::Str(id)) => id.clone(),
                    Some(id) => py_dumps(id, false, None, None).map_err(|e| error(&e))?,
                    None => "None".into(),
                };
                receipts.insert(id, s(said));
            }
        }
    }
    let state_text = match state {
        PyValue::Str(state) => state.as_str(),
        _ => return Err("a verdict without a state".into()),
    };
    let checklist = if state_text == "draft" {
        PyValue::None
    } else {
        checklist_block(result)
            .map_err(|e| error(&e))?
            .map_or(PyValue::None, s)
    };
    let mut status = PyDict::new();
    status.insert(
        "state".into(),
        field(result, "status").cloned().unwrap_or(PyValue::None),
    );
    status.insert("context".into(), s("PR Hygiene"));
    status.insert(
        "description".into(),
        s(py_slice(state_text, None, Some(140))),
    );
    let optional = |text: Option<String>| text.map_or(PyValue::None, s);
    let mut out = PyDict::new();
    for key in ["number", "head"] {
        out.insert(key.into(), field(pr, key).cloned().unwrap_or(PyValue::None));
    }
    out.insert("status".into(), PyValue::Dict(status));
    out.insert(
        "labels".into(),
        PyValue::List(labels.into_iter().map(s).collect()),
    );
    out.insert("checklist".into(), checklist);
    out.insert("move".into(), optional(move_body));
    out.insert("record".into(), group(&STATE_PATTERN));
    out.insert("diff".into(), group(&DIFF_PATTERN));
    out.insert("markers".into(), optional(markers.clone()));
    out.insert("comment".into(), optional(comment));
    out.insert("comment_refused".into(), optional(refused));
    out.insert(
        "diff_print".into(),
        optional(diff_print(pr).map_err(|e| error(&e))?),
    );
    out.insert("receipt_prints".into(), PyValue::Dict(receipts));
    Ok(PyValue::Dict(out))
}

/// What the replayed run did, beside what it decided.
struct Replayed {
    outcome: Result<Run, ReadError>,
    transport: ReplayTransport,
    reads: Vec<(String, usize)>,
    telemetry_reads: usize,
    policy_error: bool,
}

/// The URL Python's error status links to, from `GITHUB_RUN_ID`.
fn run_url(meta: &PyValue, repository: &str) -> Option<String> {
    let id = field(field(meta, "environment")?, "GITHUB_RUN_ID")?;
    match id {
        PyValue::Str(id) if !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) => {
            Some(format!("https://github.com/{repository}/actions/runs/{id}"))
        }
        _ => None,
    }
}

/// Run the port over a recording's reads, at its clock.
fn replay(recording: &Recording, args: &Argv) -> Result<Replayed, (Failure, String)> {
    let meta = &recording.meta;
    let transport = ReplayTransport::from_calls_jsonl(&recording.calls)
        .map_err(|e| (Failure::CallsUnreadable, e.to_string()))?;
    let calls: Calls = Rc::default();
    let counted = Counted {
        inner: transport,
        calls: calls.clone(),
    };
    let repository = recording
        .repository()
        .ok_or((Failure::Repository, "no repository".to_owned()))?;
    let mut api = GitHub::new(repository, Client::with_sleep(counted, NoSleep))
        .map_err(|e| (Failure::Repository, e.to_string()))?;
    let at = match field(meta, "clock") {
        Some(PyValue::Str(at)) => PyDateTime::fromisoformat(&at.replace('Z', "+00:00")).ok(),
        _ => None,
    }
    .ok_or((
        Failure::Malformed,
        "recording.json holds no clock".to_owned(),
    ))?;
    let mut clock = LoggingClock {
        at,
        calls,
        reads: Rc::default(),
    };
    let policy = recording.policy().cloned().ok_or((
        Failure::Malformed,
        "recording.json holds no policy".to_owned(),
    ))?;
    // Inside a recording a sync takes the write path, so that what it would
    // write is captured; the recorder sends none of it.
    let apply = args.command == Command::Sync;
    let invalid = match validate_policy(&policy) {
        Err(error) => Some(error.to_string()),
        Ok(()) if !matches!(field(&policy, "repository"), Some(PyValue::Str(r)) if r == repository) => {
            Some("Repository must match the registered policy".to_owned())
        }
        Ok(()) => None,
    };
    if let Some(invalid) = invalid {
        if apply {
            let url = run_url(meta, repository);
            let mut engine = Reconciler::new(&mut api, &mut clock);
            if let Err(error) =
                engine.mark_configuration_error(Some(&policy), args.pr.as_ref(), url.as_deref())
            {
                let transport = std::mem::take(&mut api.client_mut().transport_mut().inner);
                let reads = clock.reads.borrow().clone();
                return Ok(Replayed {
                    outcome: Err(error),
                    transport,
                    reads,
                    telemetry_reads: 0,
                    policy_error: true,
                });
            }
        }
        let transport = std::mem::take(&mut api.client_mut().transport_mut().inner);
        let reads = clock.reads.borrow().clone();
        return Ok(Replayed {
            outcome: Err(ReadError::Exception {
                class: crate::evidence::PyClass::ValueError,
                detail: invalid,
            }),
            transport,
            reads,
            telemetry_reads: 0,
            policy_error: true,
        });
    }
    let payload = field(meta, "telemetry").cloned().unwrap_or(PyValue::None);
    let mut telemetry_reads = 0;
    let mut telemetry = || {
        telemetry_reads += 1;
        payload.clone()
    };
    let batch_clock = clock.clone();
    let size = args.batch_size.unwrap_or(1);
    let mut choose = |prs: &[PyValue]| periodic_batch(prs, size, &batch_clock);
    let selection = match (&args.pr, args.batch_size) {
        (Some(n), _) => Selection::Pr(n.clone()),
        (None, Some(_)) => Selection::Batch(&mut choose),
        (None, None) => Selection::All,
    };
    let outcome = Reconciler::new(&mut api, &mut clock).run(
        &policy,
        RunOptions {
            command: args.command,
            selection,
            apply,
            user: args.user.clone(),
            nudges: 1,
            telemetry: &mut telemetry,
        },
    );
    let transport = std::mem::take(&mut api.client_mut().transport_mut().inner);
    let reads = clock.reads.borrow().clone();
    Ok(Replayed {
        outcome,
        transport,
        reads,
        telemetry_reads,
        policy_error: false,
    })
}

/// A list of `[site, after]` pairs, as a value to compare.
fn reads_value(reads: &[(String, usize)]) -> PyValue {
    PyValue::List(
        reads
            .iter()
            .map(|(site, after)| PyValue::List(PyList::from(vec![s(site.as_str()), int(*after)])))
            .collect(),
    )
}

/// Replay `recording` as a whole run through the port and compare it with
/// what Python's run did: its outcome, its verdict rows, its writes, the
/// outputs of each verdict, its JSON report, its clock reads and the order
/// of its calls. `files` is what the run put out.
pub fn replay_run(recording: &Recording, files: &RunFiles, own: &OwnWords) -> Comparison {
    let stopped = |failure, detail: String| Comparison {
        checks: vec![Check::failed(Layer::Run, 0, failure, detail)],
        missing_reads: 0,
    };
    let args = match argv(&recording.meta) {
        Ok(args) => args,
        Err(why) => return stopped(Failure::Command, why),
    };
    let replayed = match replay(recording, &args) {
        Ok(replayed) => replayed,
        Err((failure, detail)) => return stopped(failure, detail),
    };
    let transport = &replayed.transport;
    let missing_reads = transport.missing() + transport.exhausted();
    let mut checks = Vec::new();

    // The outcome: whether the run failed, and with which class of error.
    let python = match field(&recording.meta, "outcome").and_then(|o| field(o, "raised")) {
        Some(raised) => raised.clone(),
        None => PyValue::None,
    };
    let run = match &replayed.outcome {
        Ok(run) => {
            let raised = run.failure().map_or(PyValue::None, |_| s("GitHubError"));
            checks.push(Check::new(
                Layer::Run,
                0,
                differences(&raised, &python, "outcome"),
            ));
            Some(run)
        }
        Err(ReadError::GitHub(_)) => {
            checks.push(Check::new(
                Layer::Run,
                0,
                differences(&s("GitHubError"), &python, "outcome"),
            ));
            None
        }
        Err(ReadError::Exception { class, .. }) => {
            checks.push(Check::new(
                Layer::Run,
                0,
                differences(&s(format!("{class:?}")), &python, "outcome"),
            ));
            None
        }
        Err(error) => {
            let failure = read_failure(error, transport, (0, 0));
            checks.push(Check::failed(Layer::Run, 0, failure, error.to_string()));
            None
        }
    };
    if !replayed.policy_error {
        let recorded = field(&recording.meta, "telemetry_reads")
            .cloned()
            .unwrap_or(int(0));
        checks.push(Check::new(
            Layer::Run,
            1,
            differences(&int(replayed.telemetry_reads), &recorded, "telemetry_reads"),
        ));
    }

    // The verdict rows, every field of every row, in order.
    let ours = run.map(|run| run.verdicts.as_slice()).unwrap_or_default();
    if ours.len() != recording.verdicts.len() {
        checks.push(Check::new(
            Layer::RunVerdict,
            0,
            vec![Difference {
                path: "verdicts".into(),
                kind: Kind::Length,
            }],
        ));
    } else {
        for (index, (row, python)) in ours.iter().zip(&recording.verdicts).enumerate() {
            let exception_text = recording
                .evaluations
                .get(index)
                .and_then(|evaluation| field(evaluation, "result"))
                .is_some_and(|result| own.python_exception_text(result));
            checks.push(Check::new(
                Layer::RunVerdict,
                index,
                compare_result(row, python, "verdict", exception_text, own),
            ));
        }
    }

    // What each verdict puts on GitHub.
    if let Some(run) = run {
        if run.verdicts.len() != files.outputs.len() {
            checks.push(Check::new(
                Layer::Output,
                0,
                vec![Difference {
                    path: "outputs".into(),
                    kind: Kind::Length,
                }],
            ));
        } else {
            for (index, ((pr, row), python)) in run
                .snapshots
                .iter()
                .zip(&run.verdicts)
                .zip(&files.outputs)
                .enumerate()
            {
                checks.push(
                    match outputs_for(
                        recording.policy().unwrap_or(&PyValue::None),
                        &run.context,
                        pr,
                        row,
                    ) {
                        Ok(ours) => {
                            Check::new(Layer::Output, index, differences(&ours, python, "output"))
                        }
                        Err(why) => Check::failed(Layer::Output, index, Failure::Unmade, why),
                    },
                );
            }
        }
        // The JSON report, compared as the value printed, its keys in order.
        if args.json && run.failure().is_none() {
            checks.push(match (&run.report, py_loads(&files.printed)) {
                (Some(report), Ok(printed)) => {
                    Check::new(Layer::Report, 0, differences(report, &printed, "report"))
                }
                (None, _) => Check::failed(Layer::Report, 0, Failure::NoResult, "no report"),
                (_, Err(_)) => Check::failed(
                    Layer::Report,
                    0,
                    Failure::Malformed,
                    "printed.txt is not JSON",
                ),
            });
        }
    }

    // Every write, by its place in the recorded order: made as recorded, to
    // the same route with another body (its fields, by path), to another
    // route, past the last recorded one, or never made at all.
    let made = transport.write_checks();
    for (index, check) in made.iter().enumerate() {
        checks.push(match check {
            WriteCheck::Matched => Check::new(Layer::Write, index, Vec::new()),
            WriteCheck::Body { made, recorded } => {
                Check::new(Layer::Write, index, differences(made, recorded, "write"))
            }
            WriteCheck::Route => Check::failed(
                Layer::Write,
                index,
                Failure::WriteRouteDiffers,
                "made to another route than the recorded write's",
            ),
            WriteCheck::Unrecorded => Check::failed(
                Layer::Write,
                index,
                Failure::WriteNotRecorded,
                "made after every recorded write",
            ),
        });
    }
    // A refused write took no recorded one: the one it stood against is
    // counted where it was refused, not again as never made.
    let consumed = transport.recorded_writes() - transport.unwritten();
    let refused = usize::from(matches!(made.last(), Some(WriteCheck::Route)));
    for index in consumed + refused..transport.recorded_writes() {
        checks.push(Check::failed(
            Layer::Write,
            index,
            Failure::WriteNotMade,
            "a recorded write the run never made",
        ));
    }

    // The clock, read at the same sites at the same points among the calls.
    if let Some(PyValue::List(recorded)) = field(&recording.meta, "clock_reads") {
        let python: Vec<PyValue> = recorded
            .iter()
            .map(|read| {
                PyValue::List(PyList::from(vec![
                    field(read, "site").cloned().unwrap_or(PyValue::None),
                    field(read, "after").cloned().unwrap_or(PyValue::None),
                ]))
            })
            .collect();
        checks.push(Check::new(
            Layer::Clock,
            0,
            differences(
                &reads_value(&replayed.reads),
                &PyValue::List(python.into()),
                "clock_reads",
            ),
        ));
    }

    // Every recorded call made, once, in the order recorded.
    let recorded = recording
        .calls
        .split('\n')
        .filter(|line| !line.is_empty())
        .count();
    let served: Vec<PyValue> = transport.served().iter().map(|n| int(*n)).collect();
    let expected: Vec<PyValue> = (1..=recorded).map(int).collect();
    checks.push(Check::new(
        Layer::Call,
        0,
        differences(
            &PyValue::List(served.into()),
            &PyValue::List(expected.into()),
            "calls",
        ),
    ));
    Comparison {
        checks,
        missing_reads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock(at: &str) -> LoggingClock {
        LoggingClock {
            at: PyDateTime::fromisoformat(at).unwrap(),
            calls: Rc::default(),
            reads: Rc::default(),
        }
    }

    #[test]
    fn the_batch_rotates_with_the_hour_and_reads_the_clock_only_to_choose() {
        let prs: Vec<PyValue> = (1..=5)
            .map(|n| py_loads(&format!(r#"{{"number": {n}}}"#)).unwrap())
            .collect();
        let numbers = |batch: Vec<PyValue>| -> Vec<String> {
            batch
                .iter()
                .map(|pr| py_dumps(field(pr, "number").unwrap(), false, None, None).unwrap())
                .collect()
        };
        // 2026-09-12T10:00Z is hour 497002 of the epoch: 497002 * 2 mod 5 = 4.
        let at = clock("2026-09-12T10:00:00+00:00");
        assert_eq!(numbers(periodic_batch(&prs, 2, &at)), ["5", "1"]);
        assert_eq!(at.reads.borrow().len(), 1);
        let none = clock("2026-09-12T10:00:00+00:00");
        assert!(periodic_batch(&[], 2, &none).is_empty());
        assert!(none.reads.borrow().is_empty(), "nothing to choose, no read");
    }

    #[test]
    fn a_command_the_port_does_not_run_is_refused_by_kind() {
        let meta =
            py_loads(r#"{"argv": ["sync", "--repo", "a/b", "--waiting-on-build"]}"#).unwrap();
        assert!(argv(&meta).is_err());
        let meta = py_loads(
            r#"{"argv": ["sync", "--repo", "a/b", "--batch-size", "3", "--format", "json"]}"#,
        )
        .unwrap();
        let args = argv(&meta).unwrap();
        assert_eq!((args.batch_size, args.json), (Some(3), true));
    }
}
