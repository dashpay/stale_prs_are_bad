//! The HTTP service: ingest, the public read API, health.

use crate::api;
use crate::config::IngestConfig;
use crate::oidc::{AuthError, KeyCache, Verifier};
use crate::snapshot;
use crate::store::{Outcome, Reader, Store};
use crate::view::View;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use http_body_util::{BodyExt, LengthLimitError, Limited};
use serde_json::json;
use std::sync::{Arc, Mutex, PoisonError, RwLock};
use std::time::{Duration, Instant};
use tower_http::timeout::TimeoutLayer;

/// Longest any request may take, the body upload included.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub struct AppState {
    pub ingest: IngestConfig,
    pub verifier: Verifier,
    writer: Mutex<Store>,
    reader: Mutex<Reader>,
    /// The view last read, reused while the stored version is unchanged.
    cached: RwLock<Option<Arc<View>>>,
}

impl AppState {
    pub fn new(ingest: IngestConfig, keys: Arc<KeyCache>, writer: Store, reader: Reader) -> Self {
        Self {
            verifier: Verifier::new(&ingest, keys),
            ingest,
            writer: Mutex::new(writer),
            reader: Mutex::new(reader),
            cached: RwLock::new(None),
        }
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

    /// The latest view, or 503 before the first snapshot.
    pub async fn view(self: &Arc<Self>) -> Result<Arc<View>, ApiError> {
        let state = self.clone();
        let view = self
            .read(move |reader| {
                let Some(version) = reader.version()? else {
                    return Ok(None);
                };
                let cached = state.cached.read().unwrap_or_else(PoisonError::into_inner);
                if let Some(view) = cached.as_ref().filter(|v| v.version == version) {
                    return Ok(Some(view.clone()));
                }
                drop(cached);
                let Some(view) = reader.view()? else {
                    return Ok(None);
                };
                let view = Arc::new(view);
                *state.cached.write().unwrap_or_else(PoisonError::into_inner) = Some(view.clone());
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
        .route("/api/v1/prs", get(api::prs))
        .route("/api/v1/prs/{owner}/{repo}/{number}", get(api::pr))
        .route("/api/v1/people", get(api::people))
        .route("/api/v1/people/{login}", get(api::person))
        .route("/api/v1/repos", get(api::repos))
        .route("/api/v1/stages", get(api::stages))
        .layer(middleware::map_response(api::public_headers));
    Router::new()
        .merge(public)
        .route("/ingest", post(ingest))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .layer(middleware::from_fn(log_request))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .with_state(state)
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

    let limit = state.ingest.body_limit;
    let too_large = || {
        ApiError::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!("snapshot larger than {limit} bytes"),
        )
    };
    let declared = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());
    if declared.is_some_and(|n| n > limit as u64) {
        return Err(too_large());
    }
    let bytes = match Limited::new(body, limit).collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) if e.downcast_ref::<LengthLimitError>().is_some() => return Err(too_large()),
        Err(_) => return Err(ApiError::bad_request("could not read the body")),
    };
    let d = snapshot::parse(&bytes).map_err(|e| ApiError::bad_request(e.0))?;
    snapshot::bind(&d, &verified, state.ingest.job_timeout())
        .map_err(|e| ApiError::bad_request(e.0))?;
    let raw = String::from_utf8(bytes.to_vec())
        .map_err(|_| ApiError::bad_request("body is not UTF-8"))?;

    let writer = state.clone();
    let outcome = tokio::task::spawn_blocking(move || {
        let mut store = writer.writer.lock().unwrap_or_else(PoisonError::into_inner);
        store.ingest(&d, &raw, Utc::now())
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
