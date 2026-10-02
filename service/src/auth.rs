//! "Sign in with GitHub", sessions, and the signed-in person's own routes:
//! `/auth/*` and `/api/v1/me*`. Signing in only tells the page who you are.
//!
//! None of this is public API: no CORS, never cached, varying by cookie.
//! Every route that changes something requires the request's `Origin` to be
//! exactly the service's own, which a page on any other site cannot fake.

use crate::app::{ApiError, AppState};
use crate::config::SignInConfig;
use crate::github::{GithubUser, SignInApi, AUTHORIZE_URL};
use crate::snapshot::is_login;
use crate::store::{Prelogin, Session, PRELOGIN_LIFETIME, SESSION_LIFETIME};
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::Json;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{TimeDelta, Utc};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Holds the id of a sign-in under way. `__Host-`: only this exact host,
/// over HTTPS, can set it — not a sibling subdomain.
pub const PRELOGIN_COOKIE: &str = "__Host-prh_prelogin";
/// Holds the id of a session.
pub const SESSION_COOKIE: &str = "__Host-prh_session";

/// Where a sign-in ends. Fixed: no request can choose where it is sent.
pub const SIGNED_IN: &str = "/#/me";
pub const SIGNIN_FAILED: &str = "/#/me?signin=failed";

/// Sign-in, when configured: its settings and GitHub.
pub struct SignIn {
    pub config: SignInConfig,
    pub github: Arc<dyn SignInApi>,
}

/// Headers on every response here: for this browser only, never stored by
/// it or any cache, and readable by no other origin.
pub async fn private_headers(mut res: Response) -> Response {
    let h = res.headers_mut();
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-store"),
    );
    h.insert(header::VARY, HeaderValue::from_static("Cookie"));
    h.remove(header::ETAG);
    h.remove(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    // A 401 here means "no session cookie", not "send a bearer token".
    h.remove(header::WWW_AUTHENTICATE);
    h.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    // The callback's URL carries GitHub's code; no page reached from here
    // is told it.
    h.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    res
}

/// 256 random bits from the operating system, URL-safe: 43 characters,
/// which also makes a valid PKCE verifier.
fn random_id() -> Result<String, ApiError> {
    let mut bytes = [0u8; 32];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(|e| ApiError::internal(anyhow::anyhow!("no randomness: {e}")))?;
    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

fn is_random_id(s: &str) -> bool {
    s.len() == 43
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

/// A `__Host-` cookie: `Secure`, `Path=/` and no `Domain`, or browsers
/// refuse it. `HttpOnly` keeps it from the page's scripts; `SameSite=Lax`
/// still sends it on the way back from github.com, which `Strict` would not.
fn set_cookie(name: &str, value: &str, max_age: TimeDelta) -> Result<HeaderValue, ApiError> {
    HeaderValue::from_str(&format!(
        "{name}={value}; Path=/; Max-Age={}; Secure; HttpOnly; SameSite=Lax",
        max_age.num_seconds()
    ))
    .map_err(|e| ApiError::internal(anyhow::anyhow!("cookie {name}: {e}")))
}

fn clear_cookie(name: &str) -> Result<HeaderValue, ApiError> {
    set_cookie(name, "", TimeDelta::zero())
}

/// The id in cookie `name`, if the request carries one shaped like ours.
fn cookie<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(n, _)| *n == name)
        .map(|(_, v)| v)
        .filter(|v| is_random_id(v))
}

fn redirect(to: &str, cookies: impl IntoIterator<Item = HeaderValue>) -> Response {
    let mut res = Redirect::to(to).into_response();
    for c in cookies {
        res.headers_mut().append(header::SET_COOKIE, c);
    }
    res
}

fn unauthorized() -> ApiError {
    ApiError::new(StatusCode::UNAUTHORIZED, "not signed in")
}

/// One `Origin` header, byte for byte the service's own. Browsers send it on
/// every POST and DELETE; a request without one is not from the page.
fn same_origin(signin: &SignIn, headers: &HeaderMap) -> Result<(), ApiError> {
    let mut values = headers.get_all(header::ORIGIN).iter();
    match (values.next(), values.next()) {
        (Some(origin), None) if origin.as_bytes() == signin.config.origin.as_bytes() => Ok(()),
        _ => Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "this request must come from the PR Hygiene page itself",
        )),
    }
}

/// The unexpired session the request's cookie names.
async fn current_session(state: &Arc<AppState>, headers: &HeaderMap) -> Result<Session, ApiError> {
    let id = cookie(headers, SESSION_COOKIE)
        .map(String::from)
        .ok_or_else(unauthorized)?;
    state
        .read(move |reader| reader.session(&id, Utc::now()))
        .await?
        .ok_or_else(unauthorized)
}

/// `GET /auth/login`: remember a fresh `state` and PKCE verifier under a
/// random id, hand the browser that id in a cookie, and send it to GitHub.
/// No scopes are asked for: a GitHub App's user token has none, and this
/// App has no permissions, so it reads public profile data only.
///
/// The state and verifier stay in the database rather than in a signed
/// cookie: a row can be deleted when it is used, which a cookie cannot,
/// and there is no signing key to keep and rotate.
pub async fn login(State(state): State<Arc<AppState>>) -> Result<Response, ApiError> {
    let signin = state.signin()?;
    let id = random_id()?;
    let prelogin = Prelogin {
        state: random_id()?,
        verifier: random_id()?,
    };
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(prelogin.verifier.as_bytes()));
    let mut url = reqwest::Url::parse(AUTHORIZE_URL)
        .map_err(|e| ApiError::internal(anyhow::anyhow!("authorize URL: {e}")))?;
    url.query_pairs_mut()
        .append_pair("client_id", &signin.config.client_id)
        .append_pair("redirect_uri", &signin.config.redirect_uri())
        .append_pair("state", &prelogin.state)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256");
    let cookie = set_cookie(PRELOGIN_COOKIE, &id, PRELOGIN_LIFETIME)?;
    state
        .write(move |store| store.begin_signin(&id, &prelogin, Utc::now()))
        .await?;
    Ok(redirect(url.as_str(), [cookie]))
}

/// What GitHub sends back. Anything else in the query — GitHub's own
/// `error` and `error_description` among it — is ignored, never echoed.
#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
}

/// `GET /auth/callback`: finish the sign-in. Success and failure alike end
/// on a fixed page of the site; the pre-login cookie is cleared either way.
pub async fn callback(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    query: Result<Query<CallbackQuery>, axum::extract::rejection::QueryRejection>,
) -> Result<Response, ApiError> {
    let signin = state.signin()?;
    let query = query.ok().map(|Query(q)| q);
    let clear = clear_cookie(PRELOGIN_COOKIE)?;
    match complete(&state, signin, &headers, query).await {
        Ok(session_id) => {
            let session = set_cookie(SESSION_COOKIE, &session_id, SESSION_LIFETIME)?;
            Ok(redirect(SIGNED_IN, [clear, session]))
        }
        Err(Failed(reason)) => {
            tracing::warn!(%reason, "sign-in failed");
            Ok(redirect(SIGNIN_FAILED, [clear]))
        }
    }
}

/// Why a sign-in failed, for the log only: it never quotes a code, token,
/// cookie or GitHub's free text.
struct Failed(String);

impl From<&str> for Failed {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<ApiError> for Failed {
    fn from(e: ApiError) -> Self {
        Self(e.message)
    }
}

/// The steps of a callback, in order; the new session's id when all pass.
async fn complete(
    state: &Arc<AppState>,
    signin: &SignIn,
    headers: &HeaderMap,
    query: Option<CallbackQuery>,
) -> Result<String, Failed> {
    let id = cookie(headers, PRELOGIN_COOKIE)
        .ok_or("no pre-login cookie")?
        .to_string();
    // Looked up on the reader first: a made-up cookie must not cost a
    // write, which would queue behind (and ahead of) ingest.
    let lookup = id.clone();
    if !state.read(move |reader| reader.has_signin(&lookup)).await? {
        return Err("the pre-login is unknown, used or expired".into());
    }
    // Taken before anything else is checked, so each pre-login is tried
    // once, whatever this callback carries.
    let prelogin = state
        .write(move |store| store.take_signin(&id, Utc::now()))
        .await?
        .ok_or("the pre-login is unknown, used or expired")?;
    let query = query.ok_or("the callback's query is malformed")?;
    // A plain comparison is safe: a pre-login is gone after one try, so
    // there is no second guess to time.
    if query.state.as_deref() != Some(prelogin.state.as_str()) {
        return Err("state does not match the pre-login".into());
    }
    let code = query
        .code
        .filter(|c| is_code(c))
        .ok_or("no code, or not one GitHub would send")?;

    // A task of its own, so the token is revoked even if this request is
    // dropped meanwhile: the browser leaving, or the request timing out.
    let github = signin.github.clone();
    let user = tokio::spawn(async move { identify(&*github, &code, &prelogin.verifier).await })
        .await
        .map_err(|e| Failed(format!("the GitHub task failed: {e}")))??;
    if user.id <= 0 || !is_login(&user.login) {
        return Err("GitHub's user has no usable id or login".into());
    }

    let session_id = random_id()?;
    let cookie_id = session_id.clone();
    let replacing = cookie(headers, SESSION_COOKIE).map(String::from);
    state
        .write(move |store| {
            store.create_session(
                &cookie_id,
                user.id,
                &user.login,
                replacing.as_deref(),
                Utc::now(),
            )
        })
        .await?;
    tracing::info!("signed in");
    Ok(session_id)
}

/// Exchange the code, read whose token it is, and revoke the token.
async fn identify(
    github: &dyn SignInApi,
    code: &str,
    verifier: &str,
) -> Result<GithubUser, Failed> {
    let token = github
        .exchange(code, verifier)
        .await
        .map_err(|e| Failed(format!("exchanging the code: {e}")))?;
    let user = github.user(&token).await;
    // Whether or not the user could be read, the token is not needed again.
    if let Err(e) = github.revoke(&token).await {
        tracing::warn!("revoking the sign-in token failed: {e}");
    }
    user.map_err(|e| Failed(format!("reading the user: {e}")))
}

/// GitHub's codes are short and URL-safe; anything else is not sent on.
fn is_code(s: &str) -> bool {
    (1..=256).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
}

/// `GET /api/v1/me`: who is signed in, with their public People entry
/// (null when the data does not name them, or holds no snapshot yet) and
/// whether they opted out. The entry is found by the login at sign-in:
/// after a rename on GitHub it is null until the person signs in again.
pub async fn me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    state.signin()?;
    let session = current_session(&state, &headers).await?;
    let view = match state.view().await {
        Ok(view) => Some(view),
        Err(e) if e.status == StatusCode::SERVICE_UNAVAILABLE => None,
        Err(e) => return Err(e),
    };
    let person = view.as_ref().and_then(|v| v.person(&session.login));
    let id = session.user_id;
    let opted_out = state.read(move |reader| reader.opted_out(id)).await?;
    Ok(Json(json!({
        "id": session.user_id,
        "login": session.login,
        "person": person,
        "opted_out": opted_out,
    }))
    .into_response())
}

/// `GET /api/v1/me/speed`: the signed-in person's own speed, computed now
/// from their speed inputs alone, or `{"opted_out": true}`. It changes
/// nothing, so it needs no `Origin`; like every route here it is readable
/// by no other origin and never cached.
pub async fn speed(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    state.signin()?;
    let id = current_session(&state, &headers).await?.user_id;
    let speed = state
        .read(move |reader| {
            if reader.opted_out(id)? {
                return Ok(None);
            }
            reader.speed(id, Utc::now()).map(Some)
        })
        .await?;
    Ok(match speed {
        Some(speed) => Json(speed).into_response(),
        None => Json(json!({ "opted_out": true })).into_response(),
    })
}

/// No content, and the session cookie cleared.
fn signed_out() -> Result<Response, ApiError> {
    let mut res = StatusCode::NO_CONTENT.into_response();
    res.headers_mut()
        .append(header::SET_COOKIE, clear_cookie(SESSION_COOKIE)?);
    Ok(res)
}

/// `POST /auth/logout`: end this browser's session on the server, not just
/// its cookie. Signed out already is fine.
pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    same_origin(state.signin()?, &headers)?;
    if let Some(id) = cookie(&headers, SESSION_COOKIE).map(String::from) {
        state.write(move |store| store.end_session(&id)).await?;
    }
    signed_out()
}

/// `POST /api/v1/me/opt-out`: stop computing the person's speed and forget
/// its inputs, from now on. Their session stays.
pub async fn opt_out(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    same_origin(state.signin()?, &headers)?;
    let session = current_session(&state, &headers).await?;
    state
        .write(move |store| store.opt_out(session.user_id, &session.login, Utc::now()))
        .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

/// `DELETE /api/v1/me`: every session of the person, on every browser, and
/// their speed inputs. An opt-out is kept.
pub async fn delete_me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    same_origin(state.signin()?, &headers)?;
    let session = current_session(&state, &headers).await?;
    state
        .write(move |store| store.delete_user(session.user_id, &session.login))
        .await?;
    signed_out()
}
