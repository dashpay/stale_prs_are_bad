use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RawPr {
    pub number: u64,
    pub title: String,
    pub url: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    /// Kept for ordering / future incremental fetch; not currently consumed.
    #[allow(dead_code)]
    pub updated_at: DateTime<Utc>,
    pub is_draft: bool,
    pub mergeable: Mergeable,
    pub labels: Vec<String>,
    pub last_commit: Option<LastCommit>,
    pub reviews: Vec<Review>,
    pub threads: Vec<RawThread>,
    /// Logins of users explicitly requested for review. Teams and Mannequins are skipped.
    pub requested_reviewers: Vec<String>,
    /// Base branch the PR is targeting (e.g. `master`, `v3.0`).
    pub base_ref: String,
    /// File paths changed by this PR. Capped by the GraphQL query page size.
    pub changed_files: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mergeable {
    Mergeable,
    Conflicting,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LastCommit {
    pub committed_date: DateTime<Utc>,
    pub status_state: Option<StatusState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusState {
    Success,
    Failure,
    Pending,
    Error,
    Expected,
}

#[derive(Debug, Clone)]
pub struct Review {
    pub state: ReviewState,
    pub author: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewState {
    Approved,
    ChangesRequested,
    Commented,
    Dismissed,
    Pending,
}

#[derive(Debug, Clone)]
pub struct RawThread {
    pub id: String,
    pub is_resolved: bool,
    pub is_outdated: bool,
    pub comments: Vec<RawComment>,
}

#[derive(Debug, Clone)]
pub struct RawComment {
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThreadSource {
    Coderabbit,
    Bot,
    Human,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct AnalyzedThread {
    pub id: String,
    pub source: ThreadSource,
    pub severity: Severity,
    pub first_comment_at: DateTime<Utc>,
    /// Tracked for future "last activity" sorting; not currently rendered.
    #[allow(dead_code)]
    pub last_comment_at: DateTime<Utc>,
    /// First line of the first comment, trimmed, used for "top thread" callout.
    pub first_comment_excerpt: String,
}

#[derive(Debug, Clone)]
pub struct AnalyzedPr {
    pub raw: RawPr,
    pub unresolved_threads: Vec<AnalyzedThread>,
    /// Spec-derived field. Drives `needs_author_action`; not surfaced separately.
    #[allow(dead_code)]
    pub days_since_author_push: f64,
    /// Spec-derived field. Drives `needs_author_action`; not surfaced separately.
    #[allow(dead_code)]
    pub days_since_last_reviewer_activity: f64,
    pub needs_author_action: bool,
    pub has_merge_conflict: bool,
    pub changes_requested: bool,
    pub ci_failing: bool,
    /// Set when the PR carries a configured `deferred_labels` tag. Deferred PRs
    /// are visible in the report (in their own bucket) but don't accrue
    /// unresolved-thread counts or needs-action signals.
    pub is_deferred: bool,
    /// True when the PR targets a non-default branch OR hasn't been touched in
    /// `stale_threshold_days`. Stale PRs land in their own bucket.
    pub is_stale: bool,
    /// Human-readable reason(s) for `is_stale` — surfaced as a badge in the PR detail.
    pub stale_reasons: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BySeverity {
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BySource {
    pub coderabbit: u32,
    pub bot: u32,
    pub human: u32,
}

#[derive(Debug, Clone)]
pub struct ScoredPr {
    pub pr: AnalyzedPr,
    pub score: f64,
    pub oldest_thread_age_days: f64,
    pub unresolved_by_severity: BySeverity,
    pub unresolved_by_source: BySource,
    pub unresolved_total: u32,
    /// Logins that are responsible for reviewing this PR via path-based routing
    /// rules, excluding anyone who has already submitted a review (their job is
    /// done). The list is dedupe-against-author.
    pub routed_reviewers: Vec<String>,
    /// True when at least one `review_routing` rule matched this PR. When true,
    /// the routed_reviewers list is the AUTHORITATIVE queue: explicit
    /// `requested_reviewers` from GitHub are ignored. (Empty routed_reviewers +
    /// routing_matched == true means "the owner already reviewed; PR is handled".)
    pub routing_matched: bool,
}

#[derive(Debug, Clone)]
pub struct AuthorRollup {
    pub login: String,
    /// PRs they authored (open). Includes clean + dirty + deferred.
    pub total_open_prs: u32,
    /// Open PRs they authored with zero unresolved threads, not deferred.
    pub clean_prs: u32,
    /// Open PRs they authored with at least one unresolved thread.
    pub dirty_prs: u32,
    /// Open PRs they authored that carry a deferred label (intentionally on hold).
    pub deferred_prs: u32,
    /// Open PRs they authored that are still marked draft.
    pub draft_prs: u32,
    /// Open PRs they authored that target a non-default branch or have rotted past the threshold.
    pub stale_prs: u32,
    /// Open PRs they authored with no unresolved threads but a failing CI run.
    /// Bucketed between Unresolved Comments and Clean — the comments are addressed
    /// but the build isn't green yet.
    pub ci_failing_prs: u32,
    /// Open PRs where a reviewer's most recent review is CHANGES_REQUESTED.
    /// Bucketed after Unresolved Comments — even with all threads resolved, the
    /// formal review state is still blocking until someone re-approves or dismisses.
    pub changes_requested_prs: u32,
    pub prs_needing_author_action: u32,
    pub total_unresolved: u32,
    pub unresolved_coderabbit: u32,
    pub unresolved_human: u32,
    pub unresolved_bot: u32,
    /// Clean PRs (authored by someone else) where this user is in the requested-reviewer list.
    /// "How many PRs are sitting in your review queue right now."
    pub awaiting_review: u32,
    pub total_score: f64,
    pub oldest_stale_pr_days: f64,
    /// Difference in `prs_needing_author_action` vs the snapshot from ~7 days ago.
    /// `None` when no comparable snapshot exists.
    pub delta_vs_last_week: Option<i32>,
    /// Alias rollups merged into this principal's row. Each alias keeps its own
    /// counts so the report can show "13+(5)". Nested aliases (alias of alias)
    /// aren't supported — this vec is always empty on the inner rollups.
    pub aliases: Vec<AuthorRollup>,
}

impl AuthorRollup {
    fn sum_aliases<F: Fn(&AuthorRollup) -> u32>(&self, f: F) -> u32 {
        self.aliases.iter().map(f).sum()
    }
    fn sum_aliases_f64<F: Fn(&AuthorRollup) -> f64>(&self, f: F) -> f64 {
        self.aliases.iter().map(f).sum()
    }

    pub fn combined_total_open_prs(&self) -> u32 {
        self.total_open_prs + self.sum_aliases(|a| a.total_open_prs)
    }
    pub fn combined_clean_prs(&self) -> u32 {
        self.clean_prs + self.sum_aliases(|a| a.clean_prs)
    }
    pub fn combined_dirty_prs(&self) -> u32 {
        self.dirty_prs + self.sum_aliases(|a| a.dirty_prs)
    }
    pub fn combined_deferred_prs(&self) -> u32 {
        self.deferred_prs + self.sum_aliases(|a| a.deferred_prs)
    }
    pub fn combined_draft_prs(&self) -> u32 {
        self.draft_prs + self.sum_aliases(|a| a.draft_prs)
    }
    pub fn combined_stale_prs(&self) -> u32 {
        self.stale_prs + self.sum_aliases(|a| a.stale_prs)
    }
    pub fn combined_ci_failing_prs(&self) -> u32 {
        self.ci_failing_prs + self.sum_aliases(|a| a.ci_failing_prs)
    }
    pub fn combined_changes_requested_prs(&self) -> u32 {
        self.changes_requested_prs + self.sum_aliases(|a| a.changes_requested_prs)
    }
    pub fn combined_prs_needing_author_action(&self) -> u32 {
        self.prs_needing_author_action + self.sum_aliases(|a| a.prs_needing_author_action)
    }
    pub fn combined_total_unresolved(&self) -> u32 {
        self.total_unresolved + self.sum_aliases(|a| a.total_unresolved)
    }
    pub fn combined_unresolved_coderabbit(&self) -> u32 {
        self.unresolved_coderabbit + self.sum_aliases(|a| a.unresolved_coderabbit)
    }
    pub fn combined_unresolved_human(&self) -> u32 {
        self.unresolved_human + self.sum_aliases(|a| a.unresolved_human)
    }
    pub fn combined_awaiting_review(&self) -> u32 {
        self.awaiting_review + self.sum_aliases(|a| a.awaiting_review)
    }
    pub fn combined_total_score(&self) -> f64 {
        self.total_score + self.sum_aliases_f64(|a| a.total_score)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub date: NaiveDate,
    pub summary: Summary,
    pub per_author: Vec<AuthorSnapshot>,
    pub per_pr: Vec<PrSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub open_prs: u32,
    pub prs_needing_author_action: u32,
    pub total_unresolved: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorSnapshot {
    pub login: String,
    pub total_open_prs: u32,
    #[serde(default)]
    pub clean_prs: u32,
    #[serde(default)]
    pub dirty_prs: u32,
    #[serde(default)]
    pub deferred_prs: u32,
    #[serde(default)]
    pub draft_prs: u32,
    #[serde(default)]
    pub stale_prs: u32,
    #[serde(default)]
    pub ci_failing_prs: u32,
    #[serde(default)]
    pub changes_requested_prs: u32,
    #[serde(default)]
    pub awaiting_review: u32,
    pub prs_needing_author_action: u32,
    pub total_unresolved: u32,
    pub total_score: f64,
    pub oldest_stale_pr_days: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrSnapshot {
    pub number: u64,
    pub author: Option<String>,
    pub score: f64,
    pub unresolved_total: u32,
    pub by_severity: BySeverity,
    pub by_source: BySource,
    pub needs_author_action: bool,
}
