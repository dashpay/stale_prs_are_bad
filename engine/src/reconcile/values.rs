//! The small operations `main.py` repeats on its dynamic values, each
//! raising what Python raises on the wrong type.

use crate::pycompat::object::{getitem, iterate, py_str, str_method};
use crate::pycompat::ops::{py_hashable, py_type_name};
use crate::pycompat::text::{py_lower, py_slice};
use crate::pycompat::{PyDateTime, PyErr, PyInt, PyList, PyValue};

/// A string as a value.
pub(crate) fn s(text: impl Into<String>) -> PyValue {
    PyValue::Str(text.into())
}

/// A list as a value.
pub(crate) fn list(items: Vec<PyValue>) -> PyValue {
    PyValue::List(PyList::from(items))
}

/// `pr['number']` of a pull request the reader built, which is always an
/// `int`: anything else is refused rather than guessed at.
pub(crate) fn number(pr: &PyValue) -> Result<PyInt, PyErr> {
    match getitem(pr, "number")? {
        PyValue::Int(number) => Ok(number.clone()),
        other => Err(PyErr::Unported(format!(
            "a pull request number that is not an int: {other:?}"
        ))),
    }
}

/// The text of a value Python uses as a string, where anything else is a
/// `TypeError`: a `+`, a `.join`, a status payload.
pub(crate) fn text<'v>(value: &'v PyValue, what: &str) -> Result<&'v str, PyErr> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(PyErr::type_error(format!(
            "{what}: expected str instance, {} found",
            py_type_name(other)
        ))),
    }
}

/// `value.lower()`.
pub(crate) fn lower(value: &PyValue) -> Result<String, PyErr> {
    Ok(py_lower(str_method(value, "lower")?))
}

/// `separator.join(value)`: every item a string.
pub(crate) fn join(separator: &str, value: &PyValue) -> Result<String, PyErr> {
    let items = iterate(value)?;
    let mut parts = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        match item.as_ref() {
            PyValue::Str(part) => parts.push(part.clone()),
            other => {
                return Err(PyErr::type_error(format!(
                    "sequence item {index}: expected str instance, {} found",
                    py_type_name(other)
                )))
            }
        }
    }
    Ok(parts.join(separator))
}

/// `value[:end]` of a string.
pub(crate) fn prefix(value: &PyValue, end: isize) -> Result<&str, PyErr> {
    match value {
        PyValue::Str(s) => Ok(py_slice(s, None, Some(end))),
        other => Err(PyErr::type_error(format!(
            "'{}' object is not subscriptable",
            py_type_name(other)
        ))),
    }
}

/// `f"{value}"`.
pub(crate) fn shown(value: &PyValue) -> String {
    py_str(value).into_owned()
}

/// `value in {...}` or `{...}.get(value)` for a set of strings: an
/// unhashable value raises; anything else is in it only as one of them.
pub(crate) fn one_of<'t>(value: &PyValue, set: &[&'t str]) -> Result<Option<&'t str>, PyErr> {
    py_hashable(value)?;
    Ok(match value {
        PyValue::Str(s) => set.iter().copied().find(|item| *item == s.as_str()),
        _ => None,
    })
}

/// `policy._time(value)`: an instant with an offset, or the error Python
/// raises for anything else.
pub(crate) fn time(value: &PyValue) -> Result<PyDateTime, PyErr> {
    let parsed = PyDateTime::fromisoformat(&str_method(value, "replace")?.replace('Z', "+00:00"))?;
    if parsed.utcoffset().is_none() {
        return Err(PyErr::value("Timestamp requires timezone"));
    }
    Ok(parsed)
}
