//! Reading a [`PyValue`] as Python code reads an object: `value[key]`,
//! `value.get(key)`, iteration and the reader's own `_text` and `_login`,
//! each raising what Python raises when the value is not what was expected.

use super::error::{PyClass, ReadError};
use super::rules;
use crate::pycompat::ops::py_type_name;
use crate::pycompat::re::translate;
use crate::pycompat::{PyDict, PyList, PyValue};
use regex::Regex;
use std::borrow::Cow;

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

fn type_error(detail: impl Into<String>) -> ReadError {
    ReadError::exception(PyClass::TypeError, detail)
}

fn attribute_error(value: &PyValue, attribute: &str) -> ReadError {
    ReadError::exception(
        PyClass::AttributeError,
        format!(
            "'{}' object has no attribute '{attribute}'",
            py_type_name(value)
        ),
    )
}

/// `value[key]`: a `KeyError` from a dict without it, a `TypeError` from
/// anything that is not a dict.
pub(crate) fn item<'a>(value: &'a PyValue, key: &str) -> Read<&'a PyValue> {
    match value {
        PyValue::Dict(entries) => entries
            .get(key)
            .ok_or_else(|| ReadError::exception(PyClass::KeyError, format!("'{key}'"))),
        PyValue::List(_) => Err(type_error(
            "list indices must be integers or slices, not str",
        )),
        PyValue::Str(_) => Err(type_error("string indices must be integers, not 'str'")),
        other => Err(type_error(format!(
            "'{}' object is not subscriptable",
            py_type_name(other)
        ))),
    }
}

/// `value.get(key)`: only a dict has one.
pub(crate) fn get<'a>(value: &'a PyValue, key: &str) -> Read<Option<&'a PyValue>> {
    match value {
        PyValue::Dict(entries) => Ok(entries.get(key)),
        other => Err(attribute_error(other, "get")),
    }
}

/// `value or default`, for a value that may be absent: the value where it
/// is true, nothing where it is absent or false.
pub(crate) fn or_default(value: Option<&PyValue>) -> Option<&PyValue> {
    value.filter(|v| v.truthy())
}

/// `(value or {}).get(key)`.
pub(crate) fn get_or_empty<'a>(value: Option<&'a PyValue>, key: &str) -> Read<Option<&'a PyValue>> {
    match or_default(value) {
        Some(v) => get(v, key),
        None => Ok(None),
    }
}

/// `for x in value`: a list's items, a dict's keys, a string's characters.
pub(crate) fn iterate(value: &PyValue) -> Read<Vec<Cow<'_, PyValue>>> {
    match value {
        PyValue::List(items) => Ok(items.iter().map(Cow::Borrowed).collect()),
        PyValue::Dict(entries) => Ok(entries
            .keys()
            .map(|key| Cow::Owned(PyValue::Str(key.clone())))
            .collect()),
        PyValue::Str(s) => Ok(s
            .chars()
            .map(|c| Cow::Owned(PyValue::Str(c.to_string())))
            .collect()),
        other => Err(type_error(format!(
            "'{}' object is not iterable",
            py_type_name(other)
        ))),
    }
}

/// `for x in (value or [])`.
pub(crate) fn iterate_or_empty(value: Option<&PyValue>) -> Read<Vec<Cow<'_, PyValue>>> {
    match or_default(value) {
        Some(v) => iterate(v),
        None => Ok(Vec::new()),
    }
}

/// A value Python passes to `str` methods or to `re`: anything else is a
/// `TypeError` there.
pub(crate) fn as_str<'a>(value: &'a PyValue, what: &str) -> Read<&'a str> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(type_error(format!(
            "{what} must be a string, not '{}'",
            py_type_name(other)
        ))),
    }
}

/// `s.lower()` and the like on a value that may not be a string.
pub(crate) fn str_method<'a>(value: &'a PyValue, method: &str) -> Read<&'a str> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(attribute_error(other, method)),
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

/// `is_engine(login)`, for a login read from an answer: `(login or '')
/// .lower() in ENGINE_LOGINS`, where anything true that is not a string has
/// no `lower`.
pub(crate) fn is_engine_value(login: Option<&PyValue>) -> Read<bool> {
    match or_default(login) {
        None => Ok(false),
        Some(value) => Ok(rules::is_engine(str_method(value, "lower")?)),
    }
}
