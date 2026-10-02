use anyhow::Context;
use chrono::Utc;
use clap::{Parser, Subcommand};
use pr_hygiene_service::app::{self, AppState};
use pr_hygiene_service::config::ServeConfig;
use pr_hygiene_service::github::GithubSignIn;
use pr_hygiene_service::oidc::{GithubKeys, KeyCache};
use pr_hygiene_service::snapshot;
use pr_hygiene_service::store::{Outcome, Reader, Store};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
// Parsed once at start-up: the size of the larger variant costs nothing.
#[allow(clippy::large_enum_variant)]
enum Command {
    /// Serve the public API and accept snapshots at /ingest.
    Serve(ServeConfig),
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        // Colour only on a terminal; a log collector would store the codes.
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stdout()))
        .init();
    match Cli::parse().command {
        Command::Serve(cfg) => serve(cfg).await,
        Command::Import { db, file } => import(&db, &file),
    }
}

async fn serve(cfg: ServeConfig) -> anyhow::Result<()> {
    anyhow::ensure!(
        !cfg.ingest.audience.trim().is_empty(),
        "PR_HYGIENE_OIDC_AUDIENCE must name this service"
    );
    let signin = cfg.signin.resolve()?;
    let store = Store::open(&cfg.db)?;
    let reader = Reader::open(&cfg.db)?;
    let keys = Arc::new(KeyCache::new(Box::new(GithubKeys::new()?)));
    tokio::spawn(keep_keys_fresh(keys.clone()));
    let mut state = AppState::new(cfg.ingest, keys, store, reader);
    match signin {
        Some(signin) => {
            tracing::info!(origin = %signin.origin, "sign-in with GitHub on");
            let github = Arc::new(GithubSignIn::new(&signin)?);
            state = state.with_signin(signin, github);
        }
        None => tracing::info!("sign-in with GitHub off: PR_HYGIENE_SIGNIN_* not set"),
    }
    let state = Arc::new(state);
    tokio::spawn(purge_daily(state.clone()));
    let listener = tokio::net::TcpListener::bind(cfg.bind)
        .await
        .with_context(|| format!("binding {}", cfg.bind))?;
    tracing::info!(addr = %cfg.bind, db = %cfg.db.display(), "listening");
    let mut router = app::router(state);
    if let Some(dir) = cfg.site_dir {
        router = router.fallback_service(app::site(dir));
    }
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;
    Ok(())
}

/// Load the signing keys at start-up, retrying each minute until they load,
/// then refetch hourly so a key GitHub has retired stops being trusted. An
/// unknown key id between refetches triggers one of its own.
async fn keep_keys_fresh(keys: Arc<KeyCache>) {
    loop {
        keys.refresh().await;
        let wait = if keys.state().keys == 0 { 60 } else { 3600 };
        tokio::time::sleep(Duration::from_secs(wait)).await;
    }
}

/// Delete expired sessions and abandoned sign-ins at start-up and daily
/// after: they let no one in once expired, but they are personal data.
/// Runs with sign-in off too, so sessions from when it was on still go.
async fn purge_daily(state: Arc<AppState>) {
    loop {
        // A failure is logged where it happens; the next day tries again.
        if let Ok(purged) = state.write(|store| store.purge_expired(Utc::now())).await {
            tracing::info!(
                sessions = purged.sessions,
                prelogins = purged.prelogins,
                "expired sign-in data purged"
            );
        }
        tokio::time::sleep(Duration::from_secs(24 * 60 * 60)).await;
    }
}

async fn shutdown() {
    let interrupt = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = interrupt => {},
        () = terminate => {},
    }
    tracing::info!("shutting down");
}

fn import(db: &Path, file: &Path) -> anyhow::Result<()> {
    let raw =
        std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let d = snapshot::parse(raw.as_bytes(), Utc::now())
        .map_err(|e| anyhow::anyhow!("{}: {e}", file.display()))?;
    let mut store = Store::open(db)?;
    match store.ingest(&d, &raw, None, Utc::now())? {
        Outcome::Stored {
            snapshot,
            stale,
            stage_changes,
        } => println!(
            "stored snapshot {snapshot} ({} PRs, {} people); stale: {}; stage changes: {stage_changes}",
            d.prs.len(),
            d.people.len(),
            if stale.is_empty() { "none".to_string() } else { stale.join(", ") }
        ),
        Outcome::NotNewer { latest } => {
            println!("not stored: generated at {}, not after the latest ({latest})", d.generated_at)
        }
        // An import carries no token to have been used.
        Outcome::TokenUsed => println!("not stored: token already used"),
    }
    Ok(())
}
