//! The gate: every evaluate case, and every case of the policy functions
//! `main.py` calls, replayed through the port and compared with what
//! Python answered, except the cases `conformance/pending.txt` lists.
//!
//! The case files are read with the crate's own JSON reader, as the engine
//! reads its inputs. A result is compared as the JSON Python's writer makes
//! of it, without sorting: every value, every string byte for byte, every
//! list in order, and every map's keys in Python's order too.

use pr_hygiene_engine::conformance::{set_aside_exception_text, OwnWords};
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

/// How the port's answer to an evaluate case differs from Python's.
///
/// Where Python marked the case `python_exception_text`, its first reason
/// is not compared, but the port must have raised where Python did — a
/// reason in the same place — and that reason must not be one of the
/// engine's own words, which would mean it stopped where Python raised.
/// The differential job applies the same exclusion to live recordings.
fn evaluate_case(path: &Path, own: &OwnWords) -> Result<(), String> {
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
        if let Err(problem) = set_aside_exception_text(&mut ours, &mut python, own) {
            return Err(format!(
                "exception text: {problem:?}\n  ours   {}\n  python {}",
                dump(&ours),
                dump(&python)
            ));
        }
    }
    compare(&ours, &python)
}

/// The engine's own words, as `conformance.py` tells them from a Python
/// exception's: the reasons `evaluate` stops with (its `_OWN_BLOCKERS`) and
/// every message `policy.py` raises.
fn own_words() -> OwnWords {
    let source = |name: &str| {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../pr_review")
            .join(name);
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    };
    let own = OwnWords::from_sources(&source("conformance.py"), &source("policy.py"))
        .expect("the engine's own words are read");
    let (reasons, raised) = own.counts();
    assert!(
        reasons + raised > 30,
        "only {} of the engine's own words",
        reasons + raised
    );
    own
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
fn the_exception_text_rule_read_from_the_source_is_pythons() {
    // Live recordings carry no `python_exception_text` mark: the
    // differential job decides it from the engine's source, as
    // `conformance.python_exception_text` does. On every case Python
    // marked, and every case it did not, the two must agree.
    let own = own_words();
    let mut marked = 0;
    let mut disagree = Vec::new();
    for case in cases_in("evaluate") {
        let case_file = read(&conformance().join(&case));
        let python = field(&case_file, "python_exception_text").truthy();
        marked += usize::from(python);
        if own.python_exception_text(field(&case_file, "result")) != python {
            disagree.push(format!("{case}: python says {python}"));
        }
    }
    assert!(marked > 0, "some case carries exception text");
    assert!(disagree.is_empty(), "{}", disagree.join("\n"));
}

#[test]
fn the_corpus_matches_python_but_for_the_pending_cases() {
    let own = own_words();
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
                "evaluate" => evaluate_case(&path, &own),
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
