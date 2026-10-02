//! Holding the port to what the Python engine did on a boundary recording,
//! for the conformance tests and for the differential job that runs the
//! same comparison on live recordings.
//!
//! - [`Recording`]: the files `pr_review/conformance.py` writes for one
//!   recorded run, read through a function the caller gives, since the
//!   crate does no I/O of its own. Formats 1 and 2 both read.
//! - [`compare`]: every snapshot Python's `evaluate` saw, rebuilt from the
//!   recorded reads alone ([`rebuild_snapshots`]); every evaluation run
//!   again through the port; every verdict row as far as `evaluate`
//!   decides it.
//! - [`differences`]: where two values differ, by field path and kind,
//!   never by value.
//! - [`OwnWords`]: which reasons are the engine's own words and which the
//!   text of a Python exception, which another engine is not held to.
//! - [`replay_run`]: the whole recorded run replayed through the port's
//!   reconcile layer — the same verdicts, the same writes in the same order,
//!   the same outputs, report and clock reads.
//!
//! Nothing here formats what a recording holds. A [`Check`] that failed
//! keeps the error itself for a test to read, and says so.

mod compare;
mod diff;
mod exception;
mod recording;
mod run;

pub use compare::{
    collected, compare, rebuild_snapshots, Check, Comparison, Failure, Layer, Outcome, SHARED_HEAD,
};
pub use diff::{differences, Difference, Kind};
pub use exception::{set_aside_exception_text, ExceptionText, OwnWords, SourceError};
pub use recording::{LoadError, Recording, FILES, READABLE_FORMATS};
pub use run::{replay_run, RunFiles, RUN_FILES};
