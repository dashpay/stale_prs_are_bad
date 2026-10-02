//! A head's build, as `green`, `running` or `failed`, from its checks.

use super::error::ReadError;
use super::py::{text, Read};
use crate::pycompat::object::{get, getitem, or, or_default, str_method, EMPTY_DICT};
use crate::pycompat::ops::{
    py_compare, py_contains, py_eq_str, py_hashable, py_in_str_set, py_same_element, Compare,
};
use crate::pycompat::PyValue;

/// Conclusions and states that fail a build, as GraphQL spells them. The
/// REST spellings are a different API. `ACTION_REQUIRED` is a conclusion,
/// not a status: the check has finished and wants a human, so it is failing,
/// not running.
pub const BUILD_FAILED: &[&str] = &[
    "FAILURE",
    "TIMED_OUT",
    "STARTUP_FAILURE",
    "ACTION_REQUIRED",
    "ERROR",
];
/// Conclusions and states that pass.
pub const BUILD_PASSED: &[&str] = &["SUCCESS", "SKIPPED", "NEUTRAL", "EXPECTED"];
/// Conclusions that say nothing about the code: a cancelled attempt, and
/// this controller is the most-cancelled check on these repositories.
pub const BUILD_NO_VERDICT: &[&str] = &["CANCELLED", "STALE"];
/// The caller workflow every governed repository runs this engine from.
pub const CALLER_WORKFLOW: &str = "/pr-review-policy.yml";

/// The verdict over a head's checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Build {
    Green,
    Running,
    Failed,
}

impl Build {
    pub fn as_str(self) -> &'static str {
        match self {
            Build::Green => "green",
            Build::Running => "running",
            Build::Failed => "failed",
        }
    }
}

/// `((node.get("checkSuite") or {}).get("workflowRun") or {}).get("workflow") or {}`:
/// the workflow a check run belongs to, or nothing.
fn workflow(node: &PyValue) -> Read<Option<&PyValue>> {
    let suite = get(node, "checkSuite")?;
    let run = get(or(suite, &EMPTY_DICT), "workflowRun")?;
    Ok(or_default(get(or(run, &EMPTY_DICT), "workflow")?))
}

/// `_ours(node)`: whether a check run is this controller reviewing the pull
/// request, which it must never wait for.
///
/// Matched on the caller workflow, which the engine refuses to run from any
/// other path, rather than on the job name, which each repository is free
/// to rename. An Actions job is told apart from a check run some other tool
/// filed into the same suite, which GitHub attributes to this workflow too.
fn ours(node: &PyValue) -> Read<bool> {
    let suite = workflow(node)?;
    let details = or_default(get(node, "detailsUrl")?)
        .cloned()
        .unwrap_or_else(|| PyValue::Str(String::new()));
    let resource = match or_default(get(or(suite, &EMPTY_DICT), "resourcePath")?) {
        Some(value) => str_method(value, "endswith")?,
        None => "",
    };
    Ok(resource.ends_with(CALLER_WORKFLOW)
        && py_contains(&details, &PyValue::Str("/actions/runs/".into()))?
        && py_contains(&details, &PyValue::Str("/job/".into()))?)
}

/// One check, as the verdict keys it: by kind, by workflow and by name, so a
/// job called `build` in two workflows cannot stand in for the other.
struct Latest {
    kind: &'static str,
    workflow: PyValue,
    name: String,
    when: PyValue,
    state: PyValue,
}

/// `build_verdict(nodes)`.
///
/// Only the newest run of each check counts. Re-running a check does not
/// replace the run it repeats, it adds another beside it, so a head keeps
/// every failed and cancelled attempt for ever; without this a flaky test
/// could never be cleared by re-running it. A cancelled attempt is not a
/// result: letting one overwrite an older verdict would turn a failure green
/// by cancelling its re-run. A result this does not recognise is
/// unfinished, never green.
pub fn build_verdict(nodes: &[PyValue]) -> Result<Build, ReadError> {
    let mut latest: Vec<Latest> = Vec::new();
    for node in nodes {
        if !matches!(node, PyValue::Dict(_)) {
            return Err(ReadError::github("Incomplete check list"));
        }
        let (kind, workflow_path, name, when, state);
        if get(node, "__typename")?.is_some_and(|t| py_eq_str(t, "CheckRun")) {
            if ours(node)? {
                continue;
            }
            kind = "check";
            workflow_path = or_default(get(or(workflow(node)?, &EMPTY_DICT), "resourcePath")?)
                .cloned()
                .unwrap_or_else(|| PyValue::Str(String::new()));
            name = text(Some(getitem(node, "name")?), "check name")?.to_owned();
            when = get(node, "startedAt")?;
            state = match or_default(get(node, "conclusion")?) {
                Some(conclusion) => Some(conclusion),
                None => get(node, "status")?,
            };
        } else {
            if get(node, "context")?.is_some_and(|c| py_eq_str(c, "PR Hygiene")) {
                continue;
            }
            kind = "status";
            workflow_path = PyValue::Str(String::new());
            name = text(Some(getitem(node, "context")?), "status context")?.to_owned();
            when = get(node, "createdAt")?;
            state = get(node, "state")?;
        }
        let when = or_default(when)
            .cloned()
            .unwrap_or_else(|| PyValue::Str(String::new()));
        let state = state.cloned().unwrap_or(PyValue::None);
        if py_in_str_set(&state, BUILD_NO_VERDICT)? {
            continue;
        }
        // Looking the key up hashes it.
        py_hashable(&workflow_path)?;
        let existing = latest.iter_mut().find(|entry| {
            entry.kind == kind
                && entry.name == name
                && py_same_element(&entry.workflow, &workflow_path)
        });
        match existing {
            None => latest.push(Latest {
                kind,
                workflow: workflow_path,
                name,
                when,
                state,
            }),
            Some(entry) => {
                if py_compare(&when, Compare::Ge, &entry.when)? {
                    entry.when = when;
                    entry.state = state;
                }
            }
        }
    }
    for entry in &latest {
        if py_in_str_set(&entry.state, BUILD_FAILED)? {
            return Ok(Build::Failed);
        }
    }
    for entry in &latest {
        if !py_in_str_set(&entry.state, BUILD_PASSED)? {
            return Ok(Build::Running);
        }
    }
    Ok(Build::Green)
}
