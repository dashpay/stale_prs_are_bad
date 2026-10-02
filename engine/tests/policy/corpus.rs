//! The gate: every evaluate case, and every case of the policy functions
//! `main.py` calls, replayed through the port and compared with what
//! Python answered, except the cases `conformance/pending.txt` lists.
//!
//! The case files are read with the crate's own JSON reader, as the engine
//! reads its inputs. A result is compared as the JSON Python's writer makes
//! of it, without sorting: every value, every string byte for byte, every
//! list in order, and every map's keys in Python's order too.

use pr_hygiene_engine::policy::{admit, diff_print, evaluate, receipt_print};
use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyDict, PyValue};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// `conformance/`, where the cases and `pending.txt` live.
fn conformance() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance")
}

/// The case directories this gate runs, relative to `conformance/`.
const GATED: [&str; 4] = [
    "evaluate",
    "functions/policy.admit",
    "functions/policy.receipt_print",
    "functions/policy.diff_print",
];

fn read(path: &Path) -> PyValue {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    py_loads(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn field<'a>(value: &'a PyValue, key: &str) -> &'a PyValue {
    match value {
        PyValue::Dict(entries) => entries
            .get(key)
            .unwrap_or_else(|| panic!("no {key:?} in the case")),
        _ => panic!("the case is not an object"),
    }
}

/// Python's `_dump`: compact, unsorted. `ensure_ascii` only changes how a
/// character is spelled, never whether two strings are equal.
fn dump(value: &PyValue) -> String {
    py_dumps(value, false, Some((",", ":")), None).expect("a result holds no float")
}

/// The blockers without the one carrying the text of a Python exception:
/// the first that is not a waiver's note, as `conformance.py` finds it.
fn without_exception_text(result: &mut PyValue) -> Option<String> {
    let PyValue::Dict(entries) = result else {
        return None;
    };
    let Some(PyValue::List(blockers)) = entries.get_mut("blockers") else {
        return None;
    };
    let at = blockers
        .iter()
        .position(|b| !matches!(b, PyValue::Str(text) if text.starts_with("Proceeded without")))?;
    match blockers.remove(at) {
        PyValue::Str(text) => Some(text),
        other => Some(format!("{other:?}")),
    }
}

/// How the port's answer to an evaluate case differs from Python's.
fn evaluate_case(path: &Path) -> Result<(), String> {
    let case = read(path);
    let ours = evaluate(
        field(&case, "policy"),
        field(&case, "pr"),
        field(&case, "admitted_at"),
        field(&case, "now"),
        field(&case, "telemetry_states"),
    )
    .map_err(|error| format!("raised {error:?}"))?;
    let mut ours = PyValue::Dict(ours);
    let mut python = field(&case, "result").clone();
    if field(&case, "python_exception_text").truthy() {
        // Python's exception text is not compared, but the port must have
        // raised one where Python did: a blocker in the same place.
        let theirs = without_exception_text(&mut python);
        let mine = without_exception_text(&mut ours);
        if theirs.is_some() != mine.is_some() {
            return Err(format!("exception text: python {theirs:?}, ours {mine:?}"));
        }
        // Nor may the port's be one of the engine's own reasons, which would
        // mean it stopped where Python raised.
        if let Some(own) = mine
            .as_deref()
            .and_then(|mine| own_words().into_iter().find(|own| mine.starts_with(own)))
        {
            return Err(format!(
                "exception text {mine:?} is the engine's own {own:?}"
            ));
        }
    }
    compare(&ours, &python)
}

/// The engine's own words, as `conformance.py` tells them from a Python
/// exception's: the reasons `evaluate` stops with (its `_OWN_BLOCKERS`) and
/// the start of every message `policy.py` raises.
fn own_words() -> Vec<String> {
    let source = |name: &str| {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../pr_review")
            .join(name);
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    };
    let conformance = source("conformance.py");
    let start = conformance
        .find("_OWN_BLOCKERS = (")
        .expect("conformance.py names _OWN_BLOCKERS");
    let end = start + conformance[start..].find(")\n").expect("the tuple ends");
    let literal = regex::Regex::new(r"'([^']*)'").expect("a pattern");
    let raised = regex::Regex::new(r"raise ValueError\(f?'([^'{]*)").expect("a pattern");
    let policy = source("policy.py");
    let words: Vec<String> = literal
        .captures_iter(&conformance[start..end])
        .chain(raised.captures_iter(&policy))
        .map(|caps| caps[1].to_owned())
        .filter(|word| !word.is_empty())
        .collect();
    assert!(
        words.len() > 30,
        "only {} of the engine's own words",
        words.len()
    );
    words
}

fn compare(ours: &PyValue, python: &PyValue) -> Result<(), String> {
    let (ours, python) = (dump(ours), dump(python));
    if ours == python {
        Ok(())
    } else {
        Err(format!("\n  ours   {ours}\n  python {python}"))
    }
}

/// How the port's answer to a function case differs from Python's,
/// compared as JSON text as `conformance.py` compares them.
fn function_case(function: &str, path: &Path) -> Result<(), String> {
    let case = read(path);
    let inputs = field(&case, "inputs");
    let output = field(&case, "output");
    let raised = |error| format!("raised {error:?}");
    let ours = match function {
        "policy.admit" => {
            let slots = admit(
                field(inputs, "policy"),
                field(inputs, "prs"),
                field(inputs, "nowISO"),
            )
            .map_err(raised)?;
            // An int key is written as JSON writes any key, as a string.
            let written: PyDict = slots
                .into_iter()
                .map(|(number, since)| (number.to_string(), since))
                .collect();
            PyValue::Dict(written)
        }
        "policy.receipt_print" => {
            let print = receipt_print(field(inputs, "comment")).map_err(raised)?;
            print.map_or(PyValue::None, PyValue::Str)
        }
        "policy.diff_print" => {
            let print = diff_print(field(inputs, "pr")).map_err(raised)?;
            print.map_or(PyValue::None, PyValue::Str)
        }
        other => panic!("{other} is not a function this gate runs"),
    };
    compare(&ours, output)
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

/// `pending.txt`: one case path per line, relative to `conformance/`;
/// blank lines and `#` comments ignored.
fn pending() -> BTreeSet<String> {
    let path = conformance().join("pending.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

#[test]
fn the_corpus_matches_python_but_for_the_pending_cases() {
    let pending = pending();
    let mut problems = Vec::new();
    let mut counts = Vec::new();
    let mut seen = BTreeSet::new();
    for dir in GATED {
        let cases = cases_in(dir);
        assert!(!cases.is_empty(), "no cases in {dir}");
        let mut passed = 0;
        for case in &cases {
            seen.insert(case.clone());
            let path = conformance().join(case);
            let outcome = match dir {
                "evaluate" => evaluate_case(&path),
                _ => function_case(&dir["functions/".len()..], &path),
            };
            match (outcome, pending.contains(case)) {
                (Ok(()), false) => passed += 1,
                (Ok(()), true) => {
                    problems.push(format!("{case} passes now: remove it from pending.txt"))
                }
                (Err(_), true) => {}
                (Err(difference), false) => problems.push(format!("{case}: {difference}")),
            }
        }
        counts.push(format!("{dir}: {passed} of {} pass", cases.len()));
    }
    // A listed case must exist, as a case this gate runs or as a file other
    // gates own.
    for listed in &pending {
        let ours = GATED
            .iter()
            .any(|dir| listed.starts_with(&format!("{dir}/")));
        let exists = if ours {
            seen.contains(listed)
        } else {
            conformance().join(listed).is_file()
        };
        if !exists {
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
