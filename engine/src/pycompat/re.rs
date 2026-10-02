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
//!   hand. [`translate`] refuses a bare `$` outside a leading `(?m)`.
//! - **Flags passed as arguments** (`re.S`, `re.M`) go inline, `(?s)`, `(?m)`.
//! - **`fullmatch`** is [`fullmatch_pattern`], `\A(?:…)\z`; **`match`** is
//!   [`match_pattern`], `\A(?:…)`; **`search`** is `find`.
//! - **Patterns that can match the empty string** differ in the regex crate
//!   itself: a repeated group with an alternative that matches empty
//!   (`(?:a*|b)*`) goes on where Python's stops, and `find_iter` skips the
//!   empty match Python's `finditer` and `sub` report right after a
//!   non-empty one (`a*` over `"a"`). Port such a pattern so that it cannot
//!   match empty, or by hand.

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
    #[error("(?{0}) is not translated: port this pattern by hand, a case-insensitive one with explicit sets")]
    UnsupportedFlag(String),
    #[error("Python's $ also matches before a final newline: write \\Z for the very end, or port by hand")]
    Dollar,
    #[error(
        "{0:?} in a character class is a set operation in the regex crate and a literal in Python"
    )]
    SetOperation(String),
    #[error("a class escape at the end of a range is an error in Python")]
    ClassRange,
    #[error("a quantifier after a quantifier is possessive or an error in Python")]
    StackedQuantifier,
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

/// What the last thing written outside a class was, as far as a following
/// quantifier cares.
#[derive(Clone, Copy, PartialEq, Eq)]
enum After {
    Other,
    Quantifier,
    /// A quantifier made lazy by `?`.
    Lazy,
}

/// Inside a character class: how many members so far, and whether a `-`
/// is waiting to join the last one to the next into a range.
struct Class {
    members: usize,
    range_open: bool,
}

/// A Python `str` pattern as a `regex` pattern that matches the same text:
/// `\s`, `\w`, `\d` and their negations, inside character classes or out,
/// become the classes Python uses; `\Z` becomes `\z`; a `]` that opens a
/// class, a `[` inside one and a `{` that Python reads as a literal are
/// escaped, and `{,n}` becomes `{0,n}`. Everything else is passed through,
/// so a construct the regex crate lacks (a backreference, a lookaround)
/// fails when it is compiled.
pub fn translate(pattern: &str) -> Result<String, TranslateError> {
    let multiline = global_flags(pattern).contains('m');
    let mut out = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();
    let mut class: Option<Class> = None;
    let mut after = After::Other;
    while let Some(c) = chars.next() {
        if let Some(state) = class.as_mut() {
            let was_range = std::mem::take(&mut state.range_open);
            match c {
                '\\' => {
                    let escape = chars.next().ok_or(TranslateError::TrailingBackslash)?;
                    match escape {
                        's' | 'w' | 'd' | 'S' | 'W' | 'D' => {
                            // Python refuses a class escape at either end
                            // of a range; the regex crate would read the
                            // `-` as a literal and match more.
                            let mut ahead = chars.clone();
                            let starts_range =
                                ahead.next() == Some('-') && ahead.next().is_some_and(|n| n != ']');
                            if was_range || starts_range {
                                return Err(TranslateError::ClassRange);
                            }
                            if escape.is_ascii_lowercase() {
                                out.push_str(class_body(escape));
                            } else {
                                // A nested class is a union member.
                                out.push_str("[^");
                                out.push_str(class_body(escape.to_ascii_lowercase()));
                                out.push(']');
                            }
                        }
                        'b' => out.push_str(r"\x08"),
                        _ => {
                            out.push('\\');
                            out.push(escape);
                        }
                    }
                    state.members += 1;
                }
                ']' => {
                    class = None;
                    out.push(']');
                }
                '&' | '-' | '~' if chars.peek() == Some(&c) => {
                    return Err(TranslateError::SetOperation(format!("{c}{c}")));
                }
                '-' => {
                    // First or last it is a literal; between two members it
                    // makes a range of them.
                    if state.members > 0 && chars.peek() != Some(&']') {
                        state.range_open = true;
                    } else {
                        state.members += 1;
                    }
                    out.push('-');
                }
                '[' => {
                    out.push_str(r"\[");
                    state.members += 1;
                }
                _ => {
                    out.push(c);
                    state.members += 1;
                }
            }
            continue;
        }
        let before = std::mem::replace(&mut after, After::Other);
        match c {
            '\\' => {
                let escape = chars.next().ok_or(TranslateError::TrailingBackslash)?;
                match escape {
                    's' | 'w' | 'd' => {
                        out.push('[');
                        out.push_str(class_body(escape));
                        out.push(']');
                    }
                    'S' | 'W' | 'D' => {
                        out.push_str("[^");
                        out.push_str(class_body(escape.to_ascii_lowercase()));
                        out.push(']');
                    }
                    'b' | 'B' => return Err(TranslateError::WordBoundary),
                    'Z' => out.push_str(r"\z"),
                    _ => {
                        out.push('\\');
                        out.push(escape);
                    }
                }
            }
            '[' => {
                let mut state = Class {
                    members: 0,
                    range_open: false,
                };
                out.push('[');
                if chars.peek() == Some(&'^') {
                    chars.next();
                    out.push('^');
                }
                if chars.peek() == Some(&']') {
                    chars.next();
                    out.push_str(r"\]");
                    state.members = 1;
                }
                class = Some(state);
            }
            '*' | '+' | '?' => {
                after = quantify(before, c == '?')?;
                out.push(c);
            }
            '{' => match python_repetition(&mut chars) {
                Some(repetition) => {
                    after = quantify(before, false)?;
                    out.push_str(&repetition);
                }
                None => out.push_str(r"\{"),
            },
            '}' => out.push_str(r"\}"),
            '$' if !multiline => return Err(TranslateError::Dollar),
            '(' if chars.peek() == Some(&'?') => {
                chars.next();
                out.push_str("(?");
                check_group_flags(chars.clone())?;
            }
            _ => out.push(c),
        }
    }
    if class.is_some() {
        return Err(TranslateError::UnterminatedClass);
    }
    Ok(out)
}

/// What a quantifier makes of the thing before it: one `?` after a
/// quantifier makes it lazy; anything else after one is possessive (`*+`)
/// or an error in Python.
fn quantify(before: After, question: bool) -> Result<After, TranslateError> {
    match before {
        After::Other => Ok(After::Quantifier),
        After::Quantifier if question => Ok(After::Lazy),
        _ => Err(TranslateError::StackedQuantifier),
    }
}

/// After a `{`: Python's repetition `{m}`, `{m,}`, `{,n}` or `{m,n}`, with
/// ASCII digits and nothing else, consumed and written as the regex crate
/// spells it; `None`, consuming nothing, where Python reads the `{` as a
/// literal (`{}`, `{1, 3}`, `{x}`).
fn python_repetition(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Option<String> {
    let mut ahead = chars.clone();
    let mut low = String::new();
    let mut high: Option<String> = None;
    loop {
        match ahead.next()? {
            d @ '0'..='9' => match high.as_mut() {
                None => low.push(d),
                Some(digits) => digits.push(d),
            },
            ',' if high.is_none() => high = Some(String::new()),
            '}' => break,
            _ => return None,
        }
    }
    let repetition = match high {
        None if low.is_empty() => return None,
        None => format!("{{{low}}}"),
        Some(high) if low.is_empty() => format!("{{0,{high}}}"),
        Some(high) => format!("{{{low},{high}}}"),
    };
    *chars = ahead;
    Some(repetition)
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

/// Refuses inline flags whose meaning the translation would change: `i`
/// (case-insensitive sets come from Python), `a` (ASCII classes), `x`
/// (verbose mode treats whitespace in classes differently), `L`, and `-m`
/// (`$` is translated on the strength of a leading `(?m)`). `after` starts
/// just past `(?`.
fn check_group_flags(after: impl Iterator<Item = char>) -> Result<(), TranslateError> {
    let mut off = false;
    for c in after {
        match c {
            '-' if !off => off = true,
            'm' if off => return Err(TranslateError::UnsupportedFlag("-m".into())),
            'i' | 'a' | 'x' | 'L' if !off => {
                return Err(TranslateError::UnsupportedFlag(c.into()));
            }
            'a' | 'i' | 'L' | 'm' | 's' | 'u' | 'x' => {}
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
        let flag = |f: &str| Err(TranslateError::UnsupportedFlag(f.into()));
        assert_eq!(translate(r"\bword"), Err(TranslateError::WordBoundary));
        assert_eq!(translate("(?i)self"), flag("i"));
        assert_eq!(translate("(?si)x"), flag("i"));
        assert_eq!(translate("a(?i:b)"), flag("i"));
        assert_eq!(translate("(?x)a b"), flag("x"));
        // Under a leading (?m), a group that turns it off would bring back
        // Python's `$` before a final newline.
        assert_eq!(translate("(?m)x(?-m:a$)"), flag("-m"));
        assert!(translate("(?s)a(?-s:.)").is_ok());
        assert!(translate("(?-i:a)").is_ok());
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
    fn a_class_escape_cannot_end_a_range() {
        // Python refuses these patterns; the regex crate would read the
        // `-` as one more member and accept them.
        for pattern in [r"[\w-x]", r"[\d-z]", r"[a-\d]", r"[\W-a]", r"[a-\S]"] {
            assert_eq!(
                translate(pattern),
                Err(TranslateError::ClassRange),
                "{pattern}"
            );
        }
        // A `-` at either end of the class is a literal beside an escape.
        for pattern in [r"[\s-]", r"[-\s]", r"[^-\w]", r"[\w.-]", r"[a-z\d]"] {
            assert!(translate(pattern).is_ok(), "{pattern}");
        }
    }

    #[test]
    fn quantifiers_stack_only_as_python_allows() {
        // Python 3.11 reads `*+` as possessive, which the regex crate would
        // read as a repetition of a repetition.
        for pattern in ["a*+", "a++", "a?+", "a{2}+", "a**", "a*??", "a{2}{3}"] {
            assert_eq!(
                translate(pattern),
                Err(TranslateError::StackedQuantifier),
                "{pattern}"
            );
        }
        for pattern in ["a*?", "a+?", "a??", "a{2,3}?", r"a\++", "(?:a)+", "[+]+"] {
            assert!(translate(pattern).is_ok(), "{pattern}");
        }
    }

    #[test]
    fn braces_follow_pythons_repetition_grammar() {
        // Only digits and one comma make a repetition; anything else,
        // whitespace included, leaves the brace a literal in Python.
        assert_eq!(translate("a{2}").unwrap(), "a{2}");
        assert_eq!(translate("a{2,}").unwrap(), "a{2,}");
        assert_eq!(translate("a{,3}").unwrap(), "a{0,3}");
        assert_eq!(translate("a{,}").unwrap(), "a{0,}");
        assert_eq!(translate("a{1, 3}").unwrap(), r"a\{1, 3\}");
        assert_eq!(translate("a{}").unwrap(), r"a\{\}");
        assert_eq!(translate("a{").unwrap(), r"a\{");
        assert_eq!(translate("x{ 0 }y").unwrap(), r"x\{ 0 \}y");
        assert_eq!(translate(r"\{[^\r\n]*\}").unwrap(), r"\{[^\r\n]*\}");
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
