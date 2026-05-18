use chrono::{DateTime, Duration, NaiveDate, Utc};
use std::collections::HashMap;

use crate::config::Config;
use crate::model::{
    AnalyzedPr, AnalyzedThread, Mergeable, RawPr, RawThread, ReviewState, Severity, StatusState,
    ThreadSource,
};

const KNOWN_BOTS: &[&str] = &[
    "dependabot",
    "dependabot[bot]",
    "renovate",
    "renovate[bot]",
    "github-actions",
    "github-actions[bot]",
    "mergify",
    "mergify[bot]",
    "stale",
    "stale[bot]",
];

pub fn analyze(prs: Vec<RawPr>, cfg: &Config, now: DateTime<Utc>) -> Vec<AnalyzedPr> {
    prs.into_iter()
        .filter(|pr| !is_excluded(pr, cfg))
        .map(|pr| analyze_pr(pr, cfg, now))
        .collect()
}

/// Filter out PRs whose author has been visible in this repo for fewer than
/// `grace_period_days` days. Mutates `cache` with newly-discovered or earlier
/// first-seen dates.
pub fn apply_grace_period(
    prs: Vec<AnalyzedPr>,
    cache: &mut HashMap<String, NaiveDate>,
    grace_days: i64,
    today: NaiveDate,
) -> Vec<AnalyzedPr> {
    let mut oldest_pr_by_author: HashMap<String, NaiveDate> = HashMap::new();
    for pr in &prs {
        if let Some(login) = pr.raw.author.clone() {
            let d = pr.raw.created_at.date_naive();
            oldest_pr_by_author
                .entry(login)
                .and_modify(|cur| {
                    if d < *cur {
                        *cur = d;
                    }
                })
                .or_insert(d);
        }
    }
    for (login, oldest) in &oldest_pr_by_author {
        let existing = cache.get(login).copied();
        let candidate = existing.map(|e| e.min(*oldest)).unwrap_or(*oldest);
        cache.insert(login.clone(), candidate.min(today));
    }
    let cutoff = today - Duration::days(grace_days);
    prs.into_iter()
        .filter(|pr| match pr.raw.author.as_deref() {
            None => true,
            Some(login) => cache
                .get(login)
                .map(|first_known| *first_known <= cutoff)
                .unwrap_or(false),
        })
        .collect()
}

fn is_excluded(pr: &RawPr, cfg: &Config) -> bool {
    if let Some(author) = &pr.author {
        if cfg
            .excluded_authors
            .iter()
            .any(|a| a.eq_ignore_ascii_case(author))
        {
            return true;
        }
    }
    let label_set: Vec<String> = pr.labels.iter().map(|l| l.to_lowercase()).collect();
    cfg.excluded_labels
        .iter()
        .any(|excl| label_set.iter().any(|l| l == &excl.to_lowercase()))
}

fn analyze_pr(pr: RawPr, cfg: &Config, now: DateTime<Utc>) -> AnalyzedPr {
    let pr_author = pr.author.clone();
    let is_deferred = has_deferred_label(&pr, cfg);

    // Deferred PRs are intentionally on hold — don't accrue any negative signals.
    if is_deferred {
        return AnalyzedPr {
            raw: pr,
            unresolved_threads: vec![],
            days_since_author_push: 0.0,
            days_since_last_reviewer_activity: 0.0,
            needs_author_action: false,
            has_merge_conflict: false,
            changes_requested: false,
            ci_failing: false,
            is_deferred: true,
        };
    }

    let changes_requested = has_blocking_changes_requested(&pr);
    let has_merge_conflict = matches!(pr.mergeable, Mergeable::Conflicting);
    let ci_failing = matches!(
        pr.last_commit.as_ref().and_then(|c| c.status_state),
        Some(StatusState::Failure) | Some(StatusState::Error)
    );

    let unresolved_threads: Vec<AnalyzedThread> = pr
        .threads
        .iter()
        .filter(|t| is_unresolved(t, pr_author.as_deref()))
        .filter_map(|t| classify_thread(t, pr_author.as_deref(), changes_requested, cfg))
        .filter(|t| {
            if !cfg.count_nitpicks && t.severity == Severity::Low {
                return false;
            }
            if cfg.maintainer_only && !is_maintainer_relevant(t, &pr.threads, cfg) {
                return false;
            }
            true
        })
        .collect();

    let last_author_push_at = pr
        .last_commit
        .as_ref()
        .map(|c| c.committed_date)
        .unwrap_or(pr.created_at);
    let days_since_author_push = days_between(last_author_push_at, now);

    let last_reviewer_activity_at = latest_reviewer_activity(&pr, pr_author.as_deref());
    let days_since_last_reviewer_activity = last_reviewer_activity_at
        .map(|d| days_between(d, now))
        .unwrap_or(0.0);

    let unresolved_total = unresolved_threads.len();
    // Drafts can't "need author action" — the author is still iterating. Never tag them.
    let needs_author_action = !pr.is_draft
        && (changes_requested
            || has_merge_conflict
            || (days_since_author_push > days_since_last_reviewer_activity
                && unresolved_total > 0));

    AnalyzedPr {
        raw: pr,
        unresolved_threads,
        days_since_author_push,
        days_since_last_reviewer_activity,
        needs_author_action,
        has_merge_conflict,
        changes_requested,
        ci_failing,
        is_deferred: false,
    }
}

fn has_deferred_label(pr: &RawPr, cfg: &Config) -> bool {
    let label_set: Vec<String> = pr.labels.iter().map(|l| l.to_lowercase()).collect();
    cfg.deferred_labels
        .iter()
        .any(|d| label_set.iter().any(|l| l == &d.to_lowercase()))
}

fn is_unresolved(t: &RawThread, pr_author: Option<&str>) -> bool {
    if t.is_resolved || t.is_outdated {
        return false;
    }
    if t.comments.is_empty() {
        return false;
    }
    let has_non_author_comment = t
        .comments
        .iter()
        .any(|c| !same_user(c.author.as_deref(), pr_author));
    if !has_non_author_comment {
        return false;
    }
    // Latest comment must not be from the PR author.
    let last = t.comments.last().unwrap();
    !same_user(last.author.as_deref(), pr_author)
}

fn same_user(a: Option<&str>, b: Option<&str>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x.eq_ignore_ascii_case(y),
        _ => false,
    }
}

fn classify_thread(
    t: &RawThread,
    pr_author: Option<&str>,
    pr_has_changes_requested: bool,
    _cfg: &Config,
) -> Option<AnalyzedThread> {
    // Source is determined by the first non-author commenter.
    let first_relevant = t
        .comments
        .iter()
        .find(|c| !same_user(c.author.as_deref(), pr_author))?;
    let source = classify_source(first_relevant.author.as_deref());

    let severity = match source {
        ThreadSource::Coderabbit => coderabbit_severity(&first_relevant.body),
        ThreadSource::Human => {
            if pr_has_changes_requested {
                Severity::High
            } else {
                Severity::Medium
            }
        }
        ThreadSource::Bot => Severity::Low,
    };

    let first_comment_at = first_relevant.created_at;
    let last_comment_at = t
        .comments
        .last()
        .map(|c| c.created_at)
        .unwrap_or(first_comment_at);

    Some(AnalyzedThread {
        id: t.id.clone(),
        source,
        severity,
        first_comment_at,
        last_comment_at,
        first_comment_excerpt: excerpt(&first_relevant.body),
    })
}

fn classify_source(login: Option<&str>) -> ThreadSource {
    let l = match login {
        Some(s) => s.to_lowercase(),
        None => return ThreadSource::Human,
    };
    if l == "coderabbitai" || l == "coderabbitai[bot]" || l.contains("coderabbit") {
        return ThreadSource::Coderabbit;
    }
    if l.ends_with("[bot]") || KNOWN_BOTS.iter().any(|b| b == &l) {
        return ThreadSource::Bot;
    }
    ThreadSource::Human
}

/// Parse a CodeRabbit severity from the first line of the comment body.
/// Defaults to `Medium` if no marker is recognised.
pub fn coderabbit_severity(body: &str) -> Severity {
    let head: String = body.chars().take(400).collect();
    let lower = head.to_lowercase();
    if head.contains('⚠') || lower.contains("potential issue") || lower.contains("potential bug")
    {
        return Severity::High;
    }
    if head.contains('🛠') || lower.contains("refactor suggestion") {
        return Severity::Medium;
    }
    if head.contains('🧹') || lower.contains("nitpick") {
        return Severity::Low;
    }
    if head.contains('🔵') || lower.contains("trivial") {
        return Severity::Low;
    }
    Severity::Medium
}

fn excerpt(body: &str) -> String {
    let first_line = body
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("")
        .trim_start_matches(|c: char| c == '#' || c == '*' || c == '>' || c.is_whitespace());
    let trimmed: String = first_line.chars().take(120).collect();
    if first_line.chars().count() > 120 {
        format!("{trimmed}…")
    } else {
        trimmed
    }
}

fn has_blocking_changes_requested(pr: &RawPr) -> bool {
    // A reviewer is "blocking" when their most recent review is CHANGES_REQUESTED.
    let mut latest_by_reviewer: HashMap<String, (DateTime<Utc>, ReviewState)> = HashMap::new();
    for r in &pr.reviews {
        let Some(login) = r.author.as_deref() else {
            continue;
        };
        let Some(when) = r.submitted_at else { continue };
        match latest_by_reviewer.get(login) {
            Some((existing, _)) if *existing >= when => {}
            _ => {
                latest_by_reviewer.insert(login.to_string(), (when, r.state));
            }
        }
    }
    latest_by_reviewer
        .values()
        .any(|(_, s)| *s == ReviewState::ChangesRequested)
}

fn latest_reviewer_activity(pr: &RawPr, pr_author: Option<&str>) -> Option<DateTime<Utc>> {
    let mut latest: Option<DateTime<Utc>> = None;
    for r in &pr.reviews {
        if same_user(r.author.as_deref(), pr_author) {
            continue;
        }
        if let Some(when) = r.submitted_at {
            latest = Some(latest.map_or(when, |cur| cur.max(when)));
        }
    }
    for t in &pr.threads {
        for c in &t.comments {
            if same_user(c.author.as_deref(), pr_author) {
                continue;
            }
            latest = Some(latest.map_or(c.created_at, |cur| cur.max(c.created_at)));
        }
    }
    latest
}

fn days_between(a: DateTime<Utc>, b: DateTime<Utc>) -> f64 {
    let secs = (b - a).num_seconds().max(0) as f64;
    secs / 86_400.0
}

/// In `maintainer_only` mode, keep a thread only if it has at least one comment
/// from a maintainer or from CodeRabbit.
fn is_maintainer_relevant(t: &AnalyzedThread, raw: &[RawThread], cfg: &Config) -> bool {
    if t.source == ThreadSource::Coderabbit {
        return true;
    }
    let Some(raw_thread) = raw.iter().find(|r| r.id == t.id) else {
        return false;
    };
    raw_thread.comments.iter().any(|c| {
        c.author
            .as_deref()
            .map(|a| {
                cfg.maintainers.iter().any(|m| m.eq_ignore_ascii_case(a))
                    || classify_source(Some(a)) == ThreadSource::Coderabbit
            })
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RawComment;

    fn test_helpers_comment(author: &str, body: &str, created_at: DateTime<Utc>) -> RawComment {
        RawComment {
            author: Some(author.to_string()),
            body: body.to_string(),
            created_at,
        }
    }

    fn dt(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
    }

    fn thread(id: &str, comments: Vec<RawComment>) -> RawThread {
        RawThread {
            id: id.into(),
            is_resolved: false,
            is_outdated: false,
            comments,
        }
    }

    #[test]
    fn resolved_thread_is_skipped() {
        let t = RawThread {
            id: "t1".into(),
            is_resolved: true,
            is_outdated: false,
            comments: vec![test_helpers_comment(
                "bob",
                "fix this",
                dt("2026-01-01T00:00:00Z"),
            )],
        };
        assert!(!is_unresolved(&t, Some("alice")));
    }

    #[test]
    fn outdated_thread_is_skipped() {
        let t = RawThread {
            id: "t1".into(),
            is_resolved: false,
            is_outdated: true,
            comments: vec![test_helpers_comment(
                "bob",
                "fix this",
                dt("2026-01-01T00:00:00Z"),
            )],
        };
        assert!(!is_unresolved(&t, Some("alice")));
    }

    #[test]
    fn thread_with_only_author_comments_is_skipped() {
        let t = thread(
            "t1",
            vec![test_helpers_comment(
                "alice",
                "self note",
                dt("2026-01-01T00:00:00Z"),
            )],
        );
        assert!(!is_unresolved(&t, Some("alice")));
    }

    #[test]
    fn thread_where_author_replied_last_is_skipped() {
        let t = thread(
            "t1",
            vec![
                test_helpers_comment("bob", "fix this", dt("2026-01-01T00:00:00Z")),
                test_helpers_comment("alice", "done", dt("2026-01-02T00:00:00Z")),
            ],
        );
        assert!(!is_unresolved(&t, Some("alice")));
    }

    #[test]
    fn thread_with_reviewer_last_is_unresolved() {
        let t = thread(
            "t1",
            vec![
                test_helpers_comment("bob", "fix this", dt("2026-01-01T00:00:00Z")),
                test_helpers_comment("alice", "done", dt("2026-01-02T00:00:00Z")),
                test_helpers_comment("bob", "still broken", dt("2026-01-03T00:00:00Z")),
            ],
        );
        assert!(is_unresolved(&t, Some("alice")));
    }

    #[test]
    fn source_classification() {
        assert_eq!(
            classify_source(Some("coderabbitai")),
            ThreadSource::Coderabbit
        );
        assert_eq!(
            classify_source(Some("coderabbitai[bot]")),
            ThreadSource::Coderabbit
        );
        assert_eq!(classify_source(Some("dependabot[bot]")), ThreadSource::Bot);
        assert_eq!(classify_source(Some("renovate[bot]")), ThreadSource::Bot);
        assert_eq!(classify_source(Some("alice")), ThreadSource::Human);
        assert_eq!(classify_source(None), ThreadSource::Human);
    }

    #[test]
    fn coderabbit_severity_from_markers() {
        assert_eq!(
            coderabbit_severity("⚠️ **Potential issue**\n\nThis is a bug."),
            Severity::High
        );
        assert_eq!(
            coderabbit_severity("🛠️ Refactor suggestion: extract this"),
            Severity::Medium
        );
        assert_eq!(coderabbit_severity("🧹 Nitpick: rename foo"), Severity::Low);
        assert_eq!(coderabbit_severity("🔵 Trivial: typo"), Severity::Low);
        assert_eq!(coderabbit_severity("hello world"), Severity::Medium);
    }

    #[test]
    fn coderabbit_severity_via_text_only() {
        assert_eq!(
            coderabbit_severity("Potential issue\n\nbug"),
            Severity::High
        );
        assert_eq!(
            coderabbit_severity("Refactor suggestion\n\nclean this"),
            Severity::Medium
        );
        assert_eq!(coderabbit_severity("Nitpick: trivial nit"), Severity::Low);
    }

    #[test]
    fn human_severity_promotes_with_changes_requested() {
        let cfg = Config::default();
        let t = thread(
            "t1",
            vec![test_helpers_comment(
                "bob",
                "please fix",
                dt("2026-01-01T00:00:00Z"),
            )],
        );
        let regular = classify_thread(&t, Some("alice"), false, &cfg).unwrap();
        assert_eq!(regular.severity, Severity::Medium);
        let blocking = classify_thread(&t, Some("alice"), true, &cfg).unwrap();
        assert_eq!(blocking.severity, Severity::High);
    }

    #[test]
    fn excluded_authors_drop_pr() {
        let cfg = Config::default();
        let pr = RawPr {
            number: 1,
            title: "bump".into(),
            url: "u".into(),
            author: Some("dependabot[bot]".into()),
            created_at: dt("2026-01-01T00:00:00Z"),
            updated_at: dt("2026-01-01T00:00:00Z"),
            is_draft: false,
            mergeable: Mergeable::Mergeable,
            labels: vec![],
            last_commit: None,
            reviews: vec![],
            threads: vec![],
            requested_reviewers: vec![],
        };
        assert!(is_excluded(&pr, &cfg));
    }

    #[test]
    fn excluded_labels_drop_pr() {
        let cfg = Config::default();
        let mut pr = RawPr {
            number: 1,
            title: "wip work".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-01-01T00:00:00Z"),
            updated_at: dt("2026-01-01T00:00:00Z"),
            is_draft: false,
            mergeable: Mergeable::Mergeable,
            labels: vec!["WIP".into()],
            last_commit: None,
            reviews: vec![],
            threads: vec![],
            requested_reviewers: vec![],
        };
        assert!(is_excluded(&pr, &cfg));
        pr.labels = vec!["ready".into()];
        assert!(!is_excluded(&pr, &cfg));
    }

    #[test]
    fn count_nitpicks_false_drops_low() {
        let mut cfg = Config {
            count_nitpicks: false,
            ..Config::default()
        };
        let now = dt("2026-05-19T00:00:00Z");
        let pr = RawPr {
            number: 1,
            title: "x".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-01-01T00:00:00Z"),
            updated_at: dt("2026-01-10T00:00:00Z"),
            is_draft: false,
            mergeable: Mergeable::Mergeable,
            labels: vec![],
            last_commit: None,
            reviews: vec![],
            threads: vec![thread(
                "t1",
                vec![test_helpers_comment(
                    "coderabbitai",
                    "🧹 Nitpick: rename",
                    dt("2026-01-02T00:00:00Z"),
                )],
            )],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr.clone()], &cfg, now);
        assert_eq!(analyzed[0].unresolved_threads.len(), 0);
        cfg.count_nitpicks = true;
        let analyzed = analyze(vec![pr], &cfg, now);
        assert_eq!(analyzed[0].unresolved_threads.len(), 1);
    }

    fn last_commit(date: DateTime<Utc>) -> Option<crate::model::LastCommit> {
        Some(crate::model::LastCommit {
            committed_date: date,
            status_state: None,
        })
    }

    #[test]
    fn no_action_when_reviewer_owes_response() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        // Author pushed yesterday; reviewer commented 10 days ago: ball is in reviewer's court.
        let pr = RawPr {
            number: 1,
            title: "x".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-04-01T00:00:00Z"),
            updated_at: now,
            is_draft: false,
            mergeable: Mergeable::Mergeable,
            labels: vec![],
            last_commit: last_commit(dt("2026-05-18T00:00:00Z")),
            reviews: vec![],
            threads: vec![thread(
                "t1",
                vec![test_helpers_comment(
                    "bob",
                    "fix",
                    dt("2026-05-09T00:00:00Z"),
                )],
            )],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr], &cfg, now);
        assert!(!analyzed[0].needs_author_action);
    }

    #[test]
    fn needs_action_when_reviewer_more_recent_than_author_push() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        // Author pushed 20 days ago; reviewer commented 2 days ago.
        let pr = RawPr {
            number: 1,
            title: "x".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-04-01T00:00:00Z"),
            updated_at: now,
            is_draft: false,
            mergeable: Mergeable::Mergeable,
            labels: vec![],
            last_commit: last_commit(dt("2026-04-29T00:00:00Z")),
            reviews: vec![],
            threads: vec![thread(
                "t1",
                vec![test_helpers_comment(
                    "bob",
                    "fix",
                    dt("2026-05-17T00:00:00Z"),
                )],
            )],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr], &cfg, now);
        assert!(analyzed[0].needs_author_action);
    }

    #[test]
    fn deferred_label_marks_pr_and_clears_signals() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        // Real unresolved thread + changes-requested review, but PR is `postponed`.
        let pr = RawPr {
            number: 1,
            title: "wallet migration".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-04-01T00:00:00Z"),
            updated_at: now,
            is_draft: false,
            mergeable: Mergeable::Conflicting,
            labels: vec!["Postponed".into()],
            last_commit: None,
            reviews: vec![],
            threads: vec![thread(
                "t1",
                vec![test_helpers_comment(
                    "bob",
                    "real issue",
                    dt("2026-04-15T00:00:00Z"),
                )],
            )],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr], &cfg, now);
        assert_eq!(analyzed.len(), 1);
        let pr = &analyzed[0];
        assert!(pr.is_deferred);
        assert!(
            pr.unresolved_threads.is_empty(),
            "deferred PR should not accrue unresolved threads"
        );
        assert!(!pr.needs_author_action);
        assert!(!pr.has_merge_conflict, "deferred PR signals are suppressed");
    }

    #[test]
    fn drafts_never_need_author_action() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        // Conflicting + stale unresolved thread + draft → no action required.
        let pr = RawPr {
            number: 1,
            title: "wip foo".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-04-01T00:00:00Z"),
            updated_at: now,
            is_draft: true,
            mergeable: Mergeable::Conflicting,
            labels: vec![],
            last_commit: last_commit(dt("2026-04-29T00:00:00Z")),
            reviews: vec![],
            threads: vec![thread(
                "t1",
                vec![test_helpers_comment(
                    "bob",
                    "fix",
                    dt("2026-05-17T00:00:00Z"),
                )],
            )],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr], &cfg, now);
        assert!(!analyzed[0].needs_author_action);
    }

    #[test]
    fn merge_conflict_triggers_needs_action() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let pr = RawPr {
            number: 1,
            title: "x".into(),
            url: "u".into(),
            author: Some("alice".into()),
            created_at: dt("2026-04-01T00:00:00Z"),
            updated_at: now,
            is_draft: false,
            mergeable: Mergeable::Conflicting,
            labels: vec![],
            last_commit: None,
            reviews: vec![],
            threads: vec![],
            requested_reviewers: vec![],
        };
        let analyzed = analyze(vec![pr], &cfg, now);
        assert!(analyzed[0].needs_author_action);
        assert!(analyzed[0].has_merge_conflict);
    }

    #[test]
    fn date_arith() {
        let earlier = dt("2026-01-01T00:00:00Z");
        let later = dt("2026-01-11T00:00:00Z");
        assert!((days_between(earlier, later) - 10.0).abs() < 0.001);
        // Reversed inputs floor at zero.
        assert_eq!(days_between(later, earlier), 0.0);
    }

    #[test]
    fn grace_period_filters_new_authors() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let today = now.date_naive();
        // Alice opened her PR 3 days ago — within grace.
        // Bob opened his 30 days ago — past grace.
        let alice = analyze(
            vec![RawPr {
                number: 1,
                title: "x".into(),
                url: "u".into(),
                author: Some("alice".into()),
                created_at: now - chrono::Duration::days(3),
                updated_at: now,
                is_draft: false,
                mergeable: Mergeable::Mergeable,
                labels: vec![],
                last_commit: None,
                reviews: vec![],
                threads: vec![],
                requested_reviewers: vec![],
            }],
            &cfg,
            now,
        );
        let bob = analyze(
            vec![RawPr {
                number: 2,
                title: "x".into(),
                url: "u".into(),
                author: Some("bob".into()),
                created_at: now - chrono::Duration::days(30),
                updated_at: now,
                is_draft: false,
                mergeable: Mergeable::Mergeable,
                labels: vec![],
                last_commit: None,
                reviews: vec![],
                threads: vec![],
                requested_reviewers: vec![],
            }],
            &cfg,
            now,
        );
        let mut combined = alice;
        combined.extend(bob);
        let mut cache = HashMap::new();
        let filtered = apply_grace_period(combined, &mut cache, 14, today);
        let logins: Vec<_> = filtered
            .iter()
            .filter_map(|p| p.raw.author.clone())
            .collect();
        assert_eq!(logins, vec!["bob".to_string()]);
        assert!(cache.contains_key("alice"));
        assert!(cache.contains_key("bob"));
    }

    #[test]
    fn grace_period_uses_cache_when_pr_is_recent() {
        let cfg = Config::default();
        let now = dt("2026-05-19T00:00:00Z");
        let today = now.date_naive();
        // Carol just opened a new PR today, but the cache says we've known her since Jan.
        let prs = analyze(
            vec![RawPr {
                number: 3,
                title: "x".into(),
                url: "u".into(),
                author: Some("carol".into()),
                created_at: now,
                updated_at: now,
                is_draft: false,
                mergeable: Mergeable::Mergeable,
                labels: vec![],
                last_commit: None,
                reviews: vec![],
                threads: vec![],
                requested_reviewers: vec![],
            }],
            &cfg,
            now,
        );
        let mut cache = HashMap::from([(
            "carol".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        )]);
        let filtered = apply_grace_period(prs, &mut cache, 14, today);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn excerpt_truncates_and_strips_markdown() {
        assert_eq!(excerpt("## Header\nbody"), "Header");
        assert_eq!(excerpt("> quoted\nrest"), "quoted");
        let long: String = "x".repeat(200);
        let e = excerpt(&long);
        assert!(e.ends_with('…'));
        assert!(e.chars().count() <= 121);
    }
}
