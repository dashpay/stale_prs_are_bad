//! `test_main.py` (`PublicationTests`, `NudgeTests`),
//! `test_multi_repo_main.py` and `test_configuration_isolation.py`, as
//! scenarios on the fake.

use crate::fake::*;
use crate::scene::*;
use crate::support::*;
use pr_hygiene_engine::policy::nudged_at;
use pr_hygiene_engine::reconcile::{
    admission_conflicts, context_fingerprint, evaluate_snapshots, review_text, Command, Selection,
};

/// The engine's record of pull request `n`, admitting it at `admitted_at`.
pub fn record(n: i64, head: &str, admitted_at: Option<&str>, state: &str) -> PyValue {
    py(
        json!({"number": n, "head": head, "admitted_at": admitted_at, "ready_since": null,
              "state": state, "version": 1, "evidence": "e".repeat(64), "context": "f".repeat(64)}),
    )
}

/// The engine's record comment on pull request `n`.
pub fn recorded(
    id: i64,
    n: i64,
    head: &str,
    admitted_at: Option<&str>,
    state: &str,
    at: &str,
) -> Comment {
    Comment::new(
        id,
        ENGINE,
        &record_comment(
            &record(n, head, admitted_at, state),
            "PR Hygiene: the checklist is in the description.",
            None,
        ),
        at,
    )
}

fn is_call(call: &Call, method: Method, ends: &str) -> bool {
    matches!(call, Call::Rest { method: m, path, .. } if *m == method && path.ends_with(ends))
}

/// How many `fragment history` queries were made, and what they named.
fn history_queries(fake: &Fake) -> Vec<Vec<String>> {
    let alias = regex::Regex::new(r"pr([0-9]+): pullRequest").expect("a pattern");
    fake.calls
        .iter()
        .filter_map(|call| match call {
            Call::Graphql { query, .. } if query.contains("fragment history") => Some(
                alias
                    .captures_iter(query)
                    .map(|found| found[1].to_owned())
                    .collect(),
            ),
            _ => None,
        })
        .collect()
}

/// Which pull requests' files were read: whose full evidence was taken.
fn snapshotted(fake: &Fake) -> Vec<String> {
    fake.calls
        .iter()
        .filter_map(|call| match call {
            Call::Rest { path, .. } if path.contains("/files?") => {
                path.split('/').nth(4).map(str::to_owned)
            }
            _ => None,
        })
        .collect()
}

#[test]
fn preview_never_mutates_github() {
    let (policy, fake) = fixture();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    scene.fake.forget_calls();
    let written = scene
        .with(|engine| {
            engine.publish(
                &policy,
                &pr,
                &result,
                std::slice::from_ref(&pr),
                false,
                None,
            )
        })
        .expect("nothing to do");
    assert!(written.is_none());
    assert!(scene.fake.calls.is_empty(), "not even a read");
}

#[test]
fn a_review_change_during_publication_cannot_publish_success() {
    let (policy, mut fake) = fixture();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    // A reviewer asks for changes while the engine writes.
    fake.on_call(|state, call| {
        if matches!(call, Call::Rest { method: Method::Post, path, .. } if path.contains("/statuses/")) {
            let changes = review(99, "reviewer", "CHANGES_REQUESTED", HEAD, LATER, "no");
            if !state.pr(1).reviews.contains(&changes) {
                state.pr(1).reviews.push(changes);
            }
        }
    });
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    assert!(
        !scene.statuses().iter().any(|(state, _)| state == "success"),
        "{:?}",
        scene.statuses()
    );
}

#[test]
fn a_new_head_aborts_before_any_mutation() {
    let (policy, fake) = fixture();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let result = scene.verdict(&policy, 1, &s(NOW));
    scene.fake.pr(1).head = "c".repeat(40);
    scene.fake.forget_calls();
    scene
        .with(|engine| engine.publish(&policy, &pr, &result, std::slice::from_ref(&pr), true, None))
        .expect("nothing to publish");
    assert_eq!(scene.writes(), Vec::<String>::new());
}

#[test]
fn a_users_report_names_the_areas_they_are_asked_for() {
    let row = py(
        json!({"number": 2, "author": "bob", "state": "ready-for-human",
        "reviewers": ["alice", "carol"], "approvals": [{"area": "core", "files": [],
        "approvers": ["alice", "carol"], "approved_by": [], "owned": false}]}),
    );
    assert_eq!(
        review_text(&row, Some("alice")).expect("words"),
        "needs `core`: alice or carol; your part: `core` (you or carol)"
    );
}

#[test]
fn a_foreign_controller_history_is_rejected() {
    let (policy, mut fake) = fixture();
    // The engine's record on pull request 1 says it is pull request 2's.
    fake.pr(1)
        .comments
        .push(recorded(7, 2, HEAD, Some(NOW), "waiting-bots", NOW));
    let mut scene = Scene::new(fake);
    let read = scene.with(|engine| engine.collect(&policy, &mut Selection::All, false, false));
    assert!(
        matches!(&read, Err(ReadError::GitHub(m)) if m == "Controller admission history belongs to another PR"),
        "{read:?}"
    );
}

#[test]
fn unreadable_history_revokes_every_known_head_only_in_apply() {
    // Admission is decided from every candidate's history, so a history
    // that cannot be read leaves no pull request's slot knowable.
    for apply in [true, false] {
        let (policy, mut fake) = fixture();
        fake.add(Pr::new(2, "reviewer", &"b".repeat(40)));
        fake.refuse(
            Method::Post,
            "graphql",
            Refusal::Http(502, "missing history".into()),
        );
        let mut scene = Scene::new(fake);
        let read = scene.with(|engine| engine.collect(&policy, &mut Selection::All, apply, true));
        assert!(matches!(read, Err(ReadError::GitHub(_))));
        let errors: Vec<String> = scene
            .wrote(Method::Post, "statuses/")
            .iter()
            .map(|w| w.route.clone())
            .collect();
        if apply {
            assert_eq!(
                errors,
                [
                    format!("statuses/{HEAD}"),
                    format!("statuses/{}", "b".repeat(40))
                ]
            );
            assert!(scene.statuses().iter().all(|(state, _)| state == "error"));
        } else {
            assert_eq!(errors, Vec::<String>::new());
        }
    }
}

#[test]
fn unreadable_evidence_marks_only_the_pull_request_it_belongs_to() {
    // One rate-limited read used to mark everything selected an error — and
    // a full pass selects everything open. Admission is already decided by
    // then; the one pull request says so, the rest proceed.
    let (policy, mut fake) = fixture();
    fake.add(Pr::new(2, "owner", &"b".repeat(40)));
    fake.refuse(
        Method::Get,
        "pulls/1/files",
        Refusal::Http(403, "API rate limit exceeded".into()),
    );
    let mut scene = Scene::new(fake);
    let collected = scene
        .with(|engine| engine.collect(&policy, &mut Selection::All, true, true))
        .expect("collected");
    let numbers: Vec<String> = collected
        .snapshots
        .iter()
        .map(|pr| dump(field(pr, "number")))
        .collect();
    assert_eq!(numbers, ["2"], "the readable one is reconciled");
    assert_eq!(
        scene.writes(),
        [format!("POST statuses/{HEAD}")],
        "only the unreadable head"
    );
    assert!(scene.said("PR #1: GitHub API command failed (exit 1): gh: API rate limit exceeded (HTTP 403); its status says so"));
}

#[test]
fn a_surplus_of_admissions_is_detected() {
    let policy = fixture_policy();
    let candidates: Vec<PyValue> = (1..=6)
        .map(|n| {
            py(
                json!({"number": n, "author": "alice", "base": "v4.2-dev", "state": "open",
                      "draft": false, "controller_state": {"admitted_at": NOW}}),
            )
        })
        .collect();
    assert_eq!(
        admission_conflicts(&policy, &candidates).expect("counted"),
        ["alice".to_owned()].into_iter().collect()
    );
}

#[test]
fn another_pull_request_admitted_meanwhile_blocks_success() {
    // The author's other pull request took a slot while this one was being
    // published: the admission it was decided under no longer holds.
    let (policy, mut fake) = fixture();
    let mut other = Pr::new(2, "owner", &"b".repeat(40));
    other.draft = true;
    fake.add(other);
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    fake.on_call(|state, call| {
        if matches!(call, Call::Rest { method: Method::Post, path, .. } if path.contains("/statuses/")) {
            let admitted = recorded(77, 2, &"b".repeat(40), Some(NOW), "waiting-bots", NOW);
            if !state.pr(2).comments.iter().any(|c| c.id == 77) {
                state.pr(2).comments.push(admitted);
            }
        }
    });
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    assert!(
        !scene.statuses().iter().any(|(state, _)| state == "success"),
        "{:?}",
        scene.statuses()
    );
}

#[test]
fn a_new_head_before_the_request_does_not_invite_reviewers() {
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    // A push lands as the labels go on, just before reviewers are asked.
    fake.on_call(|state, call| {
        if is_call(call, Method::Post, "/issues/1/labels") {
            state.pr(1).head = "c".repeat(40);
        }
    });
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    assert_eq!(
        scene
            .wrote(Method::Post, "pulls/1/requested_reviewers")
            .len(),
        0
    );
    assert_eq!(scene.wrote(Method::Post, "issues/1/labels").len(), 1);
}

#[test]
fn an_event_on_a_closed_pull_request_reconciles_only_the_authors_others() {
    // Collection is scoped to the author, and the pull request that closed
    // releases a slot one of their others now takes.
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "alice".into();
    fake.pr(1).state = "closed".into();
    fake.add(Pr::new(2, "alice", &"b".repeat(40)));
    fake.add(Pr::new(3, "bob", &"c".repeat(40)));
    let mut scene = Scene::new(fake);
    let collected = scene
        .with(|engine| engine.collect(&policy, &mut Selection::Pr(PyInt::from(1)), false, true))
        .expect("collected");
    let candidates: Vec<String> = collected
        .candidates
        .iter()
        .map(|pr| dump(field(pr, "number")))
        .collect();
    assert_eq!(candidates, ["2"]);
    assert_eq!(snapshotted(&scene.fake), ["2"]);
    assert_eq!(
        history_queries(&scene.fake)[0],
        ["2"],
        "one query, naming only the author's other"
    );
}

#[test]
fn a_report_on_one_pull_request_does_not_read_its_siblings_evidence() {
    let (policy, mut fake) = fixture();
    fake.add(Pr::new(2, "owner", &"b".repeat(40)));
    let mut scene = Scene::new(fake);
    let collected = scene
        .with(|engine| engine.collect(&policy, &mut Selection::Pr(PyInt::from(1)), false, false))
        .expect("collected");
    assert_eq!(collected.candidates.len(), 2);
    assert_eq!(snapshotted(&scene.fake), ["1"]);
}

#[test]
fn a_refused_ready_label_does_not_abort_the_pull_request() {
    // Its sibling write, the waiver label, has survived its own failure
    // from the start.
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    fake.refuse(
        Method::Post,
        "issues/1/labels",
        Refusal::Http(422, "label does not exist".into()),
    );
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert!(run.failure().is_none());
    assert!(scene.said("PR #1: could not set the state label"));
    let writes = scene.writes();
    let refused = writes
        .iter()
        .position(|w| w == "POST issues/1/labels")
        .expect("the label was asked for");
    assert!(
        writes[refused..]
            .iter()
            .any(|w| w.starts_with("POST statuses/")),
        "the run carried on and published a status: {writes:?}"
    );
}

#[test]
fn someone_elses_push_does_not_demote_a_ready_pull_request() {
    // Only the author's own pull requests decide their slots, and of those
    // only which are open and not drafts.
    let pr = |n: i64, author: &str, head: char, draft: bool, state: &str| {
        py(
            json!({"number": n, "author": author, "head": head.to_string().repeat(40),
                  "base": "v4.2-dev", "draft": draft, "state": state}),
        )
    };
    let me = s("me");
    let before = context_fingerprint(
        &[
            pr(1, "me", 'a', false, "open"),
            pr(2, "someone", 'b', false, "open"),
        ],
        &me,
    )
    .expect("a print");
    let same = |prs: &[PyValue]| context_fingerprint(prs, &me).expect("a print") == before;
    assert!(same(&[
        pr(1, "me", 'a', false, "open"),
        pr(2, "someone", 'c', false, "open")
    ]));
    assert!(same(&[
        pr(1, "me", 'd', false, "open"),
        pr(2, "someone", 'b', false, "open")
    ]));
    assert!(!same(&[
        pr(1, "me", 'a', true, "open"),
        pr(2, "someone", 'b', false, "open")
    ]));
    assert!(!same(&[
        pr(1, "me", 'a', false, "closed"),
        pr(2, "someone", 'b', false, "open")
    ]));
}

#[test]
fn a_surplus_of_admissions_heals_instead_of_erroring_the_author() {
    // Two runs reconciling two pull requests of one author can both admit
    // past the check. The five oldest keep their slots; the sixth waits.
    let (policy, mut fake) = fixture();
    fake.state.prs.clear();
    for n in 1..=6 {
        let head = n.to_string().repeat(40);
        let mut pr = Pr::new(n, "busy", &head);
        pr.comments.push(recorded(
            500 + n,
            n,
            &head,
            Some(&format!("2026-09-10T0{n}:00:00Z")),
            "waiting-bots",
            NOW,
        ));
        fake.add(pr);
    }
    let mut scene = Scene::new(fake);
    let collected = scene
        .with(|engine| engine.collect(&policy, &mut Selection::All, false, false))
        .expect("collected");
    let mut log = Vec::new();
    let rows = evaluate_snapshots(
        &policy,
        &collected.prs,
        &collected.candidates,
        &collected.snapshots,
        LATER,
        &PyValue::None,
        &mut log,
    )
    .expect("verdicts");
    let states: Vec<&str> = rows.iter().map(|r| text(field(r, "state"))).collect();
    assert!(!states.contains(&"configuration-error"), "{states:?}");
    assert_eq!(
        states[5], "too-many-open-prs",
        "the newest admission is the one that yields"
    );
    assert!(states[..5]
        .iter()
        .all(|state| *state != "too-many-open-prs"));
    assert_eq!(
        log,
        ["busy: more than five persisted admissions; keeping the five oldest"]
    );
}

#[test]
fn labels_read_like_the_status() {
    // One state label at a time, the waiver beside it, unrelated labels
    // untouched — so a pull request list says what the status says.
    //
    // The retired `ready-to-merge` is cleared from a pull request that is
    // ready to merge: Python's own table calls those labels correct, and
    // passes only because its re-check of the admission stops that
    // publication first. With the record really on the pull request, the
    // label goes, as `set_state_label` says retired names do.
    for (state, waived, worn, correct) in [
        ("waiting-bots", false, &["waiting-bots"][..], true),
        ("waiting-bots", false, &["ready-for-human"][..], false),
        (
            "waiting-bots",
            true,
            &["waiting-bots", "bot-review-skipped", "bug"][..],
            true,
        ),
        ("waiting-bots", true, &["waiting-bots"][..], false),
        (
            "ready-to-merge",
            false,
            &["ready-to-merge", "bug"][..],
            false,
        ),
        ("ready-to-merge", false, &["bug"][..], true),
        ("draft", false, &["waiting-bots"][..], false),
        ("draft", false, &["bug"][..], true),
    ] {
        let (policy, mut fake) = fixture();
        fake.pr(1).labels = worn.iter().map(|l| (*l).to_owned()).collect();
        fake.pr(1).comments.push(recorded(
            50,
            1,
            HEAD,
            Some("2026-09-11T00:00:00Z"),
            state,
            NOW,
        ));
        let mut scene = Scene::new(fake);
        let pr = scene.snapshot(&policy, 1);
        let result = py(
            json!({"number": 1, "head": HEAD, "author": "owner", "state": state,
            "status": if state == "ready-to-merge" { "success" } else { "pending" },
            "blockers": [], "reviewers": [], "areas": ["drive"],
            "admitted_at": "2026-09-11T00:00:00Z", "ready_since": null,
            "waived": if waived { json!(["thepastaclaw"]) } else { json!([]) }}),
        );
        scene.fake.forget_calls();
        scene
            .with(|engine| {
                engine.publish(&policy, &pr, &result, std::slice::from_ref(&pr), true, None)
            })
            .expect("published");
        let labelled = scene.writes().iter().any(|w| w.contains("/labels"));
        assert_eq!(
            labelled,
            !correct,
            "{state} {waived} {worn:?}: {:?}",
            scene.writes()
        );
    }
}

#[test]
fn a_pass_gives_every_pull_request_its_turn_before_failing() {
    // Stopping at the first failure left the rest with whatever status they
    // had — on a full pass, possibly a passing one from before the check
    // became the gate. Every one is attempted; then the run fails.
    let (policy, mut fake) = fixture();
    fake.add(Pr::new(2, "reviewer", &"b".repeat(40)));
    fake.add(Pr::new(3, "fallback", &"c".repeat(40)));
    fake.refuse(
        Method::Post,
        "issues/1/comments",
        Refusal::Http(500, "boom".into()),
    );
    let mut scene = Scene::new(fake);
    let run = scene
        .run(&policy, Command::Sync, Pick::All, None)
        .expect("the run ends");
    assert_eq!(
        run.failure().map(|f| f.to_string()).as_deref(),
        Some("reconciliation failed for #1")
    );
    for head in ["b".repeat(40), "c".repeat(40)] {
        assert!(
            !scene
                .wrote(Method::Post, &format!("statuses/{head}"))
                .is_empty(),
            "the ones after the failure still ran"
        );
    }
}

#[test]
fn a_run_aimed_at_one_pull_request_does_not_sweep_the_repository() {
    // `--pr N` is what a pull request event runs, dozens of times an hour.
    // Clearing marks there would write to pull requests the event never
    // named.
    for (aimed, sweeps) in [(true, false), (false, true)] {
        let (policy, mut fake) = fixture();
        let mut away = Pr::new(5, "owner", &"e".repeat(40));
        away.base = "feature/x".into();
        away.labels = vec!["waiting-bots".into()];
        fake.add(away);
        let mut scene = Scene::new(fake);
        let pick = if aimed { Pick::Pr(1) } else { Pick::All };
        scene
            .run(&policy, Command::Sync, pick, None)
            .expect("the run completes");
        let cleared = !scene.wrote(Method::Delete, "issues/5/labels/").is_empty();
        assert_eq!(cleared, sweeps, "aimed at one: {aimed}");
    }
}

#[test]
fn a_draft_records_its_state_without_opening_a_comment() {
    let (policy, mut fake) = fixture();
    fake.pr(1).draft = true;
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    assert!(scene.wrote(Method::Post, "issues/1/comments").is_empty());
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("pending".to_owned(), "draft".to_owned())
    );
}

#[test]
fn a_batch_bounds_the_snapshots_and_reads_its_authors_history_once() {
    let (policy, mut fake) = fixture();
    fake.state.prs.clear();
    for n in 1..=68 {
        fake.add(Pr::new(n, &format!("user{n}"), &format!("{n:040}")));
    }
    let mut scene = Scene::new(fake);
    let collected = scene
        .run(&policy, Command::Report, Pick::Batch(vec![1, 2, 3]), None)
        .expect("a report");
    assert_eq!(collected.candidates.len(), 3);
    assert_eq!(snapshotted(&scene.fake), ["1", "2", "3"]);
    // Three candidates, still one query, not one request each.
    assert_eq!(history_queries(&scene.fake), [["1", "2", "3"]]);
}

#[test]
fn an_event_does_not_rescan_the_authors_unchanged_pull_requests() {
    let (policy, mut fake) = fixture();
    fake.state.prs.clear();
    for n in 1..=14 {
        let head = format!("{n:040}");
        let mut pr = Pr::new(n, "alice", &head);
        if n <= 5 {
            pr.comments
                .push(recorded(500 + n, n, &head, Some(NOW), "waiting-bots", NOW));
        }
        fake.add(pr);
    }
    let mut scene = Scene::new(fake);
    scene
        .with(|engine| engine.collect(&policy, &mut Selection::Pr(PyInt::from(2)), false, true))
        .expect("collected");
    assert_eq!(snapshotted(&scene.fake), ["2"]);
}

#[test]
fn a_close_event_refreshes_the_newly_admitted_waiter() {
    let (policy, mut fake) = fixture();
    fake.state.prs.clear();
    for n in 1..=7 {
        let head = format!("{n:040}");
        let mut pr = Pr::new(n, "alice", &head);
        if n <= 5 {
            pr.comments
                .push(recorded(500 + n, n, &head, Some(NOW), "waiting-bots", NOW));
        }
        if n == 2 {
            pr.state = "closed".into();
        }
        fake.add(pr);
    }
    let mut scene = Scene::new(fake);
    scene
        .with(|engine| engine.collect(&policy, &mut Selection::Pr(PyInt::from(2)), false, true))
        .expect("collected");
    assert_eq!(snapshotted(&scene.fake), ["6"]);
}

#[test]
fn a_settled_success_rechecks_the_reviews_after_the_admission_reads() {
    // Admission history reads can be slow, so the evidence is read after
    // them: an approval dismissed meanwhile is not reused from before them.
    let (policy, mut fake) = fixture();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    scene.sync_pr(&policy, 1);
    scene.fake.forget_calls();
    let mut histories = 0;
    scene.fake.on_call(move |state, call| {
        if matches!(call, Call::Graphql { query, .. } if query.contains("fragment history")) {
            histories += 1;
            // The re-check's own admission read, before its last snapshot.
            if histories == 4 {
                state.pr(1).reviews[1]["state"] = json!("DISMISSED");
            }
        }
    });
    scene.sync_pr(&policy, 1);
    assert_eq!(
        scene.statuses(),
        [(
            "pending".to_owned(),
            "Review evidence changed; reconciliation required".to_owned()
        )]
    );
}

/// `NudgeTests`: asking a bot to look at a head, a few times per run, best
/// effort, never at the reconciliation's cost.
fn nudge(bots: &[&str], allowance: usize, fake: Fake) -> (usize, Scene) {
    let mut scene = Scene::new(fake);
    let pr = py(json!({"number": 1, "head": HEAD, "state": "open"}));
    let result = py(json!({"nudge": bots}));
    let posted = scene
        .with(|engine| engine.nudge(&pr, &result, allowance))
        .expect("asked");
    (posted, scene)
}

#[test]
fn a_run_whose_allowance_is_spent_asks_nobody() {
    // Once one pull request has used it, the rest of a sweep must not
    // mention a bot at all — not even read the pull request to decide.
    let (posted, scene) = nudge(&["thepastaclaw", "coderabbitai"], 0, fixture().1);
    assert_eq!(posted, 0);
    assert!(scene.fake.calls.is_empty());
}

#[test]
fn no_more_comments_than_the_allowance() {
    // Every nudge is a comment that notifies everyone on the pull request.
    let (posted, scene) = nudge(&["thepastaclaw", "coderabbitai"], 1, fixture().1);
    assert_eq!(posted, 1);
    let asked = scene.wrote(Method::Post, "issues/1/comments");
    assert_eq!(asked.len(), 1);
    assert!(text(asked[0].field("body")).contains("@thepastaclaw review"));
}

#[test]
fn the_nudge_carries_the_marker_that_stops_a_second_ask() {
    // The policy reads the marker back to know this head was asked about;
    // a comment it did not recognise would ask the bot again every run.
    let (_, scene) = nudge(&["thepastaclaw"], 1, fixture().1);
    let body = text(scene.wrote(Method::Post, "issues/1/comments")[0].field("body")).to_owned();
    assert!(body.starts_with(&format!(
        "<!-- pr-hygiene-nudge v1 bot=thepastaclaw sha={HEAD} -->\n@thepastaclaw review\n"
    )));
    let posted = py(json!([{"user": ENGINE, "body": body, "created_at": NOW}]));
    let asked = nudged_at(&posted, "thepastaclaw", &py(json!([HEAD]))).expect("read");
    assert_eq!(asked.as_ref().map(text), Some(NOW));
}

#[test]
fn a_nudge_that_cannot_be_posted_is_retried_next_run_and_costs_nothing() {
    // Nothing was asked, so the run's one ask is still there for the next
    // bot.
    let (_, mut fake) = fixture();
    fake.refuse_once(
        Method::Post,
        "issues/1/comments",
        Refusal::Http(403, "Forbidden".into()),
    );
    let (posted, scene) = nudge(&["thepastaclaw", "coderabbitai"], 1, fake);
    assert_eq!(posted, 1);
    assert!(scene.said("PR #1: could not ask thepastaclaw to review; will retry"));
    let asked: Vec<String> = scene
        .wrote(Method::Post, "issues/1/comments")
        .iter()
        .map(|w| {
            text(w.field("body"))
                .lines()
                .nth(1)
                .unwrap_or("")
                .to_owned()
        })
        .collect();
    assert_eq!(asked, ["@thepastaclaw review", "@coderabbitai review"]);
}

#[test]
fn a_nudge_stops_at_a_head_that_moved() {
    let (_, mut fake) = fixture();
    fake.pr(1).head = "c".repeat(40);
    let (posted, scene) = nudge(&["thepastaclaw"], 1, fake);
    assert_eq!(posted, 0);
    assert_eq!(scene.writes(), Vec::<String>::new());
}

/// `test_multi_repo_main.py`: one engine, many repositories.
#[test]
fn the_same_number_in_another_repository_keeps_its_identity() {
    let (policy, fake) = fixture();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    for repo in ["dashpay/platform", "dashpay/rust-dashcore"] {
        let mut policy = policy.clone();
        if let PyValue::Dict(fields) = &mut policy {
            fields.insert("repository".into(), s(repo));
        }
        let one = std::slice::from_ref(&pr);
        let rows = evaluate_snapshots(&policy, one, one, one, NOW, &PyValue::None, &mut Vec::new())
            .expect("verdicts");
        assert_eq!(text(field(&rows[0], "repository")), repo);
        assert_eq!(dump(field(&rows[0], "number")), "1");
    }
}

#[test]
fn a_head_another_open_pull_request_shares_is_an_error() {
    let (policy, fake) = fixture();
    let mut scene = Scene::new(fake);
    let pr = scene.snapshot(&policy, 1);
    let mut other = pr.clone();
    if let PyValue::Dict(fields) = &mut other {
        fields.insert("number".into(), PyValue::Int(PyInt::from(2)));
    }
    let one = std::slice::from_ref(&pr);
    let rows = evaluate_snapshots(
        &policy,
        &[pr.clone(), other],
        one,
        one,
        NOW,
        &PyValue::None,
        &mut Vec::new(),
    )
    .expect("verdicts");
    assert_eq!(text(field(&rows[0], "status")), "error");
    let blockers = items(field(&rows[0], "blockers"));
    assert!(text(blockers.last().expect("a blocker")).contains("shares this head"));
}

/// `test_configuration_isolation.py`: a broken policy must not let one
/// event overwrite the repository's queue. The run that met it marks at
/// most the pull request the event named.
fn configuration(
    change: impl FnOnce(&mut Fake),
    policy: impl FnOnce(&mut PyValue),
    asked: Option<i64>,
) -> Scene {
    let (mut broken, mut fake) = fixture();
    if let PyValue::Dict(fields) = &mut broken {
        fields.insert("future_unknown_field".into(), PyValue::Bool(true));
    }
    policy(&mut broken);
    let mut other = Pr::new(2, "another-author", &"b".repeat(40));
    other.created_at = "2026-09-10T01:00:00Z".into();
    fake.add(other);
    change(&mut fake);
    let mut scene = Scene::new(fake);
    let asked = asked.map(PyInt::from);
    scene
        .with(|engine| {
            engine.mark_configuration_error(
                Some(&broken),
                asked.as_ref(),
                Some("https://github.com/dashpay/platform/actions/runs/36384504598"),
            )
        })
        .expect("never fatal");
    scene
}

fn pulls_read(scene: &Scene) -> usize {
    scene
        .fake
        .calls
        .iter()
        .filter(|call| is_call(call, Method::Get, "/pulls/1"))
        .count()
}

fn listed(scene: &Scene) -> bool {
    scene
        .fake
        .calls
        .iter()
        .any(|call| is_call(call, Method::Get, "/pulls?state=open&per_page=100"))
}

#[test]
fn an_unknown_schema_field_invalidates_only_the_explicit_current_head() {
    let scene = configuration(|_| {}, |_| {}, Some(1));
    assert_eq!(
        scene.statuses(),
        [(
            "error".to_owned(),
            "Invalid policy configuration; inspect workflow log".to_owned()
        )]
    );
    assert_eq!(
        scene.wrote(Method::Post, "statuses/")[0].route,
        format!("statuses/{HEAD}")
    );
    assert_eq!(
        text(scene.fake.written[0].field("target_url")),
        "https://github.com/dashpay/platform/actions/runs/36384504598"
    );
    assert_eq!(pulls_read(&scene), 2);
}

#[test]
fn a_draft_stacked_pull_request_cannot_invalidate_unrelated_heads() {
    // platform#5113: a draft on a feature base, with other PRs already green.
    let scene = configuration(
        |fake| {
            fake.pr(1).draft = true;
            fake.pr(1).base = "chore/bump-rust-dashcore-secp-033".into();
        },
        |_| {},
        Some(1),
    );
    assert!(!listed(&scene));
    assert_eq!(scene.writes(), Vec::<String>::new());
}

#[test]
fn a_draft_a_closed_and_an_out_of_scope_target_are_not_written() {
    for change in [
        |fake: &mut Fake| fake.pr(1).draft = true,
        |fake: &mut Fake| fake.pr(1).state = "closed".into(),
        |fake: &mut Fake| fake.pr(1).base = "feature/stack".into(),
    ] {
        let scene = configuration(change, |_| {}, Some(1));
        assert_eq!(scene.writes(), Vec::<String>::new());
    }
}

#[test]
fn a_sweep_does_not_guess_a_write_scope() {
    let scene = configuration(|_| {}, |_| {}, None);
    assert!(scene.fake.calls.is_empty());
}

#[test]
fn a_head_shared_with_another_pull_request_is_not_an_isolated_target() {
    let scene = configuration(|fake| fake.pr(2).head = HEAD.into(), |_| {}, Some(1));
    assert_eq!(scene.writes(), Vec::<String>::new());
    assert!(scene.said("PR #1: shared head; configuration error reported by the workflow only"));
}

#[test]
fn a_head_or_scope_that_changes_under_the_check_aborts_the_write() {
    let changes: [fn(&mut Pr); 4] = [
        |pr| pr.head = "c".repeat(40),
        |pr| pr.base = "feature/stack".into(),
        |pr| pr.state = "closed".into(),
        |pr| pr.draft = true,
    ];
    for change in changes {
        let scene = configuration(
            |fake| {
                let mut read = 0;
                fake.on_call(move |state, call| {
                    if is_call(call, Method::Get, "/pulls/1") {
                        read += 1;
                        if read == 2 {
                            change(state.pr(1));
                        }
                    }
                });
            },
            |_| {},
            Some(1),
        );
        assert_eq!(scene.writes(), Vec::<String>::new());
    }
}

#[test]
fn an_unreadable_repository_or_branch_scope_is_reported_only_in_the_job() {
    let changes: [fn(&mut PyValue); 4] = [
        |p| set(p, "repository", s("dashpay/other")),
        |p| set(p, "target_branches", PyValue::None),
        |p| set(p, "target_branches", py(json!([]))),
        |p| set(p, "target_branches", py(json!([1]))),
    ];
    for change in changes {
        let scene = configuration(|_| {}, change, Some(1));
        assert!(scene.fake.calls.is_empty());
    }
}

#[test]
fn an_identity_that_cannot_be_read_neither_widens_the_scope_nor_fails() {
    let scene = configuration(
        |fake| {
            fake.refuse(
                Method::Get,
                "pulls/1",
                Refusal::Http(503, "unavailable".into()),
            );
        },
        |_| {},
        Some(1),
    );
    assert!(!listed(&scene));
    assert_eq!(scene.writes(), Vec::<String>::new());
    assert!(scene.said("PR #1: unable to publish configuration error status"));
}

fn set(policy: &mut PyValue, key: &str, value: PyValue) {
    if let PyValue::Dict(fields) = policy {
        fields.insert(key.into(), value);
    }
}
