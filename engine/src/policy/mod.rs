//! The review policy of `pr_review/policy.py`: who owns what, what review
//! evidence a pull request has, and the verdict it earns.
//!
//! Every function here is pure: it reads the policy and a snapshot of the
//! pull request, both as [`PyValue`]s the way Python's engine holds them,
//! and the time is an argument. Inputs stay dynamic because Python works on
//! partial input before it validates it — the carried diff is read before
//! the policy is checked, and an absent key is not a `null` one — and each
//! function raises the exception Python raises, so that [`evaluate`] turns
//! the same ones into a configuration error ([`PyErr::Value`], `Type` and
//! `Key`) and lets the same others fail the run.
//!
//! - [`evaluate`]: the verdict, in the dict Python returns.
//! - [`validate_policy`], [`governs`]: reading a policy.
//! - [`admit`], [`effective_admission`]: who holds a review slot.
//! - [`diff_print`], [`carried_heads`], [`fingerprint`]: what a reviewer
//!   read, the commits that carry it, and the evidence a verdict used.
//! - [`receipt_print`], [`receipt_instant`], [`finding_severities`],
//!   [`finding_blocks`]: what the review bots said.
//! - [`bot_schedule`], [`skipped_by`], [`nudged_at`], [`rate_limited_at`]:
//!   waiting for the bots, and when to stop.
//! - [`holders`], [`machine_author`], [`is_engine`]: who is who.
//!
//! The policy's regular expressions are in `patterns`, each beside the
//! Python pattern it ports.

mod admission;
mod bots;
mod evaluate;
mod patterns;
mod prints;
mod receipts;
mod validate;
mod values;

pub use admission::{admit, effective_admission};
pub use bots::{bot_schedule, nudged_at, rate_limited_at, skipped_by, Schedule, Skip};
pub use evaluate::evaluate;
pub use prints::{carried_heads, diff_print, fingerprint};
pub use receipts::{finding_blocks, finding_severities, receipt_instant, receipt_print};
pub use validate::{governs, validate_policy};

use crate::pycompat::object::{get, get_or, getitem, iterate, or, EMPTY_LIST, EMPTY_STR, NONE};
use crate::pycompat::ops::py_in_str_set;
use crate::pycompat::{PyDict, PyErr, PyValue};
use std::collections::BTreeSet;
use values::lower;

/// The marker of this controller's record comment.
pub const STATE_MARKER: &str = "platform-pr-review-state-v1";
/// The marker this controller's nudge comments start with.
pub const NUDGE_MARKER: &str = "<!-- pr-hygiene-nudge v1";
/// The review bots, by every login they write under.
pub const BOTS: [&str; 4] = [
    "thepastaclaw",
    "coderabbitai",
    "coderabbitai[bot]",
    "claudbot[bot]",
];
/// The review bots a policy may require. Which bots a repository runs is a
/// property of that repository, not of the review rules.
pub const REVIEW_BOTS: [&str; 2] = ["thepastaclaw", "coderabbitai"];
/// The accounts this engine writes as. Its records, statuses, nudges and
/// prints are recognised only by who wrote them; another engine continuing
/// the same pull requests is listed here before it writes.
pub const ENGINE_LOGINS: [&str; 1] = ["github-actions[bot]"];
/// The permission levels that can merge.
pub const WRITE: [&str; 3] = ["write", "maintain", "admin"];
/// The label each state wears; states not listed wear none.
pub const LABEL_FOR_STATE: [(&str, &str); 5] = [
    ("waiting-bots", "waiting-bots"),
    ("waiting-self-review", "waiting-self-review"),
    ("waiting-author", "waiting-self-review"),
    ("too-many-open-prs", "too-many-open-prs"),
    ("ready-for-human", "ready-for-human"),
];
/// The labels this controller manages, once each, in that order.
pub const STATE_LABELS: [&str; 4] = [
    "waiting-bots",
    "waiting-self-review",
    "too-many-open-prs",
    "ready-for-human",
];
/// Labels this controller used to set, cleared wherever still seen.
pub const RETIRED_LABELS: [&str; 4] = [
    "waiting-slot",
    "waiting-build",
    "waiting-author",
    "ready-to-merge",
];
/// The marker the move comment starts with.
pub const MOVE_MARKER: &str = "<!-- pr-hygiene:move";
/// Where this controller's checklist starts in a description.
pub const CHECKLIST_START: &str = "<!-- pr-hygiene:start -->";
/// Where it ends.
pub const CHECKLIST_END: &str = "<!-- pr-hygiene:end -->";
/// The marker a CodeRabbit receipt carries before the JSON naming the
/// commit it covers.
pub const RECEIPT_MARKER: &str = "final_review_risk_coverage";
/// What every spelling the attestation pattern accepts contains, so that a
/// comment holding one starts a run at once.
pub const ATTESTATION_TRIGGERS: [&str; 3] = ["/self-review", "/selfreview", "/self review"];
/// Where CodeRabbit's notice that it is rate limited starts.
pub const RATE_LIMITED: &str =
    "<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->";
/// The words of that notice the caller workflow looks for.
pub const RATE_LIMITED_MARKER: &str = "rate limited by coderabbit.ai";
/// Where the notice ends.
pub const RATE_LIMITED_END: &str =
    "<!-- end of auto-generated comment: rate limited by coderabbit.ai -->";

/// The sections CodeRabbit rewrites without meaning anything by them, as
/// the names of their `_start` and `_end` markers.
const VOLATILE: [(&str, &str); 3] = [
    ("review_stack_entry", "review_stack_entry"),
    ("tips", "tips"),
    ("finishing_touch_checkbox", "finishing_touch_checkbox"),
];

/// CodeRabbit's notices about when it can review and whether it did: its
/// capacity, a review it skipped, one in progress, one paused, and an
/// invitation to praise it. Its tool failures are not here: those name the
/// file and line that stopped a tool, which is a finding about the code.
const NOTICES: [(&str, &str); 5] = [
    (
        "<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->",
        "<!-- end of auto-generated comment: rate limited by coderabbit.ai -->",
    ),
    (
        "<!-- This is an auto-generated comment: skip review by coderabbit.ai -->",
        "<!-- end of auto-generated comment: skip review by coderabbit.ai -->",
    ),
    (
        "<!-- This is an auto-generated comment: review in progress by coderabbit.ai -->",
        "<!-- end of auto-generated comment: review in progress by coderabbit.ai -->",
    ),
    (
        "<!-- This is an auto-generated comment: review paused by coderabbit.ai -->",
        "<!-- end of auto-generated comment: review paused by coderabbit.ai -->",
    ),
    (
        "<!-- This is an auto-generated comment: tweet message by coderabbit.ai -->",
        "<!-- end of auto-generated comment: tweet message by coderabbit.ai -->",
    ),
];

/// How each review bot labels a finding, and whether the label holds the
/// pull request until its thread is resolved. A label not in that bot's
/// table, or none, holds it too.
const FINDING_BLOCKS: [(&str, &[(&str, bool)]); 2] = [
    (
        "thepastaclaw",
        &[
            ("🔴 Blocking", true),
            ("🟡 Suggestion", false),
            ("💬 Nitpick", false),
        ],
    ),
    (
        "coderabbitai",
        &[
            ("🔴 Critical", true),
            ("🟠 Major", true),
            ("🟡 Minor", false),
            ("🔵 Trivial", false),
        ],
    ),
];

/// `is_engine(login)`: whether `login` is one of this engine's identities,
/// in any case. Only the `name[bot]` spelling counts: the bare name is one
/// a person can register.
pub fn is_engine(login: Option<&PyValue>) -> Result<bool, PyErr> {
    let login = lower(or(login, &EMPTY_STR))?;
    Ok(ENGINE_LOGINS.contains(&login.as_str()))
}

/// `policy.get('bot_authors', [])`, lowercased.
fn bot_authors(policy: &PyValue) -> Result<BTreeSet<String>, PyErr> {
    iterate(get_or(policy, "bot_authors", &EMPTY_LIST)?)?
        .iter()
        .map(|handle| lower(handle))
        .collect()
}

/// `holders(policy, pr)`: whoever this pull request belongs to — the one
/// who opened it and anyone it was handed to — as lowercase logins. Holding
/// it confers only that they may say they read it. Machine accounts are not
/// holders: GitHub's own, the policy's `bot_authors`, and any `[bot]`.
pub fn holders(policy: &PyValue, pr: &PyValue) -> Result<BTreeSet<String>, PyErr> {
    let mut named = BTreeSet::from([lower(getitem(pr, "author")?)?]);
    for who in iterate(or(get(pr, "assignees")?, &EMPTY_LIST))? {
        named.insert(lower(&who)?);
    }
    let machines = bot_authors(policy)?;
    Ok(named
        .into_iter()
        .filter(|who| {
            !machines.contains(who) && !BOTS.contains(&who.as_str()) && !who.ends_with("[bot]")
        })
        .collect())
}

/// `machine_author(policy, pr)`: whether this pull request was opened by
/// something that cannot attest for itself — an account GitHub marks a
/// bot, a review bot, or one the policy names under `bot_authors`.
pub fn machine_author(policy: &PyValue, pr: &PyValue) -> Result<bool, PyErr> {
    if get_or(pr, "author_is_bot", &NONE)?.truthy() {
        return Ok(true);
    }
    let author = lower(or(get(pr, "author")?, &EMPTY_STR))?;
    Ok(BOTS.contains(&author.as_str()) || bot_authors(policy)?.contains(&author))
}

/// `_may_object(permissions, user)`: whether this person's objection counts.
/// An answer that never arrived is kept: dropping an objection on the
/// strength of an unreadable permission would be the one place that
/// uncertainty let something through.
fn may_object(permissions: &PyDict, user: &str) -> Result<bool, PyErr> {
    let level = permissions.get(user).unwrap_or(&PyValue::None);
    Ok(py_in_str_set(level, &WRITE)? || matches!(level, PyValue::None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_bot_spelling_is_the_engine_in_any_case() {
        let login = |text: &str| PyValue::Str(text.to_owned());
        assert!(is_engine(Some(&login("github-actions[bot]"))).unwrap());
        assert!(is_engine(Some(&login("GitHub-Actions[BOT]"))).unwrap());
        // The bare name is one a person can register.
        assert!(!is_engine(Some(&login("github-actions"))).unwrap());
        assert!(!is_engine(Some(&login(""))).unwrap());
        assert!(!is_engine(None).unwrap());
        assert!(!is_engine(Some(&PyValue::None)).unwrap());
        // Something true that is not a string has no `lower`.
        assert!(matches!(
            is_engine(Some(&PyValue::Bool(true))),
            Err(PyErr::Attribute(_))
        ));
    }
}
