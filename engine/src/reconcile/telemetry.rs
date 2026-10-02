//! What the review system's public status page says about a head:
//! `telemetry.head_state` of `pr_review/telemetry.py`.
//!
//! The page is a third party's: a stale, missing or hostile payload may only
//! make a report less informative or cause one extra nudge. It can never
//! hold a pull request back, so nothing here is consulted when deciding to
//! waive. Reading the page is the caller's; this reads what it said.

use super::values::{one_of, shown, time};
use crate::pycompat::object::{get, or, EMPTY_DICT};
use crate::pycompat::ops::{py_compare, py_eq, py_eq_str, py_sort_by, Compare};
use crate::pycompat::text::{py_len, py_split};
use crate::pycompat::{PyErr, PyValue};

/// The bot the status page reports on.
pub const BOT: &str = "thepastaclaw";
/// How old an answer may be, in seconds, before it says nothing.
const STALE_SECONDS: f64 = 20.0 * 60.0;

/// `_age(stamp, now)`: seconds between a payload's timestamp and now;
/// enormous when either cannot be read.
fn age(stamp: Option<&PyValue>, now: &PyValue) -> f64 {
    let stamp = stamp.unwrap_or(&PyValue::None);
    match (time(now), time(stamp)) {
        (Ok(now), Ok(stamp)) => match now.py_sub(&stamp) {
            Ok(delta) => delta.total_seconds(),
            Err(_) => f64::INFINITY,
        },
        _ => f64::INFINITY,
    }
}

/// `_rows(payload, section, key)`: the dict rows of a list in the payload,
/// or none.
fn rows<'p>(payload: &'p PyValue, section: &str, key: &str) -> Result<Vec<&'p PyValue>, PyErr> {
    let value = get(or(get(payload, section)?, &EMPTY_DICT), key)?;
    Ok(match value {
        Some(PyValue::List(items)) => items
            .iter()
            .filter(|row| matches!(row, PyValue::Dict(_)))
            .collect(),
        _ => Vec::new(),
    })
}

/// `head_state(payload, repository, number, head, head_seen_at, now)`:
/// `running`, `failed`, `queued`, or nothing visible said.
///
/// Only `head.queued` and `run.spawned` name a commit, and then only its
/// first eight characters; failures name none. So an event counts for this
/// head when it names a prefix of it, or names no commit at all and
/// happened after this head was first seen — a failure recorded before
/// that belongs to an earlier push. `running` is believed only while the
/// entry's own heartbeat and deadline say so, so a leaked entry whose runner
/// died cannot look alive for ever. `failed` means no receipt is ever
/// coming.
pub fn head_state(
    payload: &PyValue,
    repository: &PyValue,
    number: &PyValue,
    head: &PyValue,
    head_seen_at: &PyValue,
    now: &PyValue,
) -> Result<Option<&'static str>, PyErr> {
    if !payload.truthy() || age(get(payload, "data_as_of")?, now) > STALE_SECONDS {
        return Ok(None);
    }
    let field = |row: &PyValue, key: &str| -> Result<PyValue, PyErr> {
        Ok(get(row, key)?.cloned().unwrap_or(PyValue::None))
    };
    for row in rows(payload, "live", "active")? {
        let mine = py_eq(&field(row, "repo")?, repository)
            && py_eq(&field(row, "number")?, number)
            && matches!(&field(row, "sha")?, sha @ PyValue::Str(_) if py_eq(sha, head));
        if mine && py_eq_str(&field(row, "status")?, "running") {
            let fresh = age(get(row, "heartbeat_at")?, now) <= STALE_SECONDS;
            if fresh && age(get(row, "deadline_at")?, now) < 0.0 {
                return Ok(Some("running"));
            }
        }
    }
    let concerns_this_head = |event: &PyValue| -> Result<bool, PyErr> {
        let detail = or(get(event, "detail")?, &PyValue::None);
        let detail = if detail.truthy() {
            shown(detail)
        } else {
            String::new()
        };
        let named: Vec<&str> = py_split(&detail)
            .into_iter()
            .filter(|word| {
                py_len(word) >= 7 && word.chars().all(|c| "0123456789abcdef".contains(c))
            })
            .collect();
        if !named.is_empty() {
            let head = crate::pycompat::object::str_method(head, "startswith")?;
            return Ok(named.iter().any(|word| head.starts_with(word)));
        }
        Ok(head_seen_at.truthy() && age(get(event, "ts")?, head_seen_at) <= 0.0)
    };
    // `sorted(events, key=ts or '', reverse=True)` as CPython does it: every
    // key read in order, the list reversed, sorted ascending, and reversed
    // back, so equal keys keep their order and the same pairs are compared.
    let mut events = Vec::new();
    for event in rows(payload, "history", "recent_events")? {
        let ts = or(get(event, "ts")?, &PyValue::None);
        let key = if ts.truthy() {
            ts.clone()
        } else {
            PyValue::Str(String::new())
        };
        events.push((key, event));
    }
    events.reverse();
    py_sort_by(&mut events, |(a, _), (b, _)| py_compare(a, Compare::Lt, b))?;
    events.reverse();
    for (_, event) in events {
        if !py_eq(&field(event, "repo")?, repository) || !py_eq(&field(event, "number")?, number) {
            continue;
        }
        if !concerns_this_head(event)? {
            continue;
        }
        let kind = field(event, "kind")?;
        if one_of(&kind, &["head.failed", "run.failed"])?.is_some() {
            return Ok(Some("failed"));
        }
        if one_of(&kind, &["head.queued", "run.spawned"])?.is_some() {
            return Ok(Some("queued"));
        }
    }
    Ok(None)
}
