//! `pycompat::ops` against what Python 3.12 answered: `==` and `<` between
//! values read from JSON, and `sorted()`, from `object.json`.

use crate::object::{golden, pairs};
use crate::{tag, text, untag};
use pr_hygiene_engine::pycompat::ops::{py_compare, py_eq, py_sort_by, Compare};
use pr_hygiene_engine::pycompat::{py_loads, PyErr, PyValue};
use serde_json::json;

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
        // `a < b`, then `b < a`, as Python asked them.
        let ours = match py_compare(&a, Compare::Lt, &b) {
            Ok(lt) => json!({ "lt": lt, "gt": py_compare(&b, Compare::Lt, &a).unwrap() }),
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
        let ours = match py_sort_by(&mut items, |a, b| py_compare(a, Compare::Lt, b)) {
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
