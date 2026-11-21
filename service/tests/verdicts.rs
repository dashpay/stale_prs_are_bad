//! `POST /ingest/verdicts`: each governed repository's PR Hygiene run pushes
//! the verdicts it just computed, and the board shows the newest verdict of
//! each PR over the latest snapshot until a newer snapshot arrives.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use chrono::{DateTime, TimeDelta, Utc};
use common::*;
use pr_hygiene_service::store::Reader;
use serde_json::{json, Value};

/// The reusable PR Hygiene workflow every governed repository calls: the
/// `job_workflow_ref` of the job that pushes.
const REUSABLE: &str =
    "dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@refs/heads/master";
const PLATFORM_ID: &str = "424232911";
const DASHCORE_ID: &str = "467841155";
const HEAD: &str = "89abcdef0123456789abcdef0123456789abcdef";

/// The open PRs of `dashpay/platform` in the snapshot fixture.
const PLATFORM_OPEN: [u64; 11] = [
    1234, 1240, 2001, 2988, 3000, 3001, 5000, 6000, 7000, 8000, 9100,
];

/// The claims GitHub puts in the token of a governed repository's PR
/// Hygiene job: a `pull_request_target` run on one of its branches, which
/// may be a re-run on a self-hosted runner.
fn verdict_claims(repo: &str, id: &str) -> Value {
    let mut c = claims();
    c["sub"] = json!(format!("repo:{repo}:pull_request"));
    c["repository"] = json!(repo);
    c["repository_id"] = json!(id);
    c["ref"] = json!("refs/heads/v3.0-dev");
    c["workflow_ref"] = json!(format!(
        "{repo}/.github/workflows/pr-review.yml@refs/heads/v3.0-dev"
    ));
    c["job_workflow_ref"] = json!(REUSABLE);
    c["event_name"] = json!("pull_request_target");
    c["runner_environment"] = json!("self-hosted");
    c["run_attempt"] = json!("2");
    c["sha"] = json!(HEAD);
    c
}

fn platform_token() -> String {
    sign(&verdict_claims(PLATFORM, PLATFORM_ID))
}

fn minutes_ago(m: i64) -> DateTime<Utc> {
    generated(m)
}

fn push(repo: &str, read_at: DateTime<Utc>, open: &[u64], verdicts: &[(u64, Value)]) -> Value {
    json!({
        "repo": repo,
        "read_at": read_at,
        "open": open,
        "verdicts": verdicts
            .iter()
            .map(|(number, verdict)| json!({"number": number, "head": HEAD, "verdict": verdict}))
            .collect::<Vec<_>>(),
    })
}

async fn post_verdicts(app: &TestApp, token: Option<&str>, body: Vec<u8>) -> (StatusCode, Value) {
    let mut req =
        Request::post("/ingest/verdicts").header(header::CONTENT_TYPE, "application/json");
    if let Some(token) = token {
        req = req.header(header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let res = send(app, req.body(Body::from(body)).unwrap()).await;
    let status = res.status();
    (status, json_body(res).await)
}

/// Push `body` for `dashpay/platform` with a fresh token; panics unless it
/// was accepted.
async fn push_platform(app: &TestApp, body: &Value) -> Value {
    let (status, res) = post_verdicts(app, Some(&platform_token()), body.to_string().into()).await;
    assert_eq!(status, StatusCode::OK, "{res}");
    assert_eq!(res["stored"], true, "{res}");
    res
}

/// The engine's verdict on a PR that waits on nothing but the merge.
fn mergeable() -> Value {
    json!({
        "state": "ready-to-merge", "status": "success", "blockers": [],
        "reviewers": [], "objectors": [], "areas": ["fallback"],
        "approvals": [{"area": "fallback", "approvers": ["QuantumExplorer", "alice"],
                       "approved_by": ["QuantumExplorer"], "owned": false}],
        "title": "PR 3000", "author": "carol", "ready_since": null,
    })
}

/// The engine's verdict on a PR waiting on review: `approvers` may approve
/// the fallback area, and nobody has.
fn in_review(approvers: &[&str]) -> Value {
    json!({
        "state": "ready-for-human", "status": "pending",
        "blockers": ["Waiting for an approval of fallback"],
        "reviewers": approvers, "objectors": [], "areas": ["fallback"],
        "approvals": [{"area": "fallback", "approvers": approvers,
                       "approved_by": [], "owned": false}],
        "title": "a PR", "author": "someone", "ready_since": null,
    })
}

async fn dashboard(app: &TestApp) -> Value {
    let (status, d) = get_json(app, "/dashboard.json").await;
    assert_eq!(status, StatusCode::OK, "{d}");
    d
}

fn pr<'a>(d: &'a Value, key: &str) -> &'a Value {
    d["prs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["key"] == key)
        .unwrap_or_else(|| panic!("{key} is not shown"))
}

fn has_pr(d: &Value, key: &str) -> bool {
    d["prs"].as_array().unwrap().iter().any(|p| p["key"] == key)
}

fn person_mut<'a>(d: &'a mut Value, login: &str) -> &'a mut Value {
    d["people"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["login"] == login)
        .unwrap_or_else(|| panic!("{login} is not on the board"))
}

fn person<'a>(d: &'a Value, login: &str) -> &'a Value {
    d["people"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["login"] == login)
        .unwrap_or_else(|| panic!("{login} is not on the board"))
}

fn pr_mut<'a>(d: &'a mut Value, key: &str) -> &'a mut Value {
    d["prs"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|p| p["key"] == key)
        .unwrap_or_else(|| panic!("{key} is not shown"))
}

fn owes(d: &Value, login: &str) -> Vec<String> {
    person(d, login)["owes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["pr"].as_str().unwrap().to_string())
        .collect()
}

// --- Who may push -----------------------------------------------------------

/// One repository's run must not be able to speak for another: the body's
/// repository is bound to the token's.
#[tokio::test]
async fn a_token_from_one_repository_posting_another_is_forbidden() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    let dashcore_token = sign(&verdict_claims(DASHCORE, DASHCORE_ID));
    let body = push(
        PLATFORM,
        minutes_ago(1),
        &PLATFORM_OPEN,
        &[(3000, mergeable())],
    );
    let (status, res) = post_verdicts(&app, Some(&dashcore_token), body.to_string().into()).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{res}");
    assert_eq!(dashboard(&app).await, before, "nothing was stored");
}

/// Only the reusable workflow at master may push. Any job a writer adds to
/// a caller carries the caller's own ref; a SHA-pinned caller fails closed.
#[tokio::test]
async fn a_wrong_job_workflow_ref_is_forbidden() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    for wrong in [
        "dashpay/platform/.github/workflows/pr-review.yml@refs/heads/v3.0-dev",
        "dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@refs/heads/feature",
        "dashpay/stale_prs_are_bad/.github/workflows/pr-review-reusable.yml@0123456789abcdef0123456789abcdef01234567",
        "dashpay/stale_prs_are_bad/.github/workflows/pr-hygiene-post.yml@refs/heads/master",
    ] {
        let token = sign(&with(
            verdict_claims(PLATFORM, PLATFORM_ID),
            "job_workflow_ref",
            json!(wrong),
        ));
        let body = push(PLATFORM, minutes_ago(1), &PLATFORM_OPEN, &[(3000, mergeable())]);
        let (status, res) = post_verdicts(&app, Some(&token), body.to_string().into()).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{wrong}: {res}");
        assert!(res["error"].as_str().unwrap().contains("job_workflow_ref"));
    }
    assert_eq!(dashboard(&app).await, before, "nothing was stored");
}

/// The repository is known by its id, under dashpay, and the token's name
/// for it must be the one configured for that id.
#[tokio::test]
async fn a_token_from_anywhere_but_a_governed_repository_is_forbidden() {
    let app = app();
    let cases: Vec<(&str, Value)> = vec![
        ("an id not governed", verdict_claims(PLATFORM, "1242761300")),
        (
            "a name that is not the id's",
            verdict_claims(DASHCORE, PLATFORM_ID),
        ),
        (
            "another owner",
            with(
                verdict_claims(PLATFORM, PLATFORM_ID),
                "repository_owner_id",
                json!("2"),
            ),
        ),
        // The scheduled snapshot job may post snapshots, not verdicts.
        ("the snapshot job", claims()),
    ];
    for (what, claims) in cases {
        let body = push(PLATFORM, minutes_ago(1), &PLATFORM_OPEN, &[]);
        let (status, res) =
            post_verdicts(&app, Some(&sign(&claims)), body.to_string().into()).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{what}: {res}");
    }
    let (status, _) = post_verdicts(
        &app,
        None,
        push(PLATFORM, minutes_ago(1), &PLATFORM_OPEN, &[])
            .to_string()
            .into(),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

/// A verdict token cannot post a snapshot either.
#[tokio::test]
async fn a_verdict_token_cannot_post_a_snapshot() {
    let app = app();
    let (status, res) = post(&app, Some(&platform_token()), body(&snapshot(5))).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{res}");
}

#[tokio::test]
async fn a_verdict_token_posts_once() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let token = platform_token();
    let body = push(PLATFORM, minutes_ago(2), &PLATFORM_OPEN, &[]);
    let (status, res) = post_verdicts(&app, Some(&token), body.to_string().into()).await;
    assert_eq!(
        (status, &res["stored"]),
        (StatusCode::OK, &json!(true)),
        "{res}"
    );
    let later = push(
        PLATFORM,
        minutes_ago(1),
        &PLATFORM_OPEN,
        &[(3000, mergeable())],
    );
    let (status, res) = post_verdicts(&app, Some(&token), later.to_string().into()).await;
    assert_eq!(status, StatusCode::OK, "{res}");
    assert_eq!(res["stored"], false, "{res}");
    assert_eq!(
        pr(&dashboard(&app).await, "dashpay/platform#3000")["stage"],
        "review"
    );
}

// --- What a push may hold ---------------------------------------------------

#[tokio::test]
async fn a_push_outside_its_limits_is_refused() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    let many: Vec<u64> = (1..=1001).collect();
    let token_iat = minutes_ago(0);
    let cases: Vec<(&str, Value)> = vec![
        (
            "more than 1,000 open PRs",
            push(PLATFORM, minutes_ago(1), &many, &[]),
        ),
        (
            "more than 1,000 verdicts",
            push(
                PLATFORM,
                minutes_ago(1),
                &many[..1000],
                &many.iter().map(|n| (*n, mergeable())).collect::<Vec<_>>(),
            ),
        ),
        (
            "a verdict on a PR not listed open",
            push(PLATFORM, minutes_ago(1), &[1234], &[(3000, mergeable())]),
        ),
        (
            "read in the future",
            push(
                PLATFORM,
                Utc::now() + TimeDelta::minutes(5),
                &PLATFORM_OPEN,
                &[],
            ),
        ),
        (
            "read over an hour before the token",
            push(
                PLATFORM,
                token_iat - TimeDelta::minutes(61),
                &PLATFORM_OPEN,
                &[],
            ),
        ),
        (
            "a reviewer that is not a login",
            push(
                PLATFORM,
                minutes_ago(1),
                &PLATFORM_OPEN,
                &[(3000, in_review(&["<!channel>"]))],
            ),
        ),
        (
            "a blocker too long",
            push(
                PLATFORM,
                minutes_ago(1),
                &PLATFORM_OPEN,
                &[(3000, {
                    let mut v = mergeable();
                    v["blockers"] = json!(["x".repeat(5000)]);
                    v
                })],
            ),
        ),
        ("a head that is not a commit", {
            let mut b = push(
                PLATFORM,
                minutes_ago(1),
                &PLATFORM_OPEN,
                &[(3000, mergeable())],
            );
            b["verdicts"][0]["head"] = json!("master");
            b
        }),
        (
            "a PR listed twice",
            push(
                PLATFORM,
                minutes_ago(1),
                &PLATFORM_OPEN,
                &[(3000, mergeable()), (3000, mergeable())],
            ),
        ),
        ("not a push", json!({"repo": PLATFORM})),
    ];
    for (what, body) in cases {
        let (status, res) =
            post_verdicts(&app, Some(&platform_token()), body.to_string().into()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{what}: {res}");
    }
    let huge = vec![b' '; 1024 * 1024 + 1];
    let (status, res) = post_verdicts(&app, Some(&platform_token()), huge).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE, "{res}");
    assert_eq!(dashboard(&app).await, before, "nothing was stored");
}

// --- The overlay ------------------------------------------------------------

/// Deploying this must change nothing until the first push arrives.
#[tokio::test]
async fn with_no_pushes_the_view_is_the_stored_snapshot_view() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let stored = Reader::open(&app.db).unwrap().view().unwrap().unwrap();
    let d = dashboard(&app).await;
    assert_eq!(d["prs"], serde_json::to_value(&stored.prs).unwrap());
    assert_eq!(d["people"], serde_json::to_value(&stored.people).unwrap());
    assert_eq!(d["repos"], serde_json::to_value(&stored.repos).unwrap());
}

/// The pushed verdict replaces only what the verdict decides: the PR's
/// stage and the asks that go with it, and who owes it a review.
#[tokio::test]
async fn a_push_newer_than_the_snapshot_changes_that_prs_stage_asks_and_owes_and_nothing_else() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    let read_at = minutes_ago(1);
    push_platform(
        &app,
        &push(PLATFORM, read_at, &PLATFORM_OPEN, &[(3000, mergeable())]),
    )
    .await;
    let after = dashboard(&app).await;

    let mut expected = before.clone();
    let p = pr_mut(&mut expected, "dashpay/platform#3000");
    p["stage"] = json!("mergeable");
    p["engine_state"] = json!("ready-to-merge");
    p["blockers"] = json!([]);
    p["next_action"] = Value::Null;
    p["asks"] = json!([]);
    p["objectors"] = json!([]);
    p["since"] = json!(read_at);
    p["since_basis"] = json!("engine");
    p["lateness"] = json!("ok");
    person_mut(&mut expected, "alice")["owes"] = json!([]);
    person_mut(&mut expected, "QuantumExplorer")["owes"]
        .as_array_mut()
        .unwrap()
        .retain(|o| o["pr"] != "dashpay/platform#3000");
    assert_eq!(after, expected);
}

#[tokio::test]
async fn a_draft_marked_ready_after_the_snapshot_leaves_draft() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    assert_eq!(pr(&before, "dashpay/platform#6000")["stage"], "draft");
    assert_eq!(person(&before, "bob")["wip"][PLATFORM], 1);
    push_platform(
        &app,
        &push(
            PLATFORM,
            minutes_ago(1),
            &PLATFORM_OPEN,
            &[(6000, in_review(&["alice"]))],
        ),
    )
    .await;
    let after = dashboard(&app).await;
    let p = pr(&after, "dashpay/platform#6000");
    assert_eq!(p["stage"], "review");
    assert_eq!(p["draft"], false);
    assert_eq!(
        p["asks"],
        json!([{"area": "fallback", "approvers": ["alice"]}])
    );
    assert!(owes(&after, "alice").contains(&"dashpay/platform#6000".to_string()));
    assert_eq!(
        person(&after, "bob")["wip"][PLATFORM],
        2,
        "now holds a slot"
    );
}

/// Back in the stage the snapshot showed, the PR's time in it starts at its
/// return, not at the snapshot's older entry.
#[tokio::test]
async fn a_review_mergeable_review_round_trip_takes_its_new_since() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let snapshot_since = pr(&dashboard(&app).await, "dashpay/platform#3000")["since"].clone();
    let first = minutes_ago(20);
    push_platform(
        &app,
        &push(PLATFORM, first, &PLATFORM_OPEN, &[(3000, mergeable())]),
    )
    .await;
    let d = dashboard(&app).await;
    assert_eq!(pr(&d, "dashpay/platform#3000")["stage"], "mergeable");
    assert_eq!(pr(&d, "dashpay/platform#3000")["since"], json!(first));

    let back = minutes_ago(10);
    push_platform(
        &app,
        &push(
            PLATFORM,
            back,
            &PLATFORM_OPEN,
            &[(3000, in_review(&["QuantumExplorer", "alice"]))],
        ),
    )
    .await;
    let d = dashboard(&app).await;
    let p = pr(&d, "dashpay/platform#3000");
    assert_eq!(p["stage"], "review");
    assert_ne!(p["since"], snapshot_since);
    assert_eq!(p["since"], json!(back));
    assert_eq!(p["since_basis"], "engine");
}

/// A run that read GitHub earlier but finished later must not undo a newer
/// verdict.
#[tokio::test]
async fn an_older_push_after_a_newer_one_is_ignored() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    push_platform(
        &app,
        &push(
            PLATFORM,
            minutes_ago(5),
            &PLATFORM_OPEN,
            &[(3000, mergeable())],
        ),
    )
    .await;
    push_platform(
        &app,
        &push(
            PLATFORM,
            minutes_ago(10),
            &PLATFORM_OPEN[1..],
            &[(3000, in_review(&["alice"]))],
        ),
    )
    .await;
    let d = dashboard(&app).await;
    assert_eq!(pr(&d, "dashpay/platform#3000")["stage"], "mergeable");
    assert!(
        has_pr(&d, "dashpay/platform#1234"),
        "an older open list does not replace a newer one"
    );
}

#[tokio::test]
async fn a_newer_snapshot_wins_over_an_older_push() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    push_platform(
        &app,
        &push(
            PLATFORM,
            minutes_ago(20),
            &PLATFORM_OPEN[1..],
            &[(3000, mergeable())],
        ),
    )
    .await;
    let d = dashboard(&app).await;
    assert_eq!(pr(&d, "dashpay/platform#3000")["stage"], "mergeable");
    assert!(!has_pr(&d, "dashpay/platform#1234"));

    let newer = snapshot(10);
    ingest(&app, &newer).await;
    let d = dashboard(&app).await;
    let stored = Reader::open(&app.db).unwrap().view().unwrap().unwrap();
    assert_eq!(pr(&d, "dashpay/platform#3000")["stage"], "review");
    assert!(has_pr(&d, "dashpay/platform#1234"));
    assert_eq!(d["prs"], serde_json::to_value(&stored.prs).unwrap());
    assert_eq!(d["people"], serde_json::to_value(&stored.people).unwrap());
}

/// A PR closed since the snapshot leaves the board at once, and with it
/// every review owed on it and its author's slot.
#[tokio::test]
async fn a_pr_missing_from_a_newer_open_list_is_hidden_everywhere() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    assert_eq!(person(&before, "alice")["wip"][PLATFORM], 3);
    assert_eq!(person(&before, "carol")["wip"][PLATFORM], 3);
    let open: Vec<u64> = PLATFORM_OPEN
        .into_iter()
        .filter(|n| ![1234, 3000].contains(n))
        .collect();
    push_platform(&app, &push(PLATFORM, minutes_ago(1), &open, &[])).await;
    let d = dashboard(&app).await;
    for gone in ["dashpay/platform#1234", "dashpay/platform#3000"] {
        assert!(!has_pr(&d, gone), "{gone}");
        for p in d["people"].as_array().unwrap() {
            assert!(
                !p["owes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|o| o["pr"] == gone),
                "{gone} still owed by {}",
                p["login"]
            );
            assert!(
                !p["authored"].as_array().unwrap().contains(&json!(gone)),
                "{gone} still authored by {}",
                p["login"]
            );
        }
    }
    assert_eq!(person(&d, "alice")["wip"][PLATFORM], 2);
    assert_eq!(person(&d, "carol")["wip"][PLATFORM], 2);
    assert_eq!(owes(&d, "QuantumExplorer"), vec!["dashpay/platform#9100"]);
    assert!(
        has_pr(&d, "dashpay/rust-dashcore#101"),
        "another repository's PRs stay"
    );
}

/// The view is cached by version; a push that only closes PRs must still
/// replace what is served.
#[tokio::test]
async fn a_push_that_changes_only_the_open_list_refreshes_the_cache() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    assert!(has_pr(&dashboard(&app).await, "dashpay/platform#1234"));
    push_platform(
        &app,
        &push(PLATFORM, minutes_ago(1), &PLATFORM_OPEN[1..], &[]),
    )
    .await;
    assert!(!has_pr(&dashboard(&app).await, "dashpay/platform#1234"));
}

/// A PR opened since the snapshot waits for the next one.
#[tokio::test]
async fn a_pushed_pr_not_in_the_snapshot_is_ignored() {
    let app = app();
    ingest(&app, &snapshot(30)).await;
    let before = dashboard(&app).await;
    let mut open = PLATFORM_OPEN.to_vec();
    open.push(9999);
    push_platform(
        &app,
        &push(PLATFORM, minutes_ago(1), &open, &[(9999, mergeable())]),
    )
    .await;
    assert_eq!(dashboard(&app).await, before);
}
