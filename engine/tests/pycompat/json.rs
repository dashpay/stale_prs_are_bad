use crate::{cases, golden, strip_repr, tag, text, untag};
use pr_hygiene_engine::pycompat::json::{INT_MAX_STR_DIGITS, MAX_DEPTH};
use pr_hygiene_engine::pycompat::{
    py_dumps, py_loads, py_raw_decode, FloatNotWritten, PyErr, PyValue, ValueError,
};
use serde_json::{json, Value};

/// What Python's answer says, in the shape the generator wrote it.
fn outcome(result: Result<(PyValue, Option<usize>), PyErr>) -> Value {
    match result {
        Ok((value, None)) => json!({ "ok": tag(&value) }),
        Ok((value, Some(end))) => json!({ "ok": tag(&value), "end": end }),
        Err(PyErr::Value(ValueError::LoneSurrogate { .. })) => json!({ "lone_surrogate": true }),
        Err(PyErr::Value(ValueError::JsonDecode(e))) => json!({
            "error": { "type": "JSONDecodeError", "message": e.to_string(), "pos": e.pos }
        }),
        Err(PyErr::Value(e)) => {
            json!({ "error": { "type": "ValueError", "message": e.to_string() } })
        }
        Err(PyErr::Recursion(m)) => json!({ "error": { "type": "RecursionError", "message": m } }),
        Err(other) => panic!("not an error json raises: {other:?}"),
    }
}

#[test]
fn loads_and_raw_decode_answer_as_python_does() {
    let file = golden("json_loads.json");
    let mut failures = Vec::new();
    for case in cases(&file) {
        let doc = text(&case["doc"]);
        let loads = outcome(py_loads(doc).map(|v| (v, None)));
        let raw = outcome(py_raw_decode(doc).map(|r| {
            // The byte offset is the same place as Python's code points.
            assert_eq!(doc[..r.end_byte].chars().count(), r.end);
            (r.value, Some(r.end))
        }));
        if loads != strip_repr(&case["loads"]) {
            failures.push(format!("loads {doc:?}: {loads} != {}", case["loads"]));
        }
        if raw != strip_repr(&case["raw_decode"]) {
            failures.push(format!(
                "raw_decode {doc:?}: {raw} != {}",
                case["raw_decode"]
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} documents differ:\n{}",
        failures.len(),
        cases(&file).len(),
        failures.join("\n")
    );
}

#[test]
fn nesting_and_integer_limits_are_pythons() {
    let meta = golden("meta.json");
    assert_eq!(meta["json"]["max_depth"], MAX_DEPTH);
    assert_eq!(meta["json"]["int_max_str_digits"], INT_MAX_STR_DIGITS);
    for (kind, open, inner, close) in [("array", "[", "", "]"), ("object", "{\"a\":", "1", "}")] {
        let at_limit = format!(
            "{}{inner}{}",
            open.repeat(MAX_DEPTH),
            close.repeat(MAX_DEPTH)
        );
        assert!(py_loads(&at_limit).is_ok(), "{kind} at Python's limit");
        let over = format!(
            "{}{inner}{}",
            open.repeat(MAX_DEPTH + 1),
            close.repeat(MAX_DEPTH + 1)
        );
        match py_loads(&over) {
            Err(PyErr::Recursion(message)) => {
                assert_eq!(message, text(&meta["json"]["recursion_messages"][kind]));
            }
            other => panic!("{kind} past Python's limit: {other:?}"),
        }
    }
}

#[test]
fn a_duplicate_key_keeps_its_first_place_and_last_value() {
    let value = py_loads(r#"{"a": 1, "b": 2, "a": 3}"#).unwrap();
    let PyValue::Dict(entries) = value else {
        panic!("not a dict")
    };
    let keys: Vec<&str> = entries.keys().map(String::as_str).collect();
    assert_eq!(keys, ["a", "b"]);
    assert_eq!(tag(&entries["a"]), json!({ "int": "3" }));
}

#[test]
fn dumps_writes_pythons_bytes_in_every_shape_the_engine_uses() {
    let file = golden("json_dumps.json");
    type Separators = Option<(&'static str, &'static str)>;
    let shapes: [(&str, bool, Separators, Option<usize>); 5] = [
        ("compact_sorted", true, Some((",", ":")), None),
        ("compact", false, Some((",", ":")), None),
        ("default_sorted", true, None, None),
        ("default", false, None, None),
        ("indent2", false, None, Some(2)),
    ];
    for case in cases(&file) {
        let value = untag(&case["value"]);
        let has_float = case["value"].to_string().contains("\"float\"");
        for (name, sort_keys, separators, indent) in shapes {
            let written = py_dumps(&value, sort_keys, separators, indent);
            if has_float {
                assert_eq!(written, Err(FloatNotWritten), "{name}: {}", case["value"]);
            } else {
                assert_eq!(
                    written.as_deref(),
                    Ok(text(&case["out"][name])),
                    "{name}: {}",
                    case["value"]
                );
            }
        }
    }
}

#[test]
fn what_dumps_writes_loads_reads_back() {
    // The engine reads its own records back; a round trip must be exact.
    let file = golden("json_dumps.json");
    for case in cases(&file) {
        if case["value"].to_string().contains("\"float\"") {
            continue;
        }
        for sort_keys in [false, true] {
            let written = py_dumps(&untag(&case["value"]), sort_keys, None, None).unwrap();
            let again = py_dumps(&py_loads(&written).unwrap(), sort_keys, None, None).unwrap();
            assert_eq!(again, written);
        }
    }
}
