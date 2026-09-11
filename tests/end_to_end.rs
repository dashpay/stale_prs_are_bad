//! Fixture-driven end-to-end test: parse → analyze → score → render, across
//! two registered repositories.
//!
//! The fixtures mimic real GitHub GraphQL responses so the parser, classifier,
//! scorer, policy routing, engine-state join and renderer all exercise their
//! hot paths together.

use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use pr_hygiene::{analyzer, config::Config, fetcher, policy, renderer, scorer};

const PLATFORM: &str = "dashpay/platform";
const DASHCORE: &str = "dashpay/rust-dashcore";

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn load_fixture(name: &str) -> serde_json::Value {
    let text = std::fs::read_to_string(fixture_dir().join(name)).expect("fixture missing");
    serde_json::from_str(&text).expect("fixture not valid JSON")
}

fn parse_all(fixture: &serde_json::Value, repo: &str) -> Vec<pr_hygiene::model::RawPr> {
    fixture
        .pointer("/data/repository/pullRequests/nodes")
        .and_then(|v| v.as_array())
        .expect("nodes")
        .iter()
        .map(|node| fetcher::parse_pr_node(node, repo).expect("parse").0)
        .collect()
}

/// A scratch directory holding a policy registry for both fixture repositories
/// and an engine-state export for platform only, removed on drop.
struct Scratch {
    root: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "pr-hygiene-e2e-{}-{}",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        let policies = root.join("policies");
        let state = root.join("policy-state");
        std::fs::create_dir_all(&policies).unwrap();
        std::fs::create_dir_all(&state).unwrap();
        std::fs::write(
            policies.join("repositories.json"),
            r#"{"version": 1, "repositories": [
                {"repository": "dashpay/platform", "policy": "platform.json", "mode": "preview", "engine_revision": null},
                {"repository": "dashpay/rust-dashcore", "policy": "rust-dashcore.json", "mode": "preview", "engine_revision": null}
            ]}"#,
        )
        .unwrap();
        std::fs::write(
            policies.join("platform.json"),
            r#"{"version": 1, "repository": "dashpay/platform", "max_active_prs": 5,
                "target_branches": ["master"],
                "fallback": {"owners": ["QuantumExplorer"], "reviewers": []},
                "areas": [
                  {"id": "wasm-sdk", "paths": ["packages/wasm-sdk/"], "owners": ["shumkov"], "reviewers": [],
                   "metadata": {"contributors": ["Sam"]}},
                  {"id": "validation", "paths": ["src/"], "owners": ["dave"], "reviewers": []}
                ]}"#,
        )
        .unwrap();
        std::fs::write(
            policies.join("rust-dashcore.json"),
            r#"{"version": 1, "repository": "dashpay/rust-dashcore", "max_active_prs": 5,
                "target_branches": ["dev"],
                "fallback": {"owners": ["QuantumExplorer"], "reviewers": []},
                "areas": [
                  {"id": "dash-spv", "paths": ["dash-spv/"], "owners": [], "reviewers": ["ZocoLini", "xdustinface"],
                   "unresolved": ["Owner: no owner visible in responsibility sheet"]}
                ]}"#,
        )
        .unwrap();
        // Engine export for platform only; rust-dashcore's is deliberately absent.
        std::fs::write(
            state.join("platform.json"),
            r#"{"generated_at": "2026-05-19T05:00:00Z", "pull_requests": [
                {"repository": "dashpay/platform", "number": 3000, "author": "carol", "state": "ready-for-human",
                 "status": "pending", "blockers": ["Human approval or objection resolution is required"],
                 "reviewers": ["alice"], "areas": ["fallback"]},
                {"repository": "dashpay/platform", "number": 3001, "author": "thepastaclaw", "state": "ready-to-merge",
                 "status": "success", "blockers": [], "reviewers": [], "areas": ["fallback"]},
                {"repository": "dashpay/platform", "number": 7000, "author": "carol", "state": "waiting-bots",
                 "status": "pending", "blockers": ["CodeRabbit review pending"], "reviewers": [], "areas": ["fallback"]},
                {"repository": "dashpay/platform", "number": 1234, "author": "alice", "state": "waiting-author",
                 "status": "pending",
                 "blockers": ["CI failing", "Changes requested by bob-reviewer", "2 unresolved review threads", "Branch behind master"],
                 "reviewers": ["bob-reviewer"], "areas": ["fallback"]}
            ]}"#,
        )
        .unwrap();
        Self { root }
    }

    fn policies(&self) -> PathBuf {
        self.root.join("policies")
    }

    fn policy_state(&self) -> PathBuf {
        self.root.join("policy-state")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn end_to_end_pipeline_matches_snapshot() {
    let scratch = Scratch::new();
    let registry = policy::load_registry(&scratch.policies()).expect("registry loads");
    let repo_names: Vec<String> = registry.iter().map(|(r, _)| r.clone()).collect();
    assert_eq!(repo_names, vec![PLATFORM, DASHCORE]);

    let cfg = Config::default();
    let now = Utc.with_ymd_and_hms(2026, 5, 19, 6, 0, 0).unwrap();
    let today = now.date_naive();

    let platform_raw = parse_all(&load_fixture("sample_prs.json"), PLATFORM);
    assert_eq!(platform_raw.len(), 12);
    let dashcore_raw = parse_all(&load_fixture("sample_prs_rust_dashcore.json"), DASHCORE);
    assert_eq!(dashcore_raw.len(), 2);
    assert!(dashcore_raw.iter().all(|p| p.repo == DASHCORE));

    // Each repository is analyzed against its own default branch, as main.rs
    // does after auto-detecting it from GraphQL.
    // Platform: two PRs are excluded — WIP-labeled #9999 and dependabot-authored
    // #4001. #5000 (postponed → deferred), #6000 (draft), #2988 (targets v3.0 →
    // stale) all survive.
    let mut analyzed = analyzer::analyze(platform_raw, &cfg, Some("master"), now);
    let numbers: Vec<u64> = analyzed.iter().map(|p| p.raw.number).collect();
    // #3001 by thepastaclaw will get merged into PastaPastaPasta's row via alias.
    // #7000 is CI-failing, #8000 is changes-requested (both by carol).
    assert_eq!(
        numbers,
        vec![1234, 1240, 8000, 7000, 3001, 2001, 3000, 5000, 6000, 2988]
    );
    analyzed.extend(analyzer::analyze(dashcore_raw, &cfg, Some("dev"), now));

    let mut cache = HashMap::new();
    let filtered = analyzer::apply_grace_period(analyzed, &mut cache, 14, today);
    assert_eq!(filtered.len(), 12);

    let policies: HashMap<String, policy::Policy> = registry.iter().cloned().collect();
    let mut scored = scorer::score_prs(filtered, &cfg, &policies, now);

    let find = |repo: &str, number: u64| {
        scored
            .iter()
            .find(|s| s.pr.raw.repo == repo && s.pr.raw.number == number)
            .unwrap_or_else(|| panic!("{repo}#{number} missing"))
    };

    // PR 5000 is deferred — its unresolved thread should be wiped, needs_action=false.
    let pr5000 = find(PLATFORM, 5000);
    assert!(pr5000.pr.is_deferred);
    assert_eq!(pr5000.unresolved_total, 0);
    assert!(!pr5000.pr.needs_author_action);

    // PR 1234 should be flagged needs_author_action (changes requested + CI failing).
    let pr1234 = find(PLATFORM, 1234);
    assert!(pr1234.pr.needs_author_action);
    assert!(pr1234.pr.changes_requested);
    assert!(pr1234.pr.ci_failing);
    // Filtered: t3 (resolved), t4 (outdated), t5 (author replied last).
    assert_eq!(pr1234.unresolved_total, 2);
    // Both unresolved threads escalate to High because of CHANGES_REQUESTED.
    assert_eq!(pr1234.unresolved_by_severity.high, 2);
    // No changed files → no routing, and nobody's queue (mirrors the engine's
    // configuration-error for missing file evidence).
    assert!(pr1234.areas.is_empty());
    assert_eq!(
        pr1234.routing_unavailable.as_deref(),
        Some("no changed-file evidence")
    );

    // PR 1240 has merge conflict → needs_author_action.
    let pr1240 = find(PLATFORM, 1240);
    assert!(pr1240.pr.has_merge_conflict);
    assert!(pr1240.pr.needs_author_action);

    // PR 2001: reviewer commented (carol), author hasn't pushed since → reviewer-owes-response.
    let pr2001 = find(PLATFORM, 2001);
    assert!(!pr2001.pr.needs_author_action);
    assert_eq!(pr2001.unresolved_total, 1);

    // PR 3000: clean, no threads; a docs file no area claims → fallback.
    let pr3000 = find(PLATFORM, 3000);
    assert!(!pr3000.pr.needs_author_action);
    assert_eq!(pr3000.unresolved_total, 0);
    assert_eq!(pr3000.areas, vec!["fallback"]);
    assert!(pr3000.routing_unavailable.is_none());

    // PR 3001 touches a workflow file no area claims → repository fallback.
    let pr3001 = find(PLATFORM, 3001);
    assert_eq!(pr3001.areas, vec!["fallback"]);
    assert_eq!(pr3001.routed_reviewers, vec!["QuantumExplorer"]);

    // PR 7000: no threads but CI failing → CI failing bucket.
    let pr7000 = find(PLATFORM, 7000);
    assert!(pr7000.pr.ci_failing);
    assert_eq!(pr7000.unresolved_total, 0);

    // PR 8000: no threads, CI green, but reviewer requested changes → Changes Requested bucket.
    let pr8000 = find(PLATFORM, 8000);
    assert!(pr8000.pr.changes_requested);
    assert_eq!(pr8000.unresolved_total, 0);
    assert!(!pr8000.pr.ci_failing);
    // GitHub reports 60 changed files but the query returned one: the partial
    // list must not be routed as if it were complete.
    assert!(pr8000.pr.raw.changed_files_truncated);
    assert!(pr8000.areas.is_empty());
    assert_eq!(
        pr8000.routing_unavailable.as_deref(),
        Some("changed files truncated (1 fetched)")
    );

    // PR 2988 touches wasm-sdk/ → routes to shumkov (but it's stale, see below).
    let pr2988 = find(PLATFORM, 2988);
    assert_eq!(pr2988.areas, vec!["wasm-sdk"]);
    assert_eq!(pr2988.routed_reviewers, vec!["shumkov"]);

    // rust-dashcore #101 lands in an area whose ownership is still unresolved;
    // its reviewers are routed anyway and the gap is flagged.
    let pr101 = find(DASHCORE, 101);
    assert_eq!(pr101.areas, vec!["dash-spv"]);
    assert_eq!(pr101.unresolved_areas, vec!["dash-spv"]);
    assert_eq!(pr101.routed_reviewers, vec!["ZocoLini", "xdustinface"]);

    // rust-dashcore #102: README falls back to QuantumExplorer, who is also
    // explicitly requested — the queue must not double-count.
    let pr102 = find(DASHCORE, 102);
    assert_eq!(pr102.areas, vec!["fallback"]);
    assert_eq!(pr102.routed_reviewers, vec!["QuantumExplorer"]);

    // Engine state: only platform's export exists.
    let engine_states =
        policy::load_engine_states(Some(&scratch.policy_state()), &registry).unwrap();
    for (repo, state) in &engine_states {
        scorer::attach_engine_state(&mut scored, repo, state);
    }
    let repos: Vec<renderer::RepoStatus> = repo_names
        .iter()
        .map(|repo| renderer::RepoStatus {
            repo: repo.clone(),
            engine_state_available: engine_states.contains_key(repo),
            fetch_error: None,
        })
        .collect();
    assert!(repos[0].engine_state_available);
    assert!(!repos[1].engine_state_available);
    let find = |repo: &str, number: u64| {
        scored
            .iter()
            .find(|s| s.pr.raw.repo == repo && s.pr.raw.number == number)
            .unwrap_or_else(|| panic!("{repo}#{number} missing"))
    };
    assert_eq!(
        find(PLATFORM, 3000)
            .policy_state
            .as_ref()
            .map(|p| p.state.as_str()),
        Some("ready-for-human")
    );
    assert!(find(PLATFORM, 2001).policy_state.is_none(), "not in export");
    assert!(find(DASHCORE, 101).policy_state.is_none(), "no export");

    let authors = scorer::rollup_authors(&scored, &cfg, None);
    let alice = authors.iter().find(|a| a.login == "alice").unwrap();
    assert_eq!(alice.total_open_prs, 4); // includes the deferred PR and dashcore #102
    assert_eq!(alice.dirty_prs, 2);
    assert_eq!(alice.deferred_prs, 1);
    assert_eq!(alice.clean_prs, 1);
    assert_eq!(alice.prs_needing_author_action, 2);
    // Alice is the requested reviewer on Carol's clean PR #3000. PR #7000 is
    // also by carol but CI is failing, so it does NOT add to alice's queue.
    assert_eq!(alice.awaiting_review, 1);
    assert_eq!(alice.ready_for_human_prs, 0, "1234 is waiting-author");

    // Carol has clean #3000, CI-failing #7000, and changes-requested #8000.
    let carol = authors.iter().find(|a| a.login == "carol").unwrap();
    assert_eq!(carol.total_open_prs, 3);
    assert_eq!(carol.clean_prs, 1);
    assert_eq!(carol.ci_failing_prs, 1);
    assert_eq!(carol.changes_requested_prs, 1);
    assert_eq!(carol.dirty_prs, 0);
    assert_eq!(carol.ready_for_human_prs, 1, "#3000 is ready-for-human");

    // Bob has #2001 (dirty), the draft #6000, and dashcore #101 (clean).
    let bob = authors.iter().find(|a| a.login == "bob").unwrap();
    assert_eq!(bob.total_open_prs, 3);
    assert_eq!(bob.dirty_prs, 1);
    assert_eq!(bob.draft_prs, 1);
    assert_eq!(bob.clean_prs, 1);

    // PR #2988 targets v3.0 → stale. Touches wasm-sdk/ → routes to shumkov.
    // But it's stale, so it should NOT enter shumkov's "To review" queue.
    // PR #3001 by thepastaclaw should merge into PastaPastaPasta's row via alias.
    let pasta = authors
        .iter()
        .find(|a| a.login == "PastaPastaPasta")
        .unwrap();
    assert_eq!(pasta.stale_prs, 1, "their own PR #2988 is stale");
    assert_eq!(pasta.dirty_prs, 0, "no dirty PRs of their own");
    assert_eq!(pasta.aliases.len(), 1);
    assert_eq!(pasta.aliases[0].login, "thepastaclaw");
    assert_eq!(
        pasta.aliases[0].clean_prs, 1,
        "thepastaclaw's #3001 is clean"
    );
    assert_eq!(
        pasta.aliases[0].ready_for_human_prs, 1,
        "thepastaclaw's #3001 is ready-to-merge"
    );
    assert_eq!(pasta.combined_total_open_prs(), 2); // #2988 + #3001
                                                    // No standalone thepastaclaw row in the output — it was absorbed.
    assert!(!authors.iter().any(|a| a.login == "thepastaclaw"));
    assert!(
        !authors.iter().any(|a| a.login == "shumkov"),
        "shumkov should NOT be added — the only routable PR is stale"
    );

    // The queue is routed ∪ requested: QuantumExplorer owes #3000 and #3001
    // (platform fallback) and #102 (dashcore fallback + explicit request) —
    // once each.
    let qe = authors
        .iter()
        .find(|a| a.login == "QuantumExplorer")
        .unwrap();
    assert_eq!(qe.awaiting_review, 3);
    for login in ["ZocoLini", "xdustinface"] {
        let r = authors.iter().find(|a| a.login == login).unwrap();
        assert_eq!(r.awaiting_review, 1, "{login} owes dashcore #101");
    }
    assert!(
        !authors.iter().any(|a| a.login == "dave"),
        "dave's area PR (#8000) has a truncated file list and changes requested"
    );

    let ctx = renderer::RenderContext {
        now,
        commit_sha: Some("abc1234"),
        config_path: ".pr-hygiene.yml",
        has_history: false,
        repos: &repos,
    };
    let md = renderer::render(&scored, &authors, &ctx);

    insta::assert_snapshot!(md);
}
