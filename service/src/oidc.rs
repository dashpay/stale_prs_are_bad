//! Who may post a snapshot: one workflow of this repository, on its default
//! branch, on a GitHub-hosted runner, on the first attempt of a scheduled or
//! manual run — proven by the OIDC token GitHub mints for that job.
//!
//! A token is checked before anything else happens: its algorithm (RS256 and
//! nothing else), its signature against GitHub's published keys, its issuer
//! and audience exactly, its validity window, its age, and then every claim
//! that names where it ran.

use crate::config::{IngestConfig, GITHUB_ISSUER, GITHUB_JWKS_URL};
use chrono::{DateTime, TimeDelta, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet, KeyAlgorithm, PublicKeyUse};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{PoisonError, RwLock};
use std::time::{Duration, Instant};

/// How far the clocks of GitHub and this service may disagree.
pub const CLOCK_SKEW: Duration = Duration::from_secs(60);

/// The oldest token accepted. GitHub's tokens expire minutes after they are
/// minted and the post job sends its token at once; this holds even if a
/// token's own expiry were ever set far out.
pub const MAX_TOKEN_AGE: Duration = Duration::from_secs(600);

/// An unknown key id triggers a refetch of the key set, but no more than this
/// often: otherwise anyone could make the service hammer GitHub by sending
/// tokens with made-up key ids.
pub const KEY_REFRESH_INTERVAL: Duration = Duration::from_secs(60);

/// The only events whose runs may post: a re-run (`run_attempt` > 1) or any
/// other trigger could run code that is not the current default branch.
const EVENTS: &[&str] = &["schedule", "workflow_dispatch"];

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Where the signing keys come from: GitHub in production, a fixed set in
/// tests.
pub trait KeySource: Send + Sync {
    fn fetch(&self) -> BoxFuture<'_, anyhow::Result<JwkSet>>;
}

/// GitHub's published key set, from its fixed URL.
pub struct GithubKeys {
    client: reqwest::Client,
}

impl GithubKeys {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("pr-hygiene-service")
            .build()?;
        Ok(Self { client })
    }
}

impl KeySource for GithubKeys {
    fn fetch(&self) -> BoxFuture<'_, anyhow::Result<JwkSet>> {
        Box::pin(async move {
            let set = self
                .client
                .get(GITHUB_JWKS_URL)
                .send()
                .await?
                .error_for_status()?
                .json::<JwkSet>()
                .await?;
            Ok(set)
        })
    }
}

#[derive(Default)]
struct Keys {
    by_kid: HashMap<String, DecodingKey>,
    loaded_at: Option<Instant>,
}

/// The signing keys last fetched successfully. A failed or empty fetch
/// keeps them; with none at all, every token is refused.
pub struct KeyCache {
    source: Box<dyn KeySource>,
    keys: RwLock<Keys>,
    last_attempt: tokio::sync::Mutex<Option<Instant>>,
    refresh_interval: Duration,
}

pub enum KeyLookup {
    Found(DecodingKey),
    Unknown,
    /// No keys are held at all: fail closed.
    Unavailable,
}

/// What `/readyz` reports about the keys.
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeysState {
    pub keys: usize,
    pub age_secs: Option<u64>,
}

impl KeyCache {
    pub fn new(source: Box<dyn KeySource>) -> Self {
        Self::with_refresh_interval(source, KEY_REFRESH_INTERVAL)
    }

    /// As `new`, with another floor between fetches (tests use none).
    pub fn with_refresh_interval(source: Box<dyn KeySource>, refresh_interval: Duration) -> Self {
        Self {
            source,
            keys: RwLock::new(Keys::default()),
            last_attempt: tokio::sync::Mutex::new(None),
            refresh_interval,
        }
    }

    pub async fn key(&self, kid: &str) -> KeyLookup {
        if let Some(key) = self.cached(kid) {
            return KeyLookup::Found(key);
        }
        self.refresh().await;
        match self.cached(kid) {
            Some(key) => KeyLookup::Found(key),
            None if self.state().keys == 0 => KeyLookup::Unavailable,
            None => KeyLookup::Unknown,
        }
    }

    /// Fetch the key set unless the last attempt was under a minute ago.
    /// Returns whether a new set was installed.
    pub async fn refresh(&self) -> bool {
        let mut last = self.last_attempt.lock().await;
        if last.is_some_and(|at| at.elapsed() < self.refresh_interval) {
            return false;
        }
        *last = Some(Instant::now());
        match self.source.fetch().await {
            Ok(set) => {
                let by_kid = usable_keys(&set);
                if by_kid.is_empty() {
                    tracing::warn!(
                        "signing key set has no usable RS256 key; keeping the keys held"
                    );
                    return false;
                }
                tracing::info!(keys = by_kid.len(), "signing keys loaded");
                *self.keys.write().unwrap_or_else(PoisonError::into_inner) = Keys {
                    by_kid,
                    loaded_at: Some(Instant::now()),
                };
                true
            }
            Err(e) => {
                tracing::warn!("fetching signing keys failed: {e:#}; keeping the keys held");
                false
            }
        }
    }

    pub fn state(&self) -> KeysState {
        let keys = self.keys.read().unwrap_or_else(PoisonError::into_inner);
        KeysState {
            keys: keys.by_kid.len(),
            age_secs: keys.loaded_at.map(|at| at.elapsed().as_secs()),
        }
    }

    fn cached(&self, kid: &str) -> Option<DecodingKey> {
        let keys = self.keys.read().unwrap_or_else(PoisonError::into_inner);
        keys.by_kid.get(kid).cloned()
    }
}

/// RSA signing keys with an id. Anything else in the set — a symmetric key
/// above all, which would turn a public key into a shared secret — is
/// ignored.
fn usable_keys(set: &JwkSet) -> HashMap<String, DecodingKey> {
    set.keys
        .iter()
        .filter_map(|jwk| {
            let kid = jwk.common.key_id.clone()?;
            if !matches!(jwk.common.key_algorithm, None | Some(KeyAlgorithm::RS256)) {
                return None;
            }
            if !matches!(
                jwk.common.public_key_use,
                None | Some(PublicKeyUse::Signature)
            ) {
                return None;
            }
            let AlgorithmParameters::RSA(rsa) = &jwk.algorithm else {
                return None;
            };
            let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).ok()?;
            Some((kid, key))
        })
        .collect()
}

/// The claims checked. GitHub sends ids and the attempt number as strings;
/// a token that sends anything else does not parse and is refused.
#[derive(Debug, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    iat: i64,
    sha: String,
    repository_id: String,
    repository_owner_id: String,
    #[serde(rename = "ref")]
    git_ref: String,
    ref_type: String,
    workflow_ref: String,
    job_workflow_ref: String,
    event_name: String,
    runner_environment: String,
    run_attempt: String,
}

/// What a verified token vouches for.
#[derive(Debug, Clone)]
pub struct Verified {
    /// The commit the posting run executed.
    pub sha: String,
    pub issued_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// No token, or one that is not a valid GitHub token for this audience.
    #[error("{0}")]
    Unauthenticated(String),
    /// A valid token from somewhere that may not post.
    #[error("{0}")]
    Forbidden(String),
    #[error("GitHub's signing keys are not available")]
    KeysUnavailable,
}

pub struct Verifier {
    keys: std::sync::Arc<KeyCache>,
    validation: Validation,
    audience: String,
    repository_id: String,
    repository_owner_id: String,
    git_ref: String,
    workflow_ref: String,
}

impl Verifier {
    pub fn new(cfg: &IngestConfig, keys: std::sync::Arc<KeyCache>) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = vec![Algorithm::RS256];
        validation.set_issuer(&[GITHUB_ISSUER]);
        validation.set_audience(&[&cfg.audience]);
        validation.leeway = CLOCK_SKEW.as_secs();
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.set_required_spec_claims(&["exp", "nbf", "iss", "aud", "sub"]);
        Self {
            keys,
            validation,
            audience: cfg.audience.clone(),
            repository_id: cfg.repository_id.to_string(),
            repository_owner_id: cfg.repository_owner_id.to_string(),
            git_ref: cfg.git_ref.clone(),
            workflow_ref: cfg.workflow_ref.clone(),
        }
    }

    pub fn keys(&self) -> &KeyCache {
        &self.keys
    }

    pub async fn verify(&self, token: &str) -> Result<Verified, AuthError> {
        let unauthenticated = |why: &str| AuthError::Unauthenticated(format!("token: {why}"));
        // The header is read unverified only to choose the key; the
        // algorithm is pinned before any key is used.
        let header = jsonwebtoken::decode_header(token)
            .map_err(|_| unauthenticated("not a JWT with a supported algorithm"))?;
        if header.alg != Algorithm::RS256 {
            return Err(unauthenticated("must be signed with RS256"));
        }
        let kid = header
            .kid
            .as_deref()
            .ok_or_else(|| unauthenticated("names no signing key"))?;
        let key = match self.keys.key(kid).await {
            KeyLookup::Found(key) => key,
            KeyLookup::Unknown => return Err(unauthenticated("signed with an unknown key")),
            KeyLookup::Unavailable => return Err(AuthError::KeysUnavailable),
        };
        let claims = jsonwebtoken::decode::<Claims>(token, &key, &self.validation)
            .map_err(|e| unauthenticated(rejection(e.kind())))?
            .claims;
        // Exactly, not "one of": an array issuer or audience does not parse
        // into a string at all, and these say so if the library ever relaxed.
        if claims.iss != GITHUB_ISSUER {
            return Err(unauthenticated("wrong issuer"));
        }
        if claims.aud != self.audience {
            return Err(unauthenticated("wrong audience"));
        }
        let issued_at = issued_at(claims.iat, Utc::now())?;
        self.check_origin(&claims)?;
        if !is_commit_sha(&claims.sha) {
            return Err(unauthenticated("`sha` is not a commit id"));
        }
        Ok(Verified {
            sha: claims.sha,
            issued_at,
        })
    }

    /// Where the token was minted: this repository, its default branch, the
    /// one workflow, a GitHub-hosted runner, the first attempt of a scheduled
    /// or manual run.
    fn check_origin(&self, c: &Claims) -> Result<(), AuthError> {
        let checks: [(&str, bool); 9] = [
            ("repository_id", c.repository_id == self.repository_id),
            (
                "repository_owner_id",
                c.repository_owner_id == self.repository_owner_id,
            ),
            ("ref", c.git_ref == self.git_ref),
            ("ref_type", c.ref_type == "branch"),
            ("workflow_ref", c.workflow_ref == self.workflow_ref),
            ("job_workflow_ref", c.job_workflow_ref == self.workflow_ref),
            ("event_name", EVENTS.contains(&c.event_name.as_str())),
            (
                "runner_environment",
                c.runner_environment == "github-hosted",
            ),
            // A re-run executes the commit of the original run, which may no
            // longer be what the default branch says.
            ("run_attempt", c.run_attempt == "1"),
        ];
        match checks.iter().find(|(_, ok)| !ok) {
            Some((claim, _)) => Err(AuthError::Forbidden(format!(
                "token: claim `{claim}` is not allowed to post"
            ))),
            None => Ok(()),
        }
    }
}

/// `iat` must be in the past (within the skew) and recent.
fn issued_at(iat: i64, now: DateTime<Utc>) -> Result<DateTime<Utc>, AuthError> {
    let unauthenticated = |why: &str| Err(AuthError::Unauthenticated(format!("token: {why}")));
    let Some(at) = DateTime::from_timestamp(iat, 0) else {
        return unauthenticated("`iat` is out of range");
    };
    let skew = TimeDelta::from_std(CLOCK_SKEW).expect("a minute fits");
    let max_age = TimeDelta::from_std(MAX_TOKEN_AGE).expect("minutes fit");
    if at > now + skew {
        return unauthenticated("issued in the future");
    }
    if now - at > max_age + skew {
        return unauthenticated("issued too long ago");
    }
    Ok(at)
}

fn rejection(kind: &ErrorKind) -> &'static str {
    match kind {
        ErrorKind::ExpiredSignature => "expired",
        ErrorKind::ImmatureSignature => "not valid yet",
        ErrorKind::InvalidIssuer => "wrong issuer",
        ErrorKind::InvalidAudience => "wrong audience",
        ErrorKind::InvalidSignature => "bad signature",
        ErrorKind::InvalidAlgorithm => "must be signed with RS256",
        ErrorKind::MissingRequiredClaim(_) => "a required claim is missing",
        ErrorKind::Json(_) => "claims are missing or malformed",
        _ => "invalid",
    }
}

pub fn is_commit_sha(s: &str) -> bool {
    (s.len() == 40 || s.len() == 64) && s.bytes().all(|b| b.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_from_the_future_or_too_old_is_refused() {
        let now = Utc::now();
        let ts = |secs: i64| (now + TimeDelta::seconds(secs)).timestamp();
        assert!(issued_at(ts(0), now).is_ok());
        assert!(issued_at(ts(59), now).is_ok(), "within the skew");
        assert!(issued_at(ts(120), now).is_err(), "minted in the future");
        assert!(issued_at(ts(-600), now).is_ok());
        assert!(issued_at(ts(-700), now).is_err(), "older than the cap");
        assert!(issued_at(i64::MAX, now).is_err());
    }

    #[test]
    fn only_rsa_signing_keys_with_an_id_are_kept() {
        let set: JwkSet = serde_json::from_value(serde_json::json!({"keys": [
            {"kty": "oct", "kid": "shared", "k": "c2VjcmV0"},
            {"kty": "RSA", "kid": "enc", "use": "enc", "n": "sXch", "e": "AQAB"},
            {"kty": "RSA", "n": "sXch", "e": "AQAB"},
            {"kty": "RSA", "kid": "rs512", "alg": "RS512", "n": "sXch", "e": "AQAB"},
            {"kty": "RSA", "kid": "good", "alg": "RS256", "use": "sig", "n": "sXch", "e": "AQAB"},
        ]}))
        .unwrap();
        let keys = usable_keys(&set);
        assert_eq!(keys.keys().collect::<Vec<_>>(), vec!["good"]);
    }
}
