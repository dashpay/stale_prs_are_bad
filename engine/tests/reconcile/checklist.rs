//! `test_checklist.py` (`PublishTests`, `RecordTests`, `SecondReviewTests`,
//! `StaleMarkTests`) as scenarios on the fake: what a publication writes
//! and what it leaves alone, read back by the next run.

use crate::fake::*;
use crate::scene::*;
use crate::support::*;
use pr_hygiene_engine::policy::{diff_print, MOVE_MARKER};
use pr_hygiene_engine::reconcile::{
    checklist_block, context_fingerprint, move_text, state_record, visible, POINTER,
};

const BUILD_RUNNING: &str = "2026-09-11T15:00:00Z";

/// A verdict with its `state` replaced, as Python's `dict(result, state=…)`.
fn with_state(result: &PyValue, state: &str) -> PyValue {
    let mut changed = result.clone();
    if let PyValue::Dict(fields) = &mut changed {
        fields.insert("state".into(), s(state));
    }
    changed
}

/// The engine's comment holding `record` and `words`, written at `at`.
fn engine_comment(
    id: i64,
    record: &PyValue,
    words: &str,
    diff: Option<&PyValue>,
    at: &str,
) -> Comment {
    Comment::new(id, ENGINE, &record_comment(record, words, diff), at)
}

fn moved(result: &PyValue) -> String {
    move_text(result).expect("words").expect("a move")
}

fn block_of(result: &PyValue) -> String {
    checklist_block(result).expect("words").expect("a block")
}

/// Run until the pull request is settled: two runs, the second writing
/// what only a record of the first can tell it.
fn settle(scene: &mut Scene, policy: &PyValue) {
    scene.sync_pr(policy, 1);
    scene.sync_pr(policy, 1);
    scene.fake.forget_calls();
    scene.log.clear();
}

/// The comment ids written over, and whether any comment was posted.
fn comment_writes(scene: &Scene) -> (Vec<String>, usize) {
    let patched = scene
        .wrote(Method::Patch, "issues/comments/")
        .iter()
        .map(|w| w.route.rsplit('/').next().unwrap_or("").to_owned())
        .collect();
    (
        patched,
        scene.wrote(Method::Post, "issues/1/comments").len(),
    )
}

/// The words of the record comment written, by whichever route.
fn written_words(scene: &Scene) -> Vec<String> {
    scene
        .fake
        .written
        .iter()
        .filter(|w| w.route.contains("comments"))
        .filter_map(|w| w.body.as_ref())
        .map(|body| visible(text(field(body, "body"))))
        .collect()
}

#[test]
fn a_move_is_announced_once_with_the_record_and_the_block_written() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    let result = &run.verdicts[0];
    assert_eq!(text(field(result, "state")), "waiting-self-review");
    let described = scene.wrote(Method::Patch, "pulls/1");
    assert_eq!(described.len(), 1);
    assert!(text(described[0].field("body")).ends_with(&block_of(result)));
    let (patched, posted) = comment_writes(&scene);
    assert_eq!(
        (patched.len(), posted),
        (0, 1),
        "a new comment: that is what notifies"
    );
    let record = field(&scene.snapshot(&policy, 1), "controller_state").clone();
    assert_eq!(text(field(&record, "state")), "waiting-self-review");
    assert!(written_words(&scene)[0].starts_with(&format!(
        "{MOVE_MARKER} state=waiting-self-review sha={HEAD} -->"
    )));
}

fn rabbit_receipt(risk: &str, finding: &str, banner: &str) -> String {
    let covered =
        format!(r#"{{"sourceCommitId":"{HEAD}","coveredCommitId":"{HEAD}","kind":"reviewed"}}"#);
    let banner = if banner.is_empty() {
        String::new()
    } else {
        format!("<!-- review_stack_entry_start -->\n{banner}<!-- review_stack_entry_end -->\n")
    };
    format!(
        "{banner}<!-- final_review_risk_start -->\n**Merge Risk:** {risk}\n\
         <!-- final_review_risk_coverage:{covered} -->\n{finding}<!-- final_review_risk_end -->"
    )
}

#[test]
fn what_the_bot_said_is_written_down_so_an_edit_can_be_told_from_a_report() {
    // Without it there is nothing to compare the next read against, and
    // every rewrite of the bot's comment reads as a new report.
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.push(Comment::new(
        21,
        "coderabbitai[bot]",
        &rabbit_receipt("Minimal", "", ""),
        NOW,
    ));
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    let receipts = field(&run.verdicts[0], "receipts").clone();
    assert!(receipts.truthy(), "the bot reported; what it said is known");
    let read_back = field(&scene.snapshot(&policy, 1), "controller_diff").clone();
    assert_eq!(dump(field(&read_back, "receipts")), dump(&receipts));
}

/// The bartek pull request's one file, with what it holds and its patch.
fn with_content(fake: &mut Fake) {
    fake.pr(1).files = vec![json!({"filename": "packages/swift-sdk/Sources/a.swift",
        "status": "modified", "sha": "c".repeat(40), "patch": "@@ -1 +1 @@\n-a\n+b"})];
}

#[test]
fn the_diff_is_written_beside_the_record_not_inside_it() {
    // The next run reads it to tell a push that only moved the base from one
    // that changed the work. The record's schema is an exact set of keys.
    let (policy, mut fake) = bartek();
    with_content(&mut fake);
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    let read_back = scene.snapshot(&policy, 1);
    let record = field(&read_back, "controller_state");
    assert!(
        !matches!(record, PyValue::Dict(f) if f.contains_key("diff")),
        "the record schema is exact"
    );
    let diff = field(&read_back, "controller_diff");
    assert_eq!(dump(field(diff, "diff_heads")), format!(r#"["{HEAD}"]"#));
    assert_eq!(dump(field(diff, "number")), "1");
}

#[test]
fn a_comment_carrying_both_records_is_left_alone_when_it_agrees() {
    // The words are read back with the records stripped off. Stripping
    // only the first left the second in the text, so the comment was
    // rewritten on every run.
    let (policy, mut fake) = bartek();
    with_content(&mut fake);
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
    assert!(scene.wrote(Method::Patch, "pulls/1").is_empty());
}

/// The bartek pull request as its author was told their move: the
/// verdict then, and its record.
fn announced(policy: &PyValue, scene: &mut Scene) -> (PyValue, PyValue) {
    let pr = scene.snapshot(policy, 1);
    let result = scene.verdict(policy, 1, &s(NOW));
    let record = state_record(&pr, &result, &"c".repeat(64)).expect("a record");
    (result, record)
}

#[test]
fn words_that_cannot_be_written_back_are_replaced_not_refused() {
    // A marker anywhere in the words is refused by the writer, and the
    // write that would repair the comment is the write that fails. The
    // pointer at the description says less, and says it.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let (result, record) = announced(&policy, &mut scene);
    let tampered = format!(
        "{}\n<!-- pr-hygiene-diff-v1 {{\"number\":1}} -->",
        record_comment(&record, &moved(&result), None)
    );
    assert!(visible(&tampered).contains("pr-hygiene-diff-v1"));
    scene.fake.pr(1).comments = vec![Comment::new(50, ENGINE, &tampered, NOW)];
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene).0, ["50"]);
    assert_eq!(written_words(&scene), [POINTER]);
}

#[test]
fn a_record_line_somebody_truncated_does_not_wedge_the_pull_request() {
    // Four characters deleted from the engine's own comment, on a pull
    // request whose state posts no words: the write that would repair the
    // comment must not be the one that fails.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let (result, record) = announced(&policy, &mut scene);
    let body = record_comment(&record, &moved(&result), None);
    assert!(body.contains(MOVE_MARKER));
    let broken = body.replacen("\n\n", "\n<!-- pr-hygiene-diff-v1 {\"number\":1} \n\n", 1);
    scene.fake.pr(1).comments = vec![Comment::new(50, ENGINE, &broken, NOW)];
    scene.fake.forget_calls();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(
        text(field(&run.verdicts[0], "state")),
        "waiting-bots",
        "a state that posts no words"
    );
    let words = written_words(&scene);
    assert_eq!(words.len(), 1);
    assert!(
        !words[0].contains("pr-hygiene-diff-v1")
            && !words[0].contains("platform-pr-review-state-v1")
    );
}

#[test]
fn a_record_line_somebody_truncated_does_not_spin() {
    for body in [
        "<!-- pr-hygiene-diff-v1 {\"number\":1} \nwords",
        "<!-- platform-pr-review-state-v1 {\"number\":1} \nwords",
        "<!-- pr-hygiene-diff-v1",
    ] {
        let words = visible(body);
        assert!(
            !words.contains("pr-hygiene-diff-v1") && !words.contains("platform-pr-review-state-v1")
        );
    }
}

#[test]
fn the_second_verdict_sees_the_same_rate_limit_notice() {
    // The notice counts only as the bot's own word. When the second read
    // could not say who edited it, the verdict flipped between the reads.
    //
    // Python's test leaves thepastaclaw unreported, so its verdict waits on
    // the bots and the second read it guards is never made. Here
    // thepastaclaw has reported, so the waiver alone lets the pull request
    // go out for review, and the second verdict has to see it too.
    let (policy, mut fake) = bartek();
    fake.pr(1).reviews.push(review(
        7,
        "thepastaclaw",
        "COMMENTED",
        HEAD,
        "2026-09-11T10:00:00Z",
        &format!("<!-- thepastaclaw-review-phase v1 phase=final sha={HEAD} -->"),
    ));
    fake.pr(1).comments = vec![
        Comment::new(
            11,
            "coderabbitai[bot]",
            "<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->\n\
             Review limit reached\n\
             <!-- end of auto-generated comment: rate limited by coderabbit.ai -->",
            "2026-09-11T09:30:00Z",
        )
        .edited("coderabbitai[bot]", "2026-09-11T10:00:00Z"),
        Comment::new(
            12,
            "llbartekll",
            &format!("/self-reviewed {HEAD}"),
            "2026-09-11T13:00:00Z",
        ),
    ];
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert!(dump(field(&run.verdicts[0], "waived")).contains("coderabbitai"));
    assert_eq!(text(field(&run.verdicts[0], "state")), "ready-for-human");
    let held: Vec<_> = scene
        .statuses()
        .into_iter()
        .filter(|(_, d)| d.contains("reconciliation required"))
        .collect();
    assert!(held.is_empty(), "{held:?}");
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("pending".to_owned(), "ready-for-human".to_owned()),
        "the verdict, published after the second read"
    );
}

#[test]
fn the_second_verdict_sees_the_same_carried_review() {
    // Everything this pull request has was said about the commit before
    // it, and only the carried marker says those still count. The re-read
    // before the write must read it the same way.
    let old_head = "d".repeat(40);
    let (policy, mut fake) = bartek();
    with_content(&mut fake);
    for review in &mut fake.pr(1).reviews {
        review["commit_id"] = json!(old_head);
    }
    fake.pr(1).comments.push(Comment::new(
        9,
        "llbartekll",
        &format!("/self-reviewed {old_head}"),
        "2026-09-11T11:00:00Z",
    ));
    let mut scene = Scene::new(fake);
    let print = diff_print(&scene.snapshot(&policy, 1))
        .expect("a print")
        .expect("files with content");
    let diff = py(json!({"number": 1, "diff": print, "diff_heads": [old_head],
                         "diff_seen": "2026-09-11T09:00:00Z"}));
    let admitted = py(
        json!({"number": 1, "head": HEAD, "admitted_at": NOW, "ready_since": null,
        "state": "ready-to-merge", "version": 1, "evidence": "e".repeat(64), "context": "c".repeat(64)}),
    );
    scene
        .fake
        .pr(1)
        .comments
        .push(engine_comment(50, &admitted, POINTER, Some(&diff), NOW));
    scene.fake.forget_calls();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(
        text(field(&run.verdicts[0], "status")),
        "success",
        "{}",
        dump(field(&run.verdicts[0], "blockers"))
    );
    let held: Vec<_> = scene
        .statuses()
        .into_iter()
        .filter(|(_, d)| d.contains("reconciliation required"))
        .collect();
    assert!(held.is_empty(), "{held:?}");
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("success".to_owned(), "ready-to-merge".to_owned())
    );
}

#[test]
fn nothing_is_rewritten_when_description_labels_and_comment_already_agree() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    scene.sync_pr(&policy, 1);
    assert_eq!(scene.writes(), Vec::<String>::new());
}

#[test]
fn a_hand_ticked_box_or_a_wiped_block_is_repaired() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let ticked = scene
        .fake
        .pr(1)
        .body
        .replace("- [ ] Self-review", "- [x] Self-review");
    scene.fake.pr(1).body = ticked;
    scene.sync_pr(&policy, 1);
    assert_eq!(scene.wrote(Method::Patch, "pulls/1").len(), 1);
    scene.fake.pr(1).body = "text only, block gone".into();
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(scene.wrote(Method::Patch, "pulls/1").len(), 1);
}

/// The id of the engine's move comment on pull request 1.
fn announcement(scene: &mut Scene) -> i64 {
    scene
        .fake
        .pr(1)
        .comments
        .iter()
        .rev()
        .find(|c| c.author == ENGINE && c.body.contains(MOVE_MARKER))
        .expect("an announcement")
        .id
}

#[test]
fn the_same_move_on_the_same_head_is_kept_current_in_place() {
    // Announced already for this head: the words are kept current without
    // notifying again.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let id = announcement(&mut scene);
    for comment in &mut scene.fake.pr(1).comments {
        if comment.id == id {
            comment.body = comment
                .body
                .replace("post `/self-reviewed`", "do something else");
        }
    }
    scene.sync_pr(&policy, 1);
    assert_eq!(
        comment_writes(&scene),
        (vec![id.to_string()], 0),
        "edited, so nobody is notified twice"
    );
}

#[test]
fn a_new_head_is_announced_afresh_so_the_author_is_told_again() {
    // tenderdash#1489: the announcement for the new head was edited into
    // the old one — which notifies nobody.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let (result, record) = announced(&policy, &mut scene);
    let older = "b".repeat(40);
    let mut old_record = record.clone();
    if let PyValue::Dict(fields) = &mut old_record {
        fields.insert("head".into(), s(&older));
    }
    let old_words = moved(&result).replace(HEAD, &older);
    scene.fake.pr(1).body = format!("text\n\n{}", block_of(&result));
    scene.fake.pr(1).labels = vec!["waiting-self-review".into(), "bot-review-skipped".into()];
    scene
        .fake
        .pr(1)
        .comments
        .push(engine_comment(50, &old_record, &old_words, None, NOW));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    let (patched, posted) = comment_writes(&scene);
    assert_eq!(
        (patched.len(), posted),
        (0, 1),
        "a new comment: that is the notification"
    );
    assert!(written_words(&scene)[0].contains(&format!("sha={HEAD}")));
}

#[test]
fn the_old_standing_comment_points_at_the_description_then_goes() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    let old =
        state_record(&pr, &with_state(&result, "waiting-bots"), &"c".repeat(64)).expect("a record");
    let standing = engine_comment(
        7,
        &old,
        "### PR Hygiene\nState: **waiting-bots**",
        None,
        "2026-09-10T00:00:00Z",
    );
    // No move yet: the old comment is kept as the record, its text
    // repointed.
    let (_, mut quiet) = bartek();
    quiet.pr(1).comments = vec![standing.clone()];
    let mut quiet = Scene::new(quiet);
    let run = quiet.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "waiting-bots");
    assert_eq!(comment_writes(&quiet), (vec!["7".to_owned()], 0));
    assert_eq!(written_words(&quiet), [POINTER]);
    assert!(quiet.wrote(Method::Delete, "issues/comments/").is_empty());
    // A move: the announcement carries the record, the old comment goes.
    let (_, mut moving) = bartek();
    moving.pr(1).comments.push(standing);
    let mut moving = Scene::new(moving);
    moving.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&moving), (Vec::new(), 1));
    assert_eq!(
        moving.wrote(Method::Delete, "issues/comments/")[0].route,
        "issues/comments/7"
    );
}

#[test]
fn a_state_nobody_acts_on_and_back_does_not_announce_again() {
    // A check re-run walks the state away and back without anybody doing
    // anything: announcing a second time is noise on a pull request that
    // never changed hands.
    let (policy, mut fake) = bartek();
    fake.pr(1).author = "owner-of-everything".into();
    fake.pr(1).comments.push(Comment::new(
        9,
        "owner-of-everything",
        &format!("/self-reviewed {HEAD}"),
        "2026-09-11T12:00:00Z",
    ));
    fake.state
        .collaborators
        .push(("owner-of-everything".into(), "write".into()));
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    let words = moved(&result);
    // The record, refreshed in place to the build's state, under the
    // words of the move it announced.
    let record = state_record(&pr, &with_state(&result, "waiting-build"), &"c".repeat(64))
        .expect("a record");
    scene
        .fake
        .pr(1)
        .comments
        .push(engine_comment(50, &record, &words, None, LATER));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(
        comment_writes(&scene),
        (vec!["50".to_owned()], 0),
        "edited: nobody is notified twice"
    );
}

#[test]
fn a_bot_reporting_with_nothing_to_answer_is_not_announced_again() {
    // A bot finished after the author was told their move, and the state
    // passed through the build on the way back. A report with nothing to
    // answer takes nothing back.
    for attested in [false, true] {
        let (policy, mut fake) = bartek();
        if attested {
            fake.pr(1).comments.push(Comment::new(
                9,
                "llbartekll",
                &format!("/self-reviewed {HEAD}"),
                "2026-09-11T09:20:00Z",
            ));
        }
        let mut scene = Scene::new(fake);
        let pr = scene.snapshot(&policy, 1);
        let result = scene.verdict(&policy, 1, &s(NOW));
        let expected = if attested {
            "ready-for-human"
        } else {
            "waiting-self-review"
        };
        assert_eq!(text(field(&result, "state")), expected);
        let record = state_record(&pr, &with_state(&result, "waiting-build"), &"c".repeat(64))
            .expect("a record");
        scene.fake.pr(1).comments.push(engine_comment(
            50,
            &record,
            &moved(&result),
            None,
            "2026-09-11T09:30:00Z",
        ));
        let again = scene.verdict(&policy, 1, &s(NOW));
        assert!(text(field(&again, "bot_completed_at")) > "2026-09-11T09:30:00Z");
        scene.fake.forget_calls();
        scene.sync_pr(&policy, 1);
        assert_eq!(
            comment_writes(&scene),
            (vec!["50".to_owned()], 0),
            "attested: {attested}"
        );
    }
}

/// A review thread CodeRabbit opened with a major finding.
fn major_finding(at: &str) -> Value {
    thread(
        "T1",
        false,
        "_\u{1f3af} Functional Correctness_ | _\u{1f7e0} Major_ | _\u{26a1} Quick win_\n\n**Fix it.**",
        &[("coderabbitai", at)],
    )
}

#[test]
fn only_a_bot_saying_something_new_tells_the_author_again() {
    // The author was told to answer a blocker. CodeRabbit then rewrote its
    // comment: a banner is not a report and must not notify anyone again,
    // while a new finding about the code is, and must.
    for (body, again_announced) in [
        (
            rabbit_receipt("Minimal", "", "<a href=\"#\">Review in Change Stack</a>\n"),
            false,
        ),
        (
            rabbit_receipt("High", "Add signer support before merging.\n", ""),
            true,
        ),
    ] {
        let (policy, mut fake) = bartek();
        fake.pr(1).threads = vec![major_finding("2026-09-11T11:00:00Z")];
        fake.pr(1).comments.push(Comment::new(
            21,
            "coderabbitai[bot]",
            &rabbit_receipt("Minimal", "", ""),
            "2026-09-11T11:00:00Z",
        ));
        let mut scene = Scene::new(fake);
        let pr = scene.snapshot(&policy, 1);
        let first = scene.verdict(&policy, 1, &s(NOW));
        assert_eq!(text(field(&first, "state")), "waiting-author");
        let record = state_record(&pr, &with_state(&first, "waiting-build"), &"c".repeat(64))
            .expect("a record");
        let diff = py(json!({"number": 1}));
        let mut diff = diff;
        if let PyValue::Dict(fields) = &mut diff {
            fields.insert("receipts".into(), field(&first, "receipts").clone());
        }
        scene.fake.pr(1).comments.push(engine_comment(
            50,
            &record,
            &moved(&first),
            Some(&diff),
            "2026-09-11T11:30:00Z",
        ));
        for comment in &mut scene.fake.pr(1).comments {
            if comment.id == 21 {
                comment.body = body.clone();
                comment.updated_at = "2026-09-11T12:30:00Z".into();
            }
        }
        let again = scene.verdict(&policy, 1, &s(NOW));
        assert_eq!(text(field(&again, "state")), "waiting-author");
        scene.fake.forget_calls();
        scene.sync_pr(&policy, 1);
        let expected = if again_announced {
            (Vec::new(), 1)
        } else {
            (vec!["50".to_owned()], 0)
        };
        assert_eq!(
            comment_writes(&scene),
            expected,
            "a new report: {again_announced}"
        );
    }
}

#[test]
fn a_bot_reporting_while_an_objection_waits_tells_the_author_again() {
    // The move is the author's — a reviewer objected after they attested —
    // and a bot then says something new about the code.
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.push(Comment::new(
        9,
        "llbartekll",
        &format!("/self-reviewed {HEAD}"),
        "2026-09-11T11:00:00Z",
    ));
    fake.pr(1).reviews[0]["state"] = json!("CHANGES_REQUESTED");
    fake.pr(1).reviews[0]["submitted_at"] = json!("2026-09-11T12:00:00Z");
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    assert_eq!(
        text(&items(field(&result, "blockers"))[0]),
        "Author response is required after the latest human objection"
    );
    let record = state_record(&pr, &with_state(&result, "waiting-build"), &"c".repeat(64))
        .expect("a record");
    scene.fake.pr(1).comments.push(engine_comment(
        50,
        &record,
        &moved(&result),
        None,
        "2026-09-11T12:30:00Z",
    ));
    scene.fake.pr(1).comments.push(Comment::new(
        21,
        "coderabbitai[bot]",
        &rabbit_receipt("Minimal", "", ""),
        "2026-09-11T13:00:00Z",
    ));
    scene.fake.forget_calls();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "waiting-author");
    assert_eq!(
        comment_writes(&scene),
        (Vec::new(), 1),
        "a new comment: that is the notification"
    );
}

#[test]
fn a_second_bot_speaking_after_the_author_was_told_tells_them_again() {
    // The author was told to answer one bot's finding; a second bot then
    // opened a thread. The completion time is on the record, so the
    // announcement no longer looks current.
    let (policy, mut fake) = bartek();
    fake.pr(1).threads = vec![major_finding("2026-09-11T10:00:00Z")];
    fake.pr(1).comments.push(Comment::new(
        9,
        "llbartekll",
        &format!("/self-reviewed {HEAD}"),
        "2026-09-11T12:00:00Z",
    ));
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    assert_eq!(text(field(&result, "state")), "waiting-author");
    assert!(
        field(&result, "bot_completed_at").truthy(),
        "a bot spoke; when is on the record"
    );
    let record = state_record(&pr, &result, &"c".repeat(64)).expect("a record");
    scene.fake.pr(1).comments.push(engine_comment(
        50,
        &record,
        &moved(&result),
        None,
        "2026-09-11T09:00:00Z",
    ));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(
        comment_writes(&scene),
        (Vec::new(), 1),
        "a new comment: that is the notification"
    );
}

#[test]
fn a_bot_author_is_still_told_the_reviewers_move() {
    let (policy, mut fake) = bartek();
    fake.pr(1).bot_author = true;
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(
        text(field(&run.verdicts[0], "state")),
        "ready-for-human",
        "a bot author skips the attestation"
    );
    assert_eq!(
        comment_writes(&scene).1,
        1,
        "the reviewer is told; that is not the bot"
    );
}

#[test]
fn a_named_machine_author_is_not_told_its_move() {
    let (policy, mut fake) = bartek();
    let mut policy = policy;
    if let PyValue::Dict(fields) = &mut policy {
        fields.insert("bot_authors".into(), py(json!(["infraclaw-dash"])));
    }
    fake.pr(1).author = "infraclaw-dash".into();
    fake.state
        .collaborators
        .push(("infraclaw-dash".into(), "write".into()));
    // A bot left a finding, so the move is the author's — and there is no
    // author to take it.
    fake.pr(1).threads = vec![major_finding("2026-09-11T10:00:00Z")];
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "waiting-author");
    let told: Vec<String> = written_words(&scene)
        .into_iter()
        .filter(|w| w.to_lowercase().contains("your move"))
        .collect();
    assert!(told.is_empty(), "{told:?}");
    // The verdict is unaffected: what it is owed is still owed.
    assert!(dump(field(&run.verdicts[0], "blockers")).contains("coderabbitai"));
}

#[test]
fn a_description_too_long_is_a_warning_not_an_error() {
    // The author made the description too long for the block while the
    // engine worked.
    let (policy, mut fake) = bartek();
    fake.on_call(|state, call| {
        if matches!(call, Call::Rest { method: Method::Post, path, .. } if path.contains("/statuses/")) {
            state.pr(1).body = "x".repeat(65_500);
        }
    });
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert!(run.failure().is_none());
    assert!(scene.said("PR #1: checklist not written: Description too long for the checklist"));
    assert!(!scene.statuses().iter().any(|(state, _)| state == "error"));
    assert_eq!(comment_writes(&scene).1, 1);
}

// RecordTests: the record is refreshed silently; a comment is posted only
// for news.

#[test]
fn an_evidence_only_change_refreshes_nothing_and_posts_nothing() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    // A reviewer's plain comment: evidence changed, nothing else did.
    scene
        .fake
        .pr(1)
        .comments
        .push(Comment::new(60, "romchornyi", "looks fine", LATER));
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
    assert!(
        !scene
            .statuses()
            .iter()
            .any(|(_, d)| d.starts_with("Evaluating")),
        "the fast path: the record holds only what matters, not the evidence print"
    );
}

#[test]
fn a_forged_newest_record_is_written_over_once_and_then_read() {
    // Nothing is read from a newest record somebody else edited, and
    // nothing older stands in for it. The run writes the engine's own
    // record over it, in place; the next run reads that back.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let id = announcement(&mut scene);
    for comment in &mut scene.fake.pr(1).comments {
        if comment.id == id {
            comment.body = comment.body.replace(LATER, "2020-01-01T00:00:00Z");
            *comment = comment.clone().edited("llbartekll", "2026-09-11T13:00:00Z");
        }
    }
    assert!(matches!(
        field(&scene.snapshot(&policy, 1), "controller_state"),
        PyValue::None
    ));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(
        comment_writes(&scene),
        (vec![id.to_string()], 0),
        "written over in place"
    );
    let read_back = scene.snapshot(&policy, 1);
    assert_eq!(
        dump(field(&read_back, "controller_comment_id")),
        id.to_string()
    );
    assert_eq!(
        text(field(field(&read_back, "controller_state"), "admitted_at")),
        LATER
    );
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
}

#[test]
fn a_state_change_that_is_not_a_move_refreshes_the_record_silently() {
    // waiting-build is not a move, so it reaches the record by editing the
    // newest holder, never by a new comment.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let id = announcement(&mut scene);
    scene
        .fake
        .pr(1)
        .comments
        .push(Comment::new(2, "llbartekll", "/self-reviewed", LATER));
    scene.fake.pr(1).checks = Some(vec![json!({"__typename": "CheckRun", "name": "tests",
        "conclusion": null, "status": "IN_PROGRESS", "startedAt": LATER,
        "detailsUrl": "https://github.com/dashpay/platform/actions/runs/1/job/2",
        "checkSuite": {"workflowRun": {"workflow": {"resourcePath": "/dashpay/platform/actions/workflows/ci.yml"}}}})]);
    scene.now = BUILD_RUNNING.into();
    scene.fake.state.now = BUILD_RUNNING.into();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "waiting-build");
    assert_eq!(
        comment_writes(&scene),
        (vec![id.to_string()], 0),
        "the newest holder, edited"
    );
    assert!(
        written_words(&scene)[0].contains("Bots are done"),
        "its words unchanged"
    );
    let record = field(&scene.snapshot(&policy, 1), "controller_state").clone();
    assert_eq!(text(field(&record, "state")), "waiting-build");
}

#[test]
fn the_first_pass_converts_a_standing_record_of_this_move_rather_than_announcing_it() {
    let (policy, mut fake) = bartek();
    fake.pr(1)
        .comments
        .push(Comment::new(2, "llbartekll", "/self-reviewed", LATER));
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    assert_eq!(text(field(&result, "state")), "ready-for-human");
    let record = state_record(&pr, &result, &"c".repeat(64)).expect("a record");
    scene.fake.pr(1).comments.push(engine_comment(
        7,
        &record,
        "### PR Hygiene\nState: **ready-for-human**",
        None,
        "2026-09-10T00:00:00Z",
    ));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(
        comment_writes(&scene),
        (vec!["7".to_owned()], 0),
        "the standing comment becomes the announcement in place"
    );
    assert!(written_words(&scene)[0].contains("Ready for review"));
    assert!(scene.wrote(Method::Delete, "issues/comments/").is_empty());
}

#[test]
fn ready_since_does_not_drift_between_announcements() {
    // Two announcements exist. The record is read from whichever was
    // written last, so a refresh of the older one is still the truth.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    scene
        .fake
        .pr(1)
        .comments
        .push(Comment::new(2, "llbartekll", "/self-reviewed", LATER));
    scene.now = BUILD_RUNNING.into();
    scene.fake.state.now = BUILD_RUNNING.into();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "ready-for-human");
    let record = field(&scene.snapshot(&policy, 1), "controller_state").clone();
    assert_eq!(text(field(&record, "ready_since")), BUILD_RUNNING);
    scene.now = "2026-09-11T16:00:00Z".into();
    scene.fake.state.now = "2026-09-11T16:00:00Z".into();
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
}

#[test]
fn two_standing_comments_converge_to_one() {
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.clear();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    let record = state_record(&pr, &result, &"c".repeat(64)).expect("a record");
    scene.fake.pr(1).comments = vec![
        engine_comment(10, &record, "old text", None, "2026-09-10T00:00:00Z"),
        engine_comment(11, &record, POINTER, None, "2026-09-11T00:00:00Z"),
    ];
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    let deleted: Vec<String> = scene
        .wrote(Method::Delete, "issues/comments/")
        .iter()
        .map(|w| w.route.clone())
        .collect();
    assert_eq!(
        deleted,
        ["issues/comments/10"],
        "all but the record holder go"
    );
}

#[test]
fn a_description_with_no_room_is_left_alone_on_the_fast_path() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    scene.fake.pr(1).body = "x".repeat(65_000);
    scene.sync_pr(&policy, 1);
    assert!(scene.wrote(Method::Patch, "pulls/1").is_empty());
    assert!(!scene
        .statuses()
        .iter()
        .any(|(_, d)| d.starts_with("Evaluating")));
}

#[test]
fn a_configuration_error_writes_no_block_and_no_announcement() {
    let (policy, mut fake) = bartek();
    fake.pr(1).files.clear();
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(
        text(field(&run.verdicts[0], "state")),
        "configuration-error"
    );
    assert!(scene.wrote(Method::Patch, "pulls/1").is_empty());
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
}

// SecondReviewTests: what the cross-model review found.

#[test]
fn a_bot_author_still_gets_the_merge_announcement_and_its_record() {
    // Only the author-directed move is withheld from a bot: "you can merge"
    // is for the humans, and it carries the record.
    let (policy, mut fake) = bartek();
    fake.pr(1).bot_author = true;
    fake.pr(1)
        .reviews
        .push(review(6, "shumkov", "APPROVED", HEAD, LATER, ""));
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "ready-to-merge");
    assert_eq!(comment_writes(&scene).1, 1);
    assert!(written_words(&scene)[0].contains("state=ready-to-merge"));
}

#[test]
fn a_first_waiting_build_with_no_comment_posts_none() {
    // An attestation naming the commit, posted before the bots finish,
    // skips waiting-self-review: no move, no comment, no record. (The scan
    // that then finds it through its status is not ported.)
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.push(Comment::new(
        2,
        "llbartekll",
        &format!("/self-reviewed {HEAD}"),
        "2026-09-11T10:30:00Z",
    ));
    fake.pr(1).checks = Some(vec![json!({"__typename": "CheckRun", "name": "tests",
        "conclusion": null, "status": "IN_PROGRESS", "startedAt": NOW,
        "detailsUrl": "https://github.com/dashpay/platform/actions/runs/1/job/2",
        "checkSuite": {"workflowRun": {"workflow": {"resourcePath": "/dashpay/platform/actions/workflows/ci.yml"}}}})]);
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "waiting-build");
    assert_eq!(comment_writes(&scene), (Vec::new(), 0));
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("pending".to_owned(), "waiting-build".to_owned()),
        "the status is the only trace of the state"
    );
}

#[test]
fn the_holder_is_the_comment_the_record_was_read_from() {
    // Two record comments; the older one was written last. The record was
    // read from it, and it is the one edited and kept.
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.clear();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    let record = state_record(&pr, &result, &"c".repeat(64)).expect("a record");
    let mut later = record.clone();
    if let PyValue::Dict(fields) = &mut later {
        fields.insert("ready_since".into(), s("2026-09-10T12:00:00Z"));
    }
    scene.fake.pr(1).comments = vec![
        engine_comment(2, &later, "other text", None, "2026-09-10T12:00:00Z"),
        engine_comment(1, &record, "old text", None, "2026-09-10T00:00:00Z")
            .edited(ENGINE, "2026-09-11T11:00:00Z"),
    ];
    assert_eq!(
        dump(field(&scene.snapshot(&policy, 1), "controller_comment_id")),
        "1"
    );
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(comment_writes(&scene).0, ["1"]);
    assert_eq!(
        scene.wrote(Method::Delete, "issues/comments/")[0].route,
        "issues/comments/2"
    );
}

#[test]
fn a_foreign_bot_comment_quoting_the_marker_is_left_alone_and_fails_the_history() {
    // Python's test hands `publish` a pull request whose record was not read
    // from these comments. Read from them, the newest record of the
    // engine's identity names another pull request: the admission history
    // is refused, every candidate's head is marked, and the comment itself
    // is neither edited nor deleted.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let (_, record) = announced(&policy, &mut scene);
    let mut foreign = record.clone();
    if let PyValue::Dict(fields) = &mut foreign {
        fields.insert("number".into(), PyValue::Int(PyInt::from(999)));
    }
    scene.fake.pr(1).comments.push(engine_comment(
        9,
        &foreign,
        "another workflow, another pull request",
        None,
        NOW,
    ));
    scene.fake.forget_calls();
    // The newest record is another pull request's: this one's history is
    // refused rather than read as its own.
    let refused = scene.run(
        &policy,
        pr_hygiene_engine::reconcile::Command::Sync,
        Pick::Pr(1),
        None,
    );
    assert!(
        matches!(&refused, Err(ReadError::GitHub(m)) if m == "Controller admission history belongs to another PR"),
        "{refused:?}"
    );
    assert!(scene.wrote(Method::Delete, "issues/comments/").is_empty());
    assert_eq!(
        comment_writes(&scene),
        (Vec::new(), 0),
        "the foreign one is not edited"
    );
    assert_eq!(
        scene.statuses(),
        [(
            "error".to_owned(),
            "Incomplete policy evidence; reconciliation required".to_owned()
        )]
    );
}

#[test]
fn the_move_coming_back_to_the_author_is_announced_again() {
    // A → B → A on one head — the author attested, it went out for review,
    // a reviewer objected. Editing A's old announcement leaves the author
    // with nothing in their inbox.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let (result, record) = announced(&policy, &mut scene);
    let a = engine_comment(50, &record, &moved(&result), None, "2026-09-11T10:00:00Z");
    let b_words = format!(
        "{MOVE_MARKER} state=ready-for-human sha={HEAD} -->\n\
         Ready for review — needs QuantumExplorer or shumkov.\nFull checklist in the description."
    );
    let b = engine_comment(
        51,
        &with_state(&record, "ready-for-human"),
        &b_words,
        None,
        "2026-09-11T11:00:00Z",
    );
    scene.fake.pr(1).comments.extend([a, b]);
    scene.fake.pr(1).body = format!("text\n\n{}", block_of(&result));
    scene.fake.pr(1).labels = vec!["waiting-self-review".into(), "bot-review-skipped".into()];
    assert_eq!(
        dump(field(&scene.snapshot(&policy, 1), "controller_comment_id")),
        "51"
    );
    scene.fake.forget_calls();
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(
        text(field(&run.verdicts[0], "state")),
        "waiting-self-review"
    );
    assert_eq!(
        comment_writes(&scene),
        (Vec::new(), 1),
        "a new comment: that is the notification"
    );
    assert!(written_words(&scene)[0].contains("state=waiting-self-review"));
    let read_back = field(&scene.snapshot(&policy, 1), "controller_state").clone();
    assert_eq!(text(field(&read_back, "state")), "waiting-self-review");
}

#[test]
fn a_pull_request_back_in_draft_loses_the_stale_block() {
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    let result = scene.verdict(&policy, 1, &s(NOW));
    scene.fake.pr(1).draft = true;
    scene.fake.pr(1).body = format!("text\n\n{}", block_of(&result));
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    let described = scene.wrote(Method::Patch, "pulls/1");
    assert_eq!(described.len(), 1, "removed, not rewritten");
    assert_eq!(text(described[0].field("body")), "text");
}

#[test]
fn the_status_never_depends_on_the_description_or_the_labels() {
    // At capacity, labels refused, description refused: the check is still
    // what the evidence says.
    let (policy, mut fake) = bartek();
    fake.pr(1).comments.push(Comment::new(
        2,
        "llbartekll",
        "/self-reviewed",
        "2026-09-11T11:00:00Z",
    ));
    fake.pr(1).reviews.push(review(
        6,
        "shumkov",
        "APPROVED",
        HEAD,
        "2026-09-11T11:30:00Z",
        "",
    ));
    fake.pr(1).body = "x".repeat(65_500);
    fake.pr(1).labels = vec![
        "waiting-bots".into(),
        "ready-to-merge".into(),
        "nonsense".into(),
    ];
    // The admission is already on record, as it would be once a human was
    // involved.
    fake.pr(1).comments.push(engine_comment(
        50,
        &crate::publication::record(1, HEAD, Some(NOW), "waiting-bots"),
        POINTER,
        None,
        NOW,
    ));
    for route in ["issues/1/labels", "pulls/1"] {
        fake.refuse(Method::Post, route, Refusal::Http(422, "no".into()));
        fake.refuse(Method::Delete, route, Refusal::Http(422, "no".into()));
        fake.refuse(Method::Patch, route, Refusal::Http(422, "no".into()));
    }
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert_eq!(text(field(&run.verdicts[0], "state")), "ready-to-merge");
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("success".to_owned(), "ready-to-merge".to_owned())
    );
}

// StaleMarkTests: a pull request that left the governed set keeps no
// verdict of the engine's — but keeps its record.

const AWAY: i64 = 4660;

/// Pull request 4660 on `base`, wearing `labels`, its description `body`,
/// with these comments.
fn away(base: &str, labels: &[&str], body: &str, comments: Vec<Comment>) -> Scene {
    let mut fake = Fake::new(LATER);
    let mut pr = Pr::new(AWAY, "owner", HEAD);
    pr.base = base.into();
    pr.labels = labels.iter().map(|l| (*l).to_owned()).collect();
    pr.body = body.into();
    pr.comments = comments;
    fake.add(pr);
    Scene::new(fake)
}

fn clear(scene: &mut Scene, apply: bool) {
    let pr = scene
        .with(|engine| engine.api().pull(&PyInt::from(AWAY)))
        .expect("the pull request");
    scene.fake.forget_calls();
    scene
        .with(|engine| engine.clear_marks(&bartek().0, std::slice::from_ref(&pr), apply))
        .expect("cleared");
}

fn held(state: &str, id: i64, updated: Option<&str>, admitted: &str) -> Comment {
    let record = py(
        json!({"number": AWAY, "head": HEAD, "admitted_at": admitted,
        "ready_since": "2026-09-02T10:00:00Z", "state": state, "version": 1,
        "evidence": "e".repeat(64), "context": "c".repeat(64)}),
    );
    let diff = py(
        json!({"number": AWAY, "diff": "d".repeat(64), "diff_heads": [HEAD],
                         "receipts": {"e".repeat(64): "2026-09-02T09:00:00Z"}}),
    );
    let comment = engine_comment(
        id,
        &record,
        "Ready for review — `dpp`: shumkov.\nFull checklist in the description.",
        Some(&diff),
        NOW,
    );
    match updated {
        Some(at) => comment.edited(ENGINE, at),
        None => comment,
    }
}

fn pointer_record(number: i64, state: &str) -> Comment {
    let record = py(
        json!({"number": number, "head": HEAD, "admitted_at": null, "ready_since": null,
        "state": state, "version": 1, "evidence": "e".repeat(64), "context": "c".repeat(64)}),
    );
    engine_comment(7, &record, POINTER, None, NOW)
}

#[test]
fn labels_and_block_are_cleared_when_the_base_leaves_the_policy() {
    // dashpay/platform#4660: rebased onto a feature branch, and two days
    // later still wearing `waiting-bots` and `bot-review-skipped`.
    let block = "<!-- pr-hygiene:start -->\nstale\n<!-- pr-hygiene:end -->";
    let mut scene = away(
        "keep-history-lifecycle",
        &["waiting-bots", "bot-review-skipped", "enhancement"],
        &format!("text\n\n{block}"),
        vec![pointer_record(AWAY, "waiting-bots")],
    );
    clear(&mut scene, true);
    let removed: Vec<String> = scene
        .wrote(Method::Delete, "issues/4660/labels/")
        .iter()
        .map(|w| w.route.rsplit('/').next().unwrap_or("").to_owned())
        .collect();
    assert_eq!(removed, ["bot-review-skipped", "waiting-bots"]);
    assert!(
        scene.wrote(Method::Post, "issues/4660/labels").is_empty(),
        "removed, never added"
    );
    assert_eq!(scene.fake.pr(AWAY).body, "text");
    assert!(scene.said("no longer governed"));
    // The record is the pull request's memory and stays; only its words,
    // which pointed at the checklist just removed, change.
    assert!(scene.wrote(Method::Delete, "issues/comments/").is_empty());
    let edited = scene.wrote(Method::Patch, "issues/comments/");
    assert_eq!(edited.len(), 1);
    assert_eq!(edited[0].route, "issues/comments/7");
    let body = text(edited[0].field("body"));
    assert!(body.contains("keep-history-lifecycle") && !body.contains(POINTER));
}

#[test]
fn the_diff_history_is_kept_and_the_slot_given_up() {
    let original = held("ready-for-human", 7, None, "2026-09-01T10:00:00Z");
    let diff_line = original
        .body
        .split("\n\n")
        .next()
        .unwrap_or("")
        .lines()
        .nth(1)
        .unwrap_or("")
        .to_owned();
    let mut scene = away("feature", &["ready-for-human"], "x", vec![original]);
    clear(&mut scene, true);
    assert_eq!(scene.wrote(Method::Patch, "issues/comments/").len(), 1);
    let read_back = scene
        .with(|engine| engine.api().histories(&[PyInt::from(AWAY)]))
        .expect("read");
    let body = read_back[&PyInt::from(AWAY)].comments[0].clone();
    let text_of = text(field(&body, "body")).to_owned();
    assert_eq!(
        text_of.split("\n\n").next().unwrap_or("").lines().nth(1),
        Some(diff_line.as_str()),
        "diff history byte for byte"
    );
    let pr = scene.snapshot(&bartek().0, AWAY);
    let state = field(&pr, "controller_state");
    assert!(matches!(field(state, "admitted_at"), PyValue::None));
    assert!(matches!(field(state, "ready_since"), PyValue::None));
    assert_eq!(text(field(state, "state")), "not-governed");
    assert_eq!(text(field(state, "head")), HEAD);
    assert!(field(&pr, "controller_diff").truthy());
}

#[test]
fn only_the_current_record_is_rewritten() {
    // Rewriting an older announcement as well could make it the newest,
    // and the stale state it carries the record.
    let mut scene = away(
        "feature",
        &["ready-for-human"],
        "x",
        vec![
            held(
                "ready-for-human",
                50,
                Some("2026-09-11T12:00:00Z"),
                "2026-09-01T10:00:00Z",
            ),
            held(
                "waiting-self-review",
                51,
                Some("2026-09-11T11:00:00Z"),
                "2026-09-01T10:00:00Z",
            ),
        ],
    );
    clear(&mut scene, true);
    let edited: Vec<String> = scene
        .wrote(Method::Patch, "issues/comments/")
        .iter()
        .map(|w| w.route.clone())
        .collect();
    assert_eq!(edited, ["issues/comments/50"]);
}

#[test]
fn a_record_the_engine_refreshed_is_set_aside() {
    // An edited record is believed only when the engine edited it, which
    // only the batched read says.
    let mut scene = away(
        "feature",
        &["ready-for-human"],
        "x",
        vec![held(
            "ready-for-human",
            7,
            Some("2026-09-11T13:00:00Z"),
            "2026-09-01T10:00:00Z",
        )],
    );
    clear(&mut scene, true);
    let edited = scene.wrote(Method::Patch, "issues/comments/");
    assert_eq!(edited.len(), 1);
    assert!(text(edited[0].field("body")).contains(r#""state":"not-governed""#));
}

#[test]
fn the_record_is_set_aside_before_the_marks_go() {
    // The labels are what bring a pull request back into the sweep. If the
    // record cannot be written they stay, and the next sweep tries again.
    let mut refused = away(
        "feature",
        &["ready-for-human"],
        "x",
        vec![held("ready-for-human", 7, None, "2026-09-01T10:00:00Z")],
    );
    refused.fake.refuse(
        Method::Patch,
        "issues/comments/",
        Refusal::Http(403, "no".into()),
    );
    clear(&mut refused, true);
    assert_eq!(refused.writes(), ["PATCH issues/comments/7"]);
    assert!(refused.said("PR #4660: could not update the record comment"));
    let mut scene = away(
        "feature",
        &["ready-for-human"],
        "x",
        vec![held("ready-for-human", 7, None, "2026-09-01T10:00:00Z")],
    );
    clear(&mut scene, true);
    assert_eq!(
        scene.writes(),
        [
            "PATCH issues/comments/7",
            "DELETE issues/4660/labels/ready-for-human"
        ]
    );
}

#[test]
fn a_record_already_set_aside_is_not_written_again() {
    let mut scene = away(
        "feature",
        &["ready-for-human"],
        "x",
        vec![held("ready-for-human", 7, None, "2026-09-01T10:00:00Z")],
    );
    clear(&mut scene, true);
    scene.fake.pr(AWAY).labels = vec!["waiting-bots".into()];
    clear(&mut scene, true);
    assert_eq!(scene.writes(), ["DELETE issues/4660/labels/waiting-bots"]);
}

#[test]
fn the_record_comment_of_another_pull_request_is_not_touched() {
    let mut scene = away(
        "feature",
        &["waiting-bots"],
        "x",
        vec![pointer_record(999, "waiting-bots")],
    );
    clear(&mut scene, true);
    assert_eq!(scene.writes(), ["DELETE issues/4660/labels/waiting-bots"]);
}

#[test]
fn a_governed_pull_request_is_never_touched() {
    let mut scene = away("v5.0-dev", &["waiting-bots"], "x", Vec::new());
    clear(&mut scene, true);
    assert!(scene.fake.calls.is_empty());
}

#[test]
fn labels_that_are_not_the_engines_are_left_alone() {
    let mut scene = away("feature", &["enhancement", "bug"], "x", Vec::new());
    clear(&mut scene, true);
    assert!(scene.fake.calls.is_empty(), "not even read");
}

#[test]
fn a_name_the_engine_no_longer_sets_is_left_alone() {
    // `waiting-build` and `ready-to-merge` were retired here, which says
    // nothing about who else uses those words.
    let mut scene = away(
        "feature",
        &["waiting-build", "ready-to-merge"],
        "x",
        Vec::new(),
    );
    clear(&mut scene, true);
    assert_eq!(scene.writes(), Vec::<String>::new());
}

#[test]
fn a_retired_name_goes_with_the_marks_beside_it() {
    let mut scene = away(
        "feature",
        &["waiting-bots", "ready-to-merge"],
        "x",
        Vec::new(),
    );
    clear(&mut scene, true);
    assert_eq!(
        scene.writes(),
        [
            "DELETE issues/4660/labels/ready-to-merge",
            "DELETE issues/4660/labels/waiting-bots"
        ]
    );
}

#[test]
fn a_retired_name_alone_is_swept_where_the_record_says_it_is_the_engines() {
    // Marked `ready-to-merge` by an older engine, the checklist since
    // removed, then rebased away. The record comment says it is the
    // engine's, and it is read only for pull requests still wearing such a
    // name.
    let mut scene = away(
        "feature",
        &["ready-to-merge"],
        "x",
        vec![pointer_record(AWAY, "ready-to-merge")],
    );
    clear(&mut scene, true);
    assert_eq!(
        scene.writes(),
        [
            "PATCH issues/comments/7",
            "DELETE issues/4660/labels/ready-to-merge"
        ]
    );
    assert!(
        scene.fake.calls.iter().any(
            |call| matches!(call, Call::Rest { path, .. } if path.contains("/issues/4660/comments"))
        ),
        "found through the comment listing"
    );
}

#[test]
fn a_preview_run_says_what_it_would_clear_and_writes_nothing() {
    let mut scene = away("feature", &["waiting-bots"], "x", Vec::new());
    clear(&mut scene, false);
    assert_eq!(scene.writes(), Vec::<String>::new());
    assert!(scene.said("#4660"));
}

#[test]
fn the_context_print_is_the_one_the_record_carries() {
    // A settled record carries the admission context of its author's pull
    // requests as `context_fingerprint` makes it.
    let (policy, fake) = bartek();
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let pr = scene.snapshot(&policy, 1);
    let open = scene
        .with(|engine| engine.api().open_prs())
        .expect("listed");
    let context = context_fingerprint(&open, field(&pr, "author")).expect("a print");
    assert_eq!(
        text(field(field(&pr, "controller_state"), "context")),
        context
    );
}
