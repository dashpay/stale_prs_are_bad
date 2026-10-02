//! The exceptions the engine catches, one variant per class it names.
//!
//! The engine's `except` clauses name `ValueError`, `TypeError` and
//! `KeyError`, and `evaluate` turns all three into a configuration error
//! carrying the exception's text. A port of `except (ValueError, TypeError)`
//! is a match on `PyErr::Value(_) | PyErr::Type(_)`. `json.JSONDecodeError`
//! is a `ValueError`, so it is one of [`ValueError`]'s kinds.
//!
//! [`PyErr::Recursion`] is the one class here no clause catches: Python's
//! `RecursionError` is a `RuntimeError`, and it fails the run.

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
