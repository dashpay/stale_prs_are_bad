use crate::{tag, text, untag};
use pr_hygiene_engine::pycompat::object::{py_eq, py_float_repr, py_order, py_sort_by, py_str};
use pr_hygiene_engine::pycompat::tables::PYTHON_VERSION;
use pr_hygiene_engine::pycompat::text::py_upper_ascii;
use pr_hygiene_engine::pycompat::{py_loads, PyErr, PyValue};
use serde_json::{json, Value};
use std::cmp::Ordering;

/// `object.json`, which `generate_policy.py` writes. It records only the
/// minor version of the Python it ran on, which must be the tables'.
fn golden() -> Value {
    let path = format!(
        "{}/../conformance/pycompat/object.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let value: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"));
    let minor: Vec<&str> = PYTHON_VERSION.split('.').take(2).collect();
    assert_eq!(
        value["python"],
        json!(minor.join(".")),
        "object.json is from another Python"
    );
    value
}

/// The section `key`, which must hold at least `least` entries.
fn pairs<'a>(file: &'a Value, key: &str, least: usize) -> &'a Vec<Value> {
    let section = file[key].as_array().unwrap_or_else(|| panic!("no {key}"));
    assert!(section.len() >= least, "only {} in {key}", section.len());
    section
}

#[test]
fn floats_are_written_as_python_writes_them() {
    let file = golden();
    let mut failures = Vec::new();
    for pair in pairs(&file, "float_repr", 400) {
        let bits = u64::from_str_radix(text(&pair[0]), 16).unwrap();
        let ours = py_float_repr(f64::from_bits(bits));
        if ours != text(&pair[1]) {
            failures.push(format!("{}: ours {ours}, python {}", pair[0], pair[1]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn str_of_a_value_is_pythons() {
    let file = golden();
    for pair in pairs(&file, "str", 20) {
        let value = untag(&pair[0]);
        assert_eq!(py_str(&value), text(&pair[1]), "{}", pair[0]);
    }
}

#[test]
fn equality_crosses_types_as_pythons_does() {
    // Two documents read separately: `==` between what `json.loads` made.
    let file = golden();
    let mut failures = Vec::new();
    for case in pairs(&file, "eq", 1000) {
        let a = py_loads(text(&case[0])).unwrap();
        let b = py_loads(text(&case[1])).unwrap();
        if json!(py_eq(&a, &b)) != case[2] {
            failures.push(format!("{} == {}: python {}", case[0], case[1], case[2]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn ordering_refuses_and_allows_what_pythons_does() {
    let file = golden();
    let mut failures = Vec::new();
    for case in pairs(&file, "order", 800) {
        let a = py_loads(text(&case[0])).unwrap();
        let b = py_loads(text(&case[1])).unwrap();
        let ours = match py_order(&a, &b) {
            Ok(order) => json!({ "lt": order == Ordering::Less, "gt": order == Ordering::Greater }),
            Err(PyErr::Type(message)) => json!({ "type_error": message }),
            Err(other) => panic!("{} < {}: {other:?}", case[0], case[1]),
        };
        if ours != case[2] {
            failures.push(format!(
                "{} < {}: ours {ours}, python {}",
                case[0], case[1], case[2]
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn uppercase_into_ascii_is_pythons_for_every_character() {
    // Python's list is every non-ASCII code point whose `upper()` is
    // ASCII; for every other one the port must say it is not.
    let file = golden();
    let listed: Vec<(char, String)> = pairs(&file, "upper_into_ascii", 10)
        .iter()
        .map(|pair| {
            let code = pair[0].as_u64().unwrap() as u32;
            (char::from_u32(code).unwrap(), text(&pair[1]).to_owned())
        })
        .collect();
    for c in crate::scalars().filter(|c| !c.is_ascii()) {
        let expected = listed
            .iter()
            .find(|(listed, _)| *listed == c)
            .map(|(_, upper)| upper.clone());
        assert_eq!(
            py_upper_ascii(&c.to_string()),
            expected,
            "U+{:04X}",
            c as u32
        );
    }
    assert_eq!(py_upper_ascii("approved").as_deref(), Some("APPROVED"));
    assert_eq!(
        py_upper_ascii("changeſ_requeſted").as_deref(),
        Some("CHANGES_REQUESTED")
    );
}

#[test]
fn sorting_leaves_pythons_order_and_raises_where_python_raises() {
    // Under 64 items CPython compares the same pairs as the port, so the
    // first incomparable pair it meets is the same one.
    let file = golden();
    let mut failures = Vec::new();
    for case in pairs(&file, "sort", 300) {
        let PyValue::List(items) = untag(&case["items"]) else {
            panic!("not a list")
        };
        let mut items = items.into_vec();
        let ours = match py_sort_by(&mut items, py_order) {
            Ok(()) => json!({ "sorted": tag(&PyValue::List(items.into())) }),
            Err(PyErr::Type(message)) => json!({ "type_error": message }),
            Err(other) => panic!("{}: {other:?}", case["items"]),
        };
        let python = match case.get("sorted") {
            Some(sorted) => json!({ "sorted": sorted }),
            None => json!({ "type_error": case["type_error"] }),
        };
        if ours != python {
            failures.push(format!("{}: ours {ours}, python {python}", case["items"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
