//! Where two values differ, said by field path and never by value.
//!
//! Two values are the same when Python's JSON writer, compact and without
//! sorting, writes the same text for both: every scalar alike, every list
//! in order, every map's keys in the same order. Where they are not, each
//! difference is a path and a kind: `pr.reviews[0].state` holds another
//! value, `pr.comments[3]` has its keys in another order.
//!
//! A path names fields of the engine's own shapes and nothing else. A key
//! that is data — a login in `permissions`, a digest in `receipts` — is
//! written `*`, and a key this module does not know as a field is written
//! `?`, so a path can never carry what a recording holds.

use crate::pycompat::PyValue;
use std::fmt;

/// The names of the fields of a snapshot, of `evaluate`'s result and of a
/// verdict row, at any depth: the only keys a path spells out.
const FIELDS: &[&str] = &[
    "address",
    "admitted_at",
    "after",
    "approvals",
    "approved_by",
    "approvers",
    "area",
    "areas",
    "assignees",
    "at",
    "author",
    "author_is_bot",
    "awaiting",
    "base",
    "base_sha",
    "blockers",
    "body",
    "bot_author",
    "bot_completed_at",
    "build",
    "calls",
    "checklist",
    "clock_reads",
    "comment",
    "comment_refused",
    "comments",
    "commit_id",
    "complete",
    "content",
    "context",
    "controller_comment_id",
    "controller_diff",
    "controller_state",
    "created_at",
    "description",
    "diff",
    "diff_heads",
    "diff_print",
    "diff_seen",
    "done",
    "draft",
    "edited_at",
    "edited_by",
    "evidence",
    "filename",
    "files",
    "generated_at",
    "head",
    "head_seen_at",
    "id",
    "is_resolved",
    "item",
    "labels",
    "latched",
    "lifecycle_at",
    "limit",
    "lines",
    "markers",
    "move",
    "nudge",
    "number",
    "objections",
    "objectors",
    "on_their_behalf",
    "outcome",
    "outputs",
    "owned",
    "permissions",
    "previous_filename",
    "pull_requests",
    "ready_published",
    "ready_since",
    "receipt_prints",
    "receipts",
    "record",
    "repo",
    "repository",
    "requested_reviewers",
    "reviewed_heads",
    "reviewed_since",
    "reviewers",
    "reviews",
    "self_reviewed_at",
    "severities",
    "shape",
    "skippable",
    "skipped_by",
    "state",
    "status",
    "submitted_at",
    "target_url",
    "telemetry_reads",
    "threads",
    "title",
    "updated_at",
    "url",
    "user",
    "verdicts",
    "version",
    "voices",
    "waived",
    "waived_at",
    "waived_reason",
];

/// Maps keyed by data rather than by field: access by login, receipts by
/// the digest of what they cover, receipt prints by comment id.
const KEYED_BY_DATA: [&str; 3] = ["permissions", "receipt_prints", "receipts"];

/// What differs at a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// Two scalars of one type, unequal.
    Value,
    /// Two values of different types: a `null` against a string, a list
    /// against a map, an `int` against a `bool`.
    Type,
    /// Two lists of different lengths. Their items are not compared: one
    /// item more shifts every one after it.
    Length,
    /// A key Python's value has and the port's lacks.
    Missing,
    /// A key the port's value has and Python's lacks.
    Extra,
    /// The keys both have, in another order.
    Order,
    /// Python's first reason was the text of a Python exception and the
    /// port gave no reason in its place, or the reverse.
    ExceptionTextAbsent,
    /// Where Python raised, the port's reason is one of the engine's own:
    /// it stopped where Python raised.
    ExceptionTextOwnWords,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Value => "value",
            Kind::Type => "type",
            Kind::Length => "length",
            Kind::Missing => "missing",
            Kind::Extra => "extra",
            Kind::Order => "key order",
            Kind::ExceptionTextAbsent => "exception text on one side only",
            Kind::ExceptionTextOwnWords => "own words where Python raised",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One difference: where, and what kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Difference {
    /// The path with its list indices: `pr.reviews[0].state`.
    pub path: String,
    pub kind: Kind,
}

impl Difference {
    /// The path without its list indices, so that the same field differing
    /// in many items is one category: `pr.reviews[].state`.
    pub fn field(&self) -> String {
        let mut out = String::with_capacity(self.path.len());
        let mut in_index = false;
        for c in self.path.chars() {
            match c {
                '[' => {
                    in_index = true;
                    out.push_str("[]");
                }
                ']' => in_index = false,
                _ if in_index => {}
                _ => out.push(c),
            }
        }
        out
    }
}

/// How a key is written in a path: as itself if it is a field, `*` inside a
/// map keyed by data, `?` otherwise.
fn segment<'a>(container: Option<&str>, key: &'a str) -> &'a str {
    if container.is_some_and(|c| KEYED_BY_DATA.contains(&c)) {
        "*"
    } else if FIELDS.contains(&key) {
        key
    } else {
        "?"
    }
}

/// Which variant a value is, for telling a type difference from a value one.
fn variant(value: &PyValue) -> u8 {
    match value {
        PyValue::None => 0,
        PyValue::Bool(_) => 1,
        PyValue::Int(_) => 2,
        PyValue::Float(_) => 3,
        PyValue::Str(_) => 4,
        PyValue::List(_) => 5,
        PyValue::Dict(_) => 6,
    }
}

/// Two scalars of one variant, equal as Python's JSON writer writes them.
fn same_scalar(a: &PyValue, b: &PyValue) -> bool {
    match (a, b) {
        (PyValue::None, PyValue::None) => true,
        (PyValue::Bool(x), PyValue::Bool(y)) => x == y,
        (PyValue::Int(x), PyValue::Int(y)) => x == y,
        // `repr` tells every float apart but NaNs, which it writes alike.
        (PyValue::Float(x), PyValue::Float(y)) => {
            x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan())
        }
        (PyValue::Str(x), PyValue::Str(y)) => x == y,
        _ => false,
    }
}

/// Every difference between the port's value and Python's, under `root`.
/// Empty exactly when both write the same compact, unsorted JSON.
///
/// The values are walked with an explicit stack, never by recursion: a
/// value read from a recording may nest as deep as Python's reader allows.
pub fn differences(ours: &PyValue, python: &PyValue, root: &str) -> Vec<Difference> {
    let mut found = Vec::new();
    // Each pending pair, its path, and the key of the map it sits under.
    let mut pending: Vec<(&PyValue, &PyValue, String, Option<&str>)> =
        vec![(ours, python, root.to_owned(), None)];
    while let Some((ours, python, path, under)) = pending.pop() {
        let at = |key: &str| format!("{path}.{}", segment(under, key));
        match (ours, python) {
            (PyValue::Dict(mine), PyValue::Dict(theirs)) => {
                for key in theirs.keys().filter(|k| !mine.contains_key(*k)) {
                    found.push(Difference {
                        path: at(key),
                        kind: Kind::Missing,
                    });
                }
                for key in mine.keys().filter(|k| !theirs.contains_key(*k)) {
                    found.push(Difference {
                        path: at(key),
                        kind: Kind::Extra,
                    });
                }
                let shared_mine = mine.keys().filter(|k| theirs.contains_key(*k));
                let shared_theirs = theirs.keys().filter(|k| mine.contains_key(*k));
                if !shared_mine.eq(shared_theirs) {
                    found.push(Difference {
                        path: path.clone(),
                        kind: Kind::Order,
                    });
                }
                // Pushed in reverse so that they are taken in Python's order.
                let shared: Vec<_> = theirs
                    .iter()
                    .filter_map(|(key, value)| mine.get(key).map(|m| (key, m, value)))
                    .collect();
                for (key, m, t) in shared.into_iter().rev() {
                    let child_under = Some(segment(under, key));
                    pending.push((
                        m,
                        t,
                        at(key),
                        child_under.filter(|s| !["*", "?"].contains(s)),
                    ));
                }
            }
            (PyValue::List(mine), PyValue::List(theirs)) => {
                if mine.len() != theirs.len() {
                    found.push(Difference {
                        path,
                        kind: Kind::Length,
                    });
                    continue;
                }
                for (index, (m, t)) in mine.iter().zip(theirs.iter()).enumerate().rev() {
                    pending.push((m, t, format!("{path}[{index}]"), None));
                }
            }
            _ if variant(ours) != variant(python) => found.push(Difference {
                path,
                kind: Kind::Type,
            }),
            _ if !same_scalar(ours, python) => found.push(Difference {
                path,
                kind: Kind::Value,
            }),
            _ => {}
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pycompat::{py_dumps, py_loads, PyList};

    fn v(json: &str) -> PyValue {
        py_loads(json).unwrap()
    }

    fn found(ours: &str, python: &str) -> Vec<(String, Kind)> {
        differences(&v(ours), &v(python), "pr")
            .into_iter()
            .map(|d| (d.path, d.kind))
            .collect()
    }

    #[test]
    fn equal_values_have_no_difference_and_unequal_ones_have_one_per_field() {
        let pr = r#"{"number": 1, "reviews": [{"id": 1, "state": "APPROVED"}], "draft": false}"#;
        assert!(found(pr, pr).is_empty());
        assert_eq!(
            found(
                r#"{"number": 1, "reviews": [{"id": 1, "state": "DISMISSED"}], "draft": null}"#,
                pr
            ),
            [
                ("pr.reviews[0].state".to_owned(), Kind::Value),
                ("pr.draft".to_owned(), Kind::Type),
            ]
        );
    }

    #[test]
    fn python_types_that_compare_equal_are_still_different_values() {
        // `True == 1` in Python, but the JSON written differs, and so does
        // what the next engine reads back.
        assert_eq!(found("[true]", "[1]"), [("pr[0]".to_owned(), Kind::Type)]);
        assert_eq!(found("[1.0]", "[1]"), [("pr[0]".to_owned(), Kind::Type)]);
    }

    #[test]
    fn keys_are_held_to_python_order_and_named_when_missing_or_extra() {
        assert_eq!(
            found(
                r#"{"head": "a", "base": "b"}"#,
                r#"{"base": "b", "head": "a"}"#
            ),
            [("pr".to_owned(), Kind::Order)]
        );
        assert_eq!(
            found(
                r#"{"head": "a", "title": "t"}"#,
                r#"{"head": "a", "body": "b"}"#
            ),
            [
                ("pr.body".to_owned(), Kind::Missing),
                ("pr.title".to_owned(), Kind::Extra),
            ]
        );
    }

    #[test]
    fn a_list_of_another_length_is_one_difference_not_one_per_item() {
        assert_eq!(
            found(r#"{"labels": ["a", "b"]}"#, r#"{"labels": ["b"]}"#),
            [("pr.labels".to_owned(), Kind::Length)]
        );
    }

    #[test]
    fn a_path_never_spells_a_key_that_is_data() {
        // Access is keyed by login, receipts by digest; neither is a field,
        // even where a login happens to be spelled like one.
        let paths = found(
            r#"{"permissions": {"alice": "write", "state": "read"}, "receipts": {"ab12": "x"}, "nickname": 1}"#,
            r#"{"permissions": {"alice": "read", "state": "write"}, "receipts": {"ab12": "y"}, "nickname": 2}"#,
        );
        assert_eq!(
            paths,
            [
                ("pr.permissions.*".to_owned(), Kind::Value),
                ("pr.permissions.*".to_owned(), Kind::Value),
                ("pr.receipts.*".to_owned(), Kind::Value),
                ("pr.?".to_owned(), Kind::Value),
            ]
        );
        let missing = found(
            r#"{"permissions": {}}"#,
            r#"{"permissions": {"mallory": "admin"}}"#,
        );
        assert_eq!(missing, [("pr.permissions.*".to_owned(), Kind::Missing)]);
    }

    #[test]
    fn a_field_drops_its_indices_so_that_one_field_is_one_category() {
        let difference = Difference {
            path: "pr.threads[12].voices[0].user".into(),
            kind: Kind::Value,
        };
        assert_eq!(difference.field(), "pr.threads[].voices[].user");
    }

    #[test]
    fn a_difference_is_found_exactly_when_the_written_json_differs() {
        let compact = |value: &PyValue| py_dumps(value, false, Some((",", ":")), None).unwrap();
        let samples = [
            r#"{"a": [1, {"b": null}], "c": "x"}"#,
            r#"{"c": "x", "a": [1, {"b": null}]}"#,
            r#"{"a": [1, {"b": false}], "c": "x"}"#,
            r#"{"a": [1, {"b": null, "d": 1}], "c": "x"}"#,
            r#"{"a": [1], "c": "x"}"#,
            r#"{"a": [1, {"b": null}], "c": "é"}"#,
            r#"{"a": [1, {"b": null}], "c": "é"}"#,
            r#"{"a": [1, {"b": null}], "c": "x", "c": "y"}"#,
        ];
        for ours in samples {
            for python in samples {
                let (a, b) = (v(ours), v(python));
                assert_eq!(
                    differences(&a, &b, "pr").is_empty(),
                    compact(&a) == compact(&b),
                    "{ours} against {python}"
                );
            }
        }
    }

    #[test]
    fn a_value_nested_as_deep_as_python_reads_does_not_overflow_the_stack() {
        let deep = |leaf: PyValue| {
            let mut value = leaf;
            for _ in 0..200_000 {
                value = PyValue::List(PyList::from(vec![value]));
            }
            value
        };
        let found = differences(&deep(PyValue::Bool(true)), &deep(PyValue::None), "pr");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].kind, Kind::Type);
    }
}
