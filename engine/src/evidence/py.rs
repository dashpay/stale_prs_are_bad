//! The reader's own reading helpers: its compiled patterns, the values it
//! builds, and `github.py`'s `_text` and `_login`. Subscripts, `.get` and
//! iteration are [`crate::pycompat::object`]'s.

use super::error::{PyClass, ReadError};
use crate::pycompat::object::get;
use crate::pycompat::ops::py_type_name;
use crate::pycompat::re::translate;
use crate::pycompat::{PyDict, PyList, PyValue};
use regex::Regex;

pub(crate) type Read<T> = Result<T, ReadError>;

/// A pattern written in Python's syntax, compiled with the classes Python
/// gives `\s`, `\w` and `\d`. Only constant patterns are passed here, and
/// each module's tests force every one of them, so a pattern that does not
/// translate or compile fails the suite and never reaches a run.
pub(crate) fn compiled(pattern: &str) -> Regex {
    let translated = translate(pattern).expect("a constant pattern translates");
    Regex::new(&translated).expect("a constant pattern compiles")
}

pub(crate) fn str_value(text: impl Into<String>) -> PyValue {
    PyValue::Str(text.into())
}

/// `None` or the text.
pub(crate) fn optional(text: Option<String>) -> PyValue {
    text.map_or(PyValue::None, PyValue::Str)
}

pub(crate) fn dict(entries: PyDict) -> PyValue {
    PyValue::Dict(entries)
}

pub(crate) fn list(items: Vec<PyValue>) -> PyValue {
    PyValue::List(PyList::from(items))
}

/// A value Python passes to `str` methods or to `re`: anything else is a
/// `TypeError` there.
pub(crate) fn as_str<'a>(value: &'a PyValue, what: &str) -> Read<&'a str> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(ReadError::exception(
            PyClass::TypeError,
            format!("{what} must be a string, not '{}'", py_type_name(other)),
        )),
    }
}

/// `_text(value, label)`: a non-empty string, or the reader's error naming
/// what was missing.
pub(crate) fn text<'a>(value: Option<&'a PyValue>, label: &str) -> Read<&'a str> {
    match value {
        Some(PyValue::Str(s)) if !s.is_empty() => Ok(s),
        _ => Err(ReadError::github(format!("Missing or invalid {label}"))),
    }
}

/// `_login(user)`: the login of an account object.
pub(crate) fn login(user: &PyValue) -> Read<String> {
    if !matches!(user, PyValue::Dict(_)) {
        return Err(ReadError::github("Missing account identity"));
    }
    Ok(text(get(user, "login")?, "account login")?.to_owned())
}
