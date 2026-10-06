//! What the engine does with a verdict: the part of `pr_review/main.py`
//! the service runs, and the writes of `pr_review/github.py` it makes.
//!
//! - [`Reconciler`]: one run's reads, writes and clock. [`Reconciler::run`]
//!   is `main.run` once the policy is loaded: it collects the pull requests
//!   asked for ([`Selection`]), decides them ([`evaluate_snapshots`]), and
//!   for a sync publishes each verdict, asks the review bots, and takes the
//!   engine's marks off pull requests it no longer governs.
//! - `writes`: the statuses, comments, labels, descriptions and reviewer
//!   requests, as methods of the evidence layer's
//!   [`GitHub`], beside its reads and caches.
//! - [`text`]: the words written for people — the description's checklist,
//!   the move comment, what a pull request still needs.
//! - [`state`]: what the engine remembers on a pull request and how it
//!   finds its own comments again.
//! - [`telemetry`]: what the review system's status page says about a head.
//!
//! The clock is the caller's ([`Clock`]), read where Python reads its
//! `clock()`, each read naming its site. What Python's command line chose
//! from its arguments and environment — the hourly batch, the per-run
//! nudge count, the status page — is the caller's too, passed in.
//!
//! Every value is a [`PyValue`] read and written as Python's engine holds
//! it. A failed write is a [`ReadError::GitHub`], the `GitHubError` both
//! reads and writes raise, and the handlers that catch it in Python catch
//! it here and nothing else.

mod clear;
mod clock;
mod collect;
mod publish;
mod recheck;
mod run;
pub mod state;
pub mod telemetry;
pub mod text;
mod values;
mod writes;

pub use clock::{Clock, ClockSite, SystemClock};
pub use collect::Collected;
pub use run::{evaluate_snapshots, selected_rows, telemetry_states, Command, Run, RunOptions};
pub use state::{
    admission_conflicts, admission_fingerprint, bot_comments, context_fingerprint, diff_record,
    same, state_record, visible,
};
pub use text::{
    area_name, asks, checklist_block, move_state, move_text, review_text, status_description,
    your_part, MOVE_STATES, POINTER,
};

use crate::evidence::{GitHub, ReadError, Transport};
use crate::pycompat::{PyInt, PyValue};

/// The label a pull request wears while a review bot's requirement is
/// waived for its head.
pub const WAIVED_LABEL: &str = "bot-review-skipped";

/// Which pull requests a run reconciles.
pub enum Selection<'s> {
    /// Every open pull request the policy governs: a full pass.
    All,
    /// One pull request, and what its author's admission depends on:
    /// `--pr N`, what an event on one pull request runs.
    Pr(PyInt),
    /// The pull requests `choose` picks from the open ones the policy
    /// governs, each with the rest of its author's as candidates: Python's
    /// rotating `--batch-size` sweep, with the choice left to the caller.
    Batch(&'s mut dyn FnMut(&[PyValue]) -> Vec<PyValue>),
}

/// One run's reads, writes, clock and log.
///
/// Everything it borrows is the run's own: a service builds it, its
/// reader and its clock inside the blocking task that runs the
/// reconciliation, and returns the [`Run`].
pub struct Reconciler<'a, T> {
    api: &'a mut GitHub<T>,
    clock: &'a mut dyn Clock,
    log: Vec<String>,
}

impl<'a, T: Transport> Reconciler<'a, T> {
    /// The engine over `api`'s reads and writes, telling the time by `clock`.
    pub fn new(api: &'a mut GitHub<T>, clock: &'a mut dyn Clock) -> Self {
        Reconciler {
            api,
            clock,
            log: Vec::new(),
        }
    }

    /// The reader and writer the run goes through, with its caches.
    pub fn api(&mut self) -> &mut GitHub<T> {
        self.api
    }

    /// What Python prints to its log along the way, oldest first.
    pub fn take_log(&mut self) -> Vec<String> {
        std::mem::take(&mut self.log)
    }

    fn say(&mut self, line: String) {
        self.log.push(line);
    }

    /// `utc_now()`: the clock read at `site`, to the second, with `Z`.
    fn utc_now(&mut self, site: ClockSite) -> String {
        clock::utc_text(&self.clock.now(site))
    }
}

/// `try: ... except GitHubError:` — the `GitHubError`'s message, caught, or
/// the value; any other error is raised on.
fn except_github<V>(result: Result<V, ReadError>) -> Result<Result<V, String>, ReadError> {
    match result {
        Ok(value) => Ok(Ok(value)),
        Err(ReadError::GitHub(message)) => Ok(Err(message)),
        Err(other) => Err(other),
    }
}
