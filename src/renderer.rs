use chrono::{DateTime, Utc};
use std::fmt::Write;

use crate::model::{AnalyzedThread, AuthorRollup, ScoredPr, Severity};

pub struct RenderContext<'a> {
    pub now: DateTime<Utc>,
    pub commit_sha: Option<&'a str>,
    pub config_path: &'a str,
    pub has_history: bool,
}

pub fn render(scored: &[ScoredPr], authors: &[AuthorRollup], ctx: &RenderContext<'_>) -> String {
    let mut out = String::new();
    // Jekyll front matter so GitHub Pages applies the theme from docs/_config.yml.
    // Empty front matter is enough — `title:` lives in _config.yml.
    let _ = writeln!(out, "---");
    let _ = writeln!(out, "---");
    write_header(&mut out, ctx);
    write_summary(&mut out, scored);
    write_scoreboard(&mut out, authors, ctx.has_history);
    write_pr_sections(&mut out, scored, authors, ctx.now);
    write_methodology(&mut out, ctx);
    out
}

fn write_header(out: &mut String, ctx: &RenderContext<'_>) {
    let _ = writeln!(out, "# PR Hygiene Report");
    let stamp = ctx.now.format("%Y-%m-%d %H:%M UTC");
    match ctx.commit_sha {
        Some(sha) => {
            let short: String = sha.chars().take(7).collect();
            let _ = writeln!(out, "*Last updated: {stamp} · commit {short}*");
        }
        None => {
            let _ = writeln!(out, "*Last updated: {stamp}*");
        }
    }
    let _ = writeln!(out);
}

fn write_summary(out: &mut String, scored: &[ScoredPr]) {
    let open_prs = scored.len();
    let deferred: usize = scored.iter().filter(|s| s.pr.is_deferred).count();
    let draft: usize = scored
        .iter()
        .filter(|s| !s.pr.is_deferred && s.pr.raw.is_draft)
        .count();
    let dirty: usize = scored
        .iter()
        .filter(|s| !s.pr.is_deferred && !s.pr.raw.is_draft && s.unresolved_total > 0)
        .count();
    let clean: usize = open_prs - dirty - deferred - draft;
    let needs: usize = scored.iter().filter(|s| s.pr.needs_author_action).count();
    let unresolved: u32 = scored.iter().map(|s| s.unresolved_total).sum();
    let _ = writeln!(out, "## Summary");
    let _ = writeln!(
        out,
        "- Open PRs: **{open_prs}** ({clean} clean · {dirty} dirty · {deferred} deferred · {draft} draft)"
    );
    let _ = writeln!(out, "- PRs needing author action: **{needs}**");
    let _ = writeln!(out, "- Total unresolved threads: **{unresolved}**");
    let _ = writeln!(out);
}

fn write_scoreboard(out: &mut String, authors: &[AuthorRollup], has_history: bool) {
    if authors.is_empty() {
        return;
    }
    let _ = writeln!(out, "## Scoreboard");
    let _ = writeln!(
        out,
        "_Sort: dirty PRs desc → needs-action desc → to-review desc. \
         Clean players sink to the bottom; pure reviewers appear after authors._"
    );
    let _ = writeln!(out);
    let dash = |n: u32| {
        if n == 0 {
            "—".to_string()
        } else {
            n.to_string()
        }
    };
    if has_history {
        let _ = writeln!(
            out,
            "| Author | Open | Clean | Dirty | Deferred | Draft | Needs action | Unresolved | CR | Human | To review | Δ |"
        );
        let _ = writeln!(
            out,
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
    } else {
        let _ = writeln!(
            out,
            "| Author | Open | Clean | Dirty | Deferred | Draft | Needs action | Unresolved | CR | Human | To review |"
        );
        let _ = writeln!(
            out,
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
    }
    for a in authors {
        if has_history {
            let _ = writeln!(
                out,
                "| @{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                a.login,
                dash(a.total_open_prs),
                dash(a.clean_prs),
                dash(a.dirty_prs),
                dash(a.deferred_prs),
                dash(a.draft_prs),
                dash(a.prs_needing_author_action),
                dash(a.total_unresolved),
                dash(a.unresolved_coderabbit),
                dash(a.unresolved_human),
                dash(a.awaiting_review),
                format_delta(a.delta_vs_last_week),
            );
        } else {
            let _ = writeln!(
                out,
                "| @{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                a.login,
                dash(a.total_open_prs),
                dash(a.clean_prs),
                dash(a.dirty_prs),
                dash(a.deferred_prs),
                dash(a.draft_prs),
                dash(a.prs_needing_author_action),
                dash(a.total_unresolved),
                dash(a.unresolved_coderabbit),
                dash(a.unresolved_human),
                dash(a.awaiting_review),
            );
        }
    }
    let _ = writeln!(out);
}

fn format_delta(d: Option<i32>) -> String {
    match d {
        None => "—".into(),
        Some(0) => "—".into(),
        Some(n) if n > 0 => format!("↑ {n}"),
        Some(n) => format!("↓ {}", -n),
    }
}

fn write_pr_sections(
    out: &mut String,
    scored: &[ScoredPr],
    authors: &[AuthorRollup],
    now: DateTime<Utc>,
) {
    let _ = writeln!(out, "## PRs needing author action");
    let needs: Vec<&ScoredPr> = scored.iter().filter(|s| s.pr.needs_author_action).collect();
    if needs.is_empty() {
        let _ = writeln!(out, "_None._");
        let _ = writeln!(out);
        return;
    }
    let author_order: Vec<&str> = authors.iter().map(|a| a.login.as_str()).collect();
    for login in author_order {
        let mut for_author: Vec<&ScoredPr> = needs
            .iter()
            .copied()
            .filter(|s| s.pr.raw.author.as_deref() == Some(login))
            .collect();
        if for_author.is_empty() {
            continue;
        }
        for_author.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.unresolved_total.cmp(&a.unresolved_total))
                .then(a.pr.raw.number.cmp(&b.pr.raw.number))
        });
        let _ = writeln!(out, "### @{login}");
        for s in for_author {
            write_pr_bullet(out, s, now);
        }
        let _ = writeln!(out);
    }
    let mut orphans: Vec<&ScoredPr> = needs
        .iter()
        .copied()
        .filter(|s| s.pr.raw.author.is_none())
        .collect();
    if !orphans.is_empty() {
        orphans.sort_by_key(|s| s.pr.raw.number);
        let _ = writeln!(out, "### (no author)");
        for s in orphans {
            write_pr_bullet(out, s, now);
        }
        let _ = writeln!(out);
    }
}

fn write_pr_bullet(out: &mut String, s: &ScoredPr, now: DateTime<Utc>) {
    let title = sanitize_inline(&s.pr.raw.title);
    let url = &s.pr.raw.url;
    let n = s.pr.raw.number;
    let mut detail_bits: Vec<String> = vec![];
    detail_bits.push(format!("{} unresolved", s.unresolved_total));
    let breakdown = source_breakdown(s);
    if !breakdown.is_empty() {
        // Tack the breakdown onto the unresolved count: "12 unresolved (6 CodeRabbit, 6 human)".
        let last = detail_bits.last_mut().unwrap();
        let _ = write!(last, " ({})", breakdown);
    }
    detail_bits.push(format!(
        "{:.0} days stale",
        s.oldest_thread_age_days.max(0.0)
    ));
    if s.pr.has_merge_conflict {
        detail_bits.push("⚠ merge conflict".into());
    }
    if s.pr.changes_requested {
        detail_bits.push("✋ changes requested".into());
    }
    if s.pr.ci_failing {
        detail_bits.push("🔴 CI failing".into());
    }
    let _ = writeln!(out, "- [#{n} {title}]({url}) — {}", detail_bits.join(" · "));

    if let Some(top) = pick_top_thread(&s.pr.unresolved_threads) {
        let days = ((now - top.first_comment_at).num_seconds().max(0) as f64) / 86_400.0;
        let _ = writeln!(
            out,
            "  - Top thread: \"{}\" — {:.0} days old",
            sanitize_inline(&top.first_comment_excerpt),
            days
        );
    }
}

/// Pick the most prominent unresolved thread: highest severity, then oldest.
fn pick_top_thread(threads: &[AnalyzedThread]) -> Option<&AnalyzedThread> {
    threads.iter().max_by(|a, b| {
        severity_rank(a.severity)
            .cmp(&severity_rank(b.severity))
            .then_with(|| b.first_comment_at.cmp(&a.first_comment_at))
    })
}

fn severity_rank(s: Severity) -> u8 {
    match s {
        Severity::High => 3,
        Severity::Medium => 2,
        Severity::Low => 1,
    }
}

fn source_breakdown(s: &ScoredPr) -> String {
    let mut parts: Vec<String> = vec![];
    let b = &s.unresolved_by_source;
    if b.coderabbit > 0 {
        parts.push(format!("{} CodeRabbit", b.coderabbit));
    }
    if b.human > 0 {
        parts.push(format!("{} human", b.human));
    }
    if b.bot > 0 {
        parts.push(format!("{} bot", b.bot));
    }
    parts.join(", ")
}

fn write_methodology(out: &mut String, ctx: &RenderContext<'_>) {
    let _ = writeln!(out, "## Methodology");
    let _ = writeln!(
        out,
        "Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). \
         A thread counts as \"unresolved\" when it is open, not outdated, has a comment from \
         someone other than the PR author, and the most recent comment is from a reviewer. \
         **Dirty** = at least one such thread. \
         **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible \
         but not counted as dirty. \
         **Draft** = the PR is still marked draft on GitHub. \
         **Clean** = open, not draft, not deferred, no unresolved threads. \
         **Needs action** further requires changes-requested, merge conflict, or that the \
         reviewer commented more recently than the author last pushed. \
         **To review** counts clean, non-draft PRs (authored by someone else) where this person \
         is in the requested-reviewer list. \
         Configurable via [`{}`]({})\u{2014}edit defaults there.",
        ctx.config_path, ctx.config_path
    );
    if !ctx.has_history {
        let _ = writeln!(
            out,
            "\n_No history snapshot from last week was found, so week-over-week deltas are omitted this run._"
        );
    }
}

fn sanitize_inline(s: &str) -> String {
    // Strip newlines and escape '|' and ']' so markdown tables / links don't break.
    s.replace(['\n', '\r'], " ")
        .replace('|', "\\|")
        .replace(']', "\\]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        AnalyzedPr, AnalyzedThread, AuthorRollup, BySeverity, BySource, Mergeable, RawPr, ScoredPr,
        Severity, ThreadSource,
    };
    use chrono::TimeZone;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 5, 19, 6, 0, 0).unwrap()
    }

    fn ctx<'a>(has_history: bool) -> RenderContext<'a> {
        RenderContext {
            now: now(),
            commit_sha: Some("abc1234567"),
            config_path: ".pr-hygiene.yml",
            has_history,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rollup(
        login: &str,
        total: u32,
        clean: u32,
        dirty: u32,
        needs: u32,
        unresolved: u32,
        cr: u32,
        human: u32,
        score: f64,
        oldest: f64,
        delta: Option<i32>,
    ) -> AuthorRollup {
        AuthorRollup {
            login: login.into(),
            total_open_prs: total,
            clean_prs: clean,
            dirty_prs: dirty,
            deferred_prs: 0,
            draft_prs: 0,
            prs_needing_author_action: needs,
            total_unresolved: unresolved,
            unresolved_coderabbit: cr,
            unresolved_human: human,
            unresolved_bot: 0,
            awaiting_review: 0,
            total_score: score,
            oldest_stale_pr_days: oldest,
            delta_vs_last_week: delta,
        }
    }

    fn pr(
        number: u64,
        author: &str,
        title: &str,
        unresolved: Vec<AnalyzedThread>,
        needs_action: bool,
        ci_failing: bool,
    ) -> ScoredPr {
        let total = unresolved.len() as u32;
        let mut bs = BySeverity::default();
        let mut bsrc = BySource::default();
        let mut oldest = 0.0;
        for t in &unresolved {
            match t.severity {
                Severity::High => bs.high += 1,
                Severity::Medium => bs.medium += 1,
                Severity::Low => bs.low += 1,
            }
            match t.source {
                ThreadSource::Coderabbit => bsrc.coderabbit += 1,
                ThreadSource::Human => bsrc.human += 1,
                ThreadSource::Bot => bsrc.bot += 1,
            }
            let age = (now() - t.first_comment_at).num_seconds() as f64 / 86_400.0;
            if age > oldest {
                oldest = age;
            }
        }
        ScoredPr {
            pr: AnalyzedPr {
                raw: RawPr {
                    number,
                    title: title.into(),
                    url: format!("https://example.com/pr/{number}"),
                    author: Some(author.into()),
                    created_at: now() - chrono::Duration::days(30),
                    updated_at: now(),
                    is_draft: false,
                    mergeable: Mergeable::Mergeable,
                    labels: vec![],
                    last_commit: None,
                    reviews: vec![],
                    threads: vec![],
                    requested_reviewers: vec![],
                },
                unresolved_threads: unresolved,
                days_since_author_push: 1.0,
                days_since_last_reviewer_activity: 1.0,
                needs_author_action: needs_action,
                has_merge_conflict: false,
                changes_requested: false,
                ci_failing,
                is_deferred: false,
            },
            score: total as f64,
            oldest_thread_age_days: oldest,
            unresolved_by_severity: bs,
            unresolved_by_source: bsrc,
            unresolved_total: total,
        }
    }

    fn thread(
        severity: Severity,
        source: ThreadSource,
        age_days: i64,
        excerpt: &str,
    ) -> AnalyzedThread {
        let first = now() - chrono::Duration::days(age_days);
        AnalyzedThread {
            id: format!("t-{age_days}-{severity:?}"),
            source,
            severity,
            first_comment_at: first,
            last_comment_at: first,
            first_comment_excerpt: excerpt.into(),
        }
    }

    #[test]
    fn empty_report_renders() {
        let out = render(&[], &[], &ctx(true));
        assert!(out.contains("# PR Hygiene Report"));
        assert!(out.contains("Open PRs: **0**"));
        assert!(out.contains("Methodology"));
    }

    #[test]
    fn no_history_omits_delta_column() {
        let rollup_data = vec![rollup("alice", 1, 0, 1, 1, 1, 0, 1, 5.0, 3.0, None)];
        let scored = vec![pr(
            1,
            "alice",
            "do thing",
            vec![thread(Severity::High, ThreadSource::Human, 3, "fix it")],
            true,
            false,
        )];
        let out = render(&scored, &rollup_data, &ctx(false));
        assert!(!out.contains(" Δ "), "no Δ column header should appear");
        assert!(out.contains("No history snapshot from last week"));
    }

    #[test]
    fn scoreboard_includes_clean_player() {
        let rollup_data = vec![
            // Slacker on top.
            rollup("alice", 3, 0, 3, 2, 8, 5, 3, 24.0, 14.0, Some(2)),
            // Clean player at bottom.
            rollup("pasta", 4, 4, 0, 0, 0, 0, 0, 0.0, 0.0, None),
        ];
        let out = render(&[], &rollup_data, &ctx(true));
        assert!(out.contains("@alice"));
        assert!(out.contains("@pasta"));
        // Clean row shows "—" for oldest and score.
        let pasta_line = out
            .lines()
            .find(|l| l.contains("@pasta"))
            .expect("pasta row");
        assert!(pasta_line.contains("| — |"), "got: {pasta_line}");
        // Slacker row shows the delta.
        let alice_line = out.lines().find(|l| l.contains("@alice")).expect("alice");
        assert!(alice_line.contains("↑ 2"));
    }

    #[test]
    fn renders_pr_bullet_with_breakdown_and_ci() {
        let rollup_data = vec![rollup("alice", 1, 0, 1, 1, 3, 2, 1, 12.0, 18.0, Some(2))];
        let scored = vec![pr(
            1234,
            "alice",
            "Add foo support",
            vec![
                thread(
                    Severity::High,
                    ThreadSource::Human,
                    18,
                    "This should use error wrapping",
                ),
                thread(Severity::Medium, ThreadSource::Coderabbit, 5, "Refactor"),
                thread(Severity::Medium, ThreadSource::Coderabbit, 4, "Suggestion"),
            ],
            true,
            true,
        )];
        let out = render(&scored, &rollup_data, &ctx(true));
        assert!(out.contains("[#1234 Add foo support](https://example.com/pr/1234)"));
        assert!(out.contains("3 unresolved (2 CodeRabbit, 1 human)"));
        assert!(out.contains("CI failing"));
        assert!(out.contains("Top thread: \"This should use error wrapping\""));
        assert!(out.contains("↑ 2"));
    }

    #[test]
    fn sanitizes_pipe_in_titles() {
        let rollup_data = vec![rollup("alice", 1, 0, 1, 1, 1, 0, 1, 5.0, 1.0, None)];
        let scored = vec![pr(
            1,
            "alice",
            "feat: add | pipe in title",
            vec![thread(Severity::Medium, ThreadSource::Human, 1, "x")],
            true,
            false,
        )];
        let out = render(&scored, &rollup_data, &ctx(false));
        assert!(out.contains("add \\| pipe in title"));
    }
}
