//! The live half of the differential: the pull requests a recorded Python
//! run read, read again now through whatever transport the caller gives —
//! in the job, the service's HTTP transport behind its read-only layer —
//! and held to what Python read and decided.
//!
//! - **Snapshots.** Each pull request read live is compared with the `pr`
//!   Python's `evaluate` was given, exactly ([`live_snapshots`] for a few of
//!   a report's, [`live_run`] for a one-author sync's).
//! - **Verdicts.** A one-author sync is run whole, live, as the service will
//!   run it ([`live_run`]), and each verdict is compared with Python's row.
//! - **No write where nothing changed.** Where every pull request that run
//!   decided carries a record of the engine's whose evidence print is the
//!   print of what was just read, and Python's own run, minutes earlier,
//!   wrote nothing, the live run must want to write nothing either.
//! - **Explained differences.** A verdict, or a write, that differs is
//!   decided again from what was read live, with Python's own inputs in
//!   place of the port's: the instant it decided at, the review system's
//!   status page, the instant a pull request was admitted. A difference
//!   that then vanishes is [`Outcome::Explained`], by whichever of them it
//!   took. Admission is substituted only where both sides admitted the pull
//!   request and the admitted pull requests are the same, in the same
//!   order: only its instant may differ. Every other difference fails.
//! - **Moved during the read.** A pull request whose `updated_at` or head
//!   moved between Python's reads and the live ones, or whose head's checks
//!   or statuses answered otherwise — which GitHub records without moving
//!   the pull request's `updated_at`, as a build finishes — is
//!   [`Outcome::Moved`]: counted, and no failure.
//!
//! What a live run would write is never sent. [`Observed`] answers every
//! call that is not a read itself, as a failure, and never passes it on;
//! the caller's transport beneath it can refuse what is not a read by its
//! own rules too. Everything read is kept in memory for the comparison and
//! for the runs again with Python's inputs, which read nothing more, and
//! is never written anywhere.

use super::compare::{
    compare_result, policy_of, shares_head, verdict_row, Check, Comparison, Explanation, Failure,
    Layer, Outcome,
};
use super::diff::{differences, Difference, Kind};
use super::exception::OwnWords;
use super::recording::Recording;
use super::run::{replayed, synced_pr};
use crate::evidence::queries;
use crate::evidence::replay::{gh_arguments, is_read};
use crate::evidence::{
    Call, Client, Failure as Failed, GitHub, NoSleep, ReadError, Reply, Transport, TransportError,
};
use crate::policy::{admit, evaluate, fingerprint, validate_policy};
use crate::pycompat::{py_dumps, py_loads, PyDateTime, PyInt, PyList, PyValue};
use crate::reconcile::{
    telemetry_states, Clock, ClockSite, Command, Reconciler, Run, RunOptions, Selection,
};
use std::collections::{BTreeSet, HashMap};

/// The `gh api` command a call becomes: its arguments and its stdin. Python's
/// recording keeps each read under it, and the live log does too, so the
/// same read is found on both sides.
type CallKey = (Vec<String>, Option<String>);

/// The words of GitHub's API a would-be write's route is named with; every
/// other segment is data — an owner, a repository, a number, a head, a
/// label — and is written `*`.
const ROUTE_WORDS: [&str; 9] = [
    "repos",
    "pulls",
    "issues",
    "comments",
    "statuses",
    "labels",
    "requested_reviewers",
    "reviews",
    "commits",
];

/// How a would-be write is named: its method and its route, with every
/// segment that is not one of [`ROUTE_WORDS`] written `*` and no query.
pub fn write_kind(call: &Call) -> String {
    match call {
        Call::Graphql { .. } => "POST graphql".to_owned(),
        Call::Rest { method, path, .. } => {
            let route = path.split('?').next().unwrap_or_default();
            let segments: Vec<&str> = route
                .split('/')
                .map(|segment| {
                    if ROUTE_WORDS.contains(&segment) {
                        segment
                    } else {
                        "*"
                    }
                })
                .collect();
            format!("{method} {}", segments.join("/"))
        }
    }
}

/// One read a live run made, and its answer.
#[derive(Debug, Clone)]
struct Logged {
    key: CallKey,
    answer: Result<Reply, TransportError>,
}

/// A transport that keeps every read a live run made, counts the requests
/// they cost, and answers every call that is not a read — by Python's own
/// rule, [`is_read`] — itself, as a failure, without passing it on: a
/// would-be write, named by [`write_kind`]. So a run goes on past it as it
/// would past a write GitHub refused, and nothing is sent.
#[derive(Debug)]
pub struct Observed<T> {
    inner: T,
    log: Vec<Logged>,
    requests: usize,
    refused: usize,
    would_be: Vec<String>,
}

impl<T> Observed<T> {
    pub fn new(inner: T) -> Self {
        Observed {
            inner,
            log: Vec::new(),
            requests: 0,
            refused: 0,
            would_be: Vec::new(),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// The requests the reads cost: one per call, one per page of a listing.
    /// A call the transport beneath refused cost none.
    pub fn requests(&self) -> usize {
        self.requests
    }

    /// Reads the transport beneath refused to send.
    pub fn refused(&self) -> usize {
        self.refused
    }

    /// Every write the run would have made, in order, by its kind. None of
    /// them was sent.
    pub fn would_be(&self) -> &[String] {
        &self.would_be
    }
}

impl<T: Transport> Transport for Observed<T> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if !is_read(call) {
            self.would_be.push(write_kind(call));
            return Err(TransportError::Failed(Failed {
                transient: false,
                status: Some(1),
                body: String::new(),
                detail: "a write, not sent: this run only reads".to_owned(),
            }));
        }
        let answer = self.inner.call(call);
        self.requests += match &answer {
            // A listing that answered no page still asked once.
            Ok(Reply::Pages(pages)) if !pages.is_empty() => pages.len(),
            Ok(_) | Err(TransportError::Failed(_)) => 1,
            Err(TransportError::Refused(_)) => {
                self.refused += 1;
                0
            }
        };
        if let Ok(key) = gh_arguments(call) {
            self.log.push(Logged {
                key,
                answer: answer.clone(),
            });
        }
        answer
    }
}

/// The answers a live run got, given again in the order it got them, the
/// last again once they run out. A read the live run never made is
/// refused. What a run again with Python's inputs reads: GitHub exactly as
/// the live run saw it, at no request's cost.
struct Echo {
    answers: HashMap<CallKey, (Vec<Result<Reply, TransportError>>, usize)>,
}

impl Echo {
    fn of(log: &[Logged]) -> Self {
        let mut answers: HashMap<CallKey, (Vec<_>, usize)> = HashMap::new();
        for logged in log {
            answers
                .entry(logged.key.clone())
                .or_default()
                .0
                .push(logged.answer.clone());
        }
        Echo { answers }
    }
}

impl Transport for Echo {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        let key = gh_arguments(call).map_err(|error| TransportError::Refused(error.to_string()))?;
        let Some((answers, next)) = self.answers.get_mut(&key) else {
            return Err(TransportError::Refused(
                "a read the live run did not make".to_owned(),
            ));
        };
        let Some(answer) = answers.get(*next).or_else(|| answers.last()) else {
            return Err(TransportError::Refused(
                "a read the live run did not make".to_owned(),
            ));
        };
        *next += 1;
        answer.clone()
    }
}

/// A clock stopped at one instant.
struct Stopped(PyDateTime);

impl Clock for Stopped {
    fn now(&mut self, _site: ClockSite) -> PyDateTime {
        self.0
    }
}

/// What a live comparison of one recording found, and what it cost.
#[derive(Debug, Clone, Default)]
pub struct Live {
    pub comparison: Comparison,
    /// The requests the live reads made, one per page of a listing.
    pub requests: usize,
    /// The pull requests that moved between Python's reads and the live
    /// ones.
    pub moved: usize,
    /// Whether a live whole run was held to writing nothing: `Some(true)`
    /// where every pull request it decided carried a current record and
    /// Python's own run wrote nothing; `Some(false)` where not; `None`
    /// where no whole run was made.
    pub settled: Option<bool>,
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

fn same(a: &PyValue, b: &PyValue) -> bool {
    differences(a, b, "").is_empty()
}

/// What stopped a live read, by its class.
fn live_failure(error: &ReadError) -> Failure {
    match error {
        ReadError::GitHub(_) => Failure::GitHubError,
        ReadError::Exception { class, .. } => Failure::Raised(*class),
        ReadError::NotPorted(_) => Failure::NotPorted,
        ReadError::Refused(_) => Failure::ReadRefused,
    }
}

/// How many of a recording's evaluations `evaluate_snapshots` made: one per
/// verdict row, or every one where the run raised before returning any.
fn first_evaluations(recording: &Recording) -> usize {
    if recording.verdicts.is_empty() {
        recording.evaluations.len()
    } else {
        recording.verdicts.len().min(recording.evaluations.len())
    }
}

/// The number of the pull request an evaluation was given.
fn evaluated_number(evaluation: &PyValue) -> Option<PyInt> {
    match field(evaluation, "pr").and_then(|pr| field(pr, "number")) {
        Some(PyValue::Int(n)) => Some(n.clone()),
        _ => None,
    }
}

fn number_of(snapshot: &PyValue) -> Option<PyInt> {
    match field(snapshot, "number") {
        Some(PyValue::Int(n)) => Some(n.clone()),
        _ => None,
    }
}

fn head_of(snapshot: &PyValue) -> Option<&str> {
    match field(snapshot, "head") {
        Some(PyValue::Str(head)) => Some(head),
        _ => None,
    }
}

/// Python's answers to its reads, by `gh api` command: every answer `gh`
/// gave it successfully, in order, as the value it held.
fn recorded_answers(calls: &str) -> HashMap<CallKey, Vec<PyValue>> {
    let mut answers: HashMap<CallKey, Vec<PyValue>> = HashMap::new();
    for line in calls.split('\n').filter(|line| !line.is_empty()) {
        let Ok(entry) = py_loads(line) else {
            continue;
        };
        let read = matches!(field(&entry, "kind"), Some(PyValue::Str(kind)) if kind == "read");
        let succeeded = matches!(field(&entry, "exit"), Some(PyValue::Int(exit)) if exit.is_zero());
        let (Some(PyValue::List(args)), Some(PyValue::Str(stdout))) =
            (field(&entry, "args"), field(&entry, "stdout"))
        else {
            continue;
        };
        if !read || !succeeded {
            continue;
        }
        let args: Option<Vec<String>> = args
            .iter()
            .map(|arg| match arg {
                PyValue::Str(arg) => Some(arg.clone()),
                _ => None,
            })
            .collect();
        let stdin = match field(&entry, "stdin") {
            Some(PyValue::Str(stdin)) => Some(stdin.clone()),
            _ => None,
        };
        if let (Some(args), Ok(value)) = (args, py_loads(stdout)) {
            answers.entry((args, stdin)).or_default().push(value);
        }
    }
    answers
}

/// How many writes Python's recorded run made.
fn recorded_writes(calls: &str) -> usize {
    calls
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter(|line| {
            py_loads(line).is_ok_and(
                |entry| matches!(field(&entry, "kind"), Some(PyValue::Str(kind)) if kind != "read"),
            )
        })
        .count()
}

/// A live answer as the value it holds: a listing as its list of pages,
/// as `gh api --paginate --slurp` printed it for Python.
fn answer_value(answer: &Result<Reply, TransportError>) -> Option<PyValue> {
    match answer {
        Ok(Reply::Text(text)) => py_loads(text).ok(),
        Ok(Reply::Pages(pages)) => pages
            .iter()
            .map(|page| py_loads(page).ok())
            .collect::<Option<Vec<_>>>()
            .map(|pages| PyValue::List(PyList::from(pages))),
        Err(_) => None,
    }
}

/// The pull request's `updated_at` and head, as its own read answers them;
/// one it does not give is `null`, and differs from one it gives.
fn identity(pull: &PyValue) -> Option<String> {
    let updated = field(pull, "updated_at").cloned().unwrap_or(PyValue::None);
    let head = field(pull, "head")
        .and_then(|head| field(head, "sha"))
        .cloned()
        .unwrap_or(PyValue::None);
    let pair = PyValue::List(PyList::from(vec![updated, head]));
    py_dumps(&pair, false, None, None).ok()
}

/// Whether the read keyed `key` is the build query of pull request
/// `number`, any page of it.
fn is_build_of(key: &CallKey, number: &PyInt) -> bool {
    let Some(stdin) = &key.1 else {
        return false;
    };
    let Ok(document) = py_loads(stdin) else {
        return false;
    };
    let is_build =
        matches!(field(&document, "query"), Some(PyValue::Str(q)) if q == queries::BUILD);
    let of_number = matches!(
        field(&document, "variables").and_then(|v| field(v, "number")),
        Some(PyValue::Int(n)) if n == number
    );
    is_build && of_number
}

/// Whether pull request `number` moved between Python's reads and the live
/// ones: its `updated_at` or head as its own read answered them, or what
/// its head's checks (the build query) or statuses answered. `heads` are
/// the heads either side saw.
fn moved(
    repository: &str,
    number: &PyInt,
    heads: &[&str],
    recorded: &HashMap<CallKey, Vec<PyValue>>,
    log: &[Logged],
) -> bool {
    let pull: CallKey = (
        ["--method", "GET"]
            .iter()
            .map(|word| (*word).to_owned())
            .chain([format!("repos/{repository}/pulls/{number}")])
            .collect(),
        None,
    );
    let live_pulls = log
        .iter()
        .filter(|logged| logged.key == pull)
        .filter_map(|logged| answer_value(&logged.answer));
    let identities: BTreeSet<String> = recorded
        .get(&pull)
        .into_iter()
        .flatten()
        .cloned()
        .chain(live_pulls)
        .filter_map(|pull| identity(&pull))
        .collect();
    if identities.len() > 1 {
        return true;
    }
    let statuses: Vec<String> = heads
        .iter()
        .map(|head| format!("repos/{repository}/commits/{head}/statuses"))
        .collect();
    let of_head = |key: &CallKey| {
        key.0.get(2).is_some_and(|route| {
            statuses
                .iter()
                .any(|prefix| route.split('?').next() == Some(prefix.as_str()))
        })
    };
    for logged in log {
        if !(is_build_of(&logged.key, number) || of_head(&logged.key)) {
            continue;
        }
        let (Some(python), Some(live)) = (recorded.get(&logged.key), answer_value(&logged.answer))
        else {
            continue;
        };
        let agreed = python.windows(2).all(|pair| same(&pair[0], &pair[1]));
        if !agreed || !python.iter().any(|answer| same(answer, &live)) {
            return true;
        }
    }
    false
}

/// Up to `picks` of the pull requests a recording's run evaluated first,
/// rotating with `slot` so that each comes round, read now through
/// `transport` and compared with the `pr` Python's `evaluate` was given.
/// They are read as Python's first snapshots were: their comments and
/// latest transition from one batched history query, then each one's own
/// reads, with one collaborator listing between them.
pub fn live_snapshots<T: Transport>(
    recording: &Recording,
    picks: usize,
    slot: usize,
    transport: T,
) -> Live {
    let mut live = Live::default();
    let first = first_evaluations(recording);
    let count = picks.min(first);
    if count == 0 {
        return live;
    }
    let chosen: BTreeSet<usize> = (0..count)
        .map(|k| slot.wrapping_mul(count).wrapping_add(k) % first)
        .collect();
    let checks = &mut live.comparison.checks;
    let Some(repository) = recording.repository() else {
        for &index in &chosen {
            checks.push(Check::failed(
                Layer::LiveSnapshot,
                index,
                Failure::Repository,
                "no repository",
            ));
        }
        return live;
    };
    let mut api = match GitHub::new(repository, Client::new(Observed::new(transport))) {
        Ok(api) => api,
        Err(error) => {
            for &index in &chosen {
                checks.push(Check::failed(
                    Layer::LiveSnapshot,
                    index,
                    Failure::Repository,
                    error.to_string(),
                ));
            }
            return live;
        }
    };
    let mut wanted = Vec::new();
    for &index in &chosen {
        let evaluation = &recording.evaluations[index];
        match (
            evaluated_number(evaluation),
            field(evaluation, "pr"),
            policy_of(recording, evaluation),
        ) {
            (Some(number), Some(pr), Some(policy)) => wanted.push((index, number, pr, policy)),
            _ => checks.push(Check::failed(
                Layer::LiveSnapshot,
                index,
                Failure::Malformed,
                "no pull request number, pr or policy",
            )),
        }
    }
    let numbers: Vec<PyInt> = wanted.iter().map(|(_, n, _, _)| n.clone()).collect();
    let histories = api.histories(&numbers);
    let mut read = Vec::with_capacity(wanted.len());
    for (index, number, pr, policy) in &wanted {
        let snapshot = match &histories {
            Ok(histories) => api.snapshot(number, policy, histories.get(number)),
            Err(error) => Err(error.clone()),
        };
        read.push((*index, number, *pr, snapshot));
    }
    let observed = api.client().transport();
    let recorded = recorded_answers(&recording.calls);
    for (index, number, pr, snapshot) in read {
        let heads: Vec<&str> = [head_of(pr), snapshot.as_ref().ok().and_then(|s| head_of(s))]
            .into_iter()
            .flatten()
            .collect();
        if moved(repository, number, &heads, &recorded, &observed.log) {
            live.moved += 1;
            checks.push(Check {
                layer: Layer::LiveSnapshot,
                index,
                outcome: Outcome::Moved,
            });
            continue;
        }
        checks.push(match snapshot {
            Ok(snapshot) => {
                Check::new(Layer::LiveSnapshot, index, differences(&snapshot, pr, "pr"))
            }
            Err(error) => Check::failed(
                Layer::LiveSnapshot,
                index,
                live_failure(&error),
                error.to_string(),
            ),
        });
    }
    live.requests = observed.requests();
    live
}

/// What a live run was given and decided, beside its verdicts: what a
/// verdict is decided again from with Python's inputs.
struct Decided<'r> {
    recording: &'r Recording,
    policy: &'r PyValue,
    run: &'r Run,
    /// The status page the live run read, if it read one.
    payload: PyValue,
    own: &'r OwnWords,
}

impl Decided<'_> {
    /// The admitted pull requests, in the order `admit` gives them, and
    /// each one's instant, for `candidates` at `now`.
    fn admitted(&self, candidates: &[PyValue], now: &str) -> Option<Vec<(PyInt, PyValue)>> {
        let candidates = PyValue::List(PyList::from(candidates.to_vec()));
        let slots = admit(self.policy, &candidates, &s(now)).ok()?;
        Some(slots.into_iter().filter(|(_, at)| at.truthy()).collect())
    }

    /// Whether Python's run and the live one admitted the same pull
    /// requests, in the same order: only then can an admission's instant
    /// be all that differs.
    fn same_admissions(&self, python_now: &str) -> bool {
        let live = self.admitted(&self.run.candidates, &self.run.generated_at);
        let python = replayed(self.recording)
            .and_then(|python| self.admitted(&python.candidates, python_now));
        match (live, python) {
            (Some(live), Some(python)) => live
                .iter()
                .map(|(n, _)| n)
                .eq(python.iter().map(|(n, _)| n)),
            _ => false,
        }
    }

    /// The live verdict of `snapshot`, decided again with whichever of
    /// Python's inputs `by` names in place of the port's own, compared with
    /// Python's row: the differences left.
    #[allow(clippy::too_many_arguments)]
    fn again(
        &self,
        snapshot: &PyValue,
        verdict: &PyValue,
        python_row: &PyValue,
        python: &PyValue,
        live_admitted: &PyValue,
        exception_text: bool,
        by: Explanation,
    ) -> Option<Vec<Difference>> {
        let now = if by.clock {
            field(python, "now")?.clone()
        } else {
            s(self.run.generated_at.as_str())
        };
        let payload = if by.telemetry {
            field(&self.recording.meta, "telemetry")
                .cloned()
                .unwrap_or(PyValue::None)
        } else {
            self.payload.clone()
        };
        let admitted = if by.admission {
            field(python, "admitted_at")?.clone()
        } else {
            live_admitted.clone()
        };
        let PyValue::Str(now_text) = &now else {
            return None;
        };
        let states = telemetry_states(
            self.policy,
            std::slice::from_ref(snapshot),
            now_text,
            &payload,
        )
        .ok()?;
        let state = states
            .last()
            .map_or(PyValue::None, |(_, state)| state.clone());
        let result = evaluate(self.policy, snapshot, &admitted, &now, &state).ok()?;
        let row = verdict_row(&PyValue::Dict(result), self.policy, shares_head(verdict));
        Some(compare_result(
            &row,
            python_row,
            "verdict",
            exception_text,
            self.own,
        ))
    }

    /// Which of Python's inputs, the fewest that do, make the live verdict
    /// of `snapshot` Python's row; `None` when none do.
    fn explain_verdict(
        &self,
        snapshot: &PyValue,
        verdict: &PyValue,
        python_row: &PyValue,
        python: &PyValue,
        exception_text: bool,
    ) -> Option<Explanation> {
        let number = number_of(snapshot)?;
        let live_admitted = self
            .admitted(&self.run.candidates, &self.run.generated_at)?
            .into_iter()
            .find(|(n, _)| *n == number)
            .map_or(PyValue::None, |(_, at)| at);
        let python_now = match field(python, "now") {
            Some(PyValue::Str(now)) => now.clone(),
            _ => return None,
        };
        let python_admitted = field(python, "admitted_at").unwrap_or(&PyValue::None);
        let clock = python_now != self.run.generated_at;
        let telemetry = !same(
            field(&self.recording.meta, "telemetry").unwrap_or(&PyValue::None),
            &self.payload,
        );
        let admission = live_admitted.truthy()
            && python_admitted.truthy()
            && !same(&live_admitted, python_admitted)
            && self.same_admissions(&python_now);
        let available = Explanation {
            clock,
            telemetry,
            admission,
        };
        subsets(available).into_iter().find(|by| {
            self.again(
                snapshot,
                verdict,
                python_row,
                python,
                &live_admitted,
                exception_text,
                *by,
            )
            .is_some_and(|left| left.is_empty())
        })
    }
}

/// Every non-empty combination of what `available` holds, the smaller
/// first: the fewest of Python's inputs that explain a difference are the
/// ones it is put down to.
fn subsets(available: Explanation) -> Vec<Explanation> {
    let mut found = Vec::new();
    for bits in 1u8..8 {
        let by = Explanation {
            clock: bits & 1 != 0,
            telemetry: bits & 2 != 0,
            admission: bits & 4 != 0,
        };
        let fits = (!by.clock || available.clock)
            && (!by.telemetry || available.telemetry)
            && (!by.admission || available.admission);
        if fits {
            found.push(by);
        }
    }
    found.sort_by_key(|by| u8::from(by.clock) + u8::from(by.telemetry) + u8::from(by.admission));
    found
}

/// What a live one-author run is asked to do, as Python's recorded run
/// was: a dry `sync --pr N` that walks the write path.
fn sync_options<'s>(number: &PyInt, telemetry: &'s mut dyn FnMut() -> PyValue) -> RunOptions<'s> {
    RunOptions {
        command: Command::Sync,
        selection: Selection::Pr(number.clone()),
        apply: true,
        user: None,
        nudges: 1,
        telemetry,
    }
}

/// A recorded `sync --pr N` run again, live, through `transport`, at
/// `clock`, with the status page as `telemetry` reads it: the snapshots
/// and verdicts it reaches, held to Python's; and, where nothing has
/// changed since the engine last wrote and Python's own run wrote nothing,
/// no write at all. Every write it would make is answered by [`Observed`]
/// as GitHub refusing it, and never sent.
pub fn live_run<T: Transport>(
    recording: &Recording,
    transport: T,
    clock: &mut dyn Clock,
    telemetry: &mut dyn FnMut() -> PyValue,
    own: &OwnWords,
) -> Live {
    let mut live = Live::default();
    let stop = |mut live: Live, failure: Failure, detail: &str| {
        live.comparison
            .checks
            .push(Check::failed(Layer::LiveSnapshot, 0, failure, detail));
        live
    };
    let Some(number) = synced_pr(&recording.meta) else {
        return stop(live, Failure::Command, "not a recorded sync --pr");
    };
    let (Some(repository), Some(policy)) = (recording.repository(), recording.policy()) else {
        return stop(live, Failure::RunIncomplete, "no repository or policy");
    };
    let registered =
        matches!(field(policy, "repository"), Some(PyValue::Str(r)) if r == repository);
    if validate_policy(policy).is_err() || !registered {
        return stop(
            live,
            Failure::RunIncomplete,
            "a policy the run would refuse",
        );
    }
    let mut api = match GitHub::new(repository, Client::new(Observed::new(transport))) {
        Ok(api) => api,
        Err(error) => return stop(live, Failure::Repository, &error.to_string()),
    };
    let mut payload = PyValue::None;
    let mut read_page = || {
        payload = telemetry();
        payload.clone()
    };
    let outcome =
        Reconciler::new(&mut api, clock).run(policy, sync_options(&number, &mut read_page));
    let observed = api.client().transport();
    live.requests = observed.requests();
    let recorded = recorded_answers(&recording.calls);
    let first = first_evaluations(recording);
    let python: Vec<&PyValue> = recording.evaluations[..first].iter().collect();
    let python_numbers: Vec<Option<PyInt>> = python.iter().map(|e| evaluated_number(e)).collect();

    let run = match outcome {
        Ok(run) => run,
        Err(error) => {
            let heads: Vec<&str> = python
                .iter()
                .filter_map(|e| field(e, "pr").and_then(head_of))
                .collect();
            if moved(repository, &number, &heads, &recorded, &observed.log) {
                live.moved += 1;
                live.comparison.checks.push(Check {
                    layer: Layer::LiveSnapshot,
                    index: 0,
                    outcome: Outcome::Moved,
                });
                return live;
            }
            return stop(live, live_failure(&error), &error.to_string());
        }
    };
    let live_numbers: Vec<Option<PyInt>> = run.snapshots.iter().map(number_of).collect();
    // A pull request moved if either side saw it move, under either head.
    let mut moved_numbers = BTreeSet::new();
    for n in python_numbers.iter().chain(&live_numbers).flatten() {
        let heads: Vec<&str> = python
            .iter()
            .map(|e| field(e, "pr"))
            .chain(run.snapshots.iter().map(Some))
            .flatten()
            .filter(|pr| number_of(pr).as_ref() == Some(n))
            .filter_map(head_of)
            .collect();
        if moved(repository, n, &heads, &recorded, &observed.log) {
            moved_numbers.insert(n.clone());
        }
    }
    live.moved = moved_numbers.len();
    let checks = &mut live.comparison.checks;
    if python_numbers != live_numbers {
        checks.push(if moved_numbers.is_empty() {
            Check::failed(
                Layer::LiveSnapshot,
                0,
                Failure::OtherPullRequests,
                "the live run evaluated other pull requests",
            )
        } else {
            Check {
                layer: Layer::LiveSnapshot,
                index: 0,
                outcome: Outcome::Moved,
            }
        });
    }
    let decided = Decided {
        recording,
        policy,
        run: &run,
        payload: payload.clone(),
        own,
    };
    for (index, evaluation) in python.iter().enumerate() {
        let Some(number) = &python_numbers[index] else {
            continue;
        };
        let Some(at) = live_numbers.iter().position(|n| n.as_ref() == Some(number)) else {
            continue;
        };
        if moved_numbers.contains(number) {
            for layer in [Layer::LiveSnapshot, Layer::LiveVerdict] {
                checks.push(Check {
                    layer,
                    index,
                    outcome: Outcome::Moved,
                });
            }
            continue;
        }
        let (snapshot, verdict) = (&run.snapshots[at], &run.verdicts[at]);
        let wanted = field(evaluation, "pr").unwrap_or(&PyValue::None);
        checks.push(Check::new(
            Layer::LiveSnapshot,
            index,
            differences(snapshot, wanted, "pr"),
        ));
        let Some(python_row) = recording.verdicts.get(index) else {
            continue;
        };
        let exception_text =
            field(evaluation, "result").is_some_and(|result| own.python_exception_text(result));
        let found = compare_result(verdict, python_row, "verdict", exception_text, own);
        let outcome = if found.is_empty() {
            Outcome::Matched
        } else {
            match decided.explain_verdict(snapshot, verdict, python_row, evaluation, exception_text)
            {
                Some(by) => Outcome::Explained {
                    differences: found,
                    by,
                },
                None => Outcome::Differs(found),
            }
        };
        checks.push(Check {
            layer: Layer::LiveVerdict,
            index,
            outcome,
        });
    }

    // Nothing changed since the engine last wrote: every pull request the
    // run decided carries the engine's record, whose evidence print is the
    // print of what was just read; and Python's own run, minutes before,
    // wrote nothing.
    let current = !run.snapshots.is_empty()
        && run.snapshots.iter().all(|snapshot| {
            let recorded = field(snapshot, "controller_state").and_then(|r| field(r, "evidence"));
            matches!((recorded, fingerprint(snapshot)), (Some(PyValue::Str(was)), Ok(now)) if *was == now)
        });
    let settled = current && recorded_writes(&recording.calls) == 0;
    live.settled = Some(settled);
    if settled {
        let would_be = observed.would_be();
        let outcome = if would_be.is_empty() {
            Outcome::Matched
        } else if !moved_numbers.is_empty() {
            Outcome::Moved
        } else {
            let found: Vec<Difference> = would_be
                .iter()
                .map(|kind| Difference {
                    path: kind.clone(),
                    kind: Kind::WouldBeWrite,
                })
                .collect();
            match explain_writes(recording, policy, &number, &run, &payload, &observed.log) {
                Some(by) => Outcome::Explained {
                    differences: found,
                    by,
                },
                None => Outcome::Differs(found),
            }
        };
        checks.push(Check {
            layer: Layer::LiveWrites,
            index: 0,
            outcome,
        });
    }
    live
}

/// Which of Python's inputs, the fewest that do, make the live run want to
/// write nothing when it is run again over exactly what it read: Python's
/// clock, which also dates any new admission as Python dated it, and
/// Python's status page. `None` when none do.
fn explain_writes(
    recording: &Recording,
    policy: &PyValue,
    number: &PyInt,
    run: &Run,
    payload: &PyValue,
    log: &[Logged],
) -> Option<Explanation> {
    let parse = |text: &str| PyDateTime::fromisoformat(&text.replace('Z', "+00:00")).ok();
    let live_now = parse(&run.generated_at)?;
    let python_now = match field(&recording.meta, "clock") {
        Some(PyValue::Str(at)) => parse(at)?,
        _ => return None,
    };
    let python_payload = field(&recording.meta, "telemetry")
        .cloned()
        .unwrap_or(PyValue::None);
    let available = Explanation {
        clock: true,
        telemetry: !same(&python_payload, payload),
        admission: false,
    };
    subsets(available).into_iter().find(|by| {
        let mut clock = Stopped(if by.clock { python_now } else { live_now });
        let page = if by.telemetry {
            python_payload.clone()
        } else {
            payload.clone()
        };
        let mut telemetry = || page.clone();
        let Ok(mut api) = GitHub::new(
            recording.repository().unwrap_or_default(),
            Client::with_sleep(Observed::new(Echo::of(log)), NoSleep),
        ) else {
            return false;
        };
        let again =
            Reconciler::new(&mut api, &mut clock).run(policy, sync_options(number, &mut telemetry));
        again.is_ok() && api.client().transport().would_be().is_empty()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::{Method, Scripted};
    use crate::pycompat::PyDict;

    fn rest(method: Method, path: &str, body: Option<PyValue>) -> Call {
        Call::Rest {
            method,
            path: path.into(),
            body,
            paginate: false,
        }
    }

    #[test]
    fn a_write_is_answered_as_refused_and_never_passed_on() {
        let mut observed = Observed::new(Scripted::new([Ok(Reply::Text("{}".into()))]));
        let body = Some(PyValue::Dict(PyDict::new()));
        for call in [
            rest(
                Method::Post,
                "repos/dashpay/platform/statuses/abc123",
                body.clone(),
            ),
            rest(
                Method::Patch,
                "repos/dashpay/platform/pulls/7",
                body.clone(),
            ),
            rest(
                Method::Delete,
                "repos/dashpay/platform/issues/7/labels/ready",
                None,
            ),
            rest(Method::Get, "repos/dashpay/platform/pulls/7", body),
            Call::Graphql {
                query: "mutation { addComment(input: {}) { clientMutationId } }".into(),
                variables: PyValue::Dict(PyDict::new()),
            },
        ] {
            let answer = observed.call(&call);
            assert!(
                matches!(&answer, Err(TransportError::Failed(f)) if !f.transient && f.status == Some(1)),
                "{call}: {answer:?}"
            );
        }
        assert!(
            observed.inner().calls().is_empty(),
            "nothing reached the transport"
        );
        assert_eq!(observed.requests(), 0);
        assert_eq!(
            observed.would_be(),
            [
                "POST repos/*/*/statuses/*",
                "PATCH repos/*/*/pulls/*",
                "DELETE repos/*/*/issues/*/labels/*",
                "GET repos/*/*/pulls/*",
                "POST graphql",
            ]
        );
    }

    #[test]
    fn a_read_is_passed_on_logged_and_counted_by_the_page() {
        let mut observed = Observed::new(Scripted::new([
            Ok(Reply::Pages(vec!["[1]".into(), "[2]".into(), "[3]".into()])),
            Ok(Reply::Text("{}".into())),
            Err(TransportError::Refused("not a read".into())),
        ]));
        let listing = Call::Rest {
            method: Method::Get,
            path: "repos/a/b/pulls?per_page=100".into(),
            body: None,
            paginate: true,
        };
        assert!(observed.call(&listing).is_ok());
        assert!(observed
            .call(&rest(Method::Get, "repos/a/b/pulls/1", None))
            .is_ok());
        assert!(observed
            .call(&rest(Method::Get, "repos/a/b/pulls/2", None))
            .is_err());
        assert_eq!(
            observed.requests(),
            4,
            "three pages, one answer, a refusal free"
        );
        assert_eq!(observed.refused(), 1);
        assert_eq!(observed.log.len(), 3);
        assert!(observed.would_be().is_empty());
    }

    #[test]
    fn a_write_is_named_without_anything_that_is_data() {
        let call = rest(
            Method::Post,
            "repos/mallory/secret/issues/4818/labels?x=alice",
            None,
        );
        assert_eq!(write_kind(&call), "POST repos/*/*/issues/*/labels");
        let call = rest(
            Method::Delete,
            "repos/a/b/issues/7/labels/bot-review-skipped",
            None,
        );
        assert_eq!(write_kind(&call), "DELETE repos/*/*/issues/*/labels/*");
    }

    #[test]
    fn a_run_again_reads_only_what_the_live_run_read_the_last_answer_repeating() {
        let get = rest(Method::Get, "repos/a/b/pulls/1", None);
        let key = gh_arguments(&get).unwrap();
        let log = vec![
            Logged {
                key: key.clone(),
                answer: Ok(Reply::Text("1".into())),
            },
            Logged {
                key,
                answer: Ok(Reply::Text("2".into())),
            },
        ];
        let mut echo = Echo::of(&log);
        let answers: Vec<_> = (0..3).map(|_| echo.call(&get)).collect();
        assert_eq!(
            answers,
            [
                Ok(Reply::Text("1".into())),
                Ok(Reply::Text("2".into())),
                Ok(Reply::Text("2".into()))
            ]
        );
        assert!(matches!(
            echo.call(&rest(Method::Get, "repos/a/b/pulls/2", None)),
            Err(TransportError::Refused(_))
        ));
    }

    #[test]
    fn the_fewest_of_pythons_inputs_are_tried_first() {
        let all = Explanation {
            clock: true,
            telemetry: true,
            admission: true,
        };
        let tried: Vec<String> = subsets(all).iter().map(ToString::to_string).collect();
        assert_eq!(
            tried,
            [
                "clock",
                "status page",
                "admission",
                "clock and status page",
                "clock and admission",
                "status page and admission",
                "clock and status page and admission",
            ]
        );
        let clock = Explanation {
            clock: true,
            ..Explanation::default()
        };
        assert_eq!(subsets(clock), [clock]);
        assert!(subsets(Explanation::default()).is_empty());
    }

    fn calls(lines: &[&str]) -> String {
        lines.iter().map(|line| format!("{line}\n")).collect()
    }

    fn logged(path: &str, body: &str) -> Logged {
        Logged {
            key: gh_arguments(&rest(Method::Get, path, None)).unwrap(),
            answer: Ok(Reply::Text(body.into())),
        }
    }

    #[test]
    fn a_pull_request_moved_when_its_update_time_or_head_did() {
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1"], "stdin": null, "exit": 0, "stdout": "{\"updated_at\": \"t1\", \"head\": {\"sha\": \"h1\"}, \"base\": {\"repo\": {\"pushed_at\": \"x\"}}}", "stderr": ""}"#,
        ]));
        let n = PyInt::from(1);
        // The repository's own fields inside the answer churn; they are not
        // the pull request moving.
        let still = logged(
            "repos/a/b/pulls/1",
            r#"{"updated_at": "t1", "head": {"sha": "h1"}, "base": {"repo": {"pushed_at": "y"}}}"#,
        );
        assert!(!moved("a/b", &n, &["h1"], &recorded, &[still]));
        let updated = logged(
            "repos/a/b/pulls/1",
            r#"{"updated_at": "t2", "head": {"sha": "h1"}}"#,
        );
        assert!(moved("a/b", &n, &["h1"], &recorded, &[updated]));
        let pushed = logged(
            "repos/a/b/pulls/1",
            r#"{"updated_at": "t1", "head": {"sha": "h2"}}"#,
        );
        assert!(moved("a/b", &n, &["h1"], &recorded, &[pushed]));
    }

    #[test]
    fn a_pull_request_moved_when_its_heads_statuses_did_and_not_otherwise() {
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/commits/h1/statuses?per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 0, "stdout": "[[{\"id\": 1}]]", "stderr": ""}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1/files?per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 0, "stdout": "[[{\"filename\": \"a\"}]]", "stderr": ""}"#,
        ]));
        let n = PyInt::from(1);
        let listing = |path: &str, page: &str| Logged {
            key: gh_arguments(&Call::Rest {
                method: Method::Get,
                path: path.into(),
                body: None,
                paginate: true,
            })
            .unwrap(),
            answer: Ok(Reply::Pages(vec![page.into()])),
        };
        let same = listing(
            "repos/a/b/commits/h1/statuses?per_page=100",
            r#"[{"id": 1}]"#,
        );
        assert!(!moved("a/b", &n, &["h1"], &recorded, &[same]));
        let posted = listing(
            "repos/a/b/commits/h1/statuses?per_page=100",
            r#"[{"id": 2}, {"id": 1}]"#,
        );
        assert!(moved("a/b", &n, &["h1"], &recorded, &[posted]));
        // Any other read answering otherwise is a difference to report, not
        // a move to excuse.
        let files = listing(
            "repos/a/b/pulls/1/files?per_page=100",
            r#"[{"filename": "b"}]"#,
        );
        assert!(!moved("a/b", &n, &["h1"], &recorded, &[files]));
    }

    /// A synthetic recording, read from the corpus.
    fn synthetic(name: &str) -> Recording {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../conformance/synthetic")
            .join(name);
        Recording::load_with(|file| std::fs::read_to_string(dir.join(file))).unwrap()
    }

    /// GitHub as the recording saw it: each read answered as recorded, the
    /// last answer again once they run out.
    fn as_recorded(recording: &Recording) -> Echo {
        let mut log = Vec::new();
        for line in recording.calls.split('\n').filter(|l| !l.is_empty()) {
            let entry = py_loads(line).unwrap();
            if !matches!(field(&entry, "kind"), Some(PyValue::Str(k)) if k == "read") {
                continue;
            }
            let args = match field(&entry, "args") {
                Some(PyValue::List(args)) => args
                    .iter()
                    .map(|a| match a {
                        PyValue::Str(a) => a.clone(),
                        _ => panic!("an argument"),
                    })
                    .collect(),
                _ => panic!("args"),
            };
            let stdin = match field(&entry, "stdin") {
                Some(PyValue::Str(stdin)) => Some(stdin.clone()),
                _ => None,
            };
            let text = |key: &str| match field(&entry, key) {
                Some(PyValue::Str(text)) => text.clone(),
                _ => String::new(),
            };
            let answer = match field(&entry, "exit") {
                Some(PyValue::Int(exit)) if exit.is_zero() => Ok(Reply::Text(text("stdout"))),
                _ => Err(TransportError::Failed(Failed {
                    transient: false,
                    status: Some(1),
                    body: text("stdout"),
                    detail: text("stderr"),
                })),
            };
            log.push(Logged {
                key: (args, stdin),
                answer,
            });
        }
        Echo::of(&log)
    }

    fn own() -> OwnWords {
        let source = |name: &str| {
            std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../pr_review")
                    .join(name),
            )
            .unwrap()
        };
        OwnWords::from_sources(&source("conformance.py"), &source("policy.py")).unwrap()
    }

    fn outcomes(live: &Live) -> Vec<String> {
        live.comparison
            .checks
            .iter()
            .map(|check| {
                let what = match &check.outcome {
                    Outcome::Matched => "matched".to_owned(),
                    Outcome::Moved => "moved".to_owned(),
                    Outcome::Explained { differences, by } => format!(
                        "explained by {by}: {}",
                        differences
                            .iter()
                            .map(|d| d.path.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    Outcome::Differs(found) => format!(
                        "differs: {}",
                        found
                            .iter()
                            .map(|d| format!("{} {}", d.path, d.kind))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    Outcome::Failed { failure, detail } => format!("failed: {failure}: {detail}"),
                };
                format!("{} {}: {what}", check.layer.as_str(), check.index)
            })
            .collect()
    }

    fn at(instant: &str) -> Stopped {
        Stopped(PyDateTime::fromisoformat(instant).unwrap())
    }

    #[test]
    fn a_sync_read_as_recorded_at_the_recorded_instant_matches() {
        let recording = synthetic("sync-pr-2");
        let mut clock = at("2026-09-12T10:00:00+00:00");
        let live = live_run(
            &recording,
            as_recorded(&recording),
            &mut clock,
            &mut || PyValue::None,
            &own(),
        );
        assert_eq!(
            outcomes(&live),
            ["live snapshot 0: matched", "live verdict 0: matched"]
        );
        assert_eq!(live.settled, Some(false), "Python's own run wrote");
        assert!(live.comparison.is_clean());
    }

    #[test]
    fn a_sync_read_later_differs_by_its_admission_alone() {
        let recording = synthetic("sync-pr-2");
        let mut clock = at("2026-09-12T13:00:00+00:00");
        let live = live_run(
            &recording,
            as_recorded(&recording),
            &mut clock,
            &mut || PyValue::None,
            &own(),
        );
        // Python admitted it at its own instant, the live run at its own:
        // both admitted it, and only it, so the admission's instant is all
        // that differs, and Python's makes the verdicts one.
        assert_eq!(
            outcomes(&live),
            [
                "live snapshot 0: matched",
                "live verdict 0: explained by admission: verdict.admitted_at"
            ]
        );
        assert!(live.comparison.is_clean());
    }

    #[test]
    fn a_verdict_read_days_later_differs_by_the_clock_alone() {
        // A pull request already admitted: its waiting time runs from the
        // instant the verdict is decided at, so two days on it reads two
        // days more, and Python's instant gives Python's verdict back.
        let recording = synthetic("long-history");
        let mut clock = at("2026-09-14T13:00:00+00:00");
        let live = live_run(
            &recording,
            as_recorded(&recording),
            &mut clock,
            &mut || PyValue::None,
            &own(),
        );
        assert_eq!(
            outcomes(&live),
            [
                "live snapshot 0: matched",
                "live verdict 0: explained by clock: verdict.ready_since"
            ]
        );
    }

    #[test]
    fn a_recordings_writes_are_counted() {
        let text = calls(&[
            r#"{"kind": "read", "args": [], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": [], "exit": 0, "stdout": ""}"#,
        ]);
        assert_eq!(recorded_writes(&text), 1);
        assert_eq!(recorded_writes(""), 0);
    }
}
