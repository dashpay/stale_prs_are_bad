//! The live half of the differential: the pull requests a recorded Python
//! run read, read again now through whatever transport the caller gives —
//! in the job, the service's HTTP transport behind its read-only layer —
//! and held to what Python read and decided.
//!
//! - **Snapshots.** Each pull request read live is compared with the `pr`
//!   Python's `evaluate` was given, exactly ([`live_snapshots`] for a few of
//!   a report's, [`live_run`] for a sync's).
//! - **Verdicts.** A sync — of one pull request's author, or of every pull
//!   request — is run whole, live, as the service will run it
//!   ([`live_run`]), and each verdict is compared with Python's row.
//! - **No write where nothing changed.** Where Python's own run, minutes
//!   earlier, ran to its end and wrote nothing to a pull request both runs
//!   decided, the live run must want to write nothing to it either; and
//!   where it wrote nothing to any other pull request (a sweep tidying one
//!   the policy no longer governs), the live run must want to write nothing
//!   to any other either.
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
use super::run::{replayed, synced, Synced};
use crate::evidence::queries;
use crate::evidence::replay::{gh_arguments, is_read};
use crate::evidence::{
    Call, Client, Failure as Failed, GitHub, Method, NoSleep, ReadError, Reply, Transport,
    TransportError,
};
use crate::policy::{admit, evaluate, validate_policy, NUDGE_MARKER};
use crate::pycompat::{py_dumps, py_loads, PyDateTime, PyInt, PyList, PyValue};
use crate::reconcile::{
    telemetry_states, Clock, ClockSite, Command, Reconciler, Run, RunOptions, Selection,
};
use std::cell::OnceCell;
use std::collections::{BTreeSet, HashMap, HashSet};

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

/// What a write is aimed at, read from its route: a pull request by its
/// number, a head by its commit, a comment by its id. Kept to set a write
/// against the pull request it was for, and never printed.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    Number(PyInt),
    Head(String),
    Comment(PyInt),
    /// A route that names none of them, or a GraphQL mutation.
    Other,
}

/// What the write to `route` is aimed at.
fn target_of(route: &str) -> Target {
    let route = route.split('?').next().unwrap_or_default();
    let segments: Vec<&str> = route.split('/').collect();
    let by = |text: &str, make: fn(PyInt) -> Target| {
        PyInt::from_decimal(text).map_or(Target::Other, make)
    };
    match segments.as_slice() {
        ["repos", _, _, "statuses", head] => Target::Head((*head).to_owned()),
        ["repos", _, _, "issues", "comments", id, ..] => by(id, Target::Comment),
        ["repos", _, _, "issues" | "pulls", number, ..] => by(number, Target::Number),
        _ => Target::Other,
    }
}

/// One write a live run would have made.
#[derive(Debug, Clone)]
struct WouldBe {
    /// How it is named: [`write_kind`].
    kind: String,
    target: Target,
    /// Whether it asks a review bot to look at a head.
    nudge: bool,
}

impl WouldBe {
    fn of(call: &Call) -> Self {
        let (target, nudge) = match call {
            Call::Graphql { .. } => (Target::Other, false),
            Call::Rest {
                method, path, body, ..
            } => (
                target_of(path),
                *method == Method::Post && is_nudge(body.as_ref()),
            ),
        };
        WouldBe {
            kind: write_kind(call),
            target,
            nudge,
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
    would_be: Vec<WouldBe>,
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
    pub fn would_be(&self) -> Vec<&str> {
        self.would_be
            .iter()
            .map(|write| write.kind.as_str())
            .collect()
    }
}

impl<T: Transport> Transport for Observed<T> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if !is_read(call) {
            self.would_be.push(WouldBe::of(call));
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
    /// The pull requests both whole runs decided that the live one was not
    /// held to writing nothing to, as Python's own run wrote to them or did
    /// not run to its end. Each one held is a [`Layer::LiveWrites`] check.
    pub unsettled: usize,
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

/// One write Python's recorded run made: what it was aimed at, and whether
/// it asked a review bot to look at a head.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Recorded {
    target: Target,
    nudge: bool,
}

/// Whether a write's body is a nudge: a comment that opens with the
/// engine's nudge marker.
fn is_nudge(body: Option<&PyValue>) -> bool {
    matches!(
        body.and_then(|body| field(body, "body")),
        Some(PyValue::Str(text)) if text.starts_with(NUDGE_MARKER)
    )
}

/// Each write Python's recorded run made, in order: what it was aimed at,
/// read from the route among its `gh api` arguments, and whether its body
/// on stdin is a nudge.
fn recorded_writes(calls: &str) -> Vec<Recorded> {
    calls
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| py_loads(line).ok())
        .filter(|entry| !matches!(field(entry, "kind"), Some(PyValue::Str(kind)) if kind == "read"))
        .map(|entry| {
            let route = match field(&entry, "args") {
                Some(PyValue::List(args)) => args.iter().find_map(|arg| match arg {
                    PyValue::Str(arg) if arg.starts_with("repos/") => Some(arg.clone()),
                    _ => None,
                }),
                _ => None,
            };
            let body = match field(&entry, "stdin") {
                Some(PyValue::Str(stdin)) => py_loads(stdin).ok(),
                _ => None,
            };
            Recorded {
                target: route.map_or(Target::Other, |route| target_of(&route)),
                nudge: is_nudge(body.as_ref()),
            }
        })
        .collect()
}

/// The pull requests a run decided, by what a write can be aimed at.
struct Aims {
    numbers: HashSet<PyInt>,
    heads: HashMap<String, Vec<PyInt>>,
    comments: HashMap<PyInt, PyInt>,
}

impl Aims {
    fn of<'a>(snapshots: impl IntoIterator<Item = &'a PyValue>) -> Self {
        let mut aims = Aims {
            numbers: HashSet::new(),
            heads: HashMap::new(),
            comments: HashMap::new(),
        };
        for pr in snapshots {
            let Some(number) = number_of(pr) else {
                continue;
            };
            if let Some(head) = head_of(pr) {
                aims.heads
                    .entry(head.to_owned())
                    .or_default()
                    .push(number.clone());
            }
            if let Some(PyValue::List(comments)) = field(pr, "comments") {
                for comment in comments.iter() {
                    if let Some(PyValue::Int(id)) = field(comment, "id") {
                        aims.comments.insert(id.clone(), number.clone());
                    }
                }
            }
            aims.numbers.insert(number);
        }
        aims
    }

    /// The decided pull requests a write aimed at `target` was for; none
    /// for a write to any other.
    fn of_target(&self, target: &Target) -> Vec<PyInt> {
        match target {
            Target::Number(n) if self.numbers.contains(n) => vec![n.clone()],
            Target::Head(head) => self.heads.get(head).cloned().unwrap_or_default(),
            Target::Comment(id) => self.comments.get(id).cloned().into_iter().collect(),
            Target::Number(_) | Target::Other => Vec::new(),
        }
    }
}

/// Writes, set against the pull requests a run decided: each one's kinds
/// in order, and those aimed at none of them.
#[derive(Debug, Default)]
struct Aimed {
    by_pr: HashMap<PyInt, Vec<String>>,
    elsewhere: Vec<(Target, String)>,
}

impl Aimed {
    fn of<'w>(writes: impl IntoIterator<Item = (&'w Target, &'w str)>, aims: &Aims) -> Self {
        let mut aimed = Aimed::default();
        for (target, kind) in writes {
            let prs = aims.of_target(target);
            if prs.is_empty() {
                aimed.elsewhere.push((target.clone(), kind.to_owned()));
            }
            for n in prs {
                aimed.by_pr.entry(n).or_default().push(kind.to_owned());
            }
        }
        aimed
    }

    fn to(&self, n: &PyInt) -> &[String] {
        self.by_pr.get(n).map_or(&[], Vec::as_slice)
    }
}

/// Where Python's run spent its one nudge: the place, in the order it
/// decided its pull requests, of the one it asked a bot about; `None` where
/// it asked about none.
struct Nudged {
    order: HashMap<PyInt, usize>,
    spent_at: Option<usize>,
}

impl Nudged {
    fn of(python_numbers: &[Option<PyInt>], recorded: &[Recorded]) -> Self {
        let order: HashMap<PyInt, usize> = python_numbers
            .iter()
            .enumerate()
            .filter_map(|(at, n)| n.clone().map(|n| (n, at)))
            .collect();
        let spent_at = recorded
            .iter()
            .filter(|write| write.nudge)
            .find_map(|write| match &write.target {
                Target::Number(n) => order.get(n).copied(),
                _ => None,
            });
        Nudged { order, spent_at }
    }

    /// Whether a live write is held to Python's run: every one but a nudge
    /// to a pull request Python decided after the one it spent its nudge
    /// on. Python's run has one nudge and, once it is spent, asks about no
    /// other pull request. A live run's every write is refused, so its
    /// nudge is never spent, and every pull request that wants a bot asked
    /// is asked: those after Python's follow from the refusal, not from the
    /// port. Where Python's run asked about none, none wanted it, and every
    /// live nudge is held.
    fn holds(&self, write: &WouldBe) -> bool {
        let (true, Some(spent), Target::Number(n)) = (write.nudge, self.spent_at, &write.target)
        else {
            return true;
        };
        self.order.get(n).is_none_or(|at| *at <= spent)
    }
}

/// The would-be writes a live run is held to, by [`Nudged::holds`].
fn held_writes<'w>(
    would_be: &'w [WouldBe],
    nudged: &'w Nudged,
) -> impl Iterator<Item = (&'w Target, &'w str)> {
    would_be
        .iter()
        .filter(|write| nudged.holds(write))
        .map(|write| (&write.target, write.kind.as_str()))
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

/// What identifies a pull request's state as its own read, or a listing,
/// answers it: when it was last updated, its head, its base and the base's
/// commit, whether it is a draft, whether it is open. A field the answer
/// does not give is `null`, and differs from one it gives.
fn identity(pull: &PyValue) -> Option<String> {
    let at = |path: &[&str]| {
        path.iter()
            .try_fold(pull, |value, key| field(value, key))
            .cloned()
            .unwrap_or(PyValue::None)
    };
    let fields = vec![
        at(&["number"]),
        at(&["updated_at"]),
        at(&["head", "sha"]),
        at(&["base", "ref"]),
        at(&["base", "sha"]),
        at(&["draft"]),
        at(&["state"]),
    ];
    py_dumps(&PyValue::List(PyList::from(fields)), false, None, None).ok()
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

/// How a pull request changed between Python's reads and the live ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Motion {
    /// Not that either read shows.
    Still,
    /// Its head's checks or statuses answered otherwise: a build finished,
    /// a status was posted. GitHub records neither as an update of the pull
    /// request. Only what those answers feed is excused: the build, when
    /// the head was first seen, whether a verdict was published on it.
    Checks,
    /// Its own read answered another update time, head, base, draft or
    /// open state.
    Pull,
}

/// The fields of a snapshot read from its head's checks and statuses.
const CHECK_FIELDS: [&str; 3] = ["pr.build", "pr.head_seen_at", "pr.ready_published"];

/// How pull request `number` moved between Python's reads and the live
/// ones. `heads` are the heads either side saw.
fn motion(
    repository: &str,
    number: &PyInt,
    heads: &[&str],
    recorded: &HashMap<CallKey, Vec<PyValue>>,
    log: &[Logged],
) -> Motion {
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
        return Motion::Pull;
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
        if answered_otherwise(recorded.get(&logged.key), &logged.answer) {
            return Motion::Checks;
        }
    }
    Motion::Still
}

/// Whether a live answer is none of Python's to the same read, or Python's
/// own answers to it disagreed; never where Python has no answer to it.
fn answered_otherwise(python: Option<&Vec<PyValue>>, live: &Result<Reply, TransportError>) -> bool {
    let (Some(python), Some(live)) = (python, answer_value(live)) else {
        return false;
    };
    let agreed = python.windows(2).all(|pair| same(&pair[0], &pair[1]));
    !agreed || !python.iter().any(|answer| same(answer, &live))
}

/// One pull request as an answer to the open listing gave it.
struct Listed {
    /// Its author's login, lowercased as `admit` groups them.
    author: Option<String>,
    number: Option<PyInt>,
    head: Option<String>,
    /// What identifies its state: [`identity`].
    identity: String,
}

/// Every answer to the open listing, Python's and the live run's, each
/// read once into what tells whether a pull request in it moved.
struct Listings(Vec<Vec<Listed>>);

impl Listings {
    fn of(repository: &str, recorded: &HashMap<CallKey, Vec<PyValue>>, log: &[Logged]) -> Self {
        let listing: CallKey = (
            [
                "--method".to_owned(),
                "GET".to_owned(),
                format!("repos/{repository}/pulls?state=open&per_page=100"),
                "--paginate".to_owned(),
                "--slurp".to_owned(),
            ]
            .to_vec(),
            None,
        );
        let listed = |pages: &PyValue| -> Vec<Listed> {
            let PyValue::List(pages) = pages else {
                return Vec::new();
            };
            let mut found = Vec::new();
            for page in pages.iter() {
                let PyValue::List(page) = page else {
                    continue;
                };
                for pr in page.iter() {
                    let Some(identity) = identity(pr) else {
                        continue;
                    };
                    let login = field(pr, "user").and_then(|user| field(user, "login"));
                    let head = field(pr, "head").and_then(|head| field(head, "sha"));
                    found.push(Listed {
                        author: match login {
                            Some(PyValue::Str(login)) => Some(login.to_lowercase()),
                            _ => None,
                        },
                        number: number_of(pr),
                        head: match head {
                            Some(PyValue::Str(head)) => Some(head.clone()),
                            _ => None,
                        },
                        identity,
                    });
                }
            }
            found
        };
        let python = recorded.get(&listing).into_iter().flatten().map(listed);
        let live = log
            .iter()
            .filter(|logged| logged.key == listing)
            .filter_map(|logged| answer_value(&logged.answer))
            .map(|pages| listed(&pages));
        Listings(python.chain(live).collect())
    }

    /// Whether the open pull requests `which` picks moved between Python's
    /// reads and the live ones, as the listing answered: one of them
    /// opened, closed, pushed to, drafted or updated.
    fn moved(&self, which: impl Fn(&Listed) -> bool) -> bool {
        let seen: BTreeSet<BTreeSet<&str>> = self
            .0
            .iter()
            .map(|answer| {
                answer
                    .iter()
                    .filter(|pr| which(pr))
                    .map(|pr| pr.identity.as_str())
                    .collect()
            })
            .collect();
        seen.len() > 1
    }

    /// Whether the pull requests `author` has open moved. Which of them are
    /// reconciled together, and when each was admitted, follow from them.
    fn author_moved(&self, author: &str) -> bool {
        let author = author.to_lowercase();
        self.moved(|pr| pr.author.as_deref() == Some(author.as_str()))
    }

    /// Whether pull request `number` moved, as its line in the listing.
    fn number_moved(&self, number: &PyInt) -> bool {
        self.moved(|pr| pr.number.as_ref() == Some(number))
    }

    /// Whether the pull request a write to one neither run decided was
    /// aimed at moved: found by its number, by its head as the listing gave
    /// it, or by a comment on it the live run read (`owners`). Where it
    /// cannot be found, whether any open pull request moved (`any`).
    fn target_moved(&self, target: &Target, owners: &HashMap<PyInt, PyInt>, any: bool) -> bool {
        let number = match target {
            Target::Number(n) => Some(n.clone()),
            Target::Head(head) => self
                .0
                .iter()
                .flatten()
                .find(|pr| pr.head.as_deref() == Some(head.as_str()))
                .and_then(|pr| pr.number.clone()),
            Target::Comment(id) => owners.get(id).cloned(),
            Target::Other => None,
        };
        number.map_or(any, |n| self.number_moved(&n))
    }
}

/// The pull request each comment the live run read is on, by the comment's
/// id: from the history queries, a pull request's `number` over its
/// comments' `databaseId`s, and from a comment listing, the number in its
/// route over each comment's `id`.
fn comment_owners(log: &[Logged]) -> HashMap<PyInt, PyInt> {
    let mut owners = HashMap::new();
    for logged in log {
        let Some(answer) = answer_value(&logged.answer) else {
            continue;
        };
        if logged.key.1.is_some() {
            let pulls = field(&answer, "data").and_then(|data| field(data, "repository"));
            let Some(PyValue::Dict(pulls)) = pulls else {
                continue;
            };
            for pull in pulls.values() {
                let nodes = field(pull, "comments").and_then(|c| field(c, "nodes"));
                let (Some(number), Some(PyValue::List(nodes))) = (number_of(pull), nodes) else {
                    continue;
                };
                for node in nodes.iter() {
                    if let Some(PyValue::Int(id)) = field(node, "databaseId") {
                        owners.insert(id.clone(), number.clone());
                    }
                }
            }
            continue;
        }
        let route = logged.key.0.get(2).map(String::as_str).unwrap_or_default();
        let Target::Number(number) = target_of(route) else {
            continue;
        };
        if !route
            .split('?')
            .next()
            .unwrap_or_default()
            .ends_with("/comments")
        {
            continue;
        }
        let PyValue::List(pages) = answer else {
            continue;
        };
        for page in pages.iter() {
            let PyValue::List(comments) = page else {
                continue;
            };
            for comment in comments.iter() {
                if let Some(PyValue::Int(id)) = field(comment, "id") {
                    owners.insert(id.clone(), number.clone());
                }
            }
        }
    }
    owners
}

/// Whether a read failed on Python's side, every time Python asked it,
/// where the live run's same read was answered: GitHub failing Python's
/// read. Python's run passes over a pull request whose evidence it could
/// not read, so the two runs then decide different pull requests for a
/// reason that is neither engine's.
fn answered_where_python_failed(calls: &str, log: &[Logged]) -> bool {
    let mut python: HashMap<CallKey, bool> = HashMap::new();
    for line in calls.split('\n').filter(|line| !line.is_empty()) {
        let Ok(entry) = py_loads(line) else {
            continue;
        };
        if !matches!(field(&entry, "kind"), Some(PyValue::Str(kind)) if kind == "read") {
            continue;
        }
        let Some(PyValue::List(args)) = field(&entry, "args") else {
            continue;
        };
        let args: Option<Vec<String>> = args
            .iter()
            .map(|arg| match arg {
                PyValue::Str(arg) => Some(arg.clone()),
                _ => None,
            })
            .collect();
        let Some(args) = args else {
            continue;
        };
        let stdin = match field(&entry, "stdin") {
            Some(PyValue::Str(stdin)) => Some(stdin.clone()),
            _ => None,
        };
        let ok = matches!(field(&entry, "exit"), Some(PyValue::Int(exit)) if exit.is_zero());
        *python.entry((args, stdin)).or_default() |= ok;
    }
    log.iter().any(|logged| {
        logged.answer.is_ok() && python.get(&logged.key).is_some_and(|answered| !answered)
    })
}

/// Whether a read failed live, every time it was asked, where Python's
/// same read was answered: GitHub failing it, or the transport. Either
/// way what follows is not a comparison of the two engines.
fn failed_where_python_read(recorded: &HashMap<CallKey, Vec<PyValue>>, log: &[Logged]) -> bool {
    let mut answered: HashMap<&CallKey, bool> = HashMap::new();
    for logged in log {
        let ok = logged.answer.is_ok();
        *answered.entry(&logged.key).or_default() |= ok;
    }
    answered
        .into_iter()
        .any(|(key, ok)| !ok && recorded.contains_key(key))
}

/// A live snapshot against Python's, by how the pull request moved.
fn snapshot_check(
    index: usize,
    motion: Motion,
    snapshot: Result<PyValue, ReadError>,
    python: &PyValue,
) -> Check {
    let moved = Check {
        layer: Layer::LiveSnapshot,
        index,
        outcome: Outcome::Moved,
    };
    match (motion, snapshot) {
        (Motion::Pull, _) => moved,
        (_, Err(error)) => Check::failed(
            Layer::LiveSnapshot,
            index,
            live_failure(&error),
            error.to_string(),
        ),
        (Motion::Still, Ok(snapshot)) => Check::new(
            Layer::LiveSnapshot,
            index,
            differences(&snapshot, python, "pr"),
        ),
        (Motion::Checks, Ok(snapshot)) => {
            let left: Vec<Difference> = differences(&snapshot, python, "pr")
                .into_iter()
                .filter(|d| !CHECK_FIELDS.iter().any(|f| d.path.starts_with(f)))
                .collect();
            if left.is_empty() {
                moved
            } else {
                Check::new(Layer::LiveSnapshot, index, left)
            }
        }
    }
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
    live.requests = observed.requests();
    let recorded = recorded_answers(&recording.calls);
    if failed_where_python_read(&recorded, &observed.log) {
        checks.push(Check::failed(
            Layer::LiveSnapshot,
            0,
            Failure::LiveReadFailed,
            "a live read failed where Python's was answered",
        ));
        return live;
    }
    for (index, number, pr, snapshot) in read {
        let heads: Vec<&str> = [head_of(pr), snapshot.as_ref().ok().and_then(|s| head_of(s))]
            .into_iter()
            .flatten()
            .collect();
        let motion = motion(repository, number, &heads, &recorded, &observed.log);
        if motion != Motion::Still {
            live.moved += 1;
        }
        checks.push(snapshot_check(index, motion, snapshot, pr));
    }
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
    /// Python's run as the port replays it over Python's own reads, made
    /// once, when an admission is first asked about.
    python_run: OnceCell<Option<Run>>,
}

/// The candidates of one author, by login as `admit` groups them.
fn of_author(candidates: &[PyValue], author: &str) -> Vec<PyValue> {
    let author = author.to_lowercase();
    candidates
        .iter()
        .filter(
            |pr| matches!(field(pr, "author"), Some(PyValue::Str(a)) if a.to_lowercase() == author),
        )
        .cloned()
        .collect()
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
    /// requests of `author`, in the same order: only then can an
    /// admission's instant be all that differs. Admission is decided per
    /// author, so another author's pull requests have no part in it.
    fn same_admissions(&self, python_now: &str, author: &str) -> bool {
        let live = self.admitted(
            &of_author(&self.run.candidates, author),
            &self.run.generated_at,
        );
        let python = self
            .python_run
            .get_or_init(|| replayed(self.recording))
            .as_ref()
            .and_then(|python| self.admitted(&of_author(&python.candidates, author), python_now));
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
        let telemetry = pages_differ(&self.recording.meta, &self.payload);
        let admission = live_admitted.truthy()
            && python_admitted.truthy()
            && !same(&live_admitted, python_admitted)
            && matches!(field(snapshot, "author"), Some(PyValue::Str(author))
                if self.same_admissions(&python_now, author));
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

/// Whether Python's run and the live one both read the review system's
/// status page and read it otherwise: the only way the page can explain a
/// difference. A page read on one side only is no explanation, so a live
/// read of it that failed is never taken for the page having changed.
fn pages_differ(meta: &PyValue, live: &PyValue) -> bool {
    let python = field(meta, "telemetry").unwrap_or(&PyValue::None);
    python.truthy() && live.truthy() && !same(python, live)
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

/// What a live run is asked to do, as Python's recorded run was: a dry
/// sync, of one pull request's author or of every pull request, that walks
/// the write path.
fn sync_options<'s>(synced: &Synced, telemetry: &'s mut dyn FnMut() -> PyValue) -> RunOptions<'s> {
    RunOptions {
        command: Command::Sync,
        selection: match synced {
            Synced::All => Selection::All,
            Synced::Pr(number) => Selection::Pr(number.clone()),
        },
        apply: true,
        user: None,
        nudges: 1,
        telemetry,
    }
}

/// What is added to a would-be write's name when it was aimed at a pull
/// request neither run decided: a sweep tidying one the policy no longer
/// governs, or marking one whose evidence could not be read.
const ELSEWHERE: &str = "to a pull request neither run decided";

/// A recorded sync — `sync --pr N`, or `sync` of every pull request — run
/// again, live, through `transport`, at `clock`, with the status page as
/// `telemetry` reads it: the snapshots and verdicts it reaches, held to
/// Python's. Where Python's own run ran to its end, each pull request both
/// runs decided that it wrote nothing to is held to the live run writing
/// nothing to it either, and where it wrote to no other pull request, the
/// live run is held to that too. Every write it would make is answered by
/// [`Observed`] as GitHub refusing it, and never sent.
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
    let Some(synced) = synced(&recording.meta) else {
        return stop(
            live,
            Failure::Command,
            "not a recorded sync of one author or of every pull request",
        );
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
        Reconciler::new(&mut api, clock).run(policy, sync_options(&synced, &mut read_page));
    let observed = api.client().transport();
    live.requests = observed.requests();
    let recorded = recorded_answers(&recording.calls);
    let first = first_evaluations(recording);
    let python: Vec<&PyValue> = recording.evaluations[..first].iter().collect();
    let python_numbers: Vec<Option<PyInt>> = python.iter().map(|e| evaluated_number(e)).collect();
    let python_prs: Vec<&PyValue> = python.iter().filter_map(|e| field(e, "pr")).collect();

    let moved = |live: &mut Live, index: usize| {
        live.comparison.checks.push(Check {
            layer: Layer::LiveSnapshot,
            index,
            outcome: Outcome::Moved,
        });
    };
    if failed_where_python_read(&recorded, &observed.log) {
        return stop(
            live,
            Failure::LiveReadFailed,
            "a live read failed where Python's was answered",
        );
    }
    let listings = Listings::of(repository, &recorded, &observed.log);
    // Whether each author's open pull requests moved: which of them are
    // reconciled together, and each one's admission, follow from them.
    let mut authors: HashMap<String, bool> = HashMap::new();
    let mut author_of_moved = |pr: &PyValue| match field(pr, "author") {
        Some(PyValue::Str(author)) => *authors
            .entry(author.to_lowercase())
            .or_insert_with(|| listings.author_moved(author)),
        _ => false,
    };
    // Whether the open pull requests whose moving changes which the run
    // decides moved: the author's, for one author's run; anyone's, for a
    // run of every pull request.
    let scope_moved = match &synced {
        Synced::All => listings.moved(|_| true),
        Synced::Pr(_) => python_prs.first().is_some_and(|pr| author_of_moved(pr)),
    };
    let run = match outcome {
        Ok(run) => run,
        Err(error) => {
            let pr_moved = match &synced {
                Synced::Pr(number) => {
                    let heads: Vec<&str> = python_prs.iter().filter_map(|pr| head_of(pr)).collect();
                    motion(repository, number, &heads, &recorded, &observed.log) != Motion::Still
                }
                Synced::All => false,
            };
            if scope_moved || pr_moved {
                live.moved += 1;
                moved(&mut live, 0);
                return live;
            }
            return stop(live, live_failure(&error), &error.to_string());
        }
    };
    let live_numbers: Vec<Option<PyInt>> = run.snapshots.iter().map(number_of).collect();
    // How each pull request either side decided moved, under either head.
    let mut motions: HashMap<PyInt, Motion> = HashMap::new();
    for n in python_numbers.iter().chain(&live_numbers).flatten() {
        if motions.contains_key(n) {
            continue;
        }
        let heads: Vec<&str> = python_prs
            .iter()
            .copied()
            .chain(&run.snapshots)
            .filter(|pr| number_of(pr).as_ref() == Some(n))
            .filter_map(head_of)
            .collect();
        motions.insert(
            n.clone(),
            motion(repository, n, &heads, &recorded, &observed.log),
        );
    }
    live.moved = motions.values().filter(|m| **m != Motion::Still).count();
    let any_moved = scope_moved || live.moved > 0;
    if python_numbers != live_numbers {
        // A pull request Python could not read, and so did not decide, is
        // GitHub answering otherwise between the two reads, as a move is.
        if any_moved || answered_where_python_failed(&recording.calls, &observed.log) {
            moved(&mut live, 0);
        } else {
            live.comparison.checks.push(Check::failed(
                Layer::LiveSnapshot,
                0,
                Failure::OtherPullRequests,
                "the live run evaluated other pull requests",
            ));
        }
    }
    let decided = Decided {
        recording,
        policy,
        run: &run,
        payload: payload.clone(),
        own,
        python_run: OnceCell::new(),
    };
    // Where each pull request Python decided stands in the live run.
    let live_at: HashMap<&PyInt, usize> = live_numbers
        .iter()
        .enumerate()
        .filter_map(|(at, n)| n.as_ref().map(|n| (n, at)))
        .collect();
    // A pull request moved when it did, or when its author's others did: a
    // verdict, and what a run writes, follow from them too.
    let mut pr_moved = |number: &PyInt, pr: &PyValue| {
        motions.get(number).is_some_and(|m| *m != Motion::Still) || author_of_moved(pr)
    };
    let checks = &mut live.comparison.checks;
    for (index, evaluation) in python.iter().enumerate() {
        let Some(number) = &python_numbers[index] else {
            continue;
        };
        let Some(&at) = live_at.get(number) else {
            continue;
        };
        let motion = motions.get(number).copied().unwrap_or(Motion::Still);
        let (snapshot, verdict) = (&run.snapshots[at], &run.verdicts[at]);
        let wanted = field(evaluation, "pr").unwrap_or(&PyValue::None);
        checks.push(snapshot_check(index, motion, Ok(snapshot.clone()), wanted));
        let Some(python_row) = recording.verdicts.get(index) else {
            continue;
        };
        if pr_moved(number, wanted) {
            checks.push(Check {
                layer: Layer::LiveVerdict,
                index,
                outcome: Outcome::Moved,
            });
            continue;
        }
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

    // Nothing to write where nothing changed: a pull request both runs
    // decided that Python's own run, minutes before, ran to its end and
    // wrote nothing to. The evidence print in the engine's record is not
    // asked to be current: the engine rewrites the record only when its
    // state, head, admission or ready time would change, so a conversation
    // that goes on without changing the verdict leaves the print behind,
    // and requiring it left most runs unjudged.
    //
    // A write is set against the pull request its route names, by number,
    // by head or by comment. The live run went on past each write as if
    // GitHub had refused it; within one pull request's turn anything after
    // its first write may follow from that refusal, so the first is the one
    // named.
    let returned = field(&recording.meta, "outcome")
        .and_then(|outcome| field(outcome, "returned"))
        .is_some();
    let python_writes = recorded_writes(&recording.calls);
    let python_aimed = Aimed::of(
        python_writes.iter().map(|write| (&write.target, "")),
        &Aims::of(python_prs.iter().copied()),
    );
    let nudged = Nudged::of(&python_numbers, &python_writes);
    let live_aimed = Aimed::of(
        held_writes(&observed.would_be, &nudged),
        &Aims::of(&run.snapshots),
    );
    let mut again = Again::new(
        recording,
        policy,
        &synced,
        &run,
        &payload,
        &observed.log,
        &nudged,
    );
    for (index, number) in python_numbers.iter().enumerate() {
        let Some(number) = number else {
            continue;
        };
        if !live_at.contains_key(number) {
            continue;
        }
        if !returned || !python_aimed.to(number).is_empty() {
            live.unsettled += 1;
            continue;
        }
        let wanted = python_prs
            .iter()
            .find(|pr| number_of(pr).as_ref() == Some(number))
            .copied()
            .unwrap_or(&PyValue::None);
        let outcome = match live_aimed.to(number).first() {
            None => Outcome::Matched,
            Some(_) if pr_moved(number, wanted) => Outcome::Moved,
            Some(kind) => {
                let found = vec![Difference {
                    path: kind.clone(),
                    kind: Kind::WouldBeWrite,
                }];
                match again.explain(|aims, aimed| {
                    aims.numbers.contains(number) && aimed.to(number).is_empty()
                }) {
                    Some(by) => Outcome::Explained {
                        differences: found,
                        by,
                    },
                    None => Outcome::Differs(found),
                }
            }
        };
        live.comparison.checks.push(Check {
            layer: Layer::LiveWrites,
            index,
            outcome,
        });
    }
    // A write to a pull request neither run decided, where Python's run
    // wrote to none: it moved when its own line in the open listing did,
    // found by its number, its head or a comment on it the live run read;
    // where it cannot be found, when anything did.
    if returned && python_aimed.elsewhere.is_empty() && !live_aimed.elsewhere.is_empty() {
        let owners = comment_owners(&observed.log);
        let unmoved = live_aimed
            .elsewhere
            .iter()
            .find(|(target, _)| !listings.target_moved(target, &owners, any_moved));
        let outcome = match unmoved {
            None => Outcome::Moved,
            Some((_, kind)) => {
                let found = vec![Difference {
                    path: format!("{kind}, {ELSEWHERE}"),
                    kind: Kind::WouldBeWrite,
                }];
                match again.explain(|_, aimed| aimed.elsewhere.is_empty()) {
                    Some(by) => Outcome::Explained {
                        differences: found,
                        by,
                    },
                    None => Outcome::Differs(found),
                }
            }
        };
        live.comparison.checks.push(Check {
            layer: Layer::LiveWrites,
            index: 0,
            outcome,
        });
    }
    live
}

/// The live run, run again over exactly the answers it got, at no
/// request's cost, with Python's clock — which also dates any new admission
/// as Python dated it — or Python's status page, or both, in place of its
/// own: what it would write then. Each is run at most once, when a
/// difference first asks for it.
struct Again<'a> {
    recording: &'a Recording,
    policy: &'a PyValue,
    synced: &'a Synced,
    log: &'a [Logged],
    /// The status page the live run read.
    payload: &'a PyValue,
    live_now: Option<PyDateTime>,
    python_now: Option<PyDateTime>,
    /// Which of its writes are held, as the live run's are.
    nudged: &'a Nudged,
    tried: Vec<(Explanation, Option<(Aims, Aimed)>)>,
}

impl<'a> Again<'a> {
    fn new(
        recording: &'a Recording,
        policy: &'a PyValue,
        synced: &'a Synced,
        run: &Run,
        payload: &'a PyValue,
        log: &'a [Logged],
        nudged: &'a Nudged,
    ) -> Self {
        let parse = |text: &str| PyDateTime::fromisoformat(&text.replace('Z', "+00:00")).ok();
        let python_now = match field(&recording.meta, "clock") {
            Some(PyValue::Str(at)) => parse(at),
            _ => None,
        };
        Again {
            recording,
            policy,
            synced,
            log,
            payload,
            live_now: parse(&run.generated_at),
            python_now,
            nudged,
            tried: Vec::new(),
        }
    }

    /// The writes of the run again with `by` of Python's inputs, set
    /// against the pull requests it decided; `None` where it could not run
    /// or could not decide.
    fn writes(&self, by: Explanation) -> Option<(Aims, Aimed)> {
        let instant = if by.clock {
            self.python_now?
        } else {
            self.live_now?
        };
        let mut clock = Stopped(instant);
        let page = if by.telemetry {
            field(&self.recording.meta, "telemetry")
                .cloned()
                .unwrap_or(PyValue::None)
        } else {
            self.payload.clone()
        };
        let mut telemetry = || page.clone();
        let mut api = GitHub::new(
            self.recording.repository()?,
            Client::with_sleep(Observed::new(Echo::of(self.log)), NoSleep),
        )
        .ok()?;
        let again = Reconciler::new(&mut api, &mut clock)
            .run(self.policy, sync_options(self.synced, &mut telemetry))
            .ok()?;
        let aims = Aims::of(&again.snapshots);
        let aimed = Aimed::of(
            held_writes(&api.client().transport().would_be, self.nudged),
            &aims,
        );
        Some((aims, aimed))
    }

    /// The fewest of Python's inputs under which the run again is
    /// `settled`; `None` when none make it so.
    fn explain(&mut self, settled: impl Fn(&Aims, &Aimed) -> bool) -> Option<Explanation> {
        let available = Explanation {
            clock: true,
            telemetry: pages_differ(&self.recording.meta, self.payload),
            admission: false,
        };
        for by in subsets(available) {
            let at = match self.tried.iter().position(|(tried, _)| *tried == by) {
                Some(at) => at,
                None => {
                    let writes = self.writes(by);
                    self.tried.push((by, writes));
                    self.tried.len() - 1
                }
            };
            if let (_, Some((aims, aimed))) = &self.tried[at] {
                if settled(aims, aimed) {
                    return Some(by);
                }
            }
        }
        None
    }
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
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[still]),
            Motion::Still
        );
        let updated = logged(
            "repos/a/b/pulls/1",
            r#"{"updated_at": "t2", "head": {"sha": "h1"}}"#,
        );
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[updated]),
            Motion::Pull
        );
        let pushed = logged(
            "repos/a/b/pulls/1",
            r#"{"updated_at": "t1", "head": {"sha": "h2"}}"#,
        );
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[pushed]),
            Motion::Pull
        );
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
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[same]),
            Motion::Still
        );
        let posted = listing(
            "repos/a/b/commits/h1/statuses?per_page=100",
            r#"[{"id": 2}, {"id": 1}]"#,
        );
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[posted]),
            Motion::Checks
        );
        // Any other read answering otherwise is a difference to report, not
        // a move to excuse.
        let files = listing(
            "repos/a/b/pulls/1/files?per_page=100",
            r#"[{"filename": "b"}]"#,
        );
        assert_eq!(
            motion("a/b", &n, &["h1"], &recorded, &[files]),
            Motion::Still
        );
    }

    #[test]
    fn a_pull_request_moved_when_its_base_or_draft_did() {
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1"], "stdin": null, "exit": 0, "stdout": "{\"updated_at\": \"t1\", \"head\": {\"sha\": \"h1\"}, \"base\": {\"ref\": \"dev\", \"sha\": \"b1\"}, \"draft\": false}", "stderr": ""}"#,
        ]));
        let n = PyInt::from(1);
        for (live, moved) in [
            (
                r#"{"updated_at": "t1", "head": {"sha": "h1"}, "base": {"ref": "dev", "sha": "b1"}, "draft": false}"#,
                Motion::Still,
            ),
            (
                r#"{"updated_at": "t1", "head": {"sha": "h1"}, "base": {"ref": "dev", "sha": "b2"}, "draft": false}"#,
                Motion::Pull,
            ),
            (
                r#"{"updated_at": "t1", "head": {"sha": "h1"}, "base": {"ref": "dev", "sha": "b1"}, "draft": true}"#,
                Motion::Pull,
            ),
        ] {
            let read = logged("repos/a/b/pulls/1", live);
            assert_eq!(
                motion("a/b", &n, &["h1"], &recorded, &[read]),
                moved,
                "{live}"
            );
        }
    }

    #[test]
    fn an_authors_open_pull_requests_moved_when_any_of_theirs_did_and_only_theirs() {
        let listing = |pages: &str| Logged {
            key: gh_arguments(&Call::Rest {
                method: Method::Get,
                path: "repos/a/b/pulls?state=open&per_page=100".into(),
                body: None,
                paginate: true,
            })
            .unwrap(),
            answer: Ok(Reply::Pages(vec![pages.into()])),
        };
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls?state=open&per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 0, "stdout": "[[{\"number\": 1, \"user\": {\"login\": \"Alice\"}, \"updated_at\": \"t1\"}, {\"number\": 2, \"user\": {\"login\": \"bob\"}, \"updated_at\": \"t1\"}]]", "stderr": ""}"#,
        ]));
        let read = |live: Logged| Listings::of("a/b", &recorded, &[live]);
        let same = read(listing(
            r#"[{"number": 1, "user": {"login": "Alice"}, "updated_at": "t1"}, {"number": 2, "user": {"login": "bob"}, "updated_at": "t9"}]"#,
        ));
        assert!(!same.author_moved("alice"), "bob's is not hers");
        assert!(same.author_moved("bob"));
        assert!(same.moved(|_| true), "anyone's did");
        assert!(!same.number_moved(&PyInt::from(1)));
        assert!(same.number_moved(&PyInt::from(2)));
        let opened = read(listing(
            r#"[{"number": 1, "user": {"login": "Alice"}, "updated_at": "t1"}, {"number": 3, "user": {"login": "alice"}, "updated_at": "t2"}]"#,
        ));
        assert!(opened.author_moved("alice"));
        let closed = read(listing(
            r#"[{"number": 2, "user": {"login": "bob"}, "updated_at": "t1"}]"#,
        ));
        assert!(closed.author_moved("alice"));
        assert!(closed.number_moved(&PyInt::from(1)), "closed: gone from it");
        assert!(!closed.number_moved(&PyInt::from(2)));
    }

    #[test]
    fn checks_that_moved_excuse_only_what_they_feed() {
        let python =
            py_loads(r#"{"number": 1, "build": "running", "head_seen_at": null, "draft": false}"#)
                .unwrap();
        let built =
            py_loads(r#"{"number": 1, "build": "green", "head_seen_at": "t", "draft": false}"#)
                .unwrap();
        let check = snapshot_check(0, Motion::Checks, Ok(built), &python);
        assert!(matches!(check.outcome, Outcome::Moved), "{check:?}");
        let drafted =
            py_loads(r#"{"number": 1, "build": "green", "head_seen_at": null, "draft": true}"#)
                .unwrap();
        let check = snapshot_check(0, Motion::Checks, Ok(drafted), &python);
        let Outcome::Differs(left) = check.outcome else {
            panic!("{check:?}")
        };
        assert_eq!(
            left.iter().map(|d| d.path.as_str()).collect::<Vec<_>>(),
            ["pr.draft"],
            "the build is excused, the rest is not"
        );
    }

    #[test]
    fn a_read_that_failed_live_where_python_was_answered_is_named() {
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1"], "stdin": null, "exit": 0, "stdout": "{}", "stderr": ""}"#,
        ]));
        let key = gh_arguments(&rest(Method::Get, "repos/a/b/pulls/1", None)).unwrap();
        let failed = || Logged {
            key: key.clone(),
            answer: Err(TransportError::Failed(Failed::unavailable())),
        };
        assert!(failed_where_python_read(&recorded, &[failed(), failed()]));
        // Failed once and answered on the retry: a read like any other.
        let answered = Logged {
            key: key.clone(),
            answer: Ok(Reply::Text("{}".into())),
        };
        assert!(!failed_where_python_read(&recorded, &[failed(), answered]));
        // Failed where Python's failed too: nothing to set against it.
        let other = Logged {
            key: gh_arguments(&rest(Method::Get, "repos/a/b/labels/x", None)).unwrap(),
            answer: Err(TransportError::Failed(Failed::unavailable())),
        };
        assert!(!failed_where_python_read(&recorded, &[other]));
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
        served(&recording.calls)
    }

    /// GitHub answering each read of `calls` as recorded there.
    fn served(calls: &str) -> Echo {
        let mut log = Vec::new();
        for line in calls.split('\n').filter(|l| !l.is_empty()) {
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
        assert_eq!(live.unsettled, 1, "Python's own run wrote to it");
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

    /// `calls` without the calls of the given ordinals: as if Python's run
    /// had not made those writes.
    fn without(calls: &str, ordinals: &[i64]) -> String {
        calls
            .lines()
            .filter(|line| {
                let entry = py_loads(line).unwrap();
                !ordinals.iter().any(|ordinal| {
                    matches!(field(&entry, "ordinal"), Some(PyValue::Int(n)) if *n == PyInt::from(*ordinal))
                })
            })
            .map(|line| format!("{line}\n"))
            .collect()
    }

    /// `recording` with its calls replaced.
    fn with_calls(recording: &Recording, calls: String) -> Recording {
        Recording {
            calls,
            ..recording.clone()
        }
    }

    // The synthetic sweep is a dry `sync` of every pull request: 2 (ready,
    // by `reviewer`) and 5 (a draft, by `drafter`) decided; 1 unreadable,
    // marked so; 3 and 4 no longer governed, their marks taken off. Its
    // writes: 21 marks 1; 36 to 45 and 58 are 2's; 61, 64 and 67 are 5's;
    // 69 to 77 take 3's and 4's marks off.
    const TO_ONE: [i64; 1] = [21];
    const TO_FIVE: [i64; 3] = [61, 64, 67];
    const TO_THREE_AND_FOUR: [i64; 6] = [69, 70, 71, 73, 76, 77];

    fn sweep_at_its_instant(recording: &Recording, served: Echo) -> Live {
        live_run(
            recording,
            served,
            &mut at("2026-09-12T10:00:00+00:00"),
            &mut || PyValue::None,
            &own(),
        )
    }

    #[test]
    fn a_sync_of_every_pull_request_is_compared_pull_request_by_pull_request() {
        let recording = synthetic("sweep");
        let live = sweep_at_its_instant(&recording, as_recorded(&recording));
        assert_eq!(
            outcomes(&live),
            [
                "live snapshot 0: matched",
                "live verdict 0: matched",
                "live snapshot 1: matched",
                "live verdict 1: matched",
            ]
        );
        // Python's run wrote to both, and to pull requests it did not
        // decide: nothing is held to writing nothing.
        assert_eq!(live.unsettled, 2);
        assert!(live.comparison.is_clean());
    }

    #[test]
    fn each_pull_request_python_wrote_nothing_to_is_held_to_no_write_alone() {
        // Python's run wrote to 2 and not to 5: 2 is not held, 5 is, and
        // the live run wants to write to 5.
        let recording = synthetic("sweep");
        let python = with_calls(&recording, without(&recording.calls, &TO_FIVE));
        let live = sweep_at_its_instant(&python, as_recorded(&recording));
        assert_eq!(
            outcomes(&live),
            [
                "live snapshot 0: matched",
                "live verdict 0: matched",
                "live snapshot 1: matched",
                "live verdict 1: matched",
                "live writes 1: differs: POST repos/*/*/statuses/* would-be write, not sent",
            ]
        );
        assert_eq!(live.unsettled, 1);
        assert!(!live.comparison.is_clean());
    }

    #[test]
    fn a_write_to_a_pull_request_neither_run_decided_is_held_where_python_made_none() {
        let recording = synthetic("sweep");
        let mut quiet = TO_ONE.to_vec();
        quiet.extend(TO_THREE_AND_FOUR);
        let python = with_calls(&recording, without(&recording.calls, &quiet));
        let live = sweep_at_its_instant(&python, as_recorded(&recording));
        // 1 could not be read, live as in Python's run, and the live run
        // marks it so; Python's, here, did not.
        assert!(
            outcomes(&live).contains(
                &"live writes 0: differs: POST repos/*/*/statuses/*, to a pull request neither run decided would-be write, not sent".to_owned()
            ),
            "{:?}",
            outcomes(&live)
        );
        assert!(!live.comparison.is_clean());
        // Where Python's run wrote there too, nothing there is held.
        let live = sweep_at_its_instant(&recording, as_recorded(&recording));
        assert!(live
            .comparison
            .checks
            .iter()
            .all(|check| check.layer != Layer::LiveWrites));
    }

    #[test]
    fn in_a_sync_of_every_pull_request_an_authors_move_excuses_that_authors_verdicts_alone() {
        // The open listing, read live, shows 5 updated since Python read
        // it: its author's admission may follow, so 5's verdict is not held
        // to Python's. 2's author's pull requests did not move: 2's is.
        let recording = synthetic("sweep");
        let listing = "repos/dashpay/platform/pulls?state=open&per_page=100";
        let moved: String = recording
            .calls
            .lines()
            .map(|line| {
                let PyValue::Dict(mut entry) = py_loads(line).unwrap() else {
                    panic!("a call")
                };
                let is_listing = matches!(entry.get("args"), Some(PyValue::List(args))
                    if args.iter().any(|a| matches!(a, PyValue::Str(a) if a == listing)));
                if is_listing {
                    let Some(PyValue::Str(stdout)) = entry.get("stdout") else {
                        panic!("an answer")
                    };
                    let edited = stdout.replace(
                        r#""number": 5,"#,
                        r#""number": 5, "updated_at": "2026-09-12T10:05:00Z","#,
                    );
                    assert_ne!(&edited, stdout, "the listing names 5");
                    entry.insert("stdout".into(), PyValue::Str(edited));
                }
                format!(
                    "{}\n",
                    py_dumps(&PyValue::Dict(entry), false, None, None).unwrap()
                )
            })
            .collect();
        let live = sweep_at_its_instant(&recording, served(&moved));
        assert_eq!(
            outcomes(&live),
            [
                "live snapshot 0: matched",
                "live verdict 0: matched",
                "live snapshot 1: matched",
                "live verdict 1: moved",
            ]
        );
        assert!(live.comparison.is_clean());
    }

    #[test]
    fn a_recordings_writes_are_read_with_what_each_was_aimed_at() {
        let text = calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/9"], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "POST", "repos/a/b/statuses/c0ffee", "--input", "-"], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "PATCH", "repos/a/b/issues/comments/300", "--input", "-"], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "DELETE", "repos/a/b/issues/4/labels/ready-to-merge"], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "POST", "repos/a/b/pulls/2/requested_reviewers", "--input", "-"], "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "POST", "repos/a/b/issues/7/comments", "--input", "-"], "stdin": "{\"body\": \"<!-- pr-hygiene-nudge v1 bot=coderabbitai sha=abc -->\\n@coderabbitai review\"}", "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": ["--method", "POST", "repos/a/b/issues/7/comments", "--input", "-"], "stdin": "{\"body\": \"a record\"}", "exit": 0, "stdout": ""}"#,
            r#"{"kind": "write", "args": [], "exit": 0, "stdout": ""}"#,
        ]);
        let at = |target: Target| Recorded {
            target,
            nudge: false,
        };
        assert_eq!(
            recorded_writes(&text),
            [
                at(Target::Head("c0ffee".into())),
                at(Target::Comment(PyInt::from(300))),
                at(Target::Number(PyInt::from(4))),
                at(Target::Number(PyInt::from(2))),
                Recorded {
                    target: Target::Number(PyInt::from(7)),
                    nudge: true,
                },
                at(Target::Number(PyInt::from(7))),
                at(Target::Other),
            ]
        );
        assert!(recorded_writes("").is_empty());
    }

    #[test]
    fn a_write_is_set_against_the_decided_pull_request_its_route_names() {
        let decided = [
            py_loads(r#"{"number": 2, "head": "c0ffee", "comments": [{"id": 300}]}"#).unwrap(),
            py_loads(r#"{"number": 5, "head": "c0ffee", "comments": []}"#).unwrap(),
        ];
        let aims = Aims::of(&decided);
        let targets = [
            Target::Number(PyInt::from(2)),
            Target::Comment(PyInt::from(300)),
            // A head two open pull requests share is either one's.
            Target::Head("c0ffee".into()),
            // Pull requests neither run decided: one the policy no longer
            // governs, a head no decided pull request has, a mutation.
            Target::Number(PyInt::from(4)),
            Target::Head("beef".into()),
            Target::Comment(PyInt::from(400)),
            Target::Other,
        ];
        let kinds = ["a", "b", "c", "d", "e", "f", "g"];
        let aimed = Aimed::of(targets.iter().zip(kinds), &aims);
        assert_eq!(aimed.to(&PyInt::from(2)), ["a", "b", "c"]);
        assert_eq!(aimed.to(&PyInt::from(5)), ["c"]);
        assert!(aimed.to(&PyInt::from(4)).is_empty());
        let elsewhere: Vec<&str> = aimed.elsewhere.iter().map(|(_, k)| k.as_str()).collect();
        assert_eq!(elsewhere, ["d", "e", "f", "g"]);
    }

    #[test]
    fn a_live_nudge_after_the_pull_request_python_spent_its_nudge_on_is_not_held() {
        // Python's run spends its one nudge on the first pull request it
        // asks a bot about, and asks about no other after it. Every write
        // of a live run is refused, so its nudge is never spent and every
        // later pull request that wants a bot asked is asked too: those
        // asks follow from the refusal, and are not held. Nudges up to and
        // at Python's are: there Python's run had its nudge to spend.
        let comment = |n: i64, body: &str| Call::Rest {
            method: Method::Post,
            path: format!("repos/a/b/issues/{n}/comments"),
            body: Some(py_loads(&format!(r#"{{"body": "{body}"}}"#)).unwrap()),
            paginate: false,
        };
        let nudge = format!("{NUDGE_MARKER} bot=coderabbitai sha=abc -->");
        let writes: Vec<WouldBe> = [
            comment(1, &nudge),
            comment(1, "a record"),
            comment(2, &nudge),
            rest(Method::Post, "repos/a/b/statuses/def", None),
            comment(3, &nudge),
        ]
        .iter()
        .map(WouldBe::of)
        .collect();
        assert_eq!(
            writes.iter().map(|w| w.nudge).collect::<Vec<_>>(),
            [true, false, true, false, true]
        );
        // Python decided 1, 2 and 3 in that order.
        let decided: Vec<Option<PyInt>> = (1..=3).map(|n| Some(PyInt::from(n))).collect();
        let python_nudged = |n: i64| Recorded {
            target: Target::Number(PyInt::from(n)),
            nudge: true,
        };
        let held = |python: &[Recorded]| -> Vec<Target> {
            let nudged = Nudged::of(&decided, python);
            held_writes(&writes, &nudged)
                .map(|(target, _)| target.clone())
                .collect()
        };
        let one = Target::Number(PyInt::from(1));
        let two = Target::Number(PyInt::from(2));
        let three = Target::Number(PyInt::from(3));
        let status = Target::Head("def".into());
        // Python spent it on 1: the asks of 2 and 3 are the refusal's.
        assert_eq!(
            held(&[python_nudged(1)]),
            [one.clone(), one.clone(), status.clone()]
        );
        // On 2, the live run having asked about 1 too: 1 and 2 are held,
        // 3 is not. Here the live run first asks about 1 where Python did
        // not: a difference to report, which this keeps.
        assert_eq!(
            held(&[python_nudged(2)]),
            [one.clone(), one.clone(), two.clone(), status.clone()]
        );
        // Python asked about none, so none wanted it: every ask is held.
        assert_eq!(
            held(&[]),
            [one.clone(), one.clone(), two, status.clone(), three]
        );
        // Python spent it on 1, which by the live read no longer wants a
        // bot asked (a push since, a bot's review): the live run's first ask
        // is about 2, which Python's, its nudge spent, never reached.
        let later: Vec<WouldBe> = writes[1..].to_vec();
        let nudged = Nudged::of(&decided, &[python_nudged(1)]);
        let held: Vec<Target> = held_writes(&later, &nudged)
            .map(|(target, _)| target.clone())
            .collect();
        assert_eq!(held, [one, status]);
    }

    #[test]
    fn a_write_to_a_pull_request_neither_run_decided_moved_with_its_own_line() {
        let listing = |pages: &str| Logged {
            key: gh_arguments(&Call::Rest {
                method: Method::Get,
                path: "repos/a/b/pulls?state=open&per_page=100".into(),
                body: None,
                paginate: true,
            })
            .unwrap(),
            answer: Ok(Reply::Pages(vec![pages.into()])),
        };
        let recorded = recorded_answers(&calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls?state=open&per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 0, "stdout": "[[{\"number\": 3, \"head\": {\"sha\": \"h3\"}, \"updated_at\": \"t1\"}, {\"number\": 4, \"head\": {\"sha\": \"h4\"}, \"updated_at\": \"t1\"}]]", "stderr": ""}"#,
        ]));
        // 4 was updated between the reads; 3 was not.
        let log = vec![
            listing(
                r#"[{"number": 3, "head": {"sha": "h3"}, "updated_at": "t1"}, {"number": 4, "head": {"sha": "h4"}, "updated_at": "t2"}]"#,
            ),
            // The live run read 4's history and 3's comments.
            Logged {
                key: (Vec::new(), Some("{}".into())),
                answer: Ok(Reply::Text(
                    r#"{"data": {"repository": {"pr4": {"number": 4, "comments": {"nodes": [{"databaseId": 400}]}}}}}"#.into(),
                )),
            },
            Logged {
                key: gh_arguments(&Call::Rest {
                    method: Method::Get,
                    path: "repos/a/b/issues/3/comments?per_page=100".into(),
                    body: None,
                    paginate: true,
                })
                .unwrap(),
                answer: Ok(Reply::Pages(vec![r#"[{"id": 300}]"#.into()])),
            },
        ];
        let listings = Listings::of("a/b", &recorded, &log);
        let owners = comment_owners(&log);
        assert_eq!(owners.get(&PyInt::from(400)), Some(&PyInt::from(4)));
        assert_eq!(owners.get(&PyInt::from(300)), Some(&PyInt::from(3)));
        let moved = |target: Target| listings.target_moved(&target, &owners, false);
        assert!(moved(Target::Number(PyInt::from(4))));
        assert!(!moved(Target::Number(PyInt::from(3))));
        assert!(moved(Target::Head("h4".into())));
        assert!(!moved(Target::Head("h3".into())));
        assert!(moved(Target::Comment(PyInt::from(400))));
        assert!(!moved(Target::Comment(PyInt::from(300))));
        // Found nowhere: whatever anything else did.
        assert!(!moved(Target::Comment(PyInt::from(999))));
        assert!(listings.target_moved(&Target::Other, &owners, true));
    }

    #[test]
    fn a_read_only_python_failed_is_named_where_the_live_one_was_answered() {
        let calls = calls(&[
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1/files?per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 1, "stdout": "", "stderr": "HTTP 502"}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/pulls/1/files?per_page=100", "--paginate", "--slurp"], "stdin": null, "exit": 1, "stdout": "", "stderr": "HTTP 502"}"#,
            r#"{"kind": "read", "args": ["--method", "GET", "repos/a/b/labels/x"], "stdin": null, "exit": 1, "stdout": "", "stderr": "HTTP 404"}"#,
        ]);
        let files = gh_arguments(&Call::Rest {
            method: Method::Get,
            path: "repos/a/b/pulls/1/files?per_page=100".into(),
            body: None,
            paginate: true,
        })
        .unwrap();
        let label = gh_arguments(&rest(Method::Get, "repos/a/b/labels/x", None)).unwrap();
        let answered = Logged {
            key: files,
            answer: Ok(Reply::Pages(vec!["[]".into()])),
        };
        assert!(answered_where_python_failed(&calls, &[answered]));
        // Failed on both sides: nothing to set against it.
        let missing = Logged {
            key: label,
            answer: Err(TransportError::Failed(Failed::unavailable())),
        };
        assert!(!answered_where_python_failed(&calls, &[missing]));
    }
}
