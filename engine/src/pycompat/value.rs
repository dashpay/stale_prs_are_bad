//! [`PyValue`]: a value as the Python engine holds one after `json.loads`.
//!
//! The engine's inputs arrive as `PyValue`, not as typed structs: Python
//! works on partial input before it validates it, and an absent key is not
//! the same as a `null` one. Dicts keep insertion order ([`IndexMap`]), as
//! Python's do.
//!
//! `PyValue` deliberately has no `PartialEq`. Python's `==` crosses types
//! (`True == 1 == 1.0`) and compares containers by identity first, so a
//! structural `==` would quietly differ from it; Python's comparison is
//! written where the engine first needs one.
//!
//! A document Python reads may nest about ten thousand containers deep, and
//! anyone who can post a comment can write one. [`PyList`] and [`PyDict`]
//! therefore drop their contents without recursing, so such a value cannot
//! overflow a thread's stack when it goes away. `Clone` and `Debug` still
//! recurse: keep them to values the engine built or has validated.

use indexmap::IndexMap;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Deref, DerefMut};

/// A Python `None`, `bool`, `int`, `float`, `str`, `list` or `dict` (with
/// string keys, as JSON objects have).
#[derive(Debug, Clone)]
pub enum PyValue {
    None,
    Bool(bool),
    Int(PyInt),
    Float(f64),
    Str(String),
    List(PyList),
    Dict(PyDict),
}

/// A Python `list`: a `Vec` that drops without recursing.
#[derive(Debug, Clone, Default)]
pub struct PyList(Vec<PyValue>);

/// A Python `dict` with string keys, in insertion order: an `IndexMap` that
/// drops without recursing.
#[derive(Debug, Clone, Default)]
pub struct PyDict(IndexMap<String, PyValue>);

impl PyList {
    pub fn new() -> Self {
        PyList(Vec::new())
    }

    pub fn into_vec(mut self) -> Vec<PyValue> {
        std::mem::take(&mut self.0)
    }
}

impl PyDict {
    pub fn new() -> Self {
        PyDict(IndexMap::new())
    }

    pub fn into_map(mut self) -> IndexMap<String, PyValue> {
        std::mem::take(&mut self.0)
    }
}

/// Drops `pending` and everything nested in it, one level at a time: each
/// container's contents are moved onto the work list before the emptied
/// container itself is dropped.
fn drop_flat(mut pending: Vec<PyValue>) {
    while let Some(value) = pending.pop() {
        match value {
            PyValue::List(mut list) => pending.append(&mut list.0),
            PyValue::Dict(mut dict) => pending.extend(std::mem::take(&mut dict.0).into_values()),
            _ => {}
        }
    }
}

impl Drop for PyList {
    fn drop(&mut self) {
        if self.0.iter().any(is_container) {
            drop_flat(std::mem::take(&mut self.0));
        }
    }
}

impl Drop for PyDict {
    fn drop(&mut self) {
        if self.0.values().any(is_container) {
            drop_flat(std::mem::take(&mut self.0).into_values().collect());
        }
    }
}

fn is_container(value: &PyValue) -> bool {
    matches!(value, PyValue::List(_) | PyValue::Dict(_))
}

impl Deref for PyList {
    type Target = Vec<PyValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PyList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for PyDict {
    type Target = IndexMap<String, PyValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PyDict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<PyValue>> for PyList {
    fn from(items: Vec<PyValue>) -> Self {
        PyList(items)
    }
}

impl From<IndexMap<String, PyValue>> for PyDict {
    fn from(entries: IndexMap<String, PyValue>) -> Self {
        PyDict(entries)
    }
}

impl FromIterator<PyValue> for PyList {
    fn from_iter<I: IntoIterator<Item = PyValue>>(items: I) -> Self {
        PyList(items.into_iter().collect())
    }
}

impl FromIterator<(String, PyValue)> for PyDict {
    fn from_iter<I: IntoIterator<Item = (String, PyValue)>>(entries: I) -> Self {
        PyDict(entries.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a PyList {
    type Item = &'a PyValue;
    type IntoIter = std::slice::Iter<'a, PyValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a PyDict {
    type Item = (&'a String, &'a PyValue);
    type IntoIter = indexmap::map::Iter<'a, String, PyValue>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl PyValue {
    /// `bool(value)`: `None`, `False`, zero, `''`, `[]` and `{}` are false.
    /// A NaN is true, `-0.0` false.
    pub fn truthy(&self) -> bool {
        match self {
            PyValue::None => false,
            PyValue::Bool(b) => *b,
            PyValue::Int(i) => !i.is_zero(),
            PyValue::Float(f) => *f != 0.0,
            PyValue::Str(s) => !s.is_empty(),
            PyValue::List(items) => !items.is_empty(),
            PyValue::Dict(map) => !map.is_empty(),
        }
    }
}

/// A Python `int`, exact at any size.
///
/// Values that fit in an `i64` are held as one. Larger ones are held as
/// their canonical decimal digits: the engine reads, compares and writes
/// such integers (an id, a count from the API) but never does arithmetic on
/// them, so exact equality, ordering and printing are all it needs, and
/// they come without a big-number dependency. Every value has exactly one
/// representation, so the derived `Eq` and `Hash` are Python's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyInt(Repr);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Repr {
    Small(i64),
    /// Outside the `i64` range; `digits` has no leading zero.
    Big {
        negative: bool,
        digits: Box<str>,
    },
}

impl PyInt {
    /// `int(text)` for `-?[0-9]+`, the only integers JSON can write.
    /// Leading zeros are allowed; `-0` is zero. `None` for anything else.
    pub fn from_decimal(text: &str) -> Option<PyInt> {
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        if let Ok(small) = text.parse::<i64>() {
            return Some(PyInt(Repr::Small(small)));
        }
        let digits = digits.trim_start_matches('0');
        // Zero and everything else within i64 parsed above; what is left
        // has at least nineteen significant digits.
        Some(PyInt(Repr::Big {
            negative,
            digits: digits.into(),
        }))
    }

    /// The value, if it fits in an `i64`.
    pub fn as_i64(&self) -> Option<i64> {
        match self.0 {
            Repr::Small(v) => Some(v),
            Repr::Big { .. } => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self.0, Repr::Small(0))
    }
}

impl From<i64> for PyInt {
    fn from(value: i64) -> Self {
        PyInt(Repr::Small(value))
    }
}

impl Ord for PyInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Repr::Small(a), Repr::Small(b)) => a.cmp(b),
            // A big value lies beyond every i64, on the side of its sign.
            (Repr::Small(_), Repr::Big { negative, .. }) => {
                if *negative {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Repr::Big { negative, .. }, Repr::Small(_)) => {
                if *negative {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (
                Repr::Big {
                    negative: an,
                    digits: ad,
                },
                Repr::Big {
                    negative: bn,
                    digits: bd,
                },
            ) => match (an, bn) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => {
                    let magnitude = ad.len().cmp(&bd.len()).then_with(|| ad.cmp(bd));
                    if *an {
                        magnitude.reverse()
                    } else {
                        magnitude
                    }
                }
            },
        }
    }
}

impl PartialOrd for PyInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `str(int)`.
impl fmt::Display for PyInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Small(v) => write!(f, "{v}"),
            Repr::Big { negative, digits } => {
                write!(f, "{}{digits}", if *negative { "-" } else { "" })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(text: &str) -> PyInt {
        PyInt::from_decimal(text).unwrap()
    }

    #[test]
    fn i64_boundaries_choose_the_representation_python_cannot_see() {
        // One step past i64 either way must still compare and print exactly.
        assert_eq!(int("9223372036854775807").as_i64(), Some(i64::MAX));
        assert_eq!(int("9223372036854775808").as_i64(), None);
        assert_eq!(int("-9223372036854775808").as_i64(), Some(i64::MIN));
        assert_eq!(int("-9223372036854775809").as_i64(), None);
        assert!(int("-9223372036854775809") < int("-9223372036854775808"));
        assert!(int("9223372036854775807") < int("9223372036854775808"));
        assert_eq!(
            int("-9223372036854775809").to_string(),
            "-9223372036854775809"
        );
    }

    #[test]
    fn one_value_one_representation() {
        // Leading zeros and negative zero must not make a second encoding
        // of the same number, or derived equality would stop being Python's.
        assert_eq!(int("-0"), PyInt::from(0));
        assert_eq!(int("007"), PyInt::from(7));
        assert_eq!(int("00009223372036854775808"), int("9223372036854775808"));
        assert_eq!(
            int("00009223372036854775808").to_string(),
            "9223372036854775808"
        );
        assert_eq!(PyInt::from_decimal(""), None);
        assert_eq!(PyInt::from_decimal("-"), None);
        assert_eq!(PyInt::from_decimal("+1"), None);
        assert_eq!(PyInt::from_decimal("1.0"), None);
        assert_eq!(PyInt::from_decimal("\u{0661}"), None);
    }
}
