use chrono::{DateTime, Utc};
use glob::Pattern;
use std::collections::{HashMap, HashSet};

use crate::config::{AgeMultiplier, Config, RoutingRule};
use crate::model::{
    AnalyzedPr, AuthorRollup, AuthorSnapshot, BySeverity, BySource, PrSnapshot, ScoredPr, Severity,
    Snapshot, Summary, ThreadSource,
};

pub fn score_prs(prs: Vec<AnalyzedPr>, cfg: &Config, now: DateTime<Utc>) -> Vec<ScoredPr> {
    let compiled_rules = compile_rules(&cfg.review_routing);
    prs.into_iter()
        .map(|p| score_one(p, cfg, &compiled_rules, now))
        .collect()
}

fn compile_rules(rules: &[RoutingRule]) -> Vec<(Vec<Pattern>, &RoutingRule)> {
    rules
        .iter()
        .map(|r| {
            let pats: Vec<Pattern> = r
                .paths
                .iter()
                .filter_map(|p| Pattern::new(p).ok())
                .collect();
            (pats, r)
        })
        .collect()
}

/// Returns (routed_reviewers_still_owing_review, any_rule_matched).
/// When a rule matches but the resolved reviewer has already submitted a review
/// on this PR (any state), they're omitted from the list — they've done their
/// job. The `matched` flag stays true so callers know routing is in play.
fn route_reviewers(pr: &AnalyzedPr, rules: &[(Vec<Pattern>, &RoutingRule)]) -> (Vec<String>, bool) {
    let mut out: Vec<String> = Vec::new();
    let mut any_matched = false;
    let author_lc = pr.raw.author.as_deref().map(str::to_ascii_lowercase);
    let reviewed_logins: HashSet<String> = pr
        .raw
        .reviews
        .iter()
        .filter_map(|r| r.author.as_deref())
        .map(|s| s.to_ascii_lowercase())
        .collect();
    for (patterns, rule) in rules {
        let matches = pr.raw.changed_files.iter().any(|f| {
            let path = std::path::Path::new(f);
            patterns
                .iter()
                .any(|p| p.matches(f) || p.matches_path(path))
        });
        if !matches {
            continue;
        }
        any_matched = true;
        let Some(login) = resolve_routed_reviewer(rule, pr.raw.author.as_deref()) else {
            continue;
        };
        let lc = login.to_ascii_lowercase();
        if Some(&lc) == author_lc.as_ref() {
            continue;
        }
        if reviewed_logins.contains(&lc) {
            // Routed primary already reviewed — they don't owe another review.
            continue;
        }
        if !out.iter().any(|r| r.eq_ignore_ascii_case(login)) {
            out.push(login.to_string());
        }
    }
    (out, any_matched)
}

fn score_one(
    pr: AnalyzedPr,
    cfg: &Config,
    rules: &[(Vec<Pattern>, &RoutingRule)],
    now: DateTime<Utc>,
) -> ScoredPr {
    let mut by_severity = BySeverity::default();
    let mut by_source = BySource::default();
    let mut oldest_age = 0.0_f64;
    for t in &pr.unresolved_threads {
        match t.severity {
            Severity::High => by_severity.high += 1,
            Severity::Medium => by_severity.medium += 1,
            Severity::Low => by_severity.low += 1,
        }
        match t.source {
            ThreadSource::Coderabbit => by_source.coderabbit += 1,
            ThreadSource::Bot => by_source.bot += 1,
            ThreadSource::Human => by_source.human += 1,
        }
        let age = (now - t.first_comment_at).num_seconds().max(0) as f64 / 86_400.0;
        if age > oldest_age {
            oldest_age = age;
        }
    }
    let base = by_severity.high as f64 * cfg.weights.high
        + by_severity.medium as f64 * cfg.weights.medium
        + by_severity.low as f64 * cfg.weights.low;
    let mult = age_multiplier(oldest_age, cfg.age_multiplier);
    let score = base * mult;
    let unresolved_total = pr.unresolved_threads.len() as u32;
    let (routed_reviewers, routing_matched) = route_reviewers(&pr, rules);
    ScoredPr {
        pr,
        score,
        oldest_thread_age_days: oldest_age,
        unresolved_by_severity: by_severity,
        unresolved_by_source: by_source,
        unresolved_total,
        routed_reviewers,
        routing_matched,
    }
}

/// Resolve a routing rule against the PR author. Returns the primary unless
/// the author IS the primary, in which case it returns the fallback (or None).
fn resolve_routed_reviewer<'a>(rule: &'a RoutingRule, author: Option<&str>) -> Option<&'a str> {
    let is_self = author.is_some_and(|a| a.eq_ignore_ascii_case(&rule.primary));
    if is_self {
        rule.fallback.as_deref()
    } else {
        Some(rule.primary.as_str())
    }
}

fn age_multiplier(days: f64, kind: AgeMultiplier) -> f64 {
    let raw = match kind {
        AgeMultiplier::None => 1.0,
        AgeMultiplier::Ln => (days + 1.0).ln(),
        AgeMultiplier::Log10 => (days + 1.0).log10(),
    };
    raw.max(1.0)
}

pub fn rollup_authors(
    scored: &[ScoredPr],
    cfg: &Config,
    previous: Option<&Snapshot>,
) -> Vec<AuthorRollup> {
    let mut by_login: HashMap<String, AuthorRollup> = HashMap::new();
    let ensure = |map: &mut HashMap<String, AuthorRollup>, login: &str| {
        map.entry(login.to_string())
            .or_insert_with(|| AuthorRollup {
                login: login.to_string(),
                total_open_prs: 0,
                clean_prs: 0,
                dirty_prs: 0,
                deferred_prs: 0,
                draft_prs: 0,
                stale_prs: 0,
                ci_failing_prs: 0,
                changes_requested_prs: 0,
                prs_needing_author_action: 0,
                total_unresolved: 0,
                unresolved_coderabbit: 0,
                unresolved_human: 0,
                unresolved_bot: 0,
                awaiting_review: 0,
                total_score: 0.0,
                oldest_stale_pr_days: 0.0,
                delta_vs_last_week: None,
                aliases: vec![],
            });
    };

    for s in scored {
        // Author-side counts.
        if let Some(login) = s.pr.raw.author.as_deref() {
            ensure(&mut by_login, login);
            let entry = by_login.get_mut(login).expect("just inserted");
            entry.total_open_prs += 1;
            // Precedence: deferred > stale > draft > unresolved-comments > changes-requested > ci-failing > clean.
            // The threads describe specific changes; the formal CHANGES_REQUESTED
            // state is what's left when all threads are resolved but the reviewer
            // hasn't re-approved yet.
            if s.pr.is_deferred {
                entry.deferred_prs += 1;
            } else if s.pr.is_stale {
                entry.stale_prs += 1;
            } else if s.pr.raw.is_draft {
                entry.draft_prs += 1;
            } else if s.unresolved_total > 0 {
                entry.dirty_prs += 1;
            } else if s.pr.changes_requested {
                entry.changes_requested_prs += 1;
            } else if s.pr.ci_failing {
                entry.ci_failing_prs += 1;
            } else {
                entry.clean_prs += 1;
            }
            if s.pr.needs_author_action {
                entry.prs_needing_author_action += 1;
            }
            entry.total_unresolved += s.unresolved_total;
            entry.unresolved_coderabbit += s.unresolved_by_source.coderabbit;
            entry.unresolved_human += s.unresolved_by_source.human;
            entry.unresolved_bot += s.unresolved_by_source.bot;
            entry.total_score += s.score;
            if s.oldest_thread_age_days > entry.oldest_stale_pr_days {
                entry.oldest_stale_pr_days = s.oldest_thread_age_days;
            }
        }

        // Reviewer-side counts: only PRs actually ready for review count.
        // CI must be passing (or at least not failing); CHANGES_REQUESTED means
        // the reviewer formally said no and the author owes a fix first.
        if s.pr.is_deferred
            || s.pr.is_stale
            || s.pr.raw.is_draft
            || s.unresolved_total > 0
            || s.pr.has_merge_conflict
            || s.pr.ci_failing
            || s.pr.changes_requested
        {
            continue;
        }

        let author = s.pr.raw.author.as_deref();
        let mut seen_lc: HashSet<String> = HashSet::new();
        let mut bump = |map: &mut HashMap<String, AuthorRollup>, login: &str| {
            let lc = login.to_ascii_lowercase();
            if !seen_lc.insert(lc) {
                return;
            }
            // Self-review never counts. Compare via alias-canonical login so that
            // an AI-assistant account reviewing its principal's PR (or vice versa)
            // is treated as self-review.
            if let Some(a) = author {
                if same_principal(a, login, &cfg.author_aliases) {
                    return;
                }
            }
            ensure(map, login);
            map.get_mut(login).expect("just inserted").awaiting_review += 1;
        };
        // When a routing rule matched, the routed reviewers ARE the queue —
        // explicit GitHub reviewers are ignored. (If routing matched but the
        // routed primary already reviewed, routed_reviewers is empty → queue is
        // empty → PR is "handled by the owner".)
        if s.routing_matched {
            for r in &s.routed_reviewers {
                bump(&mut by_login, r);
            }
        } else {
            for r in &s.pr.raw.requested_reviewers {
                bump(&mut by_login, r);
            }
        }
    }

    // Delta-vs-last-week lookup keys by snapshot's login. Compute BEFORE merging
    // aliases so each rollup compares against its own historical entry.
    if let Some(prev) = previous {
        let prev_map: HashMap<&str, u32> = prev
            .per_author
            .iter()
            .map(|a| (a.login.as_str(), a.prs_needing_author_action))
            .collect();
        for r in by_login.values_mut() {
            if let Some(prev_count) = prev_map.get(r.login.as_str()) {
                r.delta_vs_last_week =
                    Some(r.prs_needing_author_action as i32 - *prev_count as i32);
            }
        }
    }

    // Apply alias merging: move each alias's rollup into the principal's `aliases` vec.
    apply_alias_merging(&mut by_login, &cfg.author_aliases);

    let mut out: Vec<AuthorRollup> = by_login.into_values().collect();
    // Sort by COMBINED (principal + aliases) so an aliased row's true weight
    // determines its position.
    out.sort_by(|a, b| {
        b.combined_dirty_prs()
            .cmp(&a.combined_dirty_prs())
            .then(
                b.combined_prs_needing_author_action()
                    .cmp(&a.combined_prs_needing_author_action()),
            )
            .then(
                b.combined_awaiting_review()
                    .cmp(&a.combined_awaiting_review()),
            )
            .then(
                b.combined_total_score()
                    .partial_cmp(&a.combined_total_score())
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
            .then(
                b.combined_total_unresolved()
                    .cmp(&a.combined_total_unresolved()),
            )
            .then(a.login.cmp(&b.login))
    });
    out
}

/// Move alias rollups into their principals' `aliases` vec. Case-insensitive matching.
/// If the principal has no rollup yet (e.g., they haven't authored any PRs of their own
/// and aren't a reviewer), one is created so the principal still appears in the scoreboard
/// with the alias's data attached.
fn apply_alias_merging(
    by_login: &mut HashMap<String, AuthorRollup>,
    aliases: &HashMap<String, String>,
) {
    for (alias_login, principal_login) in aliases {
        let actual_alias_key = by_login
            .keys()
            .find(|k| k.eq_ignore_ascii_case(alias_login))
            .cloned();
        let Some(alias_key) = actual_alias_key else {
            continue;
        };
        let alias_rollup = by_login.remove(&alias_key).expect("just confirmed present");

        let actual_principal_key = by_login
            .keys()
            .find(|k| k.eq_ignore_ascii_case(principal_login))
            .cloned()
            .unwrap_or_else(|| principal_login.clone());
        let principal = by_login
            .entry(actual_principal_key)
            .or_insert_with(|| AuthorRollup {
                login: principal_login.clone(),
                total_open_prs: 0,
                clean_prs: 0,
                dirty_prs: 0,
                deferred_prs: 0,
                draft_prs: 0,
                stale_prs: 0,
                ci_failing_prs: 0,
                changes_requested_prs: 0,
                prs_needing_author_action: 0,
                total_unresolved: 0,
                unresolved_coderabbit: 0,
                unresolved_human: 0,
                unresolved_bot: 0,
                awaiting_review: 0,
                total_score: 0.0,
                oldest_stale_pr_days: 0.0,
                delta_vs_last_week: None,
                aliases: vec![],
            });
        principal.aliases.push(alias_rollup);
    }
}

/// True when two logins resolve to the same canonical principal via the alias map.
/// An alias maps to its principal; a principal maps to itself.
fn same_principal(a: &str, b: &str, aliases: &HashMap<String, String>) -> bool {
    let canon_a = canonicalize(a, aliases);
    let canon_b = canonicalize(b, aliases);
    canon_a.eq_ignore_ascii_case(canon_b)
}

fn canonicalize<'a>(login: &'a str, aliases: &'a HashMap<String, String>) -> &'a str {
    for (alias, principal) in aliases {
        if alias.eq_ignore_ascii_case(login) {
            return principal.as_str();
        }
    }
    login
}

pub fn build_snapshot(
    date: chrono::NaiveDate,
    scored: &[ScoredPr],
    authors: &[AuthorRollup],
) -> Snapshot {
    let open_prs = scored.len() as u32;
    let prs_needing = scored.iter().filter(|s| s.pr.needs_author_action).count() as u32;
    let total_unresolved: u32 = scored.iter().map(|s| s.unresolved_total).sum();
    // Snapshot stores COMBINED (principal + aliases) values so delta_vs_last_week
    // is computed against the principal's full attribution.
    let per_author = authors
        .iter()
        .map(|a| AuthorSnapshot {
            login: a.login.clone(),
            total_open_prs: a.combined_total_open_prs(),
            clean_prs: a.combined_clean_prs(),
            dirty_prs: a.combined_dirty_prs(),
            deferred_prs: a.combined_deferred_prs(),
            draft_prs: a.combined_draft_prs(),
            stale_prs: a.combined_stale_prs(),
            ci_failing_prs: a.combined_ci_failing_prs(),
            changes_requested_prs: a.combined_changes_requested_prs(),
            awaiting_review: a.combined_awaiting_review(),
            prs_needing_author_action: a.combined_prs_needing_author_action(),
            total_unresolved: a.combined_total_unresolved(),
            total_score: a.combined_total_score(),
            oldest_stale_pr_days: a.oldest_stale_pr_days,
        })
        .collect();
    let per_pr = scored
        .iter()
        .map(|s| PrSnapshot {
            number: s.pr.raw.number,
            author: s.pr.raw.author.clone(),
            score: s.score,
            unresolved_total: s.unresolved_total,
            by_severity: s.unresolved_by_severity.clone(),
            by_source: s.unresolved_by_source.clone(),
            needs_author_action: s.pr.needs_author_action,
        })
        .collect();
    Snapshot {
        date,
        summary: Summary {
            open_prs,
            prs_needing_author_action: prs_needing,
            total_unresolved,
        },
        per_author,
        per_pr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AnalyzedThread, Mergeable, RawPr};
    use chrono::TimeZone;

    fn dt(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    fn pr(author: &str, threads: Vec<AnalyzedThread>, needs_action: bool) -> AnalyzedPr {
        AnalyzedPr {
            raw: RawPr {
                number: 1,
                title: "t".into(),
                url: "u".into(),
                author: Some(author.into()),
                created_at: dt("2026-01-01T00:00:00Z"),
                updated_at: dt("2026-01-01T00:00:00Z"),
                is_draft: false,
                mergeable: Mergeable::Mergeable,
                labels: vec![],
                last_commit: None,
                reviews: vec![],
                threads: vec![],
                requested_reviewers: vec![],
                base_ref: "master".into(),
                changed_files: vec![],
            },
            unresolved_threads: threads,
            days_since_author_push: 0.0,
            days_since_last_reviewer_activity: 0.0,
            needs_author_action: needs_action,
            has_merge_conflict: false,
            changes_requested: false,
            ci_failing: false,
            is_deferred: false,
            is_stale: false,
            stale_reasons: vec![],
        }
    }

    fn thread(severity: Severity, source: ThreadSource, age_days: i64) -> AnalyzedThread {
        let now = Utc.with_ymd_and_hms(2026, 5, 19, 0, 0, 0).unwrap();
        let first = now - chrono::Duration::days(age_days);
        AnalyzedThread {
            id: format!("t-{age_days}-{severity:?}"),
            source,
            severity,
            first_comment_at: first,
            last_comment_at: first,
            first_comment_excerpt: "x".into(),
        }
    }

    #[test]
    fn score_with_no_threads_is_zero() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let s = score_prs(vec![pr("alice", vec![], false)], &cfg, now)
            .pop()
            .unwrap();
        assert_eq!(s.score, 0.0);
        assert_eq!(s.unresolved_total, 0);
    }

    #[test]
    fn score_formula_matches_spec() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        // 2 high + 1 medium + 3 low, oldest 23 days
        let threads = vec![
            thread(Severity::High, ThreadSource::Human, 23),
            thread(Severity::High, ThreadSource::Coderabbit, 10),
            thread(Severity::Medium, ThreadSource::Human, 5),
            thread(Severity::Low, ThreadSource::Coderabbit, 5),
            thread(Severity::Low, ThreadSource::Coderabbit, 5),
            thread(Severity::Low, ThreadSource::Coderabbit, 5),
        ];
        let s = score_prs(vec![pr("alice", threads, true)], &cfg, now)
            .pop()
            .unwrap();
        let base = 2.0 * 5.0 + 1.0 * 2.0 + 3.0 * 0.5;
        let mult = (23.0_f64 + 1.0).ln();
        assert!((s.score - base * mult).abs() < 1e-6, "got {}", s.score);
        assert_eq!(s.unresolved_by_severity.high, 2);
        assert_eq!(s.unresolved_by_severity.medium, 1);
        assert_eq!(s.unresolved_by_severity.low, 3);
        assert_eq!(s.unresolved_by_source.human, 2);
        assert_eq!(s.unresolved_by_source.coderabbit, 4);
    }

    #[test]
    fn age_multiplier_floors_at_one() {
        // Fresh thread: ln(1) = 0, but mult is floored at 1.
        assert_eq!(age_multiplier(0.0, AgeMultiplier::Ln), 1.0);
        assert_eq!(age_multiplier(0.0, AgeMultiplier::Log10), 1.0);
        assert_eq!(age_multiplier(100.0, AgeMultiplier::None), 1.0);
        // 23 days
        let m = age_multiplier(23.0, AgeMultiplier::Ln);
        assert!((m - (24.0_f64).ln()).abs() < 1e-9);
    }

    #[test]
    fn rollup_ranks_by_score_then_action_count() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let prs = score_prs(
            vec![
                pr(
                    "alice",
                    vec![thread(Severity::High, ThreadSource::Human, 23)],
                    true,
                ),
                pr(
                    "bob",
                    vec![
                        thread(Severity::Medium, ThreadSource::Human, 1),
                        thread(Severity::Medium, ThreadSource::Human, 1),
                    ],
                    true,
                ),
            ],
            &cfg,
            now,
        );
        let rolled = rollup_authors(&prs, &cfg, None);
        assert_eq!(rolled[0].login, "alice");
        assert_eq!(rolled[1].login, "bob");
        assert!(rolled.iter().all(|r| r.delta_vs_last_week.is_none()));
    }

    fn analyzed(author: &str, threads: Vec<AnalyzedThread>, needs_action: bool) -> AnalyzedPr {
        pr(author, threads, needs_action)
    }

    #[test]
    fn awaiting_review_only_counts_clean_non_draft_non_deferred_prs() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");

        let mut clean = analyzed("alice", vec![], false);
        clean.raw.requested_reviewers = vec!["reviewer-bob".into()];

        let mut dirty = analyzed(
            "alice",
            vec![thread(Severity::Medium, ThreadSource::Human, 2)],
            false,
        );
        dirty.raw.requested_reviewers = vec!["reviewer-bob".into()];

        let mut deferred = analyzed("alice", vec![], false);
        deferred.raw.requested_reviewers = vec!["reviewer-bob".into()];
        deferred.is_deferred = true;

        let mut draft = analyzed("alice", vec![], false);
        draft.raw.requested_reviewers = vec!["reviewer-bob".into()];
        draft.raw.is_draft = true;

        let scored = score_prs(vec![clean, dirty, deferred, draft], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);
        let bob = rolled
            .iter()
            .find(|r| r.login == "reviewer-bob")
            .expect("bob should be in rollup");
        assert_eq!(bob.awaiting_review, 1);
        // bob only appears as a reviewer — no authoring stats.
        assert_eq!(bob.total_open_prs, 0);
        assert_eq!(bob.dirty_prs, 0);
    }

    #[test]
    fn draft_pr_lands_in_draft_bucket_and_never_needs_action() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let mut p = analyzed(
            "alice",
            vec![thread(Severity::High, ThreadSource::Human, 3)],
            true, // analyzer would set this, but for drafts it's overridden — simulate that
        );
        p.raw.is_draft = true;
        p.needs_author_action = false; // mirrors analyzer's draft-suppression
        let scored = score_prs(vec![p], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);
        let alice = &rolled[0];
        assert_eq!(alice.total_open_prs, 1);
        assert_eq!(alice.draft_prs, 1);
        assert_eq!(alice.dirty_prs, 0);
        assert_eq!(alice.clean_prs, 0);
        assert_eq!(alice.prs_needing_author_action, 0);
    }

    #[test]
    fn path_routing_adds_implicit_reviewer_to_clean_pr() {
        let cfg = Config {
            review_routing: vec![RoutingRule {
                paths: vec!["**/wasm-sdk/**".into()],
                primary: "shumkov".into(),
                fallback: Some("QuantumExplorer".into()),
            }],
            ..Config::default()
        };
        let now = dt("2026-05-19T00:00:00Z");

        // Clean PR by pasta touching wasm-sdk → routes to shumkov.
        let mut clean = analyzed("pasta", vec![], false);
        clean.raw.changed_files = vec!["packages/wasm-sdk/index.ts".into()];
        let scored = score_prs(vec![clean], &cfg, now);
        assert_eq!(scored[0].routed_reviewers, vec!["shumkov"]);
        let rolled = rollup_authors(&scored, &cfg, None);
        let shumkov = rolled.iter().find(|r| r.login == "shumkov").unwrap();
        assert_eq!(shumkov.awaiting_review, 1);

        // Self-authored by shumkov → fallback (QuantumExplorer) gets it.
        let mut self_authored = analyzed("shumkov", vec![], false);
        self_authored.raw.changed_files = vec!["packages/wasm-sdk/foo.ts".into()];
        let scored = score_prs(vec![self_authored], &cfg, now);
        assert_eq!(scored[0].routed_reviewers, vec!["QuantumExplorer"]);
    }

    #[test]
    fn routing_match_makes_routed_reviewer_authoritative() {
        // When a routing rule matches, the routed reviewer IS the queue. Any
        // explicit GitHub-requested reviewers are ignored — we trust the routing.
        let cfg = Config {
            review_routing: vec![RoutingRule {
                paths: vec!["**/wasm-sdk/**".into()],
                primary: "shumkov".into(),
                fallback: None,
            }],
            ..Config::default()
        };
        let now = dt("2026-05-19T00:00:00Z");

        let mut pr = analyzed("pasta", vec![], false);
        pr.raw.changed_files = vec!["packages/wasm-sdk/x.ts".into()];
        // 'random-person' is explicitly requested but routing trumps that.
        pr.raw.requested_reviewers = vec!["random-person".into()];
        let scored = score_prs(vec![pr], &cfg, now);
        assert!(scored[0].routing_matched);
        assert_eq!(scored[0].routed_reviewers, vec!["shumkov"]);
        let rolled = rollup_authors(&scored, &cfg, None);
        let shumkov = rolled.iter().find(|r| r.login == "shumkov").unwrap();
        assert_eq!(shumkov.awaiting_review, 1);
        // The explicitly-requested reviewer is NOT in the queue.
        assert!(!rolled.iter().any(|r| r.login == "random-person"));
    }

    #[test]
    fn routed_reviewer_who_already_reviewed_is_dropped() {
        let cfg = Config {
            review_routing: vec![RoutingRule {
                paths: vec!["**/dashmate/**".into()],
                primary: "shumkov".into(),
                fallback: None,
            }],
            ..Config::default()
        };
        let now = dt("2026-05-19T00:00:00Z");

        let mut pr = analyzed("alice", vec![], false);
        pr.raw.changed_files = vec!["packages/dashmate/lib/x.js".into()];
        // shumkov already submitted a review of any kind.
        pr.raw.reviews = vec![crate::model::Review {
            state: crate::model::ReviewState::Approved,
            author: Some("shumkov".into()),
            submitted_at: Some(dt("2026-05-18T00:00:00Z")),
        }];
        let scored = score_prs(vec![pr], &cfg, now);
        assert!(scored[0].routing_matched);
        assert!(
            scored[0].routed_reviewers.is_empty(),
            "shumkov already reviewed; should not owe another review"
        );
        let rolled = rollup_authors(&scored, &cfg, None);
        // No one is in the queue — the routed primary handled it.
        assert!(rolled.iter().all(|r| r.awaiting_review == 0));
    }

    #[test]
    fn aliases_merge_into_principal_row() {
        let mut cfg = Config {
            default_target_branch: Some("master".into()),
            ..Config::default()
        };
        cfg.author_aliases.clear();
        cfg.author_aliases
            .insert("claude-bot".into(), "alice".into());
        let now = dt("2026-05-19T00:00:00Z");

        let alice_pr = analyzed("alice", vec![], false);
        let bot_pr = analyzed(
            "claude-bot",
            vec![thread(Severity::Medium, ThreadSource::Human, 2)],
            false,
        );
        let scored = score_prs(vec![alice_pr, bot_pr], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);

        // The bot's row is absorbed into alice's. Only alice appears in the output.
        assert_eq!(rolled.len(), 1);
        let alice = &rolled[0];
        assert_eq!(alice.login, "alice");
        assert_eq!(alice.aliases.len(), 1);
        assert_eq!(alice.aliases[0].login, "claude-bot");
        // alice's own clean = 1, alias's dirty = 1
        assert_eq!(alice.clean_prs, 1);
        assert_eq!(alice.aliases[0].dirty_prs, 1);
        // Combined helpers expose the sum
        assert_eq!(alice.combined_total_open_prs(), 2);
        assert_eq!(alice.combined_dirty_prs(), 1);
        assert_eq!(alice.combined_clean_prs(), 1);
    }

    #[test]
    fn alias_authoring_principal_reviewing_is_self_review() {
        // alice has a claude-bot alias. claude-bot authors a clean PR with alice
        // as the requested reviewer. alice shouldn't end up in her own to-review.
        let mut cfg = Config {
            default_target_branch: Some("master".into()),
            ..Config::default()
        };
        cfg.author_aliases.clear();
        cfg.author_aliases
            .insert("claude-bot".into(), "alice".into());
        let now = dt("2026-05-19T00:00:00Z");

        let mut pr = analyzed("claude-bot", vec![], false);
        pr.raw.requested_reviewers = vec!["alice".into()];
        let scored = score_prs(vec![pr], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);

        // Only one merged row, with awaiting_review = 0 (self-review).
        assert_eq!(rolled.len(), 1);
        assert_eq!(rolled[0].login, "alice");
        assert_eq!(rolled[0].combined_awaiting_review(), 0);
    }

    #[test]
    fn deferred_takes_precedence_over_draft() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let mut p = analyzed("alice", vec![], false);
        p.raw.is_draft = true;
        p.is_deferred = true;
        let scored = score_prs(vec![p], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);
        let alice = &rolled[0];
        assert_eq!(alice.deferred_prs, 1);
        assert_eq!(alice.draft_prs, 0);
    }

    #[test]
    fn deferred_pr_increments_only_deferred_bucket() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let mut p = analyzed("alice", vec![], false);
        p.is_deferred = true;
        let scored = score_prs(vec![p], &cfg, now);
        let rolled = rollup_authors(&scored, &cfg, None);
        let alice = &rolled[0];
        assert_eq!(alice.total_open_prs, 1);
        assert_eq!(alice.deferred_prs, 1);
        assert_eq!(alice.clean_prs, 0);
        assert_eq!(alice.dirty_prs, 0);
    }

    #[test]
    fn delta_vs_last_week_uses_previous_snapshot() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let prs = score_prs(
            vec![pr(
                "alice",
                vec![thread(Severity::High, ThreadSource::Human, 1)],
                true,
            )],
            &cfg,
            now,
        );
        let previous = Snapshot {
            date: chrono::NaiveDate::from_ymd_opt(2026, 5, 12).unwrap(),
            summary: Summary {
                open_prs: 0,
                prs_needing_author_action: 0,
                total_unresolved: 0,
            },
            per_author: vec![AuthorSnapshot {
                login: "alice".into(),
                total_open_prs: 3,
                clean_prs: 0,
                dirty_prs: 3,
                deferred_prs: 0,
                draft_prs: 0,
                stale_prs: 0,
                ci_failing_prs: 0,
                changes_requested_prs: 0,
                awaiting_review: 0,
                prs_needing_author_action: 5,
                total_unresolved: 10,
                total_score: 0.0,
                oldest_stale_pr_days: 0.0,
            }],
            per_pr: vec![],
        };
        let rolled = rollup_authors(&prs, &cfg, Some(&previous));
        assert_eq!(rolled[0].delta_vs_last_week, Some(1 - 5));
    }
}
