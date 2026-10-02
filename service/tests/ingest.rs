//! `POST /ingest`: only the scheduled workflow on master, on GitHub's own
//! runners, on a first attempt, can post — and only the snapshot its own run
//! produced.

mod common;

use axum::body::{Body, Bytes};
use axum::http::{header, Request, StatusCode};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use common::*;
use http_body::Frame;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use pr_hygiene::dashboard::Dashboard;
use pr_hygiene_service::oidc::KEY_REFRESH_INTERVAL;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

/// A request body that notes whether anyone read from it.
struct Tripwire(Arc<AtomicBool>);

impl http_body::Body for Tripwire {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
        self.0.store(true, Ordering::SeqCst);
        Poll::Ready(Some(Ok(Frame::data(Bytes::from_static(&[b' '; 4096])))))
    }
}

#[tokio::test]
async fn the_scheduled_run_on_master_is_stored() {
    let app = app();
    let (status, body) = post(&app, Some(&token()), common::body(&snapshot(5))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["stored"], true);
    assert_eq!(body["stale_repos"], json!([]));
    let (status, repos) = get_json(&app, "/api/v1/repos").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(repos["commit"], SHA);
    // A manual dispatch of the same workflow on master may post too.
    let manual = sign(&with(claims(), "event_name", json!("workflow_dispatch")));
    let (status, body) = post(&app, Some(&manual), common::body(&snapshot(4))).await;
    assert_eq!(
        (status, &body["stored"]),
        (StatusCode::OK, &json!(true)),
        "{body}"
    );
}

/// Every job of the scheduled workflow carries the same repository, branch,
/// event and caller workflow — the Pages job included, which holds
/// `id-token: write` and runs third-party actions. Only the post workflow's
/// own code may post: its `job_workflow_ref` is what tells them apart.
#[tokio::test]
async fn a_token_from_another_job_of_the_same_workflow_is_forbidden() {
    let app = app();
    let caller_job = sign(&with(claims(), "job_workflow_ref", json!(WORKFLOW)));
    let (status, body) = post(&app, Some(&caller_job), common::body(&snapshot(5))).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
    assert!(body["error"].as_str().unwrap().contains("job_workflow_ref"));
}

/// Each claim that says where the token was minted, wrong on its own.
#[tokio::test]
async fn a_token_minted_anywhere_else_is_forbidden() {
    let app = app();
    let other = "dashpay/stale_prs_are_bad/.github/workflows/other.yml@refs/heads/master";
    let cases: &[(&str, &str, Value)] = &[
        ("another repository", "repository_id", json!("1")),
        (
            "a re-registered owner name",
            "repository_owner_id",
            json!("2"),
        ),
        ("a pull request's ref", "ref", json!("refs/pull/7/merge")),
        ("another branch", "ref", json!("refs/heads/feature")),
        ("a tag", "ref_type", json!("tag")),
        ("a pull_request run", "event_name", json!("pull_request")),
        (
            "a pull_request_target run",
            "event_name",
            json!("pull_request_target"),
        ),
        ("a push run", "event_name", json!("push")),
        ("a re-run", "run_attempt", json!("2")),
        ("another workflow", "workflow_ref", json!(other)),
        ("another job workflow", "job_workflow_ref", json!(other)),
        (
            "the post workflow from another branch",
            "job_workflow_ref",
            json!("dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene-post.yml@refs/heads/feature"),
        ),
        (
            "the workflow on another branch",
            "workflow_ref",
            json!("dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene.yml@refs/heads/feature"),
        ),
        (
            "a self-hosted runner",
            "runner_environment",
            json!("self-hosted"),
        ),
    ];
    for (what, claim, value) in cases {
        let token = sign(&with(claims(), claim, value.clone()));
        let (status, body) = post(&app, Some(&token), common::body(&snapshot(5))).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{what}: {body}");
        assert!(
            body["error"].as_str().unwrap().contains(claim),
            "{what}: {body}"
        );
    }
    let (status, _) = get_json(&app, "/api/v1/repos").await;
    assert_eq!(
        status,
        StatusCode::SERVICE_UNAVAILABLE,
        "nothing was stored"
    );
}

/// Tokens that are not GitHub's, not for this service, or not current.
#[tokio::test]
async fn a_token_that_is_not_valid_here_is_unauthenticated() {
    let app = app();
    let now = now();
    // Each refused for its own reason, not by accident of another check.
    let cases: Vec<(&str, String, &str)> = vec![
        (
            "wrong audience",
            sign(&with(claims(), "aud", json!("https://elsewhere.example"))),
            "wrong audience",
        ),
        (
            "an audience list",
            sign(&with(
                claims(),
                "aud",
                json!([AUDIENCE, "https://elsewhere.example"]),
            )),
            "malformed",
        ),
        (
            "wrong issuer",
            sign(&with(claims(), "iss", json!("https://evil.example"))),
            "wrong issuer",
        ),
        (
            "expired",
            sign(&with(claims(), "exp", json!(now - 120))),
            "expired",
        ),
        (
            "not valid yet",
            sign(&with(claims(), "nbf", json!(now + 120))),
            "not valid yet",
        ),
        (
            "issued too long ago, however long it claims to live",
            sign(&with(
                with(claims(), "iat", json!(now - 1200)),
                "exp",
                json!(now + 300),
            )),
            "too long ago",
        ),
        (
            "issued in the future",
            sign(&with(claims(), "iat", json!(now + 300))),
            "in the future",
        ),
        (
            "an id that is a number, not GitHub's string",
            sign(&with(claims(), "repository_id", json!(1_242_761_300u64))),
            "malformed",
        ),
        (
            "no sha",
            {
                let mut c = claims();
                c.as_object_mut().unwrap().remove("sha");
                sign(&c)
            },
            "malformed",
        ),
        (
            "a sha that is not a commit",
            sign(&with(claims(), "sha", json!("master"))),
            "not a commit",
        ),
        (
            "signed with a key GitHub never published",
            sign_with_kid(&claims(), "unknown"),
            "unknown key",
        ),
        (
            "HS256",
            {
                let mut header = Header::new(Algorithm::HS256);
                header.kid = Some(KID.into());
                jsonwebtoken::encode(&header, &claims(), &EncodingKey::from_secret(b"guess"))
                    .unwrap()
            },
            "RS256",
        ),
        (
            "alg none",
            {
                let part = |v: &Value| URL_SAFE_NO_PAD.encode(serde_json::to_vec(v).unwrap());
                format!(
                    "{}.{}.",
                    part(&json!({"alg": "none", "typ": "JWT", "kid": KID})),
                    part(&claims())
                )
            },
            "not a JWT",
        ),
        (
            "a signature that does not match the claims",
            {
                let good = token();
                let forged = URL_SAFE_NO_PAD.encode(
                    serde_json::to_vec(&with(claims(), "repository_id", json!("1"))).unwrap(),
                );
                let parts: Vec<&str> = good.split('.').collect();
                format!("{}.{forged}.{}", parts[0], parts[2])
            },
            "bad signature",
        ),
        ("not a JWT", "not-a-token".to_string(), "not a JWT"),
    ];
    for (what, token, why) in cases {
        let (status, body) = post(&app, Some(&token), common::body(&snapshot(5))).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{what}: {body}");
        let error = body["error"].as_str().unwrap();
        assert!(
            error.contains(why),
            "{what}: expected {why:?}, got {error:?}"
        );
    }
}

#[tokio::test]
async fn no_token_is_unauthenticated_and_says_how_to_authenticate() {
    let app = app();
    let req = Request::post("/ingest")
        .body(Body::from(common::body(&snapshot(5))))
        .unwrap();
    let res = send(&app, req).await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(res.headers()[header::WWW_AUTHENTICATE], "Bearer");
    let req = Request::post("/ingest")
        .header(header::AUTHORIZATION, format!("Basic {}", token()))
        .body(Body::empty())
        .unwrap();
    assert_eq!(send(&app, req).await.status(), StatusCode::UNAUTHORIZED);
}

/// The token is checked before the body is read: an unauthenticated
/// client costs no upload and learns nothing about the body checks.
#[tokio::test]
async fn the_token_is_checked_before_the_body() {
    let app = app();
    let expired = sign(&with(claims(), "exp", json!(now() - 600)));
    let cases = [
        ("no token", None),
        ("an expired token", Some(expired.as_str())),
        ("garbage", Some("not-a-token")),
    ];
    for (what, token) in cases {
        let read = Arc::new(AtomicBool::new(false));
        let mut req = Request::post("/ingest");
        if let Some(token) = token {
            req = req.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }
        let req = req.body(Body::new(Tripwire(read.clone()))).unwrap();
        assert_eq!(
            send(&app, req).await.status(),
            StatusCode::UNAUTHORIZED,
            "{what}"
        );
        assert!(
            !read.load(Ordering::SeqCst),
            "the body was read with {what}"
        );
    }
    // An endless body with a valid token is read only up to the limit.
    let read = Arc::new(AtomicBool::new(false));
    let req = Request::post("/ingest")
        .header(header::AUTHORIZATION, format!("Bearer {}", token()))
        .body(Body::new(Tripwire(read.clone())))
        .unwrap();
    assert_eq!(
        send(&app, req).await.status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert!(read.load(Ordering::SeqCst));
}

#[tokio::test]
async fn a_missing_or_oversized_body_is_refused() {
    let mut cfg = config();
    cfg.body_limit = 64 * 1024;
    let app = app_with(cfg, false, KEY_REFRESH_INTERVAL);
    let (status, body) = post(&app, Some(&token()), vec![]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");

    // The fixture is larger than 64 KB once padded; sent without a length,
    // it is cut off at the limit as it streams in.
    let mut big = snapshot(5);
    big.prs[0].title = "x".repeat(1000);
    let mut bytes = common::body(&big);
    bytes.extend(std::iter::repeat_n(b' ', 64 * 1024));
    let (status, body) = post(&app, Some(&token()), bytes.clone()).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE, "{body}");

    // Declared too large: refused before reading.
    let req = Request::post("/ingest")
        .header(header::AUTHORIZATION, format!("Bearer {}", token()))
        .header(header::CONTENT_LENGTH, bytes.len())
        .body(Body::from(bytes))
        .unwrap();
    assert_eq!(
        send(&app, req).await.status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}

#[tokio::test]
async fn a_snapshot_from_another_commit_is_refused() {
    let app = app();
    let mut d = snapshot(5);
    d.commit = Some("fedcba9876543210fedcba9876543210fedcba98".into());
    let (status, body) = post(&app, Some(&token()), common::body(&d)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(body["error"].as_str().unwrap().contains("commit"));
    d.commit = None;
    let (status, _) = post(&app, Some(&token()), common::body(&d)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

/// A future-dated snapshot would make every honest one after it "not
/// newer" and freeze ingest; one older than the run could be a capture
/// replayed under a new token.
#[tokio::test]
async fn a_snapshot_dated_outside_its_run_is_refused() {
    let app = app();
    let (status, body) = post(&app, Some(&token()), common::body(&snapshot(-5))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(body["error"].as_str().unwrap().contains("generated_at"));
    // Not in the future, but generated after its token was minted.
    let now = now();
    let earlier = sign(&with(
        with(claims(), "iat", json!(now - 240)),
        "nbf",
        json!(now - 245),
    ));
    let (status, body) = post(&app, Some(&earlier), common::body(&snapshot(1))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    assert!(
        body["error"].as_str().unwrap().contains("after its token"),
        "{body}"
    );
    let (status, body) = post(&app, Some(&token()), common::body(&snapshot(31))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{body}");
    // Inside the window: up to the job timeout before the token.
    let (status, body) = post(&app, Some(&token()), common::body(&snapshot(29))).await;
    assert_eq!(status, StatusCode::OK, "{body}");
}

#[tokio::test]
async fn a_replay_or_an_older_run_is_ignored() {
    let app = app();
    let token = token();
    let d = snapshot(5);
    ingest(&app, &d).await;
    let before = get_json(&app, "/api/v1/repos").await.1;

    let (status, body) = post(&app, Some(&token), common::body(&d)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["stored"], false, "the same snapshot again: {body}");

    let mut older = snapshot(10);
    older.prs.clear();
    let (status, body) = post(&app, Some(&token), common::body(&older)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["stored"], false, "an older run: {body}");
    assert_eq!(
        get_json(&app, "/api/v1/repos").await.1,
        before,
        "nothing changed"
    );
}

/// A token is good for one post. Whoever captured one could otherwise post
/// a body of their own naming the same commit, dated just after the real
/// one, for as long as the token lives.
#[tokio::test]
async fn a_token_is_good_for_one_post() {
    let app = app();
    let token = token();
    let (status, body) = post(&app, Some(&token), common::body(&snapshot(6))).await;
    assert_eq!(
        (status, &body["stored"]),
        (StatusCode::OK, &json!(true)),
        "{body}"
    );
    let (_, before) = get_json(&app, "/api/v1/prs").await;

    let mut forged = snapshot(5);
    forged.prs.retain(|p| p.repo != PLATFORM);
    let (status, body) = post(&app, Some(&token), common::body(&forged)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["stored"], false, "{body}");
    assert!(
        body["reason"].as_str().unwrap().contains("already used"),
        "{body}"
    );
    assert_eq!(get_json(&app, "/api/v1/prs").await.1, before);
}

/// A time the store cannot write and read back would make every later read
/// of that PR fail; a time before GitHub or after the run is not data.
#[tokio::test]
async fn a_snapshot_with_a_time_out_of_range_is_refused_and_reads_stay_healthy() {
    use chrono::TimeZone;
    let app = app();
    ingest(&app, &snapshot(10)).await;
    let far_future = chrono::Utc.with_ymd_and_hms(10_000, 1, 1, 0, 0, 0).unwrap();
    let long_ago = chrono::Utc.with_ymd_and_hms(1999, 12, 31, 0, 0, 0).unwrap();
    let cases: Vec<(&str, Dashboard)> = vec![
        ("a stage entry past year 9999", {
            let mut d = snapshot(5);
            pr_mut(&mut d, 3000).since = Some(far_future);
            d
        }),
        ("a stage entry before 2000", {
            let mut d = snapshot(5);
            pr_mut(&mut d, 3000).since = Some(long_ago);
            d
        }),
        ("an update a day after the run", {
            let mut d = snapshot(5);
            let at = d.generated_at + chrono::TimeDelta::days(1);
            pr_mut(&mut d, 3000).updated_at = Some(at);
            d
        }),
        ("a PR created past year 9999", {
            let mut d = snapshot(5);
            pr_mut(&mut d, 3000).created_at = Some(far_future);
            d
        }),
    ];
    for (what, d) in cases {
        let (status, body) = post(&app, Some(&token()), common::body(&d)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: {body}");
        let (status, body) = get_json(&app, "/api/v1/prs/dashpay/platform/3000").await;
        assert_eq!(status, StatusCode::OK, "{what}: {body}");
    }
    // A PR updated while the run was still reading is real data.
    let mut d = snapshot(5);
    let at = d.generated_at + chrono::TimeDelta::minutes(10);
    pr_mut(&mut d, 3000).updated_at = Some(at);
    ingest(&app, &d).await;
}

fn pr_mut(d: &mut Dashboard, number: u64) -> &mut pr_hygiene::dashboard::PrOut {
    d.prs.iter_mut().find(|p| p.number == number).unwrap()
}

#[tokio::test]
async fn a_snapshot_outside_the_schema_or_bounds_is_refused() {
    let app = app();
    let mut cases = vec![];
    let mut d = snapshot(5);
    d.schema_version = 2;
    cases.push(("a newer schema", common::body(&d)));
    let mut d = snapshot(5);
    d.people[0].login = "<!channel>".into();
    cases.push(("a login that is not one", common::body(&d)));
    let mut d = snapshot(5);
    d.prs[0].title = "x".repeat(2000);
    cases.push(("a title longer than any GitHub allows", common::body(&d)));
    let mut d = snapshot(5);
    d.prs[0].repo = "dashpay/unlisted".into();
    d.prs[0].key = format!("dashpay/unlisted#{}", d.prs[0].number);
    cases.push(("a PR of a repository it does not list", common::body(&d)));
    cases.push(("not JSON", b"{".to_vec()));
    for (what, body) in cases {
        let (status, res) = post(&app, Some(&token()), body).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: {res}");
    }
}

/// Without GitHub's keys nothing can be verified, and nothing is accepted.
#[tokio::test]
async fn without_signing_keys_every_token_is_refused() {
    let app = app_with(config(), true, KEY_REFRESH_INTERVAL);
    let (status, body) = post(&app, Some(&token()), common::body(&snapshot(5))).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
    let (status, ready) = get_json(&app, "/readyz").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(ready["jwks"]["keys"], 0);
}

/// Tokens with made-up key ids must not make the service hammer GitHub: an
/// unknown key id refetches the key set at most once a minute.
#[tokio::test]
async fn an_unknown_key_id_refetches_at_most_once_a_minute() {
    let app = app();
    for _ in 0..3 {
        let (status, _) = post(
            &app,
            Some(&sign_with_kid(&claims(), "made-up")),
            common::body(&snapshot(8)),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    assert_eq!(app.keys.fetches.load(Ordering::SeqCst), 1);
    // The one fetch loaded the real keys; a real token needs no other.
    ingest(&app, &snapshot(7)).await;
    assert_eq!(app.keys.fetches.load(Ordering::SeqCst), 1);
}

/// A failed refetch keeps the keys already held: GitHub being briefly
/// unreachable must not stop ingest.
#[tokio::test]
async fn keys_already_held_survive_a_failed_refetch() {
    let app = app_with(config(), false, std::time::Duration::ZERO);
    ingest(&app, &snapshot(9)).await;
    app.keys.failing.store(true, Ordering::SeqCst);
    let (status, _) = post(
        &app,
        Some(&sign_with_kid(&claims(), "rotated")),
        common::body(&snapshot(8)),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(app.keys.fetches.load(Ordering::SeqCst), 2, "it did try");
    ingest(&app, &snapshot(7)).await;
    let (status, ready) = get_json(&app, "/readyz").await;
    assert_eq!(status, StatusCode::OK, "{ready}");
}
