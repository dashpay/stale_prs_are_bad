//! The engine's shadow on recorded runs: replay each boundary recording
//! through the Rust port and say, in counts and field paths, where it
//! departs from what the Python engine did.
//!
//! ```text
//! differential [--summary FILE] [--python-source DIR] DIR...
//! differential --requests DIR...
//! ```
//!
//! Each `DIR` is a recording, or a directory searched for recordings. For
//! each one the port rebuilds every snapshot Python's `evaluate` saw from
//! the recorded reads alone, runs `evaluate` on what Python's was given, and
//! compares the verdict rows as far as `evaluate` decides them. The report
//! is Markdown on stdout: a counts table per recording and per repository,
//! then the differences grouped by layer, field path and kind, with the
//! indices of the cases that show each. `--summary FILE` appends the counts
//! table alone to `FILE`.
//!
//! Nothing a recording holds is printed: no title, body, login or
//! permission level, and no error message, which can quote them. A
//! recording is named by its repository and the command recorded.
//!
//! `--requests` prints one number: how many requests to GitHub the
//! recordings' reads made, counting each page of a paginated read.
//!
//! Exit status: 0 when every recording matched, 1 when any differed or
//! could not be read, 2 when the command itself could not run.

use pr_hygiene_engine::conformance::{compare, Comparison, Layer, Outcome, OwnWords, Recording};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// How many case indices a category lists per recording before it counts
/// the rest.
const INDICES_SHOWN: usize = 12;

const USAGE: &str = "usage: differential [--summary FILE] [--python-source DIR] DIR...\n       differential --requests DIR...";

struct Options {
    summary: Option<PathBuf>,
    python_source: PathBuf,
    requests: bool,
    paths: Vec<PathBuf>,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        summary: None,
        python_source: Path::new(env!("CARGO_MANIFEST_DIR")).join("../pr_review"),
        requests: false,
        paths: Vec::new(),
    };
    let mut args = std::env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--summary") => {
                options.summary = Some(args.next().ok_or("--summary needs a file")?.into())
            }
            Some("--python-source") => {
                options.python_source = args
                    .next()
                    .ok_or("--python-source needs a directory")?
                    .into()
            }
            Some("--requests") => options.requests = true,
            Some("-h" | "--help") => return Err(String::new()),
            Some(flag) if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            _ => options.paths.push(arg.into()),
        }
    }
    if options.paths.is_empty() {
        return Err("no recording given".into());
    }
    Ok(options)
}

/// Every recording at or under each path, in a stable order.
fn recordings(paths: &[PathBuf]) -> Vec<PathBuf> {
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

fn load(dir: &Path) -> Result<Recording, String> {
    Recording::load_with(|name| std::fs::read_to_string(dir.join(name))).map_err(|e| e.to_string())
}

/// A name that can only hold what a repository and a command are spelled
/// with, so that it can carry nothing else into the report.
fn plain(text: &str) -> String {
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

/// One recording's line in the report.
struct Row {
    repository: String,
    label: String,
    /// The recording's directory name, which tells apart two recordings of
    /// the same command.
    directory: String,
    found: Result<(Comparison, usize), String>,
}

/// The counts of one row, or of a total.
#[derive(Default, Clone, Copy)]
struct Counts {
    recordings: usize,
    unreadable: usize,
    snapshots: (usize, usize),
    evaluations: (usize, usize),
    verdicts: (usize, usize),
    missing_reads: usize,
    requests: usize,
    differences: usize,
}

impl Counts {
    fn of(row: &Row) -> Counts {
        let mut counts = Counts {
            recordings: 1,
            ..Counts::default()
        };
        match &row.found {
            Ok((comparison, requests)) => {
                counts.snapshots = comparison.matched(Layer::Snapshot);
                counts.evaluations = comparison.matched(Layer::Evaluation);
                counts.verdicts = comparison.matched(Layer::Verdict);
                counts.missing_reads = comparison.missing_reads;
                counts.requests = *requests;
                counts.differences = comparison.differences();
            }
            Err(_) => {
                counts.unreadable = 1;
                counts.differences = 1;
            }
        }
        counts
    }

    fn add(&mut self, other: Counts) {
        let pair = |a: &mut (usize, usize), b: (usize, usize)| {
            a.0 += b.0;
            a.1 += b.1;
        };
        self.recordings += other.recordings;
        self.unreadable += other.unreadable;
        pair(&mut self.snapshots, other.snapshots);
        pair(&mut self.evaluations, other.evaluations);
        pair(&mut self.verdicts, other.verdicts);
        self.missing_reads += other.missing_reads;
        self.requests += other.requests;
        self.differences += other.differences;
    }

    fn cells(&self, name: &str) -> String {
        let ratio = |(matched, total): (usize, usize)| format!("{matched}/{total}");
        let differences = if self.unreadable > 0 {
            format!("{} ({} unreadable)", self.differences, self.unreadable)
        } else {
            self.differences.to_string()
        };
        format!(
            "| {name} | {} | {} | {} | {} | {} | {differences} |\n",
            ratio(self.snapshots),
            ratio(self.evaluations),
            ratio(self.verdicts),
            self.missing_reads,
            self.requests,
        )
    }
}

fn counts_table(rows: &[Row]) -> String {
    let mut out = String::from(
        "| Recording | Snapshots matched | Evaluations matched | Verdicts matched | Missing reads | Requests | Differences |\n\
         |---|---:|---:|---:|---:|---:|---:|\n",
    );
    let mut repositories: Vec<(&str, Counts)> = Vec::new();
    let mut all = Counts::default();
    for row in rows {
        let counts = Counts::of(row);
        out.push_str(&counts.cells(&row.label));
        all.add(counts);
        match repositories.iter_mut().find(|(r, _)| *r == row.repository) {
            Some((_, total)) => total.add(counts),
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
fn categories(rows: &[Row]) -> String {
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
            Ok((comparison, _)) => comparison,
            Err(problem) => {
                unreadable.push(format!("- {}: {problem}\n", row.label));
                continue;
            }
        };
        for check in &comparison.checks {
            match &check.outcome {
                Outcome::Matched => {}
                Outcome::Differs(differences) => {
                    for difference in differences {
                        let field = format!("`{}`", difference.field());
                        note(
                            check.layer,
                            field,
                            difference.kind.to_string(),
                            &row.label,
                            check.index,
                        );
                    }
                }
                Outcome::Failed { failure, .. } => {
                    note(
                        check.layer,
                        "—".into(),
                        failure.to_string(),
                        &row.label,
                        check.index,
                    );
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
    out.push_str(
        "Field paths only: `[]` is any list item, `*` a key that is data (a login, a digest), \
         `?` a key this tool does not know. Cases are indices into `evaluations.jsonl` \
         (snapshots and evaluations) and `verdicts.json` (verdicts).\n\n\
         | Layer | Field | Kind | Count | Cases |\n|---|---|---|---:|---|\n",
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
fn label_rows(rows: &mut [Row]) {
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

fn run() -> Result<bool, String> {
    let options = options()?;
    let dirs = recordings(&options.paths);
    if dirs.is_empty() {
        return Err("no recording found under the paths given".into());
    }
    if options.requests {
        let mut total = 0;
        for dir in &dirs {
            let recording = load(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            total += recording
                .requests()
                .map_err(|e| format!("{}: {e}", dir.display()))?;
        }
        println!("{total}");
        return Ok(true);
    }
    let source = |name: &str| {
        let path = options.python_source.join(name);
        std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))
    };
    let own = OwnWords::from_sources(&source("conformance.py")?, &source("policy.py")?)
        .map_err(|e| format!("the engine's own words: {e}"))?;
    let mut rows: Vec<Row> = dirs
        .iter()
        .map(|dir| {
            let directory = plain(&dir.file_name().unwrap_or_default().to_string_lossy());
            match load(dir) {
                Ok(recording) => {
                    let repository = plain(recording.repository().unwrap_or("?"));
                    let label = format!("{repository} · {}", plain(&recording.command()));
                    let found = recording
                        .requests()
                        .map(|requests| (compare(&recording, &own), requests))
                        .map_err(|e| e.to_string());
                    Row {
                        repository,
                        label,
                        directory,
                        found,
                    }
                }
                Err(problem) => Row {
                    repository: "?".into(),
                    label: format!("unreadable ({directory})"),
                    directory,
                    found: Err(problem),
                },
            }
        })
        .collect();
    label_rows(&mut rows);
    let clean = rows
        .iter()
        .all(|row| matches!(&row.found, Ok((comparison, _)) if comparison.is_clean()));
    let table = counts_table(&rows);
    print!(
        "## Engine differential\n\n{} recording(s): {}.\n\n{table}\n### Differences by category\n\n{}",
        rows.len(),
        if clean { "the Rust engine matched every one" } else { "the Rust engine differed" },
        categories(&rows)
    );
    if let Some(summary) = &options.summary {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(summary)
            .map_err(|e| format!("{}: {e}", summary.display()))?;
        write!(file, "## Engine differential\n\n{table}")
            .map_err(|e| format!("{}: {e}", summary.display()))?;
    }
    Ok(clean)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(problem) => {
            if !problem.is_empty() {
                eprintln!("differential: {problem}");
            }
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}
