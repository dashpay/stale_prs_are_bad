//! `pr_review/tests/test_bot_timeouts.py`'s tests of `bot_schedule`:
//! nudging a silent review bot, and eventually proceeding without it. Its
//! tests of `evaluate` are in the corpus; those of the status page reader
//! belong to `telemetry`, which is not part of the policy.

use crate::support::{ago, at, fixture, remove, s, set, value, NOW};
use pr_hygiene_engine::policy::{bot_schedule, Schedule};
use pr_hygiene_engine::pycompat::PyValue;
use serde_json::json;

const NOTHING: Schedule = Schedule {
    nudge: false,
    waived_at: None,
    waived_reason: None,
};

const RATE_LIMITED: &str =
    "<!-- This is an auto-generated comment: rate limited by coderabbit.ai -->";
const RATE_LIMITED_END: &str =
    "<!-- end of auto-generated comment: rate limited by coderabbit.ai -->";

/// `waiting(hours)`: a pull request whose awaited bot, thepastaclaw, has
/// not reported, first seen that many hours ago, under timeouts of six
/// hours to a nudge and sixteen to a waiver.
fn waiting(hours_since_seen: f64) -> (PyValue, PyValue) {
    let (mut policy, mut pr) = fixture();
    set(
        &mut policy,
        &["bot_timeouts"],
        value(json!({"nudge_after_hours": 6, "waive_after_hours": 16})),
    );
    if let PyValue::List(reviews) = at(&mut pr, &["reviews"]) {
        reviews.retain(|r| !matches!(r, PyValue::Dict(d) if matches!(d.get("user"), Some(PyValue::Str(u)) if u == "thepastaclaw")));
    }
    set(&mut pr, &["head_seen_at"], s(&ago(hours_since_seen)));
    (policy, pr)
}

fn schedule(policy: &PyValue, pr: &PyValue, bot: &str, state: Option<&str>) -> Schedule {
    let state = state.map_or(PyValue::None, s);
    bot_schedule(policy, pr, bot, &s(NOW), &state).unwrap()
}

/// `plan(hours, state, comments, bot)`.
fn plan(hours: f64, state: Option<&str>) -> Schedule {
    let (policy, pr) = waiting(hours);
    schedule(&policy, &pr, "thepastaclaw", state)
}

fn comments(pr: &mut PyValue, items: serde_json::Value) {
    set(pr, &["comments"], value(items));
}

fn waived_at(policy: &PyValue, pr: &PyValue) -> Option<String> {
    schedule(policy, pr, "coderabbitai", None).waived_at
}

#[test]
fn nothing_happens_before_the_window() {
    assert_eq!(plan(2.0, None), NOTHING);
}

#[test]
fn an_unheard_of_head_is_nudged_once_the_window_passes() {
    assert!(plan(7.0, None).nudge);
}

#[test]
fn a_failed_review_is_nudged_at_once() {
    assert!(plan(0.0, Some("failed")).nudge);
}

#[test]
fn a_running_or_queued_review_is_left_alone() {
    for state in ["running", "queued"] {
        assert!(!plan(7.0, Some(state)).nudge, "{state}");
    }
}

#[test]
fn a_bot_is_asked_only_once_for_a_head() {
    let (policy, mut pr) = waiting(7.0);
    assert!(schedule(&policy, &pr, "thepastaclaw", None).nudge);
    let head = "a".repeat(40);
    comments(
        &mut pr,
        json!([{"user": "github-actions[bot]", "created_at": NOW, "updated_at": NOW,
                "body": format!("<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={head} -->\n@thepastaclaw review")}]),
    );
    assert!(!schedule(&policy, &pr, "thepastaclaw", None).nudge);
}

#[test]
fn a_nudge_for_another_head_does_not_count() {
    let (policy, mut pr) = waiting(7.0);
    comments(
        &mut pr,
        json!([{"user": "github-actions[bot]", "created_at": NOW, "updated_at": NOW,
                "body": format!("<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={} -->", "f".repeat(40))}]),
    );
    assert!(schedule(&policy, &pr, "thepastaclaw", None).nudge);
}

#[test]
fn coderabbit_announcing_its_own_limit_is_not_awaited_for_the_whole_window() {
    // A bot that has said it cannot review this head is not a bot that has
    // not answered yet: its author would otherwise wait the full window.
    let (policy, mut pr) = waiting(2.0);
    let notice = |at: &str| {
        json!([{"user": "coderabbitai[bot]", "created_at": at, "updated_at": at,
                "body": format!("{RATE_LIMITED}\nwait")}])
    };
    comments(&mut pr, notice(&ago(1.5)));
    let plan = schedule(&policy, &pr, "coderabbitai", None);
    assert_eq!(plan.waived_reason, Some("rate-limit"));
    assert!(plan.waived_at.as_deref().unwrap() < NOW);
    assert!(
        !plan.nudge,
        "proceeding without it is the answer, not another ask"
    );
    // Within the hour it documents for its own retry, it is still awaited.
    comments(&mut pr, notice(&ago(0.2)));
    assert_eq!(waived_at(&policy, &pr), None);
}

#[test]
fn the_limit_is_read_from_when_the_notice_was_written_not_first_posted() {
    // It keeps one comment and edits it; reading the creation time found a
    // notice from the week the pull request opened, or nothing at all.
    let (policy, mut pr) = waiting(2.0);
    comments(
        &mut pr,
        json!([{"user": "coderabbitai[bot]", "created_at": ago(200.0), "updated_at": ago(1.5),
                "edited_by": "coderabbitai", "body": RATE_LIMITED}]),
    );
    let plan = schedule(&policy, &pr, "coderabbitai", None);
    assert_eq!(plan.waived_reason, Some("rate-limit"));
    // The instant is the head's, not the edit's: an hour after this head
    // appeared, so a later edit cannot raise it under a standing attestation.
    assert_eq!(plan.waived_at, Some(ago(1.0)));
}

#[test]
fn a_notice_that_names_this_head_counts_even_if_it_came_first() {
    // CodeRabbit refused the head minutes before this controller first
    // reported on it; the notice names the commit, which is what binds it.
    let (policy, mut pr) = waiting(2.0);
    let head = "a".repeat(40);
    comments(
        &mut pr,
        json!([{"user": "coderabbitai[bot]", "created_at": ago(2.1), "updated_at": ago(2.1),
                "body": format!("{RATE_LIMITED}\n> Review limit reached\n> Reviewing files that changed between 4ab5161 and {head}.\n{RATE_LIMITED_END}")}]),
    );
    let plan = schedule(&policy, &pr, "coderabbitai", None);
    assert_eq!(plan.waived_reason, Some("rate-limit"));
    assert_eq!(
        plan.waived_at,
        Some(ago(1.0)),
        "an hour after the head, not after the whole window"
    );
}

#[test]
fn a_notice_naming_another_head_is_not_about_this_one() {
    let (policy, mut pr) = waiting(2.0);
    comments(
        &mut pr,
        json!([{"user": "coderabbitai[bot]", "created_at": ago(50.0), "updated_at": ago(50.0),
                "body": format!("{RATE_LIMITED}\n> Reviewing files that changed between 4ab5161 and {}.\n{RATE_LIMITED_END}", "e".repeat(40))}]),
    );
    assert_eq!(waived_at(&policy, &pr), None);
}

#[test]
fn a_bot_already_asked_about_this_work_is_not_asked_again() {
    // A push that only moved the base is the same work, and the clock the
    // bot is waited on keeps running.
    let (policy, mut pr) = waiting(7.0);
    let old = "f".repeat(40);
    comments(
        &mut pr,
        json!([{"user": "github-actions[bot]", "created_at": ago(1.0), "updated_at": ago(1.0),
                "body": format!("<!-- pr-hygiene-nudge v1 bot=coderabbitai sha={old} -->")}]),
    );
    assert!(
        schedule(&policy, &pr, "coderabbitai", None).nudge,
        "a commit never asked about"
    );
    set(
        &mut pr,
        &["reviewed_heads"],
        value(json!([old, "a".repeat(40)])),
    );
    assert!(!schedule(&policy, &pr, "coderabbitai", None).nudge);
}

#[test]
fn a_notice_with_no_end_has_no_extent() {
    // Reading to the end of the comment would let the walkthrough below the
    // notice speak for the limit.
    let (policy, mut pr) = waiting(2.0);
    let head = "a".repeat(40);
    comments(
        &mut pr,
        json!([{"user": "coderabbitai[bot]", "created_at": ago(50.0), "updated_at": ago(50.0),
                "body": format!("{RATE_LIMITED}\n> Review limit reached\nWalkthrough of {head} follows.")}]),
    );
    assert_eq!(waived_at(&policy, &pr), None);
}

#[test]
fn the_head_named_outside_the_notice_does_not_count() {
    let (policy, mut pr) = waiting(2.0);
    let head = "a".repeat(40);
    comments(
        &mut pr,
        json!([{"user": "coderabbitai[bot]", "created_at": ago(50.0), "updated_at": ago(50.0),
                "body": format!("{RATE_LIMITED}\n> Review limit reached\n{RATE_LIMITED_END}\nWalkthrough of {head} follows.")}]),
    );
    assert_eq!(waived_at(&policy, &pr), None);
}

#[test]
fn a_notice_rewritten_by_somebody_else_is_not_the_bot_speaking() {
    // Anyone with write access can edit anyone's comment; one whitespace
    // edit of a week-old notice must not drop CodeRabbit's review.
    let (policy, mut pr) = waiting(2.0);
    for editor in [json!("llbartekll"), json!(null)] {
        comments(
            &mut pr,
            json!([{"user": "coderabbitai[bot]", "created_at": ago(200.0), "updated_at": ago(1.5),
                    "body": RATE_LIMITED, "edited_by": editor}]),
        );
        assert_eq!(waived_at(&policy, &pr), None, "{editor}");
    }
}

#[test]
fn the_rate_limit_instant_does_not_move_when_the_notice_is_rewritten() {
    // It dates when the bots last spoke; an instant that moved with each
    // edit would read as a new report.
    let (policy, mut pr) = waiting(4.0);
    let notice = |updated: &str| {
        json!([{"user": "coderabbitai[bot]", "created_at": ago(3.0), "updated_at": updated,
                "edited_by": "coderabbitai", "body": RATE_LIMITED}])
    };
    comments(&mut pr, notice(&ago(3.0)));
    let first = waived_at(&policy, &pr);
    assert!(first.is_some());
    comments(&mut pr, notice(&ago(0.1)));
    assert_eq!(waived_at(&policy, &pr), first);
}

#[test]
fn the_waiver_arrives_on_time_without_any_telemetry() {
    assert_eq!(plan(15.0, None).waived_at, None);
    assert!(plan(17.0, None).waived_at.is_some());
}

#[test]
fn the_status_page_cannot_postpone_a_waiver() {
    for state in [None, Some("running"), Some("queued"), Some("failed")] {
        assert_eq!(plan(15.0, state).waived_at, None, "{state:?}");
        assert!(plan(17.0, state).waived_at.is_some(), "{state:?}");
    }
}

#[test]
fn the_waiver_instant_does_not_move_with_the_clock() {
    let (policy, pr) = waiting(20.0);
    let first = schedule(&policy, &pr, "thepastaclaw", None).waived_at;
    let later = bot_schedule(&policy, &pr, "thepastaclaw", &s(&ago(-5.0)), &PyValue::None)
        .unwrap()
        .waived_at;
    assert_eq!(first, later);
    assert!(first.as_deref().unwrap() < NOW);
}

#[test]
fn a_repository_without_timeouts_never_nudges_or_waives() {
    let (mut policy, pr) = waiting(100.0);
    remove(&mut policy, &["bot_timeouts"]);
    assert_eq!(schedule(&policy, &pr, "thepastaclaw", None), NOTHING);
}

#[test]
fn a_head_the_controller_has_not_reported_on_has_no_clock() {
    let (policy, mut pr) = waiting(100.0);
    set(&mut pr, &["head_seen_at"], PyValue::None);
    assert_eq!(schedule(&policy, &pr, "thepastaclaw", None), NOTHING);
}
