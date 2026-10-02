use crate::{golden, text, untag};
use pr_hygiene_engine::pycompat::PyInt;
use serde_json::json;

#[test]
fn integers_compare_and_print_as_python_does() {
    let file = golden("values.json");
    let ints: Vec<PyInt> = file["ints"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let i = PyInt::from_decimal(text(v)).unwrap();
            assert_eq!(i.to_string(), text(v), "str(int)");
            i
        })
        .collect();
    let compare = file["compare"].as_array().unwrap();
    for (a, row) in ints.iter().zip(compare) {
        for (b, expected) in ints.iter().zip(row.as_array().unwrap()) {
            let ours = a.cmp(b) as i64;
            assert_eq!(json!(ours), *expected, "{a} vs {b}");
            assert_eq!(a == b, ours == 0, "{a} == {b}");
        }
    }
}

#[test]
fn truthiness_is_pythons() {
    let file = golden("values.json");
    for pair in file["truthiness"].as_array().unwrap() {
        assert_eq!(
            json!(untag(&pair[0]).truthy()),
            pair[1],
            "bool({})",
            pair[0]
        );
    }
}
