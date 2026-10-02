//! The engine differential's live reads: right after the Python engine
//! recorded a repository's dry runs, read the same pull requests again with
//! the Rust engine, live, and say in counts and field paths where it
//! departs from what Python read and decided.
//!
//! ```text
//! differential-live [--budget N] [--report-prs K] [--slot S]
//!                   [--summary FILE] [--spent FILE]
//!                   [--repositories FILE] [--python-source DIR] DIR...
//! ```
//!
//! Each `DIR` is a recording, or a directory searched for recordings. A
//! report has `K` of its pull requests read live (2 unless given), chosen
//! by the slot `S` so that each comes round; a one-author `sync --pr N` is
//! run whole, live; anything else is not read. The live reads may make `N`
//! requests in all (unlimited unless given); a recording whose reads would
//! pass that is skipped. See `pr_hygiene_service::differential`.
//!
//! GitHub is read with the token in `GH_TOKEN`, a token another holder
//! minted (in the job, its read-only App installation token), through the
//! reader's [`ReadOnly`](pr_hygiene_service::reader::ReadOnly) layer,
//! which lets through only the engine's reads of the repositories listed in
//! `--repositories` (`policies/repositories.json` unless given). Nothing is
//! ever written to GitHub. The token is never printed.
//!
//! The report is Markdown on stdout, the replay tool's format: counts,
//! layers, field paths and kinds, and nothing a recording or a live answer
//! holds. No log is set up, so nothing the reader logs is printed either.
//! `--summary FILE` appends the counts table alone to `FILE`; `--spent
//! FILE` writes how many requests the live reads made, as one number.
//!
//! Exit status: 0 when every recording read live matched, or differed only
//! where a difference is explained or a pull request moved; 1 when any
//! differed or could not be read; 2 when the command itself could not run.

use anyhow::Context;
use pr_hygiene_engine::conformance::report::panic_line;
use pr_hygiene_engine::conformance::OwnWords;
use pr_hygiene_engine::reconcile::SystemClock;
use pr_hygiene_service::config::Secret;
use pr_hygiene_service::differential::{compare, recordings, Reads, Settings};
use pr_hygiene_service::reader::{read_only_transport, GivenToken, StatusPage, TokenSource};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

const USAGE: &str = "usage: differential-live [--budget N] [--report-prs K] [--slot S] [--summary FILE] [--spent FILE] [--repositories FILE] [--python-source DIR] DIR...";

struct Options {
    settings: Settings,
    summary: Option<PathBuf>,
    spent: Option<PathBuf>,
    repositories: PathBuf,
    python_source: PathBuf,
    paths: Vec<PathBuf>,
}

/// The command line, or `None` when it asks for help.
fn options() -> Result<Option<Options>, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut options = Options {
        settings: Settings {
            budget: usize::MAX,
            report_prs: 2,
            slot: 0,
        },
        summary: None,
        spent: None,
        repositories: root.join("policies/repositories.json"),
        python_source: root.join("pr_review"),
        paths: Vec::new(),
    };
    let mut args = std::env::args_os().skip(1);
    let number = |value: Option<std::ffi::OsString>, flag: &str| -> Result<usize, String> {
        value
            .and_then(|v| v.to_str().and_then(|v| v.parse().ok()))
            .ok_or(format!("{flag} needs a number"))
    };
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--budget") => options.settings.budget = number(args.next(), "--budget")?,
            Some("--report-prs") => {
                options.settings.report_prs = number(args.next(), "--report-prs")?
            }
            Some("--slot") => options.settings.slot = number(args.next(), "--slot")?,
            Some("--summary") => {
                options.summary = Some(args.next().ok_or("--summary needs a file")?.into())
            }
            Some("--spent") => {
                options.spent = Some(args.next().ok_or("--spent needs a file")?.into())
            }
            Some("--repositories") => {
                options.repositories = args.next().ok_or("--repositories needs a file")?.into()
            }
            Some("--python-source") => {
                options.python_source = args
                    .next()
                    .ok_or("--python-source needs a directory")?
                    .into()
            }
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

/// The governed repositories, `owner/name`: what the read-only layer lets
/// the live reads read.
fn repositories(path: &Path) -> anyhow::Result<Vec<String>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let registry: serde_json::Value =
        serde_json::from_str(&text).with_context(|| format!("{} is not JSON", path.display()))?;
    let listed = registry["repositories"]
        .as_array()
        .context("the registry lists no repositories")?;
    listed
        .iter()
        .map(|entry| {
            entry["repository"]
                .as_str()
                .map(str::to_owned)
                .context("a registry entry names no repository")
        })
        .collect()
}

fn run(options: Options) -> anyhow::Result<bool> {
    let dirs = recordings(&options.paths);
    anyhow::ensure!(!dirs.is_empty(), "no recording found under the paths given");
    let token = std::env::var("GH_TOKEN")
        .ok()
        .filter(|token| !token.is_empty())
        .context("GH_TOKEN holds no token")?;
    let tokens: Arc<dyn TokenSource> = Arc::new(GivenToken::new(Secret::new(token)));
    let repositories = repositories(&options.repositories)?;
    let source = |name: &str| {
        let path = options.python_source.join(name);
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
    };
    let own = OwnWords::from_sources(&source("conformance.py")?, &source("policy.py")?)
        .map_err(|e| anyhow::anyhow!("the engine's own words: {e}"))?;
    // The engine blocks on each request while the runtime's workers drive
    // it: a multi-thread runtime, and the engine on a blocking thread.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    let handle = runtime.handle().clone();
    let settings = options.settings;
    let outcome = runtime
        .block_on(tokio::task::spawn_blocking(move || {
            let page = StatusPage::new(handle.clone())?;
            let mut transport =
                || read_only_transport(handle.clone(), tokens.clone(), &repositories);
            let mut status_page = || page.fetch();
            anyhow::Ok(compare(
                &dirs,
                &own,
                settings,
                Reads {
                    transport: &mut transport,
                    clock: &mut SystemClock,
                    status_page: &mut status_page,
                },
            ))
        }))
        // A task that panicked carries the panic's message, which can quote
        // what it was looking at: only that it panicked is said.
        .map_err(|_| anyhow::anyhow!("the live reads panicked; the message is not printed"))??;
    print!("{}", outcome.printed());
    if let Some(summary) = &options.summary {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(summary)
            .with_context(|| format!("opening {}", summary.display()))?;
        write!(file, "{}", outcome.summary())
            .with_context(|| format!("writing {}", summary.display()))?;
    }
    if let Some(spent) = &options.spent {
        std::fs::write(spent, format!("{}\n", outcome.spent))
            .with_context(|| format!("writing {}", spent.display()))?;
    }
    Ok(outcome.clean())
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("{}", panic_line("differential-live", info.location()))
    }));
    let options = match options() {
        Ok(Some(options)) => options,
        Ok(None) => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(problem) => {
            eprintln!("differential-live: {problem}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match run(options) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(problem) => {
            eprintln!("differential-live: {problem:#}");
            ExitCode::from(2)
        }
    }
}
