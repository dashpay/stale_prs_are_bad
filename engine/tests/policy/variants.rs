//! What the corpus cannot tell apart, against Python's answers
//! (`conformance/pycompat/policy_variants.json`, written by
//! `generate_policy.py`): which of two equal instants spelled differently a
//! verdict keeps, what `strip` takes from around an attestation or a skip,
//! the order `diff_print` sorts files in, and `fingerprint` byte for byte —
//! of every corpus snapshot, and of snapshots built to move it or not.

use pr_hygiene_engine::policy::{diff_print, evaluate, fingerprint};
use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyValue};
use std::path::Path;

fn conformance(path: &str) -> PyValue {
    let full = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../conformance")
        .join(path);
    let text = std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{}: {e}", full.display()));
    py_loads(&text).unwrap_or_else(|e| panic!("{}: {e}", full.display()))
}

fn field<'a>(value: &'a PyValue, key: &str) -> &'a PyValue {
    match value {
        PyValue::Dict(entries) => entries.get(key).unwrap_or_else(|| panic!("no {key:?}")),
        _ => panic!("not an object"),
    }
}

fn text(value: &PyValue) -> &str {
    match value {
        PyValue::Str(s) => s,
        other => panic!("not a string: {other:?}"),
    }
}

fn dump(value: &PyValue) -> String {
    py_dumps(value, false, Some((",", ":")), None).expect("no float")
}

#[test]
fn ties_strips_sorts_and_prints_are_pythons() {
    let golden = conformance("pycompat/policy_variants.json");
    let PyValue::List(entries) = field(&golden, "cases") else {
        panic!("no cases")
    };
    let mut failures = Vec::new();
    let mut fingerprints = 0;
    let mut evaluated = 0;
    let mut diff_prints = 0;
    for entry in entries.iter() {
        let kind = text(field(entry, "kind"));
        match kind {
            "evaluate" => {
                evaluated += 1;
                let name = text(field(entry, "name"));
                let ours = evaluate(
                    field(entry, "policy"),
                    field(entry, "pr"),
                    field(entry, "admitted_at"),
                    field(entry, "now"),
                    &PyValue::None,
                );
                match ours {
                    Ok(ours) => {
                        let (ours, python) =
                            (dump(&PyValue::Dict(ours)), dump(field(entry, "result")));
                        if ours != python {
                            failures.push(format!("{name}:\n  ours   {ours}\n  python {python}"));
                        }
                    }
                    Err(error) => failures.push(format!("{name}: raised {error:?}")),
                }
            }
            "diff_print" | "fingerprint" => {
                let base;
                let pr = match entry {
                    PyValue::Dict(fields) if fields.contains_key("base") => {
                        base = conformance(text(field(entry, "base")));
                        field(&base, "pr")
                    }
                    _ => field(entry, "pr"),
                };
                let ours = if kind == "diff_print" {
                    diff_prints += 1;
                    diff_print(pr).map(|print| print.map_or(PyValue::None, PyValue::Str))
                } else {
                    fingerprints += 1;
                    fingerprint(pr).map(PyValue::Str)
                };
                match ours {
                    Ok(ours) if dump(&ours) == dump(field(entry, "output")) => {}
                    other => failures.push(format!(
                        "{kind} of {}: ours {other:?}, python {:?}",
                        dump(pr),
                        field(entry, "output")
                    )),
                }
            }
            other => panic!("no such kind {other}"),
        }
    }
    assert!(fingerprints > 270, "only {fingerprints} fingerprints");
    assert!(evaluated >= 25, "only {evaluated} evaluations");
    assert!(diff_prints >= 4, "only {diff_prints} diff prints");
    assert!(
        failures.is_empty(),
        "{} of {} differ:\n{}",
        failures.len(),
        entries.len(),
        failures.join("\n")
    );
}
