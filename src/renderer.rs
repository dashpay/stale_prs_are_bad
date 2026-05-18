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
    write_summary(&mut out, scored, authors);
    write_top_offenders(&mut out, authors, ctx.has_history);
    write_pr_sections(&mut out, scored, authors, ctx.now);
    write_clean_prs(&mut out, scored);
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

fn write_summary(out: &mut String, scored: &[ScoredPr], authors: &[AuthorRollup]) {
    let open_prs = scored.len();
    let needs: usize = scored.iter().filter(|s| s.pr.needs_author_action).count();
    let unresolved: u32 = scored.iter().map(|s| s.unresolved_total).sum();
    let _ = writeln!(out, "## Summary");
    let _ = writeln!(out, "- Open PRs: {open_prs}");
    let _ = writeln!(out, "- PRs needing author action: {needs}");
    let _ = writeln!(out, "- Total unresolved threads: {unresolved}");

    let mut by_action: Vec<&AuthorRollup> = authors
        .iter()
        .filter(|a| a.prs_needing_author_action > 0)
        .collect();
    by_action.sort_by(|a, b| {
        b.prs_needing_author_action
            .cmp(&a.prs_needing_author_action)
            .then(a.login.cmp(&b.login))
    });
    if !by_action.is_empty() {
        let top: Vec<String> = by_action
            .iter()
            .take(3)
            .map(|a| format!("@{} ({} PRs)", a.login, a.prs_needing_author_action))
            .collect();
        let _ = writeln!(out, "- Top offenders this week: {}", top.join(", "));
    }
    let _ = writeln!(out);
}

fn write_top_offenders(out: &mut String, authors: &[AuthorRollup], has_history: bool) {
    let mut offenders: Vec<&AuthorRollup> = authors
        .iter()
        .filter(|a| a.total_unresolved > 0 || a.prs_needing_author_action > 0)
        .collect();
    if offenders.is_empty() {
        return;
    }
    offenders.sort_by(|a, b| {
        b.total_score
            .partial_cmp(&a.total_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(
                b.prs_needing_author_action
                    .cmp(&a.prs_needing_author_action),
            )
            .then(b.total_unresolved.cmp(&a.total_unresolved))
            .then(a.login.cmp(&b.login))
    });
    let _ = writeln!(out, "## 🚨 Top offenders");
    if has_history {
        let _ = writeln!(
            out,
            "| Author | Open PRs | Needs action | Unresolved | Oldest stale | Δ vs last week |"
        );
        let _ = writeln!(out, "|---|---|---|---|---|---|");
    } else {
        let _ = writeln!(
            out,
            "| Author | Open PRs | Needs action | Unresolved | Oldest stale |"
        );
        let _ = writeln!(out, "|---|---|---|---|---|");
    }
    for a in offenders.iter().take(15) {
        let oldest = format!("{}d", a.oldest_stale_pr_days.round() as i64);
        if has_history {
            let _ = writeln!(
                out,
                "| @{} | {} | {} | {} | {} | {} |",
                a.login,
                a.total_open_prs,
                a.prs_needing_author_action,
                a.total_unresolved,
                oldest,
                format_delta(a.delta_vs_last_week),
            );
        } else {
            let _ = writeln!(
                out,
                "| @{} | {} | {} | {} | {} |",
                a.login, a.total_open_prs, a.prs_needing_author_action, a.total_unresolved, oldest,
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

fn write_clean_prs(out: &mut String, scored: &[ScoredPr]) {
    let mut clean: Vec<&ScoredPr> = scored
        .iter()
        .filter(|s| !s.pr.raw.is_draft && s.unresolved_total == 0 && !s.pr.needs_author_action)
        .collect();
    if clean.is_empty() {
        return;
    }
    clean.sort_by_key(|s| s.pr.raw.number);
    let _ = writeln!(out, "## Clean PRs (no unresolved, awaiting review)");
    for s in clean.iter().take(20) {
        let title = sanitize_inline(&s.pr.raw.title);
        let _ = writeln!(
            out,
            "- [#{} {}]({}) — @{}",
            s.pr.raw.number,
            title,
            s.pr.raw.url,
            s.pr.raw.author.as_deref().unwrap_or("?")
        );
    }
    if clean.len() > 20 {
        let _ = writeln!(out, "- _…and {} more._", clean.len() - 20);
    }
    let _ = writeln!(out);
}

fn write_methodology(out: &mut String, ctx: &RenderContext<'_>) {
    let _ = writeln!(out, "## Methodology");
    let _ = writeln!(
        out,
        "Generated nightly by [pr-hygiene](https://github.com/dashpay/stale_prs_are_bad). \
         A thread counts as \"unresolved\" when it is open, not outdated, has a comment from \
         someone other than the PR author, and the most recent comment is from a reviewer. \
         Score weighs threads by severity and amplifies for staleness. \
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
                },
                unresolved_threads: unresolved,
                days_since_author_push: 1.0,
                days_since_last_reviewer_activity: 1.0,
                needs_author_action: needs_action,
                has_merge_conflict: false,
                changes_requested: false,
                ci_failing,
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
        assert!(out.contains("Open PRs: 0"));
        assert!(out.contains("Methodology"));
    }

    #[test]
    fn no_history_omits_delta_column() {
        let rollup = vec![AuthorRollup {
            login: "alice".into(),
            total_open_prs: 1,
            prs_needing_author_action: 1,
            total_unresolved: 1,
            total_score: 5.0,
            oldest_stale_pr_days: 3.0,
            delta_vs_last_week: None,
        }];
        let scored = vec![pr(
            1,
            "alice",
            "do thing",
            vec![thread(Severity::High, ThreadSource::Human, 3, "fix it")],
            true,
            false,
        )];
        let out = render(&scored, &rollup, &ctx(false));
        assert!(!out.contains("Δ vs last week"));
        assert!(out.contains("No history snapshot from last week"));
    }

    #[test]
    fn renders_pr_bullet_with_breakdown_and_ci() {
        let rollup = vec![AuthorRollup {
            login: "alice".into(),
            total_open_prs: 1,
            prs_needing_author_action: 1,
            total_unresolved: 3,
            total_score: 12.0,
            oldest_stale_pr_days: 18.0,
            delta_vs_last_week: Some(2),
        }];
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
        let out = render(&scored, &rollup, &ctx(true));
        assert!(out.contains("[#1234 Add foo support](https://example.com/pr/1234)"));
        assert!(out.contains("3 unresolved (2 CodeRabbit, 1 human)"));
        assert!(out.contains("CI failing"));
        assert!(out.contains("Top thread: \"This should use error wrapping\""));
        assert!(out.contains("↑ 2"));
    }

    #[test]
    fn sanitizes_pipe_in_titles() {
        let rollup = vec![AuthorRollup {
            login: "alice".into(),
            total_open_prs: 1,
            prs_needing_author_action: 1,
            total_unresolved: 1,
            total_score: 5.0,
            oldest_stale_pr_days: 1.0,
            delta_vs_last_week: None,
        }];
        let scored = vec![pr(
            1,
            "alice",
            "feat: add | pipe in title",
            vec![thread(Severity::Medium, ThreadSource::Human, 1, "x")],
            true,
            false,
        )];
        let out = render(&scored, &rollup, &ctx(false));
        assert!(out.contains("add \\| pipe in title"));
    }
}
