//! The small operations the policy repeats on its dynamic inputs: Python's
//! `x.lower()`, `x.upper() == 'WORD'`, `re` on a value that may not be
//! text, `_time`, and building the values a verdict holds.

use crate::pycompat::object::str_method;
use crate::pycompat::ops::py_type_name;
use crate::pycompat::text::{py_lower, py_upper_ascii};
use crate::pycompat::{PyDateTime, PyErr, PyValue};
use std::cmp::Ordering;

/// `value.lower()`.
pub(crate) fn lower(value: &PyValue) -> Result<String, PyErr> {
    Ok(py_lower(str_method(value, "lower")?))
}

/// The strings of a list `validate_policy` accepted as a list of strings.
/// Anything else cannot reach here; it is refused rather than read.
pub(crate) fn strings(value: &PyValue) -> Result<Vec<&str>, PyErr> {
    let refused = || PyErr::Unported(format!("a list of strings after validation: {value:?}"));
    match value {
        PyValue::List(items) => items
            .iter()
            .map(|item| match item {
                PyValue::Str(s) => Ok(s.as_str()),
                _ => Err(refused()),
            })
            .collect(),
        _ => Err(refused()),
    }
}

/// The string `validate_policy` accepted at `value`.
pub(crate) fn string(value: &PyValue) -> Result<&str, PyErr> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(PyErr::Unported(format!(
            "a string after validation: {other:?}"
        ))),
    }
}

/// `value.upper()`, where only a comparison with ASCII words follows:
/// `None` when the uppercase is not ASCII and so equals none of them.
pub(crate) fn upper(value: &PyValue) -> Result<Option<String>, PyErr> {
    Ok(py_upper_ascii(str_method(value, "upper")?))
}

/// `value.upper() in words`, for ASCII words.
pub(crate) fn upper_in(value: &PyValue, words: &[&str]) -> Result<bool, PyErr> {
    Ok(upper(value)?.is_some_and(|upper| words.contains(&upper.as_str())))
}

/// The text `re.search(pattern, value)` and its kin read; Python raises
/// `TypeError` for anything else.
pub(crate) fn re_text(value: &PyValue) -> Result<&str, PyErr> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(PyErr::type_error(format!(
            "expected string or bytes-like object, got '{}'",
            py_type_name(other)
        ))),
    }
}

/// An aware instant, as `_time` returns one. Only [`time_text`] makes one,
/// so every `Instant` has an offset.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Instant(PyDateTime);

impl Instant {
    pub(crate) fn datetime(&self) -> &PyDateTime {
        &self.0
    }
}

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        // Both are aware, since `time` refuses a naive value, and two aware
        // datetimes always compare.
        self.0.py_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Instant {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Instant {}

/// `_time(value)`: `datetime.fromisoformat(value.replace('Z', '+00:00'))`,
/// refusing a value without a timezone.
pub(crate) fn time(value: &PyValue) -> Result<Instant, PyErr> {
    time_text(str_method(value, "replace")?)
}

/// `_time` of text the engine holds.
pub(crate) fn time_text(text: &str) -> Result<Instant, PyErr> {
    let parsed = PyDateTime::fromisoformat(&text.replace('Z', "+00:00"))?;
    if parsed.utcoffset().is_none() {
        return Err(PyErr::value("Timestamp requires timezone"));
    }
    Ok(Instant(parsed))
}

/// `max(items, key=_time)`: the first of the latest. Every key is read, in
/// order, before any is compared, as Python reads them.
pub(crate) fn latest<'a, I>(items: I) -> Result<Option<&'a PyValue>, PyErr>
where
    I: IntoIterator<Item = &'a PyValue>,
{
    let keyed = items
        .into_iter()
        .map(|item| Ok((time(item)?, item)))
        .collect::<Result<Vec<_>, PyErr>>()?;
    Ok(crate::pycompat::py_max_by_key(keyed, |(key, _)| *key).map(|(_, item)| item))
}

/// `min(items, key=_time)`: the first of the earliest.
pub(crate) fn earliest<'a, I>(items: I) -> Result<Option<&'a PyValue>, PyErr>
where
    I: IntoIterator<Item = &'a PyValue>,
{
    let keyed = items
        .into_iter()
        .map(|item| Ok((time(item)?, item)))
        .collect::<Result<Vec<_>, PyErr>>()?;
    Ok(crate::pycompat::py_min_by_key(keyed, |(key, _)| *key).map(|(_, item)| item))
}

/// A string as a value.
pub(crate) fn s(text: impl Into<String>) -> PyValue {
    PyValue::Str(text.into())
}

/// A list of strings as a value.
pub(crate) fn strs<I, T>(items: I) -> PyValue
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    PyValue::List(items.into_iter().map(s).collect())
}

/// A dict, its keys in the order given.
pub(crate) fn dict<const N: usize>(entries: [(&str, PyValue); N]) -> PyValue {
    PyValue::Dict(
        entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}
