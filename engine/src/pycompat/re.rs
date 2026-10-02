//! Python's `re` patterns, ported to the `regex` crate.
//!
//! For `str` patterns, Python's `\s`, `\w` and `\d` are Unicode classes
//! defined by Python's own Unicode 15.0 data, and not as the regex crate
//! defines them: Python's `\s` holds `\x1c`–`\x1f`, its `\w` holds `²` and
//! `½` but not combining marks or joiners, and its `(?i)` matches `İ` and
//! `ı` with `i`. [`translate`] rewrites the class escapes to the sets read
//! from Python ([`super::tables`]); the regex crate is built without its
//! Unicode tables, so a `\w`, `\s`, `\d`, `\b` or `(?i)` that slips through
//! fails to compile instead of matching differently.
//!
//! What [`translate`] does not do, the port does by hand:
//!
//! - **`(?i)`**: every letter becomes its case-insensitive set
//!   ([`fold_literal`], [`tables::RE_FOLD`]) and every class its enumerated
//!   equivalent (such as [`tables::RE_IGNORECASE_HEX`]). For example, the
//!   attestation `/self[- ]?review(?:ed)?(?:\s+(?P<head>[0-9a-fA-F]{40}))?`
//!   under `re.IGNORECASE` is
//!   `/` + `fold_literal("self")` + `[- ]?` + `fold_literal("review")` +
//!   `(?:` + `fold_literal("ed")` + `)?(?:[` + `RE_SPACE` + `]+(?P<head>` +
//!   `RE_IGNORECASE_HEX` + `{40}))?`, compiled without `(?i)`.
//! - **`$` without `(?m)`** also matches before a final `\n` in Python, which
//!   the regex crate has no way to say. Where only the very end is meant,
//!   write Python's `\Z` (translated to `\z`); otherwise port the line by
//!   hand. [`translate`] refuses a bare `$` outside `(?m)`.
//! - **Flags passed as arguments** (`re.S`, `re.M`) go inline, `(?s)`, `(?m)`.
//! - **`fullmatch`** is [`fullmatch_pattern`], `\A(?:…)\z`; **`match`** is
//!   [`match_pattern`], `\A(?:…)`; **`search`** is `find`.

use super::tables;

/// Why a Python pattern cannot be translated as it stands.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TranslateError {
    #[error("pattern ends in a lone backslash")]
    TrailingBackslash,
    #[error("unterminated character class")]
    UnterminatedClass,
    #[error("\\b and \\B depend on Python's \\w, which the regex crate cannot be given")]
    WordBoundary,
    #[error("(?{0}) cannot be translated: port case-insensitive patterns with explicit sets")]
    UnsupportedFlag(char),
    #[error("Python's $ also matches before a final newline: write \\Z for the very end, or port by hand")]
    Dollar,
    #[error(
        "{0:?} in a character class is a set operation in the regex crate and a literal in Python"
    )]
    SetOperation(String),
    #[error("{0:?} has no case-insensitive set: only ASCII letters have one")]
    NoFoldSet(char),
}

fn class_body(escape: char) -> &'static str {
    match escape {
        's' => tables::RE_SPACE,
        'w' => tables::RE_WORD,
        _ => tables::RE_DIGIT,
    }
}

/// A Python `str` pattern as a `regex` pattern that matches the same text:
/// `\s`, `\w`, `\d` and their negations, inside character classes or out,
/// become the classes Python uses; `\Z` becomes `\z`; a `]` that opens a
/// class and a `[` inside one are escaped. Everything else is passed
/// through, so a construct the regex crate lacks (a backreference, a
/// lookaround) fails when it is compiled.
pub fn translate(pattern: &str) -> Result<String, TranslateError> {
    let multiline = global_flags(pattern).contains('m');
    let mut out = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();
    let mut in_class = false;
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let escape = chars.next().ok_or(TranslateError::TrailingBackslash)?;
                match escape {
                    's' | 'w' | 'd' if in_class => out.push_str(class_body(escape)),
                    's' | 'w' | 'd' => {
                        out.push('[');
                        out.push_str(class_body(escape));
                        out.push(']');
                    }
                    'S' | 'W' | 'D' => {
                        // Nested inside a class, this is a union member.
                        out.push_str("[^");
                        out.push_str(class_body(escape.to_ascii_lowercase()));
                        out.push(']');
                    }
                    'b' if in_class => out.push_str(r"\x08"),
                    'b' | 'B' => return Err(TranslateError::WordBoundary),
                    'Z' if !in_class => out.push_str(r"\z"),
                    _ => {
                        out.push('\\');
                        out.push(escape);
                    }
                }
            }
            '[' if in_class => out.push_str(r"\["),
            '[' => {
                in_class = true;
                out.push('[');
                if chars.peek() == Some(&'^') {
                    chars.next();
                    out.push('^');
                }
                if chars.peek() == Some(&']') {
                    chars.next();
                    out.push_str(r"\]");
                }
            }
            ']' if in_class => {
                in_class = false;
                out.push(']');
            }
            '&' | '-' | '~' if in_class && chars.peek() == Some(&c) => {
                return Err(TranslateError::SetOperation(format!("{c}{c}")));
            }
            '$' if !in_class && !multiline => return Err(TranslateError::Dollar),
            '(' if !in_class && chars.peek() == Some(&'?') => {
                out.push('(');
                check_group_flags(chars.clone().skip(1))?;
            }
            _ => out.push(c),
        }
    }
    if in_class {
        return Err(TranslateError::UnterminatedClass);
    }
    Ok(out)
}

/// The flags of a leading `(?flags)` group, where Python takes global flags.
fn global_flags(pattern: &str) -> &str {
    let Some(rest) = pattern.strip_prefix("(?") else {
        return "";
    };
    let flags = &rest[..rest.len()
        - rest
            .trim_start_matches(|c: char| c.is_ascii_alphabetic())
            .len()];
    if rest[flags.len()..].starts_with(')') {
        flags
    } else {
        ""
    }
}

/// Refuses inline flags whose meaning the translation would change:
/// `i` (case-insensitive sets come from Python), `a` (ASCII classes), `x`
/// (verbose mode treats whitespace in classes differently) and `L`.
fn check_group_flags(after: impl Iterator<Item = char>) -> Result<(), TranslateError> {
    for c in after {
        match c {
            'i' | 'a' | 'x' | 'L' => return Err(TranslateError::UnsupportedFlag(c)),
            's' | 'm' | 'u' => {}
            _ => return Ok(()),
        }
    }
    Ok(())
}

/// A literal under `re.IGNORECASE`: each ASCII letter becomes the set it
/// matches there (`i` matches `İ` and `ı`, `k` matches `K`, `s` matches
/// `ſ`), anything else without case is escaped. A non-ASCII letter has no
/// set read from Python, so it is refused.
pub fn fold_literal(text: &str) -> Result<String, TranslateError> {
    let mut out = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            out.push_str(tables::RE_FOLD[usize::from(c.to_ascii_lowercase() as u8 - b'a')]);
        } else if c.is_ascii() {
            out.push_str(&regex::escape(c.encode_utf8(&mut [0; 4])));
        } else {
            return Err(TranslateError::NoFoldSet(c));
        }
    }
    Ok(out)
}

/// The pattern for Python's `fullmatch`: the whole text, nothing more.
pub fn fullmatch_pattern(pattern: &str) -> String {
    format!(r"\A(?:{pattern})\z")
}

/// The pattern for Python's `match`: anchored at the start only.
pub fn match_pattern(pattern: &str) -> String {
    format!(r"\A(?:{pattern})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_escapes_become_pythons_sets_inside_and_outside_classes() {
        let space = format!("[{}]", tables::RE_SPACE);
        assert_eq!(translate(r"\s+").unwrap(), format!("{space}+"));
        assert_eq!(
            translate(r"\S").unwrap(),
            format!("[^{}]", tables::RE_SPACE)
        );
        assert_eq!(
            translate(r"[\w.-]+").unwrap(),
            format!("[{}.-]+", tables::RE_WORD)
        );
        assert_eq!(
            translate(r"[^\D<]").unwrap(),
            format!("[^[^{}]<]", tables::RE_DIGIT)
        );
        // An escaped backslash before `s` is a literal backslash and an `s`.
        assert_eq!(translate(r"\\s").unwrap(), r"\\s");
    }

    #[test]
    fn constructs_that_would_match_differently_are_refused() {
        assert_eq!(translate(r"\bword"), Err(TranslateError::WordBoundary));
        assert_eq!(
            translate("(?i)self"),
            Err(TranslateError::UnsupportedFlag('i'))
        );
        assert_eq!(
            translate("(?si)x"),
            Err(TranslateError::UnsupportedFlag('i'))
        );
        assert_eq!(
            translate("a(?i:b)"),
            Err(TranslateError::UnsupportedFlag('i'))
        );
        assert_eq!(translate("a$"), Err(TranslateError::Dollar));
        assert!(translate("(?m)^a$").is_ok());
        assert_eq!(
            translate("[a--b]"),
            Err(TranslateError::SetOperation("--".into()))
        );
        assert_eq!(translate("[ab"), Err(TranslateError::UnterminatedClass));
        assert_eq!(translate(r"a\"), Err(TranslateError::TrailingBackslash));
        // `$` inside a class is a literal dollar in both.
        assert!(translate("[$]").is_ok());
    }

    #[test]
    fn python_literal_brackets_stay_literal() {
        // Python reads `]` first in a class, and `[` inside one, as
        // literals; the regex crate would read a nested class.
        assert_eq!(translate("[]a]").unwrap(), r"[\]a]");
        assert_eq!(translate("[[a]").unwrap(), r"[\[a]");
        assert_eq!(translate("[^]a]").unwrap(), r"[^\]a]");
    }

    #[test]
    fn the_regex_crate_cannot_fall_back_on_its_own_unicode_classes() {
        // Built without its Unicode tables, an untranslated class or
        // case-insensitive pattern is a compile error rather than a
        // different set. A dependency that turned those tables back on
        // would make these compile.
        for pattern in [r"\w", r"\s", r"\d", r"\b", "(?i)k"] {
            assert!(
                regex::Regex::new(pattern).is_err(),
                "{pattern} compiled with the regex crate's own Unicode data"
            );
        }
    }

    #[test]
    fn fold_literal_uses_pythons_sets() {
        assert_eq!(
            fold_literal("ki-").unwrap(),
            format!(r"{}{}\-", tables::RE_FOLD[10], tables::RE_FOLD[8])
        );
        assert_eq!(fold_literal("é"), Err(TranslateError::NoFoldSet('é')));
    }
}
