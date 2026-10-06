//! The live half of the engine's differential job: right after the Python
//! engine records a repository's dry runs, the Rust engine reads the same
//! pull requests live, through [`ReadOnly`]`<`[`HttpTransport`]`>` with the
//! job's own read-only token, and is held to what Python read and decided
//! (`pr_hygiene_engine::conformance::live`).
//!
//! - A report's recording: a few of its pull requests, rotating with the
//!   run, read live and compared with the snapshots Python's `evaluate` was
//!   given.
//! - A sync's recording — one author's, `sync --pr N`, or every pull
//!   request's, `sync` — the same command run whole, live, walking the
//!   write path. Its snapshots and verdicts are compared with Python's,
//!   pull request by pull request; and each pull request nothing has
//!   changed on since the engine last wrote must get no write. What it
//!   would write is answered as refused by the engine's observing layer
//!   and never reaches the transport; the read-only layer beneath refuses
//!   anything that is not one of the engine's reads of an allowed
//!   repository.
//! - Any other recording (the hourly sweep's batch) is not read live: the
//!   budget goes to the paths the service will run.
//!
//! Each recording's live reads are weighed against what is left of the
//! request budget before they are made, at the requests Python's own run
//! of the same command made, or at [`PER_PULL_REQUEST`] a report pull
//! request, and skipped when they would not fit.
//!
//! A sync of every pull request is a repository's full coverage, and has a
//! line of its own besides its row ([`Outcome::coverage`]): how many pull
//! requests Python's run decided, and how they compared live.
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
            compared: snapshots and verdicts against Python's, and the pull requests of a \
            sync held to getting no write where nothing changed since the engine last \
            wrote. Moved: pull requests that changed between the two reads. Explained: \
            differences gone once given Python's clock, status page or admission instant, \
            or GitHub's answers as gh printed them. \
            Unsettled: pull requests of a sync not held to getting no write, as Python's own \
            run wrote to them or did not run to its end. Skipped: recordings over the \
            budget, not read live.",
    layers: &[
        (Layer::LiveSnapshot, "Snapshots"),
        (Layer::LiveVerdict, "Verdicts"),
        (Layer::LiveWrites, "No write"),
    ],
    counts: &["Moved", "Explained", "Unsettled", "Skipped", "Requests"],
};

/// What a case index in the live categories points at.
pub const CASES: &str = "Cases are indices into the `evaluations.jsonl` of the \
    recording read live, a would-be write by the pull request it was for; for a run that \
    evaluated other pull requests than Python's, and for a write to a pull request \
    neither run decided, 0. A would-be write is named by its method and route, every \
    part of the route that is data written `*`, and with its state and description where \
    a status carries only the engine's own words (a comment, where it is one); only the \
    first to each pull request is named; none was sent. A status the engine posts because \
    it could not reconcile a pull request says, after it, what led to it, by class: a read \
    that failed (its HTTP status, a deadline, a connection, a body cut short, GraphQL \
    errors) and its route, a read refused, a write not sent, or the engine itself. Where \
    GitHub or the transport failed the read, the pull request's writes are not compared.";

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
    /// What each sync of every pull request found, one line each, in
    /// counts: a repository's full coverage.
    pub full: Vec<String>,
    /// Each recording not compared as a whole, GitHub or the transport
    /// having failed a read Python's run was answered: its label and why.
    pub not_compared: Vec<String>,
}

/// The line a sync of every pull request gets: how many pull requests
/// Python's run decided, and how they compared live; or that they were not
/// read live.
fn coverage(decided: usize, live: Option<&Live>) -> String {
    let Some(live) = live else {
        return format!("{decided} pull requests; not read live: over the live budget");
    };
    // Said first and alone: a pass nothing of which was compared must
    // never read as one that matched.
    if let Some(why) = &live.not_compared {
        return format!("{decided} pull requests; not compared: {why}");
    }
    let layer = |layer: Layer| {
        let (matched, compared) = live.comparison.matched(layer);
        format!("{matched}/{compared}")
    };
    format!(
        "{decided} pull requests; snapshots {}, verdicts {}, no write {}; {} moved, {} \
         explained, {} unsettled, {} not compared; {} differences",
        layer(Layer::LiveSnapshot),
        layer(Layer::LiveVerdict),
        layer(Layer::LiveWrites),
        live.moved,
        live.comparison.explained(),
        live.unsettled,
        live.comparison.not_compared(),
        live.comparison.differences(),
    )
}

impl Outcome {
    /// The full coverage lines, if any recording was a sync of every pull
    /// request.
    pub fn coverage(&self) -> Option<String> {
        (!self.full.is_empty()).then(|| self.full.join("\n"))
    }

    /// Whether every recording read live matched, or differed only where a
    /// difference is explained or a pull request moved.
    pub fn clean(&self) -> bool {
        self.rows.iter().all(Row::clean)
    }

    /// The report: the counts table, then the differences by category.
    pub fn printed(&self) -> String {
        format!(
            "## Engine differential, live reads\n\n{} recording(s): {}.\n\n{}{}\n### Live differences by category\n\n{}",
            self.rows.len(),
            if !self.clean() {
                "the Rust engine differed"
            } else if self.not_compared.is_empty() {
                "the Rust engine matched every one read live"
            } else {
                "no difference, but not every one was compared"
            },
            self.table(),
            self.not_compared_line(),
            categories(&self.rows, CASES)
        )
    }

    /// The recordings not compared as a whole, one line, or nothing: said
    /// beside every counts table, so that such a run never reads as one
    /// that matched.
    fn not_compared_line(&self) -> String {
        if self.not_compared.is_empty() {
            return String::new();
        }
        format!(
            "\nNot compared as a whole: {}.\n",
            self.not_compared.join("; ")
        )
    }

    /// The counts table alone.
    pub fn table(&self) -> String {
        counts_table(&self.rows, &TABLE)
    }

    /// The counts table for a job summary that gathers one per repository:
    /// [`TABLE`]'s columns, said once by whoever gathers them.
    pub fn summary(&self) -> String {
        let table = counts_table(
            &self.rows,
            &Table {
                intro: "Read live right after Python recorded; the record step's log has the \
                        categories.",
                ..TABLE
            },
        );
        format!("{table}{}", self.not_compared_line())
    }
}

/// What a recording's live reads are, if it has any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Work {
    /// A report: a few of its pull requests.
    Report,
    /// A one-author sync, run whole.
    Run,
    /// A sync of every pull request, run whole: the repository's full
    /// coverage.
    All,
}

fn work(recording: &Recording) -> Option<Work> {
    let command = recording.command();
    if command == "report" {
        Some(Work::Report)
    } else if command.starts_with("sync --pr ") {
        Some(Work::Run)
    } else if command == "sync" {
        Some(Work::All)
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
        live.unsettled,
        usize::from(skipped),
        live.requests,
    ]
}

/// One recording read live: its row, the requests its live reads made,
/// and, for a sync of every pull request, its full coverage line.
struct Read {
    row: Row,
    cost: usize,
    coverage: Option<String>,
    /// Why the whole recording was not compared, if it was not.
    not_compared: Option<String>,
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
    let mut full = Vec::new();
    let mut whole: Vec<(usize, String)> = Vec::new();
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
            Ok(Some(read)) => {
                spent += read.cost;
                if let Some(why) = read.not_compared {
                    whole.push((rows.len(), why));
                }
                rows.push(read.row);
                full.extend(read.coverage);
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
    let not_compared = whole
        .into_iter()
        .map(|(at, why)| format!("{} ({why})", rows[at].label))
        .collect();
    Outcome {
        rows,
        spent,
        full,
        not_compared,
    }
}

/// One recording's row, the requests its live reads made, and its full
/// coverage line where it is a sync of every pull request; `None` for a
/// recording that is not read live. `left` is what is left of the budget.
fn one(
    dir: &Path,
    directory: &str,
    own: &OwnWords,
    settings: Settings,
    left: usize,
    reads: &mut Reads<'_>,
) -> Option<Read> {
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
            return Some(Read {
                row,
                cost: 0,
                coverage: None,
                not_compared: None,
            });
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
    let decided = recording.verdicts.len();
    let full = |live: Option<&Live>| (work == Work::All).then(|| coverage(decided, live));
    // Python's own run of the same command made these requests minutes
    // ago; the live run makes the same reads.
    let estimate = match work {
        Work::Report => {
            let picks = settings.report_prs.min(recording.verdicts.len());
            PER_REPORT + PER_PULL_REQUEST * picks
        }
        Work::Run | Work::All => recording.requests().unwrap_or(settings.budget),
    };
    if estimate > left {
        let skipped = Found {
            counts: counts(&Live::default(), true),
            ..Found::default()
        };
        return Some(Read {
            row: row(Ok(skipped)),
            cost: 0,
            coverage: full(None),
            not_compared: None,
        });
    }
    let Ok(transport) = (reads.transport)() else {
        let problem = "the read-only transport could not be made".to_owned();
        return Some(Read {
            row: row(Err(problem)),
            cost: 0,
            coverage: (work == Work::All)
                .then(|| format!("{decided} pull requests; could not be read live")),
            not_compared: None,
        });
    };
    let live = match work {
        Work::Report => live_snapshots(&recording, settings.report_prs, settings.slot, transport),
        Work::Run | Work::All => live_run(
            &recording,
            transport,
            &mut *reads.clock,
            &mut *reads.status_page,
            own,
        ),
    };
    let coverage = full(Some(&live));
    let cost = live.requests;
    let not_compared = live.not_compared.clone();
    let found = Found {
        counts: counts(&live, false),
        comparison: live.comparison,
    };
    Some(Read {
        row: row(Ok(found)),
        cost,
        coverage,
        not_compared,
    })
}

#[cfg(test)]
mod tests;
