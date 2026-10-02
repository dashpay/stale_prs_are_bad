//! The generated tables, checked against Python over every code point.
//!
//! The generator also wrote, for each set, how many code points Python put
//! in it and a hash of them. Each set is rebuilt here from the Rust side's
//! own answers (the string functions, or the classes compiled by the regex
//! crate) and must hash the same: that proves the tables were written,
//! read and compiled without losing or gaining a single code point.

use crate::{fnv, golden, scalars};
use pr_hygiene_engine::pycompat::tables;
use pr_hygiene_engine::pycompat::text::{
    py_isprintable_char, py_isspace_char, py_lower, py_splitlines,
};
use regex::Regex;
use serde_json::{json, Value};
use std::sync::OnceLock;

fn summary(set: &[u32]) -> Value {
    json!({ "count": set.len(), "fnv": fnv(set.iter().copied()) })
}

fn where_true(test: impl Fn(char) -> bool) -> Vec<u32> {
    scalars().filter(|&c| test(c)).map(u32::from).collect()
}

/// Every scalar value in one string, for one pass of a regex over all.
fn haystack() -> &'static str {
    static ALL: OnceLock<String> = OnceLock::new();
    ALL.get_or_init(|| scalars().collect())
}

/// What a single-character class matches, out of every scalar value.
fn matched_by(class: &str) -> Vec<u32> {
    let re = Regex::new(class).unwrap_or_else(|e| panic!("{class}: {e}"));
    re.find_iter(haystack())
        .map(|m| {
            let mut chars = m.as_str().chars();
            let c = chars.next().expect("one character");
            assert!(
                chars.next().is_none(),
                "{class} matched more than one character"
            );
            u32::from(c)
        })
        .collect()
}

#[test]
fn string_tables_are_pythons() {
    let file = golden("tables.json");
    let sets = &file["sets"];
    assert_eq!(summary(&where_true(py_isspace_char)), sets["space"]);
    assert_eq!(
        summary(&where_true(|c| py_splitlines(&format!("a{c}b"), false)
            .len()
            == 2)),
        sets["line_break"]
    );
    assert_eq!(summary(&where_true(py_isprintable_char)), sets["printable"]);
}

#[test]
fn lower_is_pythons_for_every_character() {
    let file = golden("tables.json");
    let mut count = 0;
    let mut stream = Vec::new();
    for c in scalars() {
        let lowered = py_lower(c.encode_utf8(&mut [0; 4]));
        if lowered.chars().ne([c]) {
            count += 1;
            stream.push(u32::from(c));
            stream.extend(lowered.chars().map(u32::from));
            stream.push(u32::MAX);
        }
    }
    assert_eq!(json!({ "count": count, "fnv": fnv(stream) }), file["lower"]);
}

#[test]
fn final_sigma_reads_pythons_case_properties() {
    // The generator found the two properties from how Python lowers "XΣ"
    // and "AXΣ"; the same probes through py_lower must find the same sets.
    let file = golden("tables.json");
    let ends_final = |s: String| py_lower(&s).ends_with('ς');
    let cased = where_true(|c| ends_final(format!("{c}Σ")));
    let ignorable = where_true(|c| ends_final(format!("A{c}Σ")) && !ends_final(format!("{c}Σ")));
    assert_eq!(summary(&cased), file["sets"]["cased"]);
    assert_eq!(summary(&ignorable), file["sets"]["case_ignorable"]);
}

#[test]
fn regex_classes_match_what_python_matches() {
    let file = golden("tables.json");
    let sets = &file["sets"];
    assert_eq!(
        summary(&matched_by(&format!("[{}]", tables::RE_SPACE))),
        sets["space"]
    );
    assert_eq!(
        summary(&matched_by(&format!("[{}]", tables::RE_WORD))),
        sets["re_word"]
    );
    assert_eq!(
        summary(&matched_by(&format!("[{}]", tables::RE_DIGIT))),
        sets["re_digit"]
    );
    assert_eq!(
        summary(&matched_by(tables::RE_IGNORECASE_HEX)),
        sets["RE_IGNORECASE_HEX"]
    );
    assert_eq!(
        summary(&matched_by(tables::RE_IGNORECASE_NOT_LETTER_OR_LT)),
        sets["RE_IGNORECASE_NOT_LETTER_OR_LT"]
    );
}

#[test]
fn case_insensitive_letters_match_what_python_matches() {
    let file = golden("tables.json");
    for (i, letter) in ('a'..='z').enumerate() {
        let python: Vec<u32> = file["folds"][letter.to_string()]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u32)
            .collect();
        assert_eq!(matched_by(tables::RE_FOLD[i]), python, "(?i){letter}");
    }
    // The ones the engine's patterns meet: Python folds the dotted and
    // dotless I, the long s and the Kelvin sign onto ASCII letters.
    assert_eq!(tables::RE_FOLD[8], r"[Ii\x{130}-\x{131}]");
    assert_eq!(tables::RE_FOLD[10], r"[Kk\x{212a}]");
    assert_eq!(tables::RE_FOLD[18], r"[Ss\x{17f}]");
}
