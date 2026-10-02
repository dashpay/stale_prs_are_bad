//! What the engine remembers on a pull request — the record and the diff
//! beside it — what an author's admission depends on, and how the engine
//! finds its own comments again.

use super::values::{lower, number, s};
use crate::evidence::records::{DIFF_MARKER, STATE_MARKER, STATE_PATTERN};
use crate::evidence::ReadError;
use crate::policy::{diff_print, effective_admission, fingerprint, governs, is_engine};
use crate::pycompat::hashlib::sha256_hexdigest;
use crate::pycompat::object::{
    get, get_or, getitem, no_attribute, or, EMPTY_DICT, EMPTY_STR, NONE,
};
use crate::pycompat::ops::{
    py_compare, py_compare_sequences, py_contains, py_eq, py_eq_str, py_sort_by, Compare,
};
use crate::pycompat::text::{py_slice, py_strip};
use crate::pycompat::{py_dumps, py_loads, PyDict, PyErr, PyInt, PyList, PyValue};
use std::collections::BTreeSet;

/// `state_record(pr, result, context)`: the record the engine keeps on a
/// pull request — its state, head, admission and waiting time, the print
/// of the evidence the verdict used, and of the admission context.
pub fn state_record(pr: &PyValue, result: &PyValue, context: &str) -> Result<PyValue, PyErr> {
    let mut record = PyDict::new();
    for key in ["number", "head", "admitted_at", "ready_since", "state"] {
        record.insert(key.into(), get_or(result, key, &NONE)?.clone());
    }
    record.insert("version".into(), PyValue::Int(PyInt::from(1)));
    record.insert("evidence".into(), s(fingerprint(pr)?));
    record.insert("context".into(), s(context));
    Ok(PyValue::Dict(record))
}

/// `diff_record(pr, result)`: the diff this pull request carries, the
/// commits that have carried it, and what the review bots last said — or
/// `None` when there is nothing to remember.
///
/// Beside the record, not inside it: the record's schema is an exact set
/// of keys, and an engine that predates this would refuse one carrying
/// more and report a configuration error on every pull request in its
/// repository.
pub fn diff_record(pr: &PyValue, result: &PyValue) -> Result<Option<PyValue>, PyErr> {
    if !get_or(result, "number", &NONE)?.truthy() {
        return Ok(None);
    }
    let print_now = diff_print(pr)?.filter(|print| !print.is_empty());
    let receipts = or(get(result, "receipts")?, &EMPTY_DICT);
    let PyValue::Dict(receipts) = receipts else {
        return Err(no_attribute(receipts, "items"));
    };
    let skip = receipts.len().saturating_sub(8);
    let said: PyDict = receipts
        .iter()
        .skip(skip)
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    if print_now.is_none() && said.is_empty() {
        return Ok(None);
    }
    let mut record = PyDict::new();
    record.insert("number".into(), getitem(result, "number")?.clone());
    record.insert("receipts".into(), PyValue::Dict(said));
    if let Some(print_now) = print_now {
        let heads = match get(result, "reviewed_heads")?.filter(|heads| heads.truthy()) {
            Some(heads) => heads.clone(),
            None => PyValue::List(PyList::from(vec![getitem(pr, "head")?.clone()])),
        };
        let heads = match heads {
            PyValue::List(items) => {
                let skip = items.len().saturating_sub(20);
                PyValue::List(items.iter().skip(skip).cloned().collect())
            }
            PyValue::Str(text) => s(py_slice(&text, Some(-20), None)),
            other => {
                return Err(PyErr::type_error(format!(
                    "'{}' object is not subscriptable",
                    crate::pycompat::ops::py_type_name(&other)
                )))
            }
        };
        let seen = match get(result, "reviewed_since")?.filter(|seen| seen.truthy()) {
            Some(seen) => seen.clone(),
            None => get_or(pr, "head_seen_at", &NONE)?.clone(),
        };
        record.insert("diff".into(), s(print_now));
        record.insert("diff_heads".into(), heads);
        record.insert("diff_seen".into(), seen);
    }
    Ok(Some(PyValue::Dict(record)))
}

/// `context_fingerprint(prs, author)`: what this author's admission depends
/// on, and nothing more — the fields `admit` reads of each of their pull
/// requests. A push anywhere else in the repository, or to another of
/// their pull requests, changes none of it.
pub fn context_fingerprint(prs: &[PyValue], author: &PyValue) -> Result<String, PyErr> {
    let mut values: Vec<Vec<PyValue>> = Vec::new();
    for pr in prs {
        if lower(getitem(pr, "author")?)? == lower(author)? {
            let mut fields = Vec::with_capacity(4);
            for key in ["number", "base", "draft", "state"] {
                fields.push(get_or(pr, key, &NONE)?.clone());
            }
            values.push(fields);
        }
    }
    py_sort_by(&mut values, |a, b| py_compare_sequences(a, Compare::Lt, b))?;
    let written = PyValue::List(
        values
            .into_iter()
            .map(|fields| PyValue::List(PyList::from(fields)))
            .collect(),
    );
    let text = py_dumps(&written, true, None, None)?;
    Ok(sha256_hexdigest(text.as_bytes()))
}

/// `admission_conflicts(policy, candidates)`: the authors holding more
/// admissions than the policy allows, which only a race between two runs
/// for one author leaves behind. Sorted, as Python's callers read the set.
pub fn admission_conflicts(
    policy: &PyValue,
    candidates: &[PyValue],
) -> Result<BTreeSet<String>, PyErr> {
    let mut counts: Vec<(String, i64)> = Vec::new();
    for pr in candidates {
        if py_eq_str(getitem(pr, "state")?, "open")
            && !getitem(pr, "draft")?.truthy()
            && governs(policy, getitem(pr, "base")?)?
            && effective_admission(pr)?.truthy()
        {
            let author = lower(getitem(pr, "author")?)?;
            match counts.iter_mut().find(|(known, _)| *known == author) {
                Some((_, count)) => *count += 1,
                None => counts.push((author, 1)),
            }
        }
    }
    let mut over = BTreeSet::new();
    for (author, count) in counts {
        let limit = getitem(policy, "max_active_prs")?;
        if py_compare(&PyValue::Int(PyInt::from(count)), Compare::Gt, limit)? {
            over.insert(author);
        }
    }
    Ok(over)
}

/// `admission_fingerprint(candidates)`: each pull request's number, the
/// admission it holds and when it last went inactive, in order.
pub fn admission_fingerprint<'p>(
    candidates: impl IntoIterator<Item = &'p PyValue>,
) -> Result<Vec<Vec<PyValue>>, PyErr> {
    let mut keyed = Vec::new();
    for pr in candidates {
        keyed.push(vec![
            getitem(pr, "number")?.clone(),
            effective_admission(pr)?.clone(),
            get_or(pr, "lifecycle_at", &NONE)?.clone(),
        ]);
    }
    py_sort_by(&mut keyed, |a, b| py_compare_sequences(a, Compare::Lt, b))?;
    Ok(keyed)
}

/// Two admission fingerprints, as Python's `==` compares the lists.
pub(crate) fn same_admissions(a: &[Vec<PyValue>], b: &[Vec<PyValue>]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(x, y)| {
            py_eq(
                &PyValue::List(PyList::from(x.clone())),
                &PyValue::List(PyList::from(y.clone())),
            )
        })
}

/// `bot_comments(pr, marker)`: the engine's own comments carrying `marker`,
/// oldest first by last write.
///
/// Its own: written under one of its identities and carrying a record that
/// parses and names this pull request. Another workflow's bot comment that
/// quotes a marker is nobody's business here, least of all to edit or
/// delete.
pub fn bot_comments<'p>(pr: &'p PyValue, marker: &str) -> Result<Vec<&'p PyValue>, ReadError> {
    let mut own = Vec::new();
    for comment in crate::pycompat::object::iterate(get_or(
        pr,
        "comments",
        &crate::pycompat::object::EMPTY_LIST,
    )?)? {
        let std::borrow::Cow::Borrowed(comment) = comment else {
            // Only a list's items are comments; a dict's keys or a
            // string's characters have no `.get`.
            return Err(no_attribute(&comment, "get").into());
        };
        if is_own(pr, comment, marker)? {
            own.push(comment);
        }
    }
    let mut keyed = Vec::with_capacity(own.len());
    for comment in own {
        let written = match get(comment, "updated_at")?.filter(|value| value.truthy()) {
            Some(updated) => updated.clone(),
            None => get_or(comment, "created_at", &EMPTY_STR)?.clone(),
        };
        let id = get_or(comment, "id", &PyValue::Int(PyInt::from(0)))?.clone();
        keyed.push((vec![written, id], comment));
    }
    py_sort_by(&mut keyed, |(a, _), (b, _)| {
        py_compare_sequences(a, Compare::Lt, b)
    })?;
    Ok(keyed.into_iter().map(|(_, comment)| comment).collect())
}

fn is_own(pr: &PyValue, comment: &PyValue, marker: &str) -> Result<bool, ReadError> {
    if !is_engine(get(comment, "user")?)? {
        return Ok(false);
    }
    let body = get_or(comment, "body", &EMPTY_STR)?;
    if !py_contains(body, &s(marker))? {
        return Ok(false);
    }
    let PyValue::Str(body) = body else {
        return Err(PyErr::type_error("expected string or bytes-like object").into());
    };
    let found: Vec<&str> = STATE_PATTERN
        .captures_iter(body)
        .filter_map(|found| found.get(1))
        .map(|group| group.as_str())
        .collect();
    let [json] = found[..] else {
        return Ok(false);
    };
    // `except (ValueError, TypeError, AttributeError): return False`.
    let recorded = match py_loads(json) {
        Ok(recorded) => recorded,
        Err(PyErr::Value(_)) => return Ok(false),
        Err(other) => return Err(other.into()),
    };
    let PyValue::Dict(recorded) = &recorded else {
        return Ok(false);
    };
    let named = recorded.get("number").unwrap_or(&PyValue::None);
    Ok(py_eq(named, &PyValue::Int(number(pr)?)))
}

/// `_visible(body)`: a record comment's words, without the records it
/// carries.
///
/// There is more than one marker line, and stripping only the first left
/// the second in the words: they never matched what a run would write, so
/// the comment was rewritten on every run. A line somebody truncated is
/// still that line, and leaving it in the words is not harmless either: the
/// words are written back, and a write that carries a marker is refused.
pub fn visible(body: &str) -> String {
    let mut rest = body;
    while rest.starts_with(STATE_MARKER) || rest.starts_with(DIFF_MARKER) {
        rest = match rest.find("-->") {
            Some(at) => &rest[at + "-->".len()..],
            None => match rest.find('\n') {
                Some(at) => &rest[at + 1..],
                None => "",
            },
        };
        rest = rest.trim_start_matches('\n');
    }
    py_strip(rest).to_owned()
}

/// `_same(a, b)`: two texts alike once line endings and the space around
/// them are set aside. An absent text is an empty one.
pub fn same(a: Option<&str>, b: Option<&str>) -> bool {
    let norm = |text: Option<&str>| py_strip(&text.unwrap_or("").replace("\r\n", "\n")).to_owned();
    norm(a) == norm(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_words_lose_every_record_line_however_it_was_cut() {
        assert_eq!(
            visible("<!-- platform-pr-review-state-v1 {} -->\n<!-- pr-hygiene-diff-v1 {} -->\n\nwords\n"),
            "words"
        );
        // A line whose end somebody deleted is still that line.
        assert_eq!(
            visible("<!-- pr-hygiene-diff-v1 {\"number\":1} \nwords"),
            "words"
        );
        assert_eq!(visible("<!-- pr-hygiene-diff-v1"), "");
        assert_eq!(visible("plain"), "plain");
    }

    #[test]
    fn sameness_sets_aside_line_endings_and_the_space_around() {
        assert!(same(Some("a\r\nb \n"), Some("a\nb")));
        assert!(same(None, Some("  ")));
        assert!(!same(Some("a"), Some("b")));
    }
}
