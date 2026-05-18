use pr_hygiene::{analyzer, config, fetcher, history, labeler, renderer, scorer};

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "pr-hygiene", version, about = "Nightly PR-hygiene dashboard")]
struct Args {
    /// Override the target repo (else from .pr-hygiene.yml). Form "owner/name".
    #[arg(long)]
    repo: Option<String>,

    /// GitHub token. Can also be supplied via $GITHUB_TOKEN.
    #[arg(long, env = "GITHUB_TOKEN")]
    token: String,

    /// Path to the config file.
    #[arg(long, default_value = ".pr-hygiene.yml")]
    config: PathBuf,

    /// Skip writing files and skip label mutations; print what would happen.
    #[arg(long)]
    dry_run: bool,

    /// Output path for the markdown report.
    #[arg(long, default_value = "docs/index.md")]
    out: PathBuf,

    /// Optional commit SHA to display in the header (else read from $GITHUB_SHA).
    #[arg(long, env = "GITHUB_SHA")]
    commit_sha: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("pr_hygiene=info,info")),
        )
        .with_target(false)
        .init();

    let args = Args::parse();
    let mut cfg = config::Config::load_or_default(&args.config)?;
    if let Some(repo) = &args.repo {
        cfg.target_repo = repo.clone();
    }
    let (owner, name) = {
        let (o, n) = cfg.repo_parts()?;
        (o.to_string(), n.to_string())
    };
    let now = Utc::now();
    let today = now.date_naive();
    tracing::info!(repo = %cfg.target_repo, %today, dry_run = args.dry_run, "starting");

    let fetcher = fetcher::Fetcher::new(&args.token)?;
    let (mut raw_prs, node_ids, fetched_default_branch) =
        fetcher.fetch_all_open_prs(&owner, &name).await?;
    fetcher
        .recheck_mergeable(&mut raw_prs, &node_ids, Duration::from_secs(3))
        .await?;

    // Auto-detect the default branch when the user hasn't pinned one in config.
    // Without this, every PR would be flagged as "targets non-default" → Stale.
    if cfg.default_target_branch.is_none() {
        match fetched_default_branch.as_deref() {
            Some(b) => {
                tracing::info!("auto-detected default branch: {b}");
                cfg.default_target_branch = Some(b.to_string());
            }
            None => tracing::warn!(
                "could not auto-detect default branch; branch-based stale detection disabled"
            ),
        }
    } else {
        tracing::info!(
            "using configured default branch: {}",
            cfg.default_target_branch.as_deref().unwrap_or("?")
        );
    }
    tracing::info!("fetched {} open PRs", raw_prs.len());

    let analyzed = analyzer::analyze(raw_prs, &cfg, now);

    let author_cache_path = PathBuf::from(history::AUTHOR_CACHE);
    let mut author_cache = history::load_author_cache(&author_cache_path)?;
    let filtered =
        analyzer::apply_grace_period(analyzed, &mut author_cache, cfg.grace_period_days, today);
    tracing::info!("{} PRs survive filters", filtered.len());

    let scored = scorer::score_prs(filtered, &cfg, now);

    let history_dir = PathBuf::from(history::HISTORY_DIR);
    let previous = history::load_previous(&history_dir, today)?;
    let has_history = previous.is_some();
    let authors = scorer::rollup_authors(&scored, previous.as_ref());
    let snapshot = scorer::build_snapshot(today, &scored, &authors);

    // When running in Actions, link to the config file on github.com (the file isn't in
    // the Pages-published docs/ folder). Locally, keep the bare relative path.
    let config_url = match std::env::var("GITHUB_REPOSITORY") {
        Ok(repo) if !repo.is_empty() => {
            format!("https://github.com/{repo}/blob/master/.pr-hygiene.yml")
        }
        _ => ".pr-hygiene.yml".to_string(),
    };
    let render_ctx = renderer::RenderContext {
        now,
        commit_sha: args.commit_sha.as_deref(),
        config_path: &config_url,
        has_history,
    };
    let markdown = renderer::render(&scored, &authors, &render_ctx);

    if args.dry_run {
        tracing::info!(
            "--dry-run: would write {} bytes to {}",
            markdown.len(),
            args.out.display()
        );
        println!("{markdown}");
    } else {
        let changed = write_if_changed(&args.out, &markdown)?;
        tracing::info!("report {}", if changed { "updated" } else { "unchanged" });
        history::write_snapshot(&history_dir, &snapshot)?;
        history::save_author_cache(&author_cache_path, &author_cache)?;
        let pruned = history::prune(&history_dir, today, cfg.history_retention_days)?;
        if pruned > 0 {
            tracing::info!("pruned {pruned} old snapshot(s)");
        }
    }

    if cfg.auto_label {
        let diff =
            labeler::compute_diff(scored.iter().map(|s| &s.pr), &cfg.needs_author_action_label);
        if args.dry_run {
            tracing::info!(
                "--dry-run: label diff: add {:?}, remove {:?}",
                diff.to_add,
                diff.to_remove
            );
        } else if diff.to_add.is_empty() && diff.to_remove.is_empty() {
            tracing::info!("label diff is empty; nothing to do");
        } else {
            let labeler = labeler::Labeler::new(&args.token)?;
            match labeler
                .apply(&owner, &name, &diff, &cfg.needs_author_action_label)
                .await
            {
                Ok(()) => tracing::info!(
                    "applied label diff: +{} / -{}",
                    diff.to_add.len(),
                    diff.to_remove.len()
                ),
                Err(e) => tracing::warn!(
                    "label mutations failed (often the default GITHUB_TOKEN can't write \
                     to the target repo; see README): {e:#}"
                ),
            }
        }
    }

    Ok(())
}

fn write_if_changed(path: &std::path::Path, contents: &str) -> Result<bool> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
    }
    if let Ok(existing) = std::fs::read_to_string(path) {
        if existing == contents {
            return Ok(false);
        }
    }
    std::fs::write(path, contents).with_context(|| format!("writing {}", path.display()))?;
    Ok(true)
}
