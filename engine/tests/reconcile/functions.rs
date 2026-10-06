//! The gate: every call Python's test suite made of the pure functions the
//! reconcile layer ports — the words, the record, the diff record, the
//! admission conflicts, what the status page says about a head — replayed
//! through the port and compared as JSON text, as `conformance.py`
//! compares them, except the cases `conformance/pending.txt` lists.

use crate::support::*;
use pr_hygiene_engine::evidence::records::state_comment_body;
use pr_hygiene_engine::reconcile::telemetry::head_state;
use pr_hygiene_engine::reconcile::{
    admission_conflicts, checklist_block, diff_record, move_text, state_record, status_description,
};
use std::collections::BTreeSet;

/// The case directories this gate runs, relative to `conformance/`.
const GATED: [&str; 8] = [
    "functions/main.checklist_block",
    "functions/main.move_text",
    "functions/main.status_description",
    "functions/github.GitHub.state_comment_body",
    "functions/main.state_record",
    "functions/main.diff_record",
    "functions/main.admission_conflicts",
    "functions/telemetry.head_state",
];

fn optional(text: Option<String>) -> PyValue {
    text.map_or(PyValue::None, PyValue::Str)
}

/// The port's answer to one case, or what it raised.
fn answer(function: &str, inputs: &PyValue) -> Result<PyValue, String> {
    let raised = |error: &dyn std::fmt::Debug| format!("raised {error:?}");
    let input = |name: &str| field(inputs, name);
    Ok(match function {
        "main.checklist_block" => {
            optional(checklist_block(input("result")).map_err(|e| raised(&e))?)
        }
        "main.move_text" => optional(move_text(input("result")).map_err(|e| raised(&e))?),
        "main.status_description" => {
            PyValue::Str(status_description(input("result")).map_err(|e| raised(&e))?)
        }
        "github.GitHub.state_comment_body" => {
            let diff = match input("diff") {
                PyValue::None => None,
                diff => Some(diff),
            };
            PyValue::Str(
                state_comment_body(input("state"), text(input("body")), diff)
                    .map_err(|e| raised(&e))?,
            )
        }
        "main.state_record" => state_record(input("pr"), input("result"), text(input("context")))
            .map_err(|e| raised(&e))?,
        "main.diff_record" => diff_record(input("pr"), input("result"))
            .map_err(|e| raised(&e))?
            .unwrap_or(PyValue::None),
        "main.admission_conflicts" => {
            let conflicts = admission_conflicts(input("policy"), items(input("candidates")))
                .map_err(|e| raised(&e))?;
            // A set has no JSON of its own; a case holds it sorted.
            PyValue::List(conflicts.into_iter().map(PyValue::Str).collect())
        }
        "telemetry.head_state" => head_state(
            input("payload"),
            input("repository"),
            input("number"),
            input("head"),
            input("head_seen_at"),
            input("now"),
        )
        .map_err(|e| raised(&e))?
        .map_or(PyValue::None, |state| PyValue::Str(state.into())),
        other => panic!("{other} is not a function this gate runs"),
    })
}

/// How the port's answer to a case differs from Python's.
fn case(function: &str, path: &std::path::Path) -> Result<(), String> {
    let case = read_json(path);
    let ours = answer(function, field(&case, "inputs"))?;
    let (ours, python) = (dump(&ours), dump(field(&case, "output")));
    if ours == python {
        Ok(())
    } else {
        Err(format!("\n  ours   {ours}\n  python {python}"))
    }
}

/// The case files under a gated directory, relative to `conformance/`.
fn cases_in(dir: &str) -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(conformance().join(dir))
        .unwrap_or_else(|e| panic!("{dir}: {e}"))
        .map(|entry| entry.expect("a directory entry").file_name())
        .filter_map(|name| name.to_str().map(str::to_owned))
        .filter(|name| name.ends_with(".json"))
        .map(|name| format!("{dir}/{name}"))
        .collect();
    found.sort();
    found
}

#[test]
fn every_function_case_matches_python_but_for_the_pending_ones() {
    let pending = pending();
    let mut problems = Vec::new();
    let mut counts = Vec::new();
    let mut seen = BTreeSet::new();
    for dir in GATED {
        let cases = cases_in(dir);
        assert!(!cases.is_empty(), "no cases in {dir}");
        let mut passed = 0;
        for name in &cases {
            seen.insert(name.clone());
            let outcome = case(&dir["functions/".len()..], &conformance().join(name));
            match (outcome, pending.contains(name)) {
                (Ok(()), false) => passed += 1,
                (Ok(()), true) => {
                    problems.push(format!("{name} passes now: remove it from pending.txt"))
                }
                (Err(_), true) => {}
                (Err(difference), false) => problems.push(format!("{name}: {difference}")),
            }
        }
        counts.push(format!("{dir}: {passed} of {} pass", cases.len()));
    }
    // A listed case under a directory this gate runs must be one of its
    // cases; the other gates check the rest of the list.
    for listed in &pending {
        if GATED
            .iter()
            .any(|dir| listed.starts_with(&format!("{dir}/")))
            && !seen.contains(listed)
        {
            problems.push(format!(
                "{listed} is in pending.txt but no such case exists"
            ));
        }
    }
    eprintln!("{}", counts.join("\n"));
    assert!(
        problems.is_empty(),
        "{} problem(s):\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn a_changed_answer_fails_a_function_case() {
    // The gate compares text: one character of a block moved is a
    // difference, as one key out of Python's order is.
    let dir = "functions/main.checklist_block";
    let name = cases_in(dir)
        .into_iter()
        .find(|name| {
            matches!(
                field(&read_json(&conformance().join(name)), "output"),
                PyValue::Str(_)
            )
        })
        .expect("a case that writes a block");
    let mut case = read_json(&conformance().join(&name));
    let PyValue::Dict(entries) = &mut case else {
        panic!("a case is a dict")
    };
    let Some(PyValue::Str(output)) = entries.get_mut("output") else {
        panic!("a block")
    };
    output.push(' ');
    let ours = answer("main.checklist_block", field(&case, "inputs")).expect("an answer");
    assert_ne!(dump(&ours), dump(field(&case, "output")));
}
