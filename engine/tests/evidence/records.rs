//! The engine's own record and diff, read back: who may have written them,
//! which one is current, and the checklist block in a description.
//!
//! Ported from `pr_review/tests/test_github.py` (the record tests in
//! `BuildVerdictTests`, `GitHubTests` and `EngineIdentityTests`) and the
//! reading parts of `test_checklist.py`.

use crate::support::*;
use pr_hygiene_engine::evidence::records::{
    current_checklist, parse_controller_diff, parse_controller_state, split_checklist,
    validate_diff, Record,
};
use pr_hygiene_engine::evidence::rules::{CHECKLIST_END, CHECKLIST_START, ENGINE_LOGINS};

fn state(overrides: Value) -> Value {
    merged(
        &json!({"version": 1, "number": 7, "head": "b".repeat(40), "admitted_at": "2026-09-11T10:00:00Z",
                "ready_since": null, "state": "waiting-bots", "evidence": "c".repeat(64),
                "context": "d".repeat(64)}),
        overrides,
    )
}

/// `BuildVerdictTests.record`: the engine's record comment, last edited by
/// `edited_by` at `updated_at` when anyone edited it.
fn record(updated_at: &str, edited_by: Option<&str>, overrides: Value) -> Value {
    json!({"id": 1, "user": "github-actions[bot]", "body": record_body(state(overrides), "text", None),
           "created_at": "2026-09-11T10:00:00Z", "updated_at": updated_at, "edited_by": edited_by,
           "edited_at": if edited_by.is_some() { Some(updated_at) } else { None }})
}

fn parsed(comments: &[Value]) -> Option<Record> {
    let comments: Vec<PyValue> = comments.iter().cloned().map(py).collect();
    parse_controller_state(&comments).expect("a readable history")
}

fn comment_id(found: &Option<Record>) -> Option<i64> {
    found.as_ref().map(|record| match &record.comment_id {
        PyValue::Int(id) => id.as_i64().expect("a small id"),
        other => panic!("not an id: {}", shown(other)),
    })
}

fn diff_of(comments: &[Value], number: i64) -> Option<PyValue> {
    let comments: Vec<PyValue> = comments.iter().cloned().map(py).collect();
    parse_controller_diff(&comments, &int(number)).expect("a readable history")
}

#[test]
fn a_diff_marker_somebody_else_edited_is_not_read() {
    // It says which commits a review still covers. Anyone with write access
    // can edit anyone's comment, and a forged one carries a stale approval —
    // the only human gate left — onto code nobody read.
    let diff = json!({"number": 7, "diff": "a".repeat(64), "diff_heads": ["b".repeat(40)],
                      "diff_seen": "2026-09-11T10:00:00Z"});
    let body = record_body(
        state(json!({"admitted_at": null, "state": "ready-for-human"})),
        "text",
        Some(diff.clone()),
    );
    let made = json!({"id": 1, "user": "github-actions[bot]", "body": body,
                      "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T10:00:00Z"});
    assert_py(
        &diff_of(std::slice::from_ref(&made), 7).unwrap(),
        diff.clone(),
    );
    let edited = merged(
        &made,
        json!({"updated_at": "2026-09-11T12:00:00Z", "edited_at": "2026-09-11T12:00:00Z"}),
    );
    // Edited by a person, or by an account GitHub can no longer name.
    for editor in [json!("llbartekll"), json!(null)] {
        assert!(
            diff_of(&[merged(&edited, json!({ "edited_by": editor }))], 7).is_none(),
            "{editor}"
        );
    }
    // A moved update time with no edit recorded is no edit: GitHub moves it
    // when a comment is hidden. The record is read by the same rule.
    let hidden = merged(&made, json!({"updated_at": "2026-09-11T12:00:00Z"}));
    assert_py(&diff_of(&[hidden], 7).unwrap(), diff.clone());
    // The engine's own hand: the editor is read with its type, so it comes
    // back as `github-actions[bot]`. Refusing it refused every record the
    // engine had refreshed. The bare name is one a person can register.
    let refreshed = merged(&edited, json!({"edited_by": "github-actions[bot]"}));
    assert_py(&diff_of(&[refreshed], 7).unwrap(), diff.clone());
    assert!(diff_of(
        &[merged(&edited, json!({"edited_by": "github-actions"}))],
        7
    )
    .is_none());
    // And only beside the engine's record for this same pull request: any
    // workflow can post as the Actions app.
    let python_dumped = r#"{"number": 7, "diff": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "diff_heads": ["bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"], "diff_seen": "2026-09-11T10:00:00Z"}"#;
    let alone = merged(
        &made,
        json!({"id": 2, "body": format!("<!-- pr-hygiene-diff-v1 {python_dumped} -->")}),
    );
    assert!(diff_of(&[alone], 7).is_none());
    assert!(diff_of(&[made], 8).is_none(), "another pull request");
}

#[test]
fn a_record_whose_admission_a_collaborator_edited_is_not_trusted() {
    // The record says when this pull request took one of its author's review
    // slots, and whether it was ever ready for a human on this head. An
    // earlier admission moves it ahead of the author's own queue, and
    // `ready-for-human` on the current head is remembered as having passed
    // the green build that is asked for before a human is.
    let forged = record(
        "2026-09-11T12:00:00Z",
        Some("llbartekll"),
        json!({"admitted_at": "2026-01-01T00:00:00Z", "state": "ready-for-human"}),
    );
    assert!(parsed(std::slice::from_ref(&forged)).is_none());
    // The bare name is one a person can register.
    assert!(parsed(&[merged(&forged, json!({"edited_by": "github-actions"}))]).is_none());
    // An edit GitHub records but cannot say by whom, as for a deleted or
    // suspended account, is nobody the engine trusts.
    assert!(parsed(&[merged(&forged, json!({"edited_by": null}))]).is_none());
    // Ignored, not refused: refusing it would hand anyone with write access
    // a configuration error on any pull request, one edit away.
    let body = forged["body"]
        .as_str()
        .unwrap()
        .replace("\"version\":1", "\"version\":2");
    assert!(parsed(&[merged(&forged, json!({ "body": body }))]).is_none());
    // The newest record decides even forged: nothing older stands in for it,
    // or editing the newest would bring back what an older one held.
    let kept = record("2026-09-11T10:00:00Z", None, json!({}));
    assert!(parsed(&[kept, merged(&forged, json!({"id": 2}))]).is_none());
}

#[test]
fn editing_the_newest_record_does_not_bring_back_an_older_admission() {
    // An older announcement still carries the admission of its day; the
    // newest record, set aside when the pull request left the policy,
    // carries none. Were a forged newest record merely skipped, the older
    // one would be read in its place, and one edit would hand back the place
    // in the queue the pull request gave up.
    let old = merged(
        &record(
            "2026-01-01T00:00:00Z",
            None,
            json!({"admitted_at": "2026-01-01T00:00:00Z"}),
        ),
        json!({"id": 1, "created_at": "2026-01-01T00:00:00Z"}),
    );
    let aside = merged(
        &record(
            "2026-09-12T00:00:00Z",
            Some("github-actions[bot]"),
            json!({"admitted_at": null, "state": "not-governed"}),
        ),
        json!({"id": 2, "created_at": "2026-09-01T00:00:00Z"}),
    );
    assert_eq!(comment_id(&parsed(&[old.clone(), aside.clone()])), Some(2));
    let forged = merged(
        &aside,
        json!({"edited_by": "mallory", "edited_at": "2026-09-13T00:00:00Z",
               "updated_at": "2026-09-13T00:00:00Z"}),
    );
    assert!(parsed(&[old, forged]).is_none());
}

#[test]
fn an_older_record_hidden_since_is_not_taken_for_the_newest() {
    // Hiding a comment moves its update time and records no edit. The
    // current record is the one the engine wrote last.
    let old = merged(
        &record(
            "2026-09-14T00:00:00Z",
            None,
            json!({"admitted_at": "2026-01-01T00:00:00Z"}),
        ),
        json!({"id": 1, "created_at": "2026-01-01T00:00:00Z"}),
    );
    let newer = merged(
        &record(
            "2026-09-12T00:00:00Z",
            Some("github-actions[bot]"),
            json!({"admitted_at": null, "state": "not-governed"}),
        ),
        json!({"id": 2, "created_at": "2026-09-01T00:00:00Z"}),
    );
    assert_eq!(comment_id(&parsed(&[old, newer])), Some(2));
}

#[test]
fn an_older_diff_hidden_since_is_not_taken_for_the_newest() {
    // The diff is read by the same clock as the record beside it, or the two
    // could come from different comments.
    let st = state(json!({"admitted_at": null}));
    let older = json!({"number": 7, "diff": "a".repeat(64), "diff_heads": ["b".repeat(40)]});
    let newer = json!({"number": 7, "diff": "e".repeat(64), "diff_heads": ["b".repeat(40)]});
    let old = json!({"id": 1, "user": "github-actions[bot]",
                     "body": record_body(st.clone(), "text", Some(older)),
                     "created_at": "2026-09-01T00:00:00Z", "updated_at": "2026-09-14T00:00:00Z",
                     "edited_at": null, "edited_by": null});
    let new = json!({"id": 2, "user": "github-actions[bot]",
                     "body": record_body(st, "text", Some(newer.clone())),
                     "created_at": "2026-09-10T00:00:00Z", "updated_at": "2026-09-12T00:00:00Z",
                     "edited_at": "2026-09-12T00:00:00Z", "edited_by": "github-actions[bot]"});
    assert_py(&diff_of(&[old, new], 7).unwrap(), newer);
}

#[test]
fn a_record_this_controller_refreshed_is_still_read() {
    // Every run that changes the record rewrites it in place, so all but a
    // new record have been edited — by the engine. Refusing those would give
    // up every pull request's slot and review clock each run.
    let refreshed = record(
        "2026-09-11T12:00:00Z",
        Some("github-actions[bot]"),
        json!({"state": "ready-for-human"}),
    );
    let found = parsed(&[refreshed]);
    assert_eq!(comment_id(&found), Some(1));
    assert_eq!(
        text(field(&found.unwrap().state, "state")),
        "ready-for-human"
    );
}

#[test]
fn a_record_nobody_edited_is_read_though_its_update_time_moved() {
    // GitHub moves a comment's update time for changes that are not edits,
    // and names no editor for them. A record nobody edited is the engine's
    // own words, and ignoring it gives up the slot and the review clock.
    let touched = record(
        "2026-09-11T12:00:00Z",
        None,
        json!({"state": "ready-for-human"}),
    );
    let found = parsed(&[touched]);
    assert_eq!(comment_id(&found), Some(1));
    assert_eq!(
        text(field(&found.unwrap().state, "state")),
        "ready-for-human"
    );
}

#[test]
fn a_diff_timestamp_that_is_not_one_is_refused() {
    // It is read back as a time. A string that is not one raised out of the
    // verdict and took the whole repository's run with it.
    for bad in [
        json!("banana"),
        json!("2026-09-11"),
        json!("2026-09-11T10:00:00+03:00"),
        json!(5),
    ] {
        let diff = json!({"number": 1, "diff": "a".repeat(64), "diff_heads": ["b".repeat(40)],
                          "diff_seen": bad.clone()});
        assert!(
            matches!(validate_diff(&py(diff)), Err(ReadError::GitHub(_))),
            "{bad}"
        );
    }
    // A key that is there is checked. An empty head list was accepted once,
    // and it handed whoever wrote it the instant an attestation is measured
    // against.
    let empty = json!({"number": 1, "diff": "a".repeat(64), "diff_heads": [], "diff_seen": null});
    assert_eq!(
        github_error(validate_diff(&py(empty))),
        "Invalid controller diff heads"
    );
    // A key that is not there is simply not read, so a marker written by a
    // newer engine does not make an older one refuse everything it knows.
    validate_diff(&py(json!({"number": 1}))).unwrap();
    validate_diff(&py(
        json!({"number": 1, "receipts": {"c".repeat(64): "2026-09-11T10:00:00Z"}}),
    ))
    .unwrap();
}

#[test]
fn comments_read_without_their_editors_are_refused_by_the_record_reader() {
    // The comments listing names no editor. Read from there, a record a
    // collaborator edited looks like one nobody edited, and their admission
    // or their `ready-for-human` would be believed; so the record reader
    // refuses what that route read rather than guess.
    let body = record_body(
        json!({"version": 1, "number": 1, "head": "a".repeat(40),
               "admitted_at": "2026-01-01T00:00:00Z", "ready_since": null,
               "state": "ready-for-human", "evidence": "c".repeat(64), "context": "d".repeat(64)}),
        "text",
        None,
    );
    let listing = json!([{"id": 1, "user": {"login": "github-actions[bot]"}, "body": body,
                          "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T12:00:00Z"}]);
    let mut api = api(move |_| page(listing.clone()));
    let rest = api.comments(&int(1)).unwrap();
    assert_eq!(
        github_error(parse_controller_state(&rest)),
        "Controller state read without who last edited it"
    );
}

#[test]
fn the_newest_state_comment_wins_however_the_page_is_ordered() {
    let comment = |number: i64, created_at: &str| {
        let state = json!({"version": 1, "number": 1, "head": "a".repeat(40), "admitted_at": null,
                           "ready_since": null, "state": "too-many-open-prs",
                           "evidence": number.to_string().repeat(64), "context": "c".repeat(64)});
        // Written as Python's json.dumps writes it, with spaces.
        let body = format!(
            "<!-- platform-pr-review-state-v1 {} -->",
            py_dumps(&py(state), false, None, None).unwrap()
        );
        json!({"id": number, "user": "github-actions[bot]", "body": body,
               "created_at": created_at, "updated_at": created_at})
    };
    let earlier = comment(9, "2026-09-01T00:00:00Z");
    let later = comment(4, "2026-09-02T00:00:00Z");
    for page in [
        [earlier.clone(), later.clone()],
        [later.clone(), earlier.clone()],
    ] {
        assert_eq!(
            comment_id(&parsed(&page)),
            Some(4),
            "latest comment, not highest id"
        );
    }
    // GitHub timestamps are whole seconds, so simultaneous writes can tie.
    // Without a second key two runs could each keep a different comment.
    let tied = [
        comment(7, "2026-09-01T00:00:00Z"),
        comment(5, "2026-09-01T00:00:00Z"),
    ];
    assert_eq!(comment_id(&parsed(&tied)), Some(7));
    let reversed = [tied[1].clone(), tied[0].clone()];
    assert_eq!(comment_id(&parsed(&reversed)), Some(7));
}

#[test]
fn should_reject_unknown_controller_schema() {
    let comments = [py(json!({"id": 1, "user": "github-actions[bot]",
                              "body": "<!-- platform-pr-review-state-v1 {\"version\":99} -->"}))];
    assert_eq!(
        github_error(parse_controller_state(&comments)),
        "Unknown or incomplete controller state schema"
    );
}

#[test]
fn a_record_is_held_to_its_schema_field_by_field() {
    // Each field is read back and decided from, so each is checked as what
    // it is; the first that fails names itself.
    use pr_hygiene_engine::evidence::records::validate_state;
    let check = |overrides: Value| validate_state(&py(state(overrides)));
    check(json!({})).unwrap();
    for (overrides, message) in [
        (
            json!({"version": true}),
            "Unknown or incomplete controller state schema",
        ),
        (json!({"number": 0}), "Invalid controller PR number"),
        (json!({"number": true}), "Invalid controller PR number"),
        (json!({"head": "B".repeat(40)}), "Invalid controller head"),
        (
            json!({"evidence": "c".repeat(63)}),
            "Invalid controller evidence",
        ),
        (json!({"state": ""}), "Missing controller lifecycle state"),
        // A timestamp without a zone cannot be ordered against one with.
        (
            json!({"admitted_at": "2026-09-11T10:00:00"}),
            "Invalid controller admitted_at",
        ),
        (
            json!({"ready_since": "soon"}),
            "Invalid controller ready_since",
        ),
        (json!({"ready_since": 5}), "Invalid controller ready_since"),
    ] {
        assert_eq!(
            github_error(check(overrides.clone())),
            message,
            "{overrides}"
        );
    }
    // Python's fromisoformat, with every `Z` read as UTC: an offset other
    // than zero is a zone too.
    check(json!({"admitted_at": "2026-09-11T10:00:00+03:00"})).unwrap();
    let mut extra = state(json!({}));
    extra["note"] = json!("x");
    assert_eq!(
        github_error(validate_state(&py(extra))),
        "Unknown or incomplete controller state schema"
    );
}

#[test]
fn a_diff_line_somebody_truncated_leaves_the_record_readable() {
    // The diff is never fatal: a marker line cut short beside an intact
    // record reads as no diff, and the record still reads.
    let body = record_body(state(json!({"number": 1})), "text", None);
    let broken = body.replacen("\n\n", "\n<!-- pr-hygiene-diff-v1 {\"number\":1} \n\n", 1);
    let comments = [
        json!({"id": 50, "user": "github-actions[bot]", "body": broken,
                           "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T10:00:00Z"}),
    ];
    assert_eq!(comment_id(&parsed(&comments)), Some(50));
    assert!(diff_of(&comments, 1).is_none());
}

/// `EngineIdentityTests`: the engine's memory is recognised by who wrote it.
fn written_by(user: &str) -> Vec<Value> {
    let record = json!({"version": 1, "number": 7, "head": "a".repeat(40),
                        "admitted_at": "2026-09-01T10:00:00Z", "ready_since": null,
                        "state": "waiting-bots", "evidence": "c".repeat(64), "context": "c".repeat(64)});
    let diff = json!({"number": 7, "diff": "d".repeat(64), "diff_heads": ["a".repeat(40)], "receipts": {}});
    let body = record_body(record, "x", Some(diff));
    vec![
        json!({"id": 1, "user": user, "created_at": "2026-09-11T10:00:00Z",
                "updated_at": "2026-09-11T10:00:00Z", "body": body}),
    ]
}

#[test]
fn the_list_is_lowercase() {
    for login in ENGINE_LOGINS {
        assert_eq!(*login, login.to_lowercase());
    }
}

#[test]
fn an_unlisted_identity_is_nobodys() {
    // The PR Hygiene App is not listed yet: its comments are anyone's.
    let comments = written_by("pr-hygiene[bot]");
    assert_eq!(comment_id(&parsed(&comments)), None);
    assert!(diff_of(&comments, 7).is_none());
}

#[test]
fn only_the_bot_spelling_counts_in_any_case() {
    // The bare name is one a person can register; only the `[bot]` login
    // GitHub gives an App is the engine, whatever its case.
    assert_eq!(comment_id(&parsed(&written_by("github-actions"))), None);
    assert_eq!(
        comment_id(&parsed(&written_by("GitHub-Actions[BOT]"))),
        Some(1)
    );
    assert!(diff_of(&written_by("GitHub-Actions[BOT]"), 7).is_some());
}

#[test]
fn every_listed_identity_continues_the_record() {
    // Repositories move to the App one at a time, and either writer must
    // continue the other's records, or a rollback starts every pull request
    // over. Listing an identity is the one change that needs; this holds
    // every listed one to it, the bare name of each to nothing.
    for login in ENGINE_LOGINS {
        assert_eq!(comment_id(&parsed(&written_by(login))), Some(1), "{login}");
        assert!(diff_of(&written_by(login), 7).is_some(), "{login}");
        let bare = login.trim_end_matches("[bot]");
        assert_eq!(comment_id(&parsed(&written_by(bare))), None, "{bare}");
    }
}

#[test]
fn an_older_engine_still_reads_the_record_beside_the_diff() {
    // The diff rides in its own marker because the record's
    // schema is an exact set of keys; the record next to it still reads.
    let record = state(json!({"number": 1}));
    let diff = json!({"number": 1, "diff": "a".repeat(64), "diff_heads": ["b".repeat(40)]});
    let body = record_body(record.clone(), "text", Some(diff.clone()));
    let comments = [json!({"id": 1, "user": "github-actions[bot]", "body": body,
                           "created_at": "2026-09-11T10:00:00Z", "updated_at": "2026-09-11T10:00:00Z"})];
    assert_py(&parsed(&comments).unwrap().state, record);
    assert_py(&diff_of(&comments, 1).unwrap(), diff);
}

#[test]
fn the_holder_is_the_comment_the_record_was_read_from() {
    // Two record comments; the older one was written last.
    // The record is read from it, and it is the one kept.
    let older_written_last = json!({"id": 1, "user": "github-actions[bot]",
        "created_at": "2026-09-10T00:00:00Z", "updated_at": "2026-09-11T11:00:00Z",
        "edited_at": "2026-09-11T11:00:00Z", "edited_by": "github-actions[bot]",
        "body": record_body(state(json!({})), "old text", None)});
    let newer = json!({"id": 2, "user": "github-actions[bot]",
        "created_at": "2026-09-10T12:00:00Z", "updated_at": "2026-09-10T12:00:00Z",
        "body": record_body(state(json!({"ready_since": "2026-09-10T12:00:00Z"})), "other text", None)});
    assert_eq!(comment_id(&parsed(&[newer, older_written_last])), Some(1));
}

#[test]
fn the_last_marker_pair_is_the_engines() {
    // A quoted example in a fence is not the block.
    let quoted = format!("```\n{CHECKLIST_START}\nexample\n{CHECKLIST_END}\n```");
    let real = format!("{CHECKLIST_START}\nreal\n{CHECKLIST_END}");
    let body = py(json!(format!("{quoted}\n\n{real}")));
    assert_eq!(
        current_checklist(&body).unwrap().as_deref(),
        Some(real.as_str())
    );
    let no_end = py(json!(format!("{CHECKLIST_START}\nno end")));
    assert_eq!(current_checklist(&no_end).unwrap(), None);
}

#[test]
fn a_fenced_example_below_the_block_is_not_the_block() {
    let real = format!("{CHECKLIST_START}\nreal\n{CHECKLIST_END}");
    let fenced = format!("```\n{CHECKLIST_START}\nexample\n{CHECKLIST_END}\n```");
    assert_eq!(
        split_checklist(&format!("{real}\n\n{fenced}")).1,
        Some(real.as_str())
    );
    // A fenced example alone is no block: the first write appends.
    assert_eq!(split_checklist(&fenced).1, None);
    let reversed = format!("{CHECKLIST_END}\nreversed\n{CHECKLIST_START}");
    assert_eq!(split_checklist(&reversed).1, None);
}

#[test]
fn an_extra_end_marker_after_the_block_is_the_authors() {
    // The block ends at the first end marker after it; whatever follows is
    // someone else's and is kept as the text after the block.
    let real = format!("{CHECKLIST_START}\nreal\n{CHECKLIST_END}");
    let body = format!("author\n\n{real}\nKEEP ME\n{CHECKLIST_END}");
    let (head, block, tail) = split_checklist(&body);
    assert_eq!(head, "author\n\n");
    assert_eq!(block, Some(real.as_str()));
    assert_eq!(tail, format!("\nKEEP ME\n{CHECKLIST_END}"));
    // Line breaks Python splits on beyond `\n` still delimit fences, and
    // offsets count across characters outside ASCII.
    let fence_after_form_feed = format!("é\u{c}```\n{CHECKLIST_START}\nx\n{CHECKLIST_END}\n```");
    assert_eq!(split_checklist(&fence_after_form_feed).1, None);
    // An empty or missing description has no block; one that is not text
    // has no lines, as Python's `splitlines` does not exist on it.
    assert_eq!(current_checklist(&PyValue::None).unwrap(), None);
    assert!(matches!(
        current_checklist(&py(json!(5))),
        Err(ReadError::Exception { .. })
    ));
}

#[test]
fn the_record_comment_is_written_as_python_wrote_it_in_every_harvested_case() {
    // The fixtures above build records with the port's own writer; this
    // holds that writer to every record comment Python's suite wrote.
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../conformance/functions/github.GitHub.state_comment_body");
    let mut cases = 0;
    for entry in std::fs::read_dir(&directory).expect("the harvested cases") {
        let path = entry.expect("a directory entry").path();
        let case = py_loads(&std::fs::read_to_string(&path).expect("a case")).expect("JSON");
        let inputs = field(&case, "inputs");
        let diff = match field(inputs, "diff") {
            PyValue::None => None,
            diff => Some(diff),
        };
        let written = pr_hygiene_engine::evidence::records::state_comment_body(
            field(inputs, "state"),
            text(field(inputs, "body")),
            diff,
        )
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert_eq!(written, text(field(&case, "output")), "{}", path.display());
        cases += 1;
    }
    assert!(cases >= 40, "{cases} cases");
}
