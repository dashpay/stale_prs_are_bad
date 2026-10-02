//! What a read can raise.

use crate::pycompat::PyErr;

/// A Python exception class other than `GitHubError` that the reader can
/// raise. Which of them a handler catches decides what happens next:
/// `snapshot` turns a `KeyError` or a `TypeError` into "Incomplete PR
/// snapshot", but lets an `AttributeError` through to fail the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyClass {
    KeyError,
    TypeError,
    AttributeError,
    ValueError,
    RecursionError,
    OverflowError,
}

/// Why a read gave no answer.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReadError {
    /// `GitHubError(message)`: the evidence could not be verified. The
    /// message is Python's, byte for byte.
    #[error("{0}")]
    GitHub(String),
    /// Another Python exception, raised where Python raises it. `detail`
    /// says what raised it, for whoever reads a failure; it is not Python's
    /// own text.
    #[error("{class:?}: {detail}")]
    Exception { class: PyClass, detail: String },
    /// The transport would not make a call: a read or a write a recording
    /// does not hold, or a write to a transport that only reads. Nothing
    /// catches it; it stops the run, as the recorder's own error stops
    /// Python's.
    #[error("{0}")]
    Refused(String),
    /// Something Python would have done that the port refuses rather than
    /// guess at: writing a float into JSON, which no answer the engine
    /// writes contains.
    #[error("not ported: {0}")]
    NotPorted(String),
}

impl ReadError {
    pub(crate) fn github(message: impl Into<String>) -> Self {
        ReadError::GitHub(message.into())
    }

    pub(crate) fn exception(class: PyClass, detail: impl Into<String>) -> Self {
        ReadError::Exception {
            class,
            detail: detail.into(),
        }
    }

    /// `except (KeyError, TypeError): raise GitHubError(message)`.
    pub(crate) fn key_or_type_as(self, message: &str) -> Self {
        match self {
            ReadError::Exception {
                class: PyClass::KeyError | PyClass::TypeError,
                ..
            } => ReadError::github(message),
            other => other,
        }
    }

    /// Whether this is one of the given exception classes.
    pub(crate) fn is(&self, classes: &[PyClass]) -> bool {
        matches!(self, ReadError::Exception { class, .. } if classes.contains(class))
    }
}

impl From<PyErr> for ReadError {
    fn from(error: PyErr) -> Self {
        let class = match error {
            PyErr::Value(_) => PyClass::ValueError,
            PyErr::Type(_) => PyClass::TypeError,
            PyErr::Key(_) => PyClass::KeyError,
            PyErr::Recursion(_) => PyClass::RecursionError,
            PyErr::Attribute(_) => PyClass::AttributeError,
            PyErr::Overflow(_) => PyClass::OverflowError,
            PyErr::Unported(why) => return ReadError::NotPorted(why),
        };
        ReadError::exception(class, error.to_string())
    }
}

/// `except (KeyError, TypeError) as error: raise GitHubError(message)` over a
/// whole block.
pub(crate) trait KeyOrTypeAs<T> {
    fn key_or_type_as(self, message: &str) -> Result<T, ReadError>;
}

impl<T> KeyOrTypeAs<T> for Result<T, ReadError> {
    fn key_or_type_as(self, message: &str) -> Result<T, ReadError> {
        self.map_err(|error| error.key_or_type_as(message))
    }
}
