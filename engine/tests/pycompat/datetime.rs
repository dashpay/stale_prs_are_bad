use crate::{cases, golden, text};
use pr_hygiene_engine::pycompat::{PyDateTime, PyErr};
use serde_json::{json, Value};
use std::cmp::Ordering;

fn outcome(s: &str) -> Value {
    match PyDateTime::fromisoformat(s) {
        Ok(v) => json!({ "ok": {
            "fields": [v.year(), v.month(), v.day(), v.hour(), v.minute(), v.second(), v.microsecond()],
            "offset_us": v.utcoffset().map(|o| o.micros()),
            "isoformat": v.isoformat(),
            "isoformat_seconds": v.isoformat_seconds(),
            "timestamp": v.timestamp().map(|t| format!("{:016x}", t.to_bits())),
        }}),
        Err(PyErr::Value(e)) => {
            json!({ "error": { "type": "ValueError", "message": e.to_string() } })
        }
        Err(other) => panic!("{s:?}: not a ValueError: {other:?}"),
    }
}

#[test]
fn fromisoformat_answers_as_python_does() {
    let file = golden("datetime.json");
    let mut failures = Vec::new();
    for case in cases(&file) {
        let s = text(&case["s"]);
        let mut python = case.clone();
        let fields = python.as_object_mut().unwrap();
        fields.remove("s");
        fields.remove("pure_python_differs");
        let ours = outcome(s);
        if ours != python {
            failures.push(format!("{s:?}: ours {ours}, python {python}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} differ:\n{}",
        failures.len(),
        cases(&file).len(),
        failures.join("\n")
    );
}

#[test]
fn comparison_and_subtraction_answer_as_python_does() {
    let file = golden("datetime.json");
    for pair in file["pairs"].as_array().unwrap() {
        let a = PyDateTime::fromisoformat(text(&pair["a"])).unwrap();
        let b = PyDateTime::fromisoformat(text(&pair["b"])).unwrap();
        let context = format!("{} vs {}", pair["a"], pair["b"]);
        assert_eq!(json!(a == b), pair["eq"], "== {context}");
        let lt = match a.py_cmp(&b) {
            Ok(order) => json!(order == Ordering::Less),
            Err(PyErr::Type(m)) => json!({ "type_error": m }),
            Err(other) => panic!("{context}: {other:?}"),
        };
        assert_eq!(lt, pair["lt"], "< {context}");
        let sub = match a.py_sub(&b) {
            Ok(delta) => json!({
                "micros": delta.micros(),
                "total_seconds": format!("{:016x}", delta.total_seconds().to_bits()),
            }),
            Err(PyErr::Type(m)) => json!({ "type_error": m }),
            Err(other) => panic!("{context}: {other:?}"),
        };
        assert_eq!(sub, pair["sub"], "- {context}");
    }
}

#[test]
fn the_c_module_not_the_pure_python_one_is_the_reference() {
    // The engine runs CPython's C datetime. Where the pure-Python module
    // disagrees, the goldens hold the C answer; these are among them.
    let file = golden("datetime.json");
    let differs: Vec<&str> = cases(&file)
        .iter()
        .filter(|c| c["pure_python_differs"] == true)
        .map(|c| text(&c["s"]))
        .collect();
    for s in [
        "2024-01-01T12.5",
        "2024-01-01T",
        "2024-01-01T12:34:56+00:00:00.5",
    ] {
        assert!(differs.contains(&s), "{s:?} no longer differs");
    }
    assert_eq!(
        PyDateTime::fromisoformat("2024-01-01T12.5")
            .unwrap()
            .isoformat(),
        "2024-01-01T12:00:00.500000"
    );
}
