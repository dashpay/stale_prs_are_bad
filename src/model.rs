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
}

#[derive(Debug, Clone)]
pub struct AuthorRollup {
    pub login: String,
    pub total_open_prs: u32,
    /// Open PRs with zero unresolved threads.
    pub clean_prs: u32,
    /// Open PRs with at least one unresolved thread. clean + dirty == total.
    pub dirty_prs: u32,
    pub prs_needing_author_action: u32,
    pub total_unresolved: u32,
    pub unresolved_coderabbit: u32,
    pub unresolved_human: u32,
    pub unresolved_bot: u32,
    pub total_score: f64,
    pub oldest_stale_pr_days: f64,
    /// Difference in `prs_needing_author_action` vs the snapshot from ~7 days ago.
    /// `None` when no comparable snapshot exists.
    pub delta_vs_last_week: Option<i32>,
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
