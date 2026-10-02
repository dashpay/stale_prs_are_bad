use crate::{cases, golden, text};
use pr_hygiene_engine::pycompat::{py_max_by_key, py_min_by_key};
use serde_json::json;

#[test]
fn max_and_min_keep_the_element_python_keeps() {
    let file = golden("ties.json");
    for case in cases(&file) {
        let keys: Vec<(i64, String)> = case["keys"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| (k[0].as_i64().unwrap(), text(&k[1]).to_owned()))
            .collect();
        let positions = 0..keys.len();
        let first = |i: &usize| keys[*i].0;
        let whole = |i: &usize| keys[*i].clone();
        assert_eq!(
            json!(py_max_by_key(positions.clone(), first)),
            case["max"],
            "{case}"
        );
        assert_eq!(
            json!(py_min_by_key(positions.clone(), first)),
            case["min"],
            "{case}"
        );
        assert_eq!(
            json!(py_max_by_key(positions.clone(), whole)),
            case["max_tuple"],
            "{case}"
        );
        assert_eq!(
            json!(py_min_by_key(positions, whole)),
            case["min_tuple"],
            "{case}"
        );
    }
}
