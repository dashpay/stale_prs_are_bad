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
/// last wrote: its record's evidence print is the print of what is read,
/// and Python's own run wrote nothing. What GitHub serves is the copy's
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
        said.contains("| live writes | `POST repos/*/*/statuses/*` | would-be write, not sent |"),
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
