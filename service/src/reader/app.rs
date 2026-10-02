//! The reader App's credentials: a short JWT signed with the App's private
//! key, traded for an installation token that can only read.
//!
//! The private key is held in memory only, inside [`AppKey`], and neither
//! it, the JWT nor the installation token is ever printed: every `Debug`
//! here redacts them, and no error carries them.

use super::{github_client, transport::is_transient, API_URL, API_VERSION};
use crate::config::{ReaderConfig, Secret};
use crate::oidc::BoxFuture;
use chrono::{DateTime, TimeDelta, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

/// What every installation token is asked for: reading what the engine
/// reads, and nothing else. GitHub narrows the token to these, whatever
/// else the App was granted.
///
/// - `pull_requests`: pull requests, their files and reviews.
/// - `issues`: comments, timelines and labels.
/// - `statuses`: commit statuses.
/// - `checks`: the checks on a head.
/// - `actions`: the workflow a check run belongs to: the build query reads
///   `checkSuite { workflowRun { workflow } }`, and without this GraphQL
///   answers that field with an error, which fails the build state.
/// - `contents`: the head commit, which the build query reads its checks
///   through.
/// - `metadata`: the collaborator listing and each person's permission.
pub const PERMISSIONS: [&str; 7] = [
    "actions",
    "checks",
    "contents",
    "issues",
    "metadata",
    "pull_requests",
    "statuses",
];

/// The only access level asked for, and the only one accepted back.
const READ: &str = "read";

/// A held token is renewed once it has less than this left. Tokens live an
/// hour; a token handed out must outlast the request it goes on, a page of
/// a long listing included.
pub const REFRESH_MARGIN: TimeDelta = TimeDelta::minutes(5);

/// How far into the past the App's JWT is dated, against the clocks of
/// GitHub and this service disagreeing, as GitHub advises.
const JWT_BACKDATE: TimeDelta = TimeDelta::seconds(60);

/// How long after now the App's JWT expires. Dated back a minute, it is
/// valid for ten minutes in all, the most GitHub accepts.
const JWT_LIFETIME: TimeDelta = TimeDelta::seconds(540);

/// Per call to GitHub's token route.
const CALL_TIMEOUT: Duration = Duration::from_secs(15);

/// A token answer is a few hundred bytes; a far larger one is not read.
pub const MAX_ANSWER_BYTES: usize = 64 * 1024;

/// The App's private key, held in memory only.
#[derive(Clone)]
pub struct AppKey(EncodingKey);

/// Why a private key was not accepted. It says nothing of the key itself.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("not an RSA private key of at least 2048 bits in PEM form")]
pub struct KeyError;

impl AppKey {
    /// The key from its PEM text: PKCS#1 (`BEGIN RSA PRIVATE KEY`), the
    /// form GitHub issues App keys in, or PKCS#8 (`BEGIN PRIVATE KEY`).
    pub fn from_pem(pem: &str) -> Result<Self, KeyError> {
        use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
        use rsa::pkcs8::DecodePrivateKey;
        use rsa::traits::PublicKeyParts;
        let key = rsa::RsaPrivateKey::from_pkcs1_pem(pem)
            .or_else(|_| rsa::RsaPrivateKey::from_pkcs8_pem(pem))
            .map_err(|_| KeyError)?;
        if key.size() * 8 < 2048 {
            return Err(KeyError);
        }
        let der = key.to_pkcs1_der().map_err(|_| KeyError)?;
        Ok(AppKey(EncodingKey::from_rsa_der(der.as_bytes())))
    }
}

impl fmt::Debug for AppKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AppKey(..)")
    }
}

/// Why no installation token could be had. Nothing here quotes a key, a
/// token or GitHub's free text, so it can be logged as it is.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AppAuthError {
    #[error("could not sign the App's JWT")]
    Signing,
    #[error("GitHub answered the token request with HTTP {0}")]
    Status(u16),
    #[error("unexpected token answer from GitHub: {0}")]
    Malformed(&'static str),
    #[error("GitHub's token answer is larger than {MAX_ANSWER_BYTES} bytes")]
    TooLarge,
    /// GitHub granted a permission beyond reading. The token is dropped
    /// unused.
    #[error("the installation token grants more than reading: {0}")]
    NotReadOnly(String),
    #[error("could not reach GitHub: {detail}")]
    Transport { detail: String, transient: bool },
}

impl AppAuthError {
    /// Whether asking again could plausibly succeed: a gateway error or a
    /// connection that failed on the way.
    pub fn transient(&self) -> bool {
        match self {
            AppAuthError::Status(status) => matches!(status, 502..=504),
            AppAuthError::Transport { transient, .. } => *transient,
            _ => false,
        }
    }
}

/// Where a request's token comes from.
pub trait TokenSource: Send + Sync {
    /// A token valid for at least the next request.
    fn token(&self) -> BoxFuture<'_, Result<Secret, AppAuthError>>;
}

/// The App's JWT claims: issued a minute ago, by the App, for ten minutes.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Claims {
    iat: i64,
    exp: i64,
    iss: String,
}

struct Held {
    token: Secret,
    expires_at: DateTime<Utc>,
}

/// The reader App's installation token: asked for when first needed, then
/// reused until it has less than [`REFRESH_MARGIN`] left.
pub struct InstallationTokens {
    client: reqwest::Client,
    origin: String,
    app_id: u64,
    installation_id: u64,
    key: AppKey,
    /// Held across the exchange, so that callers waiting on a renewal share
    /// its token rather than each asking for one.
    held: tokio::sync::Mutex<Option<Held>>,
}

impl fmt::Debug for InstallationTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstallationTokens")
            .field("origin", &self.origin)
            .field("app_id", &self.app_id)
            .field("installation_id", &self.installation_id)
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

impl InstallationTokens {
    pub fn new(config: &ReaderConfig) -> anyhow::Result<Self> {
        Self::at(config, API_URL)
    }

    /// Another origin, for testing the exchange itself; production code has
    /// no way to send the App's JWT anywhere but GitHub.
    #[cfg(test)]
    pub(crate) fn with_origin(config: &ReaderConfig, origin: &str) -> anyhow::Result<Self> {
        Self::at(config, origin)
    }

    fn at(config: &ReaderConfig, origin: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: github_client(CALL_TIMEOUT)?,
            origin: origin.to_owned(),
            app_id: config.app_id,
            installation_id: config.installation_id,
            key: config.key.clone(),
            held: tokio::sync::Mutex::new(None),
        })
    }

    /// The App's own JWT at `now`: RS256, issued by the App, dated a minute
    /// back and expiring nine minutes on.
    fn app_jwt(&self, now: DateTime<Utc>) -> Result<Secret, AppAuthError> {
        let claims = Claims {
            iat: (now - JWT_BACKDATE).timestamp(),
            exp: (now + JWT_LIFETIME).timestamp(),
            iss: self.app_id.to_string(),
        };
        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &self.key.0)
            .map(Secret::new)
            .map_err(|_| AppAuthError::Signing)
    }

    async fn current(&self) -> Result<Secret, AppAuthError> {
        let mut held = self.held.lock().await;
        if let Some(held) = held
            .as_ref()
            .filter(|held| held.expires_at - Utc::now() > REFRESH_MARGIN)
        {
            return Ok(held.token.clone());
        }
        let fresh = self.exchange().await?;
        let token = fresh.token.clone();
        tracing::info!(expires_at = %fresh.expires_at, "reader App installation token renewed");
        *held = Some(fresh);
        Ok(token)
    }

    /// `POST /app/installations/{id}/access_tokens`, asking for
    /// [`PERMISSIONS`], each `read`.
    async fn exchange(&self) -> Result<Held, AppAuthError> {
        let jwt = self.app_jwt(Utc::now())?;
        let mut authorization = HeaderValue::from_str(&format!("Bearer {}", jwt.expose()))
            .map_err(|_| AppAuthError::Signing)?;
        authorization.set_sensitive(true);
        let permissions: serde_json::Map<String, Value> = PERMISSIONS
            .iter()
            .map(|name| ((*name).to_owned(), Value::from(READ)))
            .collect();
        let mut res = self
            .client
            .post(format!(
                "{}/app/installations/{}/access_tokens",
                self.origin, self.installation_id
            ))
            .header(AUTHORIZATION, authorization)
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", API_VERSION)
            .json(&json!({ "permissions": permissions }))
            .send()
            .await
            .map_err(transport)?;
        if !res.status().is_success() {
            return Err(AppAuthError::Status(res.status().as_u16()));
        }
        if res
            .content_length()
            .is_some_and(|n| n > MAX_ANSWER_BYTES as u64)
        {
            return Err(AppAuthError::TooLarge);
        }
        let mut body = Vec::new();
        while let Some(chunk) = res.chunk().await.map_err(transport)? {
            if body.len() + chunk.len() > MAX_ANSWER_BYTES {
                return Err(AppAuthError::TooLarge);
            }
            body.extend_from_slice(&chunk);
        }
        #[derive(Deserialize)]
        struct Answer {
            token: String,
            expires_at: String,
            permissions: BTreeMap<String, Value>,
        }
        let answer: Answer = serde_json::from_slice(&body)
            .map_err(|_| AppAuthError::Malformed("no token, expiry and permissions"))?;
        if let Some((name, level)) = answer
            .permissions
            .iter()
            .find(|(_, level)| level.as_str() != Some(READ))
        {
            return Err(AppAuthError::NotReadOnly(format!(
                "{}: {}",
                plain(name),
                level.as_str().map_or("unrecognised".into(), plain)
            )));
        }
        if answer.token.is_empty() {
            return Err(AppAuthError::Malformed("an empty token"));
        }
        let expires_at = DateTime::parse_from_rfc3339(&answer.expires_at)
            .map_err(|_| AppAuthError::Malformed("an expiry that is not a time"))?
            .with_timezone(&Utc);
        if expires_at <= Utc::now() {
            return Err(AppAuthError::Malformed("a token that has already expired"));
        }
        Ok(Held {
            token: Secret::new(answer.token),
            expires_at,
        })
    }
}

impl TokenSource for InstallationTokens {
    fn token(&self) -> BoxFuture<'_, Result<Secret, AppAuthError>> {
        Box::pin(self.current())
    }
}

/// The error without its URL or anything else a request carried.
fn transport(error: reqwest::Error) -> AppAuthError {
    AppAuthError::Transport {
        transient: is_transient(&error),
        detail: super::transport::describe(error),
    }
}

/// GitHub's permission names and levels are short snake_case words;
/// anything else is not repeated, not even in a log.
fn plain(s: &str) -> String {
    let plain =
        (1..=64).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_lowercase() || b == b'_');
    if plain {
        s.to_owned()
    } else {
        "unrecognised".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::mock::{self, test_key, Seen};
    use axum::http::{Method, StatusCode};
    use axum::response::{IntoResponse, Response};
    use jsonwebtoken::{DecodingKey, Validation};
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn config() -> ReaderConfig {
        ReaderConfig {
            app_id: 4242,
            installation_id: 777,
            key: AppKey::from_pem(&test_key().pem).unwrap(),
        }
    }

    /// GitHub's answer to a token request: a token and when it expires.
    fn token_answer(token: &str, expires_in: TimeDelta, permissions: Value) -> Response {
        let expires_at =
            (Utc::now() + expires_in).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        (
            StatusCode::CREATED,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            json!({"token": token, "expires_at": expires_at, "permissions": permissions,
                   "repository_selection": "selected"})
            .to_string(),
        )
            .into_response()
    }

    fn read_only() -> Value {
        Value::Object(
            PERMISSIONS
                .iter()
                .map(|name| ((*name).to_owned(), Value::from("read")))
                .collect(),
        )
    }

    #[test]
    fn the_jwt_is_rs256_issued_by_the_app_dated_back_and_short_lived() {
        let tokens = InstallationTokens::with_origin(&config(), "http://127.0.0.1:9").unwrap();
        let now = Utc::now();
        let jwt = tokens.app_jwt(now).unwrap();
        let header = jsonwebtoken::decode_header(jwt.expose()).unwrap();
        assert_eq!(header.alg, Algorithm::RS256);
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_required_spec_claims(&["exp", "iat", "iss"]);
        validation.set_issuer(&["4242"]);
        let claims = jsonwebtoken::decode::<Claims>(
            jwt.expose(),
            &DecodingKey::from_rsa_der(&test_key().public_der),
            &validation,
        )
        .expect("signed by the App's key")
        .claims;
        assert_eq!(claims.iat, now.timestamp() - 60, "a minute back");
        assert!(claims.exp > now.timestamp(), "valid now");
        assert!(
            claims.exp - claims.iat <= 600,
            "GitHub refuses a JWT valid for more than ten minutes"
        );
    }

    #[tokio::test]
    async fn the_token_is_asked_for_read_only_and_reused_while_it_lasts() {
        let github =
            mock::serve(|_seen: &Seen| token_answer("ghs_first", TimeDelta::hours(1), read_only()))
                .await;
        let tokens = InstallationTokens::with_origin(&config(), &github.url).unwrap();
        assert_eq!(tokens.token().await.unwrap().expose(), "ghs_first");
        assert_eq!(tokens.token().await.unwrap().expose(), "ghs_first");
        let seen = github.seen();
        assert_eq!(seen.len(), 1, "one exchange for two requests");
        let request = &seen[0];
        assert_eq!(request.method, Method::POST);
        assert_eq!(
            request.path_and_query,
            "/app/installations/777/access_tokens"
        );
        assert_eq!(request.headers["x-github-api-version"], API_VERSION);
        let bearer = request.headers["authorization"].to_str().unwrap();
        let jwt = bearer
            .strip_prefix("Bearer ")
            .expect("the JWT as a bearer token");
        assert_eq!(
            jsonwebtoken::decode_header(jwt).unwrap().alg,
            Algorithm::RS256
        );
        let body: Value = serde_json::from_str(&request.body).unwrap();
        assert_eq!(
            body,
            json!({"permissions": {
                "actions": "read", "checks": "read", "contents": "read", "issues": "read",
                "metadata": "read", "pull_requests": "read", "statuses": "read"}}),
            "every permission asked for explicitly, and only to read"
        );
    }

    /// A token within the margin of its expiry could lapse during the
    /// request it is handed out for; a new one is asked for instead.
    #[tokio::test]
    async fn a_token_near_its_expiry_is_renewed() {
        let count = Arc::new(AtomicUsize::new(0));
        let counter = count.clone();
        let github =
            mock::serve(
                move |_seen: &Seen| match counter.fetch_add(1, Ordering::SeqCst) {
                    0 => token_answer("ghs_short", TimeDelta::minutes(4), read_only()),
                    _ => token_answer("ghs_long", TimeDelta::hours(1), read_only()),
                },
            )
            .await;
        let tokens = InstallationTokens::with_origin(&config(), &github.url).unwrap();
        assert_eq!(tokens.token().await.unwrap().expose(), "ghs_short");
        assert_eq!(tokens.token().await.unwrap().expose(), "ghs_long");
        assert_eq!(tokens.token().await.unwrap().expose(), "ghs_long");
        assert_eq!(github.seen().len(), 2);
    }

    #[tokio::test]
    async fn a_token_that_could_write_is_refused() {
        let github = mock::serve(|_seen: &Seen| {
            let mut permissions = read_only();
            permissions["pull_requests"] = Value::from("write");
            token_answer("ghs_writer", TimeDelta::hours(1), permissions)
        })
        .await;
        let tokens = InstallationTokens::with_origin(&config(), &github.url).unwrap();
        let error = tokens.token().await.unwrap_err();
        assert_eq!(
            error,
            AppAuthError::NotReadOnly("pull_requests: write".into())
        );
        assert!(!error.to_string().contains("ghs_writer"));
        // Not held: the next request asks again.
        let _ = tokens.token().await;
        assert_eq!(github.seen().len(), 2);
    }

    #[tokio::test]
    async fn a_refused_exchange_is_an_error_and_a_gateway_error_is_transient() {
        let github =
            mock::serve(|_seen: &Seen| StatusCode::UNPROCESSABLE_ENTITY.into_response()).await;
        let tokens = InstallationTokens::with_origin(&config(), &github.url).unwrap();
        let error = tokens.token().await.unwrap_err();
        assert_eq!(error, AppAuthError::Status(422));
        assert!(!error.transient());
        assert!(AppAuthError::Status(502).transient());
    }

    /// The key, the JWT and the token stay out of every `Debug` and every
    /// error, whatever goes wrong.
    #[tokio::test]
    async fn the_key_and_tokens_are_never_printed() {
        let key = test_key();
        let der = key.private.to_pkcs1_der().unwrap();
        // A stretch of the key as any form of it would print it.
        let pem_body: String = key.pem.lines().nth(5).unwrap().to_owned();
        let github = mock::serve(|_seen: &Seen| {
            token_answer("ghs_secret_token", TimeDelta::hours(1), read_only())
        })
        .await;
        let config = config();
        let tokens = InstallationTokens::with_origin(&config, &github.url).unwrap();
        let token = tokens.token().await.unwrap();
        let jwt = github.seen()[0].headers["authorization"]
            .to_str()
            .unwrap()
            .to_owned();
        let printed = [
            format!("{config:?}"),
            format!("{:?}", config.key),
            format!("{tokens:?}"),
            format!("{token:?}"),
            format!("{:?}", AppKey::from_pem("not a key").unwrap_err()),
            AppKey::from_pem(&key.pem.replace("MII", "XII"))
                .unwrap_err()
                .to_string(),
        ];
        for text in &printed {
            assert!(!text.contains(&pem_body), "key in {text}");
            assert!(!text.contains("ghs_secret_token"), "token in {text}");
            assert!(!text.contains(&jwt[7..40]), "JWT in {text}");
            assert!(
                !text
                    .as_bytes()
                    .windows(16)
                    .any(|w| der.as_bytes().windows(16).any(|d| d == w)),
                "key bytes in {text}"
            );
        }
        // A failed exchange quotes neither the JWT nor the key.
        let down = InstallationTokens::with_origin(&config, "http://127.0.0.1:9").unwrap();
        let error = down.token().await.unwrap_err();
        let text = format!("{error} {error:?}");
        assert!(
            !text.contains(&pem_body) && !text.contains("Bearer"),
            "{text}"
        );
    }

    #[test]
    fn a_key_that_is_not_an_rsa_private_key_is_refused() {
        assert_eq!(AppKey::from_pem("").unwrap_err(), KeyError);
        let garbled = "-----BEGIN RSA PRIVATE KEY-----\nAAAA\n-----END RSA PRIVATE KEY-----\n";
        assert_eq!(AppKey::from_pem(garbled).unwrap_err(), KeyError);
        // The PKCS#8 form of the same key is accepted.
        use rsa::pkcs8::EncodePrivateKey;
        let pkcs8 = test_key()
            .private
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap();
        assert!(AppKey::from_pem(&pkcs8).is_ok());
    }
}
