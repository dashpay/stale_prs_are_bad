//! The reads themselves: access, review threads, comment histories, the
//! issue timeline, statuses and the snapshot of one pull request.
//!
//! Ported from `pr_review/tests/test_github.py`. Where Python patched
//! `GitHub.request` or `GitHub.pages`, these answer the same calls at the
//! transport, as `gh` would have answered them.

use crate::builds::rollup;
use crate::support::*;
use pr_hygiene_engine::evidence::reader::pr_identity;
use pr_hygiene_engine::evidence::records::parse_controller_state;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn head() -> String {
    "a".repeat(40)
}

/// `GitHubTests.pr()`: a pull request as the REST route answers it.
fn pr() -> Value {
    json!({"number": 1, "user": {"login": "author"}, "head": {"sha": head()},
           "base": {"sha": "b".repeat(40), "ref": "v4.2-dev"}, "created_at": "2026-09-01T00:00:00Z",
           "draft": false, "state": "open", "html_url": "https://github.com/dashpay/platform/pull/1",
           "title": "Example", "changed_files": 1, "requested_reviewers": [], "labels": [],
           "assignees": [{"login": "romchornyi"}]})
}

/// `GitHubTests.graph()`: one page of review threads.
fn graph(nodes: Value, more: bool, cursor: Option<&str>, total: Option<usize>) -> Value {
    let count = total.unwrap_or_else(|| nodes.as_array().map_or(0, Vec::len));
    json!({"data": {"repository": {"pullRequest": {"reviewThreads": {
        "totalCount": count, "nodes": nodes, "pageInfo": {"hasNextPage": more, "endCursor": cursor}}}}}})
}

fn no_threads() -> Value {
    graph(json!([]), false, None, None)
}

fn rights(granted: &[&str]) -> Value {
    let mut flags = serde_json::Map::new();
    for level in ["admin", "maintain", "push", "triage", "pull"] {
        flags.insert(level.into(), json!(granted.contains(&level)));
    }
    Value::Object(flags)
}

/// The collaborator listing `snapshot_fixture` answers.
fn listing() -> Value {
    json!([{"login": "drive-owner", "role_name": "write", "permissions": rights(&["push", "pull"])},
           {"login": "swift-owner", "role_name": "read", "permissions": rights(&["pull"])},
           {"login": "owner", "role_name": "admin", "permissions": rights(&["admin", "push", "pull"])},
           {"login": "custom-role", "role_name": "security-lead",
            "permissions": rights(&["push", "pull", "triage"])}])
}

/// A REST-shaped comment as `snapshot_fixture`'s batched query answers it:
/// its author's login without `[bot]` and typed a bot, its editor typed by
/// its spelling.
fn as_graphql(c: &Value) -> Value {
    let user = match &c["user"] {
        Value::Object(user) => user["login"].as_str().unwrap().to_owned(),
        other => other.as_str().unwrap().to_owned(),
    };
    let edited_by = c
        .get("edited_by")
        .and_then(Value::as_str)
        .filter(|e| !e.is_empty());
    let last_edited = match c.get("edited_at") {
        Some(at) => at.clone(),
        None if edited_by.is_some() => c["updated_at"].clone(),
        None => Value::Null,
    };
    json!({"databaseId": c["id"], "body": c["body"], "createdAt": c["created_at"], "updatedAt": c["updated_at"],
           "lastEditedAt": last_edited,
           "author": {"login": user.trim_end_matches("[bot]"), "__typename": "Bot"},
           "editor": edited_by.map(|e| json!({"login": e.trim_end_matches("[bot]"),
                                              "__typename": if e.ends_with("[bot]") { "Bot" } else { "User" }}))})
}

/// `GitHubTests.snapshot_fixture`: one repository answering every read a
/// snapshot makes. Calls are told apart the way the engine tells them
/// apart: by what was asked for.
#[derive(Default, Clone)]
struct Fixture {
    comments: Vec<Value>,
    files: Option<Value>,
    threads: Option<Value>,
    checks: Option<Value>,
}

impl Fixture {
    fn route(self) -> impl FnMut(&Call) -> Answer + 'static {
        move |call: &Call| match call {
            Call::Graphql { query, .. } if query.contains("statusCheckRollup") => ok(self
                .checks
                .clone()
                .unwrap_or_else(|| rollup(json!([]), &head(), false, None))),
            Call::Graphql { query, .. } if query.contains("timelineItems") => {
                let nodes: Vec<Value> = self.comments.iter().map(as_graphql).collect();
                ok(json!({"data": {"repository": {"pr1": {"number": 1,
                    "comments": {"totalCount": nodes.len(), "nodes": nodes},
                    "timelineItems": {"nodes": []}}}}}))
            }
            Call::Graphql { .. } => ok(self.threads.clone().unwrap_or_else(no_threads)),
            Call::Rest {
                path,
                paginate: true,
                ..
            } => {
                if path.contains("/files") {
                    page(self.files.clone().unwrap_or_else(
                        || json!([{"filename": "packages/rs-drive/a.rs", "status": "modified"}]),
                    ))
                } else if path.contains("/comments") {
                    page(Value::Array(self.comments.clone()))
                } else if path.contains("/collaborators") {
                    page(listing())
                } else {
                    page(json!([]))
                }
            }
            Call::Rest { path, .. } if path.ends_with("/permission") => {
                ok(json!({"permission": "write"}))
            }
            Call::Rest { .. } => ok(pr()),
        }
    }
}

fn plain_policy() -> PyValue {
    py(json!({"fallback": ["owner"], "areas": []}))
}

fn snapshot(fixture: Fixture) -> Result<PyValue, ReadError> {
    let mut api = api(fixture.route());
    api.snapshot(&int(1), &plain_policy(), None)
}

fn permission_paths(api: &GitHub<Fake>) -> Vec<String> {
    calls(api)
        .iter()
        .map(|c| path(c).to_owned())
        .filter(|p| p.ends_with("/permission"))
        .collect()
}

#[test]
fn should_reject_repository_path_injection() {
    for name in [
        "dashpay/platform/../x",
        "--hostname=evil",
        "https://github.com/a/b",
        "dashpay/..",
    ] {
        let client = Client::with_sleep(nothing_asked(), NoSleep);
        assert!(
            matches!(GitHub::new(name, client), Err(ReadError::GitHub(_))),
            "{name}"
        );
    }
}

/// A transport no call should reach.
fn nothing_asked() -> impl Transport {
    |call: &Call| -> Answer { panic!("nothing should be asked, but {call} was") }
}

#[test]
fn the_snapshot_says_whether_the_author_is_a_bot() {
    // A bot author cannot attest, and the policy has to know that from the
    // snapshot rather than guess from a login.
    let raw = json!({"number": 1, "user": {"login": "Copilot", "type": "Bot"}, "head": {"sha": head()},
                     "base": {"ref": "v4.2-dev", "sha": "b".repeat(40)}, "draft": false, "state": "open",
                     "created_at": "2026-09-11T00:00:00Z",
                     "html_url": "https://github.com/dashpay/platform/pull/1", "title": "t"});
    let identity = pr_identity(&py(raw.clone())).unwrap();
    assert_py(field(&identity, "author_is_bot"), json!(true));
    let person = merged(&raw, json!({"user": {"login": "Copilot", "type": "User"}}));
    assert_py(
        field(&pr_identity(&py(person)).unwrap(), "author_is_bot"),
        json!(false),
    );
}

#[test]
fn whoever_posts_a_skip_has_their_access_read() {
    // The snapshot reads access for area people, reviewers and thread
    // authors. A skip from anyone else was silently nobody's — which was
    // most writers, and most authors.
    let raw = json!({"number": 1, "user": {"login": "author", "type": "User"}, "head": {"sha": head()},
                     "base": {"ref": "v4.2-dev", "sha": "b".repeat(40)}, "draft": false, "state": "open",
                     "created_at": "2026-09-11T00:00:00Z", "html_url": "u", "title": "t",
                     "changed_files": 1, "requested_reviewers": [], "labels": []});
    let comments = vec![
        py(json!({"id": 1, "user": "helper", "body": "/skip-bots",
                  "created_at": "2026-09-11T01:00:00Z", "updated_at": "2026-09-11T01:00:00Z"})),
        py(json!({"id": 2, "user": "chatter", "body": "nice",
                  "created_at": "2026-09-11T01:00:00Z", "updated_at": "2026-09-11T01:00:00Z"})),
    ];
    let mut api = api(move |call| match call {
        Call::Graphql { query, .. } if query.contains("statusCheckRollup") => ok(
            json!({"data": {"repository": {"pullRequest": {"commits": {"nodes": [
                {"commit": {"oid": head(), "statusCheckRollup": null}}]}}}}}),
        ),
        Call::Graphql { .. } => ok(no_threads()),
        Call::Rest {
            path,
            paginate: true,
            ..
        } if path.contains("/files") => page(json!([{"filename": "packages/rs-drive/x"}])),
        Call::Rest { paginate: true, .. } => page(json!([])),
        Call::Rest { path, .. } if path.ends_with("/permission") => {
            ok(json!({"permission": "write"}))
        }
        Call::Rest { .. } => ok(raw.clone()),
    });
    let policy = py(
        json!({"fallback": {"owners": ["QuantumExplorer"], "reviewers": []}, "areas": [],
                           "target_branches": ["v4.2-dev"]}),
    );
    let history = History {
        comments,
        lifecycle_at: None,
    };
    let snapshot = api.snapshot(&int(1), &policy, Some(&history)).unwrap();
    let asked = permission_paths(&api);
    assert!(
        asked.contains(&"repos/dashpay/platform/collaborators/helper/permission".to_owned()),
        "the skipper is looked up"
    );
    assert!(
        !asked.iter().any(|p| p.contains("/chatter/")),
        "nobody else who merely commented is"
    );
    assert_py(
        field(field(&snapshot, "permissions"), "helper"),
        json!("write"),
    );
}

#[test]
fn someone_the_listing_omits_is_asked_about_directly() {
    // An organisation's own members reach a repository through the
    // organisation, and a repository-scoped token does not enumerate them.
    // Reading that absence as "no write access" marked six pull requests a
    // configuration error the day writes were turned on.
    let mut api = api(|call| match call {
        Call::Rest { paginate: true, .. } => {
            page(json!([{"login": "direct", "permissions": {"push": true, "pull": true}}]))
        }
        _ => ok(json!({"permission": "admin"})),
    });
    assert_eq!(api.permission("direct").unwrap().as_deref(), Some("write"));
    assert_eq!(
        api.permission("via-the-org").unwrap().as_deref(),
        Some("admin")
    );
    assert_eq!(
        api.permission("via-the-org").unwrap().as_deref(),
        Some("admin"),
        "asked once, then remembered"
    );
    assert_eq!(
        permission_paths(&api),
        ["repos/dashpay/platform/collaborators/via-the-org/permission"]
    );
}

#[test]
fn an_unreadable_answer_is_asked_for_once_not_on_every_snapshot() {
    // A reconciliation snapshots the same pull request up to three times,
    // and a sweep covers six. Re-asking each time turns one unreachable
    // answer into hundreds of requests — and a rate limit into a loop that
    // answers it with more requests.
    let mut api = api(|call| match call {
        Call::Rest { paginate: true, .. } => page(json!([])),
        _ => failed(1, "", "HTTP 403: Forbidden"),
    });
    for _ in 0..5 {
        assert_eq!(api.permission("someone").unwrap(), None);
    }
    assert_eq!(permission_paths(&api).len(), 1);
    assert_eq!(api.take_diagnostics().len(), 1, "said once why");
    api.forget_cached_access();
    reroute(&mut api, |call| match call {
        Call::Rest { paginate: true, .. } => page(json!([])),
        _ => ok(json!({"permission": "admin"})),
    });
    assert_eq!(
        api.permission("someone").unwrap().as_deref(),
        Some("admin"),
        "the pre-write re-read asks again"
    );
    assert_eq!(permission_paths(&api).len(), 1);
}

#[test]
fn a_fallback_answer_does_not_stand_in_for_having_read_the_listing() {
    // The listing is read once, and "have I read it" must not be answered by
    // an entry the fallback put there — otherwise a repository whose
    // listing came back empty never reads it again.
    let mut api = api(|_| page(json!([])));
    api.access().unwrap();
    api.access().unwrap();
    assert_eq!(
        calls(&api).len(),
        1,
        "a listing that came back empty is still a listing that was read"
    );
    api.forget_cached_access();
    api.access().unwrap();
    assert_eq!(
        calls(&api).len(),
        2,
        "and the pre-write re-read reads it afresh"
    );
}

#[test]
fn the_fallback_reads_the_same_capability_flags_as_the_listing() {
    // A custom organisation role's legacy `permission` string is only an
    // approximation; the flags beside it say plainly whether they push.
    let answer = json!({"permission": "read", "user": {"permissions": {"admin": false, "maintain": false,
                        "push": true, "triage": true, "pull": true}}});
    let mut api = api(move |call| match call {
        Call::Rest { paginate: true, .. } => page(json!([])),
        _ => ok(answer.clone()),
    });
    assert_eq!(
        api.permission("custom-role").unwrap().as_deref(),
        Some("write")
    );
}

#[test]
fn an_answer_that_never_arrives_is_unknown_not_none() {
    let mut api = api(|call| match call {
        Call::Rest { paginate: true, .. } => page(json!([])),
        _ => failed(1, "", "HTTP 403: Forbidden"),
    });
    assert_eq!(api.permission("someone").unwrap(), None);
}

#[test]
fn a_listing_that_could_not_be_read_is_not_asked_again_in_the_run() {
    // Sequential Python marks the listing read before reading it, so after a
    // failed listing everyone is asked about one by one until the caches are
    // dropped. The port reads sequentially and keeps that, not the race four
    // concurrent snapshots run into.
    let mut api = api(|call| match call {
        Call::Rest { paginate: true, .. } => failed(1, "", "HTTP 403: Forbidden"),
        _ => ok(json!({"permission": "write"})),
    });
    assert!(api.permission("first").is_err());
    assert_eq!(api.permission("second").unwrap().as_deref(), Some("write"));
    let listings = calls(&api)
        .iter()
        .filter(|c| path(c).contains("collaborators?"))
        .count();
    assert_eq!(listings, 1);
}

#[test]
fn evidence_already_read_for_admission_is_not_read_again() {
    // collect() reads every candidate's comments and lifecycle in one
    // batched query; the snapshot reuses that read instead of asking for the
    // comments again and walking the whole issue timeline, which pages.
    let comment = json!({"id": 7, "user": {"login": "author"}, "body": "hello",
                         "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"});
    let fixture = Fixture {
        comments: vec![comment],
        ..Fixture::default()
    };
    let fresh = snapshot(fixture.clone()).unwrap();
    // Distinct values, so the assertions below cannot pass by both sides
    // reading the same fixture and agreeing vacuously.
    let other = {
        let PyValue::Dict(mut first) = items(field(&fresh, "comments"))[0].clone() else {
            unreachable!()
        };
        first.insert("id".into(), py(json!(8)));
        first.insert("body".into(), py(json!("from the batched read")));
        PyValue::Dict(first)
    };
    let history = History {
        comments: vec![other],
        lifecycle_at: Some("2026-09-02T00:00:00Z".into()),
    };
    let mut api = api(fixture.route());
    let reused = api
        .snapshot(&int(1), &plain_policy(), Some(&history))
        .unwrap();
    let ids: Vec<_> = items(field(&reused, "comments"))
        .iter()
        .map(|c| shown(field(c, "id")))
        .collect();
    assert_eq!(ids, ["8"]);
    assert_py(
        field(&reused, "lifecycle_at"),
        json!("2026-09-02T00:00:00Z"),
    );
    let reread = calls(&api).iter().any(|c| {
        query(c).contains("fragment history")
            || path(c).contains("/comments")
            || path(c).contains("/timeline")
    });
    assert!(
        !reread,
        "neither the comments nor the timeline may be read a second time"
    );
}

#[test]
fn the_batched_reader_and_the_per_pull_request_reader_agree() {
    // Both readers feed the same evidence, so their shapes must match. The
    // batched query names a bot `coderabbitai` where REST names it
    // `coderabbitai[bot]`; if those stop agreeing, the evidence differs from
    // itself between one read and the next.
    let node = json!({"databaseId": 11, "body": "receipt", "createdAt": "2026-09-01T00:00:00Z",
                      "updatedAt": "2026-09-01T00:05:00Z", "lastEditedAt": "2026-09-01T00:05:00Z",
                      "author": {"login": "coderabbitai", "__typename": "Bot"}});
    let graph = json!({"data": {"repository": {"pr1": {"number": 1,
        "comments": {"totalCount": 1, "nodes": [node]}, "timelineItems": {"nodes": []}}}}});
    let mut batched_api = api(move |_| ok(graph.clone()));
    let batched = batched_api.histories(&[int(1)]).unwrap()[&int(1)]
        .comments
        .clone();
    let rest = json!([{"id": 11, "user": {"login": "coderabbitai[bot]"}, "body": "receipt",
                       "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:05:00Z"}]);
    let mut rest_api = api(move |_| page(rest.clone()));
    let per_pr = rest_api.comments(&int(1)).unwrap();
    // Who last edited a comment, and when, are what the listing cannot say,
    // and it says so. Without those two, both agree.
    let editless = |comments: &[PyValue]| -> Vec<PyValue> {
        comments
            .iter()
            .map(|c| {
                let PyValue::Dict(c) = c.clone() else {
                    unreachable!()
                };
                let mut c = c.into_map();
                c.shift_remove("edited_by");
                c.shift_remove("edited_at");
                PyValue::Dict(c.into())
            })
            .collect()
    };
    let (a, b) = (editless(&per_pr), editless(&batched));
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(&b) {
        assert!(py_eq(x, y), "{} != {}", shown(x), shown(y));
    }
}

#[test]
fn should_refuse_truncated_changed_files() {
    let result = snapshot(Fixture {
        files: Some(json!([])),
        ..Fixture::default()
    });
    assert_eq!(github_error(result), "Incomplete changed-file list");
}

#[test]
fn should_ignore_copied_controller_markers_and_reject_duplicate_trusted_state() {
    let state = json!({"version": 1, "number": 1, "head": head(), "admitted_at": null,
                       "ready_since": null, "state": "too-many-open-prs", "evidence": "b".repeat(64),
                       "context": "c".repeat(64)});
    let marker = format!(
        "<!-- platform-pr-review-state-v1 {} -->",
        py_dumps(&py(state.clone()), false, None, None).unwrap()
    );
    let comment = |number: i64, actor: &str| {
        json!({"id": number, "user": {"login": actor}, "body": marker,
               "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"})
    };
    let read = snapshot(Fixture {
        comments: vec![comment(1, "author"), comment(2, "github-actions[bot]")],
        ..Fixture::default()
    })
    .unwrap();
    assert_py(field(&read, "controller_state"), state);
    assert_py(field(&read, "controller_comment_id"), json!(2));
    // Every announcement of a move carries the record as of then, so the
    // newest is the current one. Two runs writing at once still agree.
    let read = snapshot(Fixture {
        comments: vec![
            comment(3, "github-actions[bot]"),
            comment(2, "github-actions[bot]"),
        ],
        ..Fixture::default()
    })
    .unwrap();
    assert_py(field(&read, "controller_comment_id"), json!(3));
}

#[test]
fn should_fail_on_malformed_trusted_state() {
    let comments = vec![json!({"id": 2, "user": {"login": "github-actions[bot]"},
                               "body": "<!-- platform-pr-review-state-v1 {oops} -->",
                               "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-01T00:00:00Z"})];
    let mut api = api(Fixture {
        comments,
        ..Fixture::default()
    }
    .route());
    let result = api.snapshot(&int(1), &py(json!({"fallback": [], "areas": []})), None);
    assert_eq!(github_error(result), "Malformed controller state JSON");
}

#[test]
fn a_pull_request_that_vanished_mid_run_does_not_fail_the_others() {
    // `gh` exits non-zero whenever GraphQL answers with any errors array. A
    // pull request closed between listing the queue and reading its history
    // answers null for its own alias while every other alias answers
    // normally. Treating that exit code as a failure aborted the whole
    // reconciliation, and one person closing a pull request marked the rest
    // red.
    let body = json!({"data": {"repository": {
        "pr1": {"number": 1, "comments": {"totalCount": 0, "nodes": []}, "timelineItems": {"nodes": []}},
        "pr2": null}},
        "errors": [{"type": "NOT_FOUND", "path": ["repository", "pr2"],
                    "message": "Could not resolve to a PullRequest with the number of 2."}]})
    .to_string();
    let mut api = api(move |_| failed(1, &body, "not found"));
    let histories = api.histories(&[int(1), int(2)]).unwrap();
    assert_eq!(
        histories.keys().cloned().collect::<Vec<_>>(),
        [int(1)],
        "the surviving pull request still has its history"
    );
}

#[test]
fn a_partial_failure_that_is_not_a_missing_pull_request_still_fails() {
    // These reach the reader's filter rather than stopping at the exit code,
    // which is the only behaviour the relaxation changes.
    let good = json!({"number": 1, "comments": {"totalCount": 0, "nodes": []}, "timelineItems": {"nodes": []}});
    for (errors, label) in [
        (
            json!([{"type": "FORBIDDEN", "message": "no"}]),
            "one field refused",
        ),
        (
            json!([{"message": "spec-shaped error with no type"}]),
            "no type at all",
        ),
        (
            json!([{"type": "NOT_FOUND"}, {"message": "and something else"}]),
            "mixed with a real one",
        ),
        (json!([{"type": "RATE_LIMITED"}]), "rate limited"),
        (json!("boom"), "errors is not a list"),
        (json!(["boom"]), "errors is not a list of objects"),
    ] {
        let body =
            json!({"data": {"repository": {"pr1": good.clone()}}, "errors": errors}).to_string();
        let mut api = api(move |_| failed(1, &body, "failed"));
        assert_eq!(
            github_error(api.histories(&[int(1)])),
            "GraphQL history query failed",
            "{label}"
        );
    }
}

#[test]
fn a_real_graphql_failure_is_still_a_failure() {
    for (body, label) in [
        (
            r#"{"errors":[{"type":"FORBIDDEN","message":"nope"}]}"#,
            "no data",
        ),
        (
            r#"{"data":null,"errors":[{"type":"NOT_FOUND"}]}"#,
            "null data",
        ),
        ("not json", "unparseable"),
        ("", "empty"),
    ] {
        let mut api = api(move |_| failed(1, body, "failed"));
        assert!(
            matches!(api.histories(&[int(1)]), Err(ReadError::GitHub(_))),
            "{label}"
        );
    }
}

fn thread(id: &str, opening: &str, voices: Value) -> Value {
    json!({"id": id, "isResolved": false, "opening": {"nodes": [{"body": opening}]}, "comments": {"nodes": voices}})
}

#[test]
fn should_paginate_graphql_threads_and_reject_stuck_cursor() {
    let node = thread(
        "T1",
        "Rename this.",
        json!([{"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]),
    );
    let mut pages = vec![
        ok(graph(json!([node]), true, Some("c1"), None)),
        ok(graph(json!([]), false, None, Some(1))),
    ]
    .into_iter();
    let mut api = api(move |_| pages.next().expect("two pages"));
    let threads = api.threads(&int(1)).unwrap();
    assert_eq!(threads.len(), 1);
    assert_py(
        &threads[0],
        json!({"id": "T1", "is_resolved": false, "author": "reviewer", "created_at": "2026-09-01T00:00:00Z",
               "voices": [{"user": "reviewer", "created_at": "2026-09-01T00:00:00Z"}], "severities": []}),
    );
    assert_eq!(
        variable_text(&calls(&api)[1], "cursor").as_deref(),
        Some("c1")
    );
    let mut stuck = crate::support::api(|_| ok(graph(json!([]), true, Some("same"), None)));
    assert_eq!(
        github_error(stuck.threads(&int(1))),
        "Review-thread pagination did not advance"
    );
}

#[test]
fn thread_whose_comments_were_all_deleted_is_skipped() {
    let live = thread(
        "T1",
        "Rename this.",
        json!([{"author": {"login": "reviewer"}, "createdAt": "2026-09-01T00:00:00Z"}]),
    );
    let emptied = json!({"id": "T2", "isResolved": false, "opening": {"nodes": []}, "comments": {"nodes": []}});
    let answer = graph(json!([emptied, live]), false, None, Some(2));
    let mut api = api(move |_| ok(answer.clone()));
    let ids: Vec<_> = api
        .threads(&int(1))
        .unwrap()
        .iter()
        .map(|t| text(field(t, "id")).to_owned())
        .collect();
    assert_eq!(ids, ["T1"]);
}

#[test]
fn a_thread_carries_everyone_who_spoke_in_it() {
    // An author's own thread with a reviewer's objection in reply was read as
    // the author's alone, and the objection vanished with it.
    let answer = graph(
        json!([thread(
            "T1",
            "Why?",
            json!([
            {"author": {"login": "author"}, "createdAt": "2026-09-01T00:00:00Z"},
            {"author": {"login": "reviewer"}, "createdAt": "2026-09-02T00:00:00Z"}])
        )]),
        false,
        None,
        Some(1),
    );
    let mut api = api(move |_| ok(answer.clone()));
    let threads = api.threads(&int(1)).unwrap();
    assert_py(field(&threads[0], "author"), json!("author"));
    let voices: Vec<_> = items(field(&threads[0], "voices"))
        .iter()
        .map(|v| text(field(v, "user")).to_owned())
        .collect();
    assert_eq!(voices, ["author", "reviewer"]);
}

#[test]
fn a_thread_carries_the_severity_of_its_findings_not_their_text() {
    // Whether a bot's thread holds the pull request depends on how it
    // labelled its findings. The text is not kept: CodeRabbit appends to its
    // opening once a finding is addressed, and keeping that would read as
    // review evidence changing underneath a write.
    let body = "_🎯 Functional Correctness_ | _🟡 Minor_ | _⚡ Quick win_\n\n**Reject an empty identifier.**\n<!-- cr-comment:v1:1 -->";
    let read = |opening: String| {
        let answer = graph(
            json!([thread(
                "T1",
                &opening,
                json!([{"author": {"login": "coderabbitai"},
                                                   "createdAt": "2026-09-01T00:00:00Z"}])
            )]),
            false,
            None,
            Some(1),
        );
        let mut api = api(move |_| ok(answer.clone()));
        api.threads(&int(1)).unwrap().remove(0)
    };
    let only = read(body.to_owned());
    assert_py(field(&only, "severities"), json!(["🟡 Minor"]));
    let PyValue::Dict(fields) = &only else {
        unreachable!()
    };
    assert!(!fields.contains_key("body"));
    let addressed = read(format!("{body}\n\n✅ Addressed in commit 1a2b3c4"));
    assert!(py_eq(&addressed, &only));
}

#[test]
fn a_thread_whose_opening_did_not_arrive_is_refused() {
    // Read as no label it would hold the pull request; read as anything else
    // it could let a blocker through. Neither is an answer.
    let voices = json!({"nodes": [{"author": {"login": "thepastaclaw"}, "createdAt": "2026-09-01T00:00:00Z"}]});
    for opening in [
        None,
        Some(json!({"nodes": null})),
        Some(json!({"nodes": []})),
        Some(json!({"nodes": [{"body": null}]})),
    ] {
        let mut broken = json!({"id": "T1", "isResolved": false, "comments": voices});
        if let Some(opening) = &opening {
            broken["opening"] = opening.clone();
        }
        let answer = graph(json!([broken]), false, None, Some(1));
        let mut api = api(move |_| ok(answer.clone()));
        assert!(
            matches!(api.threads(&int(1)), Err(ReadError::GitHub(_))),
            "{opening:?}"
        );
    }
}

#[test]
fn should_refuse_truncated_thread_connection() {
    let answer = graph(json!([]), false, None, Some(1));
    let mut api = api(move |_| ok(answer.clone()));
    assert_eq!(
        github_error(api.threads(&int(1))),
        "Incomplete review-thread list"
    );
}

#[test]
fn head_seen_at_is_the_earliest_status_this_controller_wrote() {
    let ours = json!({"context": "PR Hygiene", "creator": {"login": "github-actions[bot]"}});
    let statuses = json!([
        merged(&ours, json!({"created_at": "2026-09-14T09:35:13Z"})),
        merged(&ours, json!({"created_at": "2026-09-13T08:43:23Z"})),
        {"context": "CodeRabbit", "creator": {"login": "coderabbitai[bot]"}, "created_at": "2026-01-01T00:00:00Z"},
        {"context": "PR Hygiene", "creator": {"login": "impostor"}, "created_at": "2020-01-01T00:00:00Z"}]);
    let mut api = api(move |_| page(statuses.clone()));
    let head = head();
    assert_eq!(
        api.head_seen_at(&head).unwrap().as_deref(),
        Some("2026-09-13T08:43:23Z")
    );
    assert_eq!(
        api.head_seen_at(&head).unwrap().as_deref(),
        Some("2026-09-13T08:43:23Z")
    );
    // A commit's own status history is read once per reconciliation.
    assert_eq!(calls(&api).len(), 1);
    assert_eq!(
        path(&calls(&api)[0]),
        format!("repos/dashpay/platform/commits/{head}/statuses?per_page=100")
    );
    api.forget_cached_access();
    reroute(&mut api, |_| page(json!([])));
    assert_eq!(api.head_seen_at(&head).unwrap(), None);
    api.forget_cached_access();
    let whenever = merged(&ours, json!({"created_at": "whenever"}));
    reroute(&mut api, move |_| page(json!([whenever.clone()])));
    assert_eq!(
        github_error(api.head_seen_at(&head)),
        "Unexpected status timestamp format"
    );
}

#[test]
fn a_status_by_a_listed_identity_is_the_engines_own() {
    // The engine's statuses are recognised by who wrote them; an identity
    // that is not listed — the App, before it is — is nobody's.
    let status = |creator: &str| {
        json!([{"state": "pending", "context": "PR Hygiene", "description": "ready-for-human",
                "created_at": "2026-09-11T10:00:00Z", "creator": {"login": creator}}])
    };
    let engine = status("github-actions[bot]");
    let mut api = api(move |_| page(engine.clone()));
    assert!(api.ready_published(&head()).unwrap());
    assert_eq!(
        api.head_seen_at(&head()).unwrap().as_deref(),
        Some("2026-09-11T10:00:00Z")
    );
    assert_py(
        &api.latest_state_from_status(&head()).unwrap(),
        json!("ready-for-human"),
    );
    let app = status("pr-hygiene[bot]");
    let mut api = crate::support::api(move |_| page(app.clone()));
    assert!(!api.ready_published(&head()).unwrap());
    assert_py(&api.latest_state_from_status(&head()).unwrap(), json!(null));
}

#[test]
fn the_latest_status_is_the_newest_the_engine_wrote_its_id_breaking_a_tie() {
    let engine = json!({"login": "github-actions[bot]"});
    let statuses = json!([
        {"id": 1, "context": "PR Hygiene", "description": "waiting-bots", "created_at": "2026-09-11T10:00:00Z", "creator": engine},
        {"id": 3, "context": "PR Hygiene", "description": "waiting-build", "created_at": "2026-09-11T12:00:00Z", "creator": engine},
        {"id": 2, "context": "PR Hygiene", "description": "ready-for-human", "created_at": "2026-09-11T12:00:00Z", "creator": engine},
        {"id": 9, "context": "PR Hygiene", "description": "ready-to-merge", "created_at": "2026-09-12T00:00:00Z",
         "creator": {"login": "impostor"}}]);
    let mut api = api(move |_| page(statuses.clone()));
    assert_py(
        &api.latest_state_from_status(&head()).unwrap(),
        json!("waiting-build"),
    );
}

fn history_response(overrides: Value) -> Value {
    let comment = json!({"databaseId": 7, "body": "hello", "createdAt": "2026-09-11T10:00:00Z",
                         "updatedAt": "2026-09-11T10:00:00Z", "lastEditedAt": null,
                         "author": {"login": "github-actions", "__typename": "Bot"}});
    let node = merged(
        &json!({"number": 1, "comments": {"totalCount": 1, "nodes": [comment]},
                "timelineItems": {"nodes": [{"createdAt": "2026-09-04T00:00:00Z"}]}}),
        overrides,
    );
    json!({"data": {"repository": {"pr1": node}}})
}

#[test]
fn batched_history_restores_the_bot_suffix_graphql_omits() {
    // REST says github-actions[bot]; GraphQL says github-actions. The engine
    // finds its own record by that login, so a bare one would make its own
    // state invisible and duplicate the report.
    let answer = history_response(json!({}));
    let mut api = api(move |_| ok(answer.clone()));
    let history = api.histories(&[int(1)]).unwrap();
    let first = &history[&int(1)].comments[0];
    assert_py(field(first, "user"), json!("github-actions[bot]"));
    assert_py(field(first, "id"), json!(7));
    assert_eq!(
        history[&int(1)].lifecycle_at.as_deref(),
        Some("2026-09-04T00:00:00Z")
    );
}

#[test]
fn batched_history_treats_a_vanished_pull_request_as_gone_not_broken() {
    let survivor = history_response(json!({}))["data"]["repository"]["pr1"].clone();
    let body = json!({"data": {"repository": {"pr1": null, "pr2": survivor}},
                      "errors": [{"type": "NOT_FOUND", "path": ["repository", "pr1"]}]})
    .to_string();
    let mut api = api(move |_| failed(1, &body, "not found"));
    let history = api.histories(&[int(1), int(2)]).unwrap();
    assert_eq!(history.keys().cloned().collect::<Vec<_>>(), [int(2)]);
}

#[test]
fn batched_history_still_fails_on_a_real_error() {
    let body =
        json!({"data": {"repository": {}}, "errors": [{"type": "RATE_LIMITED"}]}).to_string();
    let mut api = api(move |_| failed(1, &body, "rate limited"));
    assert!(api.histories(&[int(1)]).is_err());
}

/// `GitHubTests.record_node`: the engine's record as GraphQL answers it,
/// last edited by `editor`.
fn record_node(number: i64, editor: Value, edited_at: Option<&str>, overrides: Value) -> Value {
    let record = merged(
        &json!({"version": 1, "number": 1, "head": head(), "admitted_at": "2026-09-01T10:00:00Z",
                "ready_since": null, "state": "waiting-bots", "evidence": "c".repeat(64),
                "context": "d".repeat(64)}),
        overrides,
    );
    json!({"databaseId": number, "body": record_body(record, "text", None),
           "createdAt": "2026-09-01T10:00:00Z", "updatedAt": edited_at.unwrap_or("2026-09-01T10:00:00Z"),
           "lastEditedAt": edited_at, "author": {"login": "github-actions", "__typename": "Bot"},
           "editor": editor})
}

/// `GitHubTests.long_comment`: the first is the engine's record, refreshed
/// by the engine; the rest are someone's chatter.
fn long_comment(number: i64) -> Value {
    if number == 1 {
        return record_node(
            1,
            json!({"login": "github-actions", "__typename": "Bot"}),
            Some("2026-09-11T10:00:00Z"),
            json!({}),
        );
    }
    json!({"databaseId": number, "body": format!("comment {number}"), "createdAt": "2026-09-02T00:00:00Z",
           "updatedAt": "2026-09-02T00:00:00Z", "lastEditedAt": null,
           "author": {"login": "someone", "__typename": "User"}, "editor": null})
}

fn everything(total: i64) -> Vec<Value> {
    (1..=total).map(long_comment).collect()
}

fn comment_page(nodes: &[Value], total: usize, cursor: Option<&str>) -> Value {
    json!({"data": {"repository": {"pullRequest": {"comments": {
        "totalCount": total, "pageInfo": {"hasNextPage": cursor.is_some(), "endCursor": cursor},
        "nodes": nodes}}}}})
}

type Pages = HashMap<Option<String>, Value>;

fn pages(entries: Vec<(Option<&str>, Value)>) -> Pages {
    entries
        .into_iter()
        .map(|(cursor, answer)| (cursor.map(str::to_owned), answer))
        .collect()
}

/// `GitHubTests.read_long`: the history of a pull request whose comments
/// outgrow the batched window; `pages` answers each cursor. A reader that
/// asks for the same page again and again is stopped after ten asks rather
/// than hanging the suite.
fn read_long(pages: Pages, total: i64) -> Result<indexmap::IndexMap<PyInt, History>, ReadError> {
    let all = everything(total);
    let window = history_response(json!({"comments": {"totalCount": total,
                                                       "nodes": all[all.len().saturating_sub(100)..]}}));
    let asked = Rc::new(RefCell::new(0));
    let mut api = api(move |call| {
        if query(call).contains("fragment history") {
            return ok(window.clone());
        }
        *asked.borrow_mut() += 1;
        if *asked.borrow() > 10 {
            return Err(TransportError::Refused(
                "the same page was asked for again and again".into(),
            ));
        }
        let after = variable_text(call, "after");
        ok(pages.get(&after).cloned().expect("a page for this cursor"))
    });
    api.histories(&[int(1)])
}

#[test]
fn both_comment_queries_ask_who_edited_and_when() {
    // Whether a comment is still the engine's own words is decided by
    // whether GitHub records an edit and whom it names as the editor. A
    // query that stopped asking would read every forged record as one
    // nobody touched.
    let all = everything(150);
    let window = history_response(json!({"comments": {"totalCount": 150, "nodes": all[50..]}}));
    let first = comment_page(&all[..100], 150, Some("c1"));
    let second = comment_page(&all[100..], 150, None);
    let mut api = api(move |call| {
        if query(call).contains("fragment history") {
            ok(window.clone())
        } else if variable_text(call, "after").is_none() {
            ok(first.clone())
        } else {
            ok(second.clone())
        }
    });
    api.histories(&[int(1)]).unwrap();
    let asked: Vec<String> = calls(&api)
        .iter()
        .map(|c| query(c).split_whitespace().collect::<Vec<_>>().join(" "))
        .collect();
    assert_eq!(asked.len(), 3, "the batched query and both pages");
    for query in asked {
        assert!(query.contains("lastEditedAt"));
        assert!(query.contains("editor { login __typename }"));
    }
}

#[test]
fn a_forged_record_on_a_later_page_is_the_newest_and_nothing_is_read() {
    // The editor of a comment on the second page is read as surely as on
    // the first, and the forged record there, being the newest, decides.
    let mut all = everything(150);
    all[119] = record_node(
        120,
        json!({"login": "llbartekll", "__typename": "User"}),
        Some("2026-09-12T00:00:00Z"),
        json!({"admitted_at": "2020-01-01T00:00:00Z", "state": "ready-for-human"}),
    );
    let pages = pages(vec![
        (None, comment_page(&all[..100], 150, Some("c1"))),
        (Some("c1"), comment_page(&all[100..], 150, None)),
    ]);
    let history = read_long_with(pages, all);
    let comments = &history[&int(1)].comments;
    assert_py(field(&comments[119], "edited_by"), json!("llbartekll"));
    assert!(parse_controller_state(comments).unwrap().is_none());
}

/// `read_long` over a conversation of its own.
fn read_long_with(pages: Pages, all: Vec<Value>) -> indexmap::IndexMap<PyInt, History> {
    let total = all.len();
    let window =
        history_response(json!({"comments": {"totalCount": total, "nodes": all[total - 100..]}}));
    let mut api = api(move |call| {
        if query(call).contains("fragment history") {
            ok(window.clone())
        } else {
            ok(pages[&variable_text(call, "after")].clone())
        }
    });
    api.histories(&[int(1)]).unwrap()
}

#[test]
fn a_record_somebody_else_edited_is_not_trusted_through_the_batched_query() {
    // A person's edit names them. A deleted or suspended account's edit is
    // still recorded, but its editor answers null, beside a NOT_FOUND error
    // the history read tolerates; read as never edited, a forged admission
    // would stand.
    for editor in [
        json!({"login": "llbartekll", "__typename": "User"}),
        json!(null),
    ] {
        let node = record_node(
            7,
            editor.clone(),
            Some("2026-09-12T00:00:00Z"),
            json!({"admitted_at": "2020-01-01T00:00:00Z", "state": "ready-for-human"}),
        );
        let mut response =
            history_response(json!({"comments": {"totalCount": 1, "nodes": [node]}}));
        response["errors"] = json!([{"type": "NOT_FOUND",
                                     "path": ["repository", "pr1", "comments", "nodes", 0, "editor"]}]);
        let body = response.to_string();
        let mut api = api(move |_| failed(1, &body, "not found"));
        let comments = api.histories(&[int(1)]).unwrap()[&int(1)].comments.clone();
        assert!(
            parse_controller_state(&comments).unwrap().is_none(),
            "{editor}"
        );
    }
}

#[test]
fn a_long_conversation_is_read_page_by_page_with_its_editors() {
    // The engine's own record can be older than the window the batched
    // query asks for, and it is rewritten in place, so it is believed only
    // once its editor is known. The REST listing names no editor: read from
    // there, a record the engine refreshed would be ignored and the pull
    // request would lose its slot.
    let all = everything(150);
    let history = read_long(
        pages(vec![
            (None, comment_page(&all[..100], 150, Some("c1"))),
            (Some("c1"), comment_page(&all[100..], 150, None)),
        ]),
        150,
    )
    .unwrap();
    let comments = &history[&int(1)].comments;
    let ids: Vec<String> = comments.iter().map(|c| shown(field(c, "id"))).collect();
    assert_eq!(ids, (1..=150).map(|n| n.to_string()).collect::<Vec<_>>());
    assert_py(
        field(&comments[0], "edited_by"),
        json!("github-actions[bot]"),
    );
    let record = parse_controller_state(comments).unwrap().unwrap();
    assert_py(&record.comment_id, json!(1));
    assert_py(
        field(&record.state, "admitted_at"),
        json!("2026-09-01T10:00:00Z"),
    );
}

#[test]
fn a_long_conversation_that_changed_while_it_was_read_is_still_whole() {
    let all = everything(151);
    let first = comment_page(&all[..100], 150, Some("c1"));
    // A comment posted during the read lands on the last page and is counted
    // by its total.
    let added = read_long(
        pages(vec![
            (None, first.clone()),
            (Some("c1"), comment_page(&all[100..], 151, None)),
        ]),
        150,
    )
    .unwrap();
    assert_eq!(added[&int(1)].comments.len(), 151);
    // One deleted from a page already read leaves more comments read than
    // the last total counts: a superset of the conversation, nothing missing
    // from it. Refusing it failed the history read for every pull request in
    // the batch, other authors' included.
    let window = json!({"data": {"repository": {
        "pr1": {"number": 1, "comments": {"totalCount": 150, "nodes": all[50..150]}, "timelineItems": {"nodes": []}},
        "pr2": {"number": 2, "comments": {"totalCount": 1, "nodes": [long_comment(999)]}, "timelineItems": {"nodes": []}}}}});
    let shrunk = pages(vec![
        (None, first.clone()),
        (Some("c1"), comment_page(&all[100..150], 149, None)),
    ]);
    let mut api = api(move |call| {
        if query(call).contains("fragment history") {
            ok(window.clone())
        } else {
            ok(shrunk[&variable_text(call, "after")].clone())
        }
    });
    let history = api.histories(&[int(1), int(2)]).unwrap();
    assert_eq!(
        (
            history[&int(1)].comments.len(),
            history[&int(2)].comments.len()
        ),
        (150, 1)
    );
    // The pull request itself deleted during the read: gone, as the batched
    // query treats it.
    let gone =
        json!({"data": {"repository": {"pullRequest": null}}, "errors": [{"type": "NOT_FOUND"}]});
    let history = read_long(pages(vec![(None, first), (Some("c1"), gone)]), 150).unwrap();
    assert!(history.is_empty());
}

#[test]
fn a_long_conversation_read_short_or_malformed_is_refused() {
    // Fewer comments than the conversation holds, or pages that do not say
    // how to go on, are not the whole conversation, and the record may be
    // the part missing. Each answers every page a reader that let the fault
    // through would go on to ask for, so only the fault itself refuses.
    let all = everything(150);
    let first = comment_page(&all[..100], 150, Some("c1"));
    let page_with = |nodes: &[Value], total: usize, cursor: Option<&str>, change: Value| {
        let mut answer = comment_page(nodes, total, cursor);
        let info = &mut answer["data"]["repository"]["pullRequest"]["comments"]["pageInfo"];
        *info = merged(info, change);
        answer
    };
    let rest = &all[100..];
    let mut no_page_info = page_with(rest, 150, None, json!({}));
    no_page_info["data"]["repository"]["pullRequest"]["comments"]
        .as_object_mut()
        .unwrap()
        .remove("pageInfo");
    let cases = vec![
        (
            pages(vec![
                (None, first.clone()),
                (Some("c1"), page_with(&all[100..149], 150, None, json!({}))),
            ]),
            "the last page short of the total",
        ),
        (
            pages(vec![(None, first.clone()), (Some("c1"), no_page_info)]),
            "a page without its page information",
        ),
        (
            pages(vec![
                (None, first.clone()),
                (
                    Some("c1"),
                    page_with(&all[100..149], 150, Some("c2"), json!({"endCursor": null})),
                ),
            ]),
            "more and no cursor",
        ),
        (
            pages(vec![
                (None, first.clone()),
                (Some("c1"), page_with(rest, 150, Some("c1"), json!({}))),
            ]),
            "a cursor that does not move",
        ),
        (
            pages(vec![
                (None, first.clone()),
                (Some("c1"), page_with(&[], 150, Some("c2"), json!({}))),
                (Some("c2"), page_with(rest, 150, None, json!({}))),
            ]),
            "an empty page with more to come",
        ),
        (
            pages(vec![
                (
                    None,
                    page_with(&all[..100], 150, Some("c1"), json!({"hasNextPage": "yes"})),
                ),
                (Some("c1"), page_with(rest, 150, None, json!({}))),
            ]),
            "more, not said plainly",
        ),
        (
            pages(vec![
                (None, first.clone()),
                (
                    Some("c1"),
                    merged(
                        &page_with(rest, 150, None, json!({})),
                        json!({"errors": [{"type": "RATE_LIMITED"}]}),
                    ),
                ),
            ]),
            "an error not a deletion",
        ),
    ];
    for (pages, label) in cases {
        assert!(
            matches!(read_long(pages, 150), Err(ReadError::GitHub(_))),
            "{label}"
        );
    }
}

#[test]
fn a_comment_by_a_deleted_account_does_not_fail_the_history_read() {
    // GitHub can answer a deleted account's comment with no author, where
    // the listing names it `ghost`. Refused as incomplete, one such comment
    // anywhere failed the history read for every pull request in the batch.
    let ghost = merged(&long_comment(3), json!({"author": null}));
    let window = json!({"data": {"repository": {
        "pr1": {"number": 1, "comments": {"totalCount": 1, "nodes": [ghost]}, "timelineItems": {"nodes": []}},
        "pr2": {"number": 2, "comments": {"totalCount": 1, "nodes": [long_comment(1)]}, "timelineItems": {"nodes": []}}}}});
    let mut api = api(move |_| ok(window.clone()));
    let history = api.histories(&[int(1), int(2)]).unwrap();
    assert_py(field(&history[&int(1)].comments[0], "user"), json!("ghost"));
    let record = parse_controller_state(&history[&int(2)].comments)
        .unwrap()
        .unwrap();
    assert_py(&record.comment_id, json!(1));
    // And on a later page of a long conversation.
    let mut all = everything(150);
    all[120] = merged(&all[120], json!({"author": null}));
    let history = read_long_with(
        pages(vec![
            (None, comment_page(&all[..100], 150, Some("c1"))),
            (Some("c1"), comment_page(&all[100..], 150, None)),
        ]),
        all,
    );
    let comments = &history[&int(1)].comments;
    assert_py(field(&comments[120], "user"), json!("ghost"));
    assert_py(
        &parse_controller_state(comments)
            .unwrap()
            .unwrap()
            .comment_id,
        json!(1),
    );
}

#[test]
fn a_conversation_one_past_the_window_is_paged_and_one_within_it_is_not() {
    // 101 comments do not fit the batched window of 100, and the record may
    // be the one left out of it. A hundred, or none, fit: one query.
    let all = everything(101);
    let history = read_long(
        pages(vec![
            (None, comment_page(&all[..100], 101, Some("c1"))),
            (Some("c1"), comment_page(&all[100..], 101, None)),
        ]),
        101,
    )
    .unwrap();
    assert_eq!(history[&int(1)].comments.len(), 101);
    assert_py(
        &parse_controller_state(&history[&int(1)].comments)
            .unwrap()
            .unwrap()
            .comment_id,
        json!(1),
    );
    for total in [0, 100] {
        let answer = history_response(
            json!({"comments": {"totalCount": total, "nodes": everything(total)}}),
        );
        let mut api = api(move |_| ok(answer.clone()));
        assert_eq!(
            api.histories(&[int(1)]).unwrap()[&int(1)].comments.len(),
            total as usize
        );
        assert_eq!(calls(&api).len(), 1, "{total}");
    }
}

#[test]
fn batched_history_of_nothing_asks_nothing() {
    let mut api = api(|call| panic!("nothing should be asked, but {call} was"));
    assert!(api.histories(&[]).unwrap().is_empty());
}

#[test]
fn the_batched_query_names_each_pull_request_once_in_order() {
    // `sorted(set(numbers))`: the query text is the replay key, so its
    // aliases must come out as Python's do.
    let mut api = api(|_| ok(json!({"data": {"repository": {}}})));
    api.histories(&[int(12), int(3), int(12)]).unwrap();
    let asked = query(&calls(&api)[0]);
    assert!(asked.contains("{pr3: pullRequest(number:3) { ...history }\npr12: pullRequest(number:12) { ...history } } }"));
}

#[test]
fn should_recover_latest_inactive_transition_from_timeline() {
    let events = json!([
        {"event": "closed", "created_at": "2026-09-02T00:00:00Z"},
        {"event": "reopened", "created_at": "2026-09-03T00:00:00Z"},
        {"event": "convert_to_draft", "created_at": "2026-09-04T00:00:00Z"},
        {"event": "ready_for_review", "created_at": "2026-09-05T00:00:00Z"}]);
    let mut api = api(move |_| page(events.clone()));
    assert_eq!(
        api.activity(&int(1)).unwrap().as_deref(),
        Some("2026-09-04T00:00:00Z")
    );
    assert_eq!(
        path(&calls(&api)[0]),
        "repos/dashpay/platform/issues/1/timeline?per_page=100"
    );
    let mut api = crate::support::api(|_| page(json!([])));
    assert_eq!(api.activity(&int(1)).unwrap(), None);
}

#[test]
fn the_latest_transition_is_the_latest_instant_whatever_its_offset() {
    // Compared as instants, not as text, and the first spelling of an
    // instant is kept: `max` over `(instant, text)` keeps the larger text of
    // two equal instants.
    let events = json!([
        {"event": "closed", "created_at": "2026-09-04T01:00:00+01:00"},
        {"event": "closed", "created_at": "2026-09-03T23:30:00Z"},
        {"event": "convert_to_draft", "created_at": "2026-09-04T00:00:00Z"}]);
    let mut api = api(move |_| page(events.clone()));
    assert_eq!(
        api.activity(&int(1)).unwrap().as_deref(),
        Some("2026-09-04T01:00:00+01:00")
    );
}

#[test]
fn should_not_query_unrelated_roster_permissions() {
    let policy = py(
        json!({"fallback": {"owners": ["fallback"], "reviewers": []}, "areas": [
        {"paths": ["packages/rs-drive/"], "owners": ["drive-owner"], "reviewers": []},
        {"paths": ["packages/swift-sdk/"], "owners": ["swift-owner"], "reviewers": []}]}),
    );
    let mut api = api(Fixture::default().route());
    let result = api.snapshot(&int(1), &policy, None).unwrap();
    assert_py(
        field(&result, "permissions"),
        json!({"drive-owner": "write"}),
    );
    // One list answers for everyone, instead of a request per person.
    assert!(permission_paths(&api).is_empty());
}

#[test]
fn access_reads_one_list_and_distrusts_what_it_does_not_recognise() {
    let mut api = api(Fixture::default().route());
    api.access().unwrap();
    let access = api.access().unwrap().clone();
    assert_eq!(calls(&api).len(), 1);
    assert_eq!(access["drive-owner"].as_deref(), Some("write"));
    assert_eq!(access["swift-owner"].as_deref(), Some("read"));
    // A custom organisation role has a name this policy never heard of, but
    // its capabilities still say whether the holder can push.
    assert_eq!(access["custom-role"].as_deref(), Some("write"));
    // Anyone absent from the list has no access at all.
    assert!(!access.contains_key("stranger"));
}

#[test]
fn a_file_read_without_its_patch_says_nothing_about_being_the_same_work() {
    // Line counts are two small integers, and they are equal for the one
    // case the patch's digest exists to catch, so a file without a patch
    // carries nothing.
    let read = |file: Value| {
        let snapshot = snapshot(Fixture {
            files: Some(json!([file])),
            ..Fixture::default()
        })
        .unwrap();
        items(field(&snapshot, "files"))[0].clone()
    };
    let no_patch = read(
        json!({"filename": "a.rs", "status": "modified", "sha": "c".repeat(40),
                               "additions": 1, "deletions": 1, "changes": 2}),
    );
    assert_py(
        &no_patch,
        json!({"filename": "a.rs", "status": "modified", "content": "c".repeat(40)}),
    );
    // The digest is SHA-256 of the patch's UTF-8, as Python's.
    let with_patch = read(
        json!({"filename": "a.rs", "status": "modified", "sha": "c".repeat(40),
                                 "patch": "@@ -1 +1 @@\n-a\n+\u{e9}"}),
    );
    assert_py(
        field(&with_patch, "shape"),
        json!("b2d8eb69f57a270882e7ec29092bce1b837f5885ecf69fb6b997576193be7076"),
    );
    // A file the pull request adds is not in the merge base, so nothing the
    // base did can be hiding in its patch and the blob decides it alone.
    let added = read(
        json!({"filename": "golden.bin", "status": "added", "sha": "c".repeat(40),
                            "additions": 0, "deletions": 0, "changes": 0}),
    );
    assert_py(field(&added, "shape"), json!("added"));
}

#[test]
fn both_reads_of_one_pull_request_see_the_same_comments() {
    // Comments come by one route, with the editor beside each, so the read
    // before the verdict and the read before the write cannot disagree.
    let diff = json!({"number": 1, "diff": "a".repeat(64), "diff_heads": ["b".repeat(40)],
                      "diff_seen": "2026-09-11T10:00:00Z"});
    let body = record_body(
        json!({"version": 1, "number": 1, "head": "b".repeat(40), "admitted_at": null, "ready_since": null,
               "state": "ready-for-human", "evidence": "c".repeat(64), "context": "d".repeat(64)}),
        "text",
        Some(diff.clone()),
    );
    let comment = json!({"id": 7, "user": "github-actions[bot]", "body": body,
                         "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T12:00:00Z",
                         "edited_by": "github-actions[bot]"});
    let mut api = api(Fixture {
        comments: vec![comment.clone()],
        ..Fixture::default()
    }
    .route());
    let read = api.snapshot(&int(1), &plain_policy(), None).unwrap();
    let history = History {
        comments: vec![py(comment)],
        lifecycle_at: None,
    };
    let reused = api
        .snapshot(&int(1), &plain_policy(), Some(&history))
        .unwrap();
    // The editor is read with its type, so it carries `[bot]` like an author.
    assert_py(
        field(&items(field(&read, "comments"))[0], "edited_by"),
        json!("github-actions[bot]"),
    );
    assert_py(field(&read, "controller_diff"), diff);
    assert!(py_eq(
        field(&read, "controller_diff"),
        field(&reused, "controller_diff")
    ));
}

#[test]
fn a_file_that_changed_type_is_listed_twice_and_is_not_drift() {
    // A regular file becoming a symlink is one path listed twice — removed
    // and added, a different blob each time. Reading it as the pagination
    // shifting under the read left the pull request an error status on a
    // required check.
    let read = snapshot(Fixture {
        files: Some(json!([
            {"filename": "CLAUDE.md", "status": "removed", "sha": "a".repeat(40), "patch": "@@ -1 +0,0 @@\n-x"},
            {"filename": "CLAUDE.md", "status": "added", "sha": "b".repeat(40), "patch": "@@ -0,0 +1 @@\n+docs/x"}])),
        ..Fixture::default()
    })
    .unwrap();
    let statuses: Vec<_> = items(field(&read, "files"))
        .iter()
        .map(|f| text(field(f, "status")).to_owned())
        .collect();
    assert_eq!(statuses, ["removed", "added"]);
    // Still drift when the same path arrives twice the same way.
    let same = json!({"filename": "a.rs", "status": "modified", "sha": "a".repeat(40),
                      "patch": "@@ -1 +1 @@\n-a\n+b"});
    let result = snapshot(Fixture {
        files: Some(json!([same.clone(), same])),
        ..Fixture::default()
    });
    assert!(github_error(result).starts_with("Duplicate"));
}

#[test]
fn who_the_pull_request_was_handed_to_is_read() {
    // A hand-over is written down as an assignment, and every route that
    // reads a pull request carries it: a field one read supplies and another
    // does not is how three defects in a day began.
    let read = snapshot(Fixture::default()).unwrap();
    assert_py(field(&read, "assignees"), json!(["romchornyi"]));
    let mut one = api(|_| ok(pr()));
    assert_py(
        field(&one.pull(&int(1)).unwrap(), "assignees"),
        json!(["romchornyi"]),
    );
    let mut open = api(|_| page(json!([pr()])));
    assert_py(
        field(&open.open_prs().unwrap()[0], "assignees"),
        json!(["romchornyi"]),
    );
}

#[test]
fn should_preserve_rename_source_and_reuse_access_until_told_otherwise() {
    let fixture = Fixture {
        files: Some(
            json!([{"filename": "new/a.rs", "previous_filename": "old/a.rs", "status": "renamed"}]),
        ),
        ..Fixture::default()
    };
    let mut api = api(fixture.route());
    let listings = |api: &GitHub<Fake>| {
        calls(api)
            .iter()
            .filter(|c| path(c).contains("/collaborators"))
            .count()
    };
    let first = api.snapshot(&int(1), &plain_policy(), None).unwrap();
    api.snapshot(&int(1), &plain_policy(), None).unwrap();
    // Access is read once per reconciliation rather than once per pull
    // request: repeating it was a large share of the traffic.
    assert_eq!(listings(&api), 1);
    // The check made immediately before writing must not trust that.
    api.forget_cached_access();
    api.snapshot(&int(1), &plain_policy(), None).unwrap();
    assert_eq!(listings(&api), 2);
    assert_py(
        field(&items(field(&first, "files"))[0], "previous_filename"),
        json!("old/a.rs"),
    );
    assert_py(field(&first, "complete"), json!(true));
}

#[test]
fn the_snapshot_holds_pythons_keys_in_pythons_order() {
    // `evaluate` reads the snapshot as a dict, and the corpus compares it
    // key for key; the order is the one `snapshot` builds it in, labels kept
    // where the identity first put them.
    let read = snapshot(Fixture::default()).unwrap();
    let PyValue::Dict(fields) = &read else {
        unreachable!()
    };
    let keys: Vec<&str> = fields.keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "number",
            "author",
            "author_is_bot",
            "body",
            "labels",
            "assignees",
            "head",
            "base",
            "base_sha",
            "created_at",
            "draft",
            "state",
            "url",
            "title",
            "files",
            "reviews",
            "comments",
            "threads",
            "lifecycle_at",
            "head_seen_at",
            "build",
            "ready_published",
            "requested_reviewers",
            "controller_state",
            "controller_comment_id",
            "controller_diff",
            "permissions",
            "repo",
            "complete"
        ]
    );
}
