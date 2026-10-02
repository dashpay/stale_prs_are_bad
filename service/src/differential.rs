//! The live half of the engine's differential job: right after the Python
//! engine records a repository's dry runs, the Rust engine reads the same
//! pull requests live, through [`ReadOnly`]`<`[`HttpTransport`]`>` with the
//! job's own read-only token, and is held to what Python read and decided
//! (`pr_hygiene_engine::conformance::live`).
//!
//! - A report's recording: a few of its pull requests, rotating with the
//!   run, read live and compared with the snapshots Python's `evaluate` was
//!   given.
//! - A one-author `sync --pr N` recording: the same command run whole,
//!   live, walking the write path. Its snapshots and verdicts are compared
//!   with Python's; and where nothing has changed since the engine last
//!   wrote, it must want to write nothing. What it would write is answered
//!   as refused by the engine's observing layer and never reaches the
//!   transport; the read-only layer beneath refuses anything that is not
//!   one of the engine's reads of an allowed repository.
//! - Any other recording (the hourly sweep) is not read live: the budget
//!   goes to the paths the service will run.
//!
//! Each recording's live reads are weighed against what is left of the
//! request budget before they are made, at the requests Python's own run
//! of the same command made, or at [`PER_PULL_REQUEST`] a report pull
//! request, and skipped when they would not fit.
//!
//! What it says is the conformance report
//! (`pr_hygiene_engine::conformance::report`), under the same rule as the
//! replay tool's: counts, layers, field paths and kinds, and nothing a
//! recording or a live answer holds. Everything read live is kept in memory
//! and dropped when the recording's comparison ends.

use crate::reader::{HttpTransport, ReadOnly};
use pr_hygiene_engine::conformance::live::{live_run, live_snapshots, Live};
use pr_hygiene_engine::conformance::report::{
    categories, counts_table, guarded, label_rows, plain, Found, Row, Table,
};
use pr_hygiene_engine::conformance::{Layer, OwnWords, Recording};
use pr_hygiene_engine::pycompat::PyValue;
use pr_hygiene_engine::reconcile::Clock;
use std::path::{Path, PathBuf};

/// What a report's pull request costs to read live: the pull request, its
/// files, reviews, threads, statuses and checks, with a page to spare.
pub const PER_PULL_REQUEST: usize = 8;

/// What a report's live reads cost once, whatever their number: the
/// batched history query and the collaborator listing.
pub const PER_REPORT: usize = 2;

/// The live counts table.
pub const TABLE: Table = Table {
    intro: "Read live right after Python recorded. Each layer is cases matched of cases \
            compared: snapshots and verdicts against Python's, and one-author runs held to \
            writing nothing where nothing changed since the engine last wrote. Moved: pull \
            requests that changed between the two reads. Explained: differences gone once \
            given Python's clock, status page or admission instant. Unsettled: one-author \
            runs not held to writing nothing, as a record was not current or Python's own \
            run wrote. Skipped: recordings over the budget, not read live.",
    layers: &[
        (Layer::LiveSnapshot, "Snapshots"),
        (Layer::LiveVerdict, "Verdicts"),
        (Layer::LiveWrites, "No write"),
    ],
    counts: &["Moved", "Explained", "Unsettled", "Skipped", "Requests"],
};

/// What a case index in the live categories points at.
pub const CASES: &str = "Cases are indices into the `evaluations.jsonl` of the \
    recording read live; for a one-author run's writes, and for a run that evaluated \
    other pull requests than Python's, 0. A would-be write is named by its method and \
    route, every part of the route that is data written `*`; none was sent.";

/// How many live reads a run may make, and which.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// Requests the live reads may make in all.
    pub budget: usize,
    /// Pull requests read live from each report.
    pub report_prs: usize,
    /// The run's slot, which chooses them, so that each comes round.
    pub slot: usize,
}

/// How the live reads reach GitHub: what the binary sets up, and what a
/// test sets up in its place.
pub struct Reads<'a> {
    /// A read-only transport, one for each recording's live reads.
    pub transport: &'a mut dyn FnMut() -> anyhow::Result<ReadOnly<HttpTransport>>,
    pub clock: &'a mut dyn Clock,
    /// The review system's status page, read at most once a run.
    pub status_page: &'a mut dyn FnMut() -> PyValue,
}

/// Everything the live reads found.
pub struct Outcome {
    pub rows: Vec<Row>,
    /// The requests the live reads made, a page of a listing each.
    pub spent: usize,
}

impl Outcome {
    /// Whether every recording read live matched, or differed only where a
    /// difference is explained or a pull request moved.
    pub fn clean(&self) -> bool {
        self.rows.iter().all(Row::clean)
    }

    /// The report: the counts table, then the differences by category.
    pub fn printed(&self) -> String {
        format!(
            "## Engine differential, live reads\n\n{} recording(s): {}.\n\n{}\n### Live differences by category\n\n{}",
            self.rows.len(),
            if self.clean() {
                "the Rust engine matched every one read live"
            } else {
                "the Rust engine differed"
            },
            self.table(),
            categories(&self.rows, CASES)
        )
    }

    /// The counts table alone.
    pub fn table(&self) -> String {
        counts_table(&self.rows, &TABLE)
    }

    /// The counts table for a job summary that gathers one per repository:
    /// [`TABLE`]'s columns, said once by whoever gathers them.
    pub fn summary(&self) -> String {
        counts_table(
            &self.rows,
            &Table {
                intro: "Read live right after Python recorded; the record step's log has the \
                        categories.",
                ..TABLE
            },
        )
    }
}

/// What a recording's live reads are, if it has any.
enum Work {
    /// A report: a few of its pull requests.
    Report,
    /// A one-author sync, run whole.
    Run,
}

fn work(recording: &Recording) -> Option<Work> {
    let command = recording.command();
    if command == "report" {
        Some(Work::Report)
    } else if command.starts_with("sync --pr ") {
        Some(Work::Run)
    } else {
        None
    }
}

/// Every recording at or under each path, in a stable order.
pub fn recordings(paths: &[PathBuf]) -> Vec<PathBuf> {
    fn under(dir: &Path, found: &mut Vec<PathBuf>) {
        if dir.join("recording.json").is_file() {
            found.push(dir.to_owned());
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        let mut children: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        children.sort();
        for child in children.iter().filter(|child| child.is_dir()) {
            under(child, found);
        }
    }
    let mut found = Vec::new();
    for path in paths {
        under(path, &mut found);
    }
    found
}

/// A row's counts, in the order of [`TABLE`]'s.
fn counts(live: &Live, skipped: bool) -> Vec<usize> {
    vec![
        live.moved,
        live.comparison.explained(),
        usize::from(live.settled == Some(false)),
        usize::from(skipped),
        live.requests,
    ]
}

/// Read live what each recording under `dirs` read, while the budget
/// lasts, and compare. Blocks on every request: run it on a blocking
/// thread, as the engine always runs.
///
/// Everything done with one recording — reading it, naming it, weighing
/// it, reading it live — runs under [`guarded`]: a panic anywhere in it is
/// one unreadable row, its message never printed, and is charged whatever
/// was left of the budget, since what it spent is not known.
pub fn compare(dirs: &[PathBuf], own: &OwnWords, settings: Settings, reads: Reads<'_>) -> Outcome {
    let Reads {
        transport,
        clock,
        status_page,
    } = reads;
    let mut rows = Vec::new();
    let mut spent = 0;
    for dir in dirs {
        let directory = plain(&dir.file_name().unwrap_or_default().to_string_lossy());
        let left = settings.budget.saturating_sub(spent);
        let read = guarded(|| {
            let mut reads = Reads {
                transport: &mut *transport,
                clock: &mut *clock,
                status_page: &mut *status_page,
            };
            one(dir, &directory, own, settings, left, &mut reads)
        });
        match read {
            Ok(None) => {}
            Ok(Some((row, cost))) => {
                spent += cost;
                rows.push(row);
            }
            Err(problem) => {
                spent += left;
                rows.push(Row {
                    repository: "?".into(),
                    label: format!("unreadable ({directory})"),
                    directory,
                    found: Err(problem),
                });
            }
        }
    }
    label_rows(&mut rows);
    Outcome { rows, spent }
}

/// One recording's row, and the requests its live reads made; `None` for a
/// recording that is not read live. `left` is what is left of the budget.
fn one(
    dir: &Path,
    directory: &str,
    own: &OwnWords,
    settings: Settings,
    left: usize,
    reads: &mut Reads<'_>,
) -> Option<(Row, usize)> {
    let loaded = Recording::load_with(|name| std::fs::read_to_string(dir.join(name)))
        .map_err(|e| e.to_string());
    let recording = match loaded {
        Ok(recording) => recording,
        Err(problem) => {
            let row = Row {
                repository: "?".into(),
                label: format!("unreadable ({directory})"),
                directory: directory.to_owned(),
                found: Err(problem),
            };
            return Some((row, 0));
        }
    };
    let work = work(&recording)?;
    let repository = plain(recording.repository().unwrap_or("?"));
    let label = format!("{repository} · {}", plain(&recording.command()));
    let row = |found| Row {
        repository: repository.clone(),
        label: label.clone(),
        directory: directory.to_owned(),
        found,
    };
    // Python's own run of the same command made these requests minutes
    // ago; the live run makes the same reads.
    let estimate = match work {
        Work::Report => {
            let picks = settings.report_prs.min(recording.verdicts.len());
            PER_REPORT + PER_PULL_REQUEST * picks
        }
        Work::Run => recording.requests().unwrap_or(settings.budget),
    };
    if estimate > left {
        let skipped = Found {
            counts: counts(&Live::default(), true),
            ..Found::default()
        };
        return Some((row(Ok(skipped)), 0));
    }
    let Ok(transport) = (reads.transport)() else {
        let problem = "the read-only transport could not be made".to_owned();
        return Some((row(Err(problem)), 0));
    };
    let live = match work {
        Work::Report => live_snapshots(&recording, settings.report_prs, settings.slot, transport),
        Work::Run => live_run(
            &recording,
            transport,
            &mut *reads.clock,
            &mut *reads.status_page,
            own,
        ),
    };
    let found = Found {
        counts: counts(&live, false),
        comparison: live.comparison,
    };
    Some((row(Ok(found)), live.requests))
}

#[cfg(test)]
mod tests;
