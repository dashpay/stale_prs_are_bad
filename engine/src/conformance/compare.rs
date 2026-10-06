//! One recording, compared: every snapshot Python's `evaluate` saw, rebuilt
//! from the recorded reads alone; every evaluation, run again through the
//! port; every verdict row, as far as `evaluate` decides it.
//!
//! The reader is driven over a [`ReplayTransport`] as Python's run drove
//! its own: one reader for the run, with its caches; the batched history
//! read that admission made, its pull requests named by the recorded query
//! itself; a snapshot for each evaluation, the first ones (as many as there
//! are verdicts, or all of them where the run raised before writing any)
//! over that history, and each later one — the last check before a write —
//! after the caches are dropped and without it. Every read
//! must be one the recording holds, asked no more often than Python asked
//! it.
//!
//! What Python read between those snapshots and did not evaluate — the
//! re-check before a write, the admission re-read — is not asked here, so a
//! later snapshot may be answered with a recorded answer Python's earlier
//! read got. Where the same read was answered differently within one run,
//! as GitHub changing under a live run can do, a difference here is that
//! and not necessarily the port.

use super::diff::{differences, Difference, Kind};
use super::exception::{set_aside_exception_text, ExceptionText, OwnWords};
use super::recording::Recording;
use crate::evidence::{Client, GitHub, NoSleep, PyClass, ReadError, ReplayTransport};
use crate::policy::evaluate;
use crate::pycompat::{py_loads, PyErr, PyInt, PyValue};
use regex::Regex;
use std::fmt;
use std::sync::LazyLock;

/// What `evaluate_snapshots` adds to a verdict whose head another open pull
/// request shares, after `evaluate` has answered.
pub const SHARED_HEAD: &str = "Another open PR shares this head; commit-scoped status is ambiguous";

/// What a check compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer {
    /// The `pr` Python's `evaluate` was given, rebuilt from the reads.
    Snapshot,
    /// `evaluate`'s result on Python's own arguments.
    Evaluation,
    /// A row of `verdicts.json`.
    Verdict,
    /// A whole run replayed: its outcome, and how often it read the status
    /// page.
    Run,
    /// A verdict row the whole run made.
    RunVerdict,
    /// The recorded writes, every one made.
    Write,
    /// What a verdict puts on GitHub, by `outputs.json`.
    Output,
    /// The JSON report the run printed.
    Report,
    /// Where the run read the clock, and when among its calls.
    Clock,
    /// Every recorded call made once, in the order recorded.
    Call,
    /// A pull request read live, against the `pr` Python's `evaluate` was
    /// given.
    LiveSnapshot,
    /// A verdict decided live, against Python's row.
    LiveVerdict,
    /// A whole run made live against pull requests whose records are
    /// current: the writes it would have made, which must be none.
    LiveWrites,
}

impl Layer {
    pub fn as_str(self) -> &'static str {
        match self {
            Layer::Snapshot => "snapshot",
            Layer::Evaluation => "evaluation",
            Layer::Verdict => "verdict",
            Layer::Run => "run",
            Layer::RunVerdict => "run verdict",
            Layer::Write => "write",
            Layer::Output => "output",
            Layer::Report => "report",
            Layer::Clock => "clock",
            Layer::Call => "call",
            Layer::LiveSnapshot => "live snapshot",
            Layer::LiveVerdict => "live verdict",
            Layer::LiveWrites => "live writes",
        }
    }
}

/// Why the port gave no value to compare. Each names a kind of failure,
/// never what the recording held.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Failure {
    /// The port asked a read the recording does not hold.
    ReadNotRecorded,
    /// The port asked a read more often than Python did.
    ReadAskedAgain,
    /// The transport refused the call for another reason: a write.
    CallRefused,
    /// The reader raised `GitHubError`.
    GitHubError,
    /// The reader raised another Python exception.
    Raised(PyClass),
    /// The port refused something Python would have done.
    NotPorted,
    /// The batched history read every first snapshot shares failed.
    HistoryUnread,
    /// `calls.jsonl` could not be read as a replay.
    CallsUnreadable,
    /// The recording names no repository the reader accepts.
    Repository,
    /// An evaluation without the arguments `evaluate` was given.
    Malformed,
    /// An evaluation whose `pr` has no integer number.
    NoNumber,
    /// The port's `evaluate` raised where Python's answered.
    EvaluateRaised(PyClass),
    /// The port's `evaluate` refused something Python did.
    EvaluateNotPorted,
    /// A verdict row with no evaluation in its place.
    NoEvaluation,
    /// A verdict row whose evaluation gave the port no result.
    NoResult,
    /// A recorded command the port does not run, or a recording that does
    /// not say how it was run.
    Command,
    /// A write the recording holds that the run never made.
    WriteNotMade,
    /// A write made to another route than the recorded write in its place.
    WriteRouteDiffers,
    /// A write made after every write the recording holds.
    WriteNotRecorded,
    /// A recording without what replaying its run needs: its clock, its
    /// policy.
    RunIncomplete,
    /// What the run put out could not be read: `outputs.json` or
    /// `printed.txt`.
    RunFilesUnreadable,
    /// `printed.txt` is not the JSON report the run printed.
    ReportUnreadable,
    /// The replayed run made no report where Python's printed one.
    NoReport,
    /// A verdict whose outputs the port could not make.
    Unmade,
    /// A live read the read-only layer would not send: not one of the
    /// engine's reads of a repository it may read.
    ReadRefused,
    /// A live run that evaluated other pull requests than Python's run, or
    /// in another order.
    OtherPullRequests,
    /// A live read failed, every time it was asked, where Python's same
    /// read was answered: GitHub failed it, or the transport did. Nothing
    /// after it compares the two engines.
    LiveReadFailed,
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Failure::ReadNotRecorded => f.write_str("read not in the recording"),
            Failure::ReadAskedAgain => f.write_str("read asked more often than recorded"),
            Failure::CallRefused => f.write_str("call refused by the replay"),
            Failure::GitHubError => f.write_str("reader raised GitHubError"),
            Failure::Raised(class) => write!(f, "reader raised {class:?}"),
            Failure::NotPorted => f.write_str("reader refused: not ported"),
            Failure::HistoryUnread => f.write_str("batched history read failed"),
            Failure::CallsUnreadable => f.write_str("calls.jsonl unreadable"),
            Failure::Repository => f.write_str("no valid repository"),
            Failure::Malformed => f.write_str("evaluation lacks its arguments"),
            Failure::NoNumber => f.write_str("evaluated pr has no number"),
            Failure::EvaluateRaised(class) => write!(f, "evaluate raised {class:?}"),
            Failure::EvaluateNotPorted => f.write_str("evaluate refused: not ported"),
            Failure::NoEvaluation => f.write_str("verdict without an evaluation"),
            Failure::NoResult => f.write_str("verdict's evaluation gave no result"),
            Failure::Command => f.write_str("a command the port does not run"),
            Failure::WriteNotMade => f.write_str("recorded write never made"),
            Failure::WriteRouteDiffers => f.write_str("write made to another route"),
            Failure::WriteNotRecorded => f.write_str("write the recording does not hold"),
            Failure::RunIncomplete => f.write_str("recording lacks what its run needs"),
            Failure::RunFilesUnreadable => f.write_str("outputs.json or printed.txt unreadable"),
            Failure::ReportUnreadable => f.write_str("printed report is not JSON"),
            Failure::NoReport => f.write_str("run made no report"),
            Failure::Unmade => f.write_str("outputs the port could not make"),
            Failure::ReadRefused => f.write_str("live read refused by the read-only layer"),
            Failure::OtherPullRequests => {
                f.write_str("live run evaluated other pull requests than Python's")
            }
            Failure::LiveReadFailed => f.write_str("live read failed where Python's was answered"),
        }
    }
}

/// Which of Python's own inputs made a live difference vanish when the
/// port was given them in place of its own: the instant it decided at, the
/// review system's status page, the instant a pull request was admitted.
/// Or that it is how `gh api` printed GitHub's answer to Python
/// (`gh_printed`, see `gh_output`): not one of Python's inputs but its view
/// of them, a deliberate divergence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Explanation {
    pub clock: bool,
    pub telemetry: bool,
    pub admission: bool,
    pub gh_printed: bool,
}

impl fmt::Display for Explanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let named: Vec<&str> = [
            (self.clock, "clock"),
            (self.telemetry, "status page"),
            (self.admission, "admission"),
            (self.gh_printed, "gh-printed control characters"),
        ]
        .into_iter()
        .filter(|(on, _)| *on)
        .map(|(_, name)| name)
        .collect();
        f.write_str(&named.join(" and "))
    }
}

/// Why one of Python's inputs did not explain a live difference: said in
/// fixed words and counts, never by what either side read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unoffered {
    /// Offered, and the difference stayed.
    Tried,
    /// Python's input is the port's own: the same instant, the same page,
    /// the same answers.
    Same,
    /// Read on one side only.
    OneSideOnly,
    /// Read on neither side.
    NotRead,
    /// Python's input could not be read.
    Unreadable,
    /// The live run did not admit the pull request.
    LiveNotAdmitted,
    /// Python's run did not admit it.
    PythonNotAdmitted,
    /// The pull request names no author whose admissions to compare.
    NoAuthor,
    /// Python's run could not be replayed to read its admissions.
    PythonNotReplayed,
    /// Its author's admitted pull requests are not the same: how many on
    /// each side.
    AdmittedDiffer { live: usize, python: usize },
    /// The same pull requests admitted, in another order.
    OrderDiffers,
}

impl Unoffered {
    /// What it says of `input`, one of [`WhyNot`]'s.
    fn said(self, input: &str) -> String {
        match self {
            Unoffered::Tried => "offered, and the difference stayed".into(),
            Unoffered::Same if input == "status page" => "the same page".into(),
            Unoffered::Same if input == "gh-printed control characters" => {
                "every answer prints as it came".into()
            }
            Unoffered::Same => "the same instant".into(),
            Unoffered::OneSideOnly => "read on one side only".into(),
            Unoffered::NotRead => "read on neither side".into(),
            Unoffered::Unreadable => "Python's could not be read".into(),
            Unoffered::LiveNotAdmitted => "live not admitted".into(),
            Unoffered::PythonNotAdmitted => "Python not admitted".into(),
            Unoffered::NoAuthor => "no author".into(),
            Unoffered::PythonNotReplayed => "Python's run not replayed".into(),
            Unoffered::AdmittedDiffer { live, python } => {
                format!("admitted sets differ (live {live}, Python {python})")
            }
            Unoffered::OrderDiffers => "admitted in another order".into(),
        }
    }
}

/// Why none of Python's inputs explained a live difference, input by input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhyNot {
    pub clock: Unoffered,
    pub telemetry: Unoffered,
    pub admission: Unoffered,
    pub gh_printed: Unoffered,
}

impl fmt::Display for WhyNot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let said: Vec<String> = [
            ("clock", self.clock),
            ("status page", self.telemetry),
            ("admission", self.admission),
            ("gh-printed control characters", self.gh_printed),
        ]
        .into_iter()
        .map(|(input, why)| format!("{input}: {}", why.said(input)))
        .collect();
        f.write_str(&said.join("; "))
    }
}

/// What one check found.
#[derive(Debug, Clone)]
pub enum Outcome {
    Matched,
    Differs(Vec<Difference>),
    /// No value to compare. `detail` is the error itself: it may quote what
    /// the recording holds (a route naming a login, a GitHub message), so
    /// it is for a test or a local session over a recording one may read,
    /// and never for output.
    Failed {
        failure: Failure,
        detail: String,
    },
    /// Live differences that vanished once the port was given Python's own
    /// inputs (`by`): the two engines read the same and decided alike, at
    /// different moments. Not a failure.
    Explained {
        differences: Vec<Difference>,
        by: Explanation,
    },
    /// The pull request changed between Python's read and the port's live
    /// one, so what differs says nothing of the port. Not a failure.
    Moved,
    /// Live differences none of Python's inputs explained, and why each was
    /// not offered or did not remove them. A failure, as `Differs` is.
    Unexplained {
        differences: Vec<Difference>,
        why: WhyNot,
    },
    /// Not compared, for a reason that is neither engine's: GitHub or the
    /// transport failing a read. `why` is the tool's own words and classes,
    /// never what was read. Counted, and not a failure.
    NotCompared {
        why: String,
    },
}

/// One snapshot, evaluation or verdict, by its index in its file.
#[derive(Debug, Clone)]
pub struct Check {
    pub layer: Layer,
    pub index: usize,
    pub outcome: Outcome,
}

impl Check {
    pub(super) fn new(layer: Layer, index: usize, found: Vec<Difference>) -> Self {
        let outcome = if found.is_empty() {
            Outcome::Matched
        } else {
            Outcome::Differs(found)
        };
        Check {
            layer,
            index,
            outcome,
        }
    }

    pub fn failed(layer: Layer, index: usize, failure: Failure, detail: impl Into<String>) -> Self {
        Check {
            layer,
            index,
            outcome: Outcome::Failed {
                failure,
                detail: detail.into(),
            },
        }
    }

    pub fn matched(&self) -> bool {
        matches!(self.outcome, Outcome::Matched)
    }

    /// Whether this check is no failure: it matched, or what differs is
    /// explained, or the pull request moved under the read.
    pub fn passed(&self) -> bool {
        matches!(
            self.outcome,
            Outcome::Matched
                | Outcome::Explained { .. }
                | Outcome::Moved
                | Outcome::NotCompared { .. }
        )
    }
}

/// Everything one recording's comparison found.
#[derive(Debug, Clone, Default)]
pub struct Comparison {
    pub checks: Vec<Check>,
    /// Reads the port asked that the recording does not hold, or asked more
    /// often than Python did.
    pub missing_reads: usize,
}

impl Comparison {
    /// How many checks of a layer matched, of how many.
    pub fn matched(&self, layer: Layer) -> (usize, usize) {
        let of_layer = self.checks.iter().filter(|c| c.layer == layer);
        let total = of_layer.clone().count();
        (of_layer.filter(|c| c.matched()).count(), total)
    }

    /// How many differences were found: each field that differs in each
    /// case, and each case the port could not produce. What is explained,
    /// and a pull request that moved, are not counted.
    pub fn differences(&self) -> usize {
        self.checks
            .iter()
            .map(|check| match &check.outcome {
                Outcome::Matched
                | Outcome::Explained { .. }
                | Outcome::Moved
                | Outcome::NotCompared { .. } => 0,
                Outcome::Differs(found)
                | Outcome::Unexplained {
                    differences: found, ..
                } => found.len(),
                Outcome::Failed { .. } => 1,
            })
            .sum()
    }

    /// How many checks are explained by Python's own inputs.
    pub fn explained(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.outcome, Outcome::Explained { .. }))
            .count()
    }

    /// How many checks were not compared, as GitHub or the transport failed
    /// a read.
    pub fn not_compared(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| matches!(c.outcome, Outcome::NotCompared { .. }))
            .count()
    }

    /// Whether the port matched Python everywhere, or differed only where a
    /// difference is explained or a pull request moved, and needed no read
    /// the recording lacks.
    pub fn is_clean(&self) -> bool {
        self.missing_reads == 0 && self.checks.iter().all(Check::passed)
    }
}

fn field<'a>(value: &'a PyValue, key: &str) -> Option<&'a PyValue> {
    match value {
        PyValue::Dict(entries) => entries.get(key),
        _ => None,
    }
}

/// The pull requests the first batched history query named: the one
/// admission made, whose answers the first snapshots reuse.
pub fn collected(calls: &str) -> Vec<PyInt> {
    static ALIAS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"pr([0-9]+): pullRequest\(number:([0-9]+)\)")
            .expect("a constant pattern compiles")
    });
    for line in calls.split('\n').filter(|line| !line.is_empty()) {
        let Ok(entry) = py_loads(line) else {
            continue;
        };
        let (Some(PyValue::Str(kind)), Some(PyValue::Str(stdin))) =
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
        let Some(PyValue::Str(query)) = field(&document, "query") else {
            continue;
        };
        if !query.contains("fragment history") {
            continue;
        }
        return ALIAS
            .captures_iter(query)
            .filter_map(|found| PyInt::from_decimal(&found[2]))
            .collect();
    }
    Vec::new()
}

/// What stopped a read, by its class; a refusal by which counter moved.
pub(super) fn read_failure(
    error: &ReadError,
    transport: &ReplayTransport,
    before: (usize, usize),
) -> Failure {
    match error {
        ReadError::GitHub(_) => Failure::GitHubError,
        ReadError::Exception { class, .. } => Failure::Raised(*class),
        ReadError::NotPorted(_) => Failure::NotPorted,
        ReadError::Refused(_) if transport.missing() > before.0 => Failure::ReadNotRecorded,
        ReadError::Refused(_) if transport.exhausted() > before.1 => Failure::ReadAskedAgain,
        ReadError::Refused(_) => Failure::CallRefused,
    }
}

/// The policy an evaluation used: its own, or the recording's.
pub(super) fn policy_of<'a>(
    recording: &'a Recording,
    evaluation: &'a PyValue,
) -> Option<&'a PyValue> {
    field(evaluation, "policy").or_else(|| recording.policy())
}

/// Rebuild every snapshot Python's `evaluate` saw, from the recording's
/// reads alone, and compare each with Python's. Returns the checks and how
/// many reads the port needed that the recording does not hold.
pub fn rebuild_snapshots(recording: &Recording) -> (Vec<Check>, usize) {
    let every = |failure: Failure, detail: &str| {
        let checks = (0..recording.evaluations.len())
            .map(|index| Check::failed(Layer::Snapshot, index, failure, detail))
            .collect();
        (checks, 0)
    };
    let transport = match ReplayTransport::from_calls_jsonl(&recording.calls) {
        Ok(transport) => transport,
        Err(error) => return every(Failure::CallsUnreadable, &error.to_string()),
    };
    let Some(repository) = recording.repository() else {
        return every(Failure::Repository, "recording.json names no repository");
    };
    let mut api = match GitHub::new(repository, Client::with_sleep(transport, NoSleep)) {
        Ok(api) => api,
        Err(error) => return every(Failure::Repository, &error.to_string()),
    };
    let numbers = collected(&recording.calls);
    let histories = if numbers.is_empty() {
        Ok(Default::default())
    } else {
        api.histories(&numbers)
    };
    // The evaluations `evaluate_snapshots` made, one per verdict row, come
    // first. A run that raised inside it wrote no row, and made no later
    // evaluation either, since those come after it returns: then every
    // evaluation is one of the first.
    let first = if recording.verdicts.is_empty() {
        recording.evaluations.len()
    } else {
        recording.verdicts.len()
    };
    let mut checks = Vec::with_capacity(recording.evaluations.len());
    for (index, evaluation) in recording.evaluations.iter().enumerate() {
        let (Some(wanted), Some(policy)) =
            (field(evaluation, "pr"), policy_of(recording, evaluation))
        else {
            checks.push(Check::failed(
                Layer::Snapshot,
                index,
                Failure::Malformed,
                "no pr or no policy",
            ));
            continue;
        };
        let Some(PyValue::Int(number)) = field(wanted, "number") else {
            checks.push(Check::failed(
                Layer::Snapshot,
                index,
                Failure::NoNumber,
                "no pull request number",
            ));
            continue;
        };
        let transport = api.client().transport();
        let before = (transport.missing(), transport.exhausted());
        let read = if index < first {
            match &histories {
                Ok(histories) => api.snapshot(number, policy, histories.get(number)),
                Err(error) => {
                    checks.push(Check::failed(
                        Layer::Snapshot,
                        index,
                        Failure::HistoryUnread,
                        error.to_string(),
                    ));
                    continue;
                }
            }
        } else {
            api.forget_cached_access();
            api.snapshot(number, policy, None)
        };
        checks.push(match read {
            Ok(read) => Check::new(Layer::Snapshot, index, differences(&read, wanted, "pr")),
            Err(error) => {
                let failure = read_failure(&error, api.client().transport(), before);
                Check::failed(Layer::Snapshot, index, failure, error.to_string())
            }
        });
    }
    let transport = api.client().transport();
    (checks, transport.missing() + transport.exhausted())
}

/// The port's result against Python's, with the exclusion the evaluate-case
/// gate applies: where Python's `evaluate` answered with the text of a
/// Python exception as its first reason (`exception_text`), that reason is
/// set aside on both sides.
///
/// Whether it did is read from what `evaluate` returned, never from a
/// verdict row: a row whose head another pull request shares is made a
/// configuration error after `evaluate`, and its first reason is then the
/// engine's own words, whatever `python_exception_text` would make of it.
pub(super) fn compare_result(
    ours: &PyValue,
    python: &PyValue,
    root: &str,
    exception_text: bool,
    own: &OwnWords,
) -> Vec<Difference> {
    let mut found = Vec::new();
    let (mut ours, mut python) = (ours.clone(), python.clone());
    if exception_text {
        if let Err(problem) = set_aside_exception_text(&mut ours, &mut python, own) {
            found.push(Difference {
                path: format!("{root}.blockers"),
                kind: match problem {
                    ExceptionText::Absent => Kind::ExceptionTextAbsent,
                    ExceptionText::OwnWords => Kind::ExceptionTextOwnWords,
                },
                shape: None,
            });
        }
    }
    found.extend(differences(&ours, &python, root));
    found
}

fn class_of(error: &PyErr) -> Option<PyClass> {
    Some(match error {
        PyErr::Value(_) => PyClass::ValueError,
        PyErr::Type(_) => PyClass::TypeError,
        PyErr::Key(_) => PyClass::KeyError,
        PyErr::Recursion(_) => PyClass::RecursionError,
        PyErr::Attribute(_) => PyClass::AttributeError,
        PyErr::Overflow(_) => PyClass::OverflowError,
        PyErr::Unported(_) => return None,
    })
}

/// Run the port's `evaluate` on each evaluation's arguments and compare it
/// with Python's result. Returns the checks and the port's results, by
/// evaluation.
fn evaluations(recording: &Recording, own: &OwnWords) -> (Vec<Check>, Vec<Option<PyValue>>) {
    let mut checks = Vec::with_capacity(recording.evaluations.len());
    let mut results = Vec::with_capacity(recording.evaluations.len());
    for (index, evaluation) in recording.evaluations.iter().enumerate() {
        let arguments = (
            policy_of(recording, evaluation),
            field(evaluation, "pr"),
            field(evaluation, "admitted_at"),
            field(evaluation, "now"),
            field(evaluation, "telemetry_states"),
            field(evaluation, "result"),
        );
        let (Some(policy), Some(pr), Some(admitted_at), Some(now), Some(telemetry), Some(python)) =
            arguments
        else {
            checks.push(Check::failed(
                Layer::Evaluation,
                index,
                Failure::Malformed,
                "an argument or the result is absent",
            ));
            results.push(None);
            continue;
        };
        match evaluate(policy, pr, admitted_at, now, telemetry) {
            Ok(ours) => {
                let ours = PyValue::Dict(ours);
                checks.push(Check::new(
                    Layer::Evaluation,
                    index,
                    compare_result(
                        &ours,
                        python,
                        "result",
                        own.python_exception_text(python),
                        own,
                    ),
                ));
                results.push(Some(ours));
            }
            Err(error) => {
                let failure = match class_of(&error) {
                    Some(class) => Failure::EvaluateRaised(class),
                    None => Failure::EvaluateNotPorted,
                };
                checks.push(Check::failed(
                    Layer::Evaluation,
                    index,
                    failure,
                    error.to_string(),
                ));
                results.push(None);
            }
        }
    }
    (checks, results)
}

/// Whether `evaluate_snapshots` marked this row for a head another open
/// pull request shares: it appends that reason last, after `evaluate`.
pub(super) fn shares_head(row: &PyValue) -> bool {
    matches!(field(row, "blockers"), Some(PyValue::List(blockers))
        if matches!(blockers.last(), Some(PyValue::Str(last)) if last == SHARED_HEAD))
}

/// The row `evaluate_snapshots` builds from `evaluate`'s result: the
/// shared-head override where Python's row shows it was applied (which
/// heads are shared is the open listing's, not `evaluate`'s), then the
/// repository.
pub(super) fn verdict_row(result: &PyValue, policy: &PyValue, shared: bool) -> PyValue {
    let mut row = result.clone();
    if let PyValue::Dict(entries) = &mut row {
        if shared {
            for (key, value) in [
                ("state", PyValue::Str("configuration-error".into())),
                ("status", PyValue::Str("error".into())),
                ("reviewers", PyValue::List(Default::default())),
                ("objectors", PyValue::List(Default::default())),
                ("ready_since", PyValue::None),
            ] {
                entries.insert(key.into(), value);
            }
            if let Some(PyValue::List(blockers)) = entries.get_mut("blockers") {
                blockers.push(PyValue::Str(SHARED_HEAD.into()));
            }
        }
        let repository = field(policy, "repository").cloned();
        entries.insert("repository".into(), repository.unwrap_or(PyValue::None));
    }
    row
}

/// Compare each verdict row with the row the port's result for the same
/// evaluation makes.
fn verdicts(recording: &Recording, results: &[Option<PyValue>], own: &OwnWords) -> Vec<Check> {
    let mut checks = Vec::with_capacity(recording.verdicts.len());
    for (index, row) in recording.verdicts.iter().enumerate() {
        let Some(evaluation) = recording.evaluations.get(index) else {
            checks.push(Check::failed(
                Layer::Verdict,
                index,
                Failure::NoEvaluation,
                "fewer evaluations than verdicts",
            ));
            continue;
        };
        let (Some(Some(ours)), Some(policy)) =
            (results.get(index), policy_of(recording, evaluation))
        else {
            checks.push(Check::failed(
                Layer::Verdict,
                index,
                Failure::NoResult,
                "the port's evaluate gave no result",
            ));
            continue;
        };
        let ours = verdict_row(ours, policy, shares_head(row));
        // Decided from what Python's `evaluate` answered for this row,
        // before `evaluate_snapshots` touched it.
        let exception_text =
            field(evaluation, "result").is_some_and(|result| own.python_exception_text(result));
        checks.push(Check::new(
            Layer::Verdict,
            index,
            compare_result(&ours, row, "verdict", exception_text, own),
        ));
    }
    checks
}

/// Compare one recording: its snapshots, its evaluations, its verdicts.
pub fn compare(recording: &Recording, own: &OwnWords) -> Comparison {
    let (mut checks, missing_reads) = rebuild_snapshots(recording);
    let (evaluated, results) = evaluations(recording, own);
    checks.extend(evaluated);
    checks.extend(verdicts(recording, &results, own));
    Comparison {
        checks,
        missing_reads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pycompat::{py_dumps, py_loads};

    fn v(json: &str) -> PyValue {
        py_loads(json).unwrap()
    }

    fn written(value: &PyValue) -> String {
        py_dumps(value, false, Some((",", ":")), None).unwrap()
    }

    #[test]
    fn a_verdict_row_is_the_result_and_its_repository() {
        let result = v(r#"{"number": 1, "state": "ready-for-human", "blockers": []}"#);
        let row = verdict_row(&result, &v(r#"{"repository": "a/b"}"#), false);
        assert_eq!(
            written(&row),
            r#"{"number":1,"state":"ready-for-human","blockers":[],"repository":"a/b"}"#
        );
    }

    #[test]
    fn a_shared_head_overrides_the_verdict_where_python_marked_it_and_only_there() {
        let result = v(
            r#"{"state": "ready-for-human", "status": "success", "blockers": ["x"], "reviewers": ["r"], "objectors": [], "ready_since": "t"}"#,
        );
        let policy = v(r#"{"repository": "a/b"}"#);
        let row = verdict_row(&result, &policy, true);
        // `result.update(...)` keeps each key where it was; the reason goes
        // last; the repository after everything.
        assert_eq!(
            written(&row),
            format!(
                r#"{{"state":"configuration-error","status":"error","blockers":["x","{SHARED_HEAD}"],"reviewers":[],"objectors":[],"ready_since":null,"repository":"a/b"}}"#
            )
        );
        assert!(shares_head(&row));
        assert!(!shares_head(&result));
    }

    #[test]
    fn the_history_query_names_the_pull_requests_whose_answers_are_reused() {
        let calls = [
            r#"{"kind":"read","stdin":"{\"query\": \"query { pr3: pullRequest(number:3) { ...history } pr10: pullRequest(number:10) { ...history } } fragment history on PullRequest { number }\"}"}"#,
            r#"{"kind":"read","stdin":"{\"query\": \"query { pr7: pullRequest(number:7) { ...history } } fragment history on PullRequest { number }\"}"}"#,
        ]
        .join("\n");
        let numbers: Vec<String> = collected(&calls).iter().map(|n| n.to_string()).collect();
        assert_eq!(numbers, ["3", "10"]);
        assert!(collected("").is_empty());
    }
}
