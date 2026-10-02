//! The one part of a verdict another engine is not held to: a blocker
//! carrying the text of a Python exception.
//!
//! `evaluate` turns a `ValueError`, `TypeError` or `KeyError` into a
//! configuration error whose first reason is the exception's text. Where
//! that text is the engine's own — a reason `evaluate` stops with, or a
//! message `policy.py` raises — the port must give it word for word. Where
//! it is Python's — a missing key, a timestamp `fromisoformat` refused — the
//! port is held only to having raised in the same place.
//!
//! Which is which is decided as `pr_review/conformance.py` decides it, from
//! the engine's own source: its `_OWN_BLOCKERS` and every message
//! `policy.py` raises. [`OwnWords`] reads both from their text.

use crate::pycompat::PyValue;
use regex::Regex;
use std::sync::LazyLock;

/// What a waiver adds to the blockers, which is never the reason.
const WAIVER_NOTE: &str = "Proceeded without";

/// Why the engine's own words could not be read from its source.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SourceError {
    #[error("conformance.py no longer defines _OWN_BLOCKERS as a tuple of literals")]
    NoOwnBlockers,
    #[error("policy.py raises no message this reader recognises")]
    NoRaisedMessages,
}

/// The engine's own words: the reasons `evaluate` stops with, and what
/// `policy.py` raises.
#[derive(Debug, Clone)]
pub struct OwnWords {
    /// Each reason in `_OWN_BLOCKERS`, matched as a prefix.
    blockers: Vec<String>,
    /// Each message `policy.py` raises: its constant text, and whether that
    /// text is the whole message (a literal alone) or only its start (an
    /// f-string, or a literal with something added).
    raised: Vec<(String, bool)>,
}

/// A single-quoted literal, as `_OWN_BLOCKERS` lists them.
static LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"'([^']*)'").expect("a constant pattern compiles"));

/// `raise Class(` and its first argument, when that starts with a string:
/// an f-string, whose constant start runs to its first `{`, or a plain
/// literal, which is the whole message when nothing is added to it (a
/// closing parenthesis or another argument follows it). Each pattern says
/// whether it is the literal form. (The crate's regex has no Unicode
/// tables, so whitespace is spelled out.)
static RAISED: LazyLock<[(Regex, bool); 4]> = LazyLock::new(|| {
    [
        (
            r#"raise [A-Za-z_][A-Za-z0-9_]*\([ \t\r\n]*f'([^'{]*)"#,
            false,
        ),
        (
            r#"raise [A-Za-z_][A-Za-z0-9_]*\([ \t\r\n]*f"([^"{]*)"#,
            false,
        ),
        (
            r#"raise [A-Za-z_][A-Za-z0-9_]*\([ \t\r\n]*'([^']*)'([ \t\r\n]*[,)])?"#,
            true,
        ),
        (
            r#"raise [A-Za-z_][A-Za-z0-9_]*\([ \t\r\n]*"([^"]*)"([ \t\r\n]*[,)])?"#,
            true,
        ),
    ]
    .map(|(pattern, literal)| {
        let compiled = Regex::new(pattern).expect("a constant pattern compiles");
        (compiled, literal)
    })
});

impl OwnWords {
    /// The engine's own words, from the text of `pr_review/conformance.py`
    /// and of `pr_review/policy.py`.
    pub fn from_sources(conformance_py: &str, policy_py: &str) -> Result<Self, SourceError> {
        let start = conformance_py
            .find("_OWN_BLOCKERS = (")
            .ok_or(SourceError::NoOwnBlockers)?;
        let end = conformance_py[start..]
            .find(")\n")
            .map(|end| start + end)
            .ok_or(SourceError::NoOwnBlockers)?;
        let blockers: Vec<String> = LITERAL
            .captures_iter(&conformance_py[start..end])
            .map(|found| found[1].to_owned())
            .filter(|text| !text.is_empty())
            .collect();
        if blockers.is_empty() {
            return Err(SourceError::NoOwnBlockers);
        }
        let mut raised = Vec::new();
        for (pattern, literal) in RAISED.iter() {
            for found in pattern.captures_iter(policy_py) {
                if found[1].is_empty() {
                    continue;
                }
                let whole = *literal && found.get(2).is_some();
                raised.push((found[1].to_owned(), whole));
            }
        }
        if raised.is_empty() {
            return Err(SourceError::NoRaisedMessages);
        }
        Ok(OwnWords { blockers, raised })
    }

    /// How many words were read: the reasons, and the raised messages.
    pub fn counts(&self) -> (usize, usize) {
        (self.blockers.len(), self.raised.len())
    }

    /// Whether `text` starts with any of the engine's own words. The check
    /// a port's own exception text must fail: a port that stopped with one
    /// of the engine's reasons where Python raised is not raising.
    pub fn starts_with_own(&self, text: &str) -> bool {
        self.blockers
            .iter()
            .chain(self.raised.iter().map(|(text, _)| text))
            .any(|own| text.starts_with(own.as_str()))
    }

    /// `conformance.python_exception_text(result)`: whether a verdict's
    /// first reason is the text of a Python exception rather than the
    /// engine's own words.
    pub fn python_exception_text(&self, result: &PyValue) -> bool {
        let PyValue::Dict(result) = result else {
            return false;
        };
        let is = |key: &str, wanted: &str| matches!(result.get(key), Some(PyValue::Str(s)) if s == wanted);
        if !is("state", "configuration-error") || !is("status", "error") {
            return false;
        }
        let Some(PyValue::List(blockers)) = result.get("blockers") else {
            return false;
        };
        // Python's `b.startswith(...)` raises on a blocker that is not a
        // string. No result the engine writes has one, and such a result is
        // left to the exact comparison.
        let mut reasons = Vec::with_capacity(blockers.len());
        for blocker in blockers.iter() {
            match blocker {
                PyValue::Str(text) if text.starts_with(WAIVER_NOTE) => {}
                PyValue::Str(text) => reasons.push(text.as_str()),
                _ => return false,
            }
        }
        let Some(&first) = reasons.first() else {
            return false;
        };
        if self
            .blockers
            .iter()
            .any(|own| first.starts_with(own.as_str()))
        {
            return false;
        }
        !self.raised.iter().any(|(text, whole)| {
            if *whole {
                first == text
            } else {
                first.starts_with(text.as_str())
            }
        })
    }
}

/// Where the port and Python disagree about a reason carrying exception
/// text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionText {
    /// Python's result carries exception text and the port's has no reason
    /// in its place, or the reverse.
    Absent,
    /// The port's reason in its place is one of the engine's own words: it
    /// stopped where Python raised.
    OwnWords,
}

/// The first blocker that is not a waiver's note, taken out of `result`.
fn take_reason(result: &mut PyValue) -> Option<PyValue> {
    let PyValue::Dict(entries) = result else {
        return None;
    };
    let Some(PyValue::List(blockers)) = entries.get_mut("blockers") else {
        return None;
    };
    let at = blockers
        .iter()
        .position(|b| !matches!(b, PyValue::Str(text) if text.starts_with(WAIVER_NOTE)))?;
    Some(blockers.remove(at))
}

/// Take the reason carrying exception text out of both results, where
/// Python's has one, so that what is left is compared exactly: the
/// exclusion the evaluate-case gate applies to a case Python marked
/// `python_exception_text`. Both must have had a reason there, and the
/// port's must not be one of the engine's own words.
pub fn set_aside_exception_text(
    ours: &mut PyValue,
    python: &mut PyValue,
    own: &OwnWords,
) -> Result<(), ExceptionText> {
    let theirs = take_reason(python);
    let mine = take_reason(ours);
    if theirs.is_some() != mine.is_some() {
        return Err(ExceptionText::Absent);
    }
    match mine {
        Some(PyValue::Str(text)) if own.starts_with_own(&text) => Err(ExceptionText::OwnWords),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pycompat::py_loads;

    const CONFORMANCE: &str = "x = 1\n_OWN_BLOCKERS = ('Incomplete GitHub snapshot', 'Invalid head SHA',\n                 'Unresolved identities in ')\ny = ('not', 'these')\n";
    const POLICY: &str = r#"
def f(x):
    raise ValueError('Timestamp requires timezone')
    raise ValueError('required_bots must be a subset of ' + ', '.join(x))
    raise ValueError(f'Missing policy directory: {x}')
    raise TypeError("Expected handle list", x)
"#;

    fn own() -> OwnWords {
        OwnWords::from_sources(CONFORMANCE, POLICY).unwrap()
    }

    fn verdict(blockers: &[&str]) -> PyValue {
        let blockers: Vec<String> = blockers.iter().map(|b| format!("{b:?}")).collect();
        py_loads(&format!(
            r#"{{"state": "configuration-error", "status": "error", "blockers": [{}]}}"#,
            blockers.join(", ")
        ))
        .unwrap()
    }

    #[test]
    fn the_own_words_are_read_from_the_tuple_and_from_every_raise() {
        assert_eq!(own().counts(), (3, 4));
        assert_eq!(
            OwnWords::from_sources("nothing here", POLICY).unwrap_err(),
            SourceError::NoOwnBlockers
        );
        assert_eq!(
            OwnWords::from_sources(CONFORMANCE, "pass").unwrap_err(),
            SourceError::NoRaisedMessages
        );
    }

    #[test]
    fn a_literal_is_matched_whole_and_anything_built_by_its_start() {
        let own = own();
        // The engine's own words: not exception text.
        for reason in [
            "Timestamp requires timezone",
            "required_bots must be a subset of coderabbitai",
            "Missing policy directory: packages/x/",
            "Expected handle list",
            "Unresolved identities in drive",
        ] {
            assert!(!own.python_exception_text(&verdict(&[reason])), "{reason}");
        }
        // A literal raised alone is the whole message: more after it is not
        // the engine's.
        assert!(own.python_exception_text(&verdict(&["Timestamp requires timezone!"])));
        // Python's own text.
        assert!(own.python_exception_text(&verdict(&["'author'"])));
        assert!(own.python_exception_text(&verdict(&["Invalid isoformat string: 'x'"])));
    }

    #[test]
    fn only_a_configuration_error_carries_exception_text_and_a_waiver_note_is_not_it() {
        let own = own();
        assert!(
            own.python_exception_text(&verdict(&["Proceeded without coderabbitai", "'author'"]))
        );
        assert!(!own.python_exception_text(&verdict(&["Proceeded without coderabbitai"])));
        assert!(!own.python_exception_text(&verdict(&[])));
        let pending =
            py_loads(r#"{"state": "waiting-bots", "status": "pending", "blockers": ["'author'"]}"#)
                .unwrap();
        assert!(!own.python_exception_text(&pending));
    }

    #[test]
    fn setting_the_reason_aside_holds_the_port_to_raising_where_python_raised() {
        let own = own();
        let mut python = verdict(&["Proceeded without coderabbitai", "'author'", "later"]);
        let mut ours = verdict(&[
            "Proceeded without coderabbitai",
            "KeyError: author",
            "later",
        ]);
        assert_eq!(
            set_aside_exception_text(&mut ours, &mut python, &own),
            Ok(())
        );
        let rest = |v: &PyValue| match v {
            PyValue::Dict(d) => match d.get("blockers") {
                Some(PyValue::List(b)) => b.len(),
                _ => 0,
            },
            _ => 0,
        };
        assert_eq!((rest(&ours), rest(&python)), (2, 2));

        let mut python = verdict(&["'author'"]);
        let mut ours = verdict(&[]);
        assert_eq!(
            set_aside_exception_text(&mut ours, &mut python, &own),
            Err(ExceptionText::Absent)
        );

        let mut python = verdict(&["'author'"]);
        let mut ours = verdict(&["Invalid head SHA"]);
        assert_eq!(
            set_aside_exception_text(&mut ours, &mut python, &own),
            Err(ExceptionText::OwnWords)
        );
    }
}
