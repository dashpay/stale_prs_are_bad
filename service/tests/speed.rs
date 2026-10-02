//! `GET /api/v1/me/speed`: the signed-in person's own speed, and the year
//! of cycle time a backfill import gives it.

mod common;

use axum::http::{header, Method, StatusCode};
use chrono::{DateTime, Datelike, Months, TimeDelta, Utc};
use common::signin::*;
use common::*;
use pr_hygiene::dashboard::{ClosedPr, Dashboard};
use pr_hygiene_service::snapshot;
use pr_hygiene_service::store::{Outcome, Store};
use serde_json::{json, Value};

const ALICE: i64 = 1001;
const BOB: i64 = 2002;

async fn speed(app: &TestApp, session: Option<&str>) -> (StatusCode, Value) {
    let res = call(app, Method::GET, "/api/v1/me/speed", session, None).await;
    let status = res.status();
    (status, json_body(res).await)
}

/// The 15th of the month `back` months before this one, at noon.
fn mid_month(back: u32) -> DateTime<Utc> {
    let now = Utc::now();
    now.with_day(15)
        .unwrap()
        .date_naive()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc()
        .checked_sub_months(Months::new(back))
        .unwrap()
}

fn month_label(t: DateTime<Utc>) -> String {
    format!("{:04}-{:02}", t.year(), t.month())
}

fn merged(number: u64, (login, id): (&str, i64), at: DateTime<Utc>) -> ClosedPr {
    ClosedPr {
        key: format!("{PLATFORM}#{number}"),
        repo: PLATFORM.into(),
        number,
        author: Some(login.into()),
        author_id: Some(id as u64),
        created_at: at - TimeDelta::days(3),
        ready_at: Some(at - TimeDelta::hours(24)),
        merged_at: Some(at),
        closed_at: at,
        reviews: vec![],
    }
}

/// A year's backfill as `pr-hygiene --closed-days 365` writes it, read a
/// day ago: the open PRs then, and Alice's and Bob's PRs merged on the 15th
/// of each of the twelve months before this one, each a day after it was
/// ready for review.
fn backfill() -> Dashboard {
    let mut d = snapshot(24 * 60);
    d.closed = (1..=12)
        .flat_map(|back| {
            let at = mid_month(back);
            [
                merged(10_000 + u64::from(back), ("alice", ALICE), at),
                merged(20_000 + u64::from(back), ("bob", BOB), at),
            ]
        })
        .collect();
    d
}

/// What `pr-hygiene-service import` does with a file: checked as on
/// ingest, then imported into the service's database.
fn import(app: &TestApp, d: &Dashboard) -> Outcome {
    let raw = serde_json::to_string(d).unwrap();
    let checked = snapshot::parse(raw.as_bytes(), Utc::now()).unwrap();
    let mut store = Store::open(&app.db).unwrap();
    store.import(&checked, &raw, Utc::now()).unwrap().outcome
}

#[tokio::test]
async fn speed_needs_a_session_and_is_private() {
    let app = app_for(&FakeGithub::new());
    let (status, _) = speed(&app, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let res = call(&app, Method::GET, "/api/v1/me/speed", None, None).await;
    assert_private(&res);
    assert!(res.headers().get(header::WWW_AUTHENTICATE).is_none());
    assert_eq!(
        speed(&app, Some("not-a-session-id")).await.0,
        StatusCode::UNAUTHORIZED
    );

    let session = sign_in(&app).await;
    // A read changes nothing, so another origin's request is answered —
    // and, private, its page cannot read the answer.
    let res = call(
        &app,
        Method::GET,
        "/api/v1/me/speed",
        Some(&session),
        Some("https://elsewhere.example"),
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_private(&res);
    assert_eq!(
        json_body(res).await,
        json!({ "since": null, "months": [] }),
        "signed in with nothing recorded yet"
    );
    for method in [Method::POST, Method::PUT, Method::DELETE] {
        let res = call(
            &app,
            method.clone(),
            "/api/v1/me/speed",
            Some(&session),
            Some(ORIGIN),
        )
        .await;
        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED, "{method}");
        assert_private(&res);
    }

    let off = common::app();
    let res = call(&off, Method::GET, "/api/v1/me/speed", None, None).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND, "sign-in off");
    assert_private(&res);
}

/// The backfill is imported after posting has begun, so it is older than
/// the latest snapshot; its merges still give a year of cycle time, the
/// first month's window reaching twelve months back. Bob's merges, in the
/// same file, are not Alice's.
#[tokio::test]
async fn a_backfill_import_gives_a_year_of_the_signed_in_persons_cycle_time() {
    let app = app_for(&FakeGithub::as_user(ALICE, "alice"));
    ingest(&app, &snapshot(5)).await;
    assert!(matches!(
        import(&app, &backfill()),
        Outcome::NotNewer { .. }
    ));
    let session = sign_in(&app).await;

    let (status, body) = speed(&app, Some(&session)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let months = body["months"].as_array().unwrap();
    let labels: Vec<String> = (0..12)
        .rev()
        .map(|back| month_label(mid_month(back)))
        .collect();
    let listed: Vec<&str> = months
        .iter()
        .map(|m| m["month"].as_str().unwrap())
        .collect();
    assert_eq!(listed, labels, "twelve months, oldest first, to this one");
    let cycle: Vec<(Value, Value)> = months
        .iter()
        .map(|m| {
            (
                m["cycle_time"]["median_hours"].clone(),
                m["cycle_time"]["n"].clone(),
            )
        })
        .collect();
    let mut expected = vec![(Value::Null, json!(2))];
    expected.extend(std::iter::repeat_n((json!(24.0), json!(3)), 10));
    expected.push((Value::Null, json!(2)));
    assert_eq!(
        cycle, expected,
        "the oldest month's window holds the merge twelve months back; \
         this month's holds the two before it; two make no median"
    );
    assert_eq!(
        body["since"],
        json!(mid_month(12) - TimeDelta::hours(24)),
        "the oldest merge used was ready a day before it merged"
    );
    assert_eq!(
        months[11]["your_turn"],
        json!({ "median_hours": null, "n": 0 }),
        "the snapshot's turn of hers started before it was first seen: not timed"
    );
}

#[tokio::test]
async fn an_opted_out_person_gets_no_speed_and_is_not_recorded_again() {
    let app = app_for(&FakeGithub::as_user(ALICE, "alice"));
    let session = sign_in(&app).await;
    import(&app, &backfill());
    let (_, body) = speed(&app, Some(&session)).await;
    assert_eq!(body["months"].as_array().unwrap().len(), 12);
    assert!(
        rows_of(&app, ALICE) > 12,
        "her merges, and the snapshot's ask and turn"
    );
    let (_, me_body) = me(&app, Some(&session)).await;
    assert_eq!(me_body["opted_out"], false);

    let res = call(
        &app,
        Method::POST,
        "/api/v1/me/opt-out",
        Some(&session),
        Some(ORIGIN),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT);
    let (status, body) = speed(&app, Some(&session)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "opted_out": true }));
    let (_, me_body) = me(&app, Some(&session)).await;
    assert_eq!(me_body["opted_out"], true);
    assert_eq!(rows_of(&app, ALICE), 0, "her inputs are gone");
    assert_eq!(
        count(
            &app,
            &format!("SELECT count(*) FROM merges WHERE author_id = {BOB}")
        ),
        12,
        "no one else's"
    );

    let mut again = backfill();
    again.generated_at += TimeDelta::minutes(1);
    assert!(matches!(import(&app, &again), Outcome::Stored { .. }));
    assert_eq!(rows_of(&app, ALICE), 0, "and none come back");
}

/// Every speed input naming `id`.
fn rows_of(app: &TestApp, id: i64) -> i64 {
    count(
        app,
        &format!(
            "SELECT (SELECT count(*) FROM asks WHERE person_id = {id})
                  + (SELECT count(*) FROM turns WHERE author_id = {id})
                  + (SELECT count(*) FROM merges WHERE author_id = {id})
                  + (SELECT count(*) FROM reviews WHERE reviewer_id = {id})
                  + (SELECT count(*) FROM pr_authors WHERE author_id = {id})"
        ),
    )
}
