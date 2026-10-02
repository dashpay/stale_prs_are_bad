//! The engine's clock: `main.clock()`, which the caller provides.
//!
//! Python reads the time in one function, so that a recording can fix it
//! for a whole run and log where each read was made. The port reads it at
//! the same points, each read naming its site as Python's recorder names
//! it, so a replay can compare the two logs read for read.

use crate::pycompat::{PyDateTime, PyTimeDelta};
use std::time::{SystemTime, UNIX_EPOCH};

/// Where the engine reads the clock: the Python function that asks for
/// `clock()` there, by its name in a recording's `clock_reads`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockSite {
    /// `collect`: the admission an event on one pull request is decided by.
    Collect,
    /// `run`: the instant every verdict of the run is decided at.
    Run,
    /// `admission_valid`: the re-check before a write, for a pull request
    /// not yet admitted.
    AdmissionValid,
    /// `finish`: the second verdict, made immediately before a success is
    /// published.
    Finish,
}

impl ClockSite {
    /// The name Python's recorder logs this site under.
    pub fn as_str(self) -> &'static str {
        match self {
            ClockSite::Collect => "collect",
            ClockSite::Run => "run",
            ClockSite::AdmissionValid => "admission_valid",
            ClockSite::Finish => "finish",
        }
    }
}

/// What tells the engine the time.
pub trait Clock {
    /// The current instant, with an offset, read at `site`.
    fn now(&mut self, site: ClockSite) -> PyDateTime;
}

/// The system's clock, in UTC.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&mut self, _site: ClockSite) -> PyDateTime {
        // A clock set before 1970 reads as 1970: nothing the engine
        // compares is that old.
        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| {
                i64::try_from(since.as_micros()).unwrap_or(i64::MAX)
            });
        let epoch = PyDateTime::fromisoformat("1970-01-01T00:00:00+00:00")
            .expect("a constant instant parses");
        epoch
            .py_add(PyTimeDelta::from_micros(micros))
            .expect("the system clock reads a year before 10000")
    }
}

/// `clock().isoformat(timespec='seconds').replace('+00:00', 'Z')`.
pub(crate) fn utc_text(instant: &PyDateTime) -> String {
    instant.isoformat_seconds().replace("+00:00", "Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_system_clock_reads_utc_to_the_second() {
        let text = utc_text(&SystemClock.now(ClockSite::Run));
        assert_eq!(text.len(), "2026-10-02T12:00:00Z".len(), "{text}");
        assert!(text.ends_with('Z') && text.starts_with("20"), "{text}");
    }

    #[test]
    fn an_offset_other_than_utc_keeps_its_own_spelling() {
        // `replace('+00:00', 'Z')` touches UTC alone, as Python's does.
        let instant = PyDateTime::fromisoformat("2026-09-12T10:00:00.5+02:00").unwrap();
        assert_eq!(utc_text(&instant), "2026-09-12T10:00:00+02:00");
    }
}
