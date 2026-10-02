//! What CPython 3.12 does, where the engine's answers depend on it.
//!
//! Each module reproduces the C implementation the Python engine actually
//! runs, not the language's general description of it, and is tested against
//! answers Python itself gave (`conformance/pycompat/`, written by
//! `conformance/pycompat/generate.py`):
//!
//! - [`value`]: [`PyValue`], what `json.loads` returns, with insertion-ordered
//!   dicts and integers of any size;
//! - [`error`]: [`PyErr`], the exceptions the engine catches;
//! - [`json`]: `json.loads`, `JSONDecoder().raw_decode` and `json.dumps`;
//! - [`text`]: `str` methods that are Unicode-aware in Python;
//! - [`re`]: `re`'s character classes as Rust `regex` classes;
//! - [`datetime`]: `datetime.fromisoformat`, `isoformat` and comparisons;
//! - [`ties`]: `max` and `min` with a key, which keep the first of equals;
//! - [`ops`]: `==`, `<`, `in` and hashing across every pair of types;
//! - [`hashlib`]: the digests the engine prints, `hashlib.sha256`;
//! - [`urllib`]: how a login or a commit goes into a route.
//!
//! The character tables come from enumerating every code point under Python
//! 3.12 (Unicode 15.0), not from Rust's standard library or the regex
//! crate, whose Unicode data is newer and in places defined differently.

pub mod datetime;
pub mod error;
pub mod hashlib;
pub mod json;
pub mod ops;
pub mod re;
#[rustfmt::skip]
pub mod tables;
pub mod text;
pub mod ties;
pub mod urllib;
pub mod value;

pub use datetime::{PyDateTime, PyTimeDelta};
pub use error::{JsonDecodeError, PyErr, ValueError};
pub use json::{py_dumps, py_loads, py_raw_decode, FloatNotWritten, RawDecoded};
pub use ties::{py_max_by, py_max_by_key, py_min_by, py_min_by_key};
pub use value::{PyDict, PyInt, PyList, PyValue};
