//! Python's `==`, `<` and friends, `in` and hashing, over [`PyValue`].
//!
//! The engine compares values whose types come from an answer it read, so
//! the comparison has to be Python's for every pair of types, not only the
//! expected one: `1 == 1.0 == True`, a string never equals a number,
//! ordering a string against a number is a `TypeError`, and a list or a
//! dict cannot be hashed, so it cannot be looked up in a set. The messages
//! are Python 3.12's.

use super::error::PyErr;
use super::value::{PyInt, PyValue};
use std::cmp::Ordering;

/// `type(value).__name__`.
pub fn py_type_name(value: &PyValue) -> &'static str {
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

/// A number as Python compares it: `bool` is an `int`.
enum Number<'a> {
    Int(std::borrow::Cow<'a, PyInt>),
    Float(f64),
}

fn number(value: &PyValue) -> Option<Number<'_>> {
    match value {
        PyValue::Bool(b) => Some(Number::Int(std::borrow::Cow::Owned(PyInt::from(
            i64::from(*b),
        )))),
        PyValue::Int(i) => Some(Number::Int(std::borrow::Cow::Borrowed(i))),
        PyValue::Float(f) => Some(Number::Float(*f)),
        _ => None,
    }
}

/// The order of two numbers, exactly, as Python has it for any mix of
/// `int` and `float`; `None` when either is a NaN.
fn number_cmp(a: &Number<'_>, b: &Number<'_>) -> Option<Ordering> {
    match (a, b) {
        (Number::Int(a), Number::Int(b)) => Some(a.as_ref().cmp(b.as_ref())),
        (Number::Float(a), Number::Float(b)) => a.partial_cmp(b),
        (Number::Int(a), Number::Float(b)) => int_float_cmp(a, *b),
        (Number::Float(a), Number::Int(b)) => int_float_cmp(b, *a).map(Ordering::reverse),
    }
}

/// `i` against `f`, without rounding either: Python compares an `int` with
/// a `float` by their exact values.
fn int_float_cmp(i: &PyInt, f: f64) -> Option<Ordering> {
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
    // 2^63: every float at or beyond it in magnitude is an integer, and
    // every float inside it truncates to an i64 exactly.
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if f.abs() >= LIMIT {
        // An integer-valued float: its exact decimal expansion is an int.
        // Rust writes a float with a precision exactly, not rounded to the
        // shortest form, so `{:.0}` is every digit of its value.
        return PyInt::from_decimal(&format!("{f:.0}")).map(|exact| i.cmp(&exact));
    }
    let truncated = f.trunc();
    // In range, as checked above.
    let whole = PyInt::from(truncated as i64);
    Some(i.cmp(&whole).then_with(|| {
        let fraction = f - truncated;
        if fraction > 0.0 {
            Ordering::Less
        } else if fraction < 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }))
}

/// `a == b`. A NaN is not equal to itself here; inside a container it is
/// (see [`py_same_element`]).
pub fn py_eq(a: &PyValue, b: &PyValue) -> bool {
    if let (Some(x), Some(y)) = (number(a), number(b)) {
        return number_cmp(&x, &y) == Some(Ordering::Equal);
    }
    match (a, b) {
        (PyValue::None, PyValue::None) => true,
        (PyValue::Str(x), PyValue::Str(y)) => x == y,
        (PyValue::List(x), PyValue::List(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| py_same_element(p, q))
        }
        (PyValue::Dict(x), PyValue::Dict(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|other| py_same_element(v, other)))
        }
        _ => false,
    }
}

/// Whether two elements of containers are the same, as Python's list,
/// tuple and dict comparisons, `in` and dict and set lookups decide it: the
/// same object first, then `==`. Every NaN a value read by `json.loads`
/// holds is one shared object, so two of them are the same element though
/// a NaN is not `==` to itself; every other pair is decided by `==`.
pub fn py_same_element(a: &PyValue, b: &PyValue) -> bool {
    matches!((a, b), (PyValue::Float(x), PyValue::Float(y)) if x.is_nan() && y.is_nan())
        || py_eq(a, b)
}

/// `a == "text"`.
pub fn py_eq_str(a: &PyValue, text: &str) -> bool {
    matches!(a, PyValue::Str(s) if s == text)
}

/// A rich comparison other than `==` and `!=`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compare {
    Lt,
    Le,
    Gt,
    Ge,
}

impl Compare {
    fn symbol(self) -> &'static str {
        match self {
            Compare::Lt => "<",
            Compare::Le => "<=",
            Compare::Gt => ">",
            Compare::Ge => ">=",
        }
    }

    fn holds(self, order: Ordering) -> bool {
        match self {
            Compare::Lt => order == Ordering::Less,
            Compare::Le => order != Ordering::Greater,
            Compare::Gt => order == Ordering::Greater,
            Compare::Ge => order != Ordering::Less,
        }
    }
}

/// `a < b`, `a <= b`, `a > b` or `a >= b`: numbers by value (a NaN
/// compares false), strings by code point, lists element by element as
/// Python compares sequences; anything else is a `TypeError`.
pub fn py_compare(a: &PyValue, op: Compare, b: &PyValue) -> Result<bool, PyErr> {
    if let (Some(x), Some(y)) = (number(a), number(b)) {
        return Ok(number_cmp(&x, &y).is_some_and(|order| op.holds(order)));
    }
    match (a, b) {
        // UTF-8 byte order is code point order.
        (PyValue::Str(x), PyValue::Str(y)) => Ok(op.holds(x.as_str().cmp(y.as_str()))),
        (PyValue::List(x), PyValue::List(y)) => py_compare_sequences(x, op, y),
        _ => Err(PyErr::type_error(format!(
            "'{}' not supported between instances of '{}' and '{}'",
            op.symbol(),
            py_type_name(a),
            py_type_name(b)
        ))),
    }
}

/// Two sequences compared as Python compares lists and tuples: the first
/// position where the elements are not the same ([`py_same_element`])
/// decides, by comparing those two; if there is none, the shorter sequence
/// is the smaller.
pub fn py_compare_sequences(a: &[PyValue], op: Compare, b: &[PyValue]) -> Result<bool, PyErr> {
    match a.iter().zip(b.iter()).find(|(x, y)| !py_same_element(x, y)) {
        Some((x, y)) => py_compare(x, op, y),
        None => Ok(op.holds(a.len().cmp(&b.len()))),
    }
}

/// `hash(value)` succeeds: a list or a dict is a `TypeError`.
pub fn py_hashable(value: &PyValue) -> Result<(), PyErr> {
    match value {
        PyValue::List(_) | PyValue::Dict(_) => Err(PyErr::type_error(format!(
            "unhashable type: '{}'",
            py_type_name(value)
        ))),
        _ => Ok(()),
    }
}

/// `value in {"a", "b", ...}`: a set of strings, which first hashes the
/// value it is asked about.
pub fn py_in_str_set(value: &PyValue, set: &[&str]) -> Result<bool, PyErr> {
    py_hashable(value)?;
    Ok(matches!(value, PyValue::Str(s) if set.contains(&s.as_str())))
}

/// `needle in container`: a substring of a string, an element of a list,
/// a key of a dict. Anything else cannot be searched, and a string can only
/// be searched for a string.
pub fn py_contains(container: &PyValue, needle: &PyValue) -> Result<bool, PyErr> {
    match container {
        PyValue::Str(haystack) => match needle {
            PyValue::Str(n) => Ok(haystack.contains(n.as_str())),
            other => Err(PyErr::type_error(format!(
                "'in <string>' requires string as left operand, not {}",
                py_type_name(other)
            ))),
        },
        PyValue::List(items) => Ok(items.iter().any(|item| py_same_element(item, needle))),
        PyValue::Dict(entries) => {
            py_hashable(needle)?;
            Ok(matches!(needle, PyValue::Str(key) if entries.contains_key(key.as_str())))
        }
        other => Err(PyErr::type_error(format!(
            "argument of type '{}' is not iterable",
            py_type_name(other)
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(text: &str) -> PyValue {
        PyValue::Int(PyInt::from_decimal(text).unwrap())
    }

    #[test]
    fn numbers_compare_by_value_across_int_float_and_bool() {
        // A JSON `1`, `1.0` and `true` are one dict key and one set member in
        // Python; a port that told them apart would read two keys as one
        // and the other way round.
        assert!(py_eq(&int("1"), &PyValue::Float(1.0)));
        assert!(py_eq(&PyValue::Bool(true), &int("1")));
        assert!(!py_eq(&PyValue::Float(f64::NAN), &PyValue::Float(f64::NAN)));
        assert!(!py_eq(&PyValue::Str("1".into()), &int("1")));
        // 2^53 + 1 is not the float nearest it.
        assert!(!py_eq(
            &int("9007199254740993"),
            &PyValue::Float(9_007_199_254_740_992.0)
        ));
        assert!(py_compare(
            &int("9007199254740993"),
            Compare::Gt,
            &PyValue::Float(9_007_199_254_740_992.0)
        )
        .unwrap());
        // Beyond i64 on both sides, still exact.
        assert!(py_eq(&int("100000000000000000000"), &PyValue::Float(1e20)));
        assert!(py_compare(
            &int("-100000000000000000001"),
            Compare::Lt,
            &PyValue::Float(-1e20)
        )
        .unwrap());
        assert!(py_compare(&PyValue::Float(0.5), Compare::Gt, &int("0")).unwrap());
        assert!(!py_compare(&PyValue::Float(f64::NAN), Compare::Ge, &int("0")).unwrap());
    }

    #[test]
    fn a_nan_read_from_json_is_the_same_element_wherever_it_appears() {
        // From Python 3.12: `json.loads` hands back one shared NaN, and
        // containers compare their elements by identity before value.
        let nan = || PyValue::Float(f64::NAN);
        let list = |items: Vec<PyValue>| PyValue::List(items.into());
        assert!(!py_eq(&nan(), &nan()), "NaN == NaN is False");
        assert!(py_eq(&list(vec![nan()]), &list(vec![nan()])));
        assert!(py_contains(&list(vec![int("1"), nan()]), &nan()).unwrap());
        assert!(py_compare_sequences(&[nan(), int("1")], Compare::Lt, &[nan(), int("2")]).unwrap());
        assert!(py_same_element(&nan(), &nan()));
        assert!(!py_same_element(&nan(), &PyValue::Float(f64::INFINITY)));
    }

    #[test]
    fn ordering_across_types_is_a_type_error() {
        let refused = py_compare(&PyValue::Str("a".into()), Compare::Ge, &int("1")).unwrap_err();
        assert_eq!(
            refused.to_string(),
            "'>=' not supported between instances of 'str' and 'int'"
        );
        assert!(py_compare(&PyValue::None, Compare::Lt, &PyValue::None).is_err());
    }

    #[test]
    fn sequences_are_decided_by_the_first_unequal_pair() {
        let a = [PyValue::Str("x".into()), int("1")];
        let b = [PyValue::Str("x".into()), PyValue::Str("y".into())];
        // The strings tie, so the second pair decides, and cannot be ordered.
        assert!(py_compare_sequences(&a, Compare::Gt, &b).is_err());
        let c = [PyValue::Str("w".into()), PyValue::Str("y".into())];
        assert!(py_compare_sequences(&a, Compare::Gt, &c).unwrap());
        assert!(!py_compare_sequences(&a, Compare::Gt, &a).unwrap());
    }

    #[test]
    fn membership_follows_the_container() {
        let list = PyValue::List(vec![PyValue::Str("/job/".into())].into());
        assert!(py_contains(&list, &PyValue::Str("/job/".into())).unwrap());
        assert!(!py_contains(&list, &PyValue::Str("/jo".into())).unwrap());
        assert!(py_contains(&int("5"), &PyValue::Str("x".into())).is_err());
        assert!(py_contains(&PyValue::Str("abc".into()), &int("1")).is_err());
        let unhashable = PyValue::List(Vec::new().into());
        assert!(py_in_str_set(&unhashable, &["a"]).is_err());
        assert!(!py_in_str_set(&PyValue::None, &["a"]).unwrap());
    }
}
