//! The differential tool, run as the scheduled job runs it: over whole
//! recordings, judged by its exit status and by what it prints — counts,
//! field paths and case indices, and never anything a recording holds.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const FILES: [&str; 6] = [
    "recording.json",
    "calls.jsonl",
    "evaluations.jsonl",
    "verdicts.json",
    "outputs.json",
    "printed.txt",
];

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

/// Every string a recording holds as a value — titles, bodies, logins,
/// permission levels, commits, instants — that is long enough to be told
/// apart from the words of the report itself.
fn contents(recording: &Path) -> Vec<String> {
    fn strings(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::String(s) => {
                out.push(s.clone());
                // A recorded answer is JSON inside a string: its strings too.
                if let Ok(inner) = serde_json::from_str::<Value>(s) {
                    if inner.is_object() || inner.is_array() {
                        strings(&inner, out);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
            Value::Object(entries) => entries.values().for_each(|v| strings(v, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for file in ["calls.jsonl", "evaluations.jsonl"] {
        let text = std::fs::read_to_string(recording.join(file)).expect("a recording file");
        for line in text.lines() {
            strings(&serde_json::from_str(line).expect("a JSON line"), &mut out);
        }
    }
    let verdicts = std::fs::read_to_string(recording.join("verdicts.json")).expect("verdicts");
    strings(&serde_json::from_str(&verdicts).expect("JSON"), &mut out);
    // What the report is allowed to say: the repository and the command,
    // which name the recording, and the words of its own headings.
    let allowed = [
        "dashpay/platform",
        "dashpay",
        "platform",
        "report",
        "matched",
        "verdict",
    ];
    out.retain(|s| s.chars().count() >= 5 && !allowed.contains(&s.as_str()));
    out.sort();
    out.dedup();
    out
}

#[track_caller]
fn assert_no_contents(said: &str, recording: &Path) {
    let seen = contents(recording);
    assert!(seen.len() > 20, "the recording holds text to look for");
    let leaked: Vec<&String> = seen.iter().filter(|s| said.contains(s.as_str())).collect();
    assert!(
        leaked.is_empty(),
        "printed what the recording holds: {leaked:?}\n{said}"
    );
}

#[test]
fn every_synthetic_recording_matches_and_is_counted() {
    let (output, said) = run(&[&synthetic()]);
    assert!(output.status.success(), "{said}");
    assert!(
        said.contains("7 recording(s): the Rust engine matched every one."),
        "{said}"
    );
    assert!(
        said.contains("| dashpay/platform · report | 2/2 | 2/2 | 2/2 | 0 | 16 | 0 |"),
        "{said}"
    );
    assert!(
        said.contains("| **all** (7 recordings) | 10/10 | 10/10 | 8/8 | 0 | 280 | 0 |"),
        "{said}"
    );
    assert!(said.contains("No differences."), "{said}");
    for name in ["report", "rich-evidence", "long-history"] {
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
    // them; only the snapshots rebuilt from the reads differ.
    assert!(
        said.contains("| dashpay/platform · report | 0/2 | 2/2 | 2/2 | 1 | 15 | 2 |"),
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
        said.contains("| dashpay/platform · report | 2/2 | 2/2 | 1/2 | 0 | 16 | 1 |"),
        "{said}"
    );
    assert!(
        said.contains("| verdict | `verdict.state` | value | 1 | dashpay/platform · report: 0 |"),
        "{said}"
    );
    assert_no_contents(&said, &broken);
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
    assert!(said.contains("| 1/1 | 1/1 | 1/1 | 0 | 41 | 0 |"), "{said}");
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
    // Counted here independently: one per read, a paginated read that
    // succeeded one per page it printed.
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
            wanted += match (paged, call["exit"].as_i64(), call["stdout"].as_str()) {
                (true, Some(0), Some(stdout)) => serde_json::from_str::<Value>(stdout)
                    .ok()
                    .and_then(|pages| pages.as_array().map(Vec::len))
                    .unwrap_or(1),
                _ => 1,
            };
        }
    }
    let (output, said) = run(&[Path::new("--requests"), &synthetic()]);
    assert!(output.status.success(), "{said}");
    assert_eq!(said.trim(), wanted.to_string());
    // More than one call per page somewhere, or this proves nothing.
    assert!(wanted > 0);
}

#[test]
fn nothing_to_compare_is_a_usage_error() {
    let dir = scratch("empty");
    let (output, said) = run(&[&dir]);
    assert_eq!(output.status.code(), Some(2), "{said}");
    assert!(said.contains("no recording found"), "{said}");
    let _ = std::fs::remove_dir_all(dir);
}
