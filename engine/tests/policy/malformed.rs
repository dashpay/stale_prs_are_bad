//! `evaluate` on input of the wrong shape, against what Python's did:
//! corpus cases with one field deleted or replaced by a value of another
//! type (`conformance/pycompat/policy_malformed.json`, written by
//! `generate_policy.py`). Which exception escapes `evaluate`, and which it
//! turns into a configuration error, is the line between a verdict and a
//! failed run; each replay must fall on the same side of it as Python's,
//! with the same verdict. `fingerprint` of each changed snapshot is held to
//! Python's too.

use pr_hygiene_engine::policy::{evaluate, fingerprint};
use pr_hygiene_engine::pycompat::object::py_float_repr;
use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyErr, PyInt, PyValue};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

fn conformance(path: &str) -> String {
    let full = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../conformance")
        .join(path);
    std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{}: {e}", full.display()))
}

/// A replacement in the generator's tagged form.
fn untag(value: &Value) -> PyValue {
    match value {
        Value::Null => PyValue::None,
        Value::Bool(b) => PyValue::Bool(*b),
        Value::String(s) => PyValue::Str(s.clone()),
        Value::Array(items) => PyValue::List(items.iter().map(untag).collect()),
        Value::Object(map) if map.contains_key("int") => {
            PyValue::Int(PyInt::from_decimal(map["int"].as_str().unwrap()).unwrap())
        }
        Value::Object(map) if map.contains_key("float") => PyValue::Float(f64::from_bits(
            u64::from_str_radix(map["float"].as_str().unwrap(), 16).unwrap(),
        )),
        Value::Object(map) => PyValue::Dict(
            map["dict"]
                .as_array()
                .unwrap()
                .iter()
                .map(|pair| (pair[0].as_str().unwrap().to_owned(), untag(&pair[1])))
                .collect(),
        ),
        Value::Number(n) => panic!("untagged number {n}"),
    }
}

/// Deletes or replaces what `path` names inside `value`.
fn mutate(value: &mut PyValue, path: &[Value], replacement: &Value) {
    let Some((last, parents)) = path.split_last() else {
        *value = untag(replacement);
        return;
    };
    let mut here = value;
    for step in parents {
        here = step_into(here, step);
    }
    let delete = replacement.get("delete").is_some();
    match (here, last) {
        (PyValue::Dict(entries), Value::String(key)) if delete => {
            entries.shift_remove(key);
        }
        (PyValue::Dict(entries), Value::String(key)) => {
            entries.insert(key.clone(), untag(replacement));
        }
        (PyValue::List(items), Value::Number(n)) => {
            items[n.as_u64().unwrap() as usize] = untag(replacement);
        }
        (other, step) => panic!("cannot set {step} in {other:?}"),
    }
}

fn step_into<'a>(value: &'a mut PyValue, step: &Value) -> &'a mut PyValue {
    match (value, step) {
        (PyValue::Dict(entries), Value::String(key)) => entries.get_mut(key).unwrap(),
        (PyValue::List(items), Value::Number(n)) => &mut items[n.as_u64().unwrap() as usize],
        (other, step) => panic!("cannot step {step} into {other:?}"),
    }
}

/// The result with every float as `{"float": repr}`, as the generator
/// writes it before taking the digest.
fn floats_tagged(value: &PyValue) -> PyValue {
    match value {
        PyValue::Float(f) => PyValue::Dict(
            [("float".to_owned(), PyValue::Str(py_float_repr(*f)))]
                .into_iter()
                .collect(),
        ),
        PyValue::List(items) => PyValue::List(items.iter().map(floats_tagged).collect()),
        PyValue::Dict(entries) => PyValue::Dict(
            entries
                .iter()
                .map(|(k, v)| (k.clone(), floats_tagged(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn class(error: &PyErr) -> &'static str {
    match error {
        PyErr::Value(_) => "ValueError",
        PyErr::Type(_) => "TypeError",
        PyErr::Key(_) => "KeyError",
        PyErr::Attribute(_) => "AttributeError",
        PyErr::Overflow(_) => "OverflowError",
        PyErr::Recursion(_) => "RecursionError",
        PyErr::Unported(_) => "not ported",
    }
}

fn field<'a>(value: &'a PyValue, key: &str) -> &'a PyValue {
    match value {
        PyValue::Dict(entries) => &entries[key],
        _ => panic!("not a dict"),
    }
}

#[test]
fn evaluate_raises_and_answers_where_pythons_did() {
    let golden: Value =
        serde_json::from_str(&conformance("pycompat/policy_malformed.json")).unwrap();
    let mut failures = Vec::new();
    let cases = golden["cases"].as_array().unwrap();
    assert!(cases.len() > 2000, "only {} cases", cases.len());
    for entry in cases {
        let base = entry["base"].as_str().unwrap();
        let mut case = py_loads(&conformance(base)).unwrap();
        let target = entry["target"].as_str().unwrap();
        let PyValue::Dict(fields) = &mut case else {
            panic!("{base} is not an object")
        };
        mutate(
            fields.get_mut(target).unwrap(),
            entry["path"].as_array().unwrap(),
            &entry["value"],
        );
        let label = format!("{base} {target}{} = {}", entry["path"], entry["value"]);
        let is_float = entry["value"].get("float").is_some();
        // The one refusal the port makes on purpose: a float where a print
        // is hashed. Python's JSON writer would write it; the engine's
        // writes no float, and nothing GitHub answers puts one there, so the
        // run fails loudly instead.
        let refused_float = |error: &PyErr| {
            is_float && matches!(error, PyErr::Unported(why) if why.contains("float"))
        };
        if let Some(python) = entry.get("fingerprint") {
            match (fingerprint(field(&case, "pr")), python) {
                (Ok(ours), Value::String(theirs)) if ours == *theirs => {}
                (Err(error), Value::Object(raised)) if raised["raised"] == class(&error) => {}
                (Err(error), Value::String(_)) if refused_float(&error) => {}
                (ours, theirs) => {
                    failures.push(format!("{label}: fingerprint {ours:?}, python {theirs}"))
                }
            }
        }
        let expected = &entry["outcome"];
        let answer = evaluate(
            field(&case, "policy"),
            field(&case, "pr"),
            field(&case, "admitted_at"),
            field(&case, "now"),
            field(&case, "telemetry_states"),
        );
        let result = match (answer, expected["raised"].as_str()) {
            (Err(error), Some(raised)) => {
                if class(&error) != raised {
                    failures.push(format!("{label}: raised {error:?}, python {raised}"));
                }
                continue;
            }
            (Err(error), None) if refused_float(&error) => {
                continue;
            }
            (Err(error), None) => {
                failures.push(format!("{label}: raised {error:?}, python answered"));
                continue;
            }
            (Ok(result), Some(raised)) => {
                failures.push(format!(
                    "{label}: answered {:?}, python raised {raised}",
                    result.get("state")
                ));
                continue;
            }
            (Ok(result), None) => result,
        };
        let mut result = PyValue::Dict(result);
        // Where Python's verdict carries the text of an exception, the
        // port's carries the same words: the corpus leaves them out of its
        // comparison, and this holds them to Python's anyway.
        if let Some(text) = expected["exception_text"].as_str() {
            let PyValue::Dict(entries) = &mut result else {
                unreachable!()
            };
            let Some(PyValue::List(blockers)) = entries.get_mut("blockers") else {
                panic!("no blockers")
            };
            let at = blockers
                .iter()
                .position(|b| !matches!(b, PyValue::Str(s) if s.starts_with("Proceeded without")));
            match at.map(|at| blockers.remove(at)) {
                Some(PyValue::Str(ours)) if ours == text => {}
                ours => failures.push(format!("{label}: ours {ours:?}, python {text:?}")),
            }
        }
        let written = py_dumps(&floats_tagged(&result), false, Some((",", ":")), None).unwrap();
        let digest: String = Sha256::digest(written.as_bytes())[..8]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        if digest != expected["digest"].as_str().unwrap() {
            failures.push(format!("{label}: {written}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} differ:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
