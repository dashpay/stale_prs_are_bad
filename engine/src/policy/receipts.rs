//! What the review bots said: a CodeRabbit receipt for a head, the print of
//! what a receipt said apart from how it said it, and the severities of the
//! findings a bot opens a thread with.

use super::patterns::{
    BOOKKEEPING, CHECKBOX, CHECKBOX_ITEM, HIDDEN_MARKUP, PASSED, PASTA_HEADING, RABBIT_HEADING,
    RECEIPT_COVERAGE, RECEIPT_TAIL, RISK_BLOCK, SECTIONS, SEPARATOR, SPACES, TABLE, TABLE_ROW,
};
use super::values::{lower, re_text};
use super::{FINDING_BLOCKS, NOTICES, VOLATILE};
use crate::pycompat::hashlib::sha256_hexdigest;
use crate::pycompat::object::{get, getitem, iterate, or, py_str, EMPTY_DICT, EMPTY_STR};
use crate::pycompat::ops::{py_eq, py_eq_str, py_hashable};
use crate::pycompat::text::{py_splitlines, py_strip};
use crate::pycompat::{py_raw_decode, PyErr, PyValue};

/// The marker pairs `receipt_print` drops sections between: each of
/// `VOLATILE` as `<!-- {a}_start -->` and `<!-- {b}_end -->`, then
/// `NOTICES` as they are.
pub(crate) fn section_markers() -> Vec<(String, String)> {
    VOLATILE
        .iter()
        .map(|(a, b)| (format!("<!-- {a}_start -->"), format!("<!-- {b}_end -->")))
        .chain(
            NOTICES
                .iter()
                .map(|(start, end)| (start.to_string(), end.to_string())),
        )
        .collect()
}

/// `receipt_print(comment)`: what this producer said about the code, apart
/// from how it said it, as a SHA-256 hex digest; `None` unless the findings
/// block is there to be read.
///
/// The whole comment counts as what it said, less what it rewrites without
/// meaning anything by it: its banner and tips, its notices about its own
/// capacity, the checkbox a person ticks, the bookkeeping folds, the rule
/// under a table's heading and the prose explaining a passed verdict. A
/// table it reorders reads the same, and so does whitespace. Two comments
/// saying the same thing are still two reports: the print includes the id.
pub fn receipt_print(comment: &PyValue) -> Result<Option<String>, PyErr> {
    let body = re_text(getitem(comment, "body")?)?;
    if !RISK_BLOCK.is_match(body) {
        return Ok(None);
    }
    // Line endings first: the markers below are matched line by line.
    let mut said = body.replace("\r\n", "\n");
    for section in SECTIONS.iter() {
        // Its own marker, alone on its line, written once. Seen twice, one
        // of them is text it echoed back, and deleting from it would take
        // the findings with it.
        if section.alone.find_iter(&said).count() != 1 {
            continue;
        }
        said = section.whole.replace_all(&said, "").into_owned();
    }
    said = BOOKKEEPING.replace_all(&said, "").into_owned();
    said = CHECKBOX_ITEM.replace_all(&said, "").into_owned();
    said = CHECKBOX.replace_all(&said, "").into_owned();
    said = SEPARATOR.replace_all(&said, "").into_owned();
    // The prose beside a passed verdict goes; beside anything else it is
    // what the author has to do.
    said = TABLE_ROW
        .replace_all(&said, |row: &regex::Captures<'_>| {
            // Both groups take part in every match, so indexing them cannot fail.
            let keep = if PASSED.is_match(&row[2]) { 1 } else { 0 };
            row[keep].to_owned()
        })
        .into_owned();
    // A table it reordered is not it saying anything new; sorted as text,
    // nothing is dropped.
    said = TABLE
        .replace_all(&said, |block: &regex::Captures<'_>| {
            // Group 0 is the whole match.
            let mut lines = py_splitlines(&block[0], true);
            lines.sort();
            lines.concat()
        })
        .into_owned();
    let said = SPACES.replace_all(&said, " ");
    let id = py_str(get(comment, "id")?);
    let printed = format!("{id}\n{}", py_strip(&said));
    Ok(Some(sha256_hexdigest(printed.as_bytes())))
}

/// `receipt_instant(pr, comment)`: when this producer first said what the
/// comment now says, for this head. A print already recorded keeps the
/// instant it was first seen; anything else is the comment's own time.
pub fn receipt_instant(pr: &PyValue, comment: &PyValue) -> Result<PyValue, PyErr> {
    let said = receipt_print(comment)?;
    let known = or(
        get(or(get(pr, "controller_diff")?, &EMPTY_DICT), "receipts")?,
        &EMPTY_DICT,
    );
    if let Some(said) = said {
        let recorded = get(known, &said)?;
        if matches!(recorded, PyValue::Str(_)) {
            return Ok(recorded.clone());
        }
    }
    Ok(getitem(comment, "updated_at")?.clone())
}

/// `_rabbit_receipt(body, head)`: whether CodeRabbit's comment carries its
/// coverage marker for `head`, the JSON object right after the marker at
/// the start of a line, closed on the same line.
pub(crate) fn rabbit_receipt(body: &PyValue, head: &PyValue) -> Result<bool, PyErr> {
    let body = re_text(body)?;
    for marker in RECEIPT_COVERAGE.find_iter(body) {
        let rest = &body[marker.end()..];
        // A lone surrogate escape, which Python reads and a Rust string
        // cannot hold, takes the path of JSON that does not read.
        let decoded = match py_raw_decode(rest) {
            Ok(decoded) => decoded,
            Err(PyErr::Value(_) | PyErr::Type(_)) => continue,
            Err(other) => return Err(other),
        };
        if !RECEIPT_TAIL.is_match(&rest[decoded.end_byte..]) {
            continue;
        }
        if let PyValue::Dict(value) = &decoded.value {
            let kind = value.get("kind").unwrap_or(&PyValue::None);
            let covered = value.get("coveredCommitId").unwrap_or(&PyValue::None);
            if py_eq_str(kind, "reviewed") && py_eq(covered, head) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// `_producer(author)`: the login a bot reviews under, without `[bot]`.
pub(crate) fn producer(author: &PyValue) -> Result<String, PyErr> {
    let login = lower(or(author, &EMPTY_STR))?;
    Ok(login
        .strip_suffix("[bot]")
        .map_or_else(|| login.clone(), str::to_owned))
}

/// The severity table of a review bot, by its producer name.
fn finding_table(producer: &str) -> Option<&'static [(&'static str, bool)]> {
    FINDING_BLOCKS
        .iter()
        .find(|(name, _)| *name == producer)
        .map(|(_, table)| *table)
}

/// `finding_severities(author, body)`: the severity of every finding a
/// review bot opened a thread with, read from that bot's own heading shape;
/// a heading with no severity it uses is kept as written. Any blocking
/// label anywhere in the text counts too.
pub fn finding_severities(author: &PyValue, body: &PyValue) -> Result<Vec<String>, PyErr> {
    let producer = producer(author)?;
    let Some(table) = finding_table(&producer) else {
        return Ok(Vec::new());
    };
    let body = or(body, &EMPTY_STR);
    let text = HIDDEN_MARKUP.replace_all(re_text(body)?, "");
    let mut labels: Vec<String> = Vec::new();
    if producer == "thepastaclaw" {
        for heading in PASTA_HEADING.captures_iter(&text) {
            // `label` is not optional, so every match has it.
            labels.push(py_strip(&heading["label"]).to_owned());
        }
    } else {
        for line in RABBIT_HEADING.find_iter(&text) {
            let line = line.as_str();
            let known: Vec<String> = line
                .split('|')
                .map(|segment| py_strip(py_strip(segment).trim_matches('_')).to_owned())
                .filter(|segment| table.iter().any(|(label, _)| label == segment))
                .collect();
            if known.is_empty() {
                labels.push(py_strip(line).to_owned());
            } else {
                labels.extend(known);
            }
        }
    }
    // `body` is text here: `re.sub` above refused anything else.
    let body = re_text(body)?;
    let blocking: Vec<String> = table
        .iter()
        .filter(|(label, blocks)| *blocks && body.contains(label))
        .filter(|(label, _)| !labels.iter().any(|known| known == label))
        .map(|(label, _)| label.to_string())
        .collect();
    labels.extend(blocking);
    Ok(labels)
}

/// `finding_blocks(thread)`: whether a bot's thread holds the pull request
/// until it is resolved. No severity, or any label that is not one of that
/// bot's suggestions, holds it.
pub fn finding_blocks(thread: &PyValue) -> Result<bool, PyErr> {
    let table = finding_table(&producer(get(thread, "author")?)?).unwrap_or(&[]);
    let labels = get(thread, "severities")?;
    if !labels.truthy() {
        return Ok(true);
    }
    for label in iterate(labels)? {
        py_hashable(&label)?;
        let blocks = match label.as_ref() {
            PyValue::Str(label) => table
                .iter()
                .find(|(known, _)| known == label)
                .is_none_or(|(_, blocks)| *blocks),
            _ => true,
        };
        if blocks {
            return Ok(true);
        }
    }
    Ok(false)
}
