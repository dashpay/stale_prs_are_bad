//! "Sign in with GitHub", sessions and the signed-in person's own routes,
//! against a fake GitHub.

mod common;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use axum::response::Response;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use common::*;
use pr_hygiene_service::auth::{PRELOGIN_COOKIE, SESSION_COOKIE, SIGNED_IN, SIGNIN_FAILED};
use pr_hygiene_service::config::Secret;
use pr_hygiene_service::github::{GithubError, GithubUser, SignInApi};
use pr_hygiene_service::oidc::BoxFuture;
use reqwest::Url;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

const TOKEN: &str = "ghu_faketoken_0123456789abcdef";
const CODE: &str = "c0de1234567890abcdef";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Call {
    Exchange { code: String, verifier: String },
    User(String),
    Revoke(String),
}

/// GitHub as the tests need it: each call answers as set, and is recorded.
struct FakeGithub {
    exchange: Mutex<Result<(), GithubError>>,
    user: Mutex<Result<GithubUser, GithubError>>,
    revoke: Mutex<Result<(), GithubError>>,
    calls: Mutex<Vec<Call>>,
}

impl FakeGithub {
    fn new() -> Arc<Self> {
        Self::as_user(4242, "alice")
    }

    fn as_user(id: i64, login: &str) -> Arc<Self> {
        Arc::new(Self {
            exchange: Mutex::new(Ok(())),
            user: Mutex::new(Ok(GithubUser {
                id,
                login: login.into(),
            })),
            revoke: Mutex::new(Ok(())),
            calls: Mutex::new(vec![]),
        })
    }

    fn calls(&self) -> Vec<Call> {
        self.calls.lock().unwrap().clone()
    }
}

impl SignInApi for FakeGithub {
    fn exchange<'a>(
        &'a self,
        code: &'a str,
        verifier: &'a str,
    ) -> BoxFuture<'a, Result<Secret, GithubError>> {
        Box::pin(async move {
            self.calls.lock().unwrap().push(Call::Exchange {
                code: code.into(),
                verifier: verifier.into(),
            });
            self.exchange
                .lock()
                .unwrap()
                .clone()
                .map(|()| Secret::new(TOKEN))
        })
    }

    fn user<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<GithubUser, GithubError>> {
        Box::pin(async move {
            self.calls
                .lock()
                .unwrap()
                .push(Call::User(token.expose().into()));
            self.user.lock().unwrap().clone()
        })
    }

    fn revoke<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<(), GithubError>> {
        Box::pin(async move {
            self.calls
                .lock()
                .unwrap()
                .push(Call::Revoke(token.expose().into()));
            self.revoke.lock().unwrap().clone()
        })
    }
}

fn app_for(github: &Arc<FakeGithub>) -> TestApp {
    signin_app(github.clone())
}

/// The full `Set-Cookie` line for cookie `name`, if the response sets it.
fn set_cookie(res: &Response, name: &str) -> Option<String> {
    res.headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_string())
        .find(|c| c.starts_with(&format!("{name}=")))
}

fn cookie_value(line: &str) -> String {
    line.split(';')
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1
        .to_string()
}

fn cookie_attributes(line: &str) -> Vec<String> {
    let mut attrs: Vec<String> = line
        .split(';')
        .skip(1)
        .map(|a| a.trim().to_ascii_lowercase())
        .collect();
    attrs.sort();
    attrs
}

fn location(res: &Response) -> String {
    res.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string()
}

/// A sign-in started: what the browser holds and what GitHub was sent.
struct Started {
    prelogin: String,
    state: String,
    challenge: String,
    params: BTreeMap<String, String>,
}

async fn start(app: &TestApp) -> Started {
    start_at(app, "/auth/login").await
}

async fn start_at(app: &TestApp, uri: &str) -> Started {
    let res = get(app, uri).await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    let to = Url::parse(&location(&res)).unwrap();
    assert_eq!(
        to.as_str().split('?').next().unwrap(),
        "https://github.com/login/oauth/authorize"
    );
    let params: BTreeMap<String, String> = to.query_pairs().into_owned().collect();
    let line = set_cookie(&res, PRELOGIN_COOKIE).expect("a pre-login cookie");
    Started {
        prelogin: cookie_value(&line),
        state: params["state"].clone(),
        challenge: params["code_challenge"].clone(),
        params,
    }
}

async fn callback(app: &TestApp, prelogin: Option<&str>, query: &str) -> Response {
    let mut req = Request::get(format!("/auth/callback?{query}"));
    if let Some(id) = prelogin {
        req = req.header(header::COOKIE, format!("{PRELOGIN_COOKIE}={id}"));
    }
    send(app, req.body(Body::empty()).unwrap()).await
}

/// Sign in all the way; the session cookie's id.
async fn sign_in(app: &TestApp) -> String {
    let s = start(app).await;
    let res = callback(
        app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNED_IN);
    cookie_value(&set_cookie(&res, SESSION_COOKIE).expect("a session cookie"))
}

async fn call(
    app: &TestApp,
    method: Method,
    uri: &str,
    session: Option<&str>,
    origin: Option<&str>,
) -> Response {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(id) = session {
        req = req.header(header::COOKIE, format!("{SESSION_COOKIE}={id}"));
    }
    if let Some(origin) = origin {
        req = req.header(header::ORIGIN, origin);
    }
    send(app, req.body(Body::empty()).unwrap()).await
}

async fn me(app: &TestApp, session: Option<&str>) -> (StatusCode, Value) {
    let res = call(app, Method::GET, "/api/v1/me", session, None).await;
    let status = res.status();
    (status, json_body(res).await)
}

fn db(app: &TestApp) -> rusqlite::Connection {
    rusqlite::Connection::open(&app.db).unwrap()
}

fn count(app: &TestApp, sql: &str) -> i64 {
    db(app).query_row(sql, [], |row| row.get(0)).unwrap()
}

fn sha256_hex(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn assert_private(res: &Response) {
    let h = res.headers();
    assert_eq!(h[header::CACHE_CONTROL], "private, no-store");
    assert_eq!(h[header::VARY], "Cookie");
    assert!(h.get(header::ETAG).is_none(), "nothing to revalidate");
    assert!(
        h.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none(),
        "no other origin may read it"
    );
}

// --- login ---

#[tokio::test]
async fn login_sends_the_browser_to_github_with_pkce_and_no_scopes() {
    let app = app_for(&FakeGithub::new());
    let s = start(&app).await;
    let keys: Vec<&str> = s.params.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "client_id",
            "code_challenge",
            "code_challenge_method",
            "redirect_uri",
            "state"
        ],
        "no scope: the App reads only the public profile"
    );
    assert_eq!(s.params["client_id"], CLIENT_ID);
    assert_eq!(s.params["redirect_uri"], format!("{ORIGIN}/auth/callback"));
    assert_eq!(s.params["code_challenge_method"], "S256");
    assert_eq!(s.challenge.len(), 43, "base64url of a SHA-256");
    assert!(s.state.len() >= 43, "256 random bits");
    assert_ne!(s.prelogin, s.state, "the cookie holds an opaque id only");

    let again = start(&app).await;
    assert_ne!(again.state, s.state);
    assert_ne!(again.challenge, s.challenge);
    assert_ne!(again.prelogin, s.prelogin);
}

#[tokio::test]
async fn the_prelogin_cookie_is_host_only_secure_http_only_lax_and_short_lived() {
    let app = app_for(&FakeGithub::new());
    let res = get(&app, "/auth/login").await;
    assert_private(&res);
    let line = set_cookie(&res, PRELOGIN_COOKIE).unwrap();
    assert_eq!(
        cookie_attributes(&line),
        [
            "httponly",
            "max-age=600",
            "path=/",
            "samesite=lax",
            "secure"
        ],
        "{line}"
    );
}

// --- callback ---

#[tokio::test]
async fn a_sign_in_ends_with_a_session_and_the_github_token_revoked() {
    let github = FakeGithub::new();
    let app = app_for(&github);
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(res.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&res), SIGNED_IN);
    assert_private(&res);
    let cleared = set_cookie(&res, PRELOGIN_COOKIE).expect("the pre-login cookie is cleared");
    assert!(
        cleared.to_ascii_lowercase().contains("max-age=0"),
        "{cleared}"
    );

    let calls = github.calls();
    let Call::Exchange { code, verifier } = &calls[0] else {
        panic!("{calls:?}");
    };
    assert_eq!(code, CODE);
    assert_eq!(
        URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes())),
        s.challenge,
        "the verifier sent is the one the challenge was made from"
    );
    assert_eq!(
        calls[1..],
        [Call::User(TOKEN.into()), Call::Revoke(TOKEN.into())],
        "the token is used once, then revoked"
    );

    let session = cookie_value(&set_cookie(&res, SESSION_COOKIE).unwrap());
    let (status, body) = me(&app, Some(&session)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["id"], 4242);
    assert_eq!(body["login"], "alice");
}

#[tokio::test]
async fn a_state_that_does_not_match_is_refused_before_github_is_asked() {
    let github = FakeGithub::new();
    let app = app_for(&github);
    let s = start(&app).await;
    // Someone else's sign-in, landed in this browser: their code and state,
    // this browser's pre-login.
    let theirs = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", theirs.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert!(set_cookie(&res, SESSION_COOKIE).is_none());
    assert_eq!(github.calls(), [], "the code never reaches GitHub");

    // That try used the pre-login up: the right state no longer helps.
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);

    // No pre-login cookie at all: the browser never started a sign-in.
    let res = callback(&app, None, &format!("code={CODE}&state={}", theirs.state)).await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert_eq!(github.calls(), []);
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
}

#[tokio::test]
async fn a_prelogin_signs_in_once() {
    let github = FakeGithub::new();
    let app = app_for(&github);
    let s = start(&app).await;
    let query = format!("code={CODE}&state={}", s.state);
    let first = callback(&app, Some(&s.prelogin), &query).await;
    assert_eq!(location(&first), SIGNED_IN);
    let replay = callback(&app, Some(&s.prelogin), &query).await;
    assert_eq!(location(&replay), SIGNIN_FAILED);
    assert!(set_cookie(&replay, SESSION_COOKIE).is_none());
    let exchanges = github
        .calls()
        .iter()
        .filter(|c| matches!(c, Call::Exchange { .. }))
        .count();
    assert_eq!(exchanges, 1);
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 1);
}

#[tokio::test]
async fn an_error_github_returns_with_http_200_fails_the_sign_in() {
    let github = FakeGithub::new();
    *github.exchange.lock().unwrap() = Err(GithubError::Refused("bad_verification_code".into()));
    let app = app_for(&github);
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert!(set_cookie(&res, SESSION_COOKIE).is_none());
    assert_eq!(github.calls().len(), 1, "no token, nothing more to call");
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
}

#[tokio::test]
async fn a_failed_user_lookup_fails_the_sign_in_and_still_revokes_the_token() {
    let github = FakeGithub::new();
    *github.user.lock().unwrap() = Err(GithubError::Status(502));
    let app = app_for(&github);
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert!(set_cookie(&res, SESSION_COOKIE).is_none());
    assert!(
        github.calls().contains(&Call::Revoke(TOKEN.into())),
        "a token is not left alive because the lookup failed"
    );
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
}

/// The token reads only public data and expires on its own; a revocation
/// GitHub refuses is worth a log line, not a failed sign-in.
#[tokio::test]
async fn a_failed_revocation_still_signs_in() {
    let github = FakeGithub::new();
    *github.revoke.lock().unwrap() = Err(GithubError::Status(422));
    let app = app_for(&github);
    let session = sign_in(&app).await;
    assert_eq!(me(&app, Some(&session)).await.0, StatusCode::OK);
}

/// Nothing GitHub (or someone posing as its redirect) puts in the callback
/// is shown back: the page reads a fixed failure flag, nothing else.
#[tokio::test]
async fn githubs_error_text_is_never_reflected() {
    let github = FakeGithub::new();
    let app = app_for(&github);
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!(
            "error=access_denied&error_description=%3Cscript%3Ealert(1)%3C%2Fscript%3E\
             &error_uri=https%3A%2F%2Fevil.example&state={}",
            s.state
        ),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    let headers = format!("{:?}", res.headers());
    let body = text_body(res).await;
    for leaked in ["script", "evil", "access_denied"] {
        assert!(!headers.contains(leaked), "{leaked} in {headers}");
        assert!(!body.contains(leaked), "{leaked} in {body}");
    }

    *github.exchange.lock().unwrap() =
        Err(GithubError::Refused("incorrect_client_credentials".into()));
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert!(!text_body(res).await.contains("credentials"));
}

#[tokio::test]
async fn no_request_chooses_where_a_sign_in_goes_or_ends() {
    let app = app_for(&FakeGithub::new());
    let s = start_at(
        &app,
        "/auth/login?redirect_uri=https%3A%2F%2Fevil.example&return_to=https%3A%2F%2Fevil.example",
    )
    .await;
    assert_eq!(s.params["redirect_uri"], format!("{ORIGIN}/auth/callback"));
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!(
            "code={CODE}&state={}&return_to=https%3A%2F%2Fevil.example\
             &next=%2F%2Fevil.example&redirect_uri=https%3A%2F%2Fevil.example",
            s.state
        ),
    )
    .await;
    assert_eq!(location(&res), SIGNED_IN);
}

// --- sessions ---

#[tokio::test]
async fn the_session_cookie_is_host_only_secure_http_only_lax_for_thirty_days() {
    let app = app_for(&FakeGithub::new());
    let s = start(&app).await;
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    let line = set_cookie(&res, SESSION_COOKIE).unwrap();
    assert_eq!(
        cookie_attributes(&line),
        [
            "httponly",
            "max-age=2592000",
            "path=/",
            "samesite=lax",
            "secure"
        ],
        "{line}"
    );
    assert_eq!(cookie_value(&line).len(), 43, "256 random bits");
}

/// A copy of the database (a backup, say) holds no usable session id.
#[tokio::test]
async fn the_database_holds_a_hash_of_the_session_id_never_the_id() {
    let app = app_for(&FakeGithub::new());
    let session = sign_in(&app).await;
    let stored: String = db(&app)
        .query_row("SELECT id_hash FROM sessions", [], |row| row.get(0))
        .unwrap();
    assert_eq!(stored, sha256_hex(&session));
    let everything: String = db(&app)
        .query_row(
            "SELECT group_concat(id_hash || login || created_at || expires_at) FROM sessions",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(!everything.contains(&session));
}

#[tokio::test]
async fn an_expired_session_lets_no_one_in() {
    let app = app_for(&FakeGithub::new());
    let session = sign_in(&app).await;
    db(&app)
        .execute(
            "UPDATE sessions SET expires_at = '2000-01-01T00:00:00.000000000Z'",
            [],
        )
        .unwrap();
    assert_eq!(me(&app, Some(&session)).await.0, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn signing_in_again_and_again_keeps_only_the_newest_few_sessions() {
    let app = app_for(&FakeGithub::new());
    let first = sign_in(&app).await;
    let mut latest = String::new();
    for _ in 0..pr_hygiene_service::store::MAX_SESSIONS_PER_USER {
        latest = sign_in(&app).await;
    }
    assert_eq!(
        count(&app, "SELECT count(*) FROM sessions"),
        pr_hygiene_service::store::MAX_SESSIONS_PER_USER
    );
    assert_eq!(me(&app, Some(&first)).await.0, StatusCode::UNAUTHORIZED);
    assert_eq!(me(&app, Some(&latest)).await.0, StatusCode::OK);
}

// --- /me ---

#[tokio::test]
async fn me_is_401_signed_out_and_private_either_way() {
    let app = app_for(&FakeGithub::as_user(1, "QuantumExplorer"));
    ingest(&app, &snapshot(5)).await;
    let res = call(
        &app,
        Method::GET,
        "/api/v1/me",
        None,
        Some("https://elsewhere.example"),
    )
    .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    assert_private(&res);
    let (status, _) = me(&app, Some("not-a-session-id")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let session = sign_in(&app).await;
    let req = Request::get("/api/v1/me")
        .header(header::COOKIE, format!("{SESSION_COOKIE}={session}"))
        .header(header::ORIGIN, "https://elsewhere.example")
        .header(header::IF_NONE_MATCH, "*")
        .body(Body::empty())
        .unwrap();
    let res = send(&app, req).await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_private(&res);
    let body = json_body(res).await;
    assert_eq!(body["id"], 1);
    assert_eq!(body["login"], "QuantumExplorer");
    assert_eq!(
        body["person"]["login"], "QuantumExplorer",
        "the public People entry of the one signed in"
    );
}

#[tokio::test]
async fn me_has_no_person_for_someone_the_data_does_not_name() {
    let app = app_for(&FakeGithub::as_user(77, "someone-new"));
    let session = sign_in(&app).await;
    // No snapshot yet: still signed in, just nobody to show.
    let (status, body) = me(&app, Some(&session)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body,
        json!({ "id": 77, "login": "someone-new", "person": null })
    );
    ingest(&app, &snapshot(5)).await;
    let (_, body) = me(&app, Some(&session)).await;
    assert_eq!(body["person"], Value::Null);
}

// --- state-changing routes ---

const CHANGES: [(Method, &str); 3] = [
    (Method::POST, "/auth/logout"),
    (Method::POST, "/api/v1/me/opt-out"),
    (Method::DELETE, "/api/v1/me"),
];

/// A page on another site can make a browser send these requests with its
/// cookie; only the `Origin` header tells them apart, so it must be exactly
/// the service's own.
#[tokio::test]
async fn every_state_change_needs_exactly_the_services_origin() {
    for (method, uri) in CHANGES {
        let app = app_for(&FakeGithub::new());
        let session = sign_in(&app).await;
        for origin in [
            None,
            Some("https://evil.example"),
            Some("null"),
            Some("https://hygiene.example.org/"),
            Some("http://hygiene.example.org"),
            Some("https://HYGIENE.example.org"),
            Some("https://hygiene.example.org.evil.example"),
        ] {
            let res = call(&app, method.clone(), uri, Some(&session), origin).await;
            assert_eq!(
                res.status(),
                StatusCode::FORBIDDEN,
                "{method} {uri} {origin:?}"
            );
            assert_private(&res);
        }
        // Two Origin headers, one of them right: not a browser's request.
        let req = Request::builder()
            .method(method.clone())
            .uri(uri)
            .header(header::COOKIE, format!("{SESSION_COOKIE}={session}"))
            .header(header::ORIGIN, ORIGIN)
            .header(header::ORIGIN, "https://evil.example")
            .body(Body::empty())
            .unwrap();
        assert_eq!(send(&app, req).await.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            me(&app, Some(&session)).await.0,
            StatusCode::OK,
            "{method} {uri}: a refused request changed nothing"
        );
        assert_eq!(count(&app, "SELECT count(*) FROM opt_outs"), 0);

        let res = call(&app, method.clone(), uri, Some(&session), Some(ORIGIN)).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT, "{method} {uri}");
    }
}

#[tokio::test]
async fn logout_ends_the_session_on_the_server() {
    let app = app_for(&FakeGithub::new());
    let session = sign_in(&app).await;
    let res = call(
        &app,
        Method::POST,
        "/auth/logout",
        Some(&session),
        Some(ORIGIN),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
    let cleared = set_cookie(&res, SESSION_COOKIE).unwrap();
    assert!(
        cleared.to_ascii_lowercase().contains("max-age=0"),
        "{cleared}"
    );
    assert_eq!(
        me(&app, Some(&session)).await.0,
        StatusCode::UNAUTHORIZED,
        "a copy of the cookie is worth nothing after logout"
    );
    let res = call(&app, Method::POST, "/auth/logout", None, Some(ORIGIN)).await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT, "signed out already");
}

#[tokio::test]
async fn opt_out_and_delete_need_a_session() {
    let app = app_for(&FakeGithub::new());
    for (method, uri) in &CHANGES[1..] {
        let res = call(&app, method.clone(), uri, None, Some(ORIGIN)).await;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED, "{method} {uri}");
    }
}

#[tokio::test]
async fn an_opt_out_is_kept_when_the_account_is_deleted() {
    let app = app_for(&FakeGithub::new());
    let laptop = sign_in(&app).await;
    let phone = sign_in(&app).await;
    let res = call(
        &app,
        Method::POST,
        "/api/v1/me/opt-out",
        Some(&laptop),
        Some(ORIGIN),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        me(&app, Some(&laptop)).await.0,
        StatusCode::OK,
        "still signed in"
    );
    assert_eq!(
        count(&app, "SELECT count(*) FROM opt_outs WHERE user_id = 4242"),
        1
    );

    let res = call(
        &app,
        Method::DELETE,
        "/api/v1/me",
        Some(&laptop),
        Some(ORIGIN),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
    assert!(set_cookie(&res, SESSION_COOKIE).is_some(), "cookie cleared");
    assert_eq!(me(&app, Some(&laptop)).await.0, StatusCode::UNAUTHORIZED);
    assert_eq!(
        me(&app, Some(&phone)).await.0,
        StatusCode::UNAUTHORIZED,
        "every session of the person goes, not just this browser's"
    );
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
    assert_eq!(
        count(&app, "SELECT count(*) FROM opt_outs WHERE user_id = 4242"),
        1,
        "the opt-out goes on being honoured"
    );

    sign_in(&app).await;
    assert_eq!(count(&app, "SELECT count(*) FROM opt_outs"), 1);
}

// --- sign-in off ---

#[tokio::test]
async fn with_sign_in_unconfigured_its_routes_are_404_and_the_rest_works() {
    let app = app();
    for (method, uri) in [
        (Method::GET, "/auth/login"),
        (Method::GET, "/auth/callback?code=x&state=y"),
        (Method::GET, "/api/v1/me"),
    ]
    .into_iter()
    .chain(CHANGES)
    {
        let res = call(&app, method.clone(), uri, None, Some(ORIGIN)).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND, "{method} {uri}");
        assert_private(&res);
    }
    ingest(&app, &snapshot(5)).await;
    let (status, _) = get_json(&app, "/api/v1/prs").await;
    assert_eq!(status, StatusCode::OK);
}

// --- logs ---

#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Captured {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Codes, tokens, cookies, session ids, the client secret, the state and
/// the verifier: none of them is ever written to the log, on any path.
#[tokio::test]
async fn the_log_holds_no_code_token_cookie_or_secret() {
    let log = Captured::default();
    let writer = log.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(move || writer.clone())
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    let github = FakeGithub::new();
    let app = app_for(&github);
    let mut secrets = vec![
        CODE.to_string(),
        TOKEN.to_string(),
        CLIENT_SECRET.to_string(),
    ];
    let mut try_once = |s: &Started| {
        secrets.extend([s.prelogin.clone(), s.state.clone()]);
    };

    let s = start(&app).await;
    try_once(&s);
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    let session = cookie_value(&set_cookie(&res, SESSION_COOKIE).unwrap());
    me(&app, Some(&session)).await;

    let s = start(&app).await;
    try_once(&s);
    callback(&app, Some(&s.prelogin), &format!("code={CODE}&state=wrong")).await;

    *github.revoke.lock().unwrap() = Err(GithubError::Status(500));
    *github.user.lock().unwrap() = Err(GithubError::Status(502));
    let s = start(&app).await;
    try_once(&s);
    callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;

    call(
        &app,
        Method::POST,
        "/auth/logout",
        Some(&session),
        Some(ORIGIN),
    )
    .await;
    for c in github.calls() {
        if let Call::Exchange { verifier, .. } = c {
            secrets.push(verifier);
        }
    }
    secrets.push(session);

    let text = String::from_utf8(log.0.lock().unwrap().clone()).unwrap();
    assert!(
        text.contains("sign-in failed"),
        "the log was captured: {text}"
    );
    for secret in &secrets {
        assert!(!text.contains(secret.as_str()), "{secret} logged:\n{text}");
    }
}
