//! Review slots: each author holds at most five pull requests in review at
//! once, the ones admitted longest ago first.

use super::validate::{governs, validate_policy};
use super::values::{lower, or_empty_dict, time, Instant};
use crate::pycompat::object::{get, getitem, is_str, iterate, py_item_order, py_sort_by};
use crate::pycompat::{PyErr, PyInt, PyValue};
use indexmap::IndexMap;
use std::cmp::Ordering;

/// `effective_admission(pr)`: when the recorded admission says this pull
/// request took its slot, unless it went inactive (closed, drafted,
/// retargeted) at or after that, which gave the slot up.
pub fn effective_admission(pr: &PyValue) -> Result<&PyValue, PyErr> {
    let recorded = get(or_empty_dict(get(pr, "controller_state")?), "admitted_at")?;
    let inactive = get(pr, "lifecycle_at")?;
    if recorded.truthy() && inactive.truthy() && time(recorded)? <= time(inactive)? {
        return Ok(&PyValue::None);
    }
    Ok(recorded)
}

/// `admit(policy, prs, now)`: who holds a review slot, and since when.
///
/// Per author, the open, non-draft pull requests this policy governs are
/// ordered admitted ones first (by admission), then the rest by creation,
/// then by number; the first five hold a slot, since their admission or,
/// newly, since `now`. In the order Python's dict keeps them: by author as
/// first met, then by that order.
pub fn admit(
    policy: &PyValue,
    prs: &PyValue,
    now: &PyValue,
) -> Result<IndexMap<PyInt, PyValue>, PyErr> {
    validate_policy(policy)?;
    time(now)?;
    let prs = iterate(prs)?;
    let mut grouped: IndexMap<String, Vec<&PyValue>> = IndexMap::new();
    for pr in &prs {
        if is_str(getitem(pr, "state")?, "open")
            && !getitem(pr, "draft")?.truthy()
            && governs(policy, getitem(pr, "base")?)?
        {
            grouped
                .entry(lower(getitem(pr, "author")?)?)
                .or_default()
                .push(pr);
        }
    }
    let slots = match getitem(policy, "max_active_prs")? {
        PyValue::Int(i) => i
            .as_i64()
            .and_then(|n| usize::try_from(n).ok())
            .unwrap_or(0),
        _ => 0,
    };
    let mut result = IndexMap::new();
    for candidates in grouped.values() {
        let mut keyed = Vec::new();
        for pr in candidates {
            let admitted = effective_admission(pr)?;
            let key = if admitted.truthy() {
                (0, time(admitted)?, getitem(pr, "number")?)
            } else {
                (1, time(getitem(pr, "created_at")?)?, getitem(pr, "number")?)
            };
            keyed.push((key, *pr));
        }
        py_sort_by(&mut keyed, |(a, _), (b, _)| order(a, b))?;
        for (_, pr) in keyed.into_iter().take(slots) {
            let number = match getitem(pr, "number")? {
                PyValue::Int(number) => number.clone(),
                other => {
                    return Err(PyErr::Unported(format!(
                        "a slot keyed by a pull request number that is not an int: {other:?}"
                    )))
                }
            };
            let admitted = effective_admission(pr)?;
            let since = if admitted.truthy() { admitted } else { now };
            result.insert(number, since.clone());
        }
    }
    Ok(result)
}

/// `(group, instant, number)` tuples, as Python orders them.
fn order(a: &(u8, Instant, &PyValue), b: &(u8, Instant, &PyValue)) -> Result<Ordering, PyErr> {
    match a.0.cmp(&b.0).then(a.1.cmp(&b.1)) {
        Ordering::Equal => py_item_order(a.2, b.2),
        other => Ok(other),
    }
}
