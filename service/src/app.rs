//! The HTTP service: ingest, the public read API, sign-in, health.

use crate::api;
use crate::auth::{self, SignIn};
use crate::config::{IngestConfig, SignInConfig};
use crate::github::SignInApi;
use crate::oidc::{AuthError, KeyCache, Verifier};
use crate::overlay::overlay;
use crate::push::{self, Bound};
use crate::snapshot;
use crate::store::{Outcome, Pushed, Reader, Store};
use crate::view::View;
use axum::body::{Body, Bytes};
use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use http_body_util::{BodyExt, LengthLimitError, Limited};
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;

/// Longest any request may take, the body upload included.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub struct AppState {
    pub ingest: IngestConfig,
    pub verifier: Verifier,
    writer: Mutex<Store>,
    reader: Mutex<Reader>,
    /// The view last served, with the push count it was built at: reused
    /// while neither the stored snapshot nor anything pushed has changed.
    cached: RwLock<Option<(u64, Arc<View>)>>,
    /// Pushes stored since the process started.
    pushes: AtomicU64,
    /// `None` while sign-in is not configured.
    signin: Option<SignIn>,
}

impl AppState {
    pub fn new(ingest: IngestConfig, keys: Arc<KeyCache>, writer: Store, reader: Reader) -> Self {
        Self {
            verifier: Verifier::new(&ingest, keys),
            ingest,
            writer: Mutex::new(writer),
            reader: Mutex::new(reader),
            cached: RwLock::new(None),
            pushes: AtomicU64::new(0),
            signin: None,
        }
    }

    /// Turn on "Sign in with GitHub", against `github`.
    pub fn with_signin(mut self, config: SignInConfig, github: Arc<dyn SignInApi>) -> Self {
        self.signin = Some(SignIn { config, github });
        self
    }

    /// Sign-in, or a 404 for every sign-in route while it is off.
    pub fn signin(&self) -> Result<&SignIn, ApiError> {
        self.signin
            .as_ref()
            .ok_or_else(|| ApiError::not_found("sign-in is not enabled on this service"))
    }

    /// Run `f` with the writing connection, off the async runtime.
    pub async fn write<T: Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(&mut Store) -> anyhow::Result<T> + Send + 'static,
    ) -> Result<T, ApiError> {
        let state = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut store = state.writer.lock().unwrap_or_else(PoisonError::into_inner);
            f(&mut store)
        })
        .await
        .map_err(|e| ApiError::internal(anyhow::anyhow!("writer task: {e}")))?
        .map_err(ApiError::internal)
    }

    /// Run `f` with the read-only connection, off the async runtime.
    pub async fn read<T: Send + 'static>(
        self: &Arc<Self>,
        f: impl FnOnce(&Reader) -> anyhow::Result<T> + Send + 'static,
    ) -> Result<T, ApiError> {
        let state = self.clone();
        tokio::task::spawn_blocking(move || {
            let reader = state.reader.lock().unwrap_or_else(PoisonError::into_inner);
            f(&reader)
        })
        .await
        .map_err(|e| ApiError::internal(anyhow::anyhow!("reader task: {e}")))?
        .map_err(ApiError::internal)
    }

    /// The latest view, with what was pushed since shown over it, or 503
    /// before the first snapshot.
    pub async fn view(self: &Arc<Self>) -> Result<Arc<View>, ApiError> {
        let state = self.clone();
        let view = self
            .read(move |reader| {
                // Read before the pushes are: a push stored meanwhile then
                // leaves a view built with it under the older count, and the
                // next request builds it again.
                let pushes = state.pushes.load(Ordering::SeqCst);
                let Some(version) = reader.version()? else {
                    return Ok(None);
                };
                let cached = state.cached.read().unwrap_or_else(PoisonError::into_inner);
                if let Some((_, view)) = cached
                    .as_ref()
                    .filter(|(n, v)| v.version == version && *n == pushes)
                {
                    return Ok(Some(view.clone()));
                }
                drop(cached);
                let Some(view) = reader.view()? else {
                    return Ok(None);
                };
                let (verdicts, open) = reader.pushes()?;
                let view = Arc::new(overlay(&view, &verdicts, &open, Utc::now()));
                *state.cached.write().unwrap_or_else(PoisonError::into_inner) =
                    Some((pushes, view.clone()));
                Ok(Some(view))
            })
            .await?;
        view.ok_or_else(|| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "no snapshot has been ingested yet",
            )
        })
    }
}

pub fn router(state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/api/v1/prs", get(api::prs).options(api::preflight))
        .route(
            "/api/v1/prs/{owner}/{repo}/{number}",
            get(api::pr).options(api::preflight),
        )
        .route("/api/v1/people", get(api::people).options(api::preflight))
        .route(
            "/api/v1/people/{login}",
            get(api::person).options(api::preflight),
        )
        .route("/api/v1/repos", get(api::repos).options(api::preflight))
        .route("/api/v1/stages", get(api::stages).options(api::preflight))
        .route(
            "/dashboard.json",
            get(api::dashboard).options(api::preflight),
        )
        .layer(middleware::map_response(api::public_headers));
    // Outside the public router, so none of its CORS or caching applies.
    let private = Router::new()
        .route("/auth/login", get(auth::login))
        .route("/auth/callback", get(auth::callback))
        .route("/auth/logout", post(auth::logout))
        .route("/api/v1/me", get(auth::me).delete(auth::delete_me))
        .route("/api/v1/me/opt-out", post(auth::opt_out))
        .route("/api/v1/me/speed", get(auth::speed))
        .layer(middleware::map_response(auth::private_headers));
    Router::new()
        .merge(public)
        .merge(private)
        .route("/ingest", post(ingest))
        .route("/ingest/verdicts", post(ingest_verdicts))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .layer(middleware::from_fn(log_request))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .with_state(state)
}

/// The page's security policy, sent as a header: the same as its meta tag,
/// plus `frame-ancestors`, which only a header can set.
const PAGE_CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; \
img-src 'self' https://avatars.githubusercontent.com; connect-src 'self'; base-uri 'none'; \
object-src 'none'; form-action 'none'; frame-ancestors 'none'; \
require-trusted-types-for 'script'; trusted-types 'none'";

/// The page's files from `dir`, with its security headers. Paths cannot
/// leave `dir`.
pub fn site(dir: PathBuf) -> Router {
    Router::new()
        .fallback_service(ServeDir::new(dir).append_index_html_on_directories(true))
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(PAGE_CSP),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
}

/// An error as the API returns it: a status and a JSON reason.
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    /// Logged in full, answered without detail.
    pub fn internal(e: anyhow::Error) -> Self {
        tracing::error!("internal error: {e:#}");
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal error")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut res = (self.status, Json(json!({ "error": self.message }))).into_response();
        if self.status == StatusCode::UNAUTHORIZED {
            res.headers_mut()
                .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        }
        res
    }
}

impl From<AuthError> for ApiError {
    fn from(e: AuthError) -> Self {
        let status = match e {
            AuthError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden(_) => StatusCode::FORBIDDEN,
            AuthError::KeysUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        };
        Self::new(status, e.to_string())
    }
}

fn bearer(headers: &HeaderMap) -> Option<&str> {
    let mut values = headers.get_all(header::AUTHORIZATION).iter();
    let value = values.next()?;
    if values.next().is_some() {
        return None;
    }
    let value = value.to_str().ok()?;
    let (scheme, token) = value.split_once(' ')?;
    scheme
        .eq_ignore_ascii_case("bearer")
        .then(|| token.trim())
        .filter(|t| !t.is_empty())
}

/// `POST /ingest`: a snapshot from the scheduled workflow, with its OIDC
/// token. The token is checked before a byte of the body is read; the body
/// is read up to the limit, checked, bound to the token's run, and stored.
async fn ingest(State(state): State<Arc<AppState>>, headers: HeaderMap, body: Body) -> Response {
    match ingest_checked(&state, &headers, body).await {
        Ok(res) => res,
        Err(e) => {
            tracing::warn!(status = e.status.as_u16(), reason = %e.message, "ingest refused");
            e.into_response()
        }
    }
}

async fn ingest_checked(
    state: &Arc<AppState>,
    headers: &HeaderMap,
    body: Body,
) -> Result<Response, ApiError> {
    let token = bearer(headers)
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "token: missing bearer token"))?;
    let verified = state.verifier.verify(token).await?;
    let bytes = read_body(headers, body, state.ingest.body_limit, "snapshot").await?;
    let d = snapshot::parse(&bytes, Utc::now()).map_err(|e| ApiError::bad_request(e.0))?;
    snapshot::bind(&d, &verified, state.ingest.job_timeout())
        .map_err(|e| ApiError::bad_request(e.0))?;
    let raw = String::from_utf8(bytes.to_vec())
        .map_err(|_| ApiError::bad_request("body is not UTF-8"))?;

    let writer = state.clone();
    let token_id = verified.token_id.clone();
    let outcome = tokio::task::spawn_blocking(move || {
        let mut store = writer.writer.lock().unwrap_or_else(PoisonError::into_inner);
        store.ingest(&d, &raw, Some(&token_id), Utc::now())
    })
    .await
    .map_err(|e| ApiError::internal(anyhow::anyhow!("ingest task: {e}")))?
    .map_err(ApiError::internal)?;

    Ok(match outcome {
        Outcome::Stored {
            snapshot,
            stale,
            stage_changes,
        } => {
            tracing::info!(snapshot, ?stale, stage_changes, sha = %verified.sha, "snapshot stored");
            Json(json!({
                "stored": true,
                "snapshot": snapshot,
                "stale_repos": stale,
                "stage_changes": stage_changes,
            }))
            .into_response()
        }
        Outcome::NotNewer { latest } => {
            tracing::info!(%latest, "snapshot not newer than the latest; ignored");
            Json(json!({
                "stored": false,
                "reason": "not newer than the latest snapshot",
                "latest_generated_at": latest,
            }))
            .into_response()
        }
        Outcome::TokenUsed => {
            tracing::warn!("a token posted a second time; ignored");
            Json(json!({
                "stored": false,
                "reason": "token already used: each token posts once; mint a new one",
            }))
            .into_response()
        }
    })
}

/// The body, up to `limit` bytes.
async fn read_body(
    headers: &HeaderMap,
    body: Body,
    limit: usize,
    what: &str,
) -> Result<Bytes, ApiError> {
    let too_large = || {
        ApiError::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("{what} larger than {limit} bytes"),
        )
    };
    let declared = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());
    if declared.is_some_and(|n| n > limit as u64) {
        return Err(too_large());
    }
    match Limited::new(body, limit).collect().await {
        Ok(collected) => Ok(collected.to_bytes()),
        Err(e) if e.downcast_ref::<LengthLimitError>().is_some() => Err(too_large()),
        Err(_) => Err(ApiError::bad_request("could not read the body")),
    }
}

/// `POST /ingest/verdicts`: a governed repository's newest verdicts and
/// open PRs, from its PR Hygiene run, with that job's OIDC token. The token
/// is checked before the body is read; the body must be for the token's own
/// repository.
async fn ingest_verdicts(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    match ingest_verdicts_checked(&state, &headers, body).await {
        Ok(res) => res,
        Err(e) => {
            tracing::warn!(status = e.status.as_u16(), reason = %e.message, "push refused");
            e.into_response()
        }
    }
}

async fn ingest_verdicts_checked(
    state: &Arc<AppState>,
    headers: &HeaderMap,
    body: Body,
) -> Result<Response, ApiError> {
    let token = bearer(headers)
        .ok_or_else(|| ApiError::new(StatusCode::UNAUTHORIZED, "token: missing bearer token"))?;
    let pusher = state.verifier.verify_pusher(token).await?;
    let bytes = read_body(headers, body, push::BODY_LIMIT, "push").await?;
    let pushed = push::parse(&bytes, Utc::now()).map_err(|e| ApiError::bad_request(e.0))?;
    push::bind(&pushed, &pusher).map_err(|e| match e {
        Bound::OtherRepository => ApiError::new(
            StatusCode::FORBIDDEN,
            "repo: a token may push for its own repository only",
        ),
        Bound::Invalid(e) => ApiError::bad_request(e.0),
    })?;
    let repo = pushed.repo.clone();
    let token_id = pusher.token_id;
    let outcome = state
        .write(move |store| store.push(&pushed, &token_id, Utc::now()))
        .await?;
    Ok(match outcome {
        Pushed::Stored { verdicts, open } => {
            state.pushes.fetch_add(1, Ordering::SeqCst);
            tracing::info!(%repo, verdicts, open, "push stored");
            Json(json!({ "stored": true, "verdicts": verdicts, "open": open })).into_response()
        }
        Pushed::TokenUsed => {
            tracing::warn!(%repo, "a token pushed a second time; ignored");
            Json(json!({
                "stored": false,
                "reason": "token already used: each token posts once; mint a new one",
            }))
            .into_response()
        }
    })
}

/// The process is up.
async fn healthz() -> &'static str {
    "ok\n"
}

/// Ready to serve: the database answers and GitHub's signing keys are held
/// (without them every ingest is refused).
async fn readyz(State(state): State<Arc<AppState>>) -> Response {
    let db = state.read(|reader| reader.ping()).await.is_ok();
    let keys = state.verifier.keys().state();
    let ready = db && keys.keys > 0;
    let status = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(json!({
            "ready": ready,
            "db": if db { "ok" } else { "unavailable" },
            "jwks": keys,
        })),
    )
        .into_response()
}

/// One line per request: method, path, status, time. Never headers (they
/// carry the token), never the client's address.
async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path: String = req.uri().path().chars().take(200).collect();
    let start = Instant::now();
    let res = next.run(req).await;
    tracing::info!(
        %method,
        %path,
        status = res.status().as_u16(),
        ms = start.elapsed().as_millis() as u64,
        "request"
    );
    res
}
