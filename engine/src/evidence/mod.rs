//! What the engine reads from GitHub, and how it reads it: the port of what
//! `pr_review/github.py` reads, answer for answer.
//!
//! Three layers, each over the one below:
//!
//! - [`transport`]: the seam. A [`Call`] goes out, a [`Reply`] or a
//!   [`Failure`] comes back. [`ReplayTransport`] answers from a boundary
//!   recording; [`Scripted`] answers from a queue a test fills.
//! - [`client`]: the rules `GitHub._run` applies to every answer — one retry
//!   of an idempotent call that failed transiently, a failed GraphQL query
//!   that still carries data is an answer, empty output is `None`, anything
//!   else that is not JSON is an error — and the request, GraphQL and
//!   pagination forms built on them.
//! - [`reader`]: [`GitHub`], the reads themselves, the snapshot of one pull
//!   request that `evaluate` decides from, and the caches a run keeps.
//!
//! Beside them, the pure functions those reads use: [`records`] (the
//! controller's own record and diff markers, and the checklist block) and
//! [`builds`] (a head's build verdict). Whose comments are the engine's own,
//! and how a review bot labels a finding, are the policy's
//! ([`crate::policy::is_engine`], [`crate::policy::finding_severities`]).
//!
//! Every answer arrives as a [`PyValue`](crate::pycompat::PyValue) and is
//! read as Python reads it, with Python's evaluation order: where an answer
//! is malformed, the port fails where Python fails, with the same
//! `GitHubError` message, or with the same class of exception where Python
//! raises one the reader does not catch ([`ReadError`]).

pub mod builds;
pub mod client;
mod error;
mod py;
pub mod queries;
pub mod reader;
pub mod records;
pub mod replay;
pub mod transport;

pub use builds::Build;
pub use client::{Client, NoSleep, Sleep, ThreadSleep};
pub use error::{PyClass, ReadError};
pub use reader::{GitHub, History};
pub use records::Record;
pub use replay::{RecordingError, ReplayTransport, WriteCheck};
pub use transport::{Call, Failure, FromFn, Method, Reply, Scripted, Transport, TransportError};
