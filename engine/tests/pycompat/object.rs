use crate::{golden, text, untag};
use pr_hygiene_engine::pycompat::object::{py_eq, py_float_repr, py_order, py_str};
use pr_hygiene_engine::pycompat::text::py_upper_ascii;
use pr_hygiene_engine::pycompat::{py_loads, PyErr};
use serde_json::{json, Value};
use std::cmp::Ordering;

fn pairs<'a>(file: &'a Value, key: &str) -> &'a Vec<Value> {
    file[key].as_array().unwrap_or_else(|| panic!("no {key}"))
}

#[test]
fn floats_are_written_as_python_writes_them() {
    let file = golden("object.json");
    let mut failures = Vec::new();
    for pair in pairs(&file, "float_repr") {
        let bits = u64::from_str_radix(text(&pair[0]), 16).unwrap();
        let ours = py_float_repr(f64::from_bits(bits));
        if ours != text(&pair[1]) {
            failures.push(format!("{}: ours {ours}, python {}", pair[0], pair[1]));
        }
    }
    assert!(pairs(&file, "float_repr").len() > 400);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn str_of_a_value_is_pythons() {
    let file = golden("object.json");
    for pair in pairs(&file, "str") {
        let value = untag(&pair[0]);
        assert_eq!(py_str(&value), text(&pair[1]), "{}", pair[0]);
    }
}

#[test]
fn equality_crosses_types_as_pythons_does() {
    // Two documents read separately: `==` between what `json.loads` made.
    let file = golden("object.json");
    let mut failures = Vec::new();
    for case in pairs(&file, "eq") {
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
    let file = golden("object.json");
    let mut failures = Vec::new();
    for case in pairs(&file, "order") {
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
    let file = golden("object.json");
    let listed: Vec<(char, String)> = pairs(&file, "upper_into_ascii")
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
