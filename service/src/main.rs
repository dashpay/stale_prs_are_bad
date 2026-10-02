use anyhow::Context;
use chrono::Utc;
use clap::{Parser, Subcommand};
use pr_hygiene_service::snapshot;
use pr_hygiene_service::store::{Outcome, Store};
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Store a snapshot from a file, with no token: for trying the service
    /// locally and for restoring data. It needs write access to the database
    /// file itself, so it gives nothing to anyone who can only reach the
    /// service over HTTP. The schema and bounds are checked as on ingest;
    /// the binding to a posting run is not, as there is no run.
    Import {
        #[arg(
            long,
            env = "PR_HYGIENE_DB",
            default_value = "/data/pr-hygiene.sqlite3"
        )]
        db: PathBuf,
        /// A dashboard.json written by `pr-hygiene --json-out`.
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        // Colour only on a terminal; a log collector would store the codes.
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stdout()))
        .init();
    match Cli::parse().command {
        Command::Import { db, file } => import(&db, &file),
    }
}

fn import(db: &Path, file: &Path) -> anyhow::Result<()> {
    let raw =
        std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let d =
        snapshot::parse(raw.as_bytes()).map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))?;
    let mut store = Store::open(db)?;
    match store.ingest(&d, &raw, Utc::now())? {
        Outcome::Stored {
            snapshot,
            stale,
            stage_changes,
        } => println!(
            "stored snapshot {snapshot} ({} PRs, {} people); stale: {}; stage changes: {stage_changes}",
            d.prs.len(),
            d.people.len(),
            if stale.is_empty() {
                "none".to_string()
            } else {
                stale.join(", ")
            }
        ),
        Outcome::NotNewer { latest } => {
            println!(
                "not stored: generated at {}, not after the latest ({latest})",
                d.generated_at
            )
        }
    }
    Ok(())
}
