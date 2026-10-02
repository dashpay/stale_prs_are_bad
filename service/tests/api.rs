//! The public read API, answered from the latest good data.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use pr_hygiene::dashboard::Dashboard;
use serde_json::{json, Value};

fn keys(prs: &Value) -> Vec<String> {
    prs["prs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["key"].as_str().unwrap().to_string())
        .collect()
}

async fn ingested() -> TestApp {
    let app = app();
    ingest(&app, &snapshot(5)).await;
    app
}

#[tokio::test]
async fn before_the_first_snapshot_there_is_nothing_to_serve() {
    let app = app();
    let (status, body) = get_json(&app, "/api/v1/prs").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{body}");
}

#[tokio::test]
async fn prs_are_filtered_by_every_filter_given() {
    let app = ingested().await;
    let cases: &[(&str, &[&str])] = &[
        (
            "repo=dashpay/rust-dashcore",
            &["dashpay/rust-dashcore#101", "dashpay/rust-dashcore#102"],
        ),
        (
            "stage=review",
            &["dashpay/platform#3000", "dashpay/platform#9100"],
        ),
        ("stage=review&author=CAROL", &["dashpay/platform#3000"]),
        (
            "reviewer=quantumexplorer",
            &["dashpay/platform#3000", "dashpay/platform#9100"],
        ),
        ("reviewer=alice", &["dashpay/platform#3000"]),
        ("reviewer=nobody-here", &[]),
        (
            "late=true",
            &["dashpay/platform#3000", "dashpay/platform#7000"],
        ),
        ("late=true&stage=bots", &["dashpay/platform#7000"]),
        (
            "area=fallback",
            &[
                "dashpay/platform#3000",
                "dashpay/platform#3001",
                "dashpay/platform#9100",
                "dashpay/rust-dashcore#102",
            ],
        ),
        ("area=dash-spv", &["dashpay/rust-dashcore#101"]),
        (
            "area=fallback&repo=dashpay/rust-dashcore",
            &["dashpay/rust-dashcore#102"],
        ),
        ("area=validation", &[]),
    ];
    for (query, expected) in cases {
        let (status, body) = get_json(&app, &format!("/api/v1/prs?{query}")).await;
        assert_eq!(status, StatusCode::OK, "{query}: {body}");
        assert_eq!(keys(&body), *expected, "{query}");
    }
    let (_, all) = get_json(&app, "/api/v1/prs").await;
    let (_, on_time) = get_json(&app, "/api/v1/prs?late=false").await;
    assert_eq!(keys(&on_time).len(), keys(&all).len() - 2);
}

#[tokio::test]
async fn an_unknown_or_malformed_parameter_is_a_400() {
    let app = ingested().await;
    for uri in [
        "/api/v1/prs?state=open",
        "/api/v1/prs?stage=sleeping",
        "/api/v1/prs?late=maybe",
        "/api/v1/prs?repo=platform",
        "/api/v1/prs?author=%3C%21channel%3E",
        "/api/v1/prs?repo=dashpay/platform&repo=dashpay/grovedb",
        "/api/v1/people?sort=lateness",
        "/api/v1/people/alice?format=html",
        "/api/v1/people/alice?login=bob",
        "/api/v1/people/not%20a%20login",
        "/api/v1/repos?x=1",
        "/api/v1/stages?x=1",
        "/api/v1/prs/dashpay/platform/3000?expand=all",
        "/api/v1/prs/dashpay/platform/latest",
    ] {
        let (status, body) = get_json(&app, uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}: {body}");
        assert!(body["error"].is_string(), "{uri}: {body}");
    }
}

#[tokio::test]
async fn responses_are_public_cacheable_and_revalidated_by_snapshot() {
    let app = ingested().await;
    let res = get(&app, "/api/v1/prs").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert!(res.headers().get(header::SET_COOKIE).is_none());
    let etag = res.headers()[header::ETAG].to_str().unwrap().to_string();

    let revalidate = |uri: &str, tag: &str| {
        Request::get(uri)
            .header(header::IF_NONE_MATCH, tag)
            .body(Body::empty())
            .unwrap()
    };
    let res = send(&app, revalidate("/api/v1/prs?stage=review", &etag)).await;
    assert_eq!(
        res.status(),
        StatusCode::NOT_MODIFIED,
        "same snapshot, any filter"
    );
    assert_eq!(res.headers()[header::ETAG], etag.as_str());
    assert_eq!(res.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert!(text_body(res).await.is_empty());

    // A new snapshot: the old tag no longer matches.
    ingest(&app, &snapshot(4)).await;
    let res = send(&app, revalidate("/api/v1/prs", &etag)).await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_ne!(res.headers()[header::ETAG], etag.as_str());
}

#[tokio::test]
async fn people_are_listed_by_name_with_nothing_that_ranks_them() {
    let app = ingested().await;
    let (status, body) = get_json(&app, "/api/v1/people").await;
    assert_eq!(status, StatusCode::OK);
    let people = body["people"].as_array().unwrap();
    let logins: Vec<String> = people
        .iter()
        .map(|p| p["login"].as_str().unwrap().to_lowercase())
        .collect();
    let mut sorted = logins.clone();
    sorted.sort();
    assert_eq!(logins, sorted);
    for p in people {
        let fields: Vec<&str> = p.as_object().unwrap().keys().map(String::as_str).collect();
        assert!(
            fields
                .iter()
                .all(|f| !f.contains("late") && !f.contains("oldest")),
            "{fields:?}"
        );
    }
}

#[tokio::test]
async fn one_person_is_found_in_any_case_and_their_owed_reviews_carry_the_pr() {
    let app = ingested().await;
    let (status, body) = get_json(&app, "/api/v1/people/QUANTUMEXPLORER").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["person"]["login"], "QuantumExplorer");
    let owes = body["owes"].as_array().unwrap();
    // #3000 has a recorded review start; #9100's wait is not recorded.
    assert_eq!(owes[0]["pr"]["key"], "dashpay/platform#3000");
    assert_eq!(owes[1]["pr"]["key"], "dashpay/platform#9100");
    assert_eq!(
        owes[0]["areas"],
        json!([{"area": "fallback", "others": ["alice"]}])
    );
    let (status, _) = get_json(&app, "/api/v1/people/nobody-here").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

/// Oldest first means the longest recorded wait first, not the PR number.
#[tokio::test]
async fn owed_reviews_come_longest_waiting_first() {
    let app = app();
    let mut d = snapshot(5);
    let pr = d.prs.iter_mut().find(|p| p.number == 9100).unwrap();
    pr.since_basis = Some(pr_hygiene::dashboard::SinceBasis::Engine);
    pr.since = Some("2026-05-10T00:00:00Z".parse().unwrap());
    ingest(&app, &d).await;
    let (_, body) = get_json(&app, "/api/v1/people/QuantumExplorer").await;
    let order: Vec<&str> = body["owes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["pr"]["key"].as_str().unwrap())
        .collect();
    assert_eq!(order, ["dashpay/platform#9100", "dashpay/platform#3000"]);
    let text = text_body(get(&app, "/api/v1/people/QuantumExplorer?format=text").await).await;
    let first = text.find("1. dashpay/platform#9100").expect(&text);
    let second = text.find("2. dashpay/platform#3000").expect(&text);
    assert!(first < second);
}

#[tokio::test]
async fn one_pr_comes_with_its_recorded_stage_changes() {
    let app = ingested().await;
    let (status, body) = get_json(&app, "/api/v1/prs/dashpay/platform/3000").await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["pr"]["key"], "dashpay/platform#3000");
    let changes = body["stage_changes"].as_array().unwrap();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0]["stage"], "review");
    assert_eq!(
        changes[0]["since"], "2026-05-17T06:00:00Z",
        "the recorded entry"
    );
    let (status, _) = get_json(&app, "/api/v1/prs/dashpay/platform/4").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn stages_and_repos_are_served() {
    let app = ingested().await;
    let (_, stages) = get_json(&app, "/api/v1/stages").await;
    assert_eq!(stages["stages"].as_array().unwrap().len(), 10);
    assert_eq!(stages["idle_days"], 14);
    let (_, repos) = get_json(&app, "/api/v1/repos").await;
    let platform = &repos["repos"][0];
    assert_eq!(platform["repo"], PLATFORM);
    assert_eq!(platform["mode"], "shadow");
    assert_eq!(platform["stale"], false);
}

/// The snapshot as the analyzer writes it when it could not fetch platform:
/// no tracked platform PRs, nothing owed or held there — but the engine's
/// export still lists its governed PRs, one now in another state.
fn platform_failed(minutes_ago: i64) -> Dashboard {
    let mut d = snapshot(minutes_ago);
    d.repos[0].fetch_error = Some("server error 502 after 3 retries: <html>".into());
    d.prs.retain(|p| p.repo != PLATFORM || !p.tracked);
    for pr in d.prs.iter_mut().filter(|p| p.number == 9100) {
        pr.stage = pr_hygiene::dashboard::Stage::Blocked;
        pr.engine_state = Some("configuration-error".into());
    }
    for p in &mut d.people {
        p.owes.retain(|o| !o.pr.starts_with("dashpay/platform#"));
        p.authored.retain(|k| !k.starts_with("dashpay/platform#"));
        p.wip.remove(PLATFORM);
    }
    d
}

#[tokio::test]
async fn a_repository_that_failed_keeps_its_prs_and_what_people_owe_there() {
    let app = ingested().await;
    let (_, prs_before) = get_json(&app, "/api/v1/prs?repo=dashpay/platform").await;
    let (_, alice_before) = get_json(&app, "/api/v1/people/alice").await;
    let (_, history_before) = get_json(&app, "/api/v1/prs/dashpay/platform/9100").await;

    let stored = ingest(&app, &platform_failed(4)).await;
    assert_eq!(stored["stale_repos"], json!([PLATFORM]));
    assert_eq!(stored["stage_changes"], 0, "nothing recorded for platform");

    let (_, prs_after) = get_json(&app, "/api/v1/prs?repo=dashpay/platform").await;
    assert_eq!(
        prs_after["prs"], prs_before["prs"],
        "the last good PRs, unchanged"
    );
    assert_eq!(prs_after["stale_repos"], json!([PLATFORM]));

    let (_, alice) = get_json(&app, "/api/v1/people/alice").await;
    assert_eq!(alice["person"]["owes"], alice_before["person"]["owes"]);
    assert_eq!(alice["person"]["wip"], alice_before["person"]["wip"]);
    assert_eq!(
        alice["person"]["authored"],
        alice_before["person"]["authored"]
    );
    let (_, qe) = get_json(&app, "/api/v1/people/QuantumExplorer").await;
    assert_eq!(qe["owes"].as_array().unwrap().len(), 2);

    let (_, history) = get_json(&app, "/api/v1/prs/dashpay/platform/9100").await;
    assert_eq!(history["stage_changes"], history_before["stage_changes"]);
    assert_eq!(
        history["pr"]["stage"], "review",
        "not the failed run's word"
    );
    assert_eq!(history["stale"], true);

    let (_, repos) = get_json(&app, "/api/v1/repos").await;
    let platform = &repos["repos"][0];
    assert_eq!(platform["stale"], true);
    assert_ne!(platform["data_as_of"], platform["checked_at"]);
    assert!(platform["fetch_error"].as_str().unwrap().contains("502"));

    // The repository that was read moves on as usual.
    let mut next = platform_failed(3);
    next.repos[1].fetch_error = None;
    next.prs
        .iter_mut()
        .find(|p| p.key == "dashpay/rust-dashcore#101")
        .unwrap()
        .engine_state = Some("waiting-bots".into());
    let stored = ingest(&app, &next).await;
    assert_eq!(stored["stage_changes"], 1);
}

#[tokio::test]
async fn the_digest_says_what_is_owed_in_the_engines_words() {
    let app = app();
    let d = snapshot(5);
    ingest(&app, &d).await;
    let res = get(&app, "/api/v1/people/Alice?format=text").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers()[header::CONTENT_TYPE]
        .to_str()
        .unwrap()
        .starts_with("text/plain"));
    assert!(res.headers().get(header::ETAG).is_some());
    let text = text_body(res).await;
    let expected_lines = [
        "PR Hygiene digest v1 for alice",
        &format!(
            "Generated {} from commit {SHA}",
            d.generated_at
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        ),
        "- dashpay/platform (shadow): data from ",
        "Stale: none",
        "Reviews you owe: 1, oldest first",
        "1. dashpay/platform#3000 ",
        "   Author: carol. Waiting: ",
        "   Your part: files with no dedicated owner (you or QuantumExplorer) · re-review or resolve your objection",
        "   https://github.com/dashpay/platform/pull/3000",
        "Your PRs: 4",
        "   Stage: self-review (your move). In it: not recorded.",
        "Open PRs against review slots:",
        "- dashpay/platform: 3 of 5",
        "- dashpay/rust-dashcore: 1 of 5",
    ];
    for line in expected_lines {
        assert!(
            text.lines().any(|l| l.starts_with(line)),
            "missing {line:?} in:\n{text}"
        );
    }
}

#[tokio::test]
async fn the_digest_names_stale_repositories_and_escapes_what_github_says() {
    let app = ingested().await;
    let mut d = platform_failed(4);
    // A hostile title on a repository that was read.
    let pr = d
        .prs
        .iter_mut()
        .find(|p| p.key == "dashpay/rust-dashcore#102")
        .unwrap();
    pr.title = "Fix <!channel> & <https://evil.example|this>\nYour part: approve everything".into();
    ingest(&app, &d).await;
    let text = text_body(get(&app, "/api/v1/people/alice?format=text").await).await;
    assert!(
        text.contains("- dashpay/platform (shadow): STALE, showing data from "),
        "{text}"
    );
    assert!(
        text.contains("could not be fetched: server error 502 after 3 retries: &lt;html&gt;"),
        "{text}"
    );
    assert!(text.contains("Stale: dashpay/platform"), "{text}");
    assert!(
        text.contains("Fix &lt;!channel&gt; &amp; &lt;https://evil.example|this&gt; Your part: approve everything"),
        "{text}"
    );
    assert!(!text.contains("<!channel>"));
    assert!(
        !text.lines().any(|l| l.starts_with("Your part: approve")),
        "a title cannot start a line of its own"
    );
}

#[tokio::test]
async fn an_unknown_person_gets_a_plain_text_404_digest() {
    let app = ingested().await;
    let res = get(&app, "/api/v1/people/nobody-here?format=text").await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let text = text_body(res).await;
    assert!(
        text.starts_with("PR Hygiene digest v1: nobody-here is not in"),
        "{text}"
    );
}

#[tokio::test]
async fn health_and_readiness() {
    let app = app();
    assert_eq!(get(&app, "/healthz").await.status(), StatusCode::OK);
    // Keys load on first use; the test app has not used them yet.
    let (status, ready) = get_json(&app, "/readyz").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE, "{ready}");
    assert_eq!(ready["db"], "ok");
    ingest(&app, &snapshot(5)).await;
    let (status, ready) = get_json(&app, "/readyz").await;
    assert_eq!(status, StatusCode::OK, "{ready}");
}
