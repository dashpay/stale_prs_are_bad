//! "Sign in with GitHub", sessions and the signed-in person's own routes,
//! against a fake GitHub.

mod common;

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use common::signin::*;
use common::*;
use pr_hygiene_service::auth::{PRELOGIN_COOKIE, SESSION_COOKIE, SIGNED_IN, SIGNIN_FAILED};
use pr_hygiene_service::github::GithubError;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

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
    // A made-up one, shaped like ours.
    let made_up = "A".repeat(43);
    let res = callback(
        &app,
        Some(&made_up),
        &format!("code={CODE}&state={}", theirs.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert_eq!(github.calls(), []);
    assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
}

/// The pre-login is used up by any callback naming it, not only by one
/// that gets as far as comparing `state`: a callback cut short (here, with
/// no code) leaves nothing to try again.
#[tokio::test]
async fn a_callback_without_a_code_still_uses_up_the_prelogin() {
    let github = FakeGithub::new();
    let app = app_for(&github);
    let s = start(&app).await;
    let res = callback(&app, Some(&s.prelogin), &format!("state={}", s.state)).await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    let res = callback(
        &app,
        Some(&s.prelogin),
        &format!("code={CODE}&state={}", s.state),
    )
    .await;
    assert_eq!(location(&res), SIGNIN_FAILED);
    assert_eq!(github.calls(), []);
}

/// Whatever GitHub's answer, a session is keyed on a real numeric id and a
/// login of GitHub's shape, and the token is revoked all the same.
#[tokio::test]
async fn a_user_without_a_usable_id_or_login_is_not_signed_in() {
    for (id, login) in [(0, "alice"), (-1, "alice"), (42, "<!channel>"), (42, "")] {
        let github = FakeGithub::as_user(id, login);
        let app = app_for(&github);
        let s = start(&app).await;
        let res = callback(
            &app,
            Some(&s.prelogin),
            &format!("code={CODE}&state={}", s.state),
        )
        .await;
        assert_eq!(location(&res), SIGNIN_FAILED, "{id} {login:?}");
        assert!(set_cookie(&res, SESSION_COOKIE).is_none());
        assert!(github.calls().contains(&Call::Revoke(TOKEN.into())));
        assert_eq!(count(&app, "SELECT count(*) FROM sessions"), 0);
    }
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

/// Signing in again in a browser that holds a session (as someone else,
/// say) overwrites its cookie; the session it named must end with it, or
/// it would stay valid for a month with nothing left to end it.
#[tokio::test]
async fn signing_in_again_in_the_same_browser_ends_its_old_session() {
    let app = app_for(&FakeGithub::new());
    let old = sign_in(&app).await;
    let s = start(&app).await;
    let req = Request::get(format!("/auth/callback?code={CODE}&state={}", s.state))
        .header(
            header::COOKIE,
            format!("{PRELOGIN_COOKIE}={}; {SESSION_COOKIE}={old}", s.prelogin),
        )
        .body(Body::empty())
        .unwrap();
    let res = send(&app, req).await;
    assert_eq!(location(&res), SIGNED_IN);
    let new = cookie_value(&set_cookie(&res, SESSION_COOKIE).unwrap());
    assert_eq!(me(&app, Some(&old)).await.0, StatusCode::UNAUTHORIZED);
    assert_eq!(me(&app, Some(&new)).await.0, StatusCode::OK);
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
    assert!(
        res.headers().get(header::WWW_AUTHENTICATE).is_none(),
        "a cookie is missing, not a bearer token"
    );
    let (status, _) = me(&app, Some("not-a-session-id")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    // A method the route lacks is answered privately too.
    for method in [Method::PUT, Method::OPTIONS] {
        let res = call(&app, method.clone(), "/api/v1/me", None, Some(ORIGIN)).await;
        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED, "{method}");
        assert_private(&res);
    }

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
