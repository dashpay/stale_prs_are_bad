//! A service wired as in production but for its key source: an RSA key made
//! for the test run stands in for GitHub's, and tokens are signed with it.
#![allow(dead_code)]

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::response::Response;
use axum::Router;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, TimeDelta, Utc};
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use pr_hygiene::dashboard::Dashboard;
use pr_hygiene_service::app::{self, AppState};
use pr_hygiene_service::config::IngestConfig;
use pr_hygiene_service::oidc::{BoxFuture, KeyCache, KeySource, KEY_REFRESH_INTERVAL};
use pr_hygiene_service::store::{Reader, Store};
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::traits::PublicKeyParts;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tower::ServiceExt;

pub const AUDIENCE: &str = "https://pr-hygiene.example.org";
pub const KID: &str = "test-key";
pub const SHA: &str = "0123456789abcdef0123456789abcdef01234567";
pub const WORKFLOW: &str =
    "dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene.yml@refs/heads/master";
pub const PLATFORM: &str = "dashpay/platform";
pub const DASHCORE: &str = "dashpay/rust-dashcore";

struct TestKey {
    encoding: EncodingKey,
    jwks: Value,
}

/// One 2048-bit key per test binary: generating it is the slow part.
fn key() -> &'static TestKey {
    static KEY: OnceLock<TestKey> = OnceLock::new();
    KEY.get_or_init(|| {
        let private = rsa::RsaPrivateKey::new(&mut rand::thread_rng(), 2048).unwrap();
        let der = private.to_pkcs1_der().unwrap();
        let b64 = |n: &rsa::BigUint| URL_SAFE_NO_PAD.encode(n.to_bytes_be());
        TestKey {
            encoding: EncodingKey::from_rsa_der(der.as_bytes()),
            jwks: json!({"keys": [{
                "kty": "RSA", "alg": "RS256", "use": "sig", "kid": KID,
                "n": b64(private.n()), "e": b64(private.e()),
            }]}),
        }
    })
}

/// The test key set, counting fetches; it can be made to fail.
pub struct TestKeys {
    pub fetches: AtomicUsize,
    pub failing: AtomicBool,
}

struct Source(Arc<TestKeys>);

impl KeySource for Source {
    fn fetch(&self) -> BoxFuture<'_, anyhow::Result<JwkSet>> {
        Box::pin(async move {
            self.0.fetches.fetch_add(1, Ordering::SeqCst);
            anyhow::ensure!(!self.0.failing.load(Ordering::SeqCst), "GitHub is down");
            Ok(serde_json::from_value(key().jwks.clone())?)
        })
    }
}

pub struct TestApp {
    pub router: Router,
    pub keys: Arc<TestKeys>,
    pub db: std::path::PathBuf,
    _dir: tempfile::TempDir,
}

pub fn config() -> IngestConfig {
    IngestConfig {
        audience: AUDIENCE.into(),
        repository_id: 1_242_761_300,
        repository_owner_id: 11_511_719,
        git_ref: "refs/heads/master".into(),
        workflow_ref: WORKFLOW.into(),
        body_limit: 1024 * 1024,
        job_timeout_secs: 1800,
    }
}

pub fn app() -> TestApp {
    app_with(config(), false, KEY_REFRESH_INTERVAL)
}

/// `keys_down`: GitHub's key set cannot be fetched from the start.
/// `refresh`: the floor between key set fetches.
pub fn app_with(cfg: IngestConfig, keys_down: bool, refresh: Duration) -> TestApp {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("service.sqlite3");
    let store = Store::open(&db).unwrap();
    let reader = Reader::open(&db).unwrap();
    let keys = Arc::new(TestKeys {
        fetches: AtomicUsize::new(0),
        failing: AtomicBool::new(keys_down),
    });
    let cache = Arc::new(KeyCache::with_refresh_interval(
        Box::new(Source(keys.clone())),
        refresh,
    ));
    let state = Arc::new(AppState::new(cfg, cache, store, reader));
    TestApp {
        router: app::router(state),
        keys,
        db,
        _dir: dir,
    }
}

pub fn now() -> i64 {
    Utc::now().timestamp()
}

/// GitHub gives every token its own `jti`.
static NEXT_JTI: AtomicUsize = AtomicUsize::new(1);

/// The claims GitHub puts in a token for the scheduled run on master.
pub fn claims() -> Value {
    let now = now();
    json!({
        "iss": "https://token.actions.githubusercontent.com",
        "aud": AUDIENCE,
        "sub": "repo:dashpay/stale_prs_are_bad:ref:refs/heads/master",
        "iat": now,
        "nbf": now - 5,
        "exp": now + 300,
        "jti": format!("token-{}", NEXT_JTI.fetch_add(1, Ordering::SeqCst)),
        "sha": SHA,
        "repository": "dashpay/stale_prs_are_bad",
        "repository_id": "1242761300",
        "repository_owner": "dashpay",
        "repository_owner_id": "11511719",
        "ref": "refs/heads/master",
        "ref_type": "branch",
        "workflow": "PR Hygiene",
        "workflow_ref": WORKFLOW,
        "job_workflow_ref": WORKFLOW,
        "event_name": "schedule",
        "runner_environment": "github-hosted",
        "run_id": "123",
        "run_attempt": "1",
    })
}

pub fn with(mut claims: Value, field: &str, value: Value) -> Value {
    claims[field] = value;
    claims
}

pub fn sign(claims: &Value) -> String {
    sign_with_kid(claims, KID)
}

pub fn sign_with_kid(claims: &Value, kid: &str) -> String {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(kid.into());
    jsonwebtoken::encode(&header, claims, &key().encoding).unwrap()
}

pub fn token() -> String {
    sign(&claims())
}

/// The analyzer's end-to-end output, with both repositories read, as if
/// generated `minutes_ago` by the run at `SHA`.
pub fn snapshot(minutes_ago: i64) -> Dashboard {
    let snap = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../tests/snapshots/end_to_end__dashboard.snap"
    ))
    .unwrap();
    let body = snap.splitn(3, "---\n").nth(2).expect("insta front matter");
    let mut d: Dashboard = serde_json::from_str(body).unwrap();
    for r in &mut d.repos {
        r.engine_state_available = true;
    }
    d.commit = Some(SHA.into());
    d.generated_at = generated(minutes_ago);
    d
}

pub fn generated(minutes_ago: i64) -> DateTime<Utc> {
    // Whole seconds, as the token's `iat` is.
    DateTime::from_timestamp(now(), 0).unwrap() - TimeDelta::minutes(minutes_ago)
}

pub fn body(d: &Dashboard) -> Vec<u8> {
    serde_json::to_vec(d).unwrap()
}

pub async fn send(app: &TestApp, req: Request<Body>) -> Response {
    app.router.clone().oneshot(req).await.unwrap()
}

pub async fn post(app: &TestApp, token: Option<&str>, body: Vec<u8>) -> (StatusCode, Value) {
    let mut req = Request::post("/ingest").header(header::CONTENT_TYPE, "application/json");
    if let Some(token) = token {
        req = req.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let res = send(app, req.body(Body::from(body)).unwrap()).await;
    let status = res.status();
    (status, json_body(res).await)
}

/// Post `d` with a fresh valid token; panics unless it was stored.
pub async fn ingest(app: &TestApp, d: &Dashboard) -> Value {
    let (status, body) = post(app, Some(&token()), self::body(d)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["stored"], true, "{body}");
    body
}

pub async fn get(app: &TestApp, uri: &str) -> Response {
    send(app, Request::get(uri).body(Body::empty()).unwrap()).await
}

pub async fn get_json(app: &TestApp, uri: &str) -> (StatusCode, Value) {
    let res = get(app, uri).await;
    let status = res.status();
    (status, json_body(res).await)
}

pub async fn text_body(res: Response) -> String {
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

pub async fn json_body(res: Response) -> Value {
    let text = text_body(res).await;
    if text.is_empty() {
        return Value::Null;
    }
    serde_json::from_str(&text).unwrap_or(Value::String(text))
}
