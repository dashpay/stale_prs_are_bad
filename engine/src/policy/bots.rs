//! Waiting for the review bots: when to nudge a bot that has not reported,
//! when to proceed without it, and the two things that end the wait early —
//! a writer's `/skip-bots`, and CodeRabbit announcing its own rate limit.
//!
//! Positive evidence that a review is in flight delays a nudge, never a
//! waiver, so a stale or hostile status page can cost one extra nudge but
//! can never hold a pull request back.

use super::values::{earliest, latest, lower, s, time, time_text};
use super::{is_engine, NUDGE_MARKER, RATE_LIMITED, RATE_LIMITED_END, WRITE};
use crate::pycompat::object::{get, getitem, iterate, or, py_str, str_method, EMPTY_STR};
use crate::pycompat::ops::{
    py_compare, py_contains, py_eq, py_eq_str, py_in_str_set, py_type_name, Compare,
};
use crate::pycompat::text::py_strip;
use crate::pycompat::{
    py_max_by_key, py_min_by_key, PyDateTime, PyDict, PyErr, PyList, PyTimeDelta, PyValue,
};

/// CodeRabbit's logins.
const RABBIT: [&str; 2] = ["coderabbitai", "coderabbitai[bot]"];

/// What `bot_schedule` decided for one bot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    /// Ask the bot to review now.
    pub nudge: bool,
    /// When the policy stopped requiring the bot, if it has.
    pub waived_at: Option<String>,
    /// `window` or `rate-limit`, beside `waived_at`.
    pub waived_reason: Option<&'static str>,
}

impl Schedule {
    const NOTHING: Schedule = Schedule {
        nudge: false,
        waived_at: None,
        waived_reason: None,
    };
}

/// A pull request as `bot_schedule` reads it: the snapshot, with the keys
/// `evaluate` replaces for the diff it carries
/// (`dict(pr, head_seen_at=..., reviewed_heads=...)`).
pub(crate) struct Facts<'a> {
    pub(crate) pr: &'a PyValue,
    pub(crate) overrides: &'a [(&'a str, &'a PyValue)],
}

impl<'a> Facts<'a> {
    fn replaced(&self, key: &str) -> Option<&'a PyValue> {
        self.overrides
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| *value)
    }

    /// `pr.get(key)`.
    fn get(&self, key: &str) -> Result<&'a PyValue, PyErr> {
        self.replaced(key).map_or_else(|| get(self.pr, key), Ok)
    }

    /// `pr[key]`.
    fn item(&self, key: &str) -> Result<&'a PyValue, PyErr> {
        self.replaced(key).map_or_else(|| getitem(self.pr, key), Ok)
    }
}

/// `_hours(stamp, now)`: hours from `stamp` to `now`, as a float.
fn hours(stamp: &PyValue, now: &PyValue) -> Result<f64, PyErr> {
    let now = time(now)?;
    let then = time(stamp)?;
    Ok(now.datetime().py_sub(then.datetime())?.total_seconds() / 3600.0)
}

/// `hours >= bound`, a float against whatever the policy holds.
fn at_least(hours: f64, bound: &PyValue) -> Result<bool, PyErr> {
    py_compare(&PyValue::Float(hours), Compare::Ge, bound)
}

/// `timedelta(hours=hours)` for the whole hours a policy holds.
fn whole_hours(hours: &PyValue) -> Result<PyTimeDelta, PyErr> {
    let count = match hours {
        PyValue::Int(i) => i.as_i64(),
        PyValue::Bool(b) => Some(i64::from(*b)),
        // `validate_policy` accepts whole hours only.
        PyValue::Float(_) => return Err(PyErr::Unported("a timedelta of fractional hours".into())),
        other => {
            return Err(PyErr::type_error(format!(
                "unsupported type for timedelta hours component: {}",
                py_type_name(other)
            )))
        }
    };
    count
        .and_then(PyTimeDelta::checked_from_hours)
        .ok_or_else(|| PyErr::Overflow("date value out of range".into()))
}

/// `instant.isoformat().replace('+00:00', 'Z')`.
fn utc_z(instant: &PyDateTime) -> String {
    instant.isoformat().replace("+00:00", "Z")
}

/// Who told this controller not to wait for the bots, and when.
#[derive(Debug, Clone)]
pub struct Skip {
    pub user: PyValue,
    pub at: PyValue,
}

/// `skipped_by(comments, permissions, head_seen_at)`: the earliest unedited
/// `/skip-bots` a writer posted after this head first had a status. Any
/// writer may, the author included; whoever reviews next sees who did. A
/// skip written for an earlier head cannot carry.
pub fn skipped_by(
    comments: &PyValue,
    permissions: &PyDict,
    head_seen_at: &PyValue,
) -> Result<Option<Skip>, PyErr> {
    if !head_seen_at.truthy() {
        return Ok(None);
    }
    let mut skips = Vec::new();
    for c in iterate(comments)? {
        if py_strip(str_method(getitem(&c, "body")?, "strip")?) != "/skip-bots" {
            continue;
        }
        let created = getitem(&c, "created_at")?;
        if !py_eq(created, getitem(&c, "updated_at")?) {
            continue;
        }
        let user = getitem(&c, "user")?;
        let level = permissions.get(&lower(user)?).unwrap_or(&PyValue::None);
        if !py_in_str_set(level, &WRITE)? {
            continue;
        }
        if time(created)? <= time(head_seen_at)? {
            continue;
        }
        skips.push(Skip {
            user: user.clone(),
            at: created.clone(),
        });
    }
    let keyed = skips
        .into_iter()
        .map(|skip| Ok((time(&skip.at)?, skip)))
        .collect::<Result<Vec<_>, PyErr>>()?;
    Ok(py_min_by_key(keyed, |(key, _)| *key).map(|(_, skip)| skip))
}

/// `nudged_at(comments, bot, heads)`: when this controller last asked `bot`
/// to look at this work. A nudge about any commit carrying the same diff
/// counts: asking again because the base moved underneath is noise.
pub fn nudged_at(comments: &PyValue, bot: &str, heads: &PyValue) -> Result<Option<PyValue>, PyErr> {
    let heads = match heads {
        PyValue::Str(_) => vec![std::borrow::Cow::Borrowed(heads)],
        other => iterate(other)?,
    };
    let markers: Vec<String> = heads
        .iter()
        .map(|h| format!("{NUDGE_MARKER} bot={bot} sha={} -->", py_str(h)))
        .collect();
    let comments = iterate(comments)?;
    let mut stamps = Vec::new();
    for c in &comments {
        if !is_engine(getitem(c, "user")?)? {
            continue;
        }
        let mut asked = false;
        for marker in &markers {
            if py_contains(getitem(c, "body")?, &s(marker.as_str()))? {
                asked = true;
                break;
            }
        }
        if asked {
            stamps.push(getitem(c, "created_at")?);
        }
    }
    Ok(latest(stamps)?.cloned())
}

/// `_limited_block(body)`: the rate-limit notice alone, without the rest of
/// the comment around it; nothing when it has no end, since reading to the
/// end of the comment would let the walkthrough speak for the limit.
fn limited_block(body: &str) -> &str {
    let Some(start) = body.find(RATE_LIMITED) else {
        return "";
    };
    match body[start..].find(RATE_LIMITED_END) {
        Some(end) => &body[start..start + end],
        None => "",
    }
}

/// `rate_limited_at(comments, head_seen_at, head)`: when CodeRabbit first
/// said, about this head, that it was rate limited.
///
/// It keeps one comment and edits it, so the notice is read from its last
/// edit, but only an edit CodeRabbit made itself counts. A notice counts
/// when it names this head, or was written after the head was first seen.
/// The instant is a property of the head: the later of the notice and the
/// head, never the moment of the edit that revealed it.
pub fn rate_limited_at(
    comments: &PyValue,
    head_seen_at: &PyValue,
    head: &PyValue,
) -> Result<Option<PyValue>, PyErr> {
    if !head_seen_at.truthy() {
        return Ok(None);
    }
    let mut stamps: Vec<PyValue> = Vec::new();
    for c in iterate(comments)? {
        if !RABBIT.contains(&lower(getitem(&c, "user")?)?.as_str())
            || !py_contains(getitem(&c, "body")?, &s(RATE_LIMITED))?
        {
            continue;
        }
        let updated = get(&c, "updated_at")?;
        let edited = if updated.truthy() {
            updated
        } else {
            getitem(&c, "created_at")?
        };
        let about_this_head = if head.truthy() {
            let block = limited_block(str_method(getitem(&c, "body")?, "find")?);
            match head {
                PyValue::Str(head) => block.contains(head.as_str()),
                other => {
                    return Err(PyErr::type_error(format!(
                        "must be str, not {}",
                        py_type_name(other)
                    )))
                }
            }
        } else {
            false
        };
        if !about_this_head && time(edited)? < time(head_seen_at)? {
            continue;
        }
        // Unedited, it is the bot's own words. Edited, only if the bot is
        // who edited it; where that cannot be known, the ordinary window
        // applies rather than a skip nobody can attribute.
        let created = getitem(&c, "created_at")?;
        if !py_eq(edited, created) {
            let editor = lower(or(get(&c, "edited_by")?, &EMPTY_STR))?;
            if !RABBIT.contains(&editor.as_str()) {
                continue;
            }
        }
        let pair = [created, head_seen_at];
        let keyed = pair
            .iter()
            .map(|stamp| Ok((time(stamp)?, *stamp)))
            .collect::<Result<Vec<_>, PyErr>>()?;
        if let Some((_, later)) = py_max_by_key(keyed, |(key, _)| *key) {
            stamps.push(later.clone());
        }
    }
    Ok(earliest(stamps.iter())?.cloned())
}

/// `bot_schedule(policy, pr, bot, now, telemetry_state)`: whether to nudge
/// a missing bot now, and whether to stop requiring it.
///
/// Without timeouts, or before this head has a status, nothing happens.
/// A bot is nudged once the nudge window passes, at once when the status
/// page says its review failed, and not while the page says it is running
/// or queued. It is waived when the waiver window passes, at an instant
/// fixed by the head, not the run; CodeRabbit is waived an hour after it
/// announced its own rate limit, if that is sooner.
pub fn bot_schedule(
    policy: &PyValue,
    pr: &PyValue,
    bot: &str,
    now: &PyValue,
    telemetry_state: &PyValue,
) -> Result<Schedule, PyErr> {
    schedule(
        policy,
        &Facts { pr, overrides: &[] },
        bot,
        now,
        telemetry_state,
    )
}

pub(crate) fn schedule(
    policy: &PyValue,
    pr: &Facts<'_>,
    bot: &str,
    now: &PyValue,
    telemetry_state: &PyValue,
) -> Result<Schedule, PyErr> {
    let timeouts = get(policy, "bot_timeouts")?;
    let seen = pr.get("head_seen_at")?;
    if !timeouts.truthy() || !seen.truthy() {
        return Ok(Schedule::NOTHING);
    }
    let waited = hours(seen, now)?;
    let nudge_after = getitem(timeouts, "nudge_after_hours")?;
    let waive_after = getitem(timeouts, "waive_after_hours")?;
    let comments = pr.item("comments")?;
    let reviewed = pr.get("reviewed_heads")?;
    let this_head;
    let heads = if reviewed.truthy() {
        reviewed
    } else {
        this_head = PyValue::List(PyList::from(vec![pr.item("head")?.clone()]));
        &this_head
    };
    let already = nudged_at(comments, bot, heads)?;

    let limited = if bot == "coderabbitai" {
        rate_limited_at(pr.item("comments")?, seen, pr.get("head")?)?
    } else {
        None
    };
    let due = match &limited {
        // CodeRabbit announced its own limit and documents this retry.
        Some(limited) => hours(limited, now)? >= 1.0,
        // No receipt is ever coming for this head.
        None if py_eq_str(telemetry_state, "failed") => true,
        // In flight: nudging would only add load.
        None if py_in_str_set(telemetry_state, &["running", "queued"])? => false,
        None => at_least(waited, nudge_after)?,
    };

    // The moment a waiver takes effect is a property of the head, not of
    // the run that noticed it, so it does not move with the clock.
    let mut due_at = utc_z(&time(seen)?.datetime().py_add(whole_hours(waive_after)?)?);
    let mut waived = at_least(waited, waive_after)?;
    let mut reason = waived.then_some("window");
    // A bot that has said it cannot review this head is not one that has
    // not answered yet: an hour after its notice, proceed without it.
    if let Some(limited) = &limited {
        if hours(limited, now)? >= 1.0 {
            let hour = whole_hours(&PyValue::Int(1.into()))?;
            let limit_at = utc_z(&time(limited)?.datetime().py_add(hour)?);
            if !waived || time_text(&limit_at)? < time_text(&due_at)? {
                due_at = limit_at;
                waived = true;
                reason = Some("rate-limit");
            }
        }
    }
    Ok(Schedule {
        nudge: due && already.is_none() && !waived,
        waived_at: waived.then_some(due_at),
        waived_reason: reason,
    })
}
