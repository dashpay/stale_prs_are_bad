//! The differential tool, run as the scheduled job runs it: over whole
//! recordings, judged by its exit status and by what it prints — counts,
//! field paths and case indices, and never anything a recording holds.

#[path = "../support/no_content.rs"]
mod no_content;

use no_content::FILES;
use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyValue};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The sources whose string literals are the report's own words: the tool
/// and the conformance module that compares and prints for it.
fn sources() -> Vec<PathBuf> {
    [
        "src/bin/differential.rs",
        "src/conformance/compare.rs",
        "src/conformance/diff.rs",
        "src/conformance/report.rs",
    ]
    .iter()
    .map(|file| Path::new(env!("CARGO_MANIFEST_DIR")).join(file))
    .collect()
}

/// Nothing a recording holds appears in what the tool said.
#[track_caller]
fn assert_no_contents(said: &str, recording: &Path) {
    no_content::assert_no_contents(said, recording, &sources());
}

fn synthetic() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/synthetic")
}

/// A scratch directory of this test's own, emptied first.
fn scratch(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("engine-differential-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// A copy of a synthetic recording, under `into`.
fn copy(recording: &str, into: &Path) -> PathBuf {
    let to = into.join(recording);
    std::fs::create_dir_all(&to).expect("a recording directory");
    for file in FILES {
        std::fs::copy(synthetic().join(recording).join(file), to.join(file)).expect("a copy");
    }
    to
}

fn edit(path: &Path, change: impl FnOnce(String) -> String) {
    let text = std::fs::read_to_string(path).expect("a recording file");
    let changed = change(text.clone());
    assert_ne!(text, changed, "the edit changed {}", path.display());
    std::fs::write(path, changed).expect("written");
}

fn run(args: &[&Path]) -> (Output, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_differential"))
        .args(args)
        .output()
        .expect("the tool runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output, said)
}

#[test]
fn every_synthetic_recording_matches_and_is_counted() {
    let (output, said) = run(&[&synthetic()]);
    assert!(output.status.success(), "{said}");
    assert!(
        said.contains("13 recording(s): the Rust engine matched every one."),
        "{said}"
    );
    assert!(
        said.contains(
            "| dashpay/platform · report (report) | 2/2 | 2/2 | 2/2 | 2/2 | 2/2 | 0/0 | 2/2 | 0/0 | 1/1 | 1/1 | 0 | 16 | 0 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| **all** (13 recordings) | 18/18 | 18/18 | 15/15 | 25/25 | 15/15 | 67/67 | 15/15 | 4/4 | 13/13 | 13/13 | 0 | 438 | 0 |"
        ),
        "{said}"
    );
    assert!(said.contains("No differences."), "{said}");
    for name in [
        "report",
        "rich-evidence",
        "long-history",
        "sweep",
        "nudge",
        "report-json",
    ] {
        assert_no_contents(&said, &synthetic().join(name));
    }
}

#[test]
fn a_changed_field_and_a_dropped_read_are_named_by_path_and_counted() {
    let dir = scratch("broken");
    let broken = copy("report", &dir);
    // One review of pull request 1, withdrawn: one field of what GitHub
    // answered, changed.
    edit(&broken.join("calls.jsonl"), |text| {
        text.replacen(
            r#"\"state\": \"APPROVED\""#,
            r#"\"state\": \"DISMISSED\""#,
            1,
        )
    });
    // And one read of pull request 2 that the recording no longer holds.
    edit(&broken.join("calls.jsonl"), |text| {
        text.split_inclusive('\n')
            .filter(|line| !line.contains("pulls/2/reviews?per_page=100"))
            .collect()
    });
    let (output, said) = run(&[&broken]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(said.contains("the Rust engine differed"), "{said}");
    // Python's own evaluations are unchanged, so the port still agrees with
    // them; the snapshots rebuilt from the reads differ, and the whole run
    // stops at the read the recording lacks: its outcome says why, and it
    // made no verdict, no clock read and not every call.
    assert!(
        said.contains(
            "| dashpay/platform · report | 0/2 | 2/2 | 2/2 | 1/2 | 0/1 | 0/0 | 0/0 | 0/0 | 0/1 | 0/1 | 1 | 15 | 6 |"
        ),
        "{said}"
    );
    assert!(
        said.contains("| run | — | read not in the recording | 1 | dashpay/platform · report: 0 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| snapshot | `pr.reviews[].state` | value | 1 | dashpay/platform · report: 0 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| snapshot | — | read not in the recording | 1 | dashpay/platform · report: 1 |"
        ),
        "{said}"
    );
    assert_no_contents(&said, &broken);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_verdict_python_reached_differently_is_named_by_path() {
    // As if the port answered otherwise: Python's recorded verdict row and
    // evaluation result say another state than the port reaches.
    let dir = scratch("verdict");
    let broken = copy("report", &dir);
    let state = |text: &str| {
        let row: Value = serde_json::from_str(text).expect("verdicts");
        row[0]["state"].as_str().expect("a state").to_owned()
    };
    let was = state(&std::fs::read_to_string(broken.join("verdicts.json")).expect("verdicts"));
    edit(&broken.join("verdicts.json"), |text| {
        text.replacen(&format!("\"state\": \"{was}\""), "\"state\": \"draft\"", 1)
    });
    let (output, said) = run(&[&broken]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "| dashpay/platform · report | 2/2 | 2/2 | 1/2 | 2/2 | 1/2 | 0/0 | 2/2 | 0/0 | 1/1 | 1/1 | 0 | 16 | 2 |"
        ),
        "{said}"
    );
    assert!(
        said.contains("| verdict | `verdict.state` | value | 1 | dashpay/platform · report: 0 |"),
        "{said}"
    );
    // The whole run reaches the port's own state, which is not the one this
    // copy records either.
    assert!(
        said.contains(
            "| run verdict | `verdict.state` | value | 1 | dashpay/platform · report: 0 |"
        ),
        "{said}"
    );
    assert_no_contents(&said, &broken);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_shared_head_row_is_held_to_its_first_reason() {
    // `evaluate_snapshots` turns a row whose head another pull request
    // shares into a configuration error. Its first reason is still the
    // engine's own, not a Python exception's: a row whose first reason
    // changed must not match.
    let dir = scratch("shared");
    let shared = copy("report", &dir);
    // Edited with the engine's own reader and writer, which keep each key
    // where it was, as Python's `dict.update` does.
    edit(&shared.join("verdicts.json"), |text| {
        let PyValue::List(mut rows) = py_loads(&text).expect("verdicts") else {
            panic!("verdicts are a list")
        };
        let Some(PyValue::Dict(row)) = rows.first_mut() else {
            panic!("a row")
        };
        let text = |s: &str| PyValue::Str(s.to_owned());
        row.insert("state".into(), text("configuration-error"));
        row.insert("status".into(), text("error"));
        row.insert("reviewers".into(), PyValue::List(Default::default()));
        row.insert("objectors".into(), PyValue::List(Default::default()));
        row.insert("ready_since".into(), PyValue::None);
        row.insert(
            "blockers".into(),
            PyValue::List(
                vec![
                    text("A reason the port never gave"),
                    text("Another open PR shares this head; commit-scoped status is ambiguous"),
                ]
                .into(),
            ),
        );
        py_dumps(&PyValue::List(rows), false, None, None).expect("no floats")
    });
    let (output, said) = run(&[&shared]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "| verdict | `verdict.blockers[]` | value | 1 | dashpay/platform · report: 0 |"
        ),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_run_that_raised_inside_evaluate_snapshots_still_rebuilds_over_the_history() {
    // Python raised after evaluating some snapshots and before returning
    // their rows: no verdicts, and every evaluation was one of the first,
    // taken over the batched history read.
    let dir = scratch("raised");
    let raised = copy("report", &dir);
    edit(&raised.join("verdicts.json"), |_| "[]\n".to_owned());
    let (output, said) = run(&[&raised]);
    // The snapshots and evaluations match over the history. The whole run,
    // which the copy still records as returning, makes two rows where the
    // copy holds none: the one difference.
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains("| dashpay/platform · report | 2/2 | 2/2 | 0/0 | 2/2 | 0/1 |"),
        "{said}"
    );
    assert!(
        said.contains("| run verdict | `verdicts` | length | 1 | dashpay/platform · report: 0 |"),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_format_1_recording_is_compared_like_a_format_2_one() {
    let dir = scratch("format1");
    let old = copy("sync-pr-2", &dir);
    edit(&old.join("recording.json"), |text| {
        let mut meta: Value = serde_json::from_str(&text).expect("recording.json");
        let entries = meta.as_object_mut().expect("an object");
        entries.remove("clock_reads");
        entries.insert("format".into(), Value::from(1));
        serde_json::to_string(&meta).expect("JSON")
    });
    let (output, said) = run(&[&old]);
    assert!(output.status.success(), "{said}");
    // Without a clock log, the clock is the one layer not compared.
    assert!(
        said.contains("| 1/1 | 1/1 | 1/1 | 2/2 | 1/1 | 6/6 | 1/1 | 0/0 | 0/0 | 1/1 | 0 | 41 | 0 |"),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_recording_of_an_unknown_format_is_unreadable_and_fails_the_run() {
    let dir = scratch("format3");
    let odd = copy("report", &dir);
    edit(&odd.join("recording.json"), |text| {
        text.replacen("\"format\": 2", "\"format\": 3", 1)
    });
    let good = copy("sync-pr-2", &dir);
    let (output, said) = run(&[&odd, &good]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "- unreadable (report): recording.json: a recording format this reader does not know"
        ),
        "{said}"
    );
    assert!(
        said.contains("| dashpay/platform · sync --pr 2 | 1/1 |"),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn the_summary_file_gets_the_counts_table_alone() {
    let dir = scratch("summary");
    let summary = dir.join("summary.md");
    std::fs::write(&summary, "before\n").expect("written");
    let report = synthetic().join("report");
    let (output, said) = run(&[Path::new("--summary"), &summary, &report]);
    assert!(output.status.success(), "{said}");
    let written = std::fs::read_to_string(&summary).expect("the summary");
    assert!(
        written.starts_with("before\n## Engine differential\n"),
        "{written}"
    );
    assert!(
        written.contains("| dashpay/platform · report | 2/2 |"),
        "{written}"
    );
    assert!(!written.contains("Differences by category"), "{written}");
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn requests_count_every_page_a_read_fetched() {
    // Counted here independently: one per read, a paginated read one per
    // page it printed.
    let mut wanted = 0;
    for entry in std::fs::read_dir(synthetic()).expect("the synthetic recordings") {
        let calls = entry.expect("an entry").path().join("calls.jsonl");
        let Ok(text) = std::fs::read_to_string(&calls) else {
            continue;
        };
        for line in text.lines() {
            let call: Value = serde_json::from_str(line).expect("a call");
            if call["kind"] != "read" {
                continue;
            }
            let paged = call["args"]
                .as_array()
                .is_some_and(|args| args.iter().any(|a| a == "--paginate"));
            wanted += match (paged, call["stdout"].as_str()) {
                (true, Some(stdout)) => serde_json::from_str::<Value>(stdout)
                    .ok()
                    .and_then(|pages| pages.as_array().map(Vec::len))
                    .filter(|&pages| pages > 0)
                    .unwrap_or(1),
                _ => 1,
            };
        }
    }
    let (output, said) = run(&[Path::new("--requests"), &synthetic()]);
    assert!(output.status.success(), "{said}");
    assert_eq!(said.trim(), wanted.to_string());
    // The same requests, as REST requests and GraphQL queries: every
    // GraphQL query is one request, never paginated.
    let mut queries = 0;
    for entry in std::fs::read_dir(synthetic()).expect("the synthetic recordings") {
        let Ok(text) = std::fs::read_to_string(entry.expect("an entry").path().join("calls.jsonl"))
        else {
            continue;
        };
        queries += text
            .lines()
            .filter(|line| line.contains(r#""kind":"read""#) && line.contains(r#""graphql""#))
            .count();
    }
    let (output, said) = run(&[Path::new("--request-kinds"), &synthetic()]);
    assert!(output.status.success(), "{said}");
    assert_eq!(said.trim(), format!("{} {queries}", wanted - queries));
    assert!(queries > 0 && wanted > queries, "both kinds are counted");
    // More than one call per page somewhere, or this proves nothing.
    assert!(wanted > 0);
}

/// The lines of a recording's `calls.jsonl`, and which of them are writes.
fn writes_of(text: &str) -> (Vec<String>, Vec<usize>) {
    let lines: Vec<String> = text.lines().map(str::to_owned).collect();
    let writes = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(r#""kind":"write""#))
        .map(|(at, _)| at)
        .collect();
    (lines, writes)
}

fn joined(lines: &[String]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn a_write_with_another_body_is_named_by_its_place_and_field() {
    let dir = scratch("write-body");
    let broken = copy("sync-pr-2", &dir);
    // The status the run posts first, recorded as saying something else.
    edit(&broken.join("calls.jsonl"), |text| {
        text.replacen(
            "Evaluating current review policy",
            "Evaluating the policy afresh",
            1,
        )
    });
    let (output, said) = run(&[&broken]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "| dashpay/platform · sync --pr 2 | 1/1 | 1/1 | 1/1 | 2/2 | 1/1 | 5/6 | 1/1 | 0/0 | 1/1 | 1/1 | 0 | 41 | 1 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| write | `write.description` | value | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    assert_no_contents(&said, &broken);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn writes_in_another_order_are_named_where_the_order_breaks() {
    let dir = scratch("write-order");
    let broken = copy("sync-pr-2", &dir);
    // The label and the reviewer request, recorded the other way round.
    edit(&broken.join("calls.jsonl"), |text| {
        let (mut lines, writes) = writes_of(&text);
        let (label, request) = (writes[3], writes[4]);
        assert!(lines[label].contains("/labels") && lines[request].contains("requested_reviewers"));
        lines.swap(label, request);
        joined(&lines)
    });
    let (output, said) = run(&[&broken]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    // The port posts the label where the recording holds the request: a
    // write to another route, which stops the run there; the two recorded
    // writes after it are never made, and the calls stop short.
    assert!(
        said.contains(
            "| write | — | write made to another route | 1 | dashpay/platform · sync --pr 2: 3 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| write | — | recorded write never made | 2 | dashpay/platform · sync --pr 2: 4, 5 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| run | — | call refused by the replay | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    assert_no_contents(&said, &broken);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn a_clock_read_at_another_site_is_named() {
    let dir = scratch("clock");
    let broken = copy("sync-pr-2", &dir);
    edit(&broken.join("recording.json"), |text| {
        text.replacen("\"site\": \"collect\"", "\"site\": \"finish\"", 1)
    });
    let (output, said) = run(&[&broken]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "| clock | `clock_reads[][]` | value | 1 | dashpay/platform · sync --pr 2: 0 |"
        ),
        "{said}"
    );
    assert_no_contents(&said, &broken);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn what_a_write_body_and_a_printed_report_say_is_never_printed() {
    // Distinctive text where only a recording holds it: in the body of a
    // recorded write, and in the report the run printed. Both differ from
    // what the port makes, so both layers report; neither text may appear.
    const BODY: &str = "Quokka-Zanzibar-7731 left a confidential note";
    const TITLE: &str = "Marmalade-Obsidian-4402 private roadmap";
    let dir = scratch("no-content");
    let sync = copy("sync-pr-2", &dir);
    edit(&sync.join("calls.jsonl"), |text| {
        let (mut lines, writes) = writes_of(&text);
        // The checklist written into the description.
        let at = writes[1];
        assert!(lines[at].contains("pr-hygiene:start"));
        lines[at] = lines[at].replacen("Some text.", BODY, 1);
        joined(&lines)
    });
    let report = copy("report-json", &dir);
    edit(&report.join("printed.txt"), |text| {
        text.replacen("\"title\": \"PR 2\"", &format!("\"title\": \"{TITLE}\""), 1)
    });
    let (output, said) = run(&[&sync, &report]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains("| write | `write.body` | value | 1 | dashpay/platform · sync --pr 2: 1 |"),
        "{said}"
    );
    assert!(
        said.contains(
            "| report | `report.pull_requests[].title` | value | 1 | dashpay/platform · report: 0 |"
        ),
        "{said}"
    );
    for text in [BODY, TITLE, "Quokka", "Marmalade"] {
        assert!(!said.contains(text), "printed {text:?}:\n{said}");
    }
    assert_no_contents(&said, &sync);
    assert_no_contents(&said, &report);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn the_check_catches_one_word_of_a_value_not_only_the_whole_of_it() {
    // A slice of a description is not any whole value the recording holds,
    // so only its words can give it away. The check must refuse it.
    let recording = synthetic().join("rich-evidence");
    let (_, clean) = run(&[&recording]);
    assert_no_contents(&clean, &recording);
    let leaked = format!("{clean}\n| snapshot | — | Speeds | 1 | x |\n");
    let caught = std::panic::catch_unwind(|| assert_no_contents(&leaked, &recording));
    assert!(
        caught.is_err(),
        "one word of a recorded body went unnoticed"
    );
}

#[test]
fn the_check_catches_a_whole_value_made_of_the_reports_own_words() {
    // A route and a body built only of words the report also uses are
    // still what the recording holds, printed whole: the check refuses
    // them. A recorded login that is only a part of a field's name — the
    // login `reviewer` inside `verdict.reviewers` — is the report's own.
    let recording = synthetic().join("sync-pr-2");
    let (_, clean) = run(&[&recording]);
    let field = format!("{clean}\n| verdict | `verdict.reviewers` | length | 1 | x |\n");
    assert_no_contents(&field, &recording);
    for leak in ["`repos/dashpay/platform/pulls/2`", "Some text."] {
        let leaked = format!("{clean}\n| write | {leak} | value | 1 | x |\n");
        let caught = std::panic::catch_unwind(|| assert_no_contents(&leaked, &recording));
        assert!(caught.is_err(), "{leak} went unnoticed");
    }
}

#[test]
fn a_run_whose_output_cannot_be_read_keeps_its_snapshot_results() {
    let dir = scratch("no-printed");
    let partial = copy("report", &dir);
    std::fs::remove_file(partial.join("printed.txt")).expect("removed");
    let (output, said) = run(&[&partial]);
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        said.contains(
            "| dashpay/platform · report | 2/2 | 2/2 | 2/2 | 0/1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0 | 16 | 1 |"
        ),
        "{said}"
    );
    assert!(
        said.contains(
            "| run | — | outputs.json or printed.txt unreadable | 1 | dashpay/platform · report: 0 |"
        ),
        "{said}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn nothing_to_compare_is_a_usage_error() {
    let dir = scratch("empty");
    let (output, said) = run(&[&dir]);
    assert_eq!(output.status.code(), Some(2), "{said}");
    assert!(said.contains("no recording found"), "{said}");
    let _ = std::fs::remove_dir_all(dir);
}
