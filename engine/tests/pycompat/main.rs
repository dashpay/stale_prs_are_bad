//! The `pycompat` layer replayed against what Python 3.12 answered.
//!
//! The answers live in `conformance/pycompat/`, written by `generate.py`
//! there. They are read with serde_json, not with the crate's own reader,
//! so a mistake in the reader cannot hide in how its expectations are read.

mod datetime;
mod json;
mod object;
mod ops;
mod re;
mod tables;
mod text;
mod ties;
mod values;

use pr_hygiene_engine::pycompat::{PyInt, PyValue};
use serde_json::{json, Value};

/// A golden file: its fields, with `cases` one per line.
pub fn golden(name: &str) -> Value {
    let path = format!(
        "{}/../conformance/pycompat/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let value: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"));
    let meta: Value = serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{}/../conformance/pycompat/meta.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap(),
    )
    .unwrap();
    // Every file comes from one run of the generator, on one Python.
    assert_eq!(
        value["python"], meta["python"],
        "{name} is from another Python"
    );
    assert_eq!(
        value["python"],
        pr_hygiene_engine::pycompat::tables::PYTHON_VERSION,
        "{name} and the generated tables are from different Pythons"
    );
    value
}

pub fn cases(file: &Value) -> &Vec<Value> {
    file["cases"].as_array().expect("cases")
}

pub fn text(value: &Value) -> &str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("not a string: {value}"))
}

/// A `PyValue` in the generator's tagged form: ints as `{"int": text}`,
/// floats as `{"float": big-endian bits}`, dicts as ordered pairs.
pub fn tag(value: &PyValue) -> Value {
    match value {
        PyValue::None => Value::Null,
        PyValue::Bool(b) => json!(b),
        PyValue::Int(i) => json!({ "int": i.to_string() }),
        PyValue::Float(f) => json!({ "float": format!("{:016x}", f.to_bits()) }),
        PyValue::Str(s) => json!(s),
        PyValue::List(items) => Value::Array(items.iter().map(tag).collect()),
        PyValue::Dict(entries) => json!({
            "dict": entries.iter().map(|(k, v)| json!([k, tag(v)])).collect::<Vec<_>>()
        }),
    }
}

/// The tagged form without each float's `repr`, which only helps a reader.
pub fn strip_repr(value: &Value) -> Value {
    match value {
        Value::Object(map) if map.contains_key("float") => json!({ "float": map["float"] }),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), strip_repr(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(strip_repr).collect()),
        other => other.clone(),
    }
}

/// The `PyValue` a tagged value stands for.
pub fn untag(value: &Value) -> PyValue {
    match value {
        Value::Null => PyValue::None,
        Value::Bool(b) => PyValue::Bool(*b),
        Value::String(s) => PyValue::Str(s.clone()),
        Value::Array(items) => PyValue::List(items.iter().map(untag).collect()),
        Value::Object(map) if map.contains_key("int") => {
            PyValue::Int(PyInt::from_decimal(text(&map["int"])).expect("int"))
        }
        Value::Object(map) if map.contains_key("float") => {
            let bits = u64::from_str_radix(text(&map["float"]), 16).expect("float bits");
            PyValue::Float(f64::from_bits(bits))
        }
        Value::Object(map) => PyValue::Dict(
            map["dict"]
                .as_array()
                .expect("dict pairs")
                .iter()
                .map(|pair| (text(&pair[0]).to_owned(), untag(&pair[1])))
                .collect(),
        ),
        Value::Number(n) => panic!("untagged number {n}"),
    }
}

/// FNV-1a over 32-bit little-endian values, as the generator computes it.
pub fn fnv(values: impl IntoIterator<Item = u32>) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for v in values {
        for b in v.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("{h:016x}")
}

/// Every Unicode scalar value, in order.
pub fn scalars() -> impl Iterator<Item = char> {
    (0..=0x10FFFF).filter_map(char::from_u32)
}
