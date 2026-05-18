use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::config::{AgeMultiplier, Config};
use crate::model::{
    AnalyzedPr, AuthorRollup, AuthorSnapshot, BySeverity, BySource, PrSnapshot, ScoredPr, Severity,
    Snapshot, Summary, ThreadSource,
};

pub fn score_prs(prs: Vec<AnalyzedPr>, cfg: &Config, now: DateTime<Utc>) -> Vec<ScoredPr> {
    prs.into_iter().map(|p| score_one(p, cfg, now)).collect()
}

fn score_one(pr: AnalyzedPr, cfg: &Config, now: DateTime<Utc>) -> ScoredPr {
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
    ScoredPr {
        pr,
        score,
        oldest_thread_age_days: oldest_age,
        unresolved_by_severity: by_severity,
        unresolved_by_source: by_source,
        unresolved_total,
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

pub fn rollup_authors(scored: &[ScoredPr], previous: Option<&Snapshot>) -> Vec<AuthorRollup> {
    let mut by_login: HashMap<String, AuthorRollup> = HashMap::new();
    let ensure = |map: &mut HashMap<String, AuthorRollup>, login: &str| {
        map.entry(login.to_string())
            .or_insert_with(|| AuthorRollup {
                login: login.to_string(),
                total_open_prs: 0,
                clean_prs: 0,
                dirty_prs: 0,
                deferred_prs: 0,
                prs_needing_author_action: 0,
                total_unresolved: 0,
                unresolved_coderabbit: 0,
                unresolved_human: 0,
                unresolved_bot: 0,
                awaiting_review: 0,
                total_score: 0.0,
                oldest_stale_pr_days: 0.0,
                delta_vs_last_week: None,
            });
    };
    for s in scored {
        // Author-side counts.
        if let Some(login) = s.pr.raw.author.as_deref() {
            ensure(&mut by_login, login);
            let entry = by_login.get_mut(login).expect("just inserted");
            entry.total_open_prs += 1;
            if s.pr.is_deferred {
                entry.deferred_prs += 1;
            } else if s.unresolved_total == 0 {
                entry.clean_prs += 1;
            } else {
                entry.dirty_prs += 1;
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

        // Reviewer-side counts: only clean, non-draft, non-deferred PRs count
        // as "ready for this person to review". Self-review is ignored.
        if s.pr.is_deferred
            || s.pr.raw.is_draft
            || s.unresolved_total > 0
            || s.pr.has_merge_conflict
        {
            continue;
        }
        for reviewer in &s.pr.raw.requested_reviewers {
            if s.pr
                .raw
                .author
                .as_deref()
                .is_some_and(|a| a.eq_ignore_ascii_case(reviewer))
            {
                continue;
            }
            ensure(&mut by_login, reviewer);
            by_login
                .get_mut(reviewer)
                .expect("just inserted")
                .awaiting_review += 1;
        }
    }

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

    let mut out: Vec<AuthorRollup> = by_login.into_values().collect();
    // Sort: author-side slackers float to top, then reviewer-side slackers, then clean players.
    out.sort_by(|a, b| {
        b.dirty_prs
            .cmp(&a.dirty_prs)
            .then(
                b.prs_needing_author_action
                    .cmp(&a.prs_needing_author_action),
            )
            .then(b.awaiting_review.cmp(&a.awaiting_review))
            .then(
                b.total_score
                    .partial_cmp(&a.total_score)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
            .then(b.total_unresolved.cmp(&a.total_unresolved))
            .then(a.login.cmp(&b.login))
    });
    out
}

pub fn build_snapshot(
    date: chrono::NaiveDate,
    scored: &[ScoredPr],
    authors: &[AuthorRollup],
) -> Snapshot {
    let open_prs = scored.len() as u32;
    let prs_needing = scored.iter().filter(|s| s.pr.needs_author_action).count() as u32;
    let total_unresolved: u32 = scored.iter().map(|s| s.unresolved_total).sum();
    let per_author = authors
        .iter()
        .map(|a| AuthorSnapshot {
            login: a.login.clone(),
            total_open_prs: a.total_open_prs,
            clean_prs: a.clean_prs,
            dirty_prs: a.dirty_prs,
            deferred_prs: a.deferred_prs,
            awaiting_review: a.awaiting_review,
            prs_needing_author_action: a.prs_needing_author_action,
            total_unresolved: a.total_unresolved,
            total_score: a.total_score,
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
            },
            unresolved_threads: threads,
            days_since_author_push: 0.0,
            days_since_last_reviewer_activity: 0.0,
            needs_author_action: needs_action,
            has_merge_conflict: false,
            changes_requested: false,
            ci_failing: false,
            is_deferred: false,
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
        let s = score_one(pr("alice", vec![], false), &cfg, now);
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
        let s = score_one(pr("alice", threads, true), &cfg, now);
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
        let prs = vec![
            score_one(
                pr(
                    "alice",
                    vec![thread(Severity::High, ThreadSource::Human, 23)],
                    true,
                ),
                &cfg,
                now,
            ),
            score_one(
                pr(
                    "bob",
                    vec![
                        thread(Severity::Medium, ThreadSource::Human, 1),
                        thread(Severity::Medium, ThreadSource::Human, 1),
                    ],
                    true,
                ),
                &cfg,
                now,
            ),
        ];
        let rolled = rollup_authors(&prs, None);
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
        let rolled = rollup_authors(&scored, None);
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
    fn deferred_pr_increments_only_deferred_bucket() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let mut p = analyzed("alice", vec![], false);
        p.is_deferred = true;
        let scored = score_prs(vec![p], &cfg, now);
        let rolled = rollup_authors(&scored, None);
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
        let prs = vec![score_one(
            pr(
                "alice",
                vec![thread(Severity::High, ThreadSource::Human, 1)],
                true,
            ),
            &cfg,
            now,
        )];
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
                awaiting_review: 0,
                prs_needing_author_action: 5,
                total_unresolved: 10,
                total_score: 0.0,
                oldest_stale_pr_days: 0.0,
            }],
            per_pr: vec![],
        };
        let rolled = rollup_authors(&prs, Some(&previous));
        assert_eq!(rolled[0].delta_vs_last_week, Some(1 - 5));
    }
}
