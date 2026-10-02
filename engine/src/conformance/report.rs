//! What the differential tools print: a counts table per recording and per
//! repository, then the differences grouped by layer, field path and kind,
//! with the cases that show each. One format, and one rule for it: nothing
//! a recording holds is printed.
//!
//! What a row can say is fixed by where it comes from. A recording is named
//! by its repository and command through [`plain`], which keeps only the
//! characters those are spelled with. A difference is a field path
//! ([`differences`](super::differences) never spells a key that is data)
//! and a kind; a failure is named by its kind, never by its detail, which
//! can quote a recording. A row that could not be compared carries a
//! problem that names a file and a position, or only that the tool
//! panicked ([`guarded`]).

use super::compare::{Check, Comparison, Layer, Outcome};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::panic::AssertUnwindSafe;

/// How many case indices a category lists per recording before it counts
/// the rest.
pub const INDICES_SHOWN: usize = 12;

/// A name that can only hold what a repository and a command are spelled
/// with, so that it can carry nothing else into the report.
pub fn plain(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || " ./_-".contains(c) {
                c
            } else {
                '?'
            }
        })
        .collect()
}

/// The line a panic leaves on stderr: where it happened, and nothing it was
/// handed. A panic's own message can quote what it was looking at — a slice
/// of a body, an error naming a login — and stderr is the job's public log.
pub fn panic_line(location: Option<&std::panic::Location<'_>>) -> String {
    match location {
        Some(at) => format!("differential: panicked at {}:{}", at.file(), at.line()),
        None => "differential: panicked".to_owned(),
    }
}

/// `f`, with a panic inside it caught and kept to its kind.
pub fn guarded<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(AssertUnwindSafe(f))
        .map_err(|_| "the tool panicked here; its message is not printed".to_owned())
}

/// What one recording's comparison found, and the counts its row shows
/// beside the layers, in the order of [`Table::counts`].
#[derive(Debug, Clone, Default)]
pub struct Found {
    pub comparison: Comparison,
    pub counts: Vec<usize>,
}

/// One recording's line in the report.
#[derive(Debug, Clone)]
pub struct Row {
    /// The recording's repository, through [`plain`].
    pub repository: String,
    /// Its repository and command, through [`plain`].
    pub label: String,
    /// The recording's directory name, through [`plain`], which tells
    /// apart two recordings of the same command.
    pub directory: String,
    /// What was found, or why the recording could not be compared: a
    /// problem that names a file and a position, never what it holds.
    pub found: Result<Found, String>,
}

impl Row {
    /// Whether this row's recording matched everywhere.
    pub fn clean(&self) -> bool {
        matches!(&self.found, Ok(found) if found.comparison.is_clean())
    }
}

/// The columns of a counts table: a matched-of-compared column per layer,
/// then one per count, then the differences.
#[derive(Debug, Clone, Copy)]
pub struct Table {
    /// What the table says before its header.
    pub intro: &'static str,
    pub layers: &'static [(Layer, &'static str)],
    pub counts: &'static [&'static str],
}

/// The counts of one row, or of a total.
#[derive(Debug, Default, Clone)]
struct Counts {
    recordings: usize,
    unreadable: usize,
    layers: Vec<(usize, usize)>,
    counts: Vec<usize>,
    differences: usize,
}

impl Counts {
    fn of(row: &Row, table: &Table) -> Counts {
        let mut counts = Counts {
            recordings: 1,
            layers: vec![(0, 0); table.layers.len()],
            counts: vec![0; table.counts.len()],
            ..Counts::default()
        };
        match &row.found {
            Ok(found) => {
                for (cell, (layer, _)) in counts.layers.iter_mut().zip(table.layers) {
                    *cell = found.comparison.matched(*layer);
                }
                for (cell, value) in counts.counts.iter_mut().zip(&found.counts) {
                    *cell = *value;
                }
                counts.differences = found.comparison.differences();
            }
            Err(_) => {
                counts.unreadable = 1;
                counts.differences = 1;
            }
        }
        counts
    }

    fn add(&mut self, other: &Counts) {
        self.recordings += other.recordings;
        self.unreadable += other.unreadable;
        for (mine, theirs) in self.layers.iter_mut().zip(&other.layers) {
            mine.0 += theirs.0;
            mine.1 += theirs.1;
        }
        for (mine, theirs) in self.counts.iter_mut().zip(&other.counts) {
            *mine += theirs;
        }
        self.differences += other.differences;
    }

    fn cells(&self, name: &str) -> String {
        let mut line = format!("| {name} |");
        for (matched, total) in &self.layers {
            let _ = write!(line, " {matched}/{total} |");
        }
        for count in &self.counts {
            let _ = write!(line, " {count} |");
        }
        if self.unreadable > 0 {
            let _ = writeln!(
                line,
                " {} ({} unreadable) |",
                self.differences, self.unreadable
            );
        } else {
            let _ = writeln!(line, " {} |", self.differences);
        }
        line
    }
}

/// The counts table: a line per recording, then, where there is more than
/// one, a line per repository and one for all.
pub fn counts_table(rows: &[Row], table: &Table) -> String {
    let mut out = format!("{}\n\n| Recording |", table.intro);
    for (_, heading) in table.layers {
        let _ = write!(out, " {heading} |");
    }
    for heading in table.counts {
        let _ = write!(out, " {heading} |");
    }
    out.push_str(" Differences |\n|---|");
    for _ in 0..table.layers.len() + table.counts.len() + 1 {
        out.push_str("---:|");
    }
    out.push('\n');
    let empty = Counts {
        layers: vec![(0, 0); table.layers.len()],
        counts: vec![0; table.counts.len()],
        ..Counts::default()
    };
    let mut repositories: Vec<(&str, Counts)> = Vec::new();
    let mut all = empty.clone();
    for row in rows {
        let counts = Counts::of(row, table);
        out.push_str(&counts.cells(&row.label));
        all.add(&counts);
        match repositories.iter_mut().find(|(r, _)| *r == row.repository) {
            Some((_, total)) => total.add(&counts),
            None => repositories.push((&row.repository, counts)),
        }
    }
    if rows.len() > 1 {
        for (repository, counts) in &repositories {
            let name = format!("**{repository}** ({} recordings)", counts.recordings);
            out.push_str(&counts.cells(&name));
        }
        let name = format!("**all** ({} recordings)", all.recordings);
        out.push_str(&all.cells(&name));
    }
    out
}

/// The differences grouped by what differs: layer, field path and kind,
/// with how many there are and which cases of which recording show them.
/// `cases` says what a case index points at. A difference explained by
/// Python's own inputs, and a pull request that moved during the read, are
/// listed too, saying so, though neither is a failure.
pub fn categories(rows: &[Row], cases: &str) -> String {
    type Where = BTreeMap<String, Vec<usize>>;
    let mut found: BTreeMap<(Layer, String, String), (usize, Where)> = BTreeMap::new();
    let mut note = |layer: Layer, field: String, kind: String, label: &str, index: usize| {
        let (count, at) = found.entry((layer, field, kind)).or_default();
        *count += 1;
        let indices = at.entry(label.to_owned()).or_default();
        if indices.last() != Some(&index) {
            indices.push(index);
        }
    };
    let mut unreadable = Vec::new();
    for row in rows {
        let comparison = match &row.found {
            Ok(found) => &found.comparison,
            Err(problem) => {
                unreadable.push(format!("- {}: {problem}\n", row.label));
                continue;
            }
        };
        for Check {
            layer,
            index,
            outcome,
        } in &comparison.checks
        {
            match outcome {
                Outcome::Matched => {}
                Outcome::Differs(differences) => {
                    for difference in differences {
                        let field = format!("`{}`", difference.field());
                        note(
                            *layer,
                            field,
                            difference.kind.to_string(),
                            &row.label,
                            *index,
                        );
                    }
                }
                Outcome::Explained { differences, by } => {
                    for difference in differences {
                        let field = format!("`{}`", difference.field());
                        let kind = format!("{}, gone with Python's {by}", difference.kind);
                        note(*layer, field, kind, &row.label, *index);
                    }
                }
                Outcome::Moved => {
                    note(
                        *layer,
                        "—".into(),
                        "moved during the read".into(),
                        &row.label,
                        *index,
                    );
                }
                Outcome::Failed { failure, .. } => {
                    note(*layer, "—".into(), failure.to_string(), &row.label, *index);
                }
            }
        }
    }
    let mut out = String::new();
    if !unreadable.is_empty() {
        out.push_str("Recordings that could not be read:\n\n");
        unreadable.iter().for_each(|line| out.push_str(line));
        out.push('\n');
    }
    if found.is_empty() {
        out.push_str("No differences.\n");
        return out;
    }
    let _ = write!(
        out,
        "Field paths only: `[]` is any list item, `*` a key that is data (a login, a digest), \
         `?` a key this tool does not know. {cases}\n\n\
         | Layer | Field | Kind | Count | Cases |\n|---|---|---|---:|---|\n"
    );
    for ((layer, field, kind), (count, at)) in &found {
        let cases: Vec<String> = at
            .iter()
            .map(|(label, indices)| {
                let shown: Vec<String> = indices
                    .iter()
                    .take(INDICES_SHOWN)
                    .map(usize::to_string)
                    .collect();
                let more = indices.len().saturating_sub(INDICES_SHOWN);
                let more = if more > 0 {
                    format!(" and {more} more")
                } else {
                    String::new()
                };
                format!("{label}: {}{more}", shown.join(", "))
            })
            .collect();
        let _ = writeln!(
            out,
            "| {} | {field} | {kind} | {count} | {} |",
            layer.as_str(),
            cases.join("; ")
        );
    }
    out
}

/// Where two rows have the same repository and command, add each one's
/// directory name.
pub fn label_rows(rows: &mut [Row]) {
    let mut times: BTreeMap<String, usize> = BTreeMap::new();
    for row in rows.iter() {
        *times.entry(row.label.clone()).or_default() += 1;
    }
    for row in rows.iter_mut() {
        if times.get(&row.label).is_some_and(|&n| n > 1) {
            row.label = format!("{} ({})", row.label, row.directory);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: Table = Table {
        intro: "Cases matched of cases compared.",
        layers: &[(Layer::Snapshot, "Snapshots")],
        counts: &["Requests"],
    };

    #[test]
    fn a_panic_is_reported_by_where_it_happened_and_never_by_what_it_said() {
        let caught = guarded(|| -> usize { panic!("mallory has admin on dashpay/secret") });
        let problem = caught.unwrap_err();
        assert!(!problem.contains("mallory"), "{problem}");
        let line = panic_line(Some(std::panic::Location::caller()));
        assert!(line.starts_with("differential: panicked at "), "{line}");
        assert!(line.contains("report.rs:"), "{line}");
        assert_eq!(panic_line(None), "differential: panicked");
    }

    #[test]
    fn a_recording_whose_comparison_panics_is_one_unreadable_row() {
        let caught: Result<Found, String> = guarded(|| panic!("a title"));
        let row = Row {
            repository: "?".into(),
            label: "unreadable (x)".into(),
            directory: "x".into(),
            found: caught,
        };
        let table = counts_table(std::slice::from_ref(&row), &TABLE);
        assert!(
            table.contains("| unreadable (x) | 0/0 | 0 | 1 (1 unreadable) |"),
            "{table}"
        );
        assert!(!categories(&[row], "").contains("a title"));
    }

    #[test]
    fn a_name_keeps_only_what_a_repository_and_a_command_are_spelled_with() {
        assert_eq!(
            plain("dashpay/platform · sync --pr 2"),
            "dashpay/platform ? sync --pr 2"
        );
        assert_eq!(plain("a`b|c"), "a?b?c");
    }

    #[test]
    fn an_explained_difference_and_a_move_are_listed_but_are_not_failures() {
        use super::super::compare::Explanation;
        use super::super::diff::{Difference, Kind};
        let comparison = Comparison {
            checks: vec![
                Check {
                    layer: Layer::LiveVerdict,
                    index: 0,
                    outcome: Outcome::Explained {
                        differences: vec![Difference {
                            path: "verdict.admitted_at".into(),
                            kind: Kind::Value,
                        }],
                        by: Explanation {
                            clock: false,
                            telemetry: false,
                            admission: true,
                        },
                    },
                },
                Check {
                    layer: Layer::LiveSnapshot,
                    index: 1,
                    outcome: Outcome::Moved,
                },
            ],
            missing_reads: 0,
        };
        assert!(comparison.is_clean());
        assert_eq!(comparison.differences(), 0);
        let row = Row {
            repository: "a/b".into(),
            label: "a/b · sync --pr 2".into(),
            directory: "x".into(),
            found: Ok(Found {
                comparison,
                counts: vec![0],
            }),
        };
        let said = categories(&[row], "");
        assert!(
            said.contains("| live verdict | `verdict.admitted_at` | value, gone with Python's admission | 1 | a/b · sync --pr 2: 0 |"),
            "{said}"
        );
        assert!(
            said.contains(
                "| live snapshot | — | moved during the read | 1 | a/b · sync --pr 2: 1 |"
            ),
            "{said}"
        );
    }
}
