use crate::{cases, golden, text};
use pr_hygiene_engine::pycompat::text::{
    py_isspace, py_len, py_lower, py_lstrip, py_repr_str, py_rstrip, py_slice, py_split,
    py_splitlines, py_strip,
};
use serde_json::{json, Value};

fn index(value: &Value) -> Option<isize> {
    value.as_i64().map(|i| i as isize)
}

#[test]
fn string_methods_answer_as_python_does() {
    let file = golden("text.json");
    let mut failures = Vec::new();
    for case in cases(&file) {
        let s = text(&case["s"]);
        let slices: Vec<Value> = case["slices"]
            .as_array()
            .unwrap()
            .iter()
            .map(|slice| {
                json!([
                    slice[0],
                    slice[1],
                    py_slice(s, index(&slice[0]), index(&slice[1]))
                ])
            })
            .collect();
        let ours = json!({
            "s": s,
            "len": py_len(s),
            "isspace": py_isspace(s),
            "strip": py_strip(s),
            "lstrip": py_lstrip(s),
            "rstrip": py_rstrip(s),
            "split": py_split(s),
            "splitlines": py_splitlines(s, false),
            "splitlines_keepends": py_splitlines(s, true),
            "lower": py_lower(s),
            "repr": py_repr_str(s),
            "slices": slices,
        });
        if &ours != case {
            failures.push(format!("{s:?}:\n  ours   {ours}\n  python {case}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} strings differ:\n{}",
        failures.len(),
        cases(&file).len(),
        failures.join("\n")
    );
}
