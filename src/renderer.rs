use chrono::{DateTime, Utc};
use std::collections::HashSet;
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
    write_author_drilldowns(&mut out, scored, authors, ctx.now);
    write_methodology(&mut out, ctx);
    out
}

/// Slugify a GitHub login into a stable, anchor-safe id.
fn slug(login: &str) -> String {
    login
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

/// Format the Author column cell. When the rollup has aliases, show
/// `[@principal + (@alias) + (@alias2)](#principal-slug)`.
fn author_cell(a: &AuthorRollup) -> String {
    let url = format!("#{}", slug(&a.login));
    let mut display = format!("@{}", a.login);
    for alias in &a.aliases {
        display.push_str(&format!(" + (@{})", alias.login));
    }
    format!("[{display}]({url})")
}

/// Render an integer cell. When the rollup has aliases, format as
/// `principal+(alias)+(alias2)`; otherwise just the principal count.
/// Always links to the principal's drilldown subsection.
fn cell_link(
    principal: u32,
    aliases: &[AuthorRollup],
    project: impl Fn(&AuthorRollup) -> u32,
    login: &str,
    bucket: &str,
) -> String {
    let alias_counts: Vec<u32> = aliases.iter().map(&project).collect();
    let combined: u32 = principal + alias_counts.iter().sum::<u32>();
    if combined == 0 {
        return "—".to_string();
    }
    let url = format!("#{}-{}", slug(login), bucket);
    if aliases.is_empty() {
        return format!("[{principal}]({url})");
    }
    // Compact display with explicit numbers so the breakdown is readable.
    let mut parts = vec![principal.to_string()];
    for n in alias_counts {
        parts.push(format!("({n})"));
    }
    format!("[{}]({})", parts.join("+"), url)
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
    let stale: usize = scored
        .iter()
        .filter(|s| !s.pr.is_deferred && s.pr.is_stale)
        .count();
    let draft: usize = scored
        .iter()
        .filter(|s| !s.pr.is_deferred && !s.pr.is_stale && s.pr.raw.is_draft)
        .count();
    let unresolved_comments: usize = scored
        .iter()
        .filter(|s| {
            !s.pr.is_deferred && !s.pr.is_stale && !s.pr.raw.is_draft && s.unresolved_total > 0
        })
        .count();
    let changes_requested: usize = scored
        .iter()
        .filter(|s| {
            !s.pr.is_deferred
                && !s.pr.is_stale
                && !s.pr.raw.is_draft
                && s.unresolved_total == 0
                && s.pr.changes_requested
        })
        .count();
    let ci_failing: usize = scored
        .iter()
        .filter(|s| {
            !s.pr.is_deferred
                && !s.pr.is_stale
                && !s.pr.raw.is_draft
                && s.unresolved_total == 0
                && !s.pr.changes_requested
                && s.pr.ci_failing
        })
        .count();
    let clean: usize =
        open_prs - unresolved_comments - changes_requested - ci_failing - deferred - draft - stale;
    let needs: usize = scored.iter().filter(|s| s.pr.needs_author_action).count();
    let unresolved: u32 = scored.iter().map(|s| s.unresolved_total).sum();
    let _ = writeln!(out, "## Summary");
    let _ = writeln!(
        out,
        "- Open PRs: **{open_prs}** ({clean} clean · {ci_failing} CI failing · {changes_requested} changes requested · {unresolved_comments} unresolved comments · {deferred} deferred · {draft} draft · {stale} stale)"
    );
    let _ = writeln!(out, "- PRs needing author action: **{needs}**");
    let _ = writeln!(out, "- Total unresolved comments: **{unresolved}**");
    let _ = writeln!(out);
}

fn write_scoreboard(out: &mut String, authors: &[AuthorRollup], has_history: bool) {
    if authors.is_empty() {
        return;
    }
    let _ = writeln!(out, "## Scoreboard");
    let _ = writeln!(
        out,
        "_Sort: unresolved-comments desc → needs-action desc → ready-for-review desc. \
         Click any number to jump to the specific PRs it covers._"
    );
    let _ = writeln!(out);
    if has_history {
        let _ = writeln!(
            out,
            "| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review | Δ |"
        );
        let _ = writeln!(
            out,
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
    } else {
        let _ = writeln!(
            out,
            "| Author | Open | Clean | CI failing | Unresolved Comments | Changes Requested | Deferred | Draft | Stale | Needs action | Total Unresolved Comments | Ready for Review |"
        );
        let _ = writeln!(
            out,
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
    }
    for a in authors {
        let author_link = author_cell(a);
        let login = &a.login;
        let aliases = &a.aliases;
        // Total Unresolved Comments lives on the Unresolved Comments bucket PRs,
        // so that's the natural drill target.
        let total_unresolved_target = cell_link(
            a.total_unresolved,
            aliases,
            |x| x.total_unresolved,
            login,
            "unresolved-comments",
        );
        if has_history {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                author_link,
                cell_link(
                    a.total_open_prs,
                    aliases,
                    |x| x.total_open_prs,
                    login,
                    "open"
                ),
                cell_link(a.clean_prs, aliases, |x| x.clean_prs, login, "clean"),
                cell_link(
                    a.ci_failing_prs,
                    aliases,
                    |x| x.ci_failing_prs,
                    login,
                    "ci-failing"
                ),
                cell_link(
                    a.dirty_prs,
                    aliases,
                    |x| x.dirty_prs,
                    login,
                    "unresolved-comments"
                ),
                cell_link(
                    a.changes_requested_prs,
                    aliases,
                    |x| x.changes_requested_prs,
                    login,
                    "changes-requested"
                ),
                cell_link(
                    a.deferred_prs,
                    aliases,
                    |x| x.deferred_prs,
                    login,
                    "deferred"
                ),
                cell_link(a.draft_prs, aliases, |x| x.draft_prs, login, "draft"),
                cell_link(a.stale_prs, aliases, |x| x.stale_prs, login, "stale"),
                cell_link(
                    a.prs_needing_author_action,
                    aliases,
                    |x| x.prs_needing_author_action,
                    login,
                    "needs-action"
                ),
                total_unresolved_target,
                cell_link(
                    a.awaiting_review,
                    aliases,
                    |x| x.awaiting_review,
                    login,
                    "ready-for-review"
                ),
                format_delta(a.delta_vs_last_week),
            );
        } else {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                author_link,
                cell_link(
                    a.total_open_prs,
                    aliases,
                    |x| x.total_open_prs,
                    login,
                    "open"
                ),
                cell_link(a.clean_prs, aliases, |x| x.clean_prs, login, "clean"),
                cell_link(
                    a.ci_failing_prs,
                    aliases,
                    |x| x.ci_failing_prs,
                    login,
                    "ci-failing"
                ),
                cell_link(
                    a.dirty_prs,
                    aliases,
                    |x| x.dirty_prs,
                    login,
                    "unresolved-comments"
                ),
                cell_link(
                    a.changes_requested_prs,
                    aliases,
                    |x| x.changes_requested_prs,
                    login,
                    "changes-requested"
                ),
                cell_link(
                    a.deferred_prs,
                    aliases,
                    |x| x.deferred_prs,
                    login,
                    "deferred"
                ),
                cell_link(a.draft_prs, aliases, |x| x.draft_prs, login, "draft"),
                cell_link(a.stale_prs, aliases, |x| x.stale_prs, login, "stale"),
                cell_link(
                    a.prs_needing_author_action,
                    aliases,
                    |x| x.prs_needing_author_action,
                    login,
                    "needs-action"
                ),
                total_unresolved_target,
                cell_link(
                    a.awaiting_review,
                    aliases,
                    |x| x.awaiting_review,
                    login,
                    "ready-for-review"
                ),
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

/// For each author in the scoreboard, emit a top-level `## @login` section
/// containing one subsection per non-empty bucket. Subsections are anchored so
/// the scoreboard table cells link directly to them.
fn write_author_drilldowns(
    out: &mut String,
    scored: &[ScoredPr],
    authors: &[AuthorRollup],
    now: DateTime<Utc>,
) {
    if authors.is_empty() {
        return;
    }
    let _ = writeln!(out, "## Per-author detail");
    let _ = writeln!(out);
    for a in authors {
        write_author_section(out, scored, a, now);
    }
}

fn write_author_section(
    out: &mut String,
    scored: &[ScoredPr],
    rollup: &AuthorRollup,
    now: DateTime<Utc>,
) {
    let login = &rollup.login;
    let s = slug(login);
    let _ = writeln!(out, "<a id=\"{s}\"></a>");
    let header = if rollup.aliases.is_empty() {
        format!("### @{login}")
    } else {
        let mut h = format!("### @{login}");
        for alias in &rollup.aliases {
            h.push_str(&format!(" + (@{})", alias.login));
        }
        h
    };
    let _ = writeln!(out, "{header}");

    // All logins owned by this principal (own login + every alias's login),
    // lowercased for case-insensitive matching against PR authors.
    let owned_logins: HashSet<String> = std::iter::once(login.to_ascii_lowercase())
        .chain(rollup.aliases.iter().map(|a| a.login.to_ascii_lowercase()))
        .collect();

    let mine: Vec<&ScoredPr> = scored
        .iter()
        .filter(|p| {
            p.pr.raw
                .author
                .as_deref()
                .map(|a| owned_logins.contains(&a.to_ascii_lowercase()))
                .unwrap_or(false)
        })
        .collect();

    if mine.is_empty() && rollup.combined_awaiting_review() == 0 {
        // Pure reviewer with no queue right now. Just emit the anchor + an empty notice.
        let _ = writeln!(out, "_No PRs._");
        let _ = writeln!(out);
        return;
    }

    let mut by_bucket: Vec<(&str, &str, Vec<&ScoredPr>)> = Vec::new();
    let collect = |pred: &dyn Fn(&&&ScoredPr) -> bool| -> Vec<&ScoredPr> {
        let mut v: Vec<&ScoredPr> = mine.iter().filter(pred).copied().collect();
        v.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(b.unresolved_total.cmp(&a.unresolved_total))
                .then(a.pr.raw.number.cmp(&b.pr.raw.number))
        });
        v
    };

    by_bucket.push(("Open", "open", collect(&|_| true)));
    by_bucket.push((
        "Needs action",
        "needs-action",
        collect(&|p| p.pr.needs_author_action),
    ));
    by_bucket.push((
        "Unresolved Comments",
        "unresolved-comments",
        collect(&|p| {
            !p.pr.is_deferred && !p.pr.is_stale && !p.pr.raw.is_draft && p.unresolved_total > 0
        }),
    ));
    by_bucket.push((
        "Changes Requested",
        "changes-requested",
        collect(&|p| {
            !p.pr.is_deferred
                && !p.pr.is_stale
                && !p.pr.raw.is_draft
                && p.unresolved_total == 0
                && p.pr.changes_requested
        }),
    ));
    by_bucket.push((
        "CI Failing",
        "ci-failing",
        collect(&|p| {
            !p.pr.is_deferred
                && !p.pr.is_stale
                && !p.pr.raw.is_draft
                && p.unresolved_total == 0
                && !p.pr.changes_requested
                && p.pr.ci_failing
        }),
    ));
    by_bucket.push(("Deferred", "deferred", collect(&|p| p.pr.is_deferred)));
    by_bucket.push((
        "Draft",
        "draft",
        collect(&|p| !p.pr.is_deferred && !p.pr.is_stale && p.pr.raw.is_draft),
    ));
    by_bucket.push((
        "Stale",
        "stale",
        collect(&|p| !p.pr.is_deferred && p.pr.is_stale),
    ));
    by_bucket.push((
        "Clean",
        "clean",
        collect(&|p| {
            !p.pr.is_deferred
                && !p.pr.is_stale
                && !p.pr.raw.is_draft
                && p.unresolved_total == 0
                && !p.pr.changes_requested
                && !p.pr.ci_failing
        }),
    ));

    // Ready-for-Review is authored by someone else; pull from scored at large.
    // Mirrors the rollup logic: when path routing matches, the routed reviewers
    // are THE queue (explicit reviewers are ignored); otherwise explicit
    // requested-reviewers apply. CHANGES_REQUESTED and red CI disqualify the PR.
    let reviewer_logins: HashSet<String> = owned_logins.clone();
    let mut to_review: Vec<&ScoredPr> = scored
        .iter()
        .filter(|p| {
            if p.pr.is_deferred
                || p.pr.is_stale
                || p.pr.raw.is_draft
                || p.unresolved_total > 0
                || p.pr.has_merge_conflict
                || p.pr.ci_failing
                || p.pr.changes_requested
            {
                return false;
            }
            if p.pr
                .raw
                .author
                .as_deref()
                .map(|a| reviewer_logins.contains(&a.to_ascii_lowercase()))
                .unwrap_or(false)
            {
                return false;
            }
            if p.routing_matched {
                p.routed_reviewers
                    .iter()
                    .any(|r| reviewer_logins.contains(&r.to_ascii_lowercase()))
            } else {
                p.pr.raw
                    .requested_reviewers
                    .iter()
                    .any(|r| reviewer_logins.contains(&r.to_ascii_lowercase()))
            }
        })
        .collect();
    to_review.sort_by_key(|p| p.pr.raw.number);
    by_bucket.push(("Ready for Review", "ready-for-review", to_review));

    for (label, anchor, prs) in by_bucket {
        if prs.is_empty() {
            continue;
        }
        let _ = writeln!(out, "<a id=\"{s}-{anchor}\"></a>");
        let _ = writeln!(out, "#### {label} ({})", prs.len());
        let show_author = anchor == "ready-for-review";
        let principal = if show_author {
            None
        } else {
            Some(login.as_str())
        };
        for p in prs {
            write_pr_bullet(out, p, now, show_author, principal);
        }
        let _ = writeln!(out);
    }
}

fn write_pr_bullet(
    out: &mut String,
    s: &ScoredPr,
    now: DateTime<Utc>,
    show_author: bool,
    principal_login: Option<&str>,
) {
    let title = sanitize_inline(&s.pr.raw.title);
    let url = &s.pr.raw.url;
    let n = s.pr.raw.number;
    let mut detail_bits: Vec<String> = vec![];
    if show_author {
        if let Some(a) = &s.pr.raw.author {
            detail_bits.push(format!("by @{a}"));
        }
    } else if let (Some(principal), Some(a)) = (principal_login, s.pr.raw.author.as_deref()) {
        // In the principal's own drilldown section, PRs authored by an alias get
        // the "via @alias" attribution so the reader knows the original account.
        if !a.eq_ignore_ascii_case(principal) {
            detail_bits.push(format!("via @{a}"));
        }
    }
    if s.unresolved_total > 0 {
        let mut chunk = format!("{} unresolved", s.unresolved_total);
        let breakdown = source_breakdown(s);
        if !breakdown.is_empty() {
            let _ = write!(chunk, " ({})", breakdown);
        }
        detail_bits.push(chunk);
        detail_bits.push(format!(
            "{:.0} days stale",
            s.oldest_thread_age_days.max(0.0)
        ));
    }
    if s.pr.has_merge_conflict {
        detail_bits.push("⚠ merge conflict".into());
    }
    if s.pr.changes_requested {
        detail_bits.push("✋ changes requested".into());
    }
    if s.pr.ci_failing {
        detail_bits.push("🔴 CI failing".into());
    }
    if s.pr.raw.is_draft {
        detail_bits.push("📝 draft".into());
    }
    if s.pr.is_deferred {
        detail_bits.push("⏸ deferred".into());
    }
    if s.pr.is_stale {
        if s.pr.stale_reasons.is_empty() {
            detail_bits.push("🐢 stale".into());
        } else {
            detail_bits.push(format!("🐢 {}", s.pr.stale_reasons.join(", ")));
        }
    }
    let suffix = if detail_bits.is_empty() {
        String::new()
    } else {
        format!(" — {}", detail_bits.join(" · "))
    };
    let _ = writeln!(out, "- [#{n} {title}]({url}){suffix}");

    if s.unresolved_total > 0 {
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
         **Unresolved Comments** = at least one such thread. \
         **Changes Requested** = no unresolved threads but a reviewer's most recent review \
         is CHANGES_REQUESTED (still blocking until someone re-approves or dismisses). \
         **Deferred** = carries a configured deferred label (e.g. `postponed`) — visible \
         but not counted toward unresolved-comment counts. \
         **Stale** = targets a non-default branch OR hasn't been touched in \
         the configured threshold (default 120 days, but clean PRs are never reclassified as stale). \
         **Draft** = the PR is still marked draft on GitHub. \
         **CI failing** = no unresolved comments, no changes-requested, but the latest commit's status check is failing. \
         **Clean** = open, not draft, not deferred, not stale, no unresolved comments, no changes-requested, CI green. \
         **Needs action** further requires changes-requested, merge conflict, or that the \
         reviewer commented more recently than the author last pushed. \
         **Ready for Review** counts clean PRs (authored by someone else) where this person \
         owes a review. When a `review_routing` rule matches a PR's changed files, the routed \
         reviewer IS the queue (explicit GitHub reviewers are ignored); a routed reviewer who \
         has already submitted any review is excluded — their job is done. \
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
            stale_prs: 0,
            ci_failing_prs: 0,
            changes_requested_prs: 0,
            prs_needing_author_action: needs,
            total_unresolved: unresolved,
            unresolved_coderabbit: cr,
            unresolved_human: human,
            unresolved_bot: 0,
            awaiting_review: 0,
            total_score: score,
            oldest_stale_pr_days: oldest,
            delta_vs_last_week: delta,
            aliases: vec![],
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
                    base_ref: "master".into(),
                    changed_files: vec![],
                },
                unresolved_threads: unresolved,
                days_since_author_push: 1.0,
                days_since_last_reviewer_activity: 1.0,
                needs_author_action: needs_action,
                has_merge_conflict: false,
                changes_requested: false,
                ci_failing,
                is_deferred: false,
                is_stale: false,
                stale_reasons: vec![],
            },
            score: total as f64,
            oldest_thread_age_days: oldest,
            unresolved_by_severity: bs,
            unresolved_by_source: bsrc,
            unresolved_total: total,
            routed_reviewers: vec![],
            routing_matched: false,
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
    fn scoreboard_cells_link_to_drilldown_anchors() {
        let rollup_data = vec![rollup("Alice-123", 2, 0, 2, 1, 2, 1, 1, 10.0, 9.0, None)];
        let scored = vec![
            pr(
                42,
                "Alice-123",
                "feature",
                vec![thread(Severity::High, ThreadSource::Human, 9, "fix this")],
                true,
                false,
            ),
            pr(
                43,
                "Alice-123",
                "another",
                vec![thread(Severity::Medium, ThreadSource::Coderabbit, 2, "y")],
                false,
                false,
            ),
        ];
        let out = render(&scored, &rollup_data, &ctx(false));
        // Login is slugified to lowercase for anchors.
        assert!(
            out.contains("[2](#alice-123-open)"),
            "open cell should link"
        );
        assert!(
            out.contains("[2](#alice-123-unresolved-comments)"),
            "unresolved-comments cell should link"
        );
        assert!(
            out.contains("[1](#alice-123-needs-action)"),
            "needs-action cell should link"
        );
        // Zero counts are dashes, no links.
        let row = out
            .lines()
            .find(|l| l.contains("[@Alice-123]"))
            .expect("row");
        assert!(
            row.contains("| — |"),
            "zero-count cells should be dashes\nrow: {row}"
        );
        // Drilldown section exists with the slugified anchor and bucket subsections.
        assert!(out.contains("<a id=\"alice-123\"></a>"));
        assert!(out.contains("<a id=\"alice-123-unresolved-comments\"></a>"));
        assert!(out.contains("#### Unresolved Comments (2)"));
        // Author name in the table is also clickable to their section.
        assert!(out.contains("[@Alice-123](#alice-123)"));
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
