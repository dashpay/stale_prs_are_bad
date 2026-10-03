//! What `gh api` does to the JSON it prints. Python's engine reads GitHub
//! through `gh`, and so reads what gh prints; the Rust engine reads GitHub
//! itself. The two differ wherever an answer holds a control character.
//!
//! `gh api` runs every answer through go-gh's `asciisanitizer` in its JSON
//! mode before printing it, so that a terminal does not act on what an
//! answer holds:
//!
//! - every C0 control character but tab, newline, vertical tab and
//!   carriage return, and every C1 control character, raw or as a `\u00XX`
//!   escape, is printed in caret notation: escape (U+001B) as `^[`, U+009B
//!   as `^[` too;
//! - the text `\u00XX` of such a character after a backslash is replaced
//!   the same way, though in JSON it is no control character but a
//!   backslash and five letters and digits: source code quoting `"\u001b"`
//!   in a patch is printed quoting `"\^[`.
//!
//! Python's engine decides on what gh printed: a comment, a title, and
//! anything computed from them, such as the digest of a patch. This is a
//! deliberate divergence. The service reads GitHub itself and must not
//! rewrite what people wrote; at cut-over Python's engine, and gh with it,
//! is gone. The live comparison renders the answers the Rust engine got as
//! gh would have printed them ([`gh_printed`]) and decides again from
//! those: a difference that then vanishes is gh's, not the port's.

/// The caret notation gh prints for control code point `code`, as JSON
/// text, or `None` for one it leaves alone. File separator's is `^\`, which
/// gh writes with its backslash escaped, `^\\`, so that its output stays
/// valid JSON.
fn caret(code: u32) -> Option<String> {
    let c0 = match code {
        0x09 | 0x0a | 0x0b | 0x0d => return None,
        0x00..=0x1f => code,
        0x80..=0x9f => code - 0x80,
        _ => return None,
    };
    char::from_u32(c0 + 0x40).map(|c| match c {
        '\\' => r"^\\".to_owned(),
        _ => format!("^{c}"),
    })
}

/// The JSON text of one answer as `gh api` prints it: go-gh's sanitizer
/// in its JSON mode, character by character.
///
/// - A control character it maps, as itself, becomes its caret notation.
/// - A backslash followed by `u00XX` (`u` in lower case, the two digits in
///   either), where U+00XX is a control character it maps, becomes the
///   caret notation; where the backslashes before it are odd in number,
///   so that the six characters are the text of an escaped backslash and
///   five letters and digits, a backslash is put before the caret notation
///   to keep the JSON valid, which reads back as a backslash and the caret
///   notation.
/// - Anything else is copied.
pub fn gh_printed(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    // Whether the backslashes just copied are odd in number.
    let mut escaping = false;
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if let Some(repl) = caret(u32::from(c)) {
            out.push_str(&repl);
            at += 1;
            continue;
        }
        if c == '\\' {
            let code = match chars.get(at + 1..at + 6) {
                Some(['u', '0', '0', high, low])
                    if high.is_ascii_hexdigit() && low.is_ascii_hexdigit() =>
                {
                    u32::from_str_radix(&format!("{high}{low}"), 16).ok()
                }
                _ => None,
            };
            if let Some(repl) = code.and_then(caret) {
                if escaping {
                    out.push('\\');
                    escaping = false;
                }
                out.push_str(&repl);
                at += 6;
                continue;
            }
        }
        out.push(c);
        escaping = c == '\\' && !escaping;
        at += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pycompat::{py_loads, PyValue};

    /// The string a JSON answer holds, once gh has printed it.
    fn read_back(json: &str) -> String {
        match py_loads(&gh_printed(json)).unwrap() {
            PyValue::Str(text) => text,
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn control_characters_are_printed_in_caret_notation_as_gh_prints_them() {
        assert_eq!(read_back(r#""a\u001b[1mb\u001B[0m""#), "a^[[1mb^[[0m");
        assert_eq!(
            read_back(r#""\u0000\u0007\u0008\u000c\u001c\u001f""#),
            "^@^G^H^L^\\^_"
        );
        // Tab, newline, vertical tab and carriage return are left alone,
        // and so is DEL, which gh's tables do not hold.
        assert_eq!(
            read_back(r#""a\tb\nc\u000bd\re\u007f\u0009""#),
            "a\tb\nc\u{b}d\re\u{7f}\t"
        );
        // Every C1 control, raw or escaped, takes its C0 twin's.
        assert_eq!(read_back("\"\u{85}\u{9b}\""), "^E^[");
        assert_eq!(read_back(r#""\u0089\u008a\u008d\u009f""#), "^I^J^M^_");
        // Anything else is itself, other escapes included.
        assert_eq!(
            read_back(r#""naïve — ✓ \u2028 \u00a0 \ud83d\ude00 \/ \"""#),
            "naïve — ✓ \u{2028} \u{a0} 😀 / \""
        );
        // Between tokens, where only whitespace can stand, nothing changes.
        assert_eq!(
            gh_printed("[1,\n\t{\"a\": 2}\r\n]"),
            "[1,\n\t{\"a\": 2}\r\n]"
        );
    }

    #[test]
    fn the_text_of_a_control_character_after_a_backslash_is_printed_as_its_caret() {
        // Source code quoting `"\u001b"`, as JSON carries it: `\\u001b`.
        // gh keeps its output valid JSON: it reads back as `\^[`.
        assert_eq!(
            read_back(r#""let esc = \"\\u001b\";""#),
            r#"let esc = "\^[";"#
        );
        assert_eq!(read_back(r#""\\u001B""#), r"\^[", "digits in either case");
        // An escaped backslash, then a real escape character.
        assert_eq!(read_back(r#""\\\u001b""#), r"\^[");
        // Two escaped backslashes, then the text: the second one's.
        assert_eq!(read_back(r#""\\\\u001b""#), r"\\^[");
        assert_eq!(
            read_back(r#""\\U001b""#),
            r"\U001b",
            "the u in lower case only"
        );
        assert_eq!(
            read_back(r#""\\u0009 \\u007f \\u0041""#),
            r"\u0009 \u007f \u0041"
        );
        assert_eq!(read_back(r#""\\u00""#), r"\u00", "too short to be one");
        // A quote between does not carry the backslash over.
        assert_eq!(read_back(r#""\\\"\u001b""#), "\\\"^[");
    }

    #[test]
    fn a_whole_answer_keeps_its_structure() {
        let answer = r#"{"files": [{"patch": "+\"\\u001b[1m\"", "sha": "abc"}], "n": 2.5}"#;
        let printed = py_loads(&gh_printed(answer)).unwrap();
        let PyValue::Dict(fields) = printed else {
            panic!("an object")
        };
        assert!(fields.contains_key("n"));
        let Some(PyValue::List(files)) = fields.get("files") else {
            panic!("a list")
        };
        let Some(PyValue::Dict(file)) = files.iter().next() else {
            panic!("a file")
        };
        assert!(
            matches!(file.get("patch"), Some(PyValue::Str(patch)) if patch == r#"+"\^[[1m""#),
            "{file:?}"
        );
    }
}
