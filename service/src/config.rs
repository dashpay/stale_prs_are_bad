//! Configuration, read from the environment (or the matching flags).

use clap::Args;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

/// The only issuer whose tokens are accepted: GitHub Actions' OIDC provider.
pub const GITHUB_ISSUER: &str = "https://token.actions.githubusercontent.com";

/// Where GitHub publishes the keys that sign those tokens. Fixed, not
/// configurable: whoever could point it elsewhere could mint their own tokens.
pub const GITHUB_JWKS_URL: &str = "https://token.actions.githubusercontent.com/.well-known/jwks";

#[derive(Debug, Clone, Args)]
pub struct ServeConfig {
    /// Address the HTTP server listens on.
    #[arg(long, env = "PR_HYGIENE_BIND", default_value = "0.0.0.0:8080")]
    pub bind: SocketAddr,

    /// SQLite database file; keep it on a local volume (WAL needs real file locks).
    #[arg(
        long,
        env = "PR_HYGIENE_DB",
        default_value = "/data/pr-hygiene.sqlite3"
    )]
    pub db: PathBuf,

    #[command(flatten)]
    pub ingest: IngestConfig,
}

/// What a token and the snapshot it carries must be to be ingested.
#[derive(Debug, Clone, Args)]
pub struct IngestConfig {
    /// The audience the post job asks GitHub for: the service's own URL.
    #[arg(long, env = "PR_HYGIENE_OIDC_AUDIENCE")]
    pub audience: String,

    /// Numeric id of the repository whose workflow may post. Ids, unlike
    /// names, cannot be re-registered by someone else.
    #[arg(
        long,
        env = "PR_HYGIENE_REPOSITORY_ID",
        default_value_t = 1_242_761_300
    )]
    pub repository_id: u64,

    /// Numeric id of that repository's owner.
    #[arg(
        long,
        env = "PR_HYGIENE_REPOSITORY_OWNER_ID",
        default_value_t = 11_511_719
    )]
    pub repository_owner_id: u64,

    /// The only branch a posting run may run on.
    #[arg(
        long = "ref",
        env = "PR_HYGIENE_REF",
        default_value = "refs/heads/master"
    )]
    pub git_ref: String,

    /// The only workflow whose run may post, at that branch: the scheduled
    /// workflow that calls the post workflow.
    #[arg(
        long,
        env = "PR_HYGIENE_WORKFLOW_REF",
        default_value = "dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene.yml@refs/heads/master"
    )]
    pub workflow_ref: String,

    /// The only code that may hold a posting token: the reusable post
    /// workflow. Every job of the calling workflow shares its other claims —
    /// including the Pages job, which needs `id-token: write` and runs
    /// third-party actions — but only the post workflow's job carries this.
    #[arg(
        long,
        env = "PR_HYGIENE_JOB_WORKFLOW_REF",
        default_value = "dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene-post.yml@refs/heads/master"
    )]
    pub job_workflow_ref: String,

    /// Largest snapshot accepted, in bytes. A five-repository snapshot with
    /// 139 open PRs measured 109 KB; the default leaves ample headroom.
    #[arg(long, env = "PR_HYGIENE_BODY_LIMIT", default_value_t = 1024 * 1024)]
    pub body_limit: usize,

    /// How long before the token was minted the snapshot may have been
    /// generated: the analyze job's timeout plus the post job's start-up.
    /// One minute to one day.
    #[arg(
        long,
        env = "PR_HYGIENE_JOB_TIMEOUT_SECS",
        default_value_t = 1800,
        value_parser = clap::value_parser!(u64).range(60..=86_400)
    )]
    pub job_timeout_secs: u64,
}

impl IngestConfig {
    pub fn job_timeout(&self) -> Duration {
        Duration::from_secs(self.job_timeout_secs)
    }
}
