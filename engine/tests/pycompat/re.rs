use crate::{cases, golden, text};
use pr_hygiene_engine::pycompat::re::{fold_literal, fullmatch_pattern, match_pattern, translate};
use pr_hygiene_engine::pycompat::tables::{RE_IGNORECASE_HEX, RE_SPACE};
use regex::{Captures, Regex};
use serde_json::{json, Value};

/// A byte offset as Python's code-point offset.
fn chars(s: &str, byte: usize) -> usize {
    s[..byte].chars().count()
}

fn record(s: &str, re: &Regex, caps: Option<Captures>) -> Value {
    let Some(caps) = caps else {
        return Value::Null;
    };
    let whole = caps.get(0).expect("group 0");
    let groups: serde_json::Map<String, Value> = re
        .capture_names()
        .flatten()
        .map(|name| (name.to_owned(), json!(caps.name(name).map(|m| m.as_str()))))
        .collect();
    json!({ "span": [chars(s, whole.start()), chars(s, whole.end())], "groups": groups })
}

/// The attestation pattern of `pr_review/policy.py`, which Python compiles
/// with `re.IGNORECASE`:
///
/// `/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?`
///
/// Ported with explicit sets: each letter folded as Python folds it, the
/// hex class enumerated under IGNORECASE, `\s` as Python's.
fn attestation() -> String {
    format!(
        "/{}[- ]?{}(?:{})?(?:[{RE_SPACE}]+(?P<head>{RE_IGNORECASE_HEX}{{40}}))?",
        fold_literal("self").unwrap(),
        fold_literal("review").unwrap(),
        fold_literal("ed").unwrap(),
    )
}

#[test]
fn ported_patterns_match_what_python_matched() {
    let file = golden("regex.json");
    let mut failures = Vec::new();
    let mut checked = 0;
    for case in cases(&file) {
        let name = text(&case["name"]);
        let source = if case["ignorecase"] == true {
            assert_eq!(
                name, "attestation",
                "a case-insensitive pattern needs a hand port"
            );
            attestation()
        } else {
            translate(text(&case["pattern"])).unwrap_or_else(|e| panic!("{name}: {e}"))
        };
        let method = text(&case["method"]);
        let source = match method {
            "fullmatch" => fullmatch_pattern(&source),
            "match" => match_pattern(&source),
            _ => source,
        };
        let re = Regex::new(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        for pair in case["results"].as_array().unwrap() {
            let s = text(&pair[0]);
            let ours = if method == "finditer" {
                json!(re
                    .find_iter(s)
                    .map(|m| json!([chars(s, m.start()), chars(s, m.end())]))
                    .collect::<Vec<_>>())
            } else {
                record(s, &re, re.captures(s))
            };
            checked += 1;
            if ours != pair[1] {
                failures.push(format!("{name} {s:?}: ours {ours}, python {}", pair[1]));
            }
        }
    }
    assert!(checked > 1000, "only {checked} inputs");
    assert!(
        failures.is_empty(),
        "{} of {checked} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
