//! The boundary: the rules every answer is read by, the `gh api` command a
//! call becomes, and the replay of a recording.
//!
//! Ported from the `_run` and `pages` tests of `pr_review/tests/test_github.py`
//! and the read classification test of `test_conformance.py`, with the
//! rules the conformance README states about the boundary.

use crate::support::*;
use pr_hygiene_engine::evidence::replay::{gh_arguments, is_read};
use pr_hygiene_engine::evidence::{ReplayTransport, Scripted, Sleep};

use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A sleeper that only remembers how long it was asked to wait.
#[derive(Clone, Default)]
struct Waits(Arc<Mutex<Vec<Duration>>>);

impl Sleep for Waits {
    fn sleep(&mut self, duration: Duration) {
        self.0.lock().expect("one test thread").push(duration);
    }
}

fn client(answers: Vec<Answer>) -> (Client<Scripted>, Waits) {
    let waits = Waits::default();
    (
        Client::with_sleep(Scripted::new(answers), waits.clone()),
        waits,
    )
}

const PULL: &str = "repos/dashpay/platform/pulls/1";

#[test]
fn a_flaky_read_is_asked_once_more_and_a_post_never_is() {
    // One "unexpected end of JSON input" from gh marked a pull request an
    // error under the required check for hours.
    let (mut flaky, waits) = client(vec![
        failed(1, "", "unexpected end of JSON input"),
        ok(json!({"ok": true})),
    ]);
    assert_py(
        &flaky.request(Method::Get, PULL, None).unwrap(),
        json!({"ok": true}),
    );
    assert_eq!(flaky.transport().calls().len(), 2);
    assert_eq!(
        *waits.0.lock().expect("one test thread"),
        [Duration::from_secs(2)]
    );
    let (mut refused, _) = client(vec![failed(1, "", "HTTP 422: Validation Failed")]);
    assert!(refused.request(Method::Get, PULL, None).is_err());
    assert_eq!(
        refused.transport().calls().len(),
        1,
        "a refusal is not flakiness"
    );
    let (mut posted, _) = client(vec![failed(1, "", "unexpected end of JSON input")]);
    let body = Some(py(json!({"body": "x"})));
    assert!(posted
        .request(
            Method::Post,
            "repos/dashpay/platform/issues/1/comments",
            body
        )
        .is_err());
    assert_eq!(
        posted.transport().calls().len(),
        1,
        "a second POST could write twice"
    );
}

#[test]
fn a_timeout_gh_reports_is_asked_again() {
    // gh ran, the network did not answer in time, and gh said so: the call
    // never reached GitHub's decision, so asking again is safe for a read.
    let (mut api, _) = client(vec![
        failed(
            1,
            "",
            "Get \"https://api.github.com/x\": dial tcp 1.2.3.4:443: i/o timeout",
        ),
        ok(json!({"ok": true})),
    ]);
    assert_py(
        &api.request(Method::Get, PULL, None).unwrap(),
        json!({"ok": true}),
    );
    assert_eq!(api.transport().calls().len(), 2);
}

#[test]
fn pythons_own_timeout_is_not_asked_again() {
    // `gh` that ran out of Python's sixty seconds may still have been
    // working: the call is given up, not repeated, even for a read.
    let (mut api, waits) = client(vec![
        Err(TransportError::Failed(Failure::unavailable())),
        ok(json!({"ok": true})),
    ]);
    assert_eq!(
        github_error(api.request(Method::Get, PULL, None)),
        "GitHub API command unavailable or timed out"
    );
    assert_eq!(api.transport().calls().len(), 1);
    assert!(waits.0.lock().expect("one test thread").is_empty());
}

#[test]
fn a_call_is_asked_twice_at_most() {
    let (mut api, _) = client(vec![
        failed(1, "", "HTTP 502: Bad Gateway"),
        failed(1, "", "HTTP 502: Bad Gateway"),
        ok(json!({"ok": true})),
    ]);
    assert_eq!(
        github_error(api.request(Method::Get, PULL, None)),
        "GitHub API command failed (exit 1): HTTP 502: Bad Gateway"
    );
    assert_eq!(api.transport().calls().len(), 2);
}

#[test]
fn a_failure_quotes_what_gh_said_stripped_and_cut_to_three_hundred_characters() {
    let said = format!("  {}\n", "é".repeat(400));
    let (mut api, _) = client(vec![failed(4, "", &said)]);
    let message = github_error(api.request(Method::Get, PULL, None));
    assert_eq!(
        message,
        format!("GitHub API command failed (exit 4): {}", "é".repeat(300))
    );
    let (mut quiet, _) = client(vec![failed(1, "", " \n")]);
    assert_eq!(
        github_error(quiet.request(Method::Get, PULL, None)),
        "GitHub API command failed (exit 1)"
    );
}

#[test]
fn a_failing_rest_call_is_never_relaxed() {
    // The relaxation is for GraphQL only. Without that, every failing REST
    // read whose body happens to carry a data key would be a success.
    let (mut api, _) = client(vec![failed(1, r#"{"data": {"whatever": 1}}"#, "not found")]);
    assert!(matches!(
        api.request(Method::Get, PULL, None),
        Err(ReadError::GitHub(_))
    ));
}

#[test]
fn a_failed_graphql_query_that_still_carries_data_is_an_answer() {
    let body = r#"{"data": {"repository": null}, "errors": [{"type": "NOT_FOUND"}]}"#;
    let (mut api, _) = client(vec![failed(1, body, "GraphQL: Could not resolve")]);
    let answer = api.graphql("query { a }", py(json!({}))).unwrap();
    assert_py(
        &answer,
        json!({"data": {"repository": null}, "errors": [{"type": "NOT_FOUND"}]}),
    );
    for body in [
        r#"{"data": null}"#,
        r#"{"data": []}"#,
        "[1]",
        "not json",
        " ",
    ] {
        let (mut api, _) = client(vec![failed(1, body, "failed")]);
        assert!(
            matches!(
                api.graphql("query { a }", py(json!({}))),
                Err(ReadError::GitHub(_))
            ),
            "{body}"
        );
    }
}

#[test]
fn empty_output_is_none_and_output_that_is_not_json_is_an_error() {
    // Python strips with its own idea of whitespace, which includes \x1c.
    for empty in ["", "  \n", "\u{1c}\u{2028}"] {
        let (mut api, _) = client(vec![Ok(Reply::Text(empty.into()))]);
        assert!(matches!(
            api.request(Method::Get, PULL, None).unwrap(),
            PyValue::None
        ));
    }
    let (mut api, _) = client(vec![Ok(Reply::Text("{".into()))]);
    assert_eq!(
        github_error(api.request(Method::Get, PULL, None)),
        "GitHub API returned invalid JSON"
    );
    // A string that names half a surrogate pair is text Python holds and a
    // Rust string cannot. It takes the path invalid JSON takes, never
    // another value: the one divergence the reader accepts here.
    let (mut api, _) = client(vec![Ok(Reply::Text(r#"{"body": "\ud800"}"#.into()))]);
    assert_eq!(
        github_error(api.request(Method::Get, PULL, None)),
        "GitHub API returned invalid JSON"
    );
}

#[test]
fn a_refused_call_is_never_read_as_evidence() {
    // A recording that lacks a read, or a transport that will not write,
    // stops the run: it is neither retried nor taken for an unknown answer.
    let refusal = || Err(TransportError::Refused("not recorded".into()));
    let (mut api, _) = client(vec![refusal(), ok(json!({}))]);
    assert_eq!(
        api.request(Method::Get, PULL, None).unwrap_err(),
        ReadError::Refused("not recorded".into())
    );
    assert_eq!(api.transport().calls().len(), 1);
    let mut reader = GitHub::new(
        "dashpay/platform",
        Client::with_sleep(Scripted::new([page(json!([])), refusal()]), NoSleep),
    )
    .unwrap();
    assert!(matches!(
        reader.permission("someone"),
        Err(ReadError::Refused(_))
    ));
}

#[test]
fn should_flatten_all_rest_pages() {
    let (mut api, _) = client(vec![Ok(Reply::Text(r#"[[{"id":1}],[{"id":2}]]"#.into()))]);
    let items = api
        .pages("repos/dashpay/platform/issues?per_page=100")
        .unwrap();
    assert_py(&PyValue::List(items.into()), json!([{"id": 1}, {"id": 2}]));
    let Call::Rest { paginate, path, .. } = &api.transport().calls()[0] else {
        panic!("a REST call")
    };
    assert!(paginate);
    assert_eq!(path, "repos/dashpay/platform/issues?per_page=100");
}

#[test]
fn pages_ask_a_hundred_at_a_time_unless_told_otherwise() {
    for (asked, sent) in [
        (
            "repos/a/b/pulls?state=open",
            "repos/a/b/pulls?state=open&per_page=100",
        ),
        (
            "repos/a/b/pulls/1/files",
            "repos/a/b/pulls/1/files?per_page=100",
        ),
        (
            "repos/a/b/issues?per_page=30",
            "repos/a/b/issues?per_page=30",
        ),
    ] {
        let (mut api, _) = client(vec![page(json!([]))]);
        api.pages(asked).unwrap();
        assert_eq!(path(&api.transport().calls()[0]), sent);
    }
}

#[test]
fn pages_a_transport_fetched_one_by_one_read_as_gh_slurps_them() {
    let (mut api, _) = client(vec![Ok(Reply::Pages(vec![
        r#"[{"id": 1}]"#.into(),
        r#"[{"id": 2}, {"id": 3}]"#.into(),
    ]))]);
    let items = api.pages("repos/a/b/issues").unwrap();
    assert_py(
        &PyValue::List(items.into()),
        json!([{"id": 1}, {"id": 2}, {"id": 3}]),
    );
    let (mut broken, _) = client(vec![Ok(Reply::Pages(vec!["[1]".into(), "nope".into()]))]);
    assert_eq!(
        github_error(broken.pages("repos/a/b/issues")),
        "GitHub API returned invalid JSON"
    );
}

#[test]
fn should_fail_on_transport_and_non_list_pages() {
    let (mut api, _) = client(vec![failed(1, "", "API denied")]);
    assert_eq!(
        github_error(api.pages("repos/dashpay/platform/issues")),
        "GitHub API command failed (exit 1): API denied"
    );
    let (mut api, _) = client(vec![ok(json!([{"message": "not a list"}]))]);
    assert_eq!(
        github_error(api.pages("repos/dashpay/platform/issues")),
        "Expected paginated GitHub list"
    );
}

#[test]
fn empty_collection_is_valid_but_missing_evidence_is_not() {
    for output in ["[]", "[[]]"] {
        let (mut api, _) = client(vec![Ok(Reply::Text(output.into()))]);
        assert!(api
            .pages("repos/dashpay/platform/issues")
            .unwrap()
            .is_empty());
    }
    for output in ["null", ""] {
        let (mut api, _) = client(vec![Ok(Reply::Text(output.into()))]);
        assert_eq!(
            github_error(api.pages("repos/dashpay/platform/issues")),
            "Expected paginated GitHub list",
            "{output:?}"
        );
    }
}

#[test]
fn should_pass_untrusted_text_as_json_stdin() {
    // The body travels on stdin as JSON, never among the arguments a shell
    // or `gh` could read as anything else, and it is written byte for byte
    // as Python's `json.dumps` writes it.
    let body = "$(touch /tmp/never-run) `echo no`\n\"quotes\" résumé 👍";
    let call = Call::Rest {
        method: Method::Post,
        path: "repos/dashpay/platform/issues/1/comments".into(),
        body: Some(py(json!({ "body": body }))),
        paginate: false,
    };
    let (arguments, stdin) = gh_arguments(&call).unwrap();
    assert_eq!(
        arguments,
        [
            "--method",
            "POST",
            "repos/dashpay/platform/issues/1/comments",
            "--input",
            "-"
        ]
    );
    assert!(!arguments.iter().any(|a| a.contains("touch")));
    // From Python 3.12: everything outside ASCII escaped, an astral
    // character as a surrogate pair.
    let escaped = |code: &str| format!("\\u{code}");
    let python = format!(
        r#"{{"body": "$(touch /tmp/never-run) `echo no`\n\"quotes\" r{e}sum{e} {high}{low}"}}"#,
        e = escaped("00e9"),
        high = escaped("d83d"),
        low = escaped("dc4d"),
    );
    assert_eq!(stdin.as_deref(), Some(python.as_str()));
    assert!(stdin.unwrap().is_ascii());
}

#[test]
fn a_graphql_call_is_the_document_python_sends() {
    let mut variables = pr_hygiene_engine::pycompat::PyDict::new();
    variables.insert("owner".into(), py(json!("dashpay")));
    variables.insert("repo".into(), py(json!("platform")));
    variables.insert("number".into(), py(json!(2)));
    variables.insert("after".into(), PyValue::None);
    let call = Call::Graphql {
        query: "query { a }\n  b".into(),
        variables: PyValue::Dict(variables),
    };
    let (arguments, stdin) = gh_arguments(&call).unwrap();
    assert_eq!(arguments, ["--method", "POST", "graphql", "--input", "-"]);
    // From Python 3.12: the default separators, the variables in the order
    // the reader puts them.
    assert_eq!(
        stdin.as_deref(),
        Some(
            r#"{"query": "query { a }\n  b", "variables": {"owner": "dashpay", "repo": "platform", "number": 2, "after": null}}"#
        )
    );
    let listing = Call::Rest {
        method: Method::Get,
        path: "repos/a/b/pulls?state=open&per_page=100".into(),
        body: None,
        paginate: true,
    };
    assert_eq!(
        gh_arguments(&listing).unwrap(),
        (
            [
                "--method",
                "GET",
                "repos/a/b/pulls?state=open&per_page=100",
                "--paginate",
                "--slurp"
            ]
            .map(str::to_owned)
            .to_vec(),
            None
        )
    );
}

#[test]
fn only_a_proven_read_is_a_read() {
    let graphql = |query: &str| Call::Graphql {
        query: query.into(),
        variables: py(json!({})),
    };
    let rest = |method, path: &str, body: Option<Value>, paginate| Call::Rest {
        method,
        path: path.into(),
        body: body.map(py),
        paginate,
    };
    let reads = [
        rest(Method::Get, "repos/a/b/pulls/1", None, false),
        rest(
            Method::Get,
            "repos/a/b/pulls?state=open&per_page=100",
            None,
            true,
        ),
        graphql("query($owner:String!) { viewer { login } }"),
    ];
    let writes = [
        rest(
            Method::Post,
            "repos/a/b/issues/1/comments",
            Some(json!({"body": "x"})),
            false,
        ),
        rest(
            Method::Patch,
            "repos/a/b/pulls/1",
            Some(json!({"body": "x"})),
            false,
        ),
        rest(Method::Delete, "repos/a/b/issues/comments/5", None, false),
        rest(Method::Put, "repos/a/b/pulls/1/merge", None, false),
        graphql("mutation { addComment(input: {}) { clientMutationId } }"),
        graphql("query { a } mutation { b }"),
        rest(Method::Get, "repos/a/b/pulls/1", Some(json!({})), false),
        rest(Method::Get, "/graphql?query=x", None, false),
        rest(
            Method::Get,
            "https://api.github.com/repos/a/b/pulls/1",
            None,
            false,
        ),
        rest(Method::Get, "user", None, false),
    ];
    for call in &reads {
        assert!(is_read(call), "{call}");
    }
    for call in &writes {
        assert!(!is_read(call), "{call}");
    }
}

/// A recording's `calls.jsonl`, one entry per line.
fn recording(entries: &[Value]) -> ReplayTransport {
    let text: String = entries.iter().map(|e| format!("{e}\n")).collect();
    ReplayTransport::from_calls_jsonl(&text).expect("a readable recording")
}

fn read_entry(path: &str, exit: i64, stdout: &str, stderr: &str) -> Value {
    json!({"ordinal": 1, "kind": "read", "args": ["--method", "GET", path], "stdin": null,
           "exit": exit, "stdout": stdout, "stderr": stderr})
}

fn get(path: &str) -> Call {
    Call::Rest {
        method: Method::Get,
        path: path.into(),
        body: None,
        paginate: false,
    }
}

#[test]
fn a_replay_serves_each_recorded_answer_once_in_order() {
    let mut replay = recording(&[
        read_entry(PULL, 1, "", "HTTP 502: Bad Gateway"),
        read_entry(PULL, 0, r#"{"n": 1}"#, ""),
        json!({"ordinal": 3, "kind": "write", "args": ["--method", "DELETE", "repos/dashpay/platform/issues/comments/5"],
               "stdin": null, "exit": 0, "stdout": "", "stderr": ""}),
    ]);
    match replay.call(&get(PULL)) {
        Err(TransportError::Failed(failure)) => {
            assert!(failure.transient, "a 502 is worth one more try");
            assert_eq!(failure.status, Some(1));
            assert_eq!(failure.detail, "HTTP 502: Bad Gateway");
        }
        other => panic!("the recorded failure first, got {other:?}"),
    }
    assert_eq!(
        replay.call(&get(PULL)),
        Ok(Reply::Text(r#"{"n": 1}"#.into()))
    );
    // Asked once more than Python asked it.
    assert!(matches!(
        replay.call(&get(PULL)),
        Err(TransportError::Refused(why)) if why.contains("more often")
    ));
    // Never asked at all.
    assert!(matches!(
        replay.call(&get("repos/dashpay/platform/pulls/2")),
        Err(TransportError::Refused(why))
            if why == "A read the recording does not hold: gh api --method GET repos/dashpay/platform/pulls/2"
    ));
    // A write is not replayed, even one the recording holds.
    let delete = Call::Rest {
        method: Method::Delete,
        path: "repos/dashpay/platform/issues/comments/5".into(),
        body: None,
        paginate: false,
    };
    assert!(matches!(
        replay.call(&delete),
        Err(TransportError::Refused(_))
    ));
}

#[test]
fn a_replay_matches_the_request_byte_for_byte() {
    // Python's own replay matches bodies as JSON values; this one holds the
    // port to the exact text Python sent, so a request written differently
    // is a request the recording does not hold.
    let python = r#"{"query": "query { a }", "variables": {"owner": "a", "repo": "b"}}"#;
    let entry = json!({"ordinal": 1, "kind": "read", "args": ["--method", "POST", "graphql", "--input", "-"],
                       "stdin": python, "exit": 0, "stdout": "{\"data\": {}}", "stderr": ""});
    let mut variables = pr_hygiene_engine::pycompat::PyDict::new();
    variables.insert("owner".into(), py(json!("a")));
    variables.insert("repo".into(), py(json!("b")));
    let call = Call::Graphql {
        query: "query { a }".into(),
        variables: PyValue::Dict(variables.clone()),
    };
    let mut replay = recording(std::slice::from_ref(&entry));
    assert!(replay.call(&call).is_ok());
    // The same variables in another order are another request.
    let mut reordered = pr_hygiene_engine::pycompat::PyDict::new();
    reordered.insert("repo".into(), py(json!("b")));
    reordered.insert("owner".into(), py(json!("a")));
    let mut replay = recording(&[entry]);
    let other = Call::Graphql {
        query: "query { a }".into(),
        variables: PyValue::Dict(reordered),
    };
    assert!(matches!(
        replay.call(&other),
        Err(TransportError::Refused(_))
    ));
}

#[test]
fn a_reader_moves_to_another_thread_with_its_transport() {
    // The service runs the engine on a blocking thread: the reader, its
    // client and its sleeper have to be able to go there.
    fn sendable<T: Send>(_: &T) {}
    let reader = GitHub::new("dashpay/platform", Client::new(ReplayTransport::default())).unwrap();
    sendable(&reader);
    let boxed: Box<dyn Transport + Send> = Box::new(Scripted::new([ok(json!({}))]));
    let mut through_a_box = Client::with_sleep(boxed, NoSleep);
    assert_py(
        &through_a_box.request(Method::Get, PULL, None).unwrap(),
        json!({}),
    );
}

#[test]
fn a_call_python_could_not_run_replays_as_one_that_never_completed() {
    let mut replay = recording(&[
        json!({"ordinal": 1, "kind": "read", "args": ["--method", "GET", PULL],
                                        "stdin": null, "exit": null, "stdout": "", "stderr": "",
                                        "raised": "TimeoutExpired"}),
    ]);
    assert_eq!(
        replay.call(&get(PULL)),
        Err(TransportError::Failed(Failure::unavailable()))
    );
}
