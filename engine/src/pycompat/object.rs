//! Python's object model, where the engine's answers depend on it: what a
//! type is called in an error, `==` and `<` between any two values, `str()`
//! and `repr()`, and the operations on input that raise when a value is not
//! the type the code expects.
//!
//! The Python engine reads its inputs with `x['key']`, `x.get('key')`,
//! `for item in x`, `y in x` and string methods, and each raises a
//! particular exception on a value of the wrong type. The class matters:
//! `evaluate` turns `KeyError`, `TypeError` and `ValueError` into a verdict,
//! and any other exception fails the run. Each function here raises the
//! class Python raises, with Python's message.
//!
//! Nothing here recurses with the depth of a value, so a value nested as
//! deep as the JSON reader allows cannot overflow a thread's stack. Python
//! recurses, and raises `RecursionError` where these do not: `==` and
//! `repr` near ten thousand levels, `copy.deepcopy` near five hundred.
//! Nothing GitHub answers nests a tenth as deep.

use super::error::PyErr;
use super::text::py_repr_str;
use super::value::{PyInt, PyValue};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Write as _;

/// What `.get` answers for a missing key.
pub static NONE: PyValue = PyValue::None;

/// `type(value).__name__`.
pub fn type_name(value: &PyValue) -> &'static str {
    match value {
        PyValue::None => "NoneType",
        PyValue::Bool(_) => "bool",
        PyValue::Int(_) => "int",
        PyValue::Float(_) => "float",
        PyValue::Str(_) => "str",
        PyValue::List(_) => "list",
        PyValue::Dict(_) => "dict",
    }
}

/// The `AttributeError` of looking up `name` on `value`.
pub fn no_attribute(value: &PyValue, name: &str) -> PyErr {
    PyErr::attribute(format!(
        "'{}' object has no attribute '{name}'",
        type_name(value)
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
            type_name(other)
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
            type_name(other)
        ))),
    }
}

/// `needle in haystack`, for a string needle: a substring of a string, an
/// equal item of a list, a key of a dict.
pub fn contains_str(haystack: &PyValue, needle: &str) -> Result<bool, PyErr> {
    match haystack {
        PyValue::Str(s) => Ok(s.contains(needle)),
        PyValue::List(items) => Ok(items
            .iter()
            .any(|item| matches!(item, PyValue::Str(s) if s == needle))),
        PyValue::Dict(entries) => Ok(entries.contains_key(needle)),
        other => Err(PyErr::type_error(format!(
            "argument of type '{}' is not iterable",
            type_name(other)
        ))),
    }
}

/// Python cannot hash a list or a dict, so it raises `TypeError` where one
/// is put in a set, used as a key, or looked up in either.
pub fn hashable(value: &PyValue) -> Result<(), PyErr> {
    match value {
        PyValue::List(_) | PyValue::Dict(_) => Err(PyErr::type_error(format!(
            "unhashable type: '{}'",
            type_name(value)
        ))),
        _ => Ok(()),
    }
}

/// `value in {'a', 'b'}`, a set of strings: only an equal string is in it.
pub fn in_str_set(value: &PyValue, set: &[&str]) -> Result<bool, PyErr> {
    hashable(value)?;
    Ok(matches!(value, PyValue::Str(s) if set.contains(&s.as_str())))
}

/// `value == text`.
pub fn is_str(value: &PyValue, text: &str) -> bool {
    matches!(value, PyValue::Str(s) if s == text)
}

/// A number as Python compares one: `bool` is an `int`.
enum Number<'a> {
    Int(Cow<'a, PyInt>),
    Float(f64),
}

fn number(value: &PyValue) -> Option<Number<'_>> {
    match value {
        PyValue::Bool(b) => Some(Number::Int(Cow::Owned(PyInt::from(i64::from(*b))))),
        PyValue::Int(i) => Some(Number::Int(Cow::Borrowed(i))),
        PyValue::Float(f) => Some(Number::Float(*f)),
        _ => None,
    }
}

/// The exact value of a finite float with no fraction, as an `int`.
fn integral(f: f64) -> Option<PyInt> {
    // `{:.0}` writes a float's exact decimal value, every digit of it.
    (f.is_finite() && f.fract() == 0.0)
        .then(|| PyInt::from_decimal(&format!("{f:.0}")))
        .flatten()
}

/// `int < float` and the rest, exactly, as Python compares them; `None`
/// where either is NaN.
fn compare_numbers(a: &Number<'_>, b: &Number<'_>) -> Option<Ordering> {
    match (a, b) {
        (Number::Int(x), Number::Int(y)) => Some(x.as_ref().cmp(y.as_ref())),
        (Number::Float(x), Number::Float(y)) => x.partial_cmp(y),
        (Number::Int(i), Number::Float(f)) => int_against_float(i, *f),
        (Number::Float(f), Number::Int(i)) => int_against_float(i, *f).map(Ordering::reverse),
    }
}

fn int_against_float(i: &PyInt, f: f64) -> Option<Ordering> {
    if f.is_nan() {
        return None;
    }
    if f.is_infinite() {
        return Some(if f > 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        });
    }
    let floor = integral(f.floor())?;
    Some(match i.cmp(&floor) {
        // Equal to the floor: less than the float when it has a fraction.
        Ordering::Equal if f.fract() != 0.0 => Ordering::Less,
        other => other,
    })
}

/// `a == b`. Numbers compare by value across `bool`, `int` and `float`; a
/// string equals only a string; lists and dicts compare item by item.
///
/// Inside a list or a dict Python takes an item to equal itself before it
/// asks `==`, and every NaN the JSON reader makes is the same object
/// (`json.decoder.NaN`), so `[NaN] == [NaN]` holds where `NaN == NaN` does
/// not. This follows that for values read from JSON.
pub fn py_eq(a: &PyValue, b: &PyValue) -> bool {
    equal(a, b, false)
}

/// `a == b`, as an item of a list or a dict when `nested`.
fn equal(a: &PyValue, b: &PyValue, nested: bool) -> bool {
    let mut pending = vec![(a, b, nested)];
    while let Some((a, b, nested)) = pending.pop() {
        let equal = match (a, b) {
            (PyValue::None, PyValue::None) => true,
            (PyValue::Str(x), PyValue::Str(y)) => x == y,
            (PyValue::List(x), PyValue::List(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                pending.extend(x.iter().zip(y.iter()).rev().map(|(p, q)| (p, q, true)));
                true
            }
            (PyValue::Dict(x), PyValue::Dict(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                for (key, p) in x.iter().rev() {
                    match y.get(key) {
                        Some(q) => pending.push((p, q, true)),
                        None => return false,
                    }
                }
                true
            }
            (PyValue::Float(x), PyValue::Float(y)) if nested && x.is_nan() && y.is_nan() => true,
            _ => match (number(a), number(b)) {
                (Some(x), Some(y)) => compare_numbers(&x, &y) == Some(Ordering::Equal),
                _ => false,
            },
        };
        if !equal {
            return false;
        }
    }
    true
}

/// The `TypeError` of `a < b` between two values Python cannot order.
fn unorderable(a: &PyValue, b: &PyValue) -> PyErr {
    PyErr::type_error(format!(
        "'<' not supported between instances of '{}' and '{}'",
        type_name(a),
        type_name(b)
    ))
}

/// How `a` and `b` sort, as `a < b` decides it: numbers by value, strings
/// by code point, lists item by item from the first that differs, then by
/// length. Anything else, `None` included, is Python's `TypeError`.
///
/// A NaN compares neither less nor greater than anything, so no order puts
/// it in place; that is refused as not ported rather than guessed at.
pub fn py_order(a: &PyValue, b: &PyValue) -> Result<Ordering, PyErr> {
    let (mut a, mut b) = (a, b);
    loop {
        match (a, b) {
            (PyValue::Str(x), PyValue::Str(y)) => return Ok(x.as_str().cmp(y.as_str())),
            (PyValue::List(x), PyValue::List(y)) => {
                match x.iter().zip(y.iter()).find(|(p, q)| !equal(p, q, true)) {
                    Some((p, q)) => (a, b) = (p, q),
                    None => return Ok(x.len().cmp(&y.len())),
                }
            }
            _ => {
                return match (number(a), number(b)) {
                    (Some(x), Some(y)) => compare_numbers(&x, &y).ok_or_else(|| {
                        PyErr::Unported("ordering a NaN, which has no place in a sort".into())
                    }),
                    _ => Err(unorderable(a, b)),
                };
            }
        }
    }
}

/// How two items of a tuple order, as tuple comparison asks: items `==`
/// finds equal (an item counting as equal to itself, as in a list) are
/// equal without asking `<`; any others are ordered by [`py_order`].
pub fn py_item_order(a: &PyValue, b: &PyValue) -> Result<Ordering, PyErr> {
    if equal(a, b, true) {
        Ok(Ordering::Equal)
    } else {
        py_order(a, b)
    }
}

/// `sorted(...)` where comparing two items can raise: CPython 3.12's
/// `list.sort` for a list of fewer than 64 items, which asks `<` of the same
/// pairs in the same order, so it raises where Python raises and, failing
/// that, leaves the same order. It finds the run at the start (reversing
/// one that strictly descends), then inserts each later item by binary
/// search. A longer list is sorted the same way; CPython would merge runs
/// instead, which leaves the same order but asks `<` of other pairs, so on
/// values it cannot compare the two may differ in whether they raise.
pub fn py_sort_by<T>(
    items: &mut [T],
    mut compare: impl FnMut(&T, &T) -> Result<Ordering, PyErr>,
) -> Result<(), PyErr> {
    let n = items.len();
    if n < 2 {
        return Ok(());
    }
    let mut less = |a: &T, b: &T| compare(a, b).map(|order| order == Ordering::Less);
    // `count_run`: the longest run at the start, ascending or strictly
    // descending.
    let mut run = 2;
    if less(&items[1], &items[0])? {
        while run < n && less(&items[run], &items[run - 1])? {
            run += 1;
        }
        items[..run].reverse();
    } else {
        while run < n && !less(&items[run], &items[run - 1])? {
            run += 1;
        }
    }
    // `binarysort`: each later item goes after every equal one before it.
    for start in run..n {
        let (mut low, mut high) = (0, start);
        while low < high {
            let middle = low + (high - low) / 2;
            if less(&items[start], &items[middle])? {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        items[low..=start].rotate_right(1);
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_equal_across_types_by_exact_value() {
        let int = |v: i64| PyValue::Int(PyInt::from(v));
        assert!(py_eq(&PyValue::Bool(true), &int(1)));
        assert!(py_eq(&int(1), &PyValue::Float(1.0)));
        assert!(!py_eq(&int(1), &PyValue::Float(1.5)));
        // 2**53 + 1 has no float; the nearest float is a different number.
        let big = PyValue::Int(PyInt::from(9_007_199_254_740_993));
        assert!(!py_eq(&big, &PyValue::Float(9_007_199_254_740_992.0)));
        assert!(!py_eq(&PyValue::Float(f64::NAN), &PyValue::Float(f64::NAN)));
        assert!(!py_eq(&PyValue::Str("1".into()), &int(1)));
    }

    #[test]
    fn ordering_refuses_what_python_refuses() {
        let int = |v: i64| PyValue::Int(PyInt::from(v));
        assert_eq!(
            py_order(&PyValue::None, &PyValue::None),
            Err(PyErr::type_error(
                "'<' not supported between instances of 'NoneType' and 'NoneType'"
            ))
        );
        assert_eq!(
            py_order(&PyValue::Str("a".into()), &int(1)),
            Err(PyErr::type_error(
                "'<' not supported between instances of 'str' and 'int'"
            ))
        );
        assert_eq!(py_order(&int(2), &PyValue::Float(2.5)), Ok(Ordering::Less));
        assert_eq!(
            py_order(&PyValue::Float(-2.5), &int(-3)),
            Ok(Ordering::Greater)
        );
    }
}
