//! The live comparison held to its rule for a pull request nothing has
//! changed since the engine last wrote, over the stateful fake: settled by
//! earlier runs, a one-author run made live, or a run of every pull
//! request, wants to write nothing, and each pull request is counted as
//! held to it.
//!
//! "Python's" run here is the port's own dry run of the same command over
//! the same state: its snapshots, verdicts and admissions stand where
//! Python's would, and it recorded no call, so it wrote nothing.

use crate::fake::*;
use crate::properties::mixed;
use crate::scene::*;
use crate::support::*;
use pr_hygiene_engine::conformance::live::{live_run, Live};
use pr_hygiene_engine::conformance::{Layer, Outcome, OwnWords, Recording};
use pr_hygiene_engine::policy::admit;
use pr_hygiene_engine::pycompat::{PyDict, PyList};
use pr_hygiene_engine::reconcile::Command;

fn own() -> OwnWords {
    let source = |name: &str| {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../pr_review")
                .join(name),
        )
        .expect("the engine's source")
    };
    OwnWords::from_sources(&source("conformance.py"), &source("policy.py")).expect("own words")
}

struct Stopped(PyDateTime);

impl Clock for Stopped {
    fn now(&mut self, _site: ClockSite) -> PyDateTime {
        self.0
    }
}

fn at(instant: &str) -> Stopped {
    Stopped(PyDateTime::fromisoformat(&instant.replace('Z', "+00:00")).expect("an instant"))
}

/// Syncs of pull request 1 until one writes nothing, as the engine's
/// hourly runs settle it.
fn settle(scene: &mut Scene, policy: &PyValue) {
    for _ in 0..5 {
        scene.fake.forget_calls();
        let before = scene.fake.written.len();
        scene.sync_pr(policy, 1);
        if scene.fake.written.len() == before {
            return;
        }
    }
    panic!("never settled: {:?}", scene.writes());
}

/// A dict from its entries.
fn dict(entries: Vec<(&str, PyValue)>) -> PyValue {
    let mut fields = PyDict::new();
    for (key, value) in entries {
        fields.insert(key.into(), value);
    }
    PyValue::Dict(fields)
}

/// The recording of a dry `sync --pr 1` over the fake as it stands, with
/// the status page `page`.
fn recorded(scene: &mut Scene, policy: &PyValue, page: &PyValue) -> Recording {
    recorded_of(scene, policy, page, Pick::Pr(1))
}

/// The recording of a dry sync of `pick` — `Pick::Pr(n)` or `Pick::All` —
/// over the fake as it stands, with the status page `page`.
fn recorded_of(scene: &mut Scene, policy: &PyValue, page: &PyValue, pick: Pick) -> Recording {
    let argv: Vec<String> = match &pick {
        Pick::Pr(n) => vec![
            "sync".into(),
            "--repo".into(),
            REPO.into(),
            "--pr".into(),
            n.to_string(),
        ],
        Pick::All => vec!["sync".into(), "--repo".into(), REPO.into()],
        Pick::Batch(_) => panic!("a batch is not read live"),
    };
    let run = scene
        .run_with(policy, Command::Sync, pick, None, false, page.clone())
        .expect("the dry run decides");
    let now = run.generated_at.clone();
    let candidates = PyValue::List(PyList::from(run.candidates.clone()));
    let slots = admit(policy, &candidates, &s(&now)).expect("admission");
    let mut evaluations = String::new();
    for (snapshot, row) in run.snapshots.iter().zip(&run.verdicts) {
        let PyValue::Int(number) = field(snapshot, "number") else {
            panic!("a numbered pull request")
        };
        let admitted = slots.get(number).cloned().unwrap_or(PyValue::None);
        let mut result = row.clone();
        if let PyValue::Dict(fields) = &mut result {
            fields.shift_remove("repository");
        }
        let evaluation = dict(vec![
            ("pr", snapshot.clone()),
            ("admitted_at", admitted),
            ("now", s(&now)),
            ("telemetry_states", PyValue::None),
            ("result", result),
        ]);
        evaluations.push_str(&format!("{}\n", dump(&evaluation)));
    }
    let meta = dict(vec![
        ("format", PyValue::Int(PyInt::from(2))),
        ("repository", s(REPO)),
        (
            "argv",
            PyValue::List(PyList::from(
                argv.iter().map(|word| s(word)).collect::<Vec<_>>(),
            )),
        ),
        ("clock", s(&now)),
        ("clock_reads", PyValue::List(PyList::from(Vec::new()))),
        ("policy", policy.clone()),
        ("telemetry", page.clone()),
        (
            "outcome",
            dict(vec![("returned", PyValue::Int(PyInt::from(0)))]),
        ),
    ]);
    let files = [
        ("recording.json", dump(&meta)),
        ("calls.jsonl", String::new()),
        ("evaluations.jsonl", evaluations),
        (
            "verdicts.json",
            dump(&PyValue::List(PyList::from(run.verdicts.clone()))),
        ),
    ];
    Recording::load_with(|name| -> Result<String, String> {
        files
            .iter()
            .find(|(file, _)| *file == name)
            .map(|(_, text)| text.clone())
            .ok_or_else(|| name.to_owned())
    })
    .expect("a recording")
}

fn outcome(live: &Live, layer: Layer) -> &Outcome {
    &live
        .comparison
        .checks
        .iter()
        .find(|check| check.layer == layer)
        .unwrap_or_else(|| panic!("no {} check: {:?}", layer.as_str(), live.comparison))
        .outcome
}

#[test]
fn a_settled_pull_request_read_live_wants_to_write_nothing() {
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    let recording = recorded(&mut scene, &policy, &PyValue::None);
    let written = scene.fake.written.len();
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(live.unsettled, 0, "{:?}", live.comparison);
    assert!(
        matches!(outcome(&live, Layer::LiveWrites), Outcome::Matched),
        "{:?}",
        live.comparison
    );
    assert!(matches!(
        outcome(&live, Layer::LiveSnapshot),
        Outcome::Matched
    ));
    assert!(matches!(
        outcome(&live, Layer::LiveVerdict),
        Outcome::Matched
    ));
    assert!(live.comparison.is_clean());
    assert_eq!(scene.fake.written.len(), written, "nothing written");
}

#[test]
fn a_settled_pull_request_whose_evidence_moved_on_is_still_held_to_writing_nothing() {
    // The engine rewrites its record only when the state, head, admission or
    // ready time would change; the evidence print in it is left as it was, so
    // the record does not churn on every comment. A conversation that goes on
    // without changing the verdict therefore leaves the print behind, and
    // Python's own dry run writes nothing. Requiring the print to be current
    // left most live one-author runs unjudged; writing nothing is the test.
    let (policy, mut fake) = fixture();
    fake.pr(1).author = "reviewer".into();
    fake.pr(1).comments[0].author = "reviewer".into();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    let mut scene = Scene::new(fake);
    settle(&mut scene, &policy);
    scene.fake.pr(1).comments.push(Comment::new(
        9001,
        "reviewer",
        "Rebased on the latest base; nothing else changed.",
        "2026-09-11T09:30:00Z",
    ));
    let recording = recorded(&mut scene, &policy, &PyValue::None);
    let written = scene.fake.written.len();
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(live.unsettled, 0, "{:?}", live.comparison);
    assert!(
        matches!(outcome(&live, Layer::LiveWrites), Outcome::Matched),
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean());
    assert_eq!(scene.fake.written.len(), written, "nothing written");
}

#[test]
fn every_pull_request_of_a_settled_repository_read_live_is_held_to_no_write_and_gets_none() {
    // A repository settled by full passes: admitted, recorded, described,
    // labelled and asked. A run of every pull request, made live, decides
    // each as Python's run did and wants to write to none of them, nor to
    // the one rebased off the policy, whose marks are already gone.
    let (policy, fake) = mixed();
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    assert!(scene.writes().is_empty(), "settled: {:?}", scene.writes());
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    assert_eq!(recording.verdicts.len(), 4, "every governed pull request");
    let written = scene.fake.written.len();
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let held: Vec<usize> = live
        .comparison
        .checks
        .iter()
        .filter(|check| check.layer == Layer::LiveWrites)
        .map(|check| {
            assert!(
                matches!(check.outcome, Outcome::Matched),
                "{:?}",
                live.comparison
            );
            check.index
        })
        .collect();
    assert_eq!(held, [0, 1, 2, 3], "each pull request held, and none wrote");
    assert_eq!(live.unsettled, 0);
    assert_eq!(live.comparison.matched(Layer::LiveSnapshot), (4, 4));
    assert_eq!(live.comparison.matched(Layer::LiveVerdict), (4, 4));
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
    assert_eq!(scene.fake.written.len(), written, "nothing written");
}

#[test]
fn a_pull_request_of_a_settled_repository_that_wants_a_write_is_named_alone() {
    // The same repository, one pull request's state label taken off by
    // hand after the last pass: Python's run would put it back, so here
    // "Python's" dry run, made before the label went, wrote nothing, and
    // the live run, made after, wants to write to that one pull request
    // and to no other.
    let (policy, fake) = mixed();
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    let labels = &mut scene.fake.pr(2).labels;
    assert!(!labels.is_empty(), "2 wears its state label: {labels:?}");
    labels.clear();
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let writes: Vec<(usize, String)> = live
        .comparison
        .checks
        .iter()
        .filter(|check| check.layer == Layer::LiveWrites)
        .map(|check| {
            let said = match &check.outcome {
                Outcome::Matched => "matched".to_owned(),
                Outcome::Differs(found) => found[0].path.clone(),
                other => format!("{other:?}"),
            };
            (check.index, said)
        })
        .collect();
    assert_eq!(
        writes,
        [
            (0, "matched".to_owned()),
            (
                1,
                r#"POST repos/*/*/statuses/* pending "Evaluating current review policy""#
                    .to_owned(),
            ),
            (2, "matched".to_owned()),
            (3, "matched".to_owned()),
        ],
        "{:?}",
        live.comparison
    );
}

/// Each pull request's no-write check: matched, moved, or the first write
/// it wanted.
fn writes_of(live: &Live) -> Vec<(usize, String)> {
    live.comparison
        .checks
        .iter()
        .filter(|check| check.layer == Layer::LiveWrites)
        .map(|check| {
            let said = match &check.outcome {
                Outcome::Matched => "matched".to_owned(),
                Outcome::Moved => "moved".to_owned(),
                Outcome::Differs(found) => found[0].path.clone(),
                Outcome::NotCompared { why } => format!("not compared: {why}"),
                other => format!("{other:?}"),
            };
            (check.index, said)
        })
        .collect()
}

/// The settled repository of `mixed()`, Python's dry run of every pull
/// request over it, and `during` registered after that run: what happens
/// on GitHub while the live run reads.
fn settled_then(during: impl FnMut(&mut State, &Call) + 'static) -> (Scene, Recording, PyValue) {
    let (policy, fake) = mixed();
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    scene.fake.on_call(during);
    (scene, recording, policy)
}

/// Whether `call` reads `route` of the repository.
fn reads(call: &Call, route: &str) -> bool {
    matches!(call, Call::Rest { method: Method::Get, path, .. }
        if path.split('?').next() == Some(&format!("repos/{REPO}/{route}")[..]))
}

#[test]
fn a_review_landing_between_a_live_runs_read_and_its_recheck_is_a_move() {
    // Pull request 2 is ready, so the run reads it again before it would
    // post its status. Between its first read and that one, someone leaves
    // a review: the evidence changed under the run, which says so in its
    // status, as it must. That is GitHub moving, not the port.
    let mut asked = 0;
    let (mut scene, recording, _) = settled_then(move |state, call| {
        if reads(call, "pulls/2/reviews") {
            asked += 1;
            if asked == 2 {
                let head = state.pr(2).head.clone();
                state.pr(2).reviews.push(review(
                    98,
                    "passerby",
                    "COMMENTED",
                    &head,
                    LATER,
                    "One more thought.",
                ));
            }
        }
    });
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        writes_of(&live),
        [
            (0, "matched".to_owned()),
            (1, "moved".to_owned()),
            (2, "matched".to_owned()),
            (3, "matched".to_owned()),
        ],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
}

/// The fake, with one route answered otherwise than its state says: two of
/// GitHub's routes to the same thing disagreeing, with nothing having moved.
struct Disagreeing<'a> {
    fake: &'a mut Fake,
    route: &'static str,
    page: &'static str,
}

impl Transport for Disagreeing<'_> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if reads(call, self.route) {
            return Ok(Reply::Pages(vec![self.page.to_owned()]));
        }
        self.fake.call(call)
    }
}

#[test]
fn the_same_status_where_no_read_answered_otherwise_on_asking_again_still_fails() {
    // The run's re-check reads pull request 2 by another route than its
    // first read did: its timeline, where the first read took the batched
    // history. That route says it went to draft once; the history says
    // not. The re-check posts the same status, but nothing it read was
    // answered otherwise when asked again: not GitHub moving under the run,
    // and held to Python's.
    let (mut scene, recording, _) = settled_then(|_, _| {});
    let live = live_run(
        &recording,
        Disagreeing {
            fake: &mut scene.fake,
            route: "issues/2/timeline",
            page: r#"[{"event": "convert_to_draft", "created_at": "2026-09-11T08:00:00Z"}]"#,
        },
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        writes_of(&live),
        [
            (0, "matched".to_owned()),
            (
                1,
                r#"POST repos/*/*/statuses/* pending "Review evidence changed; reconciliation required""#
                    .to_owned()
            ),
            (2, "matched".to_owned()),
            (3, "matched".to_owned()),
        ],
        "{:?}",
        live.comparison
    );
}

/// A patch quoting an escape character in source code: the text `\u001b`,
/// which GitHub sends as it is and `gh api` prints as `\^[`.
const QUOTING: &str = "@@ -1 +1 @@\n-a\n+let esc = \"\\u001b\";";

/// A file whose patch is `patch`, with its blob, so that its pull
/// request's diff has a print.
fn file_with(patch: &str) -> Value {
    json!({"filename": "packages/drive/a.rs", "status": "modified",
           "sha": "1".repeat(40), "patch": patch})
}

/// The repository of `mixed()`, pull request 2's one file patched with
/// `patch`, settled by full passes and recorded by an engine that reads
/// GitHub through `gh api`, as the Python engine does: its records hold
/// the print of the diff as gh printed it. The fake then answers as GitHub
/// sends, as the live run reads it.
fn settled_through_gh(patch: &str) -> (Scene, Recording) {
    let (policy, mut fake) = mixed();
    fake.pr(2).files = vec![file_with(patch)];
    fake.prints_as_gh = true;
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    assert!(scene.writes().is_empty(), "settled: {:?}", scene.writes());
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    scene.fake.prints_as_gh = false;
    (scene, recording)
}

/// Each check of pull request `index`, by layer, as one line.
fn described(live: &Live, index: usize) -> Vec<String> {
    live.comparison
        .checks
        .iter()
        .filter(|check| check.index == index)
        .map(|check| {
            let paths = |found: &[pr_hygiene_engine::conformance::Difference]| {
                found
                    .iter()
                    .map(|d| d.path.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let what = match &check.outcome {
                Outcome::Matched => "matched".to_owned(),
                Outcome::Moved => "moved".to_owned(),
                Outcome::Explained { differences, by } => {
                    format!("explained by {by}: {}", paths(differences))
                }
                Outcome::Differs(found) => format!("differs: {}", paths(found)),
                other => format!("{other:?}"),
            };
            format!("{}: {what}", check.layer.as_str())
        })
        .collect()
}

#[test]
fn a_patch_gh_prints_otherwise_is_gh_s_down_to_its_digest_and_the_write_that_follows() {
    // Python's engine hashed the patch gh printed; the live run hashes the
    // one GitHub sent. The digests differ, and so do the diff's print and
    // the record the engine keeps beside it: the live run would rewrite
    // it. Read again as gh prints GitHub's answers, the live run reaches
    // Python's snapshot and wants to write nothing: all of it is gh's.
    let (mut scene, recording) = settled_through_gh(QUOTING);
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        described(&live, 1),
        [
            "live snapshot: explained by gh-printed control characters: pr.files[0].shape",
            "live verdict: matched",
            r#"live writes: explained by gh-printed control characters: POST repos/*/*/statuses/* pending "Evaluating current review policy""#,
        ],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
}

#[test]
fn a_patch_that_differs_otherwise_still_fails_though_gh_prints_it_otherwise_too() {
    // The same repository, the patch changed since Python read it with
    // nothing to say GitHub moved: rendered as gh prints it, it is still
    // not what Python read, so the difference is the port's.
    let (mut scene, recording) = settled_through_gh(QUOTING);
    scene.fake.pr(2).files = vec![file_with(&format!("{QUOTING}\n+let bell = 7;"))];
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        described(&live, 1),
        [
            "live snapshot: differs: pr.files[0].shape",
            "live verdict: matched",
            r#"live writes: differs: POST repos/*/*/statuses/* pending "Evaluating current review policy""#,
        ],
        "{:?}",
        live.comparison
    );
    assert!(!live.comparison.is_clean());
}

/// A comment by someone who has not spoken on the pull request before.
fn passerby_comment(id: i64, at: &str) -> Comment {
    Comment::new(id, "passerby", "Thanks for this.", at)
}

/// The settled repository of `mixed()` with a passerby's comment on pull
/// request 2 when Python's run read it, recorded.
fn settled_with_comment() -> (Scene, Recording) {
    let (policy, mut fake) = mixed();
    fake.pr(2)
        .comments
        .push(passerby_comment(900, "2026-09-11T11:30:00Z"));
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    (scene, recording)
}

/// The fake, but its GraphQL answers leave comment `id` out: a read that
/// misses a comment GitHub still has.
struct Hiding<'a> {
    fake: &'a mut Fake,
    id: i64,
}

impl Transport for Hiding<'_> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        let answer = self.fake.call(call)?;
        let (Call::Graphql { .. }, Reply::Text(text)) = (call, &answer) else {
            return Ok(answer);
        };
        let Ok(mut value) = serde_json::from_str::<Value>(text) else {
            return Ok(answer);
        };
        if let Some(pulls) = value["data"]["repository"].as_object_mut() {
            for pull in pulls.values_mut() {
                let comments = &mut pull["comments"];
                if let Some(nodes) = comments["nodes"].as_array_mut() {
                    let before = nodes.len();
                    nodes.retain(|node| node["databaseId"].as_i64() != Some(self.id));
                    let gone = (before - nodes.len()) as i64;
                    if let Some(total) = comments["totalCount"].as_i64() {
                        comments["totalCount"] = json!(total - gone);
                    }
                }
            }
        }
        Ok(Reply::Text(value.to_string()))
    }
}

/// Whether the live run asked GitHub for comment `id` by itself.
fn asked_for_comment(scene: &Scene, id: i64) -> bool {
    scene
        .fake
        .calls
        .iter()
        .any(|call| reads(call, &format!("issues/comments/{id}")))
}

#[test]
fn a_comment_added_between_the_reads_moved_its_pull_request() {
    // Someone comments on pull request 2 after Python's run read it: GitHub
    // keeps the pull request's update time where it was, and the live read
    // has one comment more, created after Python's run began.
    let (policy, fake) = mixed();
    let mut scene = Scene::new(fake);
    for _ in 0..3 {
        scene.fake.forget_calls();
        scene.sync_all(&policy);
    }
    let recording = recorded_of(&mut scene, &policy, &PyValue::None, Pick::All);
    scene
        .fake
        .pr(2)
        .comments
        .push(passerby_comment(901, "2026-09-11T14:05:00Z"));
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let checks = described(&live, 1);
    assert_eq!(
        &checks[..2],
        // The verdict, compared first, is Python's: a comment that says
        // nothing the engine reads leaves it as it was.
        ["live snapshot: moved", "live verdict: matched"],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
}

#[test]
fn a_comment_github_no_longer_has_was_deleted_between_the_reads() {
    // Python's read had the passerby's comment; the live read has not, and
    // GitHub, asked for it by itself, answers that it does not have it.
    let (mut scene, recording) = settled_with_comment();
    scene
        .fake
        .pr(2)
        .comments
        .retain(|comment| comment.id != 900);
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let checks = described(&live, 1);
    assert_eq!(
        &checks[..2],
        // The verdict, compared first, is Python's: a comment that says
        // nothing the engine reads leaves it as it was.
        ["live snapshot: moved", "live verdict: matched"],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
    assert!(
        asked_for_comment(&scene, 900),
        "the deletion is proven, not assumed"
    );
}

#[test]
fn a_comment_the_live_read_lacks_that_github_still_has_is_the_ports_difference() {
    // The live read lacks the passerby's comment, but GitHub, asked for it
    // by itself, still has it: nothing was deleted; the port dropped it.
    let (mut scene, recording) = settled_with_comment();
    let live = live_run(
        &recording,
        Hiding {
            fake: &mut scene.fake,
            id: 900,
        },
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let checks = described(&live, 1);
    assert!(
        checks[0].starts_with("live snapshot: differs: ") && checks[0].contains("pr.comments"),
        "{checks:?}"
    );
    assert!(!live.comparison.is_clean());
    assert!(asked_for_comment(&scene, 900), "asked, and answered");
}

/// The settled repository of `mixed()`, recorded, with GitHub then
/// answering pull request 2's timeline, which only the re-check before its
/// status reads (the first read takes its history in one query), with
/// `status`: a read Python's run never made.
fn timeline_failing(status: u16) -> (Scene, Recording) {
    let (mut scene, recording, _) = settled_then(|_, _| {});
    scene.fake.refuse(
        Method::Get,
        "issues/2/timeline",
        Refusal::Http(status, "GitHub said no".into()),
    );
    (scene, recording)
}

#[test]
fn a_read_github_fails_twice_with_502_before_the_failure_status_is_not_compared() {
    // The re-check's read of pull request 2's timeline fails with 502, is
    // asked again, and fails again: the run marks 2's reconciliation
    // failed, its first write. GitHub failed the read; nothing about the
    // port is shown, so 2's writes are not compared, and that is no failure.
    let (mut scene, recording) = timeline_failing(502);
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        writes_of(&live),
        [
            (0, "matched".to_owned()),
            (
                1,
                "not compared: live read failed (HTTP 502 after retry on GET repos/*/*/issues/*/timeline)"
                    .to_owned()
            ),
            (2, "matched".to_owned()),
            (3, "matched".to_owned()),
        ],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
}

#[test]
fn a_read_answered_404_before_the_failure_status_still_fails_named_by_its_class() {
    // The same read answered 404: the port asked for something GitHub does
    // not have. The failure status is the port's, named by what caused it.
    let (mut scene, recording) = timeline_failing(404);
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        writes_of(&live)[1],
        (
            1,
            r#"POST repos/*/*/statuses/* error "Policy reconciliation failed; inspect workflow log", after HTTP 404 on GET repos/*/*/issues/*/timeline"#
                .to_owned()
        ),
        "{:?}",
        live.comparison
    );
    assert!(!live.comparison.is_clean());
}

/// The fake, behind a layer that refuses every read of `route` as the
/// read-only layer refuses a read it does not let through.
struct ReadOnlyRefusing<'a> {
    fake: &'a mut Fake,
    route: &'static str,
}

impl Transport for ReadOnlyRefusing<'_> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if reads(call, self.route) {
            return Err(TransportError::Refused(format!(
                "not one of the engine's reads: {call}"
            )));
        }
        self.fake.call(call)
    }
}

#[test]
fn a_read_the_read_only_layer_refuses_still_fails_named_by_its_route() {
    let (mut scene, recording) = timeline_failing(200);
    let live = live_run(
        &recording,
        ReadOnlyRefusing {
            fake: &mut scene.fake,
            route: "issues/2/timeline",
        },
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let failing: Vec<String> = live
        .comparison
        .checks
        .iter()
        .filter(|check| !check.passed())
        .map(|check| match &check.outcome {
            Outcome::Differs(found) => format!("{}: {}", found[0].path, found[0].kind),
            other => format!("{other:?}"),
        })
        .collect();
    assert_eq!(
        failing,
        ["GET repos/*/*/issues/*/timeline: read refused by the read-only layer"],
        "{:?}",
        live.comparison
    );
}

/// The settled repository of `mixed()`, recorded, with pull request 3 —
/// whose author also opened 2 — rebased onto another base commit while the
/// live run reads: the open listing answers it otherwise the second time.
fn sibling_moving() -> (Scene, Recording) {
    let mut listed = 0;
    let (scene, recording, _) = settled_then(move |state, call| {
        if reads(call, "pulls") {
            listed += 1;
            if listed == 2 {
                state.pr(3).base_sha = "f".repeat(40);
            }
        }
    });
    (scene, recording)
}

#[test]
fn a_verdict_equal_to_pythons_is_matched_though_a_sibling_moved() {
    // 3 moved during the read; 2, by the same author, did not, and the live
    // run decided it as Python did: that is evidence, and counts as matched.
    let (mut scene, recording) = sibling_moving();
    let live = live_run(
        &recording,
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        described(&live, 1),
        [
            "live snapshot: matched",
            "live verdict: matched",
            "live writes: matched"
        ],
        "{:?}",
        live.comparison
    );
    // 3's own first read is Python's: matched too, though its later reads
    // show it moved.
    assert_eq!(
        &described(&live, 2)[..2],
        ["live snapshot: matched", "live verdict: matched"],
        "{:?}",
        live.comparison
    );
    assert!(live.comparison.is_clean(), "{:?}", live.comparison);
}

#[test]
fn a_verdict_that_differs_where_a_sibling_moved_is_still_moved() {
    // Python's run decided 2 otherwise than the live run does, with 2's
    // snapshot the same on both sides; its author's other pull request
    // moved, which a verdict follows from, so the difference is put down to
    // the move, as before. Where nothing moved, the same difference fails.
    let with_other_state = |mut recording: Recording| {
        if let PyValue::Dict(row) = &mut recording.verdicts[1] {
            row.insert("state".into(), s("waiting-bots"));
        }
        recording
    };
    let (mut scene, recording) = sibling_moving();
    let live = live_run(
        &with_other_state(recording),
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    assert_eq!(
        &described(&live, 1)[..2],
        ["live snapshot: matched", "live verdict: moved"],
        "{:?}",
        live.comparison
    );
    let (mut scene, recording, _) = settled_then(|_, _| {});
    let live = live_run(
        &with_other_state(recording),
        &mut scene.fake,
        &mut at(LATER),
        &mut || PyValue::None,
        &own(),
    );
    let verdict = live
        .comparison
        .checks
        .iter()
        .find(|check| check.layer == Layer::LiveVerdict && check.index == 1)
        .expect("2's verdict");
    assert!(!verdict.passed(), "{:?}", live.comparison);
}
