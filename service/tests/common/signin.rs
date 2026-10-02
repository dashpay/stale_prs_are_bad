//! Signing in against a fake GitHub, and reading what the service answers.

use super::*;
use axum::http::Method;
use pr_hygiene_service::auth::{PRELOGIN_COOKIE, SESSION_COOKIE, SIGNED_IN};
use pr_hygiene_service::config::Secret;
use pr_hygiene_service::github::{GithubError, GithubUser, SignInApi};
use pr_hygiene_service::oidc::BoxFuture;
use reqwest::Url;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::Mutex;

pub const TOKEN: &str = "ghu_faketoken_0123456789abcdef";
pub const CODE: &str = "c0de1234567890abcdef";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Call {
    Exchange { code: String, verifier: String },
    User(String),
    Revoke(String),
}

/// GitHub as the tests need it: each call answers as set, and is recorded.
pub struct FakeGithub {
    pub exchange: Mutex<Result<(), GithubError>>,
    pub user: Mutex<Result<GithubUser, GithubError>>,
    pub revoke: Mutex<Result<(), GithubError>>,
    calls: Mutex<Vec<Call>>,
}

impl FakeGithub {
    pub fn new() -> Arc<Self> {
        Self::as_user(4242, "alice")
    }

    pub fn as_user(id: i64, login: &str) -> Arc<Self> {
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

    pub fn calls(&self) -> Vec<Call> {
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

pub fn app_for(github: &Arc<FakeGithub>) -> TestApp {
    signin_app(github.clone())
}

/// The full `Set-Cookie` line for cookie `name`, if the response sets it.
pub fn set_cookie(res: &Response, name: &str) -> Option<String> {
    res.headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|v| v.to_str().unwrap().to_string())
        .find(|c| c.starts_with(&format!("{name}=")))
}

pub fn cookie_value(line: &str) -> String {
    line.split(';')
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1
        .to_string()
}

pub fn cookie_attributes(line: &str) -> Vec<String> {
    let mut attrs: Vec<String> = line
        .split(';')
        .skip(1)
        .map(|a| a.trim().to_ascii_lowercase())
        .collect();
    attrs.sort();
    attrs
}

pub fn location(res: &Response) -> String {
    res.headers()[header::LOCATION]
        .to_str()
        .unwrap()
        .to_string()
}

/// A sign-in started: what the browser holds and what GitHub was sent.
pub struct Started {
    pub prelogin: String,
    pub state: String,
    pub challenge: String,
    pub params: BTreeMap<String, String>,
}

pub async fn start(app: &TestApp) -> Started {
    start_at(app, "/auth/login").await
}

pub async fn start_at(app: &TestApp, uri: &str) -> Started {
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

pub async fn callback(app: &TestApp, prelogin: Option<&str>, query: &str) -> Response {
    let mut req = Request::get(format!("/auth/callback?{query}"));
    if let Some(id) = prelogin {
        req = req.header(header::COOKIE, format!("{PRELOGIN_COOKIE}={id}"));
    }
    send(app, req.body(Body::empty()).unwrap()).await
}

/// Sign in all the way; the session cookie's id.
pub async fn sign_in(app: &TestApp) -> String {
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

pub async fn call(
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

pub async fn me(app: &TestApp, session: Option<&str>) -> (StatusCode, Value) {
    let res = call(app, Method::GET, "/api/v1/me", session, None).await;
    let status = res.status();
    (status, json_body(res).await)
}

pub fn db(app: &TestApp) -> rusqlite::Connection {
    rusqlite::Connection::open(&app.db).unwrap()
}

pub fn count(app: &TestApp, sql: &str) -> i64 {
    db(app).query_row(sql, [], |row| row.get(0)).unwrap()
}

pub fn sha256_hex(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn assert_private(res: &Response) {
    let h = res.headers();
    assert_eq!(h[header::CACHE_CONTROL], "private, no-store");
    assert_eq!(h[header::VARY], "Cookie");
    assert!(h.get(header::ETAG).is_none(), "nothing to revalidate");
    assert!(
        h.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none(),
        "no other origin may read it"
    );
}
