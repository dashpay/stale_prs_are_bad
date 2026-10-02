//! What `gh api` does to the JSON it prints. Python's engine reads GitHub
//! through `gh`, and so reads what it prints; the Rust engine reads GitHub
//! itself. They differ where a string holds a control character.
//!
//! `gh api` runs its output through go-gh's `asciisanitizer`, which keeps
//! a terminal from acting on what an answer holds. In the JSON it prints:
//!
//! - every C0 control character but tab, newline, vertical tab and
//!   carriage return, and every C1 control character, raw or escaped, is
//!   written in caret notation: escape (U+001B) as `^[`, U+009B as `^[`
//!   too;
//! - so is the text `\u00XX` of such a character after a backslash in a
//!   string, which is no control character at all: a comment that quotes
//!   `\u001b` is printed quoting `\^[`.
//!
//! Python's engine therefore decides on what gh printed: a comment, a
//! title, a description with those characters in caret notation. This is
//! a deliberate divergence. The service reads GitHub itself and must not
//! rewrite what people wrote; at cut-over Python's engine, and with it gh,
//! is gone. The live comparison holds a value to Python's under this rule:
//! a difference that vanishes once every string of the Rust engine's value
//! is written as gh prints it ([`printed`]) is gh's, not the port's.

use crate::pycompat::{PyDict, PyList, PyValue};

/// The caret notation gh prints for control code point `code`, or `None`
/// for one it leaves alone.
fn caret(code: u32) -> Option<String> {
    let c0 = match code {
        0x09 | 0x0a | 0x0b | 0x0d => return None,
        0x00..=0x1f => code,
        0x80..=0x9f => code - 0x80,
        _ => return None,
    };
    char::from_u32(c0 + 0x40).map(|c| format!("^{c}"))
}

/// `text` as `gh api` prints it, read back: each control character it
/// sanitizes in caret notation, and each backslash followed by the text
/// `u00XX` of one (`u` in lower case, the digits in either) followed by
/// that character's caret notation in place of the text.
pub fn as_gh_prints(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if let Some(repl) = caret(u32::from(c)) {
            out.push_str(&repl);
            at += 1;
            continue;
        }
        if c == '\\' {
            let quoted: Option<String> =
                chars.get(at + 1..at + 6).map(|tail| tail.iter().collect());
            let code = quoted
                .as_deref()
                .and_then(|tail| tail.strip_prefix("u00"))
                .filter(|hex| hex.chars().all(|h| h.is_ascii_hexdigit()))
                .and_then(|hex| u32::from_str_radix(hex, 16).ok());
            if let Some(repl) = code.and_then(caret) {
                out.push('\\');
                out.push_str(&repl);
                at += 6;
                continue;
            }
        }
        out.push(c);
        at += 1;
    }
    out
}

/// `value` with every string in it, keys too, as `gh api` prints it.
pub fn printed(value: &PyValue) -> PyValue {
    match value {
        PyValue::Str(text) => PyValue::Str(as_gh_prints(text)),
        PyValue::List(items) => {
            PyValue::List(PyList::from(items.iter().map(printed).collect::<Vec<_>>()))
        }
        PyValue::Dict(entries) => {
            let mut out = PyDict::new();
            for (key, item) in entries.iter() {
                out.insert(as_gh_prints(key), printed(item));
            }
            PyValue::Dict(out)
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_characters_are_printed_in_caret_notation_as_gh_prints_them() {
        assert_eq!(as_gh_prints("a\u{1b}[1mb\u{1b}[0m"), "a^[[1mb^[[0m");
        assert_eq!(
            as_gh_prints("\u{0}\u{7}\u{8}\u{c}\u{1c}\u{1f}"),
            "^@^G^H^L^\\^_"
        );
        // Tab, newline, vertical tab and carriage return are left alone,
        // and so is DEL, which gh's tables do not hold.
        assert_eq!(
            as_gh_prints("a\tb\nc\u{b}d\re\u{7f}"),
            "a\tb\nc\u{b}d\re\u{7f}"
        );
        // Every C1 control, those four's included, takes its C0 twin's.
        assert_eq!(
            as_gh_prints("\u{80}\u{85}\u{89}\u{8a}\u{8d}\u{9b}\u{9f}"),
            "^@^E^I^J^M^[^_"
        );
        // Anything else is itself.
        assert_eq!(
            as_gh_prints("naïve — ✓ \u{2028} \u{a0}"),
            "naïve — ✓ \u{2028} \u{a0}"
        );
    }

    #[test]
    fn the_text_of_a_control_character_after_a_backslash_is_printed_as_its_caret() {
        // `\u001b` written out in a comment, as JSON carries it: `\\u001b`.
        // gh keeps its output valid JSON, and so reads back as `\^[`.
        assert_eq!(as_gh_prints(r"quoting \u001b here"), r"quoting \^[ here");
        assert_eq!(as_gh_prints(r"\u001B"), r"\^[", "digits in either case");
        assert_eq!(as_gh_prints(r"\\u001b"), r"\\^[", "after any backslash");
        assert_eq!(
            as_gh_prints(r"\U001b"),
            r"\U001b",
            "the u in lower case only"
        );
        assert_eq!(as_gh_prints(r"\u0009 \u007f A"), r"\u0009 \u007f A");
        assert_eq!(as_gh_prints(r"\u009b"), r"\^[");
        assert_eq!(as_gh_prints(r"\u00"), r"\u00", "too short to be one");
    }

    #[test]
    fn a_value_is_printed_string_by_string() {
        let value = crate::pycompat::py_loads(
            r#"{"body": "\u001b[1m", "list": ["\u0085", 1, null], "n": 2}"#,
        )
        .unwrap();
        let written =
            |value: &PyValue| crate::pycompat::py_dumps(value, false, None, None).unwrap();
        assert_eq!(
            written(&printed(&value)),
            r#"{"body": "^[[1m", "list": ["^E", 1, null], "n": 2}"#
        );
    }
}
