//! The exceptions the engine can raise, one variant per Python class.
//!
//! The engine's `except` clauses name `ValueError`, `TypeError` and
//! `KeyError`, and `evaluate` turns all three into a configuration error
//! carrying the exception's text. A port of `except (ValueError, TypeError)`
//! is a match on `PyErr::Value(_) | PyErr::Type(_)`. `json.JSONDecodeError`
//! is a `ValueError`, so it is one of [`ValueError`]'s kinds.
//!
//! The other classes are ones no clause catches, so each fails the run:
//! `RecursionError`, `AttributeError` (a method looked up on a value of the
//! wrong type) and `OverflowError`. [`PyErr::Unported`] is not Python: it
//! is the port refusing something Python would have done, and it is never
//! caught either.

use super::text::py_repr_str;

/// A Python exception. `Display` gives what `str(exception)` gives.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PyErr {
    /// `ValueError`, `json.JSONDecodeError` among them.
    #[error("{0}")]
    Value(ValueError),
    /// `TypeError`, with its message.
    #[error("{0}")]
    Type(String),
    /// `KeyError` for a missing string key. Like Python's, its text is the
    /// key's `repr`: `'name'`.
    #[error("{}", py_repr_str(.0))]
    Key(String),
    /// `RecursionError`: JSON nested deeper than Python's C stack allows.
    #[error("{0}")]
    Recursion(String),
    /// `AttributeError`, with its message: `None.lower()` is
    /// `'NoneType' object has no attribute 'lower'`.
    #[error("{0}")]
    Attribute(String),
    /// `OverflowError`: a date moved outside years 1 to 9999.
    #[error("{0}")]
    Overflow(String),
    /// Not Python. Something Python would have done that the port refuses
    /// rather than guess at, such as hashing a float it has no writer for.
    /// Nothing catches it, so the run fails where Python's would have gone
    /// on; no input the engine's own reader produces reaches it.
    #[error("not ported: {0}")]
    Unported(String),
}

impl PyErr {
    /// A plain `ValueError(message)`.
    pub fn value(message: impl Into<String>) -> Self {
        PyErr::Value(ValueError::Message(message.into()))
    }

    /// A `TypeError(message)`.
    pub fn type_error(message: impl Into<String>) -> Self {
        PyErr::Type(message.into())
    }

    /// An `AttributeError(message)`.
    pub fn attribute(message: impl Into<String>) -> Self {
        PyErr::Attribute(message.into())
    }

    /// Whether `except (ValueError, TypeError, KeyError)` catches it: the
    /// clause `evaluate` and its helpers use.
    pub fn is_value_type_or_key(&self) -> bool {
        matches!(self, PyErr::Value(_) | PyErr::Type(_) | PyErr::Key(_))
    }
}

impl From<super::json::FloatNotWritten> for PyErr {
    fn from(error: super::json::FloatNotWritten) -> Self {
        PyErr::Unported(error.to_string())
    }
}

/// The kinds of `ValueError` the engine can meet.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValueError {
    /// `ValueError(message)`.
    #[error("{0}")]
    Message(String),
    /// `json.JSONDecodeError`.
    #[error("{0}")]
    JsonDecode(JsonDecodeError),
    /// Not Python. A JSON string escape names a lone surrogate (`"\ud800"`):
    /// Python reads it into a `str`, which a Rust `String` cannot hold. It is
    /// a `ValueError` so that a read of it takes the path invalid JSON takes
    /// in Python; `pos` is the escape's offset in code points.
    #[error("lone surrogate \\u escape at char {pos}, which a Rust string cannot hold")]
    LoneSurrogate { pos: usize },
}

/// `json.JSONDecodeError`: what was expected, and where, in code points.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{msg}: line {lineno} column {colno} (char {pos})")]
pub struct JsonDecodeError {
    pub msg: &'static str,
    pub pos: usize,
    pub lineno: usize,
    pub colno: usize,
}

impl From<ValueError> for PyErr {
    fn from(error: ValueError) -> Self {
        PyErr::Value(error)
    }
}
