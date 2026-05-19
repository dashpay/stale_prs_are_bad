//! Fixture-driven end-to-end test: parse → analyze → score → render.
//!
//! The fixture mimics a real GitHub GraphQL response so the parser, classifier,
//! scorer, and renderer all exercise their hot paths together.

use chrono::{TimeZone, Utc};
use std::collections::HashMap;

use pr_hygiene::{analyzer, config::Config, fetcher, renderer, scorer};

fn load_fixture() -> serde_json::Value {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sample_prs.json"),
    )
    .expect("fixture missing");
    serde_json::from_str(&text).expect("fixture not valid JSON")
}

fn parse_all(fixture: &serde_json::Value) -> Vec<pr_hygiene::model::RawPr> {
    fixture
        .pointer("/data/repository/pullRequests/nodes")
        .and_then(|v| v.as_array())
        .expect("nodes")
        .iter()
        .map(|node| fetcher::parse_pr_node(node).expect("parse").0)
        .collect()
}

#[test]
fn end_to_end_pipeline_matches_snapshot() {
    let fixture = load_fixture();
    let raw = parse_all(&fixture);
    assert_eq!(raw.len(), 12);

    // The fixture's PRs target `master`; pin that here so the branch-stale check
    // mirrors what main.rs does after auto-detecting from GraphQL.
    let cfg = Config {
        default_target_branch: Some("master".into()),
        ..Config::default()
    };
    let now = Utc.with_ymd_and_hms(2026, 5, 19, 6, 0, 0).unwrap();
    let today = now.date_naive();

    // Two PRs are excluded: WIP-labeled #9999 and dependabot-authored #4001.
    // #5000 (postponed → deferred), #6000 (draft), #2988 (targets v3.0 → stale)
    // all survive.
    let analyzed = analyzer::analyze(raw, &cfg, now);
    let numbers: Vec<u64> = analyzed.iter().map(|p| p.raw.number).collect();
    // #3001 by thepastaclaw will get merged into PastaPastaPasta's row via alias.
    // #7000 is CI-failing, #8000 is changes-requested (both by carol).
    assert_eq!(
        numbers,
        vec![1234, 1240, 8000, 7000, 3001, 2001, 3000, 5000, 6000, 2988]
    );

    let mut cache = HashMap::new();
    let filtered = analyzer::apply_grace_period(analyzed, &mut cache, 14, today);
    assert_eq!(filtered.len(), 10);

    let scored = scorer::score_prs(filtered, &cfg, now);

    // PR 5000 is deferred — its unresolved thread should be wiped, needs_action=false.
    let pr5000 = scored.iter().find(|s| s.pr.raw.number == 5000).unwrap();
    assert!(pr5000.pr.is_deferred);
    assert_eq!(pr5000.unresolved_total, 0);
    assert!(!pr5000.pr.needs_author_action);

    // PR 1234 should be flagged needs_author_action (changes requested + CI failing).
    let pr1234 = scored.iter().find(|s| s.pr.raw.number == 1234).unwrap();
    assert!(pr1234.pr.needs_author_action);
    assert!(pr1234.pr.changes_requested);
    assert!(pr1234.pr.ci_failing);
    // Filtered: t3 (resolved), t4 (outdated), t5 (author replied last).
    assert_eq!(pr1234.unresolved_total, 2);
    // Both unresolved threads escalate to High because of CHANGES_REQUESTED.
    assert_eq!(pr1234.unresolved_by_severity.high, 2);

    // PR 1240 has merge conflict → needs_author_action.
    let pr1240 = scored.iter().find(|s| s.pr.raw.number == 1240).unwrap();
    assert!(pr1240.pr.has_merge_conflict);
    assert!(pr1240.pr.needs_author_action);

    // PR 2001: reviewer commented (carol), author hasn't pushed since → reviewer-owes-response.
    let pr2001 = scored.iter().find(|s| s.pr.raw.number == 2001).unwrap();
    assert!(!pr2001.pr.needs_author_action);
    assert_eq!(pr2001.unresolved_total, 1);

    // PR 3000: clean, no threads.
    let pr3000 = scored.iter().find(|s| s.pr.raw.number == 3000).unwrap();
    assert!(!pr3000.pr.needs_author_action);
    assert_eq!(pr3000.unresolved_total, 0);

    // PR 7000: no threads but CI failing → CI failing bucket.
    let pr7000 = scored.iter().find(|s| s.pr.raw.number == 7000).unwrap();
    assert!(pr7000.pr.ci_failing);
    assert_eq!(pr7000.unresolved_total, 0);

    // PR 8000: no threads, CI green, but reviewer requested changes → Changes Requested bucket.
    let pr8000 = scored.iter().find(|s| s.pr.raw.number == 8000).unwrap();
    assert!(pr8000.pr.changes_requested);
    assert_eq!(pr8000.unresolved_total, 0);
    assert!(!pr8000.pr.ci_failing);

    let authors = scorer::rollup_authors(&scored, &cfg, None);
    let alice = authors.iter().find(|a| a.login == "alice").unwrap();
    assert_eq!(alice.total_open_prs, 3); // includes the deferred PR
    assert_eq!(alice.dirty_prs, 2);
    assert_eq!(alice.deferred_prs, 1);
    assert_eq!(alice.prs_needing_author_action, 2);
    // Alice is the requested reviewer on Carol's clean PR #3000. PR #7000 is
    // also by carol but CI is failing, so it does NOT add to alice's queue.
    assert_eq!(alice.awaiting_review, 1);

    // Carol has clean #3000, CI-failing #7000, and changes-requested #8000.
    let carol = authors.iter().find(|a| a.login == "carol").unwrap();
    assert_eq!(carol.total_open_prs, 3);
    assert_eq!(carol.clean_prs, 1);
    assert_eq!(carol.ci_failing_prs, 1);
    assert_eq!(carol.changes_requested_prs, 1);
    assert_eq!(carol.dirty_prs, 0);

    // Bob has #2001 (dirty) and the new draft #6000.
    let bob = authors.iter().find(|a| a.login == "bob").unwrap();
    assert_eq!(bob.total_open_prs, 2);
    assert_eq!(bob.dirty_prs, 1);
    assert_eq!(bob.draft_prs, 1);
    assert_eq!(bob.clean_prs, 0);

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
    assert_eq!(pasta.combined_total_open_prs(), 2); // #2988 + #3001
                                                    // No standalone thepastaclaw row in the output — it was absorbed.
    assert!(!authors.iter().any(|a| a.login == "thepastaclaw"));
    assert!(
        !authors.iter().any(|a| a.login == "shumkov"),
        "shumkov should NOT be added — the only routable PR is stale"
    );

    let ctx = renderer::RenderContext {
        now,
        commit_sha: Some("abc1234"),
        config_path: ".pr-hygiene.yml",
        has_history: false,
    };
    let md = renderer::render(&scored, &authors, &ctx);

    insta::assert_snapshot!(md);
}
