//! The prints: digests that say whether something changed. `diff_print`
//! is what a reviewer read, `fingerprint` the evidence a verdict was made
//! on, and `carried_heads` the commits a diff print says still carry the
//! same review. Each digest is SHA-256 over the bytes Python's `json.dumps`
//! writes, so the two engines' prints are the same strings.

use super::{is_engine, NUDGE_MARKER, STATE_MARKER};
use crate::pycompat::hashlib::sha256_hexdigest;
use crate::pycompat::object::{
    get, get_or, getitem, iterate, no_attribute, or, str_method, EMPTY_DICT, EMPTY_LIST, EMPTY_STR,
};
use crate::pycompat::ops::{py_compare_sequences, py_eq, py_sort_by, Compare};
use crate::pycompat::{py_dumps, PyErr, PyList, PyValue};

/// `diff_print(pr)`: what a reviewer read, this pull request's own changes
/// whatever commit carries them. Each changed file's name, status, content
/// id, patch id and former name, sorted; `None` when a file lacks its
/// content or its patch, since a read that cannot say what changed says
/// nothing.
pub fn diff_print(pr: &PyValue) -> Result<Option<String>, PyErr> {
    let files = iterate(or(get(pr, "files")?, &EMPTY_LIST))?;
    for file in &files {
        if !get(file, "content")?.truthy() || !get(file, "shape")?.truthy() {
            return Ok(None);
        }
    }
    let mut items = files
        .iter()
        .map(|file| {
            let item = [
                getitem(file, "filename")?.clone(),
                or(get(file, "status")?, &EMPTY_STR).clone(),
                getitem(file, "content")?.clone(),
                getitem(file, "shape")?.clone(),
                or(get(file, "previous_filename")?, &EMPTY_STR).clone(),
            ];
            Ok(item)
        })
        .collect::<Result<Vec<_>, PyErr>>()?;
    py_sort_by(&mut items, |a, b| py_compare_sequences(a, Compare::Lt, b))?;
    let items: PyList = items
        .into_iter()
        .map(|item| PyValue::List(item.into_iter().collect()))
        .collect();
    let written = py_dumps(&PyValue::List(items), false, Some((",", ":")), None)?;
    Ok(Some(sha256_hexdigest(written.as_bytes())))
}

/// `carried_heads(pr)`: the commits whose review still applies here, newest
/// last, and when the first of them was seen. A push that leaves the diff
/// print as the record has it carries the commits recorded with it (the
/// last twenty); anything that changes the diff starts over with this head.
pub fn carried_heads(pr: &PyValue) -> Result<(Vec<PyValue>, PyValue), PyErr> {
    let record = or(get(pr, "controller_diff")?, &EMPTY_DICT);
    let print_now = diff_print(pr)?;
    let unchanged = match &print_now {
        Some(print_now) => matches!(get(record, "diff")?, PyValue::Str(d) if d == print_now),
        None => false,
    };
    if !unchanged {
        return Ok((
            vec![getitem(pr, "head")?.clone()],
            get(pr, "head_seen_at")?.clone(),
        ));
    }
    let recorded = iterate(or(get(record, "diff_heads")?, &EMPTY_LIST))?;
    let mut kept: Vec<PyValue> = recorded
        .into_iter()
        .filter(|head| matches!(head.as_ref(), PyValue::Str(_)))
        .map(|head| head.into_owned())
        .collect();
    let from = kept.len().saturating_sub(20);
    kept.drain(..from);
    let head = getitem(pr, "head")?;
    if !kept.iter().any(|kept| py_eq(kept, head)) {
        kept.push(head.clone());
    }
    // The moment the first of them was seen: an attestation written then
    // was written about this same diff.
    let since = or(get(record, "diff_seen")?, get(pr, "head_seen_at")?).clone();
    Ok((kept, since))
}

/// `value.pop(key, None)` on a dict; Python's error on anything else.
fn pop(value: &mut PyValue, key: &str) -> Result<(), PyErr> {
    match value {
        PyValue::Dict(entries) => {
            entries.shift_remove(key);
            Ok(())
        }
        PyValue::List(_) => Err(PyErr::type_error("pop expected at most 1 argument, got 2")),
        other => Err(no_attribute(other, "pop")),
    }
}

/// The comments as `relevant.get('comments', [])` iterates them, to change
/// each in place: a list's items. Iterating anything else yields keys or
/// characters, on which `pop` and `get` raise.
fn comments_of(relevant: &mut PyValue) -> Result<Vec<&mut PyValue>, PyErr> {
    let PyValue::Dict(entries) = relevant else {
        return Err(no_attribute(relevant, "get"));
    };
    match entries.get_mut("comments") {
        None => Ok(Vec::new()),
        Some(PyValue::List(items)) => Ok(items.iter_mut().collect()),
        Some(other) => {
            let first = iterate(other)?.into_iter().next();
            match first {
                // A key or a character: a string, which has no `pop`.
                Some(item) => Err(no_attribute(&item, "pop")),
                None => Ok(Vec::new()),
            }
        }
    }
}

/// `fingerprint(pr)`: the evidence a verdict was made on, as a digest.
///
/// This controller's own effects are left out: its record, its diff
/// record, the labels, the reviewers it requested, the build (which can
/// change under a write without the evidence changing), the description
/// (which carries its checklist), and its own state and nudge comments.
/// Who last edited a comment and when GraphQL says it was edited are left
/// out too, so the print does not depend on which route read the comment;
/// the update time already moves with every edit.
///
/// Python copies the snapshot with `copy.deepcopy`, which raises
/// `RecursionError` on a value nested some five hundred levels deep; this
/// does not, and nothing GitHub answers nests that deep.
pub fn fingerprint(pr: &PyValue) -> Result<String, PyErr> {
    let mut relevant = pr.clone();
    for name in [
        "controller_state",
        "controller_comment_id",
        "controller_diff",
        "labels",
        "requested_reviewers",
        "build",
        "body",
    ] {
        pop(&mut relevant, name)?;
    }
    for comment in comments_of(&mut relevant)? {
        pop(comment, "edited_by")?;
        pop(comment, "edited_at")?;
    }
    let state = format!("<!-- {STATE_MARKER}");
    let mut kept = Vec::new();
    for comment in iterate(get_or(&relevant, "comments", &EMPTY_LIST)?)? {
        let own = is_engine(get(&comment, "user")?)? && {
            let body = get_or(&comment, "body", &EMPTY_STR)?;
            str_method(body, "startswith")?.starts_with(&state)
                || str_method(body, "startswith")?.starts_with(NUDGE_MARKER)
        };
        if !own {
            kept.push(comment.into_owned());
        }
    }
    let PyValue::Dict(entries) = &mut relevant else {
        return Err(no_attribute(&relevant, "get"));
    };
    entries.insert("comments".into(), PyValue::List(kept.into_iter().collect()));
    for name in ["comments", "reviews", "threads", "files"] {
        let Some(value) = entries.get(name) else {
            continue;
        };
        let mut keyed = iterate(value)?
            .into_iter()
            .map(|item| {
                let key = py_dumps(&item, true, None, None)?;
                Ok((key, item.into_owned()))
            })
            .collect::<Result<Vec<_>, PyErr>>()?;
        keyed.sort_by(|a, b| a.0.cmp(&b.0));
        let sorted: PyList = keyed.into_iter().map(|(_, item)| item).collect();
        entries.insert(name.into(), PyValue::List(sorted));
    }
    let written = py_dumps(&relevant, true, Some((",", ":")), None)?;
    Ok(sha256_hexdigest(written.as_bytes()))
}
