//! Python's object model where the engine reads its input: subscripts,
//! `.get`, attribute lookups, iteration, `x or y`, and `str()` and `repr()`.
//! Comparisons, `in` and hashing are [`super::ops`].
//!
//! The Python engine reads its inputs with `x['key']`, `x.get('key')`,
//! `for item in x` and string methods, and each raises a particular
//! exception on a value of the wrong type. The class matters: `evaluate`
//! turns `KeyError`, `TypeError` and `ValueError` into a verdict, the
//! evidence reader turns some of them into a `GitHubError`, and any other
//! exception fails the run. Each function here raises the class Python
//! raises, with Python's message.
//!
//! Nothing here recurses with the depth of a value, so a value nested as
//! deep as the JSON reader allows cannot overflow a thread's stack. Python
//! recurses, and raises `RecursionError` where these do not: `repr` near ten
//! thousand levels, `copy.deepcopy` near five hundred. Nothing GitHub
//! answers nests a tenth as deep.

use super::error::PyErr;
use super::ops::py_type_name;
use super::text::py_repr_str;
use super::value::{PyDict, PyList, PyValue};
use std::borrow::Cow;
use std::fmt::Write as _;
use std::sync::LazyLock;

/// What `.get` answers for a missing key.
pub static NONE: PyValue = PyValue::None;

/// `{}`, for `x or {}` and `.get(key, {})`.
pub static EMPTY_DICT: LazyLock<PyValue> = LazyLock::new(|| PyValue::Dict(PyDict::new()));

/// `[]`, for `x or []` and `.get(key, [])`.
pub static EMPTY_LIST: LazyLock<PyValue> = LazyLock::new(|| PyValue::List(PyList::new()));

/// `''`, for `x or ''`.
pub static EMPTY_STR: PyValue = PyValue::Str(String::new());

/// The `AttributeError` of looking up `name` on `value`.
pub fn no_attribute(value: &PyValue, name: &str) -> PyErr {
    PyErr::attribute(format!(
        "'{}' object has no attribute '{name}'",
        py_type_name(value)
    ))
}

/// `value[key]`, with a string key.
pub fn getitem<'a>(value: &'a PyValue, key: &str) -> Result<&'a PyValue, PyErr> {
    match value {
        PyValue::Dict(entries) => entries.get(key).ok_or_else(|| PyErr::Key(key.to_owned())),
        PyValue::List(_) => Err(PyErr::type_error(
            "list indices must be integers or slices, not str",
        )),
        PyValue::Str(_) => Err(PyErr::type_error(
            "string indices must be integers, not 'str'",
        )),
        other => Err(PyErr::type_error(format!(
            "'{}' object is not subscriptable",
            py_type_name(other)
        ))),
    }
}

/// `value.get(key)`: [`NONE`] for a missing key. Only a dict has `get`.
pub fn get<'a>(value: &'a PyValue, key: &str) -> Result<&'a PyValue, PyErr> {
    get_or(value, key, &NONE)
}

/// `value.get(key, default)`.
pub fn get_or<'a>(
    value: &'a PyValue,
    key: &str,
    default: &'a PyValue,
) -> Result<&'a PyValue, PyErr> {
    match value {
        PyValue::Dict(entries) => Ok(entries.get(key).unwrap_or(default)),
        other => Err(no_attribute(other, "get")),
    }
}

/// `a or b`: `a` when it is true, else `b`.
pub fn or<'a>(a: &'a PyValue, b: &'a PyValue) -> &'a PyValue {
    if a.truthy() {
        a
    } else {
        b
    }
}

/// The text of `value`, to call the `str` method `method` on it. Python
/// raises `AttributeError` for anything but a `str`.
pub fn str_method<'a>(value: &'a PyValue, method: &str) -> Result<&'a str, PyErr> {
    match value {
        PyValue::Str(s) => Ok(s),
        other => Err(no_attribute(other, method)),
    }
}

/// `for item in value`: a list's items, a dict's keys, a string's
/// characters.
pub fn iterate(value: &PyValue) -> Result<Vec<Cow<'_, PyValue>>, PyErr> {
    match value {
        PyValue::List(items) => Ok(items.iter().map(Cow::Borrowed).collect()),
        PyValue::Dict(entries) => Ok(entries
            .keys()
            .map(|k| Cow::Owned(PyValue::Str(k.clone())))
            .collect()),
        PyValue::Str(s) => Ok(s
            .chars()
            .map(|c| Cow::Owned(PyValue::Str(c.to_string())))
            .collect()),
        other => Err(PyErr::type_error(format!(
            "'{}' object is not iterable",
            py_type_name(other)
        ))),
    }
}

/// `repr(x)` for a float: the shortest digits that read back as the same
/// float, written positionally from `1e-4` up to below `1e16` and in
/// scientific notation outside that, with `.0` on a whole number.
pub fn py_float_repr(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.into();
    }
    // `{:e}` writes the same shortest round-tripping digits Python finds,
    // as `d.ddde±x`; only the layout differs.
    let scientific = format!("{x:e}");
    let (mantissa, exponent) = scientific
        .split_once('e')
        .unwrap_or((scientific.as_str(), "0"));
    // `{:e}` of a finite float always has an `e` and a whole exponent, so
    // neither fallback is taken.
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let (sign, mantissa) = match mantissa.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", mantissa),
    };
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    // Where the decimal point falls after the first `point` digits.
    let point = exponent + 1;
    let mut out = String::from(sign);
    if !(-4 < point && point <= 16) {
        // A finite float's `{:e}` has at least one digit.
        let (first, rest) = digits.split_at(1);
        out.push_str(first);
        if !rest.is_empty() {
            out.push('.');
            out.push_str(rest);
        }
        let _ = write!(
            out,
            "e{}{:02}",
            if exponent < 0 { '-' } else { '+' },
            exponent.unsigned_abs()
        );
    } else if point <= 0 {
        out.push_str("0.");
        out.extend(std::iter::repeat_n('0', point.unsigned_abs() as usize));
        out.push_str(&digits);
    } else {
        let point = point as usize;
        if digits.len() <= point {
            out.push_str(&digits);
            out.extend(std::iter::repeat_n('0', point - digits.len()));
            out.push_str(".0");
        } else {
            out.push_str(&digits[..point]);
            out.push('.');
            out.push_str(&digits[point..]);
        }
    }
    out
}

/// What is left to write of a `repr`; the last pushed is written next.
enum Repr<'v> {
    Value(&'v PyValue),
    Text(&'static str),
    Key(&'v str),
}

/// `repr(value)`: `None`, `True`, `1`, `1.5`, `'text'`, `[1, 'a']`,
/// `{'k': None}`.
pub fn py_repr(value: &PyValue) -> String {
    let mut out = String::new();
    let mut work = vec![Repr::Value(value)];
    while let Some(task) = work.pop() {
        match task {
            Repr::Text(text) => out.push_str(text),
            Repr::Key(key) => {
                out.push_str(&py_repr_str(key));
                out.push_str(": ");
            }
            Repr::Value(value) => match value {
                PyValue::None => out.push_str("None"),
                PyValue::Bool(true) => out.push_str("True"),
                PyValue::Bool(false) => out.push_str("False"),
                PyValue::Int(i) => {
                    let _ = write!(out, "{i}");
                }
                PyValue::Float(f) => out.push_str(&py_float_repr(*f)),
                PyValue::Str(s) => out.push_str(&py_repr_str(s)),
                PyValue::List(items) => {
                    out.push('[');
                    work.push(Repr::Text("]"));
                    for (n, item) in items.iter().enumerate().rev() {
                        work.push(Repr::Value(item));
                        if n > 0 {
                            work.push(Repr::Text(", "));
                        }
                    }
                }
                PyValue::Dict(entries) => {
                    out.push('{');
                    work.push(Repr::Text("}"));
                    for (n, (key, item)) in entries.iter().enumerate().rev() {
                        work.push(Repr::Value(item));
                        work.push(Repr::Key(key));
                        if n > 0 {
                            work.push(Repr::Text(", "));
                        }
                    }
                }
            },
        }
    }
    out
}

/// `str(value)`: a string's own text, anything else its `repr`.
pub fn py_str(value: &PyValue) -> Cow<'_, str> {
    match value {
        PyValue::Str(s) => Cow::Borrowed(s),
        other => Cow::Owned(py_repr(other)),
    }
}
