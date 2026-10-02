//! The engine's shadow on recorded runs: replay each boundary recording
//! through the Rust port and say, in counts and field paths, where it
//! departs from what the Python engine did.
//!
//! ```text
//! differential [--summary FILE] [--python-source DIR] DIR...
//! differential --requests DIR...
//! differential --request-kinds DIR...
//! ```
//!
//! Each `DIR` is a recording, or a directory searched for recordings. For
//! each one the port rebuilds every snapshot Python's `evaluate` saw from
//! the recorded reads alone, runs `evaluate` on what Python's was given, and
//! compares the verdict rows as far as `evaluate` decides them. Then it
//! replays the whole run through its reconcile layer: the run's outcome,
//! its verdict rows, every write in the recorded order with its body, what
//! each verdict puts on GitHub (`outputs.json`), the JSON report it printed,
//! where it read the clock and the order of every call. The report is
//! Markdown on stdout: a counts table per recording and per repository, then
//! the differences grouped by layer, field path and kind, with the indices
//! of the cases that show each. `--summary FILE` appends the counts table
//! alone to `FILE`.
//!
//! Nothing a recording holds is printed: no title, body, login or
//! permission level, and no error message, which can quote them. A panic
//! prints only where it happened, and a recording whose comparison panics
//! is one unreadable row. A recording is named by its repository and the
//! command recorded. The report is the conformance module's
//! ([`report`](pr_hygiene_engine::conformance::report)), which the live
//! tool prints too.
//!
//! `--requests` prints one number: how many requests to GitHub the
//! recordings' reads made, counting each page of a paginated read.
//! `--request-kinds` prints the same as two: the REST requests, then the
//! GraphQL queries, the two limits GitHub counts them against.
//!
//! Exit status: 0 when every recording matched, 1 when any differed or
//! could not be read, 2 when the command itself could not run.

use pr_hygiene_engine::conformance::report::{
    categories, counts_table, guarded, label_rows, panic_line, plain, Found, Row, Table,
};
use pr_hygiene_engine::conformance::{
    compare, replay_run, Check, Failure, Layer, OwnWords, Recording, RunFiles,
};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// The counts table: the layers in the order a recording is compared.
const TABLE: Table = Table {
    intro: "Each layer is cases matched of cases compared. The first three hold \
            `evaluate` to Python's; the rest replay the whole run.",
    layers: &[
        (Layer::Snapshot, "Snapshots"),
        (Layer::Evaluation, "Evaluations"),
        (Layer::Verdict, "Verdicts"),
        (Layer::Run, "Outcome"),
        (Layer::RunVerdict, "Run verdicts"),
        (Layer::Write, "Writes"),
        (Layer::Output, "Outputs"),
        (Layer::Report, "Report"),
        (Layer::Clock, "Clock"),
        (Layer::Call, "Calls"),
    ],
    counts: &["Missing reads", "Requests"],
};

/// What a case index in the categories points at.
const CASES: &str = "Cases are indices: into `evaluations.jsonl` for \
    snapshots and evaluations; into `verdicts.json` for verdicts, run verdicts and \
    outputs; into the recorded writes, in order from 0, for writes (a write the recording \
    does not hold is numbered on past its last); and 0 for the run as \
    a whole (outcome, report, clock, calls), with 1 for how often the outcome read the \
    review system's status page.";

const USAGE: &str = "usage: differential [--summary FILE] [--python-source DIR] DIR...\n       differential --requests DIR...\n       differential --request-kinds DIR...";

struct Options {
    summary: Option<PathBuf>,
    python_source: PathBuf,
    requests: bool,
    request_kinds: bool,
    paths: Vec<PathBuf>,
}

/// The command line, or `None` when it asks for help.
fn options() -> Result<Option<Options>, String> {
    let mut options = Options {
        summary: None,
        python_source: Path::new(env!("CARGO_MANIFEST_DIR")).join("../pr_review"),
        requests: false,
        request_kinds: false,
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
            Some("--request-kinds") => options.request_kinds = true,
            Some("-h" | "--help") => return Ok(None),
            Some(flag) if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            _ => options.paths.push(arg.into()),
        }
    }
    if options.paths.is_empty() {
        return Err("no recording given".into());
    }
    Ok(Some(options))
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

/// What the recorded run put out: its outputs and its printed report.
fn load_run(dir: &Path) -> Result<RunFiles, String> {
    RunFiles::load_with(|name| std::fs::read_to_string(dir.join(name))).map_err(|e| e.to_string())
}

/// One recording's row: loaded, counted and compared, a panic included.
fn row(dir: &Path, own: &OwnWords) -> Row {
    let directory = plain(&dir.file_name().unwrap_or_default().to_string_lossy());
    let compared = guarded(|| {
        let recording = load(dir)?;
        let repository = plain(recording.repository().unwrap_or("?"));
        let label = format!("{repository} · {}", plain(&recording.command()));
        let found = recording
            .requests()
            .map_err(|e| e.to_string())
            .map(|requests| {
                let mut comparison = compare(&recording, own);
                // The whole run's reads are the snapshots' and more: a read it
                // lacks stops it, and its outcome says so. The missing reads
                // counted are the snapshots'. What the run put out that cannot
                // be read fails the whole-run layers and keeps the rest.
                match load_run(dir) {
                    Ok(files) => comparison
                        .checks
                        .extend(replay_run(&recording, &files, own).checks),
                    Err(problem) => comparison.checks.push(Check::failed(
                        Layer::Run,
                        0,
                        Failure::RunFilesUnreadable,
                        problem,
                    )),
                }
                let missing_reads = comparison.missing_reads;
                Found {
                    comparison,
                    counts: vec![missing_reads, requests],
                }
            });
        Ok::<_, String>((repository, label, found))
    })
    .and_then(|loaded| loaded);
    match compared {
        Ok((repository, label, found)) => Row {
            repository,
            label,
            directory,
            found,
        },
        Err(problem) => Row {
            repository: "?".into(),
            label: format!("unreadable ({directory})"),
            directory,
            found: Err(problem),
        },
    }
}

fn run(options: Options) -> Result<bool, String> {
    let dirs = recordings(&options.paths);
    if dirs.is_empty() {
        return Err("no recording found under the paths given".into());
    }
    if options.request_kinds {
        let (mut rest, mut graphql) = (0, 0);
        for dir in &dirs {
            let counted = guarded(|| load(dir)?.requests_by_kind().map_err(|e| e.to_string()))
                .and_then(|counted| counted);
            let (r, g) = counted.map_err(|e| format!("{}: {e}", dir.display()))?;
            rest += r;
            graphql += g;
        }
        println!("{rest} {graphql}");
        return Ok(true);
    }
    if options.requests {
        let mut total = 0;
        for dir in &dirs {
            let counted = guarded(|| load(dir)?.requests().map_err(|e| e.to_string()))
                .and_then(|counted| counted);
            total += counted.map_err(|e| format!("{}: {e}", dir.display()))?;
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
    let mut rows: Vec<Row> = dirs.iter().map(|dir| row(dir, &own)).collect();
    label_rows(&mut rows);
    let clean = rows.iter().all(Row::clean);
    let table = counts_table(&rows, &TABLE);
    print!(
        "## Engine differential\n\n{} recording(s): {}.\n\n{table}\n### Differences by category\n\n{}",
        rows.len(),
        if clean { "the Rust engine matched every one" } else { "the Rust engine differed" },
        categories(&rows, CASES)
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
    std::panic::set_hook(Box::new(|info| {
        eprintln!("{}", panic_line("differential", info.location()))
    }));
    let options = match options() {
        Ok(Some(options)) => options,
        Ok(None) => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(problem) => {
            eprintln!("differential: {problem}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match run(options) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(problem) => {
            eprintln!("differential: {problem}");
            ExitCode::from(2)
        }
    }
}
