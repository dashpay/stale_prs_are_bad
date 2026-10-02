//! The PR-review engine of `pr_review/`, ported to Rust: given the same
//! reads, it reaches the same verdicts and makes the same writes, and the
//! conformance corpus in `conformance/` holds it to that.
//!
//! The crate is synchronous and does no I/O of its own. `pycompat` holds
//! the Python behaviours the engine's answers depend on: insertion-ordered
//! values, Python's JSON reader and writer, its string methods, its regular
//! expression classes, its `datetime` and the ties of `max` and `min`.

pub mod conformance;
pub mod evidence;
pub mod policy;
pub mod pycompat;
