//! The engine end to end over HTTP: a report on a small repository, read
//! through `ReadOnly<HttpTransport>` on a blocking thread, as the service
//! runs it.

use super::mock::{self, fixed_token, json_answer, Seen};
use super::{HttpTransport, ReadOnly};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use pr_hygiene_engine::evidence::{queries, Client, GitHub};
use pr_hygiene_engine::pycompat::{py_loads, PyDateTime, PyValue};
use pr_hygiene_engine::reconcile::{Clock, ClockSite, Command, Reconciler, RunOptions, Selection};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::runtime::Handle;

const REPO: &str = "dashpay/platform";
const HEAD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const NOW: &str = "2026-09-11T14:00:00+00:00";

/// One area, `drive`, owned by `owner` and reviewed by `reviewer`.
fn policy() -> PyValue {
    let policy = json!({
        "version": 1, "repository": REPO, "max_active_prs": 5,
        "target_branches": ["v4.2-dev"],
        "fallback": {"owners": ["fallback"], "reviewers": []},
        "areas": [{"id": "drive", "paths": ["packages/drive/"], "owners": ["owner"],
                   "reviewers": ["reviewer"]}],
    });
    py_loads(&policy.to_string()).unwrap()
}

/// A clock stopped at [`NOW`].
struct Stopped;

impl Clock for Stopped {
    fn now(&mut self, _site: ClockSite) -> PyDateTime {
        PyDateTime::fromisoformat(NOW).unwrap()
    }
}

fn answer(value: Value) -> Response {
    json_answer(&value.to_string())
}

/// Pull request 1, by `owner`, on the drive: both review bots had their
/// final word on its head, and its author attested to having reviewed it.
fn pull() -> Value {
    json!({
        "number": 1, "user": {"login": "owner", "type": "User"}, "body": "Some text.",
        "labels": [], "assignees": [], "head": {"sha": HEAD},
        "base": {"ref": "v4.2-dev", "sha": "b".repeat(40)},
        "created_at": "2026-09-10T00:00:00Z", "draft": false, "state": "open",
        "html_url": "https://github.com/dashpay/platform/pull/1", "title": "PR 1",
        "changed_files": 1, "requested_reviewers": [],
    })
}

fn graphql(body: &str) -> Response {
    let document: Value = serde_json::from_str(body).unwrap();
    let query = document["query"].as_str().unwrap();
    let pull_request = |pull: Value| answer(json!({"data": {"repository": {"pullRequest": pull}}}));
    if query == queries::THREADS {
        return pull_request(json!({"reviewThreads": {"totalCount": 0,
            "pageInfo": {"hasNextPage": false, "endCursor": null}, "nodes": []}}));
    }
    if query == queries::BUILD {
        return pull_request(json!({"commits": {"nodes": [{"commit": {"oid": HEAD,
            "statusCheckRollup": null}}]}}));
    }
    // The batched history: the author's attestation, and no record yet.
    let attestation = json!({"databaseId": 103, "body": format!("/self-reviewed {HEAD}"),
        "createdAt": "2026-09-11T11:00:00Z", "updatedAt": "2026-09-11T11:00:00Z",
        "lastEditedAt": null, "author": {"login": "owner", "__typename": "User"},
        "editor": null});
    answer(json!({"data": {"repository": {"pr1": {"number": 1,
        "comments": {"totalCount": 1, "nodes": [attestation]},
        "timelineItems": {"nodes": []}}}}}))
}

/// GitHub, as far as this repository's reads go. Its open pull requests
/// come in two pages, the second linked under the repository's id.
fn github(seen: &Seen) -> Response {
    if (seen.method.as_str(), seen.path()) == ("POST", "/graphql") {
        return graphql(&seen.body);
    }
    if seen.method.as_str() != "GET" {
        return StatusCode::METHOD_NOT_ALLOWED.into_response();
    }
    let review = |id: i64, user: &str, state: &str, body: String| {
        json!({"id": id, "user": {"login": user}, "state": state, "commit_id": HEAD,
               "submitted_at": "2026-09-11T10:00:00Z", "body": body})
    };
    let route = seen
        .path()
        .strip_prefix("/repos/dashpay/platform/")
        .or_else(|| seen.path().strip_prefix("/repositories/42/"));
    match route {
        Some("pulls") if seen.path_and_query.ends_with("&page=2") => answer(json!([])),
        Some("pulls") => {
            let mut page = answer(json!([pull()]));
            let link = format!(
                "<{}/repositories/42/pulls?state=open&per_page=100&page=2>; rel=\"next\"",
                seen.origin()
            );
            page.headers_mut()
                .insert(header::LINK, link.parse().unwrap());
            page
        }
        Some("pulls/1") => answer(pull()),
        Some("pulls/1/files") => answer(
            json!([{"filename": "packages/drive/a.rs", "status": "modified", "sha": "c".repeat(40)}]),
        ),
        Some("pulls/1/reviews") => answer(json!([
            review(
                11,
                "thepastaclaw",
                "COMMENTED",
                format!("<!-- thepastaclaw-review-phase v1 phase=final sha={HEAD} -->")
            ),
            review(12, "coderabbitai[bot]", "APPROVED", String::new()),
        ])),
        Some(route) if route.starts_with("commits/") && route.ends_with("/statuses") => {
            answer(json!([]))
        }
        Some("collaborators") => answer(json!([
            {"login": "owner", "permissions": {"pull": true, "triage": true, "push": true}},
            {"login": "reviewer", "permissions": {"pull": true, "triage": true, "push": true}},
            {"login": "fallback", "permissions": {"pull": true, "triage": true, "push": true}},
        ])),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_report_reaches_a_verdict_through_the_read_only_transport_and_writes_nothing() {
    let server = mock::serve(github).await;
    let transport = HttpTransport::with_origin(
        Handle::current(),
        fixed_token("ghs_reader"),
        &server.url,
        Duration::from_secs(5),
    )
    .unwrap();
    let transport = ReadOnly::new(transport, [REPO]).unwrap();
    let (run, refused) = tokio::task::spawn_blocking(move || {
        let mut api = GitHub::new(REPO, Client::new(transport)).unwrap();
        let mut clock = Stopped;
        let mut telemetry = || PyValue::None;
        let run = Reconciler::new(&mut api, &mut clock).run(
            &policy(),
            RunOptions {
                command: Command::Report,
                selection: Selection::All,
                apply: false,
                user: None,
                nudges: 0,
                telemetry: &mut telemetry,
            },
        );
        (run, api.client().transport().refused())
    })
    .await
    .unwrap();
    let run = run.expect("the run decides");
    assert!(run.failure().is_none(), "{:?}", run.failure());
    assert_eq!(refused, 0, "every call the engine made was a read");
    assert_eq!(run.verdicts.len(), 1);
    let PyValue::Dict(verdict) = &run.verdicts[0] else {
        panic!("{:?}", run.verdicts[0])
    };
    // The owner of the only area it touches, with both bots' final word
    // and its author's attestation: nothing is missing. The verdict is the
    // engine's, decided from evidence read over HTTP, every field of it.
    assert!(
        matches!(verdict.get("state"), Some(PyValue::Str(state)) if state == "ready-to-merge"),
        "{verdict:?}"
    );
    assert!(run.report.is_some());

    let seen = server.seen();
    assert!(
        seen.iter().all(|request| request.method == "GET"
            || (request.method == "POST" && request.path() == "/graphql")),
        "only reads reached GitHub: {:?}",
        seen.iter()
            .map(|r| format!("{} {}", r.method, r.path_and_query))
            .collect::<Vec<_>>()
    );
    assert!(
        seen.iter().any(|request| request.path_and_query
            == "/repositories/42/pulls?state=open&per_page=100&page=2"),
        "the second page was followed"
    );
    assert!(seen
        .iter()
        .all(|request| request.headers["authorization"] == "token ghs_reader"));
}
