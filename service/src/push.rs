//! What a verdict push must look like to be stored: one governed
//! repository's open PRs and the engine's newest verdicts on them, as read
//! by the run whose token carries it.

use crate::oidc::{is_commit_sha, VerifiedPusher, CLOCK_SKEW};
use crate::snapshot::{
    area, bounded, engine_state, invalid, is_repo, login, text, Invalid, MAX_LIST, MAX_TEXT,
    MAX_TITLE,
};
use chrono::{DateTime, TimeDelta, Utc};
use pr_hygiene::model::PolicyState;
use serde::Deserialize;
use std::collections::HashSet;

/// Largest push accepted, in bytes.
pub const BODY_LIMIT: usize = 1024 * 1024;

/// How long before its token was minted a push may have been read: the
/// run reads GitHub first, writes every status, and only then mints it.
const READ_ALLOWANCE: TimeDelta = TimeDelta::hours(1);

#[derive(Debug, Clone, Deserialize)]
pub struct Push {
    pub repo: String,
    /// When the run started reading GitHub: everything in the push is at
    /// least this new.
    pub read_at: DateTime<Utc>,
    /// Every open PR of the repository.
    pub open: Vec<u64>,
    pub verdicts: Vec<PushedVerdict>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PushedVerdict {
    pub number: u64,
    pub head: String,
    /// The row the engine's `report --format json` prints for the PR.
    pub verdict: PolicyState,
}

/// Read and check a push received at `now`.
pub fn parse(body: &[u8], now: DateTime<Utc>) -> Result<Push, Invalid> {
    let push: Push =
        serde_json::from_slice(body).map_err(|e| Invalid(format!("not a push: {e}")))?;
    validate(&push, now)?;
    Ok(push)
}

fn validate(push: &Push, now: DateTime<Utc>) -> Result<(), Invalid> {
    if !is_repo(&push.repo) {
        return invalid("repo is not owner/name");
    }
    let skew = TimeDelta::from_std(CLOCK_SKEW).expect("a minute fits");
    if push.read_at > now + skew {
        return invalid("read_at is in the future");
    }
    bounded("open", push.open.len(), MAX_LIST)?;
    bounded("verdicts", push.verdicts.len(), MAX_LIST)?;
    if let Some(n) = push.open.iter().find(|n| !is_number(**n)) {
        return invalid(format!("open: {n} is not a PR number"));
    }
    let open: HashSet<u64> = push.open.iter().copied().collect();
    let mut seen = HashSet::new();
    for v in &push.verdicts {
        let at = format!("verdicts[{}]", v.number);
        if !open.contains(&v.number) {
            return invalid(format!("{at}: not listed in open"));
        }
        if !seen.insert(v.number) {
            return invalid(format!("{at}: listed twice"));
        }
        if !is_commit_sha(&v.head) {
            return invalid(format!("{at}: head is not a commit id"));
        }
        verdict(&at, &v.verdict)?;
    }
    Ok(())
}

/// The push is for the repository whose run minted the token, and was read
/// during that run.
pub fn bind(push: &Push, pusher: &VerifiedPusher) -> Result<(), Bound> {
    if push.repo != pusher.repo {
        return Err(Bound::OtherRepository);
    }
    if push.read_at < pusher.issued_at - READ_ALLOWANCE {
        return Err(Bound::Invalid(Invalid(
            "read_at is over an hour before the token was minted".into(),
        )));
    }
    Ok(())
}

pub enum Bound {
    /// The token's repository may push for itself only.
    OtherRepository,
    Invalid(Invalid),
}

fn is_number(n: u64) -> bool {
    n > 0 && i64::try_from(n).is_ok()
}

/// Every text and name in a verdict, bounded and shaped as a snapshot's are.
fn verdict(at: &str, v: &PolicyState) -> Result<(), Invalid> {
    engine_state(at, &v.state)?;
    text(at, "status", &v.status, 64)?;
    bounded(at, v.blockers.len(), MAX_LIST)?;
    for b in &v.blockers {
        text(at, "blockers", b, MAX_TEXT)?;
    }
    bounded(at, v.reviewers.len(), MAX_LIST)?;
    for r in &v.reviewers {
        login(at, r)?;
    }
    text(at, "title", &v.title, MAX_TITLE)?;
    if !v.author.is_empty() {
        login(at, &v.author)?;
    }
    bounded(at, v.areas.len(), MAX_LIST)?;
    for a in &v.areas {
        area(at, a)?;
    }
    if let Some(since) = &v.ready_since {
        text(at, "ready_since", since, 64)?;
    }
    bounded(at, v.approvals.len(), MAX_LIST)?;
    for a in &v.approvals {
        area(at, &a.area)?;
        bounded(at, a.approvers.len(), MAX_LIST)?;
        bounded(at, a.approved_by.len(), MAX_LIST)?;
        for l in a.approvers.iter().chain(&a.approved_by) {
            login(at, l)?;
        }
    }
    bounded(at, v.objectors.len(), MAX_LIST)?;
    for o in &v.objectors {
        login(at, o)?;
    }
    bounded(at, v.checklist.len(), MAX_LIST)?;
    for item in &v.checklist {
        text(at, "checklist", &item.item, MAX_TEXT)?;
    }
    Ok(())
}
