//! The fixtures of `pr_review/tests/test_policy.py`, as `PyValue`s.

use pr_hygiene_engine::pycompat::{py_loads, PyDateTime, PyDict, PyErr, PyTimeDelta, PyValue};
use serde_json::{json, Value};

pub const HEAD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub const NOW: &str = "2026-09-11T12:00:00Z";

/// A JSON value as the engine reads one.
pub fn value(json: Value) -> PyValue {
    py_loads(&json.to_string()).expect("serde_json writes JSON")
}

pub fn s(text: &str) -> PyValue {
    PyValue::Str(text.to_owned())
}

pub fn dict_mut(value: &mut PyValue) -> &mut PyDict {
    match value {
        PyValue::Dict(entries) => entries,
        other => panic!("not a dict: {other:?}"),
    }
}

/// The value at `path`, each step a key or a list index.
pub fn at<'a>(value: &'a mut PyValue, path: &[&str]) -> &'a mut PyValue {
    let mut here = value;
    for step in path {
        here = match here {
            PyValue::Dict(entries) => entries
                .get_mut(*step)
                .unwrap_or_else(|| panic!("no {step}")),
            PyValue::List(items) => &mut items[step.parse::<usize>().expect("an index")],
            other => panic!("cannot step into {other:?}"),
        };
    }
    here
}

/// The list at `path`; anything else is a mistake in the test.
pub fn list_mut<'a>(target: &'a mut PyValue, path: &[&str]) -> &'a mut Vec<PyValue> {
    match at(target, path) {
        PyValue::List(items) => items,
        other => panic!("not a list at {path:?}: {other:?}"),
    }
}

/// `target[path] = new`, the last step a key.
pub fn set(target: &mut PyValue, path: &[&str], new: PyValue) {
    let (key, parents) = path.split_last().expect("a path");
    dict_mut(at(target, parents)).insert((*key).to_owned(), new);
}

/// `del target[path]`, the last step a key.
pub fn remove(target: &mut PyValue, path: &[&str]) {
    let (key, parents) = path.split_last().expect("a path");
    dict_mut(at(target, parents)).shift_remove(*key);
}

/// Whether a call raised Python's `ValueError`.
pub fn is_value_error<T: std::fmt::Debug>(outcome: Result<T, PyErr>) -> bool {
    matches!(outcome, Err(PyErr::Value(_)))
}

/// `fixture()`: a policy with one area, and a pull request into it by the
/// area's owner that both bots reported on and the owner attested.
pub fn fixture() -> (PyValue, PyValue) {
    let policy = json!({
        "version": 1, "repository": "dashpay/platform", "max_active_prs": 5,
        "target_branches": ["v4.2-dev"],
        "fallback": {"owners": ["fallback"], "reviewers": []},
        "areas": [{"id": "drive", "paths": ["packages/drive/"], "owners": ["owner"],
                   "reviewers": ["reviewer"]}],
    });
    let pr = json!({
        "number": 1, "author": "owner", "head": HEAD, "base": "v4.2-dev", "base_sha": "b".repeat(40),
        "created_at": "2026-09-10T00:00:00Z", "draft": false, "state": "open", "url": "url",
        "title": "title",
        "files": [{"filename": "packages/drive/a.rs"}],
        "reviews": [
            {"id": 1, "user": "thepastaclaw", "state": "COMMENTED", "commit_id": HEAD,
             "submitted_at": "2026-09-11T10:00:00Z",
             "body": format!("<!-- thepastaclaw-review-phase v1 phase=final sha={HEAD} -->")},
            {"id": 2, "user": "coderabbitai[bot]", "state": "APPROVED", "commit_id": HEAD,
             "submitted_at": "2026-09-11T10:00:00Z", "body": ""},
        ],
        "comments": [{"id": 3, "user": "owner", "body": format!("/self-reviewed {HEAD}"),
                      "created_at": "2026-09-11T11:00:00Z", "updated_at": "2026-09-11T11:00:00Z"}],
        "threads": [], "requested_reviewers": [],
        "permissions": {"owner": "write", "reviewer": "write", "fallback": "write"},
        "controller_state": null, "labels": [], "complete": true, "build": "green",
    });
    (value(policy), value(pr))
}

/// `ago(hours)`: that many hours before `NOW`, as the engine writes an
/// instant.
pub fn ago(hours: f64) -> String {
    let now = PyDateTime::fromisoformat("2026-09-11T12:00:00+00:00").expect("an instant");
    let micros = (-hours * 3_600_000_000.0).round() as i64;
    now.py_add(PyTimeDelta::from_micros(micros))
        .expect("in range")
        .isoformat()
        .replace("+00:00", "Z")
}

/// What `rabbit()` varies in CodeRabbit's comment.
pub struct Rabbit<'a> {
    pub finding: &'a str,
    pub extra: &'a str,
    pub checks: &'a str,
    pub why: &'a str,
    pub rows: Option<Vec<(&'a str, &'a str)>>,
    pub found: &'a str,
    pub run: &'a str,
}

impl Default for Rabbit<'_> {
    fn default() -> Self {
        Rabbit {
            finding: "",
            extra: "",
            checks: "\u{2705} Passed",
            why: "It reads well.",
            rows: None,
            found: "No actionable comments were generated.",
            run: "d5a7d983",
        }
    }
}

/// `rabbit()`: CodeRabbit's comment in the shape it really writes. The
/// findings and the commit they cover are in one block; the checks are a
/// table elsewhere, with a heading that counts them and a column of prose
/// it rewrites without changing a verdict.
pub fn rabbit(r: Rabbit<'_>) -> String {
    let covered =
        format!(r#"{{"sourceCommitId":"{HEAD}","coveredCommitId":"{HEAD}","kind":"reviewed"}}"#);
    let extra = if r.extra.is_empty() {
        String::new()
    } else {
        format!(
            "<!-- review_stack_entry_start -->\n{}<!-- review_stack_entry_end -->\n",
            r.extra
        )
    };
    let rows = r.rows.unwrap_or_else(|| vec![("Title check", r.checks)]);
    let rows: String = rows
        .iter()
        .map(|(name, state)| format!("| {name} | {state} | {} |\n", r.why))
        .collect();
    format!(
        "<!-- This is an auto-generated comment: summarize by coderabbit.ai -->\n\
         {extra}\
         <!-- recent_review_start -->\n{found}\n\
         <details><summary>\u{2699}\u{fe0f} Run configuration</summary>\nRun ID: {run}\n</details>\n\
         <!-- recent_review_end -->\n\
         <!-- walkthrough_start -->\n\
         | Layer / File(s) | Summary |\n| :--- | :--- |\n\
         | `a.rs` | Adds a field and its tests. |\n\
         <!-- walkthrough_end -->\n\
         <!-- final_review_risk_start -->\n\
         **Merge Risk:** Minimal\n\
         <!-- final_review_risk_coverage:{covered} -->\n\
         {finding}\n<!-- final_review_risk_end -->\n\
         <!-- pre_merge_checks_walkthrough_start -->\n\
         <summary>\u{1f6a5} Pre-merge checks | {checks}</summary>\n\n\
         | Check name | Status | Explanation |\n| :---: | :--- | :--- |\n\
         {rows}\
         <!-- pre_merge_checks_walkthrough_end -->\n\
         <!-- tips_start -->\n<details><summary>\u{1faa7} Tips</summary>\nChat with CodeRabbit.\n</details>\n\
         <!-- tips_end -->\n",
        found = r.found,
        run = r.run,
        finding = r.finding,
        checks = r.checks,
    )
}

/// A comment as `receipt_print` reads one.
pub fn comment(id: i64, body: &str) -> PyValue {
    value(json!({"id": id, "body": body}))
}
