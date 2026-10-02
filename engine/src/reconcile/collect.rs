//! `collect`: the admission history of every candidate, then the full
//! evidence of the pull requests asked for.

use super::state::admission_conflicts;
use super::values::{list, lower, number, s};
use super::{except_github, ClockSite, Reconciler, Selection};
use crate::evidence::records::{parse_controller_diff, parse_controller_state};
use crate::evidence::{History, ReadError, Transport};
use crate::policy::{admit, effective_admission, governs};
use crate::pycompat::object::{get, getitem, iterate};
use crate::pycompat::ops::{py_eq, py_eq_str};
use crate::pycompat::{PyDict, PyInt, PyValue};

/// What `collect` read: every open pull request, the candidates whose
/// histories decide admission, and the snapshots of those asked for.
#[derive(Debug, Clone)]
pub struct Collected {
    /// Every open pull request's identity, as listed.
    pub prs: Vec<PyValue>,
    /// The pull requests whose admission was decided, each with its
    /// comments, record, diff and latest inactive transition.
    pub candidates: Vec<PyValue>,
    /// The full evidence of each pull request asked for that could be read.
    pub snapshots: Vec<PyValue>,
}

/// `{'comments': p['comments'], 'lifecycle_at': p['lifecycle_at']}` of a
/// candidate: the history `load_histories` already read for it.
fn history_of(candidate: &PyValue) -> Result<History, ReadError> {
    let comments = iterate(getitem(candidate, "comments")?)?
        .into_iter()
        .map(|comment| comment.into_owned())
        .collect();
    let lifecycle_at = match getitem(candidate, "lifecycle_at")? {
        PyValue::Str(at) => Some(at.clone()),
        _ => None,
    };
    Ok(History {
        comments,
        lifecycle_at,
    })
}

impl<T: Transport> Reconciler<'_, T> {
    /// `load_histories(api, selected)`: admission evidence for every
    /// candidate, in one query rather than two each. A pull request closed
    /// or deleted since the listing holds no slot either way, and is left
    /// out.
    pub fn load_histories(&mut self, selected: &[PyValue]) -> Result<Vec<PyValue>, ReadError> {
        let numbers = selected.iter().map(number).collect::<Result<Vec<_>, _>>()?;
        let histories = self.api.histories(&numbers)?;
        let mut loaded = Vec::new();
        for (pr, n) in selected.iter().zip(&numbers) {
            let Some(history) = histories.get(n) else {
                continue;
            };
            let record = parse_controller_state(&history.comments)?;
            let (state, comment_id) = match record {
                Some(record) => (record.state, record.comment_id),
                None => (PyValue::None, PyValue::None),
            };
            if !matches!(state, PyValue::None)
                && !py_eq(getitem(&state, "number")?, &PyValue::Int(n.clone()))
            {
                return Err(ReadError::GitHub(
                    "Controller admission history belongs to another PR".into(),
                ));
            }
            let diff = parse_controller_diff(&history.comments, n)?;
            let PyValue::Dict(fields) = pr else {
                return Err(ReadError::NotPorted(
                    "a listed pull request that is not a dict".into(),
                ));
            };
            let mut entry: PyDict = fields.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            entry.insert("comments".into(), list(history.comments.clone()));
            entry.insert("controller_state".into(), state);
            entry.insert("controller_comment_id".into(), comment_id);
            entry.insert(
                "lifecycle_at".into(),
                history.lifecycle_at.clone().map_or(PyValue::None, s),
            );
            entry.insert("controller_diff".into(), diff.unwrap_or(PyValue::None));
            loaded.push(PyValue::Dict(entry));
        }
        Ok(loaded)
    }

    /// `collect(api, policy, number, apply, reconcile_author, batch_size)`:
    /// every open pull request, the admission history of the candidates
    /// `selection` names, then the full evidence of those asked for.
    ///
    /// Admission is decided from every candidate's history, so a history
    /// that cannot be read leaves no slot knowable: under `apply` every
    /// candidate's head is marked, and the error is raised on. Evidence
    /// that cannot be read marks only the pull request it belongs to: one
    /// rate-limited read used to mark everything selected, and a full pass
    /// selects everything open.
    pub fn collect(
        &mut self,
        policy: &PyValue,
        selection: &mut Selection<'_>,
        apply: bool,
        reconcile_author: bool,
    ) -> Result<Collected, ReadError> {
        let prs = self.api.open_prs()?;
        let mut selected = Vec::new();
        for pr in &prs {
            if governs(policy, getitem(pr, "base")?)? {
                selected.push(pr.clone());
            }
        }
        let mut batch_numbers: Option<Vec<PyValue>> = None;
        let mut asked: Option<PyInt> = None;
        match selection {
            Selection::All => {}
            Selection::Batch(choose) => {
                let batch = choose(&selected);
                let mut numbers = Vec::new();
                let mut authors = Vec::new();
                for pr in &batch {
                    numbers.push(getitem(pr, "number")?.clone());
                    authors.push(lower(getitem(pr, "author")?)?);
                }
                let mut kept = Vec::new();
                for pr in selected {
                    if authors.contains(&lower(getitem(&pr, "author")?)?) {
                        kept.push(pr);
                    }
                }
                selected = kept;
                batch_numbers = Some(numbers);
            }
            Selection::Pr(n) => {
                let wanted = PyValue::Int(n.clone());
                let listed = prs.iter().find(|pr| {
                    get(pr, "number")
                        .is_ok_and(|found| found.is_some_and(|found| py_eq(found, &wanted)))
                });
                let principal = match listed.filter(|pr| pr.truthy()) {
                    Some(pr) => pr.clone(),
                    None => self.api.pull(n)?,
                };
                let author = lower(getitem(&principal, "author")?)?;
                let mut kept = Vec::new();
                for pr in selected {
                    if lower(getitem(&pr, "author")?)? == author {
                        kept.push(pr);
                    }
                }
                selected = kept;
                asked = Some(n.clone());
            }
        }
        let read = self.collect_selected(
            policy,
            &selected,
            asked.as_ref(),
            batch_numbers.as_deref(),
            apply,
            reconcile_author,
        );
        match read {
            Ok((candidates, snapshots)) => Ok(Collected {
                prs,
                candidates,
                snapshots,
            }),
            Err(ReadError::GitHub(message)) => {
                if apply {
                    for pr in &selected {
                        self.mark_unreadable(policy, pr)?;
                    }
                }
                Err(ReadError::GitHub(message))
            }
            Err(other) => Err(other),
        }
    }

    /// The part of `collect` inside its `try`.
    fn collect_selected(
        &mut self,
        policy: &PyValue,
        selected: &[PyValue],
        asked: Option<&PyInt>,
        batch_numbers: Option<&[PyValue]>,
        apply: bool,
        reconcile_author: bool,
    ) -> Result<(Vec<PyValue>, Vec<PyValue>), ReadError> {
        let candidates = self.load_histories(selected)?;
        let is_asked = |pr: &PyValue| -> Result<bool, ReadError> {
            Ok(match asked {
                None => true,
                Some(n) => py_eq(getitem(pr, "number")?, &PyValue::Int(n.clone())),
            })
        };
        let mut requested = Vec::new();
        for pr in &candidates {
            if is_asked(pr)? {
                requested.push(pr);
            }
        }
        if asked.is_some() && reconcile_author {
            // Whoever's slot this pull request moves is reconciled with it:
            // the one asked for, every candidate whose admission is about to
            // change, and every pull request of an author holding more slots
            // than allowed.
            let now = s(self.utc_now(ClockSite::Collect));
            let slots = admit(policy, &list(candidates.clone()), &now)?;
            let conflicts = admission_conflicts(policy, &candidates)?;
            requested = Vec::new();
            for pr in &candidates {
                let n = number(pr)?;
                let slotted = slots.get(&n).is_some_and(PyValue::truthy);
                if is_asked(pr)?
                    || slotted != effective_admission(pr)?.truthy()
                    || conflicts.contains(&lower(getitem(pr, "author")?)?)
                {
                    requested.push(pr);
                }
            }
        }
        if let Some(batch) = batch_numbers {
            let mut kept = Vec::new();
            for pr in requested {
                let n = getitem(pr, "number")?;
                if batch.iter().any(|b| py_eq(b, n)) {
                    kept.push(pr);
                }
            }
            requested = kept;
        }
        if let Some(n) = asked {
            if requested.is_empty() && !reconcile_author {
                return Err(ReadError::GitHub(format!(
                    "PR #{n} is not open on a configured target branch"
                )));
            }
        }
        // The comments and the latest transition were read for every
        // candidate in one query; the snapshot reuses that read.
        let mut taken = Vec::with_capacity(requested.len());
        for pr in &requested {
            let history = history_of(pr)?;
            taken.push(except_github(self.api.snapshot(
                &number(pr)?,
                policy,
                Some(&history),
            ))?);
        }
        // Admission was decided above, from every candidate's history, so
        // one pull request whose evidence could not be read invalidates
        // only itself.
        let mut snapshots = Vec::new();
        for (pr, taken) in requested.iter().zip(taken) {
            match taken {
                Ok(snapshot) => snapshots.push(snapshot),
                Err(error) => {
                    self.say(format!(
                        "PR #{}: {error}; its status says so",
                        super::values::shown(getitem(pr, "number")?)
                    ));
                    if apply {
                        self.mark_unreadable(policy, pr)?;
                    }
                }
            }
        }
        Ok((candidates, snapshots))
    }

    /// `_mark_unreadable(api, policy, pr)`: the error status on a pull
    /// request still open on a governed branch whose evidence could not be
    /// read.
    fn mark_unreadable(&mut self, policy: &PyValue, pr: &PyValue) -> Result<(), ReadError> {
        let n = number(pr)?;
        let marked = (|| -> Result<(), ReadError> {
            let current = self.api.pull(&n)?;
            if py_eq_str(getitem(&current, "state")?, "open")
                && governs(policy, getitem(&current, "base")?)?
            {
                let head = super::values::text(getitem(&current, "head")?, "head")?.to_owned();
                self.api.post_status(
                    &head,
                    "error",
                    "Incomplete policy evidence; reconciliation required",
                    None,
                )?;
            }
            Ok(())
        })();
        if except_github(marked)?.is_err() {
            self.say(format!("PR #{n}: unable to publish evidence error status"));
        }
        Ok(())
    }

    /// `_mark_configuration_error(repository, policy, number)`: the error
    /// status on the one pull request an event named, when the policy
    /// could not be read — and only where that cannot reach any other pull
    /// request.
    ///
    /// A sweep has no verified target set, so nothing is written for one:
    /// the failed job is the diagnostic. Nor is anything written where even
    /// the repository or branch scope is unreadable, on a draft, which
    /// cannot merge, or on a head another pull request shares, which a
    /// commit-scoped status cannot isolate. `policy` is what was read
    /// before validation failed, if anything; `target_url` is where the
    /// status links.
    pub fn mark_configuration_error(
        &mut self,
        policy: Option<&PyValue>,
        asked: Option<&PyInt>,
        target_url: Option<&str>,
    ) -> Result<(), ReadError> {
        let Some(n) = asked.filter(|n| **n >= PyInt::from(1)) else {
            return Ok(());
        };
        let Some(policy @ PyValue::Dict(fields)) = policy else {
            return Ok(());
        };
        if !fields
            .get("repository")
            .is_some_and(|repository| py_eq_str(repository, self.api.repo()))
        {
            return Ok(());
        }
        let Some(PyValue::List(branches)) = fields.get("target_branches") else {
            return Ok(());
        };
        if branches.is_empty()
            || branches.iter().any(|branch| {
                !matches!(branch, PyValue::Str(b) if !crate::pycompat::text::py_strip(b).is_empty())
            })
        {
            return Ok(());
        }
        let marked = (|| -> Result<(), ReadError> {
            let current = self.api.pull(n)?;
            if !py_eq_str(getitem(&current, "state")?, "open")
                || getitem(&current, "draft")?.truthy()
                || !governs(policy, getitem(&current, "base")?)?
            {
                return Ok(());
            }
            let head = getitem(&current, "head")?.clone();
            let wanted = PyValue::Int(n.clone());
            for pr in self.api.open_prs()? {
                if !py_eq(getitem(&pr, "number")?, &wanted) && py_eq(getitem(&pr, "head")?, &head) {
                    self.say(format!(
                        "PR #{n}: shared head; configuration error reported by the workflow only"
                    ));
                    return Ok(());
                }
            }
            let confirmed = self.api.pull(n)?;
            for key in ["head", "base", "state", "draft"] {
                if !py_eq(getitem(&confirmed, key)?, getitem(&current, key)?) {
                    return Ok(());
                }
            }
            let head = super::values::text(&head, "head")?.to_owned();
            self.api.post_status(
                &head,
                "error",
                "Invalid policy configuration; inspect workflow log",
                target_url,
            )?;
            Ok(())
        })();
        if except_github(marked)?.is_err() {
            self.say(format!(
                "PR #{n}: unable to publish configuration error status"
            ));
        }
        Ok(())
    }
}
