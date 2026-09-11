use pr_hygiene::{analyzer, config, fetcher, history, policy, renderer, scorer};

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "pr-hygiene", version, about = "Nightly PR-hygiene dashboard")]
struct Args {
    /// Only analyze this registered repository (form "owner/name"). Default: every
    /// repository in the policy registry.
    #[arg(long)]
    repo: Option<String>,

    /// GitHub token. Can also be supplied via $GITHUB_TOKEN.
    #[arg(long, env = "GITHUB_TOKEN")]
    token: String,

    /// Path to the config file.
    #[arg(long, default_value = ".pr-hygiene.yml")]
    config: PathBuf,

    /// Directory holding `repositories.json` and the per-repository policy files.
    #[arg(long, default_value = "policies")]
    policies_root: PathBuf,

    /// Directory of review-engine exports, one `<name>.json` per repository
    /// (`pr_review.main sync --format json`). Missing files render as unavailable.
    #[arg(long)]
    policy_state: Option<PathBuf>,

    /// Skip writing files; print the report instead.
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
    let cfg = config::Config::load_or_default(&args.config)?;
    let mut registry = policy::load_registry(&args.policies_root)?;
    if let Some(only) = &args.repo {
        registry.retain(|(repo, _)| repo == only);
        if registry.is_empty() {
            anyhow::bail!(
                "{only} is not registered in {}",
                args.policies_root.join(policy::REGISTRY_FILE).display()
            );
        }
    }
    let now = Utc::now();
    let today = now.date_naive();
    let repo_names: Vec<String> = registry.iter().map(|(r, _)| r.clone()).collect();
    tracing::info!(repos = ?repo_names, %today, dry_run = args.dry_run, "starting");

    let fetcher = fetcher::Fetcher::new(&args.token)?;
    let mut analyzed = Vec::new();
    for repo in &repo_names {
        let (owner, name) = config::repo_parts(repo)?;
        let (mut raw_prs, node_ids, default_branch) =
            fetcher.fetch_all_open_prs(owner, name).await?;
        fetcher
            .recheck_mergeable(&mut raw_prs, &node_ids, Duration::from_secs(3))
            .await?;
        // Without a known default branch every PR would be flagged as
        // "targets non-default" → Stale, so detection failure disables that check.
        match default_branch.as_deref() {
            Some(b) => tracing::info!(%repo, "default branch: {b}"),
            None => tracing::warn!(
                %repo,
                "could not detect default branch; branch-based stale detection disabled"
            ),
        }
        tracing::info!(%repo, "fetched {} open PRs", raw_prs.len());
        analyzed.extend(analyzer::analyze(
            raw_prs,
            &cfg,
            default_branch.as_deref(),
            now,
        ));
    }

    let author_cache_path = PathBuf::from(history::AUTHOR_CACHE);
    let mut author_cache = history::load_author_cache(&author_cache_path)?;
    let filtered =
        analyzer::apply_grace_period(analyzed, &mut author_cache, cfg.grace_period_days, today);
    tracing::info!("{} PRs survive filters", filtered.len());

    let policies: HashMap<String, policy::Policy> = registry.into_iter().collect();
    let mut scored = scorer::score_prs(filtered, &cfg, &policies, now);

    let mut repos = Vec::with_capacity(repo_names.len());
    for repo in &repo_names {
        let (_, name) = config::repo_parts(repo)?;
        let engine_state_available = match &args.policy_state {
            Some(dir) => match policy::load_engine_state(&dir.join(format!("{name}.json"))) {
                Ok(state) => {
                    scorer::attach_engine_state(&mut scored, repo, &state);
                    true
                }
                Err(e) => {
                    tracing::warn!(%repo, "engine state unavailable: {e:#}");
                    false
                }
            },
            None => {
                tracing::warn!(%repo, "no --policy-state directory; engine state unavailable");
                false
            }
        };
        repos.push(renderer::RepoStatus {
            repo: repo.clone(),
            engine_state_available,
        });
    }

    let history_dir = PathBuf::from(history::HISTORY_DIR);
    let previous = history::load_previous(&history_dir, today)?;
    let has_history = previous.is_some();
    let authors = scorer::rollup_authors(&scored, &cfg, previous.as_ref());
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
        repos: &repos,
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
