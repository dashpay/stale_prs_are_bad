//! The live reads over HTTP against a stand-in for GitHub that answers as a
//! synthetic recording's GitHub answered: what the job does, with the
//! recording's world served as "live".

#[path = "../../../engine/tests/support/no_content.rs"]
mod no_content;

use super::*;
use crate::reader::mock::{self, as_recorded, fixed_token, json_answer, Mock, Seen};
use crate::reader::StatusPage;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use pr_hygiene_engine::policy::fingerprint;
use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyDateTime, PyValue};
use pr_hygiene_engine::reconcile::ClockSite;
use std::time::Duration;
use tokio::runtime::Handle;

const REPO: &str = "dashpay/platform";
/// The instant every synthetic recording was made at.
const RECORDED: &str = "2026-09-12T10:00:00+00:00";

fn synthetic() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/synthetic")
}

/// A copy of a synthetic recording, under `into`.
fn copy(recording: &str, into: &Path) -> PathBuf {
    let to = into.join(recording);
    std::fs::create_dir_all(&to).unwrap();
    for file in no_content::FILES {
        std::fs::copy(synthetic().join(recording).join(file), to.join(file)).unwrap();
    }
    to
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn own() -> OwnWords {
    let source = |name: &str| {
        read(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../pr_review")
                .join(name),
        )
    };
    OwnWords::from_sources(&source("conformance.py"), &source("policy.py")).unwrap()
}

struct Stopped(PyDateTime);

impl Clock for Stopped {
    fn now(&mut self, _site: ClockSite) -> PyDateTime {
        self.0
    }
}

/// Each recorded read whose arguments `matches` accepts, its answer
/// changed by `change`: GitHub answering otherwise than it did.
fn edited(
    calls: &str,
    matches: impl Fn(&[String]) -> bool,
    change: impl Fn(&mut PyValue),
) -> String {
    let mut out = String::new();
    for line in calls.lines() {
        let PyValue::Dict(mut entry) = py_loads(line).unwrap() else {
            panic!("a call")
        };
        let args: Vec<String> = match entry.get("args") {
            Some(PyValue::List(args)) => args
                .iter()
                .map(|a| match a {
                    PyValue::Str(a) => a.clone(),
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        };
        if matches(&args) {
            if let Some(PyValue::Str(stdout)) = entry.get("stdout") {
                let mut answer = py_loads(stdout).unwrap();
                change(&mut answer);
                let text = py_dumps(&answer, false, None, None).unwrap();
                entry.insert("stdout".into(), PyValue::Str(text));
            }
        }
        out.push_str(&py_dumps(&PyValue::Dict(entry), false, None, None).unwrap());
        out.push('\n');
    }
    out
}

/// `answer[key] = value` for a JSON object, keeping the key in its place.
fn set(answer: &mut PyValue, key: &str, value: &str) {
    if let PyValue::Dict(fields) = answer {
        fields.insert(key.into(), PyValue::Str(value.into()));
    }
}

/// The first review of a listing, withdrawn.
fn withdraw_first_review(answer: &mut PyValue) {
    if let PyValue::List(pages) = answer {
        if let Some(PyValue::List(page)) = pages.iter_mut().next() {
            if let Some(review) = page.iter_mut().next() {
                set(review, "state", "DISMISSED");
            }
        }
    }
}

fn is_route(args: &[String], route: &str) -> bool {
    args.get(2).is_some_and(|asked| asked == route)
}

/// The recording's status page payload, as its recorder read it.
fn status_page_of(recording: &Path) -> Option<String> {
    let meta = py_loads(&read(&recording.join("recording.json"))).unwrap();
    match meta {
        PyValue::Dict(fields) => match fields.get("telemetry") {
            Some(PyValue::None) | None => None,
            Some(page) => Some(py_dumps(page, false, None, None).unwrap()),
        },
        _ => None,
    }
}

/// A stand-in for GitHub answering `calls`, and serving the review
/// system's status page `page` at `/status.json`.
async fn github(calls: &str, page: Option<String>) -> Mock {
    let answer = as_recorded(calls);
    mock::serve(move |seen: &Seen| {
        if seen.path() == "/status.json" {
            return match &page {
                Some(page) => json_answer(page),
                None => StatusCode::NOT_FOUND.into_response(),
            };
        }
        answer(seen)
    })
    .await
}

/// The live reads of `recording` against `github`, at `instant`, with
/// `budget` requests, as the job makes them: on a blocking thread, through
/// the read-only layer over the HTTP transport.
async fn live(github: &Mock, recording: &Path, instant: &str, budget: usize) -> Outcome {
    let (url, dirs, instant) = (
        github.url.clone(),
        vec![recording.to_owned()],
        instant.to_owned(),
    );
    let handle = Handle::current();
    mock::blocking(move || {
        let mut transport = || {
            let http = HttpTransport::with_origin(
                handle.clone(),
                fixed_token("ghs_test"),
                &url,
                Duration::from_secs(5),
            )?;
            Ok(ReadOnly::new(http, [REPO])?)
        };
        let page = StatusPage::with_url(handle.clone(), &format!("{url}/status.json")).unwrap();
        let mut status_page = || page.fetch();
        let mut clock = Stopped(PyDateTime::fromisoformat(&instant).unwrap());
        compare(
            &dirs,
            &own(),
            Settings {
                budget,
                report_prs: 2,
                slot: 0,
            },
            Reads {
                transport: &mut transport,
                clock: &mut clock,
                status_page: &mut status_page,
            },
        )
    })
    .await
}

/// Whether a request to GitHub is one of the engine's reads: a `GET`, or a
/// GraphQL document that is exactly one of the engine's own queries about
/// the repository, as the read-only layer judges one.
fn is_engine_read(request: &Seen) -> bool {
    if request.method == "GET" {
        return true;
    }
    if (request.method.as_str(), request.path()) != ("POST", "/graphql") {
        return false;
    }
    let Ok(PyValue::Dict(document)) = py_loads(&request.body) else {
        return false;
    };
    let (Some(PyValue::Str(query)), Some(variables)) =
        (document.get("query"), document.get("variables"))
    else {
        return false;
    };
    let call = pr_hygiene_engine::evidence::Call::Graphql {
        query: query.clone(),
        variables: variables.clone(),
    };
    ReadOnly::new((), [REPO]).unwrap().check(&call).is_ok()
}

/// Every request GitHub saw was a read: a `GET`, or one of the engine's
/// GraphQL queries — never a mutation.
#[track_caller]
fn only_reads(github: &Mock) {
    let seen = github.seen();
    let writes: Vec<String> = seen
        .iter()
        .filter(|request| request.path() != "/status.json" && !is_engine_read(request))
        .map(|request| format!("{} {}", request.method, request.path_and_query))
        .collect();
    assert!(writes.is_empty(), "sent: {writes:?}");
    assert!(
        seen.iter()
            .filter(|request| request.path() != "/status.json")
            .all(|request| request.headers["authorization"] == "token ghs_test"),
        "the job's token on every request to GitHub"
    );
}

/// The sources whose string literals are the live report's own words.
fn sources() -> Vec<PathBuf> {
    let service = Path::new(env!("CARGO_MANIFEST_DIR"));
    let engine = service.join("../engine/src/conformance");
    vec![
        service.join("src/differential.rs"),
        service.join("src/bin/differential-live.rs"),
        engine.join("report.rs"),
        engine.join("live.rs"),
        engine.join("compare.rs"),
        engine.join("diff.rs"),
    ]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_one_author_sync_read_live_as_recorded_matches_and_sends_no_write() {
    let recording = synthetic().join("sync-pr-2");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains("1 recording(s): the Rust engine matched every one read live."),
        "{said}"
    );
    // Snapshot and verdict matched. Python's own run wrote, so this one is
    // not held to writing nothing.
    let requests = outcome.spent;
    assert!(
        said.contains(&format!(
            "| dashpay/platform · sync --pr 2 | 1/1 | 1/1 | 0/0 | 0 | 0 | 1 | 0 | {requests} | 0 |"
        )),
        "{said}"
    );
    assert!(said.contains("No differences."), "{said}");
    // Every request it made is counted, and every one was a read.
    let to_github = github
        .seen()
        .iter()
        .filter(|request| request.path() != "/status.json")
        .count();
    assert_eq!(requests, to_github);
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_reports_pull_requests_read_live_as_recorded_match() {
    let recording = synthetic().join("report");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains(&format!(
            "| dashpay/platform · report | 2/2 | 0/0 | 0/0 | 0 | 0 | 0 | 0 | {} | 0 |",
            outcome.spent
        )),
        "{said}"
    );
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_pull_request_that_moved_between_the_reads_is_counted_not_failed() {
    let recording = synthetic().join("sync-pr-2");
    // A review withdrawn since Python read it: GitHub moved the pull
    // request's update time with it.
    let calls = edited(
        &read(&recording.join("calls.jsonl")),
        |args| is_route(args, "repos/dashpay/platform/pulls/2"),
        |answer| set(answer, "updated_at", "2026-09-12T10:03:00Z"),
    );
    let calls = edited(
        &calls,
        |args| is_route(args, "repos/dashpay/platform/pulls/2/reviews?per_page=100"),
        withdraw_first_review,
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "a move is no failure: {said}");
    assert!(
        said.contains("| dashpay/platform · sync --pr 2 | 0/1 | 0/1 | 0/0 | 1 | 0 | 1 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| live snapshot | — | moved during the read | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    only_reads(&github);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_build_that_started_between_the_reads_excuses_the_build_alone() {
    // GitHub records a check starting without moving the pull request's
    // update time: the build query's answer is what shows it moved.
    let recording = synthetic().join("sync-pr-2");
    let running = r#""statusCheckRollup": {"contexts": {"nodes": [{"__typename": "CheckRun", "name": "test", "status": "IN_PROGRESS", "conclusion": null, "startedAt": "2026-09-12T10:01:00Z", "checkSuite": null}], "pageInfo": {"hasNextPage": false, "endCursor": null}}}"#;
    let calls: String = read(&recording.join("calls.jsonl"))
        .lines()
        .map(|line| {
            let line = if line.contains("statusCheckRollup") {
                line.replace(
                    r#"\"statusCheckRollup\": null"#,
                    &running.replace('"', r#"\""#),
                )
            } else {
                line.to_owned()
            };
            format!("{line}\n")
        })
        .collect();
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    // The snapshot differs in its build alone, which the move excuses; the
    // verdict follows from the build, and is excused with it.
    assert!(
        said.contains("| dashpay/platform · sync --pr 2 | 0/1 | 0/1 | 0/0 | 1 | 0 | 1 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| live verdict | — | moved during the read | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_difference_where_nothing_moved_fails_by_its_field() {
    let recording = synthetic().join("sync-pr-2");
    // The same review withdrawn, and the pull request's update time as it
    // was: nothing says GitHub moved, so the difference is the port's.
    let calls = edited(
        &read(&recording.join("calls.jsonl")),
        |args| is_route(args, "repos/dashpay/platform/pulls/2/reviews?per_page=100"),
        withdraw_first_review,
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(said.contains("the Rust engine differed"), "{said}");
    assert!(
        said.contains(
            "| live snapshot | `pr.reviews[].state` | value | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    // The verdict that follows is not explained either, and says why not,
    // input by input, in the tool's own words alone.
    assert!(
        said.contains(
            "`verdict.self_reviewed_at`, `verdict.state` | dashpay/platform · sync --pr 2: 0 | \
             clock: the same instant; status page: read on neither side; admission: the same \
             instant; gh-printed control characters: every answer prints as it came |"
        ),
        "{said}"
    );
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_verdict_that_differs_by_the_clock_alone_is_explained() {
    // Admitted before: its waiting time runs to the instant the verdict is
    // decided at. Read two days on, that alone differs.
    let recording = synthetic().join("long-history");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, "2026-09-14T13:00:00+00:00", usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains(
            "| live verdict | `verdict.ready_since` | value, gone with Python's clock | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    assert!(
        said.contains("| dashpay/platform · sync --pr 2 | 1/1 | 0/1 | 0/0 | 0 | 1 | 1 | 0 |"),
        "{said}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_new_admission_dated_by_the_live_clock_is_explained_by_admission() {
    // Admitted by this very run, on both sides: the same pull request, and
    // only it, admitted at each engine's own instant.
    let recording = synthetic().join("sync-pr-2");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, "2026-09-12T10:04:00+00:00", usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains(
            "| live verdict | `verdict.admitted_at` | value, gone with Python's admission | 1 |"
        ),
        "{said}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_verdict_that_differs_by_the_status_page_alone_is_explained() {
    // Python read that the review bot's run on this pull request failed,
    // which makes asking it again due; read live, a newer event says it is
    // queued again, and asking would only add load.
    let recording = synthetic().join("nudge");
    let page = status_page_of(&recording).unwrap().replace(
            r#""recent_events": ["#,
            r#""recent_events": [{"kind": "head.queued", "repo": "dashpay/platform", "number": 2, "ts": "2026-09-12T09:58:00Z", "detail": "requeued"}, "#,
        );
    assert!(page.contains("requeued"), "{page}");
    let github = github(&read(&recording.join("calls.jsonl")), Some(page)).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    let pages: Vec<_> = github
        .seen()
        .into_iter()
        .filter(|s| s.path() == "/status.json")
        .collect();
    assert_eq!(pages.len(), 1, "{said}");
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains(
            "| live verdict | `verdict.nudge` | length, gone with Python's status page | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
}

/// A copy of long-history in which nothing has changed since the engine
/// last wrote: Python's own run wrote nothing. What GitHub serves is the copy's
/// calls.
fn settled(into: &Path) -> PathBuf {
    let recording = copy("long-history", into);
    let first = read(&recording.join("evaluations.jsonl"));
    let first = first.lines().next().unwrap();
    let pr = match py_loads(first).unwrap() {
        PyValue::Dict(fields) => fields.get("pr").unwrap().clone(),
        _ => panic!("an evaluation"),
    };
    let print = fingerprint(&pr).unwrap();
    let recorded = "e".repeat(64);
    for file in ["evaluations.jsonl", "calls.jsonl"] {
        let text = read(&recording.join(file));
        assert!(text.contains(&recorded));
        std::fs::write(recording.join(file), text.replace(&recorded, &print)).unwrap();
    }
    let calls: String = read(&recording.join("calls.jsonl"))
        .lines()
        .filter(|line| !line.contains(r#""kind": "write""#) && !line.contains(r#""kind":"write""#))
        .map(|line| format!("{line}\n"))
        .collect();
    std::fs::write(recording.join("calls.jsonl"), calls).unwrap();
    recording
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_would_be_write_where_nothing_changed_is_reported_and_never_sent() {
    let dir = tempfile::tempdir().unwrap();
    let recording = settled(dir.path());
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    // Held to writing nothing, it wanted to write: each write it would
    // have made is named by its route, and none of them reached GitHub.
    assert!(
        said.contains("| dashpay/platform · sync --pr 2 | 1/1 | 1/1 | 0/1 | 0 | 0 | 0 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains(r#"| live writes | `POST repos/*/*/statuses/* pending "Evaluating current review policy"` | would-be write, not sent |"#),
        "{said}"
    );
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[test]
fn a_panic_anywhere_in_a_recordings_reads_is_one_unreadable_row_and_never_said() {
    // Outside the live comparison itself: here, making the transport.
    let mut transport = || -> anyhow::Result<ReadOnly<HttpTransport>> {
        panic!("mallory has admin on dashpay/secret")
    };
    let mut clock = Stopped(PyDateTime::fromisoformat(RECORDED).unwrap());
    let outcome = compare(
        &[synthetic().join("sync-pr-2"), synthetic().join("report")],
        &own(),
        Settings {
            budget: 1000,
            report_prs: 2,
            slot: 0,
        },
        Reads {
            transport: &mut transport,
            clock: &mut clock,
            status_page: &mut || PyValue::None,
        },
    );
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(
        said.contains("- unreadable (sync-pr-2): the tool panicked here"),
        "{said}"
    );
    assert!(!said.contains("mallory"), "{said}");
    // What the panicked reads spent is not known: all that was left is
    // charged, and nothing more is read.
    assert_eq!(outcome.spent, 1000);
    assert!(
        said.contains("| dashpay/platform · report | 0/0 | 0/0 | 0/0 | 0 | 0 | 0 | 1 | 0 | 0 |"),
        "{said}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_recording_over_the_budget_is_skipped_unread() {
    let recording = synthetic().join("sync-pr-2");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, 10).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert_eq!(outcome.spent, 0);
    assert!(
        said.contains(
            "| dashpay/platform · sync --pr 2 | 0/0 | 0/0 | 0/0 | 0 | 0 | 0 | 1 | 0 | 0 |"
        ),
        "{said}"
    );
    assert!(github.seen().is_empty(), "nothing read");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn what_github_answered_live_is_never_printed() {
    // Distinctive text only the live answers hold, where it differs from
    // the recording: a review's body and a file's name.
    const BODY: &str = "Quokka-Zanzibar-7731 left a confidential note";
    const FILE: &str = "Marmalade-Obsidian-4402/private.rs";
    let recording = synthetic().join("sync-pr-2");
    let calls = edited(
        &read(&recording.join("calls.jsonl")),
        |args| is_route(args, "repos/dashpay/platform/pulls/2/reviews?per_page=100"),
        |answer| {
            if let PyValue::List(pages) = answer {
                if let Some(PyValue::List(page)) = pages.iter_mut().next() {
                    if let Some(review) = page.iter_mut().next() {
                        set(review, "body", BODY);
                    }
                }
            }
        },
    );
    let calls = edited(
        &calls,
        |args| is_route(args, "repos/dashpay/platform/pulls/2/files?per_page=100"),
        |answer| {
            if let PyValue::List(pages) = answer {
                if let Some(PyValue::List(page)) = pages.iter_mut().next() {
                    if let Some(file) = page.iter_mut().next() {
                        set(file, "filename", FILE);
                    }
                }
            }
        },
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(said.contains("`pr.reviews[].body`"), "{said}");
    for text in [BODY, FILE, "Quokka", "Marmalade", "ghs_test"] {
        assert!(!said.contains(text), "printed {text:?}:\n{said}");
    }
    no_content::assert_no_contents(&said, &recording, &sources());
}

/// `calls` without the calls of the given ordinals: as if Python's run had
/// not made those writes. GitHub, served from the reads, answers the same.
fn without(calls: &str, ordinals: &[i64]) -> String {
    calls
        .lines()
        .filter(|line| {
            let PyValue::Dict(entry) = py_loads(line).unwrap() else {
                panic!("a call")
            };
            !matches!(entry.get("ordinal"), Some(PyValue::Int(n))
                if ordinals.iter().any(|o| n.as_i64() == Some(*o)))
        })
        .map(|line| format!("{line}\n"))
        .collect()
}

// The synthetic sweep is a dry `sync` of every pull request: 2 and 5
// decided, 1 unreadable and marked so, 3 and 4 no longer governed and their
// marks taken off. Python's run writes to each.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_sync_of_every_pull_request_read_live_as_recorded_matches_and_sends_no_write() {
    let recording = synthetic().join("sweep");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    // Every pull request decided, compared one by one. Python's run wrote
    // to both, so neither is held to getting no write.
    let requests = outcome.spent;
    assert!(
        said.contains(&format!(
            "| dashpay/platform · sync | 2/2 | 2/2 | 0/0 | 0 | 0 | 2 | 0 | {requests} | 0 |"
        )),
        "{said}"
    );
    let coverage = outcome.coverage().unwrap();
    assert_eq!(
        coverage,
        "2 pull requests; snapshots 2/2, verdicts 2/2, no write 0/0; 0 moved, 0 explained, \
         2 unsettled; 0 differences"
    );
    let to_github = github
        .seen()
        .iter()
        .filter(|request| request.path() != "/status.json")
        .count();
    assert_eq!(requests, to_github, "every request counted");
    only_reads(&github);
    no_content::assert_no_contents(&format!("{said}\n{coverage}"), &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn in_a_sync_of_every_pull_request_each_one_python_wrote_nothing_to_is_held() {
    // Python's run, here, wrote to 2 and not to 5. The live run wants to
    // write to both: 2 is not held to anything, 5 is, and differs.
    let dir = tempfile::tempdir().unwrap();
    let recording = copy("sweep", dir.path());
    let calls = without(&read(&recording.join("calls.jsonl")), &[61, 64, 67]);
    std::fs::write(recording.join("calls.jsonl"), &calls).unwrap();
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(
        said.contains("| dashpay/platform · sync | 2/2 | 2/2 | 0/1 | 0 | 0 | 1 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            r#"| live writes | `POST repos/*/*/statuses/* pending "Evaluating current review policy"` | would-be write, not sent | 1 | dashpay/platform · sync: 1 |"#
        ),
        "{said}"
    );
    let coverage = outcome.coverage().unwrap();
    assert!(
        coverage.ends_with("no write 0/1; 0 moved, 0 explained, 1 unsettled; 1 differences"),
        "{coverage}"
    );
    // The status is named by the engine's own words; what it would write
    // into 5's description, its author's text, never is.
    assert!(!said.contains("Work in progress"), "{said}");
    only_reads(&github);
    no_content::assert_no_contents(&format!("{said}\n{coverage}"), &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_write_to_a_pull_request_neither_run_decided_is_named_by_route_alone() {
    // Python's run, here, marked no unreadable pull request and tidied
    // none off the policy. The live run marks 1, which it could not read
    // either: a write to a pull request neither run decided.
    let dir = tempfile::tempdir().unwrap();
    let recording = copy("sweep", dir.path());
    let calls = without(
        &read(&recording.join("calls.jsonl")),
        &[21, 69, 70, 71, 73, 76, 77],
    );
    std::fs::write(recording.join("calls.jsonl"), &calls).unwrap();
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(
        said.contains(
            r#"| live writes | `POST repos/*/*/statuses/* error "Incomplete policy evidence; reconciliation required", to a pull request neither run decided` | would-be write, not sent | 1 | dashpay/platform · sync: 0 |"#
        ),
        "{said}"
    );
    only_reads(&github);
    let coverage = outcome.coverage().unwrap();
    no_content::assert_no_contents(&format!("{said}\n{coverage}"), &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_sync_of_every_pull_request_over_the_budget_is_skipped_unread_and_says_so() {
    let recording = synthetic().join("sweep");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, 10).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert_eq!(outcome.spent, 0);
    assert!(github.seen().is_empty(), "nothing read");
    assert_eq!(
        outcome.coverage().unwrap(),
        "2 pull requests; not read live: over the live budget"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn only_a_sync_of_every_pull_request_is_a_repositorys_full_coverage() {
    let recording = synthetic().join("sync-pr-2");
    let github = github(&read(&recording.join("calls.jsonl")), None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    assert!(outcome.coverage().is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn what_github_answered_a_sync_of_every_pull_request_is_never_printed() {
    // Distinctive text only the live answers hold, in each decided pull
    // request: a review's body on 2 and a file's name on 5.
    const BODY: &str = "Wombat-Kilimanjaro-5519 wrote something private";
    const FILE: &str = "Tangerine-Basalt-0827/secret.rs";
    let recording = synthetic().join("sweep");
    let calls = edited(
        &read(&recording.join("calls.jsonl")),
        |args| is_route(args, "repos/dashpay/platform/pulls/2/reviews?per_page=100"),
        |answer| {
            if let PyValue::List(pages) = answer {
                if let Some(PyValue::List(page)) = pages.iter_mut().next() {
                    if let Some(review) = page.iter_mut().next() {
                        set(review, "body", BODY);
                    }
                }
            }
        },
    );
    let calls = edited(
        &calls,
        |args| is_route(args, "repos/dashpay/platform/pulls/5/files?per_page=100"),
        |answer| {
            if let PyValue::List(pages) = answer {
                if let Some(PyValue::List(page)) = pages.iter_mut().next() {
                    if let Some(file) = page.iter_mut().next() {
                        set(file, "filename", FILE);
                    }
                }
            }
        },
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = format!(
        "{}\n{}\n{}",
        outcome.printed(),
        outcome.summary(),
        outcome.coverage().unwrap()
    );
    assert!(!outcome.clean(), "{said}");
    for text in [BODY, FILE, "Wombat", "Tangerine", "ghs_test"] {
        assert!(!said.contains(text), "printed {text:?}:\n{said}");
    }
    no_content::assert_no_contents(&said, &recording, &sources());
}

/// `calls` with every recorded answer's `from` replaced by `to`: GitHub
/// answering otherwise than it did when Python read it.
fn rewritten(calls: &str, from: &str, to: &str) -> String {
    let mut out = String::new();
    let mut found = false;
    for line in calls.lines() {
        let PyValue::Dict(mut entry) = py_loads(line).unwrap() else {
            panic!("a call")
        };
        if let Some(PyValue::Str(stdout)) = entry.get("stdout") {
            if stdout.contains(from) {
                found = true;
                let changed = stdout.replace(from, to);
                entry.insert("stdout".into(), PyValue::Str(changed));
            }
        }
        out.push_str(&py_dumps(&PyValue::Dict(entry), false, None, None).unwrap());
        out.push('\n');
    }
    assert!(found, "the recording holds {from:?}");
    out
}

/// Pull request 2's one comment in the synthetic sweep, as GitHub's
/// history query answered it to Python.
const COMMENT: &str = r#""databaseId": 102, "body": "/self-reviewed cccccccccccccccccccccccccccccccccccccccc", "createdAt": "2026-09-11T11:00:00Z", "updatedAt": "2026-09-11T11:00:00Z""#;

/// The same comment rewritten in place, GitHub's update time of it at
/// `updated`.
fn rewrote(updated: &str) -> String {
    format!(
        r#""databaseId": 102, "body": "/self-reviewed cccccccccccccccccccccccccccccccccccccccc (summary updated)", "createdAt": "2026-09-11T11:00:00Z", "updatedAt": "{updated}""#
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_comment_edited_between_the_reads_moved_its_pull_request_and_no_other() {
    // A bot rewrote its comment in place after Python read it: GitHub moved
    // the comment's update time and not the pull request's.
    let recording = synthetic().join("sweep");
    let calls = rewritten(
        &read(&recording.join("calls.jsonl")),
        COMMENT,
        &rewrote("2026-09-12T10:03:00Z"),
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    // 2 moved, its snapshot and verdict with it; 5 is still held to
    // Python's, and matched.
    assert!(
        said.contains("| dashpay/platform · sync | 1/2 | 1/2 | 0/0 | 1 | 0 | 2 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| live snapshot | — | moved during the read | 1 | dashpay/platform · sync: 0 |"
        ),
        "{said}"
    );
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_comment_that_differs_with_its_update_time_unmoved_still_fails() {
    // The same body, read otherwise, with the comment's update time as
    // Python read it: nothing says GitHub changed it, so the difference is
    // the port's.
    let recording = synthetic().join("sweep");
    let calls = rewritten(
        &read(&recording.join("calls.jsonl")),
        COMMENT,
        &rewrote("2026-09-11T11:00:00Z"),
    );
    let github = github(&calls, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(
        said.contains(
            "| live snapshot | `pr.comments[].body` | value | 1 | dashpay/platform · sync: 0 |"
        ),
        "{said}"
    );
    no_content::assert_no_contents(&said, &recording, &sources());
}

/// Pull request 2's description in the synthetic sweep, as its listing and
/// its own read answered it.
const DESCRIPTION: &str = r#""body": "Some text.""#;

/// A copy of the sweep in which Python read pull request 2's description
/// as `printed`, where GitHub answers `raw`: Python's reads in its calls
/// and its `evaluate`'s `pr`, both as gh printed them to it. Returns the
/// copy and what GitHub serves.
fn description_read_as(into: &Path, printed: &str, raw: &str) -> (PathBuf, String) {
    let recording = copy("sweep", into);
    let calls = read(&recording.join("calls.jsonl"));
    std::fs::write(
        recording.join("calls.jsonl"),
        rewritten(&calls, DESCRIPTION, &format!(r#""body": {printed}"#)),
    )
    .unwrap();
    let evaluations = read(&recording.join("evaluations.jsonl"));
    let compact = r#""body":"Some text.""#;
    assert!(evaluations.contains(compact));
    std::fs::write(
        recording.join("evaluations.jsonl"),
        evaluations.replace(compact, &format!(r#""body":{printed}"#)),
    )
    .unwrap();
    (
        recording,
        rewritten(&calls, DESCRIPTION, &format!(r#""body": {raw}"#)),
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn control_characters_gh_prints_in_caret_notation_are_gh_s_and_not_the_ports() {
    // GitHub answers the description with an escape character, and with a
    // backslash followed by the text of one: what a terminal's colour codes
    // leave in a bot's output. `gh api` prints both in caret notation, so
    // Python's engine read `^[` where the Rust engine, reading GitHub
    // itself, reads the characters.
    let dir = tempfile::tempdir().unwrap();
    let (recording, served) = description_read_as(
        dir.path(),
        r#""Some text. ^[[1mbold^[[0m and \\^[""#,
        r#""Some text. \u001b[1mbold\u001b[0m and \\u001b""#,
    );
    let github = github(&served, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(outcome.clean(), "{said}");
    assert!(
        said.contains(
            "| live snapshot | `pr.body` | value, gone with Python's gh-printed control characters | 1 | dashpay/platform · sync: 0 |"
        ),
        "{said}"
    );
    only_reads(&github);
    no_content::assert_no_contents(&said, &recording, &sources());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_string_that_differs_otherwise_fails_and_is_described_by_its_shape_alone() {
    // An escape character where Python read something gh would never have
    // printed for it: the port's difference, said by its shape — lengths,
    // where it starts, what kind of character stands there on each side —
    // and never by what either side holds.
    let dir = tempfile::tempdir().unwrap();
    let (recording, served) =
        description_read_as(dir.path(), r#""Some text. ^Z""#, r#""Some text. \u001b""#);
    let github = github(&served, None).await;
    let outcome = live(&github, &recording, RECORDED, usize::MAX).await;
    let said = outcome.printed();
    assert!(!outcome.clean(), "{said}");
    assert!(
        said.contains("| live snapshot | `pr.body` | value | 1 | dashpay/platform · sync: 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| live snapshot | `pr.body` | dashpay/platform · sync: 0 | lengths 12 and 13, first difference at 11: control against ASCII punct; equal with line endings made one: no, under NFC: no, without trailing whitespace: no |"
        ),
        "{said}"
    );
    no_content::assert_no_contents(&said, &recording, &sources());
}
