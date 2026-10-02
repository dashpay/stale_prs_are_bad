//! What the engine remembers on a pull request, and how it is read back:
//! the record comment's state and diff markers, and the checklist block in
//! the description.
//!
//! Anyone with write access can edit anyone's comment, and the record holds
//! the author's place in the review queue and whether a head was ever ready
//! for a human; the diff says which commits a review still covers. So a
//! comment counts only if the engine wrote it and nobody but the engine has
//! edited it since, and the newest record decides even when it does not
//! count: an older one is never brought back in its place.

use super::error::{PyClass, ReadError};
use super::py::{self, compiled, text, Read};
use crate::policy::{is_engine, CHECKLIST_END, CHECKLIST_START};
use crate::pycompat::object::{get, getitem, or_default, str_method};
use crate::pycompat::ops::{py_compare, py_contains, py_eq, Compare};
use crate::pycompat::re::fullmatch_pattern;
use crate::pycompat::text::{py_lstrip, py_splitlines};
use crate::pycompat::{py_dumps, py_loads, PyDateTime, PyErr, PyInt, PyValue};
use regex::Regex;
use std::fmt::Write as _;
use std::sync::LazyLock;

/// The record's marker, which opens the record comment.
pub const STATE_MARKER: &str = "<!-- platform-pr-review-state-v1";
/// The diff's marker, on the line after the record's.
///
/// The diff rides beside the record rather than inside it. The record's
/// schema is an exact set of keys, so an engine that predates a new one
/// would refuse a record carrying it and report a configuration error on
/// every pull request in its repository until it is re-pinned; a marker of
/// its own is simply not read by an engine that does not know it.
pub const DIFF_MARKER: &str = "<!-- pr-hygiene-diff-v1";
/// The editor of a comment read from the REST listing, which names none.
/// Not `None`, which says nobody edited it, and nothing a login can spell,
/// so no account is ever taken for it.
pub const EDITOR_UNKNOWN: &str = "(editor unknown)";

/// The record's marker line, its JSON the one group.
pub(crate) static STATE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| compiled(r"<!-- platform-pr-review-state-v1 (\{[^\r\n]*\}) -->"));
static DIFF_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| compiled(r"<!-- pr-hygiene-diff-v1 (\{[^\r\n]*\}) -->"));
/// A whole-second UTC timestamp. `\d` is Python's: any Unicode digit.
static TIMESTAMP: LazyLock<Regex> =
    LazyLock::new(|| compiled(&fullmatch_pattern(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z")));

/// `re.fullmatch(r"[0-9a-f]{length}", text)`.
fn lower_hex(text: &str, length: usize) -> bool {
    text.len() == length && text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// `re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", text)`.
pub(crate) fn utc_timestamp(text: &str) -> bool {
    TIMESTAMP.is_match(text)
}

/// The record read back: the state it holds, and the id of the comment it
/// was read from.
#[derive(Debug, Clone)]
pub struct Record {
    pub state: PyValue,
    pub comment_id: PyValue,
}

/// `_validate_state(state)`: the record's exact schema.
pub fn validate_state(state: &PyValue) -> Result<(), ReadError> {
    const KEYS: [&str; 8] = [
        "version",
        "number",
        "head",
        "admitted_at",
        "ready_since",
        "state",
        "evidence",
        "context",
    ];
    let schema = || ReadError::github("Unknown or incomplete controller state schema");
    let PyValue::Dict(entries) = state else {
        return Err(schema());
    };
    if entries.len() != KEYS.len() || !KEYS.iter().all(|key| entries.contains_key(*key)) {
        return Err(schema());
    }
    let field = |key: &str| getitem(state, key);
    if !matches!(field("version")?, PyValue::Int(v) if v.as_i64() == Some(1)) {
        return Err(schema());
    }
    if !matches!(field("number")?, PyValue::Int(n) if *n >= PyInt::from(1)) {
        return Err(ReadError::github("Invalid controller PR number"));
    }
    for (key, length) in [("head", 40), ("evidence", 64), ("context", 64)] {
        if !matches!(field(key)?, PyValue::Str(s) if lower_hex(s, length)) {
            return Err(ReadError::github(format!("Invalid controller {key}")));
        }
    }
    if !matches!(field("state")?, PyValue::Str(s) if !s.is_empty()) {
        return Err(ReadError::github("Missing controller lifecycle state"));
    }
    for key in ["admitted_at", "ready_since"] {
        let invalid = || ReadError::github(format!("Invalid controller {key}"));
        match field(key)? {
            PyValue::None => {}
            PyValue::Str(s) => match PyDateTime::fromisoformat(&s.replace('Z', "+00:00")) {
                Ok(instant) if instant.utcoffset().is_some() => {}
                Ok(_) | Err(PyErr::Value(_)) => return Err(invalid()),
                Err(other) => return Err(other.into()),
            },
            _ => return Err(invalid()),
        }
    }
    Ok(())
}

/// `_validate_diff(diff)`. What is there is checked; what is not there is
/// simply not read, so a field a newer engine adds does not make this one
/// refuse the whole marker.
pub fn validate_diff(diff: &PyValue) -> Result<(), ReadError> {
    const KEYS: [&str; 5] = ["number", "diff", "diff_heads", "diff_seen", "receipts"];
    let PyValue::Dict(entries) = diff else {
        return Err(ReadError::github(
            "Unknown or incomplete controller diff schema",
        ));
    };
    if !entries.contains_key("number") || entries.keys().any(|key| !KEYS.contains(&key.as_str())) {
        return Err(ReadError::github(
            "Unknown or incomplete controller diff schema",
        ));
    }
    if !matches!(entries.get("number"), Some(PyValue::Int(n)) if *n >= PyInt::from(1)) {
        return Err(ReadError::github("Invalid controller diff PR number"));
    }
    if let Some(print) = entries.get("diff") {
        if !matches!(print, PyValue::Str(s) if lower_hex(s, 64)) {
            return Err(ReadError::github("Invalid controller diff print"));
        }
    }
    // Present means at least one: an empty list would hand whoever wrote it
    // the instant an attestation is measured against.
    if let Some(heads) = entries.get("diff_heads") {
        let valid = matches!(heads, PyValue::List(heads)
            if (1..=20).contains(&heads.len())
                && heads.iter().all(|h| matches!(h, PyValue::Str(h) if lower_hex(h, 40))));
        if !valid {
            return Err(ReadError::github("Invalid controller diff heads"));
        }
    }
    if let Some(said) = entries.get("receipts") {
        let valid = matches!(said, PyValue::Dict(said)
        if said.len() <= 8
            && said.iter().all(|(k, v)| {
                lower_hex(k, 64) && matches!(v, PyValue::Str(v) if utc_timestamp(v))
            }));
        if !valid {
            return Err(ReadError::github("Invalid controller receipt record"));
        }
    }
    // A timestamp, checked as one: it is read back as a time, and a string
    // that is not one would raise out of the verdict, which catches no such
    // error, and end the whole repository's run.
    match entries.get("diff_seen") {
        None | Some(PyValue::None) => {}
        Some(PyValue::Str(seen)) if utc_timestamp(seen) => {}
        Some(_) => return Err(ReadError::github("Invalid controller diff timestamp")),
    }
    Ok(())
}

/// `_engine_words(comment)`: whether a comment the engine posted still says
/// only what it wrote.
///
/// An edit is what GitHub records as one — a time it was edited, and the
/// editor when GitHub can name them — not a moved update time, which GitHub
/// also moves when a comment is hidden. An edited comment is the engine's
/// own only if GitHub names the engine as the editor; an edit by an account
/// GitHub can no longer name is nobody's it trusts, and a comment read from
/// a route that names no editor is never its own.
fn engine_words(comment: &PyValue) -> Read<bool> {
    let editor = get(comment, "edited_by")?;
    let edited_at = get(comment, "edited_at")?;
    let none = |value: Option<&PyValue>| matches!(value, None | Some(PyValue::None));
    if none(edited_at) && none(editor) {
        return Ok(true);
    }
    Ok(is_engine(editor)?)
}

/// `_text(_written_at(comment), "comment write time")`: when a comment was
/// last written, its last recorded edit or its posting. Not its update
/// time, which GitHub also moves when a comment is hidden; ordered by that,
/// hiding an older record would make it the current one.
fn written_at(comment: &PyValue) -> Read<String> {
    let edited = or_default(get(comment, "edited_at")?);
    let when = match edited {
        Some(value) => Some(value),
        None => get(comment, "created_at")?,
    };
    Ok(text(when, "comment write time")?.to_owned())
}

/// `max(found, key=lambda entry: entry[:2])`: the entry written last, the
/// comment id breaking a tie, the first of equals kept.
fn newest<T>(found: Vec<(String, PyValue, T)>) -> Read<Option<(String, PyValue, T)>> {
    let mut kept: Option<(String, PyValue, T)> = None;
    for entry in found {
        let later = match &kept {
            None => true,
            Some((when, id, _)) => {
                if entry.0 != *when {
                    entry.0 > *when
                } else if !py_eq(&entry.1, id) {
                    py_compare(&entry.1, Compare::Gt, id)?
                } else {
                    false
                }
            }
        };
        if later {
            kept = Some(entry);
        }
    }
    Ok(kept)
}

/// The text of a comment's body, as `re` takes it.
fn body_text(body: &PyValue) -> Read<&str> {
    py::as_str(body, "expected string or bytes-like object, got")
}

/// `marker in body`, whatever the body is.
fn holds(body: &PyValue, marker: &str) -> Read<bool> {
    Ok(py_contains(body, &PyValue::Str(marker.to_owned()))?)
}

/// The first group of every match of a marker's pattern: its JSON.
fn marker_json<'a>(pattern: &Regex, body: &'a str) -> Vec<&'a str> {
    // The group is not optional, so every match has it.
    pattern
        .captures_iter(body)
        .filter_map(|found| found.get(1))
        .map(|group| group.as_str())
        .collect()
}

/// `json.loads(text)` where `except (ValueError, ...)` follows: nothing for
/// what a `ValueError` stops, which the caller catches; a `RecursionError`
/// is not caught.
fn loads(text: &str) -> Read<Option<PyValue>> {
    match py_loads(text) {
        Ok(value) => Ok(Some(value)),
        Err(PyErr::Value(_)) => Ok(None),
        Err(other) => Err(other.into()),
    }
}

/// `parse_controller_state(comments)`: the newest record among the
/// engine's own comments, or nothing.
///
/// The record most recently written is the current one: a refresh edits the
/// newest holder in place. Admission is carried forward unchanged from
/// record to record, which is what keeps the author's slots stable, and is
/// why a newest record somebody else edited decides that nothing is known,
/// rather than letting an older one speak. An edited record is ignored, not
/// refused: refusing it would put any pull request into a configuration
/// error, one edit away. A corrupt record of the engine's own is refused,
/// and so are comments read without who last edited them, among which a
/// forged record would be believed.
pub fn parse_controller_state(comments: &[PyValue]) -> Result<Option<Record>, ReadError> {
    let mut found: Vec<(String, PyValue, Option<PyValue>)> = Vec::new();
    for comment in comments {
        if get(comment, "edited_by")?
            .is_some_and(|editor| py_eq(editor, &PyValue::Str(EDITOR_UNKNOWN.into())))
        {
            return Err(ReadError::github(
                "Controller state read without who last edited it",
            ));
        }
        if !is_engine(Some(getitem(comment, "user")?))? {
            continue;
        }
        let body = getitem(comment, "body")?;
        if !holds(body, STATE_MARKER)? {
            continue;
        }
        if !engine_words(comment)? {
            found.push((written_at(comment)?, getitem(comment, "id")?.clone(), None));
            continue;
        }
        let body = body_text(body)?;
        let matches = marker_json(&STATE_PATTERN, body);
        let ([json], 1) = (matches.as_slice(), body.matches(STATE_MARKER).count()) else {
            return Err(ReadError::github("Malformed controller state marker"));
        };
        let Some(state) = loads(json)? else {
            return Err(ReadError::github("Malformed controller state JSON"));
        };
        validate_state(&state)?;
        found.push((
            written_at(comment)?,
            getitem(comment, "id")?.clone(),
            Some(state),
        ));
    }
    Ok(match newest(found)? {
        Some((_, comment_id, Some(state))) => Some(Record { state, comment_id }),
        _ => None,
    })
}

/// `parse_controller_diff(comments, number)`: the newest recorded diff for
/// this pull request, or nothing.
///
/// Read beside the record, and never fatal: without it a push is read as
/// new work, which is what happened before it was written down at all. Only
/// the engine's own words count, and only beside its record for this same
/// pull request: any workflow can post as the Actions app, and one that
/// echoes text a person wrote would otherwise carry a marker with it.
pub fn parse_controller_diff(
    comments: &[PyValue],
    number: &PyInt,
) -> Result<Option<PyValue>, ReadError> {
    let wanted = PyValue::Int(number.clone());
    let mut found: Vec<(String, PyValue, PyValue)> = Vec::new();
    for comment in comments {
        if !is_engine(Some(getitem(comment, "user")?))?
            || !holds(getitem(comment, "body")?, DIFF_MARKER)?
        {
            continue;
        }
        if !engine_words(comment)? {
            continue;
        }
        let body = body_text(getitem(comment, "body")?)?;
        let [state] = marker_json(&STATE_PATTERN, body)[..] else {
            continue;
        };
        // `except (ValueError, TypeError, AttributeError): continue`.
        let Some(recorded) = loads(state)? else {
            continue;
        };
        let PyValue::Dict(recorded) = recorded else {
            continue;
        };
        if !recorded.get("number").is_some_and(|n| py_eq(n, &wanted)) {
            continue;
        }
        let [json] = marker_json(&DIFF_PATTERN, body)[..] else {
            continue;
        };
        // `except (ValueError, TypeError, GitHubError): continue`.
        let Some(diff) = loads(json)? else {
            continue;
        };
        match validate_diff(&diff) {
            Ok(()) => {}
            Err(ReadError::GitHub(_)) => continue,
            Err(error) if error.is(&[PyClass::ValueError, PyClass::TypeError]) => continue,
            Err(error) => return Err(error),
        }
        if !py_eq(getitem(&diff, "number")?, &wanted) {
            continue;
        }
        found.push((written_at(comment)?, getitem(comment, "id")?.clone(), diff));
    }
    Ok(newest(found)?.map(|(_, _, diff)| diff))
}

/// `GitHub.state_comment_body(state, body, diff)`: the record comment as
/// the engine writes it — the record's marker, the diff's beside it, a blank
/// line, then the words for people.
pub fn state_comment_body(
    state: &PyValue,
    body: &str,
    diff: Option<&PyValue>,
) -> Result<String, ReadError> {
    let compact = |value: &PyValue| {
        py_dumps(value, true, Some((",", ":")), None)
            .map_err(|_| ReadError::NotPorted("a float in a record".into()))
    };
    let mut marker = format!("{STATE_MARKER} {} -->", compact(state)?);
    if body.contains(STATE_MARKER) || body.contains(DIFF_MARKER) {
        return Err(ReadError::github(
            "Controller display body must not contain a state marker",
        ));
    }
    if let Some(diff) = diff {
        validate_diff(diff)?;
        // Writing to a String cannot fail.
        let _ = write!(marker, "\n{DIFF_MARKER} {} -->", compact(diff)?);
    }
    Ok(format!("{marker}\n\n{body}"))
}

/// Byte ranges of `body` outside fenced code blocks, line by line.
fn outside_fences(body: &str) -> Vec<(usize, usize)> {
    let mut inside = false;
    let mut offset = 0;
    let mut spans = Vec::new();
    for line in py_splitlines(body, true) {
        let opening = py_lstrip(line);
        if opening.starts_with("```") || opening.starts_with("~~~") {
            inside = !inside;
        } else if !inside {
            spans.push((offset, offset + line.len()));
        }
        offset += line.len();
    }
    spans
}

/// `_split_checklist(body)`: the author's text, the engine's block and the
/// text after it; or the whole body, no block and nothing.
///
/// The block is the last start marker outside a fenced code block, up to
/// the first end marker after it. A quoted example in a fence is not a
/// block; an extra end marker further down is the author's, and stays; a
/// start with no end after it is no block at all.
pub fn split_checklist(body: &str) -> (&str, Option<&str>, &str) {
    // The markers are ASCII, so a match found in bytes is one in characters,
    // and every offset here lies on a character boundary.
    let mut start = None;
    for (lo, hi) in outside_fences(body) {
        if let Some(found) = body[lo..hi].find(CHECKLIST_START) {
            start = Some(lo + found);
        }
    }
    let Some(start) = start else {
        return (body, None, "");
    };
    let from = start + CHECKLIST_START.len();
    let Some(end) = body[from..].find(CHECKLIST_END) else {
        return (body, None, "");
    };
    let end = from + end + CHECKLIST_END.len();
    (&body[..start], Some(&body[start..end]), &body[end..])
}

/// `current_checklist(body)`: the block a description carries now. A body
/// that is false reads as empty; one that is not a string has no lines.
pub fn current_checklist(body: &PyValue) -> Result<Option<String>, ReadError> {
    match or_default(Some(body)) {
        None => Ok(None),
        Some(value) => {
            let text = str_method(value, "splitlines")?;
            Ok(split_checklist(text).1.map(str::to_owned))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pattern_compiles() {
        for pattern in [&STATE_PATTERN, &DIFF_PATTERN, &TIMESTAMP] {
            LazyLock::force(pattern);
        }
    }

    #[test]
    fn a_timestamp_is_digits_as_python_reads_them() {
        assert!(utc_timestamp("2026-09-11T10:00:00Z"));
        // Python's `\d` is any Unicode digit, so these pass there too.
        assert!(utc_timestamp("٢٠٢٦-09-11T10:00:00Z"));
        assert!(!utc_timestamp("2026-09-11T10:00:00+00:00"));
        assert!(!utc_timestamp("2026-09-11T10:00:00Z\n"));
    }
}
