//! A head's build: which checks count, and how the rollup is read.
//!
//! Ported from the build tests of `pr_review/tests/test_github.py`.

use crate::support::*;
use pr_hygiene_engine::evidence::builds::{build_verdict, Build};

const OURS: &str = "/dashpay/x/actions/workflows/pr-review-policy.yml";
const CI: &str = "/dashpay/x/actions/workflows/ci.yml";

/// A check run as the rollup lists it.
fn check(name: &str, conclusion: Option<&str>, started: &str, workflow: &str, job: bool) -> Value {
    json!({"__typename": "CheckRun", "name": name, "conclusion": conclusion, "status": "COMPLETED",
           "startedAt": started,
           "detailsUrl": if job { "https://github.com/dashpay/x/actions/runs/1/job/2" }
                         else { "https://example.test/report" },
           "checkSuite": {"workflowRun": {"workflow": {"resourcePath": workflow}}}})
}

fn ci(name: &str, conclusion: &str, started: &str) -> Value {
    check(name, Some(conclusion), started, CI, true)
}

fn verdict(nodes: Value) -> Build {
    let PyValue::List(nodes) = py(nodes) else {
        panic!("not a list")
    };
    build_verdict(&nodes).expect("a readable check list")
}

/// The rollup query's answer for one page of checks.
pub fn rollup(nodes: Value, head: &str, more: bool, cursor: Option<&str>) -> Value {
    let total = nodes.as_array().map_or(0, Vec::len);
    json!({"data": {"repository": {"pullRequest": {"commits": {"nodes": [{"commit": {
        "oid": head,
        "statusCheckRollup": {"contexts": {
            "totalCount": total,
            "pageInfo": {"hasNextPage": more, "endCursor": cursor},
            "nodes": nodes}}}}]}}}}})
}

#[test]
fn re_running_a_flaky_check_clears_it() {
    // A re-run does not replace the run it repeats, it adds another beside
    // it, so the failure stays on the head for ever. The newest run decides,
    // and the newest is not the last one listed.
    assert_eq!(
        verdict(json!([
            ci("tests", "SUCCESS", "2026-09-01T11:00:00Z"),
            ci("tests", "FAILURE", "2026-09-01T10:00:00Z")
        ])),
        Build::Green
    );
    assert_eq!(
        verdict(json!([
            ci("tests", "FAILURE", "2026-09-01T11:00:00Z"),
            ci("tests", "SUCCESS", "2026-09-01T10:00:00Z")
        ])),
        Build::Failed
    );
}

#[test]
fn this_controller_never_waits_for_itself() {
    // Its own run ends cancelled on about a fifth of heads; counting any of
    // them would deadlock those pull requests with no error anywhere. The
    // decisive shape is its own run, still going, while it decides.
    let running = merged(
        &check(
            "policy / reconcile",
            None,
            "2026-09-01T10:02:00Z",
            OURS,
            true,
        ),
        json!({"status": "IN_PROGRESS"}),
    );
    let nodes = json!([
        check("policy / reconcile", Some("CANCELLED"), "2026-09-01T10:00:00Z", OURS, true),
        check("policy / reconcile", Some("CANCELLED"), "2026-09-01T10:00:01Z", OURS, true),
        running,
        {"__typename": "StatusContext", "context": "PR Hygiene", "state": "pending",
         "createdAt": "2026-09-01T10:00:00Z"},
        ci("tests", "SUCCESS", "2026-09-01T10:00:00Z")]);
    assert_eq!(verdict(nodes), Build::Green);
}

#[test]
fn a_renamed_job_in_our_workflow_is_still_ours() {
    // Each repository owns its caller and may rename the job; the workflow
    // path is what the engine refuses to run from anywhere else.
    let renamed = merged(
        &check(
            "hygiene / anything",
            None,
            "2026-09-01T10:00:00Z",
            OURS,
            true,
        ),
        json!({"status": "IN_PROGRESS"}),
    );
    assert_eq!(verdict(json!([renamed])), Build::Green);
}

#[test]
fn another_tool_filed_into_our_suite_is_not_ours() {
    // GitHub files API-created check runs into whichever Actions suite is
    // current, so workflow attribution alone would swallow real failures.
    let report = check(
        "Clippy Report",
        Some("FAILURE"),
        "2026-09-01T10:00:00Z",
        OURS,
        false,
    );
    assert_eq!(verdict(json!([report])), Build::Failed);
}

#[test]
fn cancelled_alone_is_not_a_failure() {
    // It is what concurrency looks like.
    assert_eq!(
        verdict(json!([ci(
            "check-title",
            "CANCELLED",
            "2026-09-01T10:00:00Z"
        )])),
        Build::Green
    );
}

#[test]
fn nothing_to_check_is_green_not_blocked_for_ever() {
    assert_eq!(verdict(json!([])), Build::Green);
}

#[test]
fn skipped_and_neutral_do_not_fail() {
    assert_eq!(
        verdict(json!([ci("a", "SKIPPED", "1"), ci("b", "NEUTRAL", "1")])),
        Build::Green
    );
}

#[test]
fn unfinished_work_is_running_not_green() {
    for status in ["QUEUED", "IN_PROGRESS", "WAITING", "REQUESTED"] {
        let node = merged(
            &check("a", None, "1", CI, true),
            json!({ "status": status }),
        );
        assert_eq!(verdict(json!([node])), Build::Running, "{status}");
    }
    let pending = json!([{"__typename": "StatusContext", "context": "ci", "state": "pending",
                          "createdAt": "1"}]);
    assert_eq!(verdict(pending), Build::Running);
}

#[test]
fn a_finished_check_that_wants_a_human_is_not_waited_for() {
    // ACTION_REQUIRED is a conclusion, not a status: the check has finished
    // and needs someone. Calling it running would hold the pull request at
    // "waiting for the build" for ever.
    assert_eq!(
        verdict(json!([ci("deploy", "ACTION_REQUIRED", "1")])),
        Build::Failed
    );
}

#[test]
fn a_status_in_error_is_a_failure_not_a_pass() {
    // Integrations post `error` for infrastructure failures as often as
    // `failure` for test failures, and GraphQL spells it ERROR.
    for (state, expected) in [
        ("ERROR", Build::Failed),
        ("FAILURE", Build::Failed),
        ("PENDING", Build::Running),
        ("EXPECTED", Build::Green),
        ("SUCCESS", Build::Green),
    ] {
        let node = json!({"__typename": "StatusContext", "context": "ci", "state": state, "createdAt": "1"});
        assert_eq!(verdict(json!([node])), expected, "{state}");
    }
}

#[test]
fn a_status_and_a_check_of_the_same_name_are_different_checks() {
    let nodes = json!([
        {"__typename": "StatusContext", "context": "build", "state": "FAILURE", "createdAt": "1"},
        merged(&ci("build", "SUCCESS", "2"), json!({"checkSuite": null}))]);
    assert_eq!(verdict(nodes), Build::Failed);
}

#[test]
fn the_same_job_name_in_two_workflows_does_not_mask_the_other() {
    let nodes = json!([
        check(
            "build",
            Some("SUCCESS"),
            "2026-09-01T11:00:00Z",
            "/dashpay/x/actions/workflows/a.yml",
            true
        ),
        check(
            "build",
            Some("FAILURE"),
            "2026-09-01T10:00:00Z",
            "/dashpay/x/actions/workflows/b.yml",
            true
        )
    ]);
    assert_eq!(verdict(nodes), Build::Failed);
}

#[test]
fn a_check_list_python_could_not_read_is_not_read_here_either() {
    // Python raises a TypeError ordering a time against a string, and an
    // AttributeError on a check suite that is not an object; neither is a
    // verdict.
    let PyValue::List(mixed) = py(json!([
        ci("tests", "SUCCESS", "2026-09-01T11:00:00Z"),
        merged(&ci("tests", "FAILURE", "x"), json!({"startedAt": 5}))
    ])) else {
        unreachable!()
    };
    assert!(matches!(
        build_verdict(&mixed),
        Err(ReadError::Exception {
            class: pr_hygiene_engine::evidence::PyClass::TypeError,
            ..
        })
    ));
    let PyValue::List(odd) = py(json!([merged(
        &ci("tests", "SUCCESS", "1"),
        json!({"checkSuite": "x"})
    )])) else {
        unreachable!()
    };
    assert!(matches!(
        build_verdict(&odd),
        Err(ReadError::Exception {
            class: pr_hygiene_engine::evidence::PyClass::AttributeError,
            ..
        })
    ));
}

#[test]
fn more_than_one_page_of_checks_is_read_not_refused() {
    // A pull request can carry more than a hundred contexts. Refusing would
    // error every pull request of that author.
    let head = "a".repeat(40);
    let pages = vec![
        ok(rollup(
            json!([ci("a", "SUCCESS", "1")]),
            &head,
            true,
            Some("next"),
        )),
        ok(rollup(json!([ci("b", "FAILURE", "1")]), &head, false, None)),
    ];
    let mut queue = pages.into_iter();
    let mut api = api(move |_| queue.next().expect("asked once per page"));
    assert_eq!(api.build_state(&int(1), &head).unwrap(), Build::Failed);
    let asked: Vec<_> = calls(&api)
        .iter()
        .map(|c| variable_text(c, "cursor"))
        .collect();
    assert_eq!(asked, [None, Some("next".to_owned())]);
}

#[test]
fn a_partial_answer_is_never_read_as_no_checks() {
    // statusCheckRollup is nullable, so a rate-limited read nulls it and
    // reports the error beside it. Reading that as "this repository has no
    // CI" would quietly open the gate for every pull request in the run.
    let partial = json!({"data": {"repository": {"pullRequest": {"commits": {"nodes": [
        {"commit": {"oid": "a".repeat(40), "statusCheckRollup": null}}]}}}},
        "errors": [{"type": "RATE_LIMITED", "message": "rate limited"}]});
    // gh exits 1 on it, and the client hands the data on.
    let body = partial.to_string();
    let mut api = api(move |_| failed(1, &body, "rate limited"));
    assert_eq!(
        github_error(api.build_state(&int(1), &"a".repeat(40))),
        "Build state query failed"
    );
}

#[test]
fn the_answer_is_read_once_per_head() {
    let head = "a".repeat(40);
    let answer = rollup(json!([]), &head, false, None);
    let mut api = api(move |_| ok(answer.clone()));
    api.build_state(&int(1), &head).unwrap();
    api.build_state(&int(1), &head).unwrap();
    assert_eq!(calls(&api).len(), 1);
}

#[test]
fn a_head_that_moved_under_the_read_is_not_reported_green() {
    let answer = rollup(json!([]), &"b".repeat(40), false, None);
    let mut api = api(move |_| ok(answer.clone()));
    assert_eq!(
        api.build_state(&int(1), &"a".repeat(40)).unwrap(),
        Build::Running
    );
}

#[test]
fn a_pull_request_with_no_checks_at_all_is_green() {
    let empty = json!({"data": {"repository": {"pullRequest": {"commits": {"nodes": [
        {"commit": {"oid": "a".repeat(40), "statusCheckRollup": null}}]}}}}});
    let mut api = api(move |_| ok(empty.clone()));
    assert_eq!(
        api.build_state(&int(1), &"a".repeat(40)).unwrap(),
        Build::Green
    );
}

#[test]
fn the_check_before_a_write_reads_the_build_afresh() {
    // A head's build is read once per reconciliation, but the re-read made
    // immediately before writing must not trust a green read minutes ago:
    // a check that failed since would be posted over.
    let head = "a".repeat(40);
    let green = rollup(json!([ci("tests", "SUCCESS", "1")]), &head, false, None);
    let mut api = api(move |_| ok(green.clone()));
    assert_eq!(api.build_state(&int(1), &head).unwrap(), Build::Green);
    api.forget_cached_access();
    let red = rollup(json!([ci("tests", "FAILURE", "2")]), &head, false, None);
    reroute(&mut api, move |_| ok(red.clone()));
    assert_eq!(api.build_state(&int(1), &head).unwrap(), Build::Failed);
    assert_eq!(calls(&api).len(), 1);
}

#[test]
fn more_checks_with_nowhere_to_read_them_from_is_refused() {
    // Asking for the next page without a cursor would ask for the first page
    // again, for ever.
    let head = "a".repeat(40);
    let stuck = rollup(json!([ci("tests", "SUCCESS", "1")]), &head, true, None);
    let mut api = api(move |_| ok(stuck.clone()));
    assert_eq!(
        github_error(api.build_state(&int(1), &head)),
        "Check pagination did not advance"
    );
}
