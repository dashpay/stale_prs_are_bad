//! The policy's regular expressions, ported from Python's `re`.
//!
//! Each is built from Python's own pattern through [`crate::pycompat::re`]:
//! `\s`, `\w` and `\d` become the classes Python 3.12 uses, a pattern
//! compiled with `re.IGNORECASE` spells out the set each letter matches,
//! and `fullmatch` and `match` become anchors. The Python source of every
//! pattern sits beside its port; `conformance/pycompat/policy_regex.json`
//! holds what Python's `re` matched on inputs chosen to find where the two
//! could differ, and the tests below replay it.
//!
//! Patterns built from what the engine reads (a branch target, the heads a
//! review may name) are cached, as `re` caches the patterns it compiles.

use crate::pycompat::re::{fold_literal, fullmatch_pattern, match_pattern, translate};
use crate::pycompat::tables::{
    RE_DIGIT, RE_IGNORECASE_HEX, RE_IGNORECASE_NOT_LETTER_OR_LT, RE_SPACE,
};
use crate::pycompat::PyErr;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// A pattern built from constants. The tests below compile every one, so
/// a mistake in one fails them rather than a run.
fn constant(pattern: String) -> Regex {
    Regex::new(&pattern).expect("a constant pattern compiles, as the pattern tests show")
}

/// A Python pattern with nothing case-insensitive in it, translated.
fn python(pattern: &str) -> String {
    translate(pattern).expect("a constant pattern translates, as the pattern tests show")
}

/// Text spelled out with the set each letter matches under `re.IGNORECASE`.
fn folded(text: &str) -> String {
    fold_literal(text).expect("constant text is ASCII, which has a fold set for every letter")
}

/// `re.fullmatch(r'[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?', handle)`.
pub(crate) static HANDLE: LazyLock<Regex> = LazyLock::new(|| {
    constant(fullmatch_pattern(&python(
        r"[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?",
    )))
});

/// `re.fullmatch(r'[\w.-]+/[\w.-]+', repository)`.
pub(crate) static REPOSITORY: LazyLock<Regex> =
    LazyLock::new(|| constant(fullmatch_pattern(&python(r"[\w.-]+/[\w.-]+"))));

/// `re.search(r'[?\[\]{}\\]|\*\*', branch)`: branch syntax `governs` does
/// not understand.
pub(crate) static BRANCH_SYNTAX: LazyLock<Regex> =
    LazyLock::new(|| constant(python(r"[?\[\]{}\\]|\*\*")));

/// `re.fullmatch(r'[a-z0-9][a-z0-9-]*', area_id)`.
pub(crate) static AREA_ID: LazyLock<Regex> =
    LazyLock::new(|| constant(fullmatch_pattern(&python(r"[a-z0-9][a-z0-9-]*"))));

/// `re.fullmatch(r'(?:[A-Za-z0-9_.-]+/)+', prefix)`.
pub(crate) static PREFIX: LazyLock<Regex> =
    LazyLock::new(|| constant(fullmatch_pattern(&python(r"(?:[A-Za-z0-9_.-]+/)+"))));

/// `re.fullmatch(r'[0-9a-f]{40}', head)`.
pub(crate) static HEAD_SHA: LazyLock<Regex> =
    LazyLock::new(|| constant(fullmatch_pattern(&python(r"[0-9a-f]{40}"))));

/// `RISK_BLOCK`: `re.compile(r'<!-- final_review_risk_start -->(.*?)<!-- final_review_risk_end -->', re.S)`.
pub(crate) static RISK_BLOCK: LazyLock<Regex> = LazyLock::new(|| {
    constant(python(
        r"(?s)<!-- final_review_risk_start -->(.*?)<!-- final_review_risk_end -->",
    ))
});

/// `BOOKKEEPING`, which Python compiles case-insensitively:
///
/// `(?is)<details>\s*<summary>[^A-Za-z<]*(?:run configuration|commits|files selected for processing|recent review info)\s*(?:\(\d+\))?\s*</summary>.*?</details>`
pub(crate) static BOOKKEEPING: LazyLock<Regex> = LazyLock::new(|| {
    let names = [
        "run configuration",
        "commits",
        "files selected for processing",
        "recent review info",
    ]
    .map(folded)
    .join("|");
    constant(format!(
        r"(?s){details}[{RE_SPACE}]*{summary}{RE_IGNORECASE_NOT_LETTER_OR_LT}*(?:{names})[{RE_SPACE}]*(?:\([{RE_DIGIT}]+\))?[{RE_SPACE}]*{end_summary}.*?{end_details}",
        details = folded("<details>"),
        summary = folded("<summary>"),
        end_summary = folded("</summary>"),
        end_details = folded("</details>"),
    ))
});

/// `CHECKBOX_ITEM`: `(?m)^[-*]\s*\[[ xX]\]\s*<!--\s*\{"checkboxId"[^>]*-->.*$`.
pub(crate) static CHECKBOX_ITEM: LazyLock<Regex> = LazyLock::new(|| {
    constant(python(
        r#"(?m)^[-*]\s*\[[ xX]\]\s*<!--\s*\{"checkboxId"[^>]*-->.*$"#,
    ))
});

/// `CHECKBOX`: `<!--\s*\{"checkboxId"[^>]*-->`.
pub(crate) static CHECKBOX: LazyLock<Regex> =
    LazyLock::new(|| constant(python(r#"<!--\s*\{"checkboxId"[^>]*-->"#)));

/// `PASSED`: `✅`.
pub(crate) static PASSED: LazyLock<Regex> = LazyLock::new(|| constant(python("✅")));

/// `SEPARATOR`: `(?m)^\|(?:\s*:?-+:?\s*\|)+[ \t]*$\n?`.
pub(crate) static SEPARATOR: LazyLock<Regex> =
    LazyLock::new(|| constant(python(r"(?m)^\|(?:\s*:?-+:?\s*\|)+[ \t]*$\n?")));

/// `TABLE_ROW`: `(?m)^(\|(?:\\.|[^|\n\\])*\|((?:\\.|[^|\n\\])*)\|).*$`.
pub(crate) static TABLE_ROW: LazyLock<Regex> = LazyLock::new(|| {
    constant(python(
        r"(?m)^(\|(?:\\.|[^|\n\\])*\|((?:\\.|[^|\n\\])*)\|).*$",
    ))
});

/// `TABLE`: `(?m)(?:^\|.*\n?)+`.
pub(crate) static TABLE: LazyLock<Regex> = LazyLock::new(|| constant(python(r"(?m)(?:^\|.*\n?)+")));

/// `SPACES`: `\s+`.
pub(crate) static SPACES: LazyLock<Regex> = LazyLock::new(|| constant(python(r"\s+")));

/// `ATTESTATION.fullmatch`, which Python compiles case-insensitively:
///
/// `/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?`
pub(crate) static ATTESTATION: LazyLock<Regex> = LazyLock::new(|| {
    constant(fullmatch_pattern(&format!(
        "/{}[- ]?{}(?:{})?(?:[{RE_SPACE}]+(?P<head>{RE_IGNORECASE_HEX}{{40}}))?",
        folded("self"),
        folded("review"),
        folded("ed"),
    )))
});

/// `HIDDEN_MARKUP`: `re.compile(r'<!--.*?-->', re.S)`.
pub(crate) static HIDDEN_MARKUP: LazyLock<Regex> =
    LazyLock::new(|| constant(python(r"(?s)<!--.*?-->")));

/// `PASTA_HEADING`: `(?m)^\*\*(?P<label>[^\x00-\x7f][^*:\n]*):`.
pub(crate) static PASTA_HEADING: LazyLock<Regex> =
    LazyLock::new(|| constant(python(r"(?m)^\*\*(?P<label>[^\x00-\x7f][^*:\n]*):")));

/// `RABBIT_HEADING`: `(?m)^_[^_\n]+_(?:[ \t]*\|[ \t]*_[^_\n]+_)+[ \t\r]*$`.
pub(crate) static RABBIT_HEADING: LazyLock<Regex> = LazyLock::new(|| {
    constant(python(
        r"(?m)^_[^_\n]+_(?:[ \t]*\|[ \t]*_[^_\n]+_)+[ \t\r]*$",
    ))
});

/// `_rabbit_receipt`'s marker: `(?m)^<!-- ` + `re.escape(RECEIPT_MARKER)` + `:\s*`.
pub(crate) static RECEIPT_COVERAGE: LazyLock<Regex> = LazyLock::new(|| {
    constant(python(&format!(
        r"(?m)^<!-- {}:\s*",
        regex::escape(super::RECEIPT_MARKER)
    )))
});

/// `re.match(r'\s*-->[ \t]*(?:\r?\n|$)', rest)`, where only whether it
/// matches is read. Python's `$` without `(?m)` matches at the end and
/// before a final `\n`; consuming that `\n` (`\n?\z`) matches the same
/// texts.
pub(crate) static RECEIPT_TAIL: LazyLock<Regex> = LazyLock::new(|| {
    constant(match_pattern(&format!(
        r"{}(?:\r?\n|\n?\z)",
        python(r"\s*-->[ \t]*")
    )))
});

/// A marker section `receipt_print` drops: its opening alone on a line,
/// and the section from it to its own end.
pub(crate) struct Section {
    /// `(?m)^` + `re.escape(start)` + `[ \t]*$`
    pub(crate) alone: Regex,
    /// `(?ms)^` + `re.escape(start)` + `[ \t]*$.*?^` + `re.escape(end)` + `[ \t]*$`
    pub(crate) whole: Regex,
}

/// The sections `receipt_print` drops, in Python's order: `VOLATILE`, then
/// `NOTICES`.
pub(crate) static SECTIONS: LazyLock<Vec<Section>> = LazyLock::new(|| {
    super::receipts::section_markers()
        .iter()
        .map(|(start, end)| {
            let (start, end) = (regex::escape(start), regex::escape(end));
            Section {
                alone: constant(python(&format!(r"(?m)^{start}[ \t]*$"))),
                whole: constant(python(&format!(r"(?ms)^{start}[ \t]*$.*?^{end}[ \t]*$"))),
            }
        })
        .collect()
});

/// How many patterns `re` keeps compiled; this cache starts over once it
/// holds as many.
const MAX_CACHE: usize = 512;

/// Patterns built at run time, by the pattern text they compile.
static CACHE: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// `pattern`, compiled once. Every pattern built here is escaped text and
/// fixed classes, so one that does not compile is either a bug in the port
/// or input far past what the engine writes (a commit id thousands of
/// characters long, past the regex crate's size limit); either fails the
/// run rather than matching nothing.
fn cached(pattern: String) -> Result<Regex, PyErr> {
    // A thread that panicked holding the lock left a map of finished
    // entries; nothing is ever half-written to it.
    let lock = || {
        CACHE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    };
    if let Some(found) = lock().get(&pattern) {
        return Ok(found.clone());
    }
    // Compiled outside the lock; two threads may both compile one pattern.
    let built = Regex::new(&pattern)
        .map_err(|error| PyErr::Unported(format!("pattern {pattern:?}: {error}")))?;
    let mut cache = lock();
    if cache.len() >= MAX_CACHE {
        cache.clear();
    }
    cache.insert(pattern, built.clone());
    Ok(built)
}

/// `re.fullmatch('[^/]*'.join(map(re.escape, target.split('*'))), branch)`:
/// a branch target, `*` matching any run without `/`.
pub(crate) fn branch_target(target: &str) -> Result<Regex, PyErr> {
    let pattern = target
        .split('*')
        .map(regex::escape)
        .collect::<Vec<_>>()
        .join("[^/]*");
    let translated = translate(&pattern)
        .map_err(|error| PyErr::Unported(format!("target {target:?}: {error}")))?;
    cached(fullmatch_pattern(&translated))
}

/// The final-phase marker thepastaclaw writes in a review, for any of
/// `heads` (lowercase, sorted), which Python compiles case-insensitively:
///
/// `(?mi)^<!-- thepastaclaw-review-phase v1 phase=final sha=(` + the heads,
/// escaped and joined by `|` + `)(?:\s+[^<>]*?)?\s*-->`
///
/// A head holding a character outside ASCII has no case-insensitive set
/// read from Python, so it is left out and matches nothing: the bot then
/// reads as not having reported on it. The engine's own records only ever
/// name hexadecimal commits. `None` when no head is left.
pub(crate) fn final_phase(heads: &[String]) -> Result<Option<Regex>, PyErr> {
    let alternatives: Vec<String> = heads
        .iter()
        .filter_map(|head| fold_literal(head).ok())
        .collect();
    if alternatives.is_empty() {
        return Ok(None);
    }
    cached(format!(
        r"(?m)^{}({})(?:[{RE_SPACE}]+[^<>]*?)?[{RE_SPACE}]*\-\->",
        folded("<!-- thepastaclaw-review-phase v1 phase=final sha="),
        alternatives.join("|"),
    ))
    .map(Some)
}

#[cfg(test)]
mod tests {
    //! Every pattern above, replayed against what Python's `re` did with the
    //! same inputs (`conformance/pycompat/policy_regex.json`, written by
    //! `conformance/pycompat/generate_policy.py`).

    use super::*;
    use serde_json::{json, Value};

    fn golden() -> Value {
        let path = format!(
            "{}/../conformance/pycompat/policy_regex.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let value: Value = serde_json::from_str(&text).unwrap();
        let minor: Vec<&str> = crate::pycompat::tables::PYTHON_VERSION
            .split('.')
            .take(2)
            .collect();
        assert_eq!(
            value["python"],
            minor.join("."),
            "the golden and the generated tables are from different Pythons"
        );
        value
    }

    /// A byte offset as Python's code-point offset.
    fn chars(s: &str, byte: usize) -> usize {
        s[..byte].chars().count()
    }

    fn text(value: &Value) -> &str {
        value
            .as_str()
            .unwrap_or_else(|| panic!("not a string: {value}"))
    }

    /// The port of a named pattern, for the golden's parameters.
    fn port(name: &str, params: &Value) -> Option<Regex> {
        let regex = match name {
            "handle" => HANDLE.clone(),
            "repository" => REPOSITORY.clone(),
            "branch_syntax" => BRANCH_SYNTAX.clone(),
            "area_id" => AREA_ID.clone(),
            "prefix" => PREFIX.clone(),
            "head_sha" => HEAD_SHA.clone(),
            "risk_block" => RISK_BLOCK.clone(),
            "bookkeeping" => BOOKKEEPING.clone(),
            "checkbox_item" => CHECKBOX_ITEM.clone(),
            "checkbox" => CHECKBOX.clone(),
            "passed" => PASSED.clone(),
            "separator" => SEPARATOR.clone(),
            "table_row" => TABLE_ROW.clone(),
            "table" => TABLE.clone(),
            "spaces" => SPACES.clone(),
            "attestation" => ATTESTATION.clone(),
            "hidden_markup" => HIDDEN_MARKUP.clone(),
            "pasta_heading" => PASTA_HEADING.clone(),
            "rabbit_heading" => RABBIT_HEADING.clone(),
            "receipt_coverage" => RECEIPT_COVERAGE.clone(),
            "receipt_tail" => RECEIPT_TAIL.clone(),
            "section_alone" | "section_whole" => {
                let index = params["section"].as_u64().unwrap() as usize;
                let section = &SECTIONS[index];
                if name == "section_alone" {
                    section.alone.clone()
                } else {
                    section.whole.clone()
                }
            }
            "branch_target" => return Some(branch_target(text(&params["target"])).unwrap()),
            "final_phase" => {
                let heads: Vec<String> = params["heads"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|h| text(h).to_owned())
                    .collect();
                return final_phase(&heads).unwrap();
            }
            other => panic!("no port of a pattern named {other}"),
        };
        Some(regex)
    }

    fn groups(re: &Regex, caps: &regex::Captures<'_>) -> Value {
        Value::Array(
            (1..re.captures_len())
                .map(|i| json!(caps.get(i).map(|m| m.as_str())))
                .collect(),
        )
    }

    /// What the method made of `s`, in the golden's shape.
    fn answer(method: &str, re: Option<&Regex>, s: &str) -> Value {
        let Some(re) = re else {
            return match method {
                "finditer" => json!([]),
                "count" => json!(0),
                "sub" => json!(s),
                _ => Value::Null,
            };
        };
        match method {
            "finditer" => Value::Array(
                re.captures_iter(s)
                    .map(|caps| {
                        let whole = caps.get(0).unwrap();
                        json!([
                            [chars(s, whole.start()), chars(s, whole.end())],
                            groups(re, &caps)
                        ])
                    })
                    .collect(),
            ),
            "count" => json!(re.find_iter(s).count()),
            "sub" => json!(re.replace_all(s, regex::NoExpand("")).into_owned()),
            // `search`, `match` and `fullmatch` differ only in their anchors,
            // which the ports carry.
            _ => match re.captures(s) {
                None => Value::Null,
                Some(caps) => {
                    let whole = caps.get(0).unwrap();
                    json!([
                        [chars(s, whole.start()), chars(s, whole.end())],
                        groups(re, &caps)
                    ])
                }
            },
        }
    }

    /// The Python pattern, and its flags among `I`, `M` and `S`, each port
    /// above was made from.
    const PORTED_FROM: [(&str, &str, &str); 21] = [
        (
            "handle",
            r"[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?",
            "",
        ),
        ("repository", r"[\w.-]+/[\w.-]+", ""),
        ("branch_syntax", r"[?\[\]{}\\]|\*\*", ""),
        ("area_id", r"[a-z0-9][a-z0-9-]*", ""),
        ("prefix", r"(?:[A-Za-z0-9_.-]+/)+", ""),
        ("head_sha", r"[0-9a-f]{40}", ""),
        (
            "risk_block",
            r"<!-- final_review_risk_start -->(.*?)<!-- final_review_risk_end -->",
            "S",
        ),
        (
            "bookkeeping",
            r"(?is)<details>\s*<summary>[^A-Za-z<]*(?:run configuration|commits|files selected for processing|recent review info)\s*(?:\(\d+\))?\s*</summary>.*?</details>",
            "IS",
        ),
        (
            "checkbox_item",
            r#"(?m)^[-*]\s*\[[ xX]\]\s*<!--\s*\{"checkboxId"[^>]*-->.*$"#,
            "M",
        ),
        ("checkbox", r#"<!--\s*\{"checkboxId"[^>]*-->"#, ""),
        ("passed", "\u{2705}", ""),
        ("separator", r"(?m)^\|(?:\s*:?-+:?\s*\|)+[ \t]*$\n?", "M"),
        (
            "table_row",
            r"(?m)^(\|(?:\\.|[^|\n\\])*\|((?:\\.|[^|\n\\])*)\|).*$",
            "M",
        ),
        ("table", r"(?m)(?:^\|.*\n?)+", "M"),
        ("spaces", r"\s+", ""),
        (
            "attestation",
            r"/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?",
            "I",
        ),
        ("hidden_markup", r"<!--.*?-->", "S"),
        (
            "pasta_heading",
            r"(?m)^\*\*(?P<label>[^\x00-\x7f][^*:\n]*):",
            "M",
        ),
        (
            "rabbit_heading",
            r"(?m)^_[^_\n]+_(?:[ \t]*\|[ \t]*_[^_\n]+_)+[ \t\r]*$",
            "M",
        ),
        (
            "receipt_coverage",
            r"(?m)^<!-- final_review_risk_coverage:\s*",
            "M",
        ),
        ("receipt_tail", r"\s*-->[ \t]*(?:\r?\n|$)", ""),
    ];

    #[test]
    fn a_head_outside_ascii_names_no_final_phase() {
        // Python folds such a head with its own Unicode case data, which
        // the port does not have: it matches nothing instead, so the bot
        // reads as not having reported. The engine's records name only
        // hexadecimal commits.
        let body = "<!-- thepastaclaw-review-phase v1 phase=final sha=\u{e9} -->";
        assert!(final_phase(&["\u{e9}".to_owned()]).unwrap().is_none());
        let mixed = final_phase(&["\u{e9}".to_owned(), "ab".to_owned()])
            .unwrap()
            .unwrap();
        assert!(!mixed.is_match(body));
        assert!(mixed.is_match("<!-- thepastaclaw-review-phase v1 phase=final sha=AB -->"));
    }

    #[test]
    fn every_policy_pattern_matches_what_python_matched() {
        let file = golden();
        let mut failures = Vec::new();
        let mut checked = 0;
        let mut names = std::collections::BTreeSet::new();
        for case in file["cases"].as_array().unwrap() {
            let name = text(&case["name"]);
            names.insert(name.to_owned());
            if let Some(source) = case["source"].as_str() {
                // The pattern policy.py compiles now must be the one this
                // port was made from.
                let ported = PORTED_FROM
                    .iter()
                    .find(|(ported, _, _)| *ported == name)
                    .map(|(_, source, flags)| (*source, *flags));
                let python = (source, text(&case["flags"]));
                assert_eq!(ported, Some(python), "{name}: policy.py's pattern changed");
            }
            let method = text(&case["method"]);
            let re = port(name, &case["params"]);
            for pair in case["results"].as_array().unwrap() {
                let s = text(&pair[0]);
                let ours = answer(method, re.as_ref(), s);
                checked += 1;
                if ours != pair[1] {
                    failures.push(format!(
                        "{name} {} {s:?}:\n  ours   {ours}\n  python {}",
                        case["params"], pair[1]
                    ));
                }
            }
        }
        assert!(checked > 1000, "only {checked} inputs");
        // Every pattern above has a table.
        for name in [
            "handle",
            "repository",
            "branch_syntax",
            "area_id",
            "prefix",
            "head_sha",
            "risk_block",
            "bookkeeping",
            "checkbox_item",
            "checkbox",
            "passed",
            "separator",
            "table_row",
            "table",
            "spaces",
            "attestation",
            "hidden_markup",
            "pasta_heading",
            "rabbit_heading",
            "receipt_coverage",
            "receipt_tail",
            "section_alone",
            "section_whole",
            "branch_target",
            "final_phase",
        ] {
            assert!(names.contains(name), "no table for {name}");
        }
        assert!(
            failures.is_empty(),
            "{} of {checked} differ:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
