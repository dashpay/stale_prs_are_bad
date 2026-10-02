//! A run of the engine over the fake: each run a fresh reader, as each of
//! Python's runs is a fresh process, over the state the last one left.

use crate::fake::*;
use crate::support::*;
use pr_hygiene_engine::evidence::records::state_comment_body;
use pr_hygiene_engine::policy::evaluate;
use pr_hygiene_engine::reconcile::{Command, Reconciler, Run, RunOptions, Selection};

pub const HEAD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub const NOW: &str = "2026-09-11T12:00:00Z";
pub const LATER: &str = "2026-09-11T14:00:00Z";

/// A value written as JSON.
pub fn py(value: Value) -> PyValue {
    py_loads(&value.to_string()).expect("serde_json writes JSON")
}

pub fn s(text: &str) -> PyValue {
    PyValue::Str(text.into())
}

/// `fixture()`'s policy: one area, `drive`, owned by `owner` and reviewed
/// by `reviewer`.
pub fn fixture_policy() -> PyValue {
    py(json!({
        "version": 1, "repository": REPO, "max_active_prs": 5,
        "target_branches": ["v4.2-dev"],
        "fallback": {"owners": ["fallback"], "reviewers": []},
        "areas": [{"id": "drive", "paths": ["packages/drive/"], "owners": ["owner"],
                   "reviewers": ["reviewer"]}],
    }))
}

/// `platform_policy()`: dashpay/platform's policy, frozen for the tests.
pub fn platform_policy() -> PyValue {
    read_json(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../pr_review/tests/fixtures/platform.json"),
    )
}

/// `bartek()`: dashpay/platform#4818 — an approval in hand that covered
/// nothing it needed. The author skipped the bots and has not attested;
/// the head was first seen at 09:00.
pub fn bartek() -> (PyValue, Fake) {
    let mut fake = Fake::new(LATER);
    let mut pr = Pr::new(1, "llbartekll", HEAD);
    pr.files = [
        "packages/swift-sdk/Sources/a.swift",
        ".editorconfig",
        ".github/workflows/swift-sdk-build.yml",
        ".github/workflows/tests.yml",
        "AGENTS.md",
    ]
    .iter()
    .map(|path| json!({"filename": path, "status": "modified"}))
    .collect();
    pr.comments = vec![Comment::new(
        1,
        "llbartekll",
        "/skip-bots",
        "2026-09-11T10:00:00Z",
    )];
    pr.reviews = vec![review(
        5,
        "romchornyi",
        "APPROVED",
        HEAD,
        "2026-09-11T10:30:00Z",
        "",
    )];
    fake.add(pr);
    fake.state.collaborators = [
        ("llbartekll", "write"),
        ("romchornyi", "write"),
        ("QuantumExplorer", "admin"),
        ("shumkov", "admin"),
        ("ktechmidas", "admin"),
    ]
    .iter()
    .map(|(login, level)| ((*login).into(), (*level).into()))
    .collect();
    fake.state
        .engine_status(HEAD, "pending", "waiting-bots", "2026-09-11T09:00:00Z");
    (platform_policy(), fake)
}

/// `fixture()`'s pull request, by `owner`, which both bots reported on and
/// its author attested: ready to merge once admitted.
pub fn fixture() -> (PyValue, Fake) {
    let mut fake = Fake::new(LATER);
    fake.add(Pr::new(1, "owner", HEAD));
    (fixture_policy(), fake)
}

/// The engine's record comment, as `GitHub.state_comment_body` writes it.
pub fn record_comment(state: &PyValue, words: &str, diff: Option<&PyValue>) -> String {
    state_comment_body(state, words, diff).expect("a record the engine writes")
}

/// A run every pull request of which was reconciled, which a scenario
/// may read or not.
pub struct Ran(pub Run);

impl std::ops::Deref for Ran {
    type Target = Run;
    fn deref(&self) -> &Run {
        &self.0
    }
}

/// Which pull requests a run reconciles.
pub enum Pick {
    All,
    Pr(i64),
    /// The batch a sweep chooses: these pull requests, with their authors'
    /// others as candidates.
    Batch(Vec<i64>),
}

/// The fake, and runs of the engine over it at `now`.
pub struct Scene {
    pub fake: Fake,
    pub now: String,
    pub log: Vec<String>,
}

impl Scene {
    pub fn new(fake: Fake) -> Self {
        let now = fake.state.now.clone();
        Scene {
            fake,
            now,
            log: Vec::new(),
        }
    }

    /// A fresh reader over the fake, with its own caches, and the engine
    /// over it.
    pub fn with<R>(&mut self, f: impl FnOnce(&mut Reconciler<'_, &mut Fake>) -> R) -> R {
        let mut api = GitHub::new(REPO, Client::with_sleep(&mut self.fake, NoSleep))
            .expect("a valid repository");
        let mut clock = LoggingClock::new(&self.now, Default::default());
        let mut reconciler = Reconciler::new(&mut api, &mut clock);
        let out = f(&mut reconciler);
        let said = reconciler.take_log();
        self.log.extend(said);
        out
    }

    /// One run, as `main.run` makes it: a sync applies.
    pub fn run(
        &mut self,
        policy: &PyValue,
        command: Command,
        pick: Pick,
        user: Option<&str>,
    ) -> Result<Run, ReadError> {
        self.run_with(
            policy,
            command,
            pick,
            user,
            command == Command::Sync,
            PyValue::None,
        )
    }

    /// One run, applying or not, over the status page `page`.
    pub fn run_with(
        &mut self,
        policy: &PyValue,
        command: Command,
        pick: Pick,
        user: Option<&str>,
        apply: bool,
        page: PyValue,
    ) -> Result<Run, ReadError> {
        let mut telemetry = move || page.clone();
        let mut choose = |prs: &[PyValue]| -> Vec<PyValue> {
            let Pick::Batch(numbers) = &pick else {
                return Vec::new();
            };
            prs.iter()
                .filter(|pr| {
                    numbers.iter().any(
                        |n| matches!(field(pr, "number"), PyValue::Int(m) if *m == PyInt::from(*n)),
                    )
                })
                .cloned()
                .collect()
        };
        let selection = match &pick {
            Pick::All => Selection::All,
            Pick::Pr(n) => Selection::Pr(PyInt::from(*n)),
            Pick::Batch(_) => Selection::Batch(&mut choose),
        };
        self.with(|engine| {
            engine.run(
                policy,
                RunOptions {
                    command,
                    selection,
                    apply,
                    user: user.map(str::to_owned),
                    nudges: 1,
                    telemetry: &mut telemetry,
                },
            )
        })
    }

    /// `sync --pr n --apply`: the run an event on one pull request makes.
    pub fn sync_pr(&mut self, policy: &PyValue, n: i64) -> Ran {
        self.reconciled(Pick::Pr(n), policy)
    }

    /// `sync --apply`: a full pass.
    pub fn sync_all(&mut self, policy: &PyValue) -> Ran {
        self.reconciled(Pick::All, policy)
    }

    /// A sync every pull request of which was reconciled.
    fn reconciled(&mut self, pick: Pick, policy: &PyValue) -> Ran {
        let run = self
            .run(policy, Command::Sync, pick, None)
            .expect("the run completes");
        if let Some(failure) = run.failure() {
            panic!("{failure}: {:?}", self.log);
        }
        Ran(run)
    }

    /// The pull request as the engine reads it now.
    pub fn snapshot(&mut self, policy: &PyValue, n: i64) -> PyValue {
        self.with(|engine| engine.api().snapshot(&PyInt::from(n), policy, None))
            .expect("a snapshot")
    }

    /// The verdict on the pull request as it stands, under `admitted_at`.
    pub fn verdict(&mut self, policy: &PyValue, n: i64, admitted_at: &PyValue) -> PyValue {
        let pr = self.snapshot(policy, n);
        let now = s(&self.now);
        PyValue::Dict(evaluate(policy, &pr, admitted_at, &now, &PyValue::None).expect("a verdict"))
    }

    /// The writes made since the calls were last forgotten.
    pub fn writes(&self) -> Vec<String> {
        self.fake.writes()
    }

    /// The descriptions of the statuses posted, in order.
    pub fn statuses(&self) -> Vec<(String, String)> {
        self.fake
            .written
            .iter()
            .filter(|w| w.route.starts_with("statuses/"))
            .map(|w| {
                (
                    text(w.field("state")).to_owned(),
                    text(w.field("description")).to_owned(),
                )
            })
            .collect()
    }

    /// The writes to routes starting with `route`.
    pub fn wrote(&self, method: Method, route: &str) -> Vec<&Written> {
        self.fake
            .written
            .iter()
            .filter(|w| w.method == method && w.route.starts_with(route))
            .collect()
    }

    /// Whether the log says `words` anywhere.
    pub fn said(&self, words: &str) -> bool {
        self.log.iter().any(|line| line.contains(words))
    }
}

pub use pr_hygiene_engine::evidence::Method;
