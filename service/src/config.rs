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

    /// Directory of the page's files; when set, they are served at `/`.
    #[arg(long, env = "PR_HYGIENE_SITE_DIR")]
    pub site_dir: Option<PathBuf>,

    #[command(flatten)]
    pub ingest: IngestConfig,

    #[command(flatten)]
    pub signin: SignInArgs,
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

/// A credential: never printed, whether by `Debug` or in a log line.
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }

    fn parse(value: &str) -> Result<Self, String> {
        Ok(Self::new(value))
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret(..)")
    }
}

/// "Sign in with GitHub", through a public GitHub App with no permissions.
/// All of it unset turns sign-in off; part of it set is a mistake, and the
/// service refuses to start.
#[derive(Debug, Clone, Args)]
pub struct SignInArgs {
    /// The sign-in App's client id (not its numeric App id).
    #[arg(long, env = "PR_HYGIENE_SIGNIN_CLIENT_ID")]
    pub signin_client_id: Option<String>,

    /// A file holding the sign-in App's client secret, such as a mounted
    /// secret. Preferred to passing the secret itself.
    #[arg(long, env = "PR_HYGIENE_SIGNIN_CLIENT_SECRET_FILE")]
    pub signin_client_secret_file: Option<PathBuf>,

    /// The sign-in App's client secret itself.
    #[arg(
        long,
        env = "PR_HYGIENE_SIGNIN_CLIENT_SECRET",
        hide_env_values = true,
        value_parser = Secret::parse
    )]
    pub signin_client_secret: Option<Secret>,

    /// The origin the page is served from, e.g. `https://hygiene.dash.org`:
    /// GitHub sends people back to `<origin>/auth/callback`, and a request
    /// that changes anything must come from exactly this origin.
    #[arg(long, env = "PR_HYGIENE_PUBLIC_ORIGIN")]
    pub public_origin: Option<String>,
}

/// Sign-in, configured and checked.
#[derive(Debug, Clone)]
pub struct SignInConfig {
    pub client_id: String,
    pub client_secret: Secret,
    /// Scheme, host and port only, as a browser sends it in `Origin`.
    pub origin: String,
}

impl SignInConfig {
    /// The one callback URL: registered with the App, sent with every
    /// sign-in, never taken from a request.
    pub fn redirect_uri(&self) -> String {
        format!("{}/auth/callback", self.origin)
    }
}

impl SignInArgs {
    /// `None` when sign-in is off: nothing of it is configured.
    pub fn resolve(&self) -> anyhow::Result<Option<SignInConfig>> {
        let secret = match (&self.signin_client_secret_file, &self.signin_client_secret) {
            (Some(_), Some(_)) => anyhow::bail!(
                "set PR_HYGIENE_SIGNIN_CLIENT_SECRET_FILE or PR_HYGIENE_SIGNIN_CLIENT_SECRET, not both"
            ),
            (Some(path), None) => {
                let raw = std::fs::read_to_string(path)
                    .map_err(|e| anyhow::anyhow!("reading {}: {e}", path.display()))?;
                Some(Secret::new(raw.trim()))
            }
            (None, Some(secret)) => Some(Secret::new(secret.expose().trim())),
            (None, None) => None,
        };
        let (client_id, secret, origin) =
            match (&self.signin_client_id, secret, &self.public_origin) {
                (None, None, None) => return Ok(None),
                (Some(id), Some(secret), Some(origin)) => (id, secret, origin),
                _ => anyhow::bail!(
                    "sign-in needs PR_HYGIENE_SIGNIN_CLIENT_ID, a client secret and \
                     PR_HYGIENE_PUBLIC_ORIGIN together, or none of them"
                ),
            };
        anyhow::ensure!(
            is_client_id(client_id),
            "PR_HYGIENE_SIGNIN_CLIENT_ID is not a GitHub client id"
        );
        anyhow::ensure!(
            !secret.expose().is_empty(),
            "the sign-in client secret is empty"
        );
        anyhow::ensure!(
            is_origin(origin),
            "PR_HYGIENE_PUBLIC_ORIGIN must be an origin such as https://hygiene.dash.org: \
             https (http only for localhost), a lower-case host, an optional port, \
             no path and no trailing slash"
        );
        Ok(Some(SignInConfig {
            client_id: client_id.clone(),
            client_secret: secret,
            origin: origin.clone(),
        }))
    }
}

/// GitHub's client ids are short and alphanumeric, older ones with a dot
/// (`Iv23li…`, `Iv1.8a61…`); anything else would need escaping in the
/// revocation URL's path. Starting alphanumeric, it can never be a `.` or
/// `..` path segment either.
fn is_client_id(s: &str) -> bool {
    s.len() <= 100
        && s.bytes().next().is_some_and(|b| b.is_ascii_alphanumeric())
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'-')
}

/// An origin exactly as a browser serialises it in `Origin`:
/// `https://host[:port]`, lower case, no default port, no path. Plain `http`
/// only for the loopback host, where browsers still keep `Secure` cookies,
/// so sign-in can be tried locally.
fn is_origin(s: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(s) else {
        return false;
    };
    let secure = match url.scheme() {
        "https" => true,
        "http" => matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]")),
        _ => false,
    };
    secure && url.origin().ascii_serialization() == s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(id: Option<&str>, secret: Option<&str>, origin: Option<&str>) -> SignInArgs {
        SignInArgs {
            signin_client_id: id.map(String::from),
            signin_client_secret_file: None,
            signin_client_secret: secret.map(Secret::new),
            public_origin: origin.map(String::from),
        }
    }

    #[test]
    fn sign_in_is_off_when_nothing_is_set_and_refused_when_half_set() {
        assert!(args(None, None, None).resolve().unwrap().is_none());
        for half in [
            args(Some("Iv23liabc"), None, None),
            args(None, Some("s3cret"), Some("https://hygiene.dash.org")),
            args(Some("Iv23liabc"), Some("s3cret"), None),
        ] {
            assert!(half.resolve().is_err(), "{half:?}");
        }
        let on = args(
            Some("Iv23liabc"),
            Some("s3cret\n"),
            Some("https://hygiene.dash.org"),
        )
        .resolve()
        .unwrap()
        .unwrap();
        assert_eq!(on.client_secret.expose(), "s3cret");
        assert_eq!(on.redirect_uri(), "https://hygiene.dash.org/auth/callback");
    }

    #[test]
    fn the_secret_is_read_from_a_file_and_not_from_both_places() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("secret");
        std::fs::write(&path, "from-file\n").unwrap();
        let mut a = args(Some("Iv23liabc"), None, Some("https://hygiene.dash.org"));
        a.signin_client_secret_file = Some(path);
        assert_eq!(
            a.resolve().unwrap().unwrap().client_secret.expose(),
            "from-file"
        );
        a.signin_client_secret = Some(Secret::new("from-env"));
        assert!(a.resolve().is_err(), "two sources: which one is meant?");
    }

    /// The client id goes into the revocation URL's path unescaped.
    #[test]
    fn a_client_id_cannot_change_the_revocation_path() {
        for good in ["Iv23liABC123", "Iv1.8a61f9b3a7aba766"] {
            assert!(is_client_id(good), "{good}");
        }
        for bad in ["", "..", ".", "a/b", "a?b", "a%2fb", &"x".repeat(101)] {
            assert!(!is_client_id(bad), "{bad}");
        }
    }

    /// The configured origin is compared byte for byte with the browser's
    /// `Origin` header, so it must be written exactly as browsers write it.
    #[test]
    fn the_origin_must_be_written_as_a_browser_sends_it() {
        for good in [
            "https://hygiene.dash.org",
            "https://hygiene.dash.org:8443",
            "http://localhost:8080",
            "http://127.0.0.1:8080",
        ] {
            assert!(is_origin(good), "{good}");
        }
        for bad in [
            "https://hygiene.dash.org/",
            "https://hygiene.dash.org/app",
            "https://Hygiene.dash.org",
            "https://hygiene.dash.org:443",
            "http://hygiene.dash.org",
            "hygiene.dash.org",
            "https://user@hygiene.dash.org",
            "",
        ] {
            assert!(!is_origin(bad), "{bad}");
        }
    }

    #[test]
    fn the_secret_is_never_printed() {
        let cfg = args(
            Some("Iv23liabc"),
            Some("s3cret"),
            Some("https://hygiene.dash.org"),
        );
        assert!(!format!("{cfg:?}").contains("s3cret"));
        let resolved = cfg.resolve().unwrap().unwrap();
        assert!(!format!("{resolved:?}").contains("s3cret"));
    }
}
