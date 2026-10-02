//! `run`: one reconciliation of a repository, from the listing to the
//! report, once its policy is loaded.

use super::state::admission_conflicts;
use super::telemetry::{head_state, BOT};
use super::values::{list, lower, number, s, text};
use super::{except_github, ClockSite, Reconciler, Selection, WAIVED_LABEL};
use crate::evidence::{Method, ReadError, Transport};
use crate::policy::{admit, evaluate, STATE_LABELS};
use crate::pycompat::object::{get, get_or, getitem, iterate, str_method, EMPTY_LIST, NONE};
use crate::pycompat::ops::{py_compare_sequences, py_eq, py_eq_str, py_sort_by, Compare};
use crate::pycompat::text::py_lower;
use crate::pycompat::{PyDict, PyErr, PyInt, PyValue};
use std::collections::HashMap;

/// What a run does with its verdicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Decide and report; write nothing.
    Report,
    /// Decide, report, and — where `apply` — publish every verdict.
    Sync,
}

/// How one run is asked for.
pub struct RunOptions<'s> {
    pub command: Command,
    pub selection: Selection<'s>,
    /// Whether a sync writes. A report never does.
    pub apply: bool,
    /// Whose pull requests the report lists — their own, and those they
    /// are asked to review — or everyone's.
    pub user: Option<String>,
    /// How many review bots this run may ask to look at a head.
    pub nudges: usize,
    /// The review system's status page, read at most once and only for a
    /// policy with bot timeouts: the payload, or `None` when it could not
    /// be read.
    pub telemetry: &'s mut dyn FnMut() -> PyValue,
}

/// What one run decided and did.
#[derive(Debug, Clone)]
pub struct Run {
    /// The instant every verdict was decided at.
    pub generated_at: String,
    /// What `collect` read.
    pub context: Vec<PyValue>,
    pub candidates: Vec<PyValue>,
    pub snapshots: Vec<PyValue>,
    /// The verdicts, one per snapshot, in the order decided.
    pub verdicts: Vec<PyValue>,
    /// The pull requests whose reconciliation failed. Each was still given
    /// its turn; the run fails once all have had one, and reports nothing.
    pub failed: Vec<PyInt>,
    /// `{"generated_at", "pull_requests"}`: the verdicts sorted for the
    /// report and filtered to the user. `None` when the run failed.
    pub report: Option<PyValue>,
}

impl Run {
    /// The run's own failure, as Python raises it once every pull request
    /// has had its turn: `None` when every one was reconciled.
    pub fn failure(&self) -> Option<ReadError> {
        if self.failed.is_empty() {
            return None;
        }
        let numbers: Vec<String> = self.failed.iter().map(|n| format!("#{n}")).collect();
        Some(ReadError::GitHub(format!(
            "reconciliation failed for {}",
            numbers.join(", ")
        )))
    }
}

/// `telemetry_states(policy, snapshots, now, payload)`: per pull request,
/// what the review system last said about the head it is awaiting, as
/// `(number, {bot: state})`.
pub fn telemetry_states(
    policy: &PyValue,
    snapshots: &[PyValue],
    now: &str,
    payload: &PyValue,
) -> Result<Vec<(PyValue, PyValue)>, PyErr> {
    let mut states = Vec::new();
    if !payload.truthy() {
        return Ok(states);
    }
    for pr in snapshots {
        let state = head_state(
            payload,
            getitem(policy, "repository")?,
            getitem(pr, "number")?,
            getitem(pr, "head")?,
            get_or(pr, "head_seen_at", &NONE)?,
            &s(now),
        )?;
        if let Some(state) = state {
            let mut by_bot = PyDict::new();
            by_bot.insert(BOT.into(), s(state));
            states.push((getitem(pr, "number")?.clone(), PyValue::Dict(by_bot)));
        }
    }
    Ok(states)
}

/// `evaluate_snapshots(policy, context, candidates, snapshots, now,
/// payload)`: the verdicts, one per snapshot, under the admissions decided
/// from every candidate. `log` receives what Python prints.
///
/// More persisted admissions than the policy allows is a race between two
/// runs for one author. `admit` keeps the oldest, and the surplus returns
/// to waiting for a slot on its next write: marking the whole queue an
/// error instead held every pull request of that author. A head another
/// open pull request shares is a configuration error, since a
/// commit-scoped status cannot tell the two apart.
pub fn evaluate_snapshots(
    policy: &PyValue,
    context: &[PyValue],
    candidates: &[PyValue],
    snapshots: &[PyValue],
    now: &str,
    payload: &PyValue,
    log: &mut Vec<String>,
) -> Result<Vec<PyValue>, ReadError> {
    let states = telemetry_states(policy, snapshots, now, payload)?;
    let admissions = admit(policy, &list(candidates.to_vec()), &s(now))?;
    for author in admission_conflicts(policy, candidates)? {
        log.push(format!(
            "{author}: more than five persisted admissions; keeping the five oldest"
        ));
    }
    let mut heads: HashMap<String, usize> = HashMap::new();
    for pr in context {
        *heads
            .entry(text(getitem(pr, "head")?, "head")?.to_owned())
            .or_default() += 1;
    }
    let mut rows = Vec::with_capacity(snapshots.len());
    for pr in snapshots {
        let n = number(pr)?;
        let admitted = admissions.get(&n).unwrap_or(&PyValue::None);
        let wanted = PyValue::Int(n.clone());
        let state = states
            .iter()
            .find(|(number, _)| py_eq(number, &wanted))
            .map_or(&PyValue::None, |(_, state)| state);
        let mut result = evaluate(policy, pr, admitted, &s(now), state)?;
        let head = text(getitem(pr, "head")?, "head")?;
        if heads.get(head).copied().unwrap_or(0) > 1 {
            result.insert("state".into(), s("configuration-error"));
            result.insert("status".into(), s("error"));
            result.insert("reviewers".into(), list(Vec::new()));
            result.insert("objectors".into(), list(Vec::new()));
            result.insert("ready_since".into(), PyValue::None);
            match result.get_mut("blockers") {
                Some(PyValue::List(blockers)) => blockers.push(s(
                    "Another open PR shares this head; commit-scoped status is ambiguous",
                )),
                _ => return Err(PyErr::Key("blockers".into()).into()),
            }
        }
        result.insert("repository".into(), getitem(policy, "repository")?.clone());
        rows.push(PyValue::Dict(result));
    }
    Ok(rows)
}

/// `selected_rows(rows, user)`: the rows of the user's own pull requests and
/// of those they are asked to review, or every row.
pub fn selected_rows(rows: &[PyValue], user: Option<&str>) -> Result<Vec<PyValue>, PyErr> {
    let Some(user) = user.filter(|user| !user.is_empty()) else {
        return Ok(rows.to_vec());
    };
    let user = py_lower(user);
    let mut selected = Vec::new();
    for row in rows {
        let mut mine = lower(getitem(row, "author")?)? == user;
        if !mine {
            for reviewer in iterate(get_or(row, "reviewers", &EMPTY_LIST)?)? {
                if py_lower(str_method(&reviewer, "lower")?) == user {
                    mine = true;
                }
            }
        }
        if mine {
            selected.push(row.clone());
        }
    }
    Ok(selected)
}

impl<T: Transport> Reconciler<'_, T> {
    /// `run(argv)`, from the listing on: collect what `options.selection`
    /// names, decide it, and for a sync publish each verdict — every pull
    /// request its turn even after one fails — then, on a run not aimed at
    /// one pull request, take the engine's marks off the pull requests it
    /// no longer governs. `policy` is one `validate_policy` accepted.
    pub fn run(&mut self, policy: &PyValue, options: RunOptions<'_>) -> Result<Run, ReadError> {
        let RunOptions {
            command,
            mut selection,
            apply,
            user,
            nudges,
            telemetry,
        } = options;
        let sync = command == Command::Sync;
        let aimed = matches!(selection, Selection::Pr(_));
        let collected = self.collect(policy, &mut selection, apply, sync)?;
        let (context, mut candidates, snapshots) =
            (collected.prs, collected.candidates, collected.snapshots);
        let now = self.utc_now(ClockSite::Run);
        // The review system's page is read once per run, never inside a
        // verdict: that runs twice per publication and must give the same
        // answer both times.
        let payload = if get(policy, "bot_timeouts")?.is_some_and(PyValue::truthy) {
            telemetry()
        } else {
            PyValue::None
        };
        let mut log = Vec::new();
        let verdicts = evaluate_snapshots(
            policy,
            &context,
            &candidates,
            &snapshots,
            &now,
            &payload,
            &mut log,
        )?;
        for line in log {
            self.say(line);
        }
        let mut nudged = 0;
        let mut failed = Vec::new();
        let mut checked_labels = false;
        for (pr, result) in snapshots.iter().zip(&verdicts) {
            if !sync {
                continue;
            }
            let reconciled = (|| -> Result<(), ReadError> {
                if apply {
                    if !checked_labels {
                        self.check_labels()?;
                        checked_labels = true;
                    }
                    nudged += self.nudge(pr, result, nudges.saturating_sub(nudged))?;
                }
                let written =
                    self.publish(policy, pr, result, &context, apply, Some(&candidates))?;
                if let Some(written) = written {
                    for candidate in candidates.iter_mut() {
                        if py_eq(getitem(candidate, "number")?, getitem(pr, "number")?) {
                            if let PyValue::Dict(fields) = candidate {
                                fields.insert("controller_state".into(), written.clone());
                            }
                        }
                    }
                }
                Ok(())
            })();
            if let Err(error) = except_github(reconciled)? {
                // Every pull request selected gets its turn. Stopping at the
                // first failure left the rest with whatever status they had —
                // on a full pass, possibly a passing one from before the
                // check became the gate.
                let n = number(pr)?;
                self.say(format!("PR #{n}: {error}"));
                failed.push(n.clone());
                if apply {
                    let head = text(getitem(pr, "head")?, "head")?.to_owned();
                    let posted = self.api.post_status(
                        &head,
                        "error",
                        "Policy reconciliation failed; inspect workflow log",
                        None,
                    );
                    if except_github(posted)?.is_err() {
                        self.say(format!("PR #{n}: unable to publish the failure either"));
                    }
                }
            }
        }
        let mut run = Run {
            generated_at: now.clone(),
            context,
            candidates,
            snapshots,
            verdicts,
            failed,
            report: None,
        };
        if !run.failed.is_empty() {
            return Ok(run);
        }
        // A sweep tidies what it passes; a run aimed at one pull request
        // does not go looking through the repository.
        if !aimed {
            self.clear_marks(policy, &run.context, apply && sync)?;
        }
        let mut rows: Vec<(Vec<PyValue>, PyValue)> = Vec::with_capacity(run.verdicts.len());
        for row in &run.verdicts {
            let since = match get(row, "ready_since")?.filter(|since| since.truthy()) {
                Some(since) => since.clone(),
                None => s(now.as_str()),
            };
            let key = vec![
                PyValue::Bool(!py_eq_str(getitem(row, "state")?, "ready-for-human")),
                since,
                getitem(row, "number")?.clone(),
            ];
            rows.push((key, row.clone()));
        }
        py_sort_by(&mut rows, |(a, _), (b, _)| {
            py_compare_sequences(a, Compare::Lt, b)
        })?;
        let rows: Vec<PyValue> = rows.into_iter().map(|(_, row)| row).collect();
        let mut report = PyDict::new();
        report.insert("generated_at".into(), s(now));
        report.insert(
            "pull_requests".into(),
            list(selected_rows(&rows, user.as_deref())?),
        );
        run.report = Some(PyValue::Dict(report));
        Ok(run)
    }

    /// Say once which labels are missing. A label written to a pull request
    /// is created by that write, in a default colour — ugly, but a missing
    /// label must not mark every pull request an error under a required
    /// check.
    fn check_labels(&mut self) -> Result<(), ReadError> {
        let repo = self.api.repo().to_owned();
        for label in STATE_LABELS.iter().chain([WAIVED_LABEL].iter()) {
            let path = format!("repos/{repo}/labels/{label}");
            let read = self.api.client_mut().request(Method::Get, &path, None);
            if except_github(read)?.is_err() {
                self.say(format!("Label {label} does not exist in {repo}; create it"));
            }
        }
        Ok(())
    }
}
