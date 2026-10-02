//! What the corpus cannot show, shown on the stateful fake: the engine
//! reading its own writes back, and the writes GitHub refuses.

use crate::fake::*;
use crate::scene::*;
use crate::support::*;
use pr_hygiene_engine::reconcile::Command;

/// Pull requests in every shape a run leaves differently: a first
/// admission of a ready-to-merge pull request, one that asks a reviewer,
/// one whose author is told their move, a draft with a stale checklist,
/// and one rebased off the policy still wearing the engine's marks.
fn mixed() -> (PyValue, Fake) {
    let mut fake = Fake::new(LATER);
    fake.add(Pr::new(1, "owner", HEAD));
    let mut asks = Pr::new(2, "reviewer", &"b".repeat(40));
    asks.created_at = "2026-09-10T01:00:00Z".into();
    fake.add(asks);
    let mut told = Pr::new(3, "reviewer", &"c".repeat(40));
    told.comments.clear();
    fake.add(told);
    let mut draft = Pr::new(4, "owner", &"d".repeat(40));
    draft.draft = true;
    draft.body = "Draft.\n\n<!-- pr-hygiene:start -->\nold\n<!-- pr-hygiene:end -->".into();
    fake.add(draft);
    let mut away = Pr::new(5, "owner", &"e".repeat(40));
    away.base = "feature/x".into();
    away.labels = vec!["waiting-bots".into(), "bug".into()];
    away.body = "Away.\n\n<!-- pr-hygiene:start -->\nold\n<!-- pr-hygiene:end -->".into();
    fake.add(away);
    // Every head but the draft's was seen by an earlier run.
    for (head, at) in [
        (HEAD.to_owned(), "2026-09-11T09:00:00Z"),
        ("b".repeat(40), "2026-09-11T09:00:00Z"),
        ("c".repeat(40), "2026-09-11T09:00:00Z"),
    ] {
        fake.state
            .engine_status(&head, "pending", "waiting-bots", at);
    }
    (fixture_policy(), fake)
}

#[test]
fn replaying_the_state_after_the_writes_produces_no_writes() {
    let (policy, fake) = mixed();
    let mut scene = Scene::new(fake);
    let first = scene.sync_all(&policy);
    assert!(first.failure().is_none());
    let wrote = scene.writes();
    assert!(wrote.len() > 10, "the first run has work to do: {wrote:?}");
    // The run's own writes, read back by the next, leave one thing to do,
    // and only for the pull request first asked for a review: its waiting
    // time starts once a record shows the admission, and the first run is
    // the one that writes that record. Python does the same; the record is
    // refreshed in place, silently, and the status put back.
    scene.fake.forget_calls();
    let second = scene.sync_all(&policy);
    let b = "b".repeat(40);
    let patched = scene.wrote(Method::Patch, "issues/comments/");
    assert_eq!(patched.len(), 1, "{:?}", scene.writes());
    assert!(text(patched[0].field("body")).contains(r#""ready_since":"2026-09-11T14:00:00Z""#));
    assert_eq!(
        scene.writes(),
        [
            format!("POST statuses/{b}"),
            patched[0].named(),
            format!("POST statuses/{b}"),
        ],
        "{:?}",
        scene.log
    );
    // After that, nothing: what the engine wrote is what it would write.
    scene.fake.forget_calls();
    let third = scene.sync_all(&policy);
    assert_eq!(scene.writes(), Vec::<String>::new(), "{:?}", scene.log);
    // And no write moved a verdict.
    let states = |run: &pr_hygiene_engine::reconcile::Run| -> Vec<String> {
        run.verdicts
            .iter()
            .map(|v| text(field(v, "state")).to_owned())
            .collect()
    };
    assert_eq!(states(&first), states(&second));
    assert_eq!(states(&second), states(&third));
    assert_eq!(
        states(&third),
        [
            "ready-to-merge",
            "ready-for-human",
            "waiting-self-review",
            "draft"
        ]
    );
}

#[test]
fn a_settled_pull_request_costs_reads_and_no_writes() {
    // Settled: admitted, recorded, described, labelled, its reviewer
    // asked and its status posted by earlier runs. An event on it reads it
    // again and writes nothing, the status included.
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    scene.sync_pr(&policy, 1);
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(scene.writes(), Vec::<String>::new());
    assert!(scene.fake.calls.len() > 10, "it did read");
}

#[test]
fn a_head_never_seen_settles_on_the_second_run() {
    // The first status the engine posts is when it first saw the head, and
    // that time is evidence: the re-check before a success reads it where
    // the verdict did not, and holds the status back. The next run reads
    // it both times, publishes, and the one after has nothing to do.
    let (policy, fake) = fixture();
    let mut scene = Scene::new(fake);
    scene.sync_pr(&policy, 1);
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &(
            "pending".to_owned(),
            "Review evidence changed; reconciliation required".to_owned()
        )
    );
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(
        scene.statuses(),
        [("success".to_owned(), "ready-to-merge".to_owned())]
    );
    scene.fake.forget_calls();
    scene.sync_pr(&policy, 1);
    assert_eq!(scene.writes(), Vec::<String>::new());
}

#[test]
fn a_first_admission_completes() {
    // The run admits the pull request, writes the record, reads it back in
    // the re-check before the success and finds its own admission there:
    // the verdict is published, not held as "reconciliation required".
    for (author, status, state) in [
        ("owner", "success", "ready-to-merge"),
        ("reviewer", "pending", "ready-for-human"),
    ] {
        let (policy, mut fake) = fixture();
        fake.pr(1).author = author.into();
        fake.pr(1).comments[0].author = author.into();
        fake.state
            .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
        let mut scene = Scene::new(fake);
        let run = scene.sync_pr(&policy, 1);
        assert_eq!(text(field(&run.verdicts[0], "admitted_at")), LATER);
        assert_eq!(
            scene.statuses().last().expect("a status"),
            &(status.to_owned(), state.to_owned()),
            "{author}"
        );
        let record = field(&scene.snapshot(&policy, 1), "controller_state").clone();
        assert_eq!(text(field(&record, "admitted_at")), LATER, "{author}");
        assert_eq!(text(field(&record, "state")), state, "{author}");
    }
}

#[test]
fn a_label_that_is_already_gone_does_not_stop_the_status() {
    // The labels are read from a snapshot another run can invalidate:
    // removing one already gone is a 404, said and passed over.
    let (policy, mut fake) = fixture();
    fake.pr(1).labels = vec!["waiting-bots".into()];
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    // Another run takes the label off once this one has read it.
    fake.on_call(|state, call| {
        if matches!(call, Call::Rest { method: Method::Post, path, .. } if path.contains("/statuses/")) {
            state.pr(1).labels.clear();
        }
    });
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert!(run.failure().is_none());
    assert_eq!(
        scene
            .wrote(Method::Delete, "issues/1/labels/waiting-bots")
            .len(),
        1
    );
    assert!(
        scene.said("PR #1: could not set the state label"),
        "{:?}",
        scene.log
    );
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("success".to_owned(), "ready-to-merge".to_owned())
    );
}

#[test]
fn a_reviewer_request_refused_with_422_does_not_stop_the_status() {
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    fake.refuse(
        Method::Post,
        "pulls/1/requested_reviewers",
        Refusal::Http(
            422,
            "Reviews may only be requested from collaborators".into(),
        ),
    );
    let mut scene = Scene::new(fake);
    let run = scene.sync_pr(&policy, 1);
    assert!(run.failure().is_none());
    assert_eq!(
        scene
            .wrote(Method::Post, "pulls/1/requested_reviewers")
            .len(),
        1
    );
    assert!(
        scene.said("PR #1: could not request owner; the description still names them"),
        "{:?}",
        scene.log
    );
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &("pending".to_owned(), "ready-for-human".to_owned())
    );
}

#[test]
fn a_status_that_is_not_acknowledged_fails_the_pull_request_and_then_the_run() {
    // A status GitHub does not answer for is not known to be there: the
    // pull request is marked failed, the failure status is tried and not
    // acknowledged either, every other pull request still has its turn,
    // and the run fails at the end.
    let (policy, mut fake) = fixture();
    let mut other = Pr::new(2, "reviewer", &"b".repeat(40));
    other.created_at = "2026-09-10T01:00:00Z".into();
    fake.add(other);
    fake.refuse(Method::Post, &format!("statuses/{HEAD}"), Refusal::Empty);
    let mut scene = Scene::new(fake);
    let run = scene
        .run(&policy, Command::Sync, Pick::All, None)
        .expect("the run ends");
    let failure = run.failure().expect("the run fails");
    assert_eq!(failure.to_string(), "reconciliation failed for #1");
    assert!(
        scene.said("PR #1: Commit status was not acknowledged"),
        "{:?}",
        scene.log
    );
    assert!(scene.said("PR #1: unable to publish the failure either"));
    assert!(
        scene
            .wrote(Method::Post, &format!("statuses/{}", "b".repeat(40)))
            .len()
            >= 2,
        "the next pull request still had its turn"
    );
    assert!(run.report.is_none(), "a failed run reports nothing");
}

#[test]
fn a_record_comment_github_refuses_fails_the_pull_request() {
    // The record holds the author's place in the queue: a pull request whose
    // record cannot be written is marked failed, not left to look reconciled.
    let (policy, mut fake) = bartek();
    fake.refuse(
        Method::Post,
        "issues/1/comments",
        Refusal::Http(403, "Resource not accessible by integration".into()),
    );
    let mut scene = Scene::new(fake);
    let run = scene
        .run(&policy, Command::Sync, Pick::Pr(1), None)
        .expect("the run ends");
    assert_eq!(
        run.failure().map(|f| f.to_string()).as_deref(),
        Some("reconciliation failed for #1")
    );
    assert_eq!(
        scene.statuses().last().expect("a status"),
        &(
            "error".to_owned(),
            "Policy reconciliation failed; inspect workflow log".to_owned()
        )
    );
}
