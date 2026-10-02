//! The tests of `pr_review/tests/test_policy.py` that test the policy's
//! helpers directly; what they assert about `evaluate` itself the corpus
//! holds. Each keeps its Python name, which says the behaviour.

use crate::support::{
    at, comment, fixture, is_value_error, rabbit, s, set, value, Rabbit, HEAD, NOW,
};
use pr_hygiene_engine::policy::{
    admit, diff_print, finding_blocks, finding_severities, fingerprint, governs, receipt_print,
    validate_policy,
};
use pr_hygiene_engine::pycompat::{PyErr, PyInt, PyValue};
use serde_json::json;
use std::collections::BTreeSet;

fn strs(items: &[&str]) -> PyValue {
    value(json!(items))
}

fn print(body: &str) -> Option<String> {
    receipt_print(&comment(1, body)).unwrap()
}

// --- which branches a policy governs -------------------------------------------

#[test]
fn target_branches_are_patterns_like_the_repositorys_branch_rules() {
    // Platform renames its development branches every release; a list of
    // names went stale the day they changed, and every pull request on the
    // new branches silently left the policy.
    let (mut p, _) = fixture();
    set(&mut p, &["target_branches"], strs(&["v*-dev"]));
    validate_policy(&p).unwrap();
    for base in ["v4.2-dev", "v5.1-dev", "v6.0-dev"] {
        assert!(governs(&p, &s(base)).unwrap(), "{base}");
    }
    // As in GitHub's own branch patterns, `*` stops at a slash.
    for base in ["master", "feat/v5-dev", "v5/x-dev", "v5.1-dev-old"] {
        assert!(!governs(&p, &s(base)).unwrap(), "{base}");
    }
    // A plain name still means exactly that branch.
    set(&mut p, &["target_branches"], strs(&["develop"]));
    assert!(governs(&p, &s("develop")).unwrap());
    assert!(!governs(&p, &s("developer")).unwrap());
    // A pull request with no known base is never governed, whatever the pattern.
    set(&mut p, &["target_branches"], strs(&["*"]));
    assert!(!governs(&p, &PyValue::None).unwrap());
    assert!(!governs(&p, &s("")).unwrap());
}

#[test]
fn only_the_star_is_a_pattern() {
    // GitHub's rules also know `?`, `[...]` and `**`; read literally they
    // would match nothing and quietly take a branch out of the policy, so
    // a policy using them is refused rather than half-honoured.
    let (mut p, _) = fixture();
    for target in ["v?-dev", "v[45]-dev", "release/**", "v{4,5}-dev", "v\\-dev"] {
        set(&mut p, &["target_branches"], strs(&[target]));
        assert!(is_value_error(validate_policy(&p)), "{target}");
    }
}

// --- who may be named where ----------------------------------------------------

#[test]
fn a_machine_author_is_a_github_handle_like_any_other() {
    let (mut p, _) = fixture();
    set(&mut p, &["bot_authors"], strs(&["infraclaw-dash"]));
    validate_policy(&p).unwrap();
    let long = "a".repeat(40);
    for bad in [
        json!("not a handle"),
        json!(""),
        json!("-leading"),
        json!(long),
        json!(1),
        json!(["x"]),
    ] {
        set(&mut p, &["bot_authors"], value(json!([bad])));
        assert!(is_value_error(validate_policy(&p)), "{bad}");
    }
}

#[test]
fn a_machine_author_that_owns_an_area_is_refused_by_the_policy() {
    // It would need neither an attestation nor an approval: the owner
    // exemption and the stand-in together merge a pull request nobody has
    // read. Handles are case-insensitive, and so is the refusal.
    let (mut p, _) = fixture();
    set(&mut p, &["bot_authors"], strs(&["infraclaw-dash"]));
    validate_policy(&p).unwrap();
    for place in [
        &["areas", "0", "owners"][..],
        &["areas", "0", "reviewers"],
        &["fallback", "owners"],
        &["fallback", "reviewers"],
    ] {
        for spelling in ["infraclaw-dash", "INFRACLAW-DASH"] {
            let mut q = p.clone();
            if let PyValue::List(items) = at(&mut q, place) {
                items.push(s(spelling));
            }
            assert!(is_value_error(validate_policy(&q)), "{place:?} {spelling}");
        }
    }
}

#[test]
fn required_bots_must_name_known_producers() {
    let (p, _) = fixture();
    for bots in [
        json!(["thepastaclaw", "thepastaclaw"]),
        json!(["dependabot"]),
        json!("thepastaclaw"),
        json!([null]),
    ] {
        let mut q = p.clone();
        set(&mut q, &["required_bots"], value(bots.clone()));
        assert!(is_value_error(validate_policy(&q)), "{bots}");
    }
    let mut q = p.clone();
    set(&mut q, &["required_bots"], value(json!([])));
    validate_policy(&q).unwrap();
}

#[test]
fn a_bot_cannot_be_named_as_an_owner() {
    // A bot owning an area would satisfy the owner path with no human at
    // all; validate_policy is where that has to be refused.
    for bot in ["Copilot", "coderabbitai", "thepastaclaw"] {
        let (mut p, _) = fixture();
        set(&mut p, &["areas", "0", "owners"], strs(&[bot]));
        assert!(is_value_error(validate_policy(&p)), "{bot}");
    }
}

#[test]
fn integer_configuration_does_not_accept_float_lookalikes() {
    for key in ["version", "max_active_prs"] {
        let (mut p, _) = fixture();
        let lookalike = if key == "version" { 1.0 } else { 5.0 };
        set(&mut p, &[key], PyValue::Float(lookalike));
        assert!(is_value_error(validate_policy(&p)), "{key}");
    }
}

#[test]
fn validation_rejects_overlap_and_excluded_identity() {
    let (mut p, _) = fixture();
    validate_policy(&p).unwrap();
    if let PyValue::List(paths) = at(&mut p, &["areas", "0", "paths"]) {
        paths.push(s("packages/drive/nested/"));
    }
    assert!(is_value_error(validate_policy(&p)));
    let (mut p, _) = fixture();
    set(&mut p, &["areas", "0", "reviewers"], strs(&["strophy"]));
    assert!(is_value_error(validate_policy(&p)));
}

#[test]
fn whole_repository_owner_covers_root_files_and_cross_directory_rename() {
    let (mut p, _) = fixture();
    set(&mut p, &["areas", "0", "paths"], strs(&[""]));
    validate_policy(&p).unwrap();
}

#[test]
fn whole_repository_prefix_cannot_overlap_other_areas() {
    let (mut p, _) = fixture();
    if let PyValue::List(areas) = at(&mut p, &["areas"]) {
        areas.push(value(
            json!({"id": "whole", "paths": [""], "owners": ["whole"], "reviewers": []}),
        ));
    }
    match validate_policy(&p) {
        Err(PyErr::Value(error)) => assert!(error.to_string().contains("Overlapping"), "{error}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn named_area_without_owner_is_explicit_configuration_gap() {
    let (mut p, _) = fixture();
    set(&mut p, &["areas", "0", "owners"], strs(&[]));
    set(
        &mut p,
        &["areas", "0", "unresolved"],
        strs(&["Owner missing from source sheet"]),
    );
    validate_policy(&p).unwrap();
}

#[test]
fn empty_unresolved_area_never_emits_native_owner_suppression() {
    let (mut p, _) = fixture();
    set(&mut p, &["areas", "0", "paths"], strs(&[""]));
    set(&mut p, &["areas", "0", "owners"], strs(&[]));
    set(&mut p, &["areas", "0", "reviewers"], strs(&[]));
    set(
        &mut p,
        &["areas", "0", "unresolved"],
        strs(&["Owner missing"]),
    );
    validate_policy(&p).unwrap();
}

#[test]
fn empty_owner_without_explicit_gap_is_rejected() {
    for unresolved in [
        None,
        Some(json!([])),
        Some(json!([""])),
        Some(json!("missing")),
    ] {
        let (mut p, _) = fixture();
        set(&mut p, &["areas", "0", "owners"], strs(&[]));
        if let Some(unresolved) = &unresolved {
            set(
                &mut p,
                &["areas", "0", "unresolved"],
                value(unresolved.clone()),
            );
        }
        assert!(is_value_error(validate_policy(&p)), "{unresolved:?}");
    }
}

// --- the prints ------------------------------------------------------------------

#[test]
fn the_evidence_print_does_not_depend_on_which_route_read_a_comment() {
    // One route carries who last edited a comment and the other cannot.
    // With that in the print, a pull request with an edited comment read as
    // changed between the read and the write on every run, and was never
    // written to again. The time of the edit says something changed.
    let (_, mut pr) = fixture();
    set(
        &mut pr,
        &["comments", "0", "updated_at"],
        s("2026-09-11T11:30:00Z"),
    );
    let mut graphql = pr.clone();
    set(
        &mut graphql,
        &["comments", "0", "edited_by"],
        s("coderabbitai"),
    );
    set(
        &mut graphql,
        &["comments", "0", "edited_at"],
        s("2026-09-11T11:30:00Z"),
    );
    let mut rest = pr.clone();
    set(
        &mut rest,
        &["comments", "0", "edited_by"],
        s("(editor unknown)"),
    );
    assert_eq!(fingerprint(&graphql).unwrap(), fingerprint(&rest).unwrap());
    // Nor does knowing who edited it, or when, move the print from what it
    // was before either was read.
    assert_eq!(fingerprint(&graphql).unwrap(), fingerprint(&pr).unwrap());
    let mut later = pr.clone();
    set(
        &mut later,
        &["comments", "0", "updated_at"],
        s("2026-09-11T12:00:00Z"),
    );
    assert_ne!(
        fingerprint(&pr).unwrap(),
        fingerprint(&later).unwrap(),
        "an edit is still noticed"
    );
}

#[test]
fn the_print_is_not_part_of_the_evidence_print() {
    // It is this controller's own note about the evidence; in the print it
    // would differ between the read and the write.
    let (_, pr) = fixture();
    let mut with_print = pr.clone();
    set(
        &mut with_print,
        &["controller_diff"],
        value(json!({"diff": "a".repeat(64)})),
    );
    let mut without = pr.clone();
    set(&mut without, &["controller_diff"], PyValue::None);
    assert_eq!(
        fingerprint(&with_print).unwrap(),
        fingerprint(&without).unwrap()
    );
}

#[test]
fn the_build_is_not_evidence_that_can_change_under_a_write() {
    // publish re-reads and bails when the print moves; on a pull request
    // with 45 checks one finishing mid-run would strand it.
    let (_, mut pr) = fixture();
    let original = fingerprint(&pr).unwrap();
    set(&mut pr, &["build"], s("failed"));
    assert_eq!(original, fingerprint(&pr).unwrap());
}

#[test]
fn fingerprint_ignores_controller_effects_but_not_evidence() {
    let (_, mut pr) = fixture();
    let original = fingerprint(&pr).unwrap();
    set(&mut pr, &["labels"], strs(&["ready-for-human"]));
    set(&mut pr, &["requested_reviewers"], strs(&["reviewer"]));
    if let PyValue::List(comments) = at(&mut pr, &["comments"]) {
        comments.push(value(json!({
            "user": "github-actions[bot]",
            "body": "<!-- platform-pr-review-state-v1 -->",
        })));
    }
    assert_eq!(original, fingerprint(&pr).unwrap());
    set(&mut pr, &["reviews", "0", "state"], s("CHANGES_REQUESTED"));
    assert_ne!(original, fingerprint(&pr).unwrap());
}

#[test]
fn controller_comment_creation_does_not_invalidate_evidence() {
    let (_, mut pr) = fixture();
    let before = fingerprint(&pr).unwrap();
    set(&mut pr, &["controller_comment_id"], value(json!(55)));
    assert_eq!(fingerprint(&pr).unwrap(), before);
}

#[test]
fn asking_a_bot_to_review_is_not_mistaken_for_changed_evidence() {
    // From test_bot_timeouts.py: a nudge is this controller's own comment.
    let (_, mut pr) = fixture();
    let before = fingerprint(&pr).unwrap();
    if let PyValue::List(comments) = at(&mut pr, &["comments"]) {
        comments.push(value(json!({
            "user": "github-actions[bot]", "created_at": NOW, "updated_at": NOW,
            "body": format!("<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={HEAD} -->\n@thepastaclaw review"),
        })));
    }
    assert_eq!(fingerprint(&pr).unwrap(), before);
}

fn files(items: serde_json::Value) -> PyValue {
    value(json!({ "files": items }))
}

#[test]
fn a_read_that_cannot_say_how_a_file_changed_is_not_carried() {
    // Content without a patch says what the file holds and not what it took
    // to get there, and keeping your own side of a conflict is exactly the
    // case those two disagree about.
    let pr = files(json!([{"filename": "a.rs", "status": "modified", "content": "c".repeat(40)}]));
    assert_eq!(diff_print(&pr).unwrap(), None);
}

#[test]
fn a_new_file_needs_no_patch_to_be_carried() {
    // Nothing the base did can hide in the patch of a file the base does not
    // have; the base acquiring that path cannot pass in silence.
    let added = json!({"filename": "golden.bin", "status": "added", "content": "c".repeat(40), "shape": "added"});
    let print = |file: serde_json::Value| diff_print(&files(json!([file]))).unwrap();
    assert!(print(added.clone()).is_some());
    let mut acquired = added.clone();
    acquired["status"] = json!("modified");
    acquired["shape"] = json!("f".repeat(64));
    assert_ne!(print(added.clone()), print(acquired));
    // With the same content it leaves the listing, which moves the print.
    assert_ne!(print(added.clone()), diff_print(&files(json!([]))).unwrap());
    let mut rewritten = added.clone();
    rewritten["content"] = json!("d".repeat(40));
    assert_ne!(print(added), print(rewritten), "a new blob is new work");
}

#[test]
fn a_change_with_no_content_is_not_carried() {
    // A mode bit or a type change shows as an entry with nothing in it.
    let pr = files(json!([{"filename": "a.sh", "status": "modified", "shape": "x".repeat(64)}]));
    assert_eq!(diff_print(&pr).unwrap(), None);
}

// --- what a receipt said ---------------------------------------------------------

#[test]
fn the_checks_are_read_as_a_set_and_their_explanations_not_at_all() {
    // It reorders its own table between two writes of the same report, and
    // rewords the column explaining a verdict while the verdict stands.
    let first = rabbit(Rabbit {
        rows: Some(vec![
            ("Title check", "\u{2705} Passed"),
            ("Docstring Coverage", "\u{2705} Passed"),
        ]),
        ..Default::default()
    });
    let same = rabbit(Rabbit {
        rows: Some(vec![
            ("Docstring Coverage", "\u{2705} Passed"),
            ("Title check", "\u{2705} Passed"),
        ]),
        why: "It identifies the change well.",
        ..Default::default()
    });
    assert_eq!(print(&first), print(&same));
    let flipped = rabbit(Rabbit {
        rows: Some(vec![
            ("Title check", "\u{274c} Failed"),
            ("Docstring Coverage", "\u{2705} Passed"),
        ]),
        ..Default::default()
    });
    assert_ne!(print(&first), print(&flipped));
}

#[test]
fn a_fold_is_bookkeeping_only_when_it_is_named_as_such() {
    // A fold whose title merely mentions commits is a place a finding could
    // hide where nothing would read it.
    let plain = rabbit(Rabbit::default());
    let fold = |summary: &str, inside: &str| {
        plain.replace(
            "<!-- final_review_risk_start -->",
            &format!("<details><summary>{summary}</summary>\n{inside}\n</details>\n<!-- final_review_risk_start -->"),
        )
    };
    for summary in [
        "\u{1f4e5} Commits with problems",
        "\u{1f9f9} Nitpick comments (1)",
        "Files selected and rejected",
    ] {
        assert_ne!(
            print(&plain),
            print(&fold(summary, "Do not merge.")),
            "{summary}"
        );
    }
    // The ones it really names stay dropped.
    for summary in [
        "\u{2699}\u{fe0f} Run configuration",
        "\u{1f4e5} Commits",
        "\u{1f4d2} Files selected for processing (17)",
    ] {
        assert_eq!(
            print(&plain),
            print(&fold(summary, "Run ID: 9f1c")),
            "{summary}"
        );
    }
}

#[test]
fn a_verdict_that_is_not_passed_keeps_what_it_asks_for() {
    // Passed is the one verdict that says there is nothing to do; a finding
    // beside any other is read.
    let asks = "Remove the unchecked index before merging.";
    for verdict in [
        "\u{274c} Failed",
        "\u{26a0}\u{fe0f} Warning",
        "\u{2753} Inconclusive",
        "\u{23ed}\u{fe0f} Skipped",
        "\u{1f195} Whatever it invents",
    ] {
        let plain = rabbit(Rabbit {
            checks: verdict,
            why: "It could not run.",
            ..Default::default()
        });
        let worse = rabbit(Rabbit {
            checks: verdict,
            why: asks,
            ..Default::default()
        });
        assert_ne!(print(&plain), print(&worse), "{verdict}");
    }
}

#[test]
fn a_pipe_in_a_name_it_echoes_does_not_hide_the_verdict() {
    let rows = Some(vec![("Security | Key handling", "\u{274c} Failed")]);
    let plain = rabbit(Rabbit {
        rows: rows.clone(),
        why: "It reads well.",
        ..Default::default()
    });
    let worse = rabbit(Rabbit {
        rows,
        why: "Forge a session token.",
        ..Default::default()
    });
    assert_ne!(print(&plain), print(&worse));
}

#[test]
fn the_rule_under_a_heading_is_not_what_it_said() {
    // It is redrawn as wide as the widest cell under it.
    let plain = rabbit(Rabbit::default());
    let wider = plain.replace("| :---: |", "| :-------------: |");
    assert_ne!(plain, wider);
    assert_eq!(print(&plain), print(&wider));
}

#[test]
fn what_a_failed_check_asks_for_is_read_and_a_passing_one_is_not() {
    let passing = rabbit(Rabbit::default());
    let reworded = rabbit(Rabbit {
        why: "Reads well enough.",
        ..Default::default()
    });
    assert_eq!(print(&passing), print(&reworded));
    let failed = rabbit(Rabbit {
        checks: "\u{274c} Failed",
        why: "Out of scope changes.",
        ..Default::default()
    });
    let worse = rabbit(Rabbit {
        checks: "\u{274c} Failed",
        why: "Remove the unchecked index before merging.",
        ..Default::default()
    });
    assert_ne!(print(&failed), print(&worse));
}

#[test]
fn line_endings_are_not_what_it_said() {
    let plain = rabbit(Rabbit::default());
    assert_eq!(print(&plain), print(&plain.replace('\n', "\r\n")));
}

#[test]
fn a_marker_in_text_it_echoes_cannot_delete_the_report() {
    // An opening marker inside an echoed path, paired with the real closing
    // one far below, would delete the findings and everything between.
    let plain = rabbit(Rabbit::default());
    let planted = plain.replace("| `a.rs` |", "| `a<!-- tips_start -->b.rs` |");
    // A second one alone on its line: seen twice, neither is deleted.
    let twice = plain.replace(
        "<!-- walkthrough_start -->",
        "<!-- walkthrough_start -->\n<!-- tips_start -->",
    );
    let critical = |body: &str| body.replace("**Merge Risk:** Minimal", "**Merge Risk:** Critical");
    assert_ne!(print(&twice), print(&critical(&twice)));
    assert_ne!(print(&planted), print(&critical(&planted)));
}

#[test]
fn what_it_is_doing_is_not_what_it_found() {
    // Its capacity, a review it has not run, one in progress or paused: each
    // is rewritten while the report stands, and says nothing about the code.
    let plain = rabbit(Rabbit::default());
    for what in [
        "rate limited",
        "skip review",
        "review in progress",
        "review paused",
        "tweet message",
    ] {
        let noisy = format!(
            "<!-- This is an auto-generated comment: {what} by coderabbit.ai -->\n\
             > Next included review available in 27 minutes. Run ID: 9f1c.\n\
             <!-- end of auto-generated comment: {what} by coderabbit.ai -->\n{plain}"
        );
        assert_eq!(print(&plain), print(&noisy), "{what}");
    }
}

#[test]
fn a_tool_of_its_own_that_would_not_run_says_why() {
    // It names the file and line that stopped the tool: a finding about the
    // author's code, which dropping the notice would drop with it.
    let notice = |inside: &str| {
        format!(
            "<!-- This is an auto-generated comment: all tool run failures by coderabbit.ai -->\n\
             <details><summary>\u{1f527} Biome</summary>\n{inside}\n</details>\n\
             <!-- end of auto-generated comment: all tool run failures by coderabbit.ai -->\n"
        )
    };
    let plain = rabbit(Rabbit::default());
    let a = plain.clone() + &notice("Line 126: Expected an array.");
    let b = plain + &notice("Line 203: Expected an array; plus 41 more in auth.ts.");
    assert_ne!(print(&a), print(&b));
}

#[test]
fn the_fold_it_signs_with_a_letter_is_still_a_fold() {
    // ℹ is a letter, so a rule that skipped everything but letters could
    // never reach the name behind it; that fold carries an allowance that
    // changes on its own.
    let fold = |remaining: &str| {
        format!(
            "<details><summary>\u{2139}\u{fe0f} Recent review info</summary>\n\
             **Included review availability:** {remaining} remain after this review.\n</details>\n"
        )
    };
    let plain = rabbit(Rabbit::default());
    let a = plain.clone() + &fold("0");
    let b = plain.clone() + &fold("1");
    assert_eq!(print(&a), print(&b));
    assert_eq!(print(&plain), print(&a));
}

#[test]
fn a_pipe_it_escaped_is_text_not_the_end_of_a_cell() {
    // A configured check name carrying an escaped pipe and a passed mark put
    // that mark in the column this reads, hiding the real verdict.
    let name = "Team sanity \\| \u{2705} ok";
    let rows = Some(vec![(name, "\u{26a0}\u{fe0f} Warning")]);
    let tame = rabbit(Rabbit {
        rows: rows.clone(),
        why: "Coverage is fine.",
        ..Default::default()
    });
    let real = rabbit(Rabbit {
        rows,
        why: "Signs with a disabled key. Do not merge.",
        ..Default::default()
    });
    assert_ne!(print(&tame), print(&real));
}

#[test]
fn a_box_a_person_ticks_is_not_the_bot_speaking() {
    let item = "- [ ] <!-- {\"checkboxId\":\"585bb3f6\"} --> Fix all pre-merge checks with AI\n";
    let body = rabbit(Rabbit::default()) + item;
    assert_eq!(print(&body), print(&body.replace("- [ ]", "- [x]")));
    // The item goes whole, as the bot adds and removes it; the same marker
    // anywhere else is somebody else's, and only the marker goes.
    assert_eq!(print(&rabbit(Rabbit::default())), print(&body));
    let row = rabbit(Rabbit::default())
        + "| Key handling | \u{274c} Failed | Forge a token. | <!-- {\"checkboxId\":\"z\"} -->\n";
    let tame = rabbit(Rabbit::default())
        + "| Key handling | \u{274c} Failed | It reads well. | <!-- {\"checkboxId\":\"z\"} -->\n";
    assert_ne!(print(&row), print(&tame));
}

#[test]
fn a_finding_written_inside_a_fold_is_still_a_finding() {
    let plain = rabbit(Rabbit::default());
    let nitpicks = plain.replace(
        "<!-- recent_review_end -->",
        "<details><summary>\u{1f9f9} Nitpick comments (2)</summary>\n\
         Unchecked index; the node aborts.\n</details>\n<!-- recent_review_end -->",
    );
    assert_ne!(print(&plain), print(&nitpicks));
}

#[test]
fn what_it_found_is_read_and_which_run_found_it_is_not() {
    let plain = print(&rabbit(Rabbit::default()));
    let again = print(&rabbit(Rabbit {
        run: "a-different-run",
        ..Default::default()
    }));
    assert_eq!(plain, again, "which run walked the code");
    let posted = print(&rabbit(Rabbit {
        found: "Actionable comments posted: 2",
        ..Default::default()
    }));
    assert_ne!(plain, posted, "what the run turned up");
}

#[test]
fn findings_this_cannot_read_are_not_read_as_nothing() {
    // A findings block with no end leaves the findings invisible while the
    // checks around them would still print; nothing is compared instead.
    let whole = rabbit(Rabbit::default());
    assert!(print(&whole).is_some());
    let torn = whole.replace(
        "<!-- final_review_risk_end -->",
        "<!-- final_review_risk_finished -->",
    );
    assert_eq!(print(&torn), None);
    assert_eq!(print(&(torn + "\nCRITICAL: do not merge.")), None);
}

#[test]
fn a_finding_shaped_like_a_table_row_is_not_a_table_row() {
    // The same words: in one they are the finding, in the other a check.
    let (start, end) = (
        "<!-- final_review_risk_start -->",
        "<!-- final_review_risk_end -->",
    );
    let a = format!("{start}A{end}\n|B|\u{2705}|\n");
    let b = format!("{start}A\nB|\u{2705}{end}\n");
    assert_ne!(print(&a), print(&b));
}

#[test]
fn what_two_comments_said_is_told_apart() {
    // Two reports are two reports even when they say the same thing.
    let body = rabbit(Rabbit::default());
    assert_ne!(
        receipt_print(&comment(1, &body)).unwrap(),
        receipt_print(&comment(2, &body)).unwrap()
    );
}

#[test]
fn a_comment_that_states_nothing_has_nothing_to_compare() {
    assert_eq!(receipt_print(&comment(1, "thanks!")).unwrap(), None);
    assert_eq!(receipt_print(&comment(2, "")).unwrap(), None);
}

// --- review slots ----------------------------------------------------------------

fn numbered(pr: &PyValue, count: i64) -> Vec<PyValue> {
    (1..=count)
        .map(|n| {
            let mut copy = pr.clone();
            set(&mut copy, &["number"], value(json!(n)));
            copy
        })
        .collect()
}

fn slots(policy: &PyValue, prs: Vec<PyValue>) -> BTreeSet<i64> {
    admit(policy, &PyValue::List(prs.into()), &s(NOW))
        .unwrap()
        .keys()
        .map(|n: &PyInt| n.as_i64().unwrap())
        .collect()
}

#[test]
fn sixth_waits_without_blocking_admitted_five() {
    // An admitted pull request keeps its slot; the newest unadmitted one is
    // the one that waits.
    let (p, pr) = fixture();
    let mut prs = numbered(&pr, 6);
    set(
        &mut prs[5],
        &["controller_state"],
        value(json!({"admitted_at": "2026-09-10T01:00:00Z"})),
    );
    assert_eq!(slots(&p, prs), BTreeSet::from([1, 2, 3, 4, 6]));
}

#[test]
fn draft_releases_sticky_slot_and_retargeted_pr_is_excluded() {
    let (p, pr) = fixture();
    let mut prs = numbered(&pr, 6);
    set(&mut prs[0], &["draft"], PyValue::Bool(true));
    set(
        &mut prs[0],
        &["controller_state"],
        value(json!({"admitted_at": NOW})),
    );
    assert_eq!(slots(&p, prs.clone()), BTreeSet::from([2, 3, 4, 5, 6]));
    set(&mut prs[1], &["base"], s("another-branch"));
    assert!(!slots(&p, prs).contains(&2));
}

#[test]
fn reopened_pr_does_not_retain_slot_from_previous_open_cycle() {
    let (p, pr) = fixture();
    let mut prs = numbered(&pr, 6);
    set(
        &mut prs[5],
        &["controller_state"],
        value(json!({"admitted_at": "2026-09-10T01:00:00Z"})),
    );
    set(&mut prs[5], &["lifecycle_at"], s("2026-09-11T01:00:00Z"));
    assert_eq!(slots(&p, prs), BTreeSet::from([1, 2, 3, 4, 5]));
}

// --- the severity of a bot's finding --------------------------------------------

const PASTA_SUGGESTION: &str =
    "<!-- thepastaclaw-review v1 finding=1ae5c0d05709 dedupe=823d079e92be8910 -->\n\
    **\u{1f7e1} Suggestion: Add coverage for the ambiguous unpadded base64/base58 case**\n\n\
    The regression test uses a padded encoding, so it never reaches the ambiguous case.";
const PASTA_BLOCKING: &str =
    "<!-- thepastaclaw-review v1\nfinding=86cd4cfc5f6d dedupe=eabaeb4163b84d5f -->\n\
    **\u{1f534} Blocking: Do not require a live Tokio runtime for every later UI frame**\n\n\
    **Why:** `spawn_blocking` panics when no runtime is entered.";
const PASTA_NITPICK: &str = "<!-- thepastaclaw-review v1 finding=1 dedupe=2 -->\n\
    **\u{1f4ac} Nitpick: Document why the probe stays synchronous**\n\nA sentence would do.";
const RABBIT_MINOR: &str =
    "_\u{1f3af} Functional Correctness_ | _\u{1f7e1} Minor_ | _\u{26a1} Quick win_\n\n\
    **Reject an empty identifier.**\n\n_Note: this mirrors the parser above._\n\
    <!-- cr-comment:v1:1 -->";
const RABBIT_MAJOR: &str = "_\u{1f5c4}\u{fe0f} Data Integrity & Integration_ | _\u{1f7e0} Major_ | _\u{1f3d7}\u{fe0f} Heavy lift_\n\n\
    **Support `ECDSA_HASH160` in the external signer before selecting it.**";
const RABBIT_REFACTOR_MAJOR: &str =
    "_\u{1f6e0}\u{fe0f} Refactor suggestion_ | _\u{1f7e0} Major_\n\n**Extract the shared decoder.**";

fn rabbit_minor_then_major() -> String {
    format!(
        "{RABBIT_MINOR}\n\n---\n\n\
         _\u{1f4d0} Maintainability & Code Quality_ | _\u{1f7e0} Major_ | _\u{1f3d7}\u{fe0f} Heavy lift_\n\n\
         **Move state-transition protocol interpretation to Rust.**\n<!-- cr-comment:v1:2 -->"
    )
}

fn severities(author: &str, body: &str) -> Vec<String> {
    finding_severities(&s(author), &s(body)).unwrap()
}

/// `bot_thread(author, severities)`: an unresolved thread a bot opened.
fn bot_thread(author: &str, severities: &[String]) -> PyValue {
    value(json!({"id": 7, "author": author, "is_resolved": false,
                 "created_at": "2026-09-11T10:00:00Z", "severities": severities}))
}

fn blocks(author: &str, body: &str) -> bool {
    finding_blocks(&bot_thread(author, &severities(author, body))).unwrap()
}

#[test]
fn every_finding_in_an_opening_is_read() {
    // Reading the first heading alone lets a Major sit behind a Minor.
    assert_eq!(
        severities("coderabbitai", &rabbit_minor_then_major()),
        ["\u{1f7e1} Minor", "\u{1f7e0} Major"]
    );
    let grouped = format!("{PASTA_SUGGESTION}\n\n{PASTA_BLOCKING}");
    assert_eq!(
        severities("thepastaclaw", &grouped),
        ["\u{1f7e1} Suggestion", "\u{1f534} Blocking"]
    );
    // A later finding under a label nobody knows holds the thread too.
    let pasta = format!("{PASTA_SUGGESTION}\n\n**\u{1f7e3} Urgent: Something new**");
    let rabbit = format!(
        "{RABBIT_MINOR}\n\n---\n\n_\u{1f3af} Functional Correctness_ | _\u{1f7e3} Urgent_\n"
    );
    assert!(blocks("thepastaclaw", &pasta));
    assert!(blocks("coderabbitai", &rabbit));
}

#[test]
fn each_bot_is_read_in_its_own_heading_shape() {
    for (author, body, expected) in [
        ("thepastaclaw", PASTA_SUGGESTION, "\u{1f7e1} Suggestion"),
        ("thepastaclaw", PASTA_NITPICK, "\u{1f4ac} Nitpick"),
        // A bold "Why:" is not a heading, and a marker spanning lines is not text.
        ("thepastaclaw", PASTA_BLOCKING, "\u{1f534} Blocking"),
        // An italic line in the text is not a heading either.
        ("coderabbitai", RABBIT_MINOR, "\u{1f7e1} Minor"),
        ("coderabbitai[bot]", RABBIT_MAJOR, "\u{1f7e0} Major"),
        // "suggestion" in a CodeRabbit category is not a severity.
        ("coderabbitai", RABBIT_REFACTOR_MAJOR, "\u{1f7e0} Major"),
    ] {
        assert_eq!(severities(author, body), [expected], "{author}");
    }
}

#[test]
fn a_label_outside_a_heading_is_not_a_severity() {
    let quoted = format!(
        "{RABBIT_MAJOR}\n\n> _\u{1f3af} Functional Correctness_ | _\u{1f7e1} Minor_\n\nThe \u{1f7e1} Minor note above is stale."
    );
    assert_eq!(severities("coderabbitai", &quoted), ["\u{1f7e0} Major"]);
    let hidden = format!(
        "<!--\n_\u{1f3af} Functional Correctness_ | _\u{1f7e1} Minor_\n-->\n{RABBIT_MAJOR}"
    );
    assert_eq!(severities("coderabbitai", &hidden), ["\u{1f7e0} Major"]);
    let prose = "<!-- thepastaclaw-review v1 -->\nThis reads like a \u{1f7e1} Suggestion but has no heading.";
    assert!(severities("thepastaclaw", prose).is_empty());
}

#[test]
fn the_severity_is_read_not_the_kind_of_finding() {
    // Nitpick is the kind of finding beside the severity; taking it for the
    // severity made a Major read as optional.
    for (heading, holds) in [
        ("_\u{1f9f9} Nitpick_ | _\u{1f535} Trivial_", false),
        ("_\u{1f9f9} Nitpick_ | _\u{1f7e0} Major_", true),
        ("_\u{1f9f9} Nitpick_ | _\u{1f7e3} Something new_", true),
    ] {
        assert_eq!(
            blocks("coderabbitai", &format!("{heading}\n\n**Rename it.**")),
            holds,
            "{heading}"
        );
    }
}

#[test]
fn a_blocker_this_cannot_parse_still_holds_the_thread() {
    // One heading parsed is not every heading parsed.
    for second in [
        " _\u{1f3af} Functional Correctness_ | _\u{1f7e0} Major_",
        "_\u{26a0}\u{fe0f} Potential issue_ | **\u{1f7e0} Major**",
        "> _\u{1f3af} Functional Correctness_ | _\u{1f7e0} Major_",
        "_\u{1f7e1} Minor_ | _\u{1f7e0} Major_",
        "`<!--` opens a comment in the sample.\n\n_\u{1f3af} Functional Correctness_ | _\u{1f7e0} Major_",
    ] {
        let body = format!("{RABBIT_MINOR}\n\n---\n\n{second}\n\n**Fix the decoder.**\n<!-- cr-comment:v1:2 -->");
        assert!(blocks("coderabbitai", &body), "{second}");
    }
    let body = format!("{PASTA_SUGGESTION}\n\n### \u{1f534} Blocking: the fence hides the heading");
    assert!(blocks("thepastaclaw", &body));
}

#[test]
fn each_bot_is_held_to_its_own_labels() {
    // A label one bot uses is unknown in the other's heading, and unknown
    // holds the thread.
    assert!(blocks(
        "thepastaclaw",
        "<!-- thepastaclaw-review v1 -->\n**\u{1f7e1} Minor: Rename it**"
    ));
    assert!(blocks(
        "coderabbitai",
        "_\u{1f3af} Functional Correctness_ | _\u{1f7e1} Suggestion_\n\n**Rename it.**"
    ));
}

#[test]
fn a_heading_with_windows_line_endings_is_still_a_heading() {
    assert_eq!(
        severities("coderabbitai", &RABBIT_MINOR.replace('\n', "\r\n")),
        ["\u{1f7e1} Minor"]
    );
}

#[test]
fn a_heading_without_a_known_severity_is_kept_so_it_blocks() {
    let labels = severities(
        "coderabbitai",
        "_\u{1f3af} Functional Correctness_ | _\u{26a1} Quick win_\n\n**Fix it.**",
    );
    assert_eq!(labels.len(), 1);
    assert!(finding_blocks(&bot_thread("coderabbitai[bot]", &labels)).unwrap());
}

#[test]
fn a_person_has_no_severity() {
    assert!(severities("reviewer", PASTA_SUGGESTION).is_empty());
}

#[test]
fn a_bots_suggestion_alone_does_not_hold_the_thread() {
    // The basis of `test_a_bots_suggestion_does_not_hold_the_pull_request`:
    // a suggestion is the author's to take or leave.
    assert!(!blocks("thepastaclaw", PASTA_SUGGESTION));
    assert!(!blocks("coderabbitai", RABBIT_MINOR));
    assert!(blocks("coderabbitai", RABBIT_MAJOR));
    // No severity recorded at all is a thread that holds.
    assert!(finding_blocks(&bot_thread("coderabbitai", &[])).unwrap());
}
