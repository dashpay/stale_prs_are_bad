//! What the log holds across sign-in. A test binary of its own, with one
//! test: the log is captured by the process-wide subscriber, which any other
//! test in the process would write into or race with.

mod common;

use axum::http::Method;
use common::signin::*;
use common::*;
use pr_hygiene_service::auth::SESSION_COOKIE;
use pr_hygiene_service::github::GithubError;
use std::sync::{Arc, Mutex};

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
    tracing::subscriber::set_global_default(subscriber).unwrap();

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
