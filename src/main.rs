use pr_hygiene::{analyzer, config, dashboard, fetcher, history, policy, renderer, scorer, stages};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
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
    /// (`pr_review.main report --format json`). Missing files render as unavailable.
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
    /// Also write the interactive dashboard's data (JSON) here. Written even
    /// with --dry-run: it is an output asked for by name, not history.
    #[arg(long)]
    json_out: Option<PathBuf>,

    /// With --json-out, also list the PRs merged or closed in this many
    /// days before the run (365 for a one-time year's backfill). A year's
    /// list is larger than the service accepts in a post (1.7 MB measured
    /// for five repositories); import that file into the service instead.
    #[arg(long, default_value_t = 30, value_parser = clap::value_parser!(u32).range(1..=3650))]
    closed_days: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("pr_hygiene=info,info")),
        )
        .with_target(false)
        // Keep stdout for the report so `--dry-run > file` captures only it.
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();
    let cfg = config::Config::load_or_default(&args.config)?;
    dashboard::validate_lateness(&cfg)?;
    // Bots are not people: no account is folded into another's rows.
    if !cfg.author_aliases.is_empty() {
        anyhow::bail!(
            "{}: author_aliases is no longer supported — bots keep their own rows",
            args.config.display()
        );
    }
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
    // One repository failing to fetch must not blank the whole board: it is
    // rendered as unavailable and the run still exits non-zero at the end.
    let mut fetch_errors: HashMap<String, String> = HashMap::new();
    // Only the dashboard data times stages and lists closed PRs; the report
    // has no use for either.
    let mut evidence: HashMap<String, stages::RepoEvidence> = HashMap::new();
    let mut facts: HashMap<String, HashMap<u64, dashboard::PrFacts>> = HashMap::new();
    let mut closed: HashMap<String, dashboard::ClosedRead> = HashMap::new();
    let closed_since = args
        .json_out
        .is_some()
        .then(|| now - chrono::Duration::days(args.closed_days.into()));
    for repo in &repo_names {
        match fetch_repo(&fetcher, repo, args.json_out.is_some(), closed_since).await {
            Ok(fetched) => {
                evidence.extend(fetched.evidence.map(|e| (repo.clone(), e)));
                closed.extend(fetched.closed.map(|c| (repo.clone(), c)));
                facts.insert(repo.clone(), fetched.facts);
                analyzed.extend(analyzer::analyze(
                    fetched.raw_prs,
                    &cfg,
                    fetched.default_branch.as_deref(),
                    now,
                ));
            }
            Err(e) => {
                tracing::error!(%repo, "fetch failed: {e:#}");
                fetch_errors.insert(repo.clone(), format!("{e:#}"));
            }
        }
    }
    tracing::info!("GitHub GraphQL: {} points", fetcher.points());

    let author_cache_path = PathBuf::from(history::AUTHOR_CACHE);
    let mut author_cache = history::load_author_cache(&author_cache_path)?;
    let filtered =
        analyzer::apply_grace_period(analyzed, &mut author_cache, cfg.grace_period_days, today);
    tracing::info!("{} PRs survive filters", filtered.len());

    let engine_states = policy::load_engine_states(args.policy_state.as_deref(), &registry)?;
    let policies: HashMap<String, policy::Policy> = registry.into_iter().collect();
    let mut scored = scorer::score_prs(filtered, &cfg, &policies, now);
    for (repo, state) in &engine_states {
        scorer::attach_engine_state(&mut scored, repo, state);
    }
    let repos: Vec<renderer::RepoStatus> = repo_names
        .iter()
        .map(|repo| renderer::RepoStatus {
            repo: repo.clone(),
            engine_state_available: engine_states.contains_key(repo),
            fetch_error: fetch_errors.get(repo).cloned(),
        })
        .collect();

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

    if let Some(path) = &args.json_out {
        let board = dashboard::build(&dashboard::Inputs {
            scored: &scored,
            engine: &engine_states,
            evidence: &evidence,
            facts: &facts,
            closed: &closed,
            policies: &policies,
            repos: &repos,
            cfg: &cfg,
            now,
            commit: args.commit_sha.as_deref(),
        });
        let json = serde_json::to_string(&board).context("serializing dashboard data")?;
        write_if_changed(path, &json)?;
        tracing::info!(
            "dashboard data: {} PRs, {} people, {} closed PRs → {}",
            board.prs.len(),
            board.people.len(),
            board.closed.len(),
            path.display()
        );
    }

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
        if fetch_errors.is_empty() {
            history::write_snapshot(&history_dir, &snapshot)?;
            history::save_author_cache(&author_cache_path, &author_cache)?;
            let pruned = history::prune(&history_dir, today, cfg.history_retention_days)?;
            if pruned > 0 {
                tracing::info!("pruned {pruned} old snapshot(s)");
            }
        } else {
            // A partial snapshot would make next week's delta read as an improvement.
            tracing::warn!("history not updated: at least one repository could not be fetched");
        }
    }

    if !fetch_errors.is_empty() {
        let mut failed: Vec<&String> = fetch_errors.keys().collect();
        failed.sort();
        anyhow::bail!(
            "report written, but {} repositor{} could not be fetched: {}",
            failed.len(),
            if failed.len() == 1 { "y" } else { "ies" },
            failed
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

/// One repository's open PRs, its default branch, what GitHub attaches to
/// each open PR and, when asked for, what GitHub and the engine record about
/// each PR's stage changes and the PRs closed since a time.
struct Fetched {
    raw_prs: Vec<pr_hygiene::model::RawPr>,
    default_branch: Option<String>,
    evidence: Option<stages::RepoEvidence>,
    facts: HashMap<u64, dashboard::PrFacts>,
    closed: Option<dashboard::ClosedRead>,
}

async fn fetch_repo(
    fetcher: &fetcher::Fetcher,
    repo: &str,
    with_evidence: bool,
    closed_since: Option<DateTime<Utc>>,
) -> Result<Fetched> {
    let (owner, name) = config::repo_parts(repo)?;
    let (mut raw_prs, node_ids, default_branch, facts) =
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
    // Every open PR, those the board's own filters drop included: the
    // engine's queue still shows them. A PR without evidence shows its age,
    // which is no reason to fail the repository; the page says which.
    let evidence = if with_evidence {
        let evidence = fetcher.fetch_stage_evidence(&node_ids).await;
        if let Some(error) = &evidence.error {
            tracing::warn!(%repo, "stage entry times incomplete: {error}");
        }
        Some(evidence)
    } else {
        None
    };
    // Read after the open PRs and only when they were: a repository that
    // could not be read contributes nothing. Failing here costs the closed
    // list alone, which the page is told.
    let mut closed = None;
    if let Some(since) = closed_since {
        let points = fetcher.points();
        let read = fetcher.fetch_closed_prs(owner, name, since).await;
        let points = fetcher.points() - points;
        match &read {
            Ok(prs) => tracing::info!(
                %repo,
                points,
                "{} PRs closed since {since}",
                prs.len()
            ),
            Err(error) => tracing::warn!(%repo, points, "closed PRs unread: {error}"),
        }
        closed = Some(read);
    }
    Ok(Fetched {
        raw_prs,
        default_branch,
        evidence,
        facts,
        closed,
    })
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
