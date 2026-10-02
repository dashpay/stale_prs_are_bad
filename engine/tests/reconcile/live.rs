//! The live comparison held to its rule for a pull request nothing has
//! changed since the engine last wrote, over the stateful fake: settled by
//! earlier runs, a one-author run made live wants to write nothing, and is
//! counted as held to it.
//!
//! "Python's" run here is the port's own dry run of the same command over
//! the same state: its snapshot, verdict and admission stand where Python's
//! would, and it recorded no call, so it wrote nothing.

use crate::fake::*;
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
    let run = scene
        .run_with(
            policy,
            Command::Sync,
            Pick::Pr(1),
            None,
            false,
            page.clone(),
        )
        .expect("the dry run decides");
    let now = run.generated_at.clone();
    let candidates = PyValue::List(PyList::from(run.candidates.clone()));
    let admitted = admit(policy, &candidates, &s(&now))
        .expect("admission")
        .get(&PyInt::from(1))
        .cloned()
        .unwrap_or(PyValue::None);
    let row = run.verdicts[0].clone();
    let mut result = row.clone();
    if let PyValue::Dict(fields) = &mut result {
        fields.shift_remove("repository");
    }
    let evaluation = dict(vec![
        ("pr", run.snapshots[0].clone()),
        ("admitted_at", admitted),
        ("now", s(&now)),
        ("telemetry_states", PyValue::None),
        ("result", result),
    ]);
    let meta = dict(vec![
        ("format", PyValue::Int(PyInt::from(2))),
        ("repository", s(REPO)),
        (
            "argv",
            PyValue::List(PyList::from(
                ["sync", "--repo", REPO, "--pr", "1"]
                    .iter()
                    .map(|word| s(word))
                    .collect::<Vec<_>>(),
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
        ("evaluations.jsonl", format!("{}\n", dump(&evaluation))),
        (
            "verdicts.json",
            dump(&PyValue::List(PyList::from(vec![row]))),
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
    assert_eq!(live.settled, Some(true), "{:?}", live.comparison);
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
