use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;

use crate::model::{
    LastCommit, Mergeable, RawComment, RawPr, RawThread, Review, ReviewState, StatusState,
};
use crate::stages::{CommentHistory, Evidence, PrEvent, Revision, ENGINE_LOGIN};

const GITHUB_API: &str = "https://api.github.com/graphql";
const USER_AGENT_STR: &str = concat!("pr-hygiene/", env!("CARGO_PKG_VERSION"));
const THREADS_PER_PAGE: u32 = 100;
const COMMENTS_PER_THREAD: u32 = 50;
const MAX_RETRIES: u32 = 5;
const FILES_PER_PAGE: u32 = 100;
/// GitHub stops listing a PR's files at this many; beyond it the list is
/// genuinely incomplete.
const MAX_CHANGED_FILES: usize = 3000;

const PR_LIST_QUERY: &str = r#"
query PrHygieneList($owner: String!, $name: String!, $cursor: String, $threads: Int!, $comments: Int!) {
  rateLimit { remaining resetAt cost }
  repository(owner: $owner, name: $name) {
    defaultBranchRef { name }
    pullRequests(states: OPEN, first: 50, after: $cursor,
                 orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo { endCursor hasNextPage }
      nodes {
        id number title url isDraft mergeable createdAt updatedAt
        baseRefName
        files(first: 50) { totalCount pageInfo { endCursor hasNextPage } nodes { path } }
        author { login __typename }
        labels(first: 20) { nodes { name } }
        commits(last: 1) {
          nodes { commit {
            committedDate
            statusCheckRollup { state }
          } }
        }
        reviews(first: 50) {
          nodes { state author { login __typename } submittedAt }
        }
        reviewRequests(first: 20) {
          nodes {
            requestedReviewer {
              __typename
              ... on User { login }
            }
          }
        }
        reviewThreads(first: $threads) {
          pageInfo { endCursor hasNextPage }
          nodes {
            id isResolved isOutdated
            comments(first: $comments) {
              pageInfo { hasNextPage }
              nodes { author { login __typename } createdAt body }
            }
          }
        }
      }
    }
  }
}
"#;

const PR_THREADS_QUERY: &str = r#"
query PrHygieneThreads($id: ID!, $cursor: String, $comments: Int!) {
  rateLimit { remaining resetAt cost }
  node(id: $id) {
    ... on PullRequest {
      reviewThreads(first: 100, after: $cursor) {
        pageInfo { endCursor hasNextPage }
        nodes {
          id isResolved isOutdated
          comments(first: $comments) {
            pageInfo { hasNextPage }
            nodes { author { login __typename } createdAt body }
          }
        }
      }
    }
  }
}
"#;

const PR_FILES_QUERY: &str = r#"
query PrHygieneFiles($id: ID!, $cursor: String, $files: Int!) {
  rateLimit { remaining resetAt cost }
  node(id: $id) {
    ... on PullRequest {
      files(first: $files, after: $cursor) {
        pageInfo { endCursor hasNextPage }
        nodes { path }
      }
    }
  }
}
"#;

const PR_MERGEABLE_QUERY: &str = r#"
query PrHygieneMergeable($id: ID!) {
  node(id: $id) { ... on PullRequest { mergeable } }
}
"#;

/// When each PR changed draft state or base, and which comments the engine
/// account wrote — not what they say, which only the engine's few need. Kept
/// out of the PR list query, the heaviest one; this costs one point a page.
const STAGE_EVIDENCE_QUERY: &str = r#"
query PrHygieneStageEvidence($ids: [ID!]!) {
  rateLimit { remaining resetAt cost }
  nodes(ids: $ids) {
    ... on PullRequest {
      number createdAt isDraft baseRefName
      timelineItems(last: 100, itemTypes: [CONVERT_TO_DRAFT_EVENT, READY_FOR_REVIEW_EVENT,
                    BASE_REF_CHANGED_EVENT, AUTOMATIC_BASE_CHANGE_SUCCEEDED_EVENT,
                    REOPENED_EVENT]) {
        pageInfo { hasPreviousPage }
        nodes {
          __typename
          ... on ConvertToDraftEvent { createdAt }
          ... on ReadyForReviewEvent { createdAt }
          ... on BaseRefChangedEvent { createdAt previousRefName }
          ... on AutomaticBaseChangeSucceededEvent { createdAt oldBase }
          ... on ReopenedEvent { createdAt }
        }
      }
      comments(last: 100) {
        pageInfo { hasPreviousPage }
        nodes { id createdAt author { login __typename } }
      }
    }
  }
}
"#;

/// Every revision of the engine account's comments, newest first. GitHub lists
/// a comment's original among its revisions once it has been edited.
const COMMENT_HISTORY_QUERY: &str = r#"
query PrHygieneCommentHistory($ids: [ID!]!) {
  rateLimit { remaining resetAt cost }
  nodes(ids: $ids) {
    ... on IssueComment {
      createdAt body author { login __typename }
      userContentEdits(first: 100) {
        pageInfo { hasNextPage }
        nodes { editedAt diff editor { login __typename } }
      }
    }
  }
}
"#;

/// PRs per stage-evidence request.
const EVIDENCE_PRS_PER_REQUEST: usize = 50;
/// Comments per comment-history request: each can carry up to 100 revisions.
const HISTORIES_PER_REQUEST: usize = 50;

pub struct Fetcher {
    client: Client,
    endpoint: String,
}

impl Fetcher {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("token contains invalid header characters")?,
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(60))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            client,
            endpoint: GITHUB_API.to_string(),
        })
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Fetch every open PR in `owner/name`, fully populated.
    /// Returns the PRs, (node_id, number) pairs for follow-up queries, and the
    /// repository's default branch name (used to drive stale-branch detection).
    pub async fn fetch_all_open_prs(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<(Vec<RawPr>, Vec<(String, u64)>, Option<String>)> {
        let mut out: Vec<RawPr> = Vec::new();
        let mut node_ids: Vec<(String, u64)> = Vec::new();
        let mut paginated_threads: Vec<(String, u64)> = Vec::new();
        let mut paginated_files: Vec<(String, u64, Option<String>)> = Vec::new();
        let mut cursor: Option<String> = None;
        let mut default_branch: Option<String> = None;
        let repo = format!("{owner}/{name}");
        loop {
            let vars = json!({
                "owner": owner,
                "name": name,
                "cursor": cursor,
                "threads": THREADS_PER_PAGE,
                "comments": COMMENTS_PER_THREAD,
            });
            let resp = self.execute(PR_LIST_QUERY, vars).await?;
            if default_branch.is_none() {
                default_branch = resp
                    .pointer("/data/repository/defaultBranchRef/name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
            let pr_conn = resp
                .pointer("/data/repository/pullRequests")
                .ok_or_else(|| anyhow!("response missing data.repository.pullRequests"))?;
            let nodes = pr_conn
                .get("nodes")
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow!("pullRequests.nodes missing"))?;
            for node in nodes {
                let parsed = parse_pr_node(node, &repo).context("parsing PR node")?;
                node_ids.push((parsed.node_id.clone(), parsed.pr.number));
                if parsed.threads_have_more {
                    paginated_threads.push((parsed.node_id.clone(), parsed.pr.number));
                }
                if parsed.files_have_more {
                    paginated_files.push((parsed.node_id, parsed.pr.number, parsed.files_cursor));
                }
                out.push(parsed.pr);
            }
            let page_info = pr_conn
                .get("pageInfo")
                .ok_or_else(|| anyhow!("pullRequests.pageInfo missing"))?;
            let has_next = page_info
                .get("hasNextPage")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !has_next {
                break;
            }
            cursor = page_info
                .get("endCursor")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
        }

        // Fill in extra thread pages for any PR that paginated.
        for (id, number) in paginated_threads {
            tracing::debug!(pr = number, "fetching extra review-thread pages");
            let extra = self.fetch_extra_threads(&id).await?;
            if let Some(pr) = out.iter_mut().find(|p| p.number == number) {
                pr.threads.extend(extra);
            }
        }

        // Fill in extra changed-file pages. A page that fails after retries
        // leaves the list truncated (and therefore unrouted) rather than
        // failing the whole repository.
        for (id, number, files_cursor) in paginated_files {
            tracing::debug!(pr = number, "fetching extra changed-file pages");
            let Some(pr) = out.iter_mut().find(|p| p.number == number) else {
                continue;
            };
            let (paths, truncated) = collect_files(
                std::mem::take(&mut pr.changed_files),
                files_cursor,
                |cursor| self.fetch_files_page(&id, cursor),
            )
            .await;
            if truncated {
                tracing::warn!(
                    pr = number,
                    "changed-file list truncated at {} paths",
                    paths.len()
                );
            }
            pr.changed_files = paths;
            pr.changed_files_truncated = truncated;
        }

        Ok((out, node_ids, default_branch))
    }

    /// One page of a PR's changed files: the `files` connection JSON.
    async fn fetch_files_page(&self, pr_id: &str, cursor: Option<String>) -> Result<Value> {
        let vars = json!({ "id": pr_id, "cursor": cursor, "files": FILES_PER_PAGE });
        let resp = self.execute(PR_FILES_QUERY, vars).await?;
        resp.pointer("/data/node/files")
            .cloned()
            .ok_or_else(|| anyhow!("node.files missing"))
    }

    /// Retry once for PRs whose mergeable came back UNKNOWN. Caller passes (node_id, pr_number)
    /// pairs gathered during the main fetch.
    pub async fn recheck_mergeable(
        &self,
        prs: &mut [RawPr],
        id_pairs: &[(String, u64)],
        delay: Duration,
    ) -> Result<()> {
        let unknowns: Vec<(String, u64)> = id_pairs
            .iter()
            .filter(|(_, n)| {
                prs.iter()
                    .find(|p| p.number == *n)
                    .map(|p| p.mergeable == Mergeable::Unknown)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        if unknowns.is_empty() {
            return Ok(());
        }
        tracing::info!(
            "{} PR(s) had mergeable=UNKNOWN; sleeping {:?} then re-querying",
            unknowns.len(),
            delay
        );
        tokio::time::sleep(delay).await;
        for (id, number) in unknowns {
            let vars = json!({ "id": id });
            let resp = self.execute(PR_MERGEABLE_QUERY, vars).await?;
            let m = parse_mergeable(resp.pointer("/data/node/mergeable"));
            if let Some(pr) = prs.iter_mut().find(|p| p.number == number) {
                pr.mergeable = m;
            }
        }
        Ok(())
    }

    /// What GitHub and the engine record about each PR's stage changes, by PR
    /// number, for the (node_id, number) pairs of the main fetch. A PR deleted
    /// meanwhile is left out; a record comment deleted meanwhile leaves its
    /// PR's records incomplete.
    pub async fn fetch_stage_evidence(
        &self,
        prs: &[(String, u64)],
    ) -> Result<HashMap<u64, Evidence>> {
        let mut out: HashMap<u64, Evidence> = HashMap::new();
        let mut histories: Vec<(String, u64)> = Vec::new();
        for batch in prs.chunks(EVIDENCE_PRS_PER_REQUEST) {
            let ids: Vec<&str> = batch.iter().map(|(id, _)| id.as_str()).collect();
            let resp = self
                .execute_nodes(STAGE_EVIDENCE_QUERY, json!({ "ids": ids }))
                .await?;
            histories.extend(add_evidence_page(&mut out, batch, &resp)?);
        }
        for batch in histories.chunks(HISTORIES_PER_REQUEST) {
            let ids: Vec<&str> = batch.iter().map(|(id, _)| id.as_str()).collect();
            let resp = self
                .execute_nodes(COMMENT_HISTORY_QUERY, json!({ "ids": ids }))
                .await?;
            add_history_page(&mut out, batch, &resp)?;
        }
        Ok(out)
    }

    async fn fetch_extra_threads(&self, pr_id: &str) -> Result<Vec<RawThread>> {
        let mut out = Vec::new();
        let mut cursor: Option<String> = None;
        // The first page is already in the main query; start from its cursor.
        // For simplicity here, we re-fetch from the start and skip duplicates by id.
        let mut seen_ids = std::collections::HashSet::new();
        loop {
            let vars = json!({
                "id": pr_id,
                "cursor": cursor,
                "comments": COMMENTS_PER_THREAD,
            });
            let resp = self.execute(PR_THREADS_QUERY, vars).await?;
            let conn = resp
                .pointer("/data/node/reviewThreads")
                .ok_or_else(|| anyhow!("node.reviewThreads missing"))?;
            let nodes = conn
                .get("nodes")
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow!("reviewThreads.nodes missing"))?;
            for node in nodes {
                let t = parse_thread(node)?;
                if seen_ids.insert(t.id.clone()) {
                    out.push(t);
                }
            }
            let page_info = conn
                .get("pageInfo")
                .ok_or_else(|| anyhow!("reviewThreads.pageInfo missing"))?;
            if !page_info
                .get("hasNextPage")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                break;
            }
            cursor = page_info
                .get("endCursor")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
        }
        Ok(out)
    }

    async fn execute(&self, query: &str, variables: Value) -> Result<Value> {
        self.request(query, variables, false).await
    }

    /// As `execute`, for a `nodes(ids:)` query: a node deleted since its id
    /// was read comes back `null` with a NOT_FOUND error, which is an answer
    /// about that node, not a failed request.
    async fn execute_nodes(&self, query: &str, variables: Value) -> Result<Value> {
        self.request(query, variables, true).await
    }

    async fn request(&self, query: &str, variables: Value, missing_ok: bool) -> Result<Value> {
        let body = json!({ "query": query, "variables": variables });
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            // Bundle send + body-read so that mid-stream truncation (chunked
            // transfer cut by GitHub during a 502/503 storm) is treated as a
            // transient transport failure, same as a 5xx response.
            let res = match self.client.post(&self.endpoint).json(&body).send().await {
                Ok(r) => r,
                Err(e) => {
                    if attempt > MAX_RETRIES {
                        return Err(anyhow!(e)
                            .context(format!("POST to {} (final attempt)", self.endpoint)));
                    }
                    let sleep_secs = backoff_secs(attempt);
                    tracing::warn!(attempt, sleep_secs, "send failed: {e}; retrying");
                    tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                    continue;
                }
            };
            let status = res.status();
            let retry_after = res
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok());
            let ratelimit_reset = res
                .headers()
                .get("x-ratelimit-reset")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<i64>().ok());
            let text = match res.text().await {
                Ok(t) => t,
                Err(e) => {
                    if attempt > MAX_RETRIES {
                        return Err(anyhow!(e).context("reading response body (final attempt)"));
                    }
                    let sleep_secs = backoff_secs(attempt);
                    tracing::warn!(
                        attempt,
                        sleep_secs,
                        "body read failed (status {status}): {e}; retrying"
                    );
                    tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                    continue;
                }
            };

            if status == StatusCode::TOO_MANY_REQUESTS
                || status == StatusCode::FORBIDDEN && text.contains("rate limit")
            {
                let sleep_secs = retry_after.unwrap_or_else(|| {
                    if let Some(reset_at) = ratelimit_reset {
                        let now = Utc::now().timestamp();
                        (reset_at - now).max(1) as u64
                    } else {
                        backoff_secs(attempt)
                    }
                });
                if attempt > MAX_RETRIES {
                    bail!("rate-limited after {MAX_RETRIES} retries");
                }
                tracing::warn!(
                    attempt,
                    sleep_secs,
                    "rate-limited (status {status}); sleeping then retrying"
                );
                tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                continue;
            }
            if status.is_server_error() {
                if attempt > MAX_RETRIES {
                    bail!("server error {status} after {MAX_RETRIES} retries: {text}");
                }
                let sleep_secs = backoff_secs(attempt);
                tracing::warn!(attempt, sleep_secs, "server error {status}; retrying");
                tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                continue;
            }
            if !status.is_success() {
                bail!("HTTP {status}: {text}");
            }

            // A success status with a body cut short — large pages are several
            // MB — is the same transient failure as a body that could not be
            // read at all, and is retried the same way.
            let value: Value = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(e) if attempt <= MAX_RETRIES => {
                    let sleep_secs = backoff_secs(attempt);
                    tracing::warn!(
                        attempt,
                        sleep_secs,
                        bytes = text.len(),
                        "unparsable response body: {e}; retrying"
                    );
                    tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                    continue;
                }
                Err(e) => {
                    return Err(anyhow!(e).context(format!(
                        "parsing JSON response of {} bytes (final attempt)",
                        text.len()
                    )))
                }
            };
            if let Some(errors) = value.get("errors").and_then(|v| v.as_array()) {
                if !errors.is_empty() {
                    let is_rate = errors
                        .iter()
                        .any(|e| e.get("type").and_then(|t| t.as_str()) == Some("RATE_LIMITED"));
                    if is_rate && attempt <= MAX_RETRIES {
                        let sleep_secs = backoff_secs(attempt);
                        tracing::warn!(attempt, sleep_secs, "graphql RATE_LIMITED; retrying");
                        tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                        continue;
                    }
                    if missing_ok && only_missing_nodes(errors) {
                        return Ok(value);
                    }
                    bail!("graphql errors: {errors:?}");
                }
            }
            return Ok(value);
        }
    }
}

/// Whether every error says only that a node asked for by id no longer
/// exists: NOT_FOUND at `nodes[i]` itself, not at a field inside a node.
fn only_missing_nodes(errors: &[Value]) -> bool {
    errors.iter().all(|e| {
        let path = e.get("path").and_then(|p| p.as_array());
        e.get("type").and_then(|t| t.as_str()) == Some("NOT_FOUND")
            && path.is_some_and(|p| p.len() == 2 && p[0].as_str() == Some("nodes") && p[1].is_u64())
    })
}

fn backoff_secs(attempt: u32) -> u64 {
    // 2, 4, 8, 16, 32 ...
    1u64 << attempt.min(6)
}

/// Continue paging a PR's changed files from `cursor`, appending to
/// `initial`, until the connection ends or `MAX_CHANGED_FILES` is reached.
/// Returns the deduplicated paths and whether the list is incomplete
/// (bound hit, or a page could not be fetched).
async fn collect_files<F, Fut>(
    initial: Vec<String>,
    mut cursor: Option<String>,
    mut fetch_page: F,
) -> (Vec<String>, bool)
where
    F: FnMut(Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<Value>>,
{
    let mut seen: std::collections::HashSet<String> = initial.iter().cloned().collect();
    let mut paths = initial;
    loop {
        if paths.len() >= MAX_CHANGED_FILES {
            return (paths, true);
        }
        let page = match fetch_page(cursor.take()).await {
            Ok(page) => page,
            Err(e) => {
                tracing::warn!("changed-file page failed: {e:#}");
                return (paths, true);
            }
        };
        for path in page
            .pointer("/nodes")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
            .filter_map(|n| n.get("path").and_then(|v| v.as_str()))
        {
            if seen.insert(path.to_string()) {
                paths.push(path.to_string());
            }
        }
        let page_info = page.get("pageInfo");
        let has_next = page_info
            .and_then(|p| p.get("hasNextPage"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !has_next {
            return (paths, false);
        }
        cursor = page_info
            .and_then(|p| p.get("endCursor"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        if cursor.is_none() {
            tracing::warn!("changed-file page has more results but no cursor");
            return (paths, true);
        }
    }
}

/// A PR node parsed from the list query plus what is still left to page.
pub struct ParsedPr {
    pub pr: RawPr,
    pub node_id: String,
    pub threads_have_more: bool,
    pub files_have_more: bool,
    /// Cursor to continue the changed-file list from, when `files_have_more`.
    pub files_cursor: Option<String>,
}

/// Parse a PR node from GraphQL JSON.
pub fn parse_pr_node(node: &Value, repo: &str) -> Result<ParsedPr> {
    let number = node
        .get("number")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow!("missing number"))?;
    let title = string_field(node, "title")?;
    let url = string_field(node, "url")?;
    let id = string_field(node, "id")?;
    let is_draft = node
        .get("isDraft")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let mergeable = parse_mergeable(node.get("mergeable"));
    let created_at = parse_datetime(node, "createdAt")?;
    let updated_at = parse_datetime(node, "updatedAt")?;
    let author = author_login(node);

    let labels = node
        .pointer("/labels/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|n| {
                    n.get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();

    let last_commit = node
        .pointer("/commits/nodes/0/commit")
        .map(|c| -> Result<LastCommit> {
            Ok(LastCommit {
                committed_date: parse_datetime(c, "committedDate")?,
                status_state: c
                    .pointer("/statusCheckRollup/state")
                    .and_then(|v| v.as_str())
                    .map(parse_status_state),
            })
        })
        .transpose()?;

    let reviews = node
        .pointer("/reviews/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(parse_review).collect::<Result<Vec<_>>>())
        .transpose()?
        .unwrap_or_default();

    let base_ref = node
        .get("baseRefName")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let changed_files: Vec<String> = node
        .pointer("/files/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|n| {
                    n.get("path")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    let files_have_more = node
        .pointer("/files/pageInfo/hasNextPage")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let files_cursor = node
        .pointer("/files/pageInfo/endCursor")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    // More files than returned, and no page to continue from: the list is
    // final and incomplete. (With a next page the caller pages on.)
    let changed_files_truncated = !files_have_more
        && node
            .pointer("/files/totalCount")
            .and_then(|v| v.as_u64())
            .is_some_and(|total| total > changed_files.len() as u64);

    let requested_reviewers: Vec<String> = node
        .pointer("/reviewRequests/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|n| {
                    let r = n.get("requestedReviewer")?;
                    // Only Users contribute logins; Team / Mannequin are skipped.
                    if r.get("__typename").and_then(|v| v.as_str()) != Some("User") {
                        return None;
                    }
                    r.get("login")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();

    let threads_conn = node
        .get("reviewThreads")
        .ok_or_else(|| anyhow!("missing reviewThreads"))?;
    let thread_nodes = threads_conn
        .get("nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("reviewThreads.nodes missing"))?;
    let threads = thread_nodes
        .iter()
        .map(parse_thread)
        .collect::<Result<Vec<_>>>()?;
    let page_info = threads_conn.get("pageInfo");
    let threads_have_more = page_info
        .and_then(|p| p.get("hasNextPage"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Ok(ParsedPr {
        pr: RawPr {
            repo: repo.to_string(),
            number,
            title,
            url,
            author,
            created_at,
            updated_at,
            is_draft,
            mergeable,
            labels,
            last_commit,
            reviews,
            threads,
            requested_reviewers,
            base_ref,
            changed_files,
            changed_files_truncated,
        },
        node_id: id,
        threads_have_more,
        files_have_more,
        files_cursor,
    })
}

fn parse_thread(node: &Value) -> Result<RawThread> {
    let id = string_field(node, "id")?;
    let is_resolved = node
        .get("isResolved")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| anyhow!("thread missing isResolved"))?;
    let is_outdated = node
        .get("isOutdated")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| anyhow!("thread missing isOutdated"))?;
    let comments = node
        .pointer("/comments/nodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().map(parse_comment).collect::<Result<Vec<_>>>())
        .transpose()?
        .unwrap_or_default();
    Ok(RawThread {
        id,
        is_resolved,
        is_outdated,
        comments,
    })
}

/// The `nodes` of a `nodes(ids:)` response, one per id asked for.
fn nodes_of(resp: &Value, asked: usize) -> Result<&Vec<Value>> {
    let nodes = resp
        .pointer("/data/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("response missing data.nodes"))?;
    if nodes.len() != asked {
        bail!("asked for {asked} nodes, got {}", nodes.len());
    }
    Ok(nodes)
}

/// Add one stage-evidence response for the (node_id, number) pairs `asked` to
/// `out`. Returns the engine account's comments found, each with its PR, for
/// `add_history_page` to read.
pub fn add_evidence_page(
    out: &mut HashMap<u64, Evidence>,
    asked: &[(String, u64)],
    resp: &Value,
) -> Result<Vec<(String, u64)>> {
    let mut histories = Vec::new();
    let nodes = nodes_of(resp, asked.len())?;
    for ((_, number), node) in asked.iter().zip(nodes).filter(|(_, n)| !n.is_null()) {
        let (evidence, engine_comments) = parse_evidence_node(node, *number)
            .with_context(|| format!("parsing stage evidence of #{number}"))?;
        histories.extend(engine_comments.into_iter().map(|id| (id, *number)));
        out.insert(*number, evidence);
    }
    Ok(histories)
}

/// Add one comment-history response for the (comment id, PR number) pairs
/// `asked` to their PRs in `out`. A comment deleted since it was listed may
/// have been a record, so its PR's records are no longer complete.
pub fn add_history_page(
    out: &mut HashMap<u64, Evidence>,
    asked: &[(String, u64)],
    resp: &Value,
) -> Result<()> {
    let nodes = nodes_of(resp, asked.len())?;
    for ((id, number), node) in asked.iter().zip(nodes) {
        let Some(evidence) = out.get_mut(number) else {
            continue;
        };
        if node.is_null() {
            evidence.comments_complete = false;
            continue;
        }
        let history = parse_comment_history(node)
            .with_context(|| format!("parsing comment {id} of #{number}"))?;
        evidence.comments.push(history);
    }
    Ok(())
}

/// One PR's draft and base changes, oldest first, and the ids of the engine
/// account's comments on it, whose histories are read separately.
fn parse_evidence_node(node: &Value, number: u64) -> Result<(Evidence, Vec<String>)> {
    let events = node
        .pointer("/timelineItems/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("timelineItems.nodes missing"))?
        .iter()
        .filter_map(|n| parse_stage_event(n).transpose())
        .collect::<Result<Vec<_>>>()?;
    let engine_comments = node
        .pointer("/comments/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("comments.nodes missing"))?
        .iter()
        .filter(|c| author_login(c).is_some_and(|a| a.eq_ignore_ascii_case(ENGINE_LOGIN)))
        .map(|c| string_field(c, "id"))
        .collect::<Result<Vec<_>>>()?;
    let earlier = |connection: &str| {
        node.pointer(&format!("/{connection}/pageInfo/hasPreviousPage"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    };
    Ok((
        Evidence {
            number,
            created_at: parse_datetime(node, "createdAt")?,
            is_draft: node
                .get("isDraft")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| anyhow!("missing isDraft"))?,
            base: string_field(node, "baseRefName")?,
            events,
            events_complete: !earlier("timelineItems"),
            comments: vec![],
            comments_complete: !earlier("comments"),
        },
        engine_comments,
    ))
}

/// A draft or base change; `None` for an event type the query did not ask for.
fn parse_stage_event(node: &Value) -> Result<Option<PrEvent>> {
    let typename = node.get("__typename").and_then(|v| v.as_str());
    let at = || parse_datetime(node, "createdAt");
    Ok(Some(match typename {
        Some("ConvertToDraftEvent") => PrEvent::ConvertedToDraft { at: at()? },
        Some("ReadyForReviewEvent") => PrEvent::ReadyForReview { at: at()? },
        Some("BaseRefChangedEvent") => PrEvent::BaseChanged {
            at: at()?,
            from: string_field(node, "previousRefName")?,
        },
        Some("AutomaticBaseChangeSucceededEvent") => PrEvent::BaseChanged {
            at: at()?,
            from: string_field(node, "oldBase")?,
        },
        Some("ReopenedEvent") => PrEvent::Reopened { at: at()? },
        _ => return Ok(None),
    }))
}

/// Every revision of one comment. An edited comment's revisions include its
/// original; one never edited has only its body.
fn parse_comment_history(node: &Value) -> Result<CommentHistory> {
    let author = author_login(node);
    let edits = node
        .pointer("/userContentEdits/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("userContentEdits.nodes missing"))?;
    let revisions = if edits.is_empty() {
        vec![Revision {
            at: parse_datetime(node, "createdAt")?,
            editor: author.clone(),
            body: node
                .get("body")
                .and_then(|v| v.as_str())
                .map(str::to_string),
        }]
    } else {
        // GitHub lists them newest first.
        edits
            .iter()
            .rev()
            .map(|e| {
                Ok(Revision {
                    at: parse_datetime(e, "editedAt")?,
                    editor: e.get("editor").and_then(actor_login),
                    body: e.get("diff").and_then(|v| v.as_str()).map(str::to_string),
                })
            })
            .collect::<Result<Vec<_>>>()?
    };
    let more = node
        .pointer("/userContentEdits/pageInfo/hasNextPage")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    Ok(CommentHistory {
        author,
        revisions,
        complete: !more,
    })
}

/// The login of a node's `author`. A GitHub App account is named as REST, the
/// engine and the config name it (`dependabot[bot]`), not by the bare slug
/// GraphQL returns — on PRs, reviews and comments alike, so a bot's own
/// replies on its own PR still read as the author's.
fn author_login(node: &Value) -> Option<String> {
    actor_login(node.get("author")?)
}

/// An actor's login, a GitHub App account's in full, as `author_login`.
fn actor_login(actor: &Value) -> Option<String> {
    let login = actor.get("login")?.as_str()?;
    let is_bot = actor.get("__typename").and_then(|v| v.as_str()) == Some("Bot");
    Some(if is_bot && !login.ends_with("[bot]") {
        format!("{login}[bot]")
    } else {
        login.to_string()
    })
}

fn parse_comment(node: &Value) -> Result<RawComment> {
    let author = author_login(node);
    let created_at = parse_datetime(node, "createdAt")?;
    let body = string_field(node, "body").unwrap_or_default();
    Ok(RawComment {
        author,
        created_at,
        body,
    })
}

fn parse_review(node: &Value) -> Result<Review> {
    let state = node
        .get("state")
        .and_then(|v| v.as_str())
        .map(parse_review_state)
        .unwrap_or(ReviewState::Commented);
    let author = author_login(node);
    let submitted_at = node
        .get("submittedAt")
        .and_then(|v| v.as_str())
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&Utc));
    Ok(Review {
        state,
        author,
        submitted_at,
    })
}

fn parse_review_state(s: &str) -> ReviewState {
    match s {
        "APPROVED" => ReviewState::Approved,
        "CHANGES_REQUESTED" => ReviewState::ChangesRequested,
        "DISMISSED" => ReviewState::Dismissed,
        "PENDING" => ReviewState::Pending,
        _ => ReviewState::Commented,
    }
}

fn parse_status_state(s: &str) -> StatusState {
    match s {
        "SUCCESS" => StatusState::Success,
        "FAILURE" => StatusState::Failure,
        "PENDING" => StatusState::Pending,
        "ERROR" => StatusState::Error,
        _ => StatusState::Expected,
    }
}

fn parse_mergeable(v: Option<&Value>) -> Mergeable {
    match v.and_then(|v| v.as_str()) {
        Some("MERGEABLE") => Mergeable::Mergeable,
        Some("CONFLICTING") => Mergeable::Conflicting,
        _ => Mergeable::Unknown,
    }
}

fn parse_datetime(node: &Value, field: &str) -> Result<DateTime<Utc>> {
    let s = node
        .get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("missing {field}"))?;
    let dt = DateTime::parse_from_rfc3339(s).with_context(|| format!("parsing {field}={s}"))?;
    Ok(dt.with_timezone(&Utc))
}

fn string_field(node: &Value, field: &str) -> Result<String> {
    node.get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("missing {field}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows() {
        assert_eq!(backoff_secs(1), 2);
        assert_eq!(backoff_secs(2), 4);
        assert_eq!(backoff_secs(3), 8);
        assert!(backoff_secs(20) <= 64);
    }

    #[test]
    fn parse_mergeable_values() {
        assert_eq!(
            parse_mergeable(Some(&json!("MERGEABLE"))),
            Mergeable::Mergeable
        );
        assert_eq!(
            parse_mergeable(Some(&json!("CONFLICTING"))),
            Mergeable::Conflicting
        );
        assert_eq!(parse_mergeable(Some(&json!("UNKNOWN"))), Mergeable::Unknown);
        assert_eq!(parse_mergeable(None), Mergeable::Unknown);
    }

    #[test]
    fn changed_files_truncation_is_detected_from_total_count() {
        let mut node = json!({
            "id": "PR_1", "number": 1, "title": "t", "url": "u",
            "createdAt": "2026-04-01T00:00:00Z", "updatedAt": "2026-05-01T00:00:00Z",
            "files": { "totalCount": 3, "nodes": [{"path": "a"}, {"path": "b"}] },
            "reviewThreads": { "pageInfo": { "hasNextPage": false }, "nodes": [] }
        });
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert_eq!(parsed.pr.changed_files, vec!["a", "b"]);
        assert!(parsed.pr.changed_files_truncated);
        assert!(!parsed.files_have_more);

        node["files"]["totalCount"] = json!(2);
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert!(!parsed.pr.changed_files_truncated);

        // A next page means the caller will page on: not truncated (yet).
        node["files"]["totalCount"] = json!(120);
        node["files"]["pageInfo"] = json!({ "hasNextPage": true, "endCursor": "c1" });
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert!(!parsed.pr.changed_files_truncated);
        assert!(parsed.files_have_more);
        assert_eq!(parsed.files_cursor.as_deref(), Some("c1"));

        // Fixtures written before totalCount existed still parse.
        node["files"].as_object_mut().unwrap().remove("totalCount");
        node["files"].as_object_mut().unwrap().remove("pageInfo");
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert!(!parsed.pr.changed_files_truncated);
    }

    fn page(paths: &[&str], next: Option<&str>) -> Value {
        json!({
            "pageInfo": { "hasNextPage": next.is_some(), "endCursor": next },
            "nodes": paths.iter().map(|p| json!({ "path": p })).collect::<Vec<_>>(),
        })
    }

    #[tokio::test]
    async fn collect_files_follows_every_page_and_dedupes() {
        // First page (50 of totalCount 120) already parsed; two follow-up pages.
        let initial: Vec<String> = (0..50).map(|i| format!("f{i}")).collect();
        let mut requested: Vec<Option<String>> = Vec::new();
        let (paths, truncated) = collect_files(initial, Some("c50".into()), |cursor| {
            requested.push(cursor.clone());
            let resp = match cursor.as_deref() {
                Some("c50") => {
                    let mut p: Vec<String> = (50..100).map(|i| format!("f{i}")).collect();
                    p.push("f49".into()); // overlap with the first page
                    let refs: Vec<&str> = p.iter().map(String::as_str).collect();
                    page(&refs, Some("c100"))
                }
                Some("c100") => {
                    let p: Vec<String> = (100..120).map(|i| format!("f{i}")).collect();
                    let refs: Vec<&str> = p.iter().map(String::as_str).collect();
                    page(&refs, None)
                }
                other => panic!("unexpected cursor {other:?}"),
            };
            async move { Ok(resp) }
        })
        .await;
        assert!(!truncated);
        assert_eq!(paths.len(), 120, "every path once");
        assert_eq!(paths[0], "f0");
        assert_eq!(paths[119], "f119");
        assert_eq!(
            requested,
            vec![Some("c50".to_string()), Some("c100".to_string())]
        );
    }

    #[tokio::test]
    async fn collect_files_stops_at_the_bound_and_on_page_failure() {
        // An endless connection is cut at MAX_CHANGED_FILES and flagged.
        let mut n = 0usize;
        let (paths, truncated) = collect_files(vec![], Some("c".into()), |_| {
            let p: Vec<String> = (n..n + 1000).map(|i| format!("f{i}")).collect();
            n += 1000;
            let refs: Vec<&str> = p.iter().map(String::as_str).collect();
            let resp = page(&refs, Some("more"));
            async move { Ok(resp) }
        })
        .await;
        assert!(truncated);
        assert_eq!(paths.len(), MAX_CHANGED_FILES);

        // A page that fails keeps what was fetched and flags the list.
        let (paths, truncated) = collect_files(vec!["a".into()], Some("c".into()), |_| async {
            Err(anyhow!("boom"))
        })
        .await;
        assert!(truncated);
        assert_eq!(paths, vec!["a"]);
    }

    #[test]
    fn parse_pr_node_minimal() {
        let node = json!({
            "id": "PR_1",
            "number": 42,
            "title": "Add foo",
            "url": "https://example.com/pr/42",
            "isDraft": false,
            "mergeable": "MERGEABLE",
            "createdAt": "2026-04-01T00:00:00Z",
            "updatedAt": "2026-05-01T00:00:00Z",
            "author": { "login": "alice" },
            "labels": { "nodes": [{"name": "bug"}] },
            "commits": { "nodes": [{
                "commit": {
                    "committedDate": "2026-04-15T00:00:00Z",
                    "statusCheckRollup": { "state": "FAILURE" }
                }
            }] },
            "reviews": { "nodes": [] },
            "reviewThreads": {
                "pageInfo": { "endCursor": null, "hasNextPage": false },
                "nodes": []
            }
        });
        let ParsedPr {
            pr,
            node_id: id,
            threads_have_more: more,
            ..
        } = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert_eq!(pr.repo, "dashpay/platform");
        assert_eq!(pr.number, 42);
        assert!(
            !pr.changed_files_truncated,
            "no files block → not truncated"
        );
        assert_eq!(pr.title, "Add foo");
        assert_eq!(pr.author.as_deref(), Some("alice"));
        assert_eq!(pr.labels, vec!["bug".to_string()]);
        assert_eq!(pr.mergeable, Mergeable::Mergeable);
        assert!(!more);
        assert_eq!(id, "PR_1");
        assert_eq!(
            pr.last_commit.as_ref().unwrap().status_state,
            Some(StatusState::Failure)
        );
    }

    /// GraphQL names a GitHub App account by its bare slug (`dependabot`),
    /// while the REST API, the review engine and `excluded_authors` all say
    /// `dependabot[bot]`. Read bare, the exclusion never matched and the
    /// account could not be told apart from a person of the same name.
    #[test]
    fn a_bot_account_author_is_named_as_rest_names_it() {
        let mut node = json!({
            "id": "PR_2", "number": 7, "title": "Bump", "url": "u", "isDraft": false,
            "mergeable": "MERGEABLE",
            "createdAt": "2026-04-01T00:00:00Z", "updatedAt": "2026-05-01T00:00:00Z",
            "author": { "login": "dependabot", "__typename": "Bot" },
            "reviewThreads": { "pageInfo": { "endCursor": null, "hasNextPage": false }, "nodes": [] }
        });
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert_eq!(parsed.pr.author.as_deref(), Some("dependabot[bot]"));
        node["author"] = json!({ "login": "thepastaclaw", "__typename": "User" });
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert_eq!(parsed.pr.author.as_deref(), Some("thepastaclaw"));
        assert!(PR_LIST_QUERY.contains("author { login __typename }"));
    }

    /// A bot's own replies on its own PR must read as the author's, or they
    /// count as reviewer activity and as unresolved review threads.
    #[test]
    fn review_and_comment_authors_are_named_like_the_pr_author() {
        let bot = json!({ "login": "dependabot", "__typename": "Bot" });
        let comment = parse_comment(&json!({
            "author": bot, "createdAt": "2026-04-01T00:00:00Z", "body": "x"
        }))
        .unwrap();
        assert_eq!(comment.author.as_deref(), Some("dependabot[bot]"));
        let review = parse_review(&json!({
            "state": "COMMENTED", "author": bot, "submittedAt": "2026-04-01T00:00:00Z"
        }))
        .unwrap();
        assert_eq!(review.author.as_deref(), Some("dependabot[bot]"));
        for query in [
            PR_LIST_QUERY,
            PR_THREADS_QUERY,
            STAGE_EVIDENCE_QUERY,
            COMMENT_HISTORY_QUERY,
        ] {
            assert!(
                !query.contains("author { login }") && !query.contains("editor { login }"),
                "every author and editor asks for its type"
            );
        }
    }

    fn evidence_node() -> Value {
        json!({
            "number": 42, "createdAt": "2026-09-01T00:00:00Z", "isDraft": false,
            "baseRefName": "v5.0-dev",
            "timelineItems": { "pageInfo": { "hasPreviousPage": false }, "nodes": [
                { "__typename": "ConvertToDraftEvent", "createdAt": "2026-09-02T00:00:00Z" },
                { "__typename": "ReadyForReviewEvent", "createdAt": "2026-09-03T00:00:00Z" },
                { "__typename": "AutomaticBaseChangeSucceededEvent",
                  "createdAt": "2026-09-04T00:00:00Z", "oldBase": "feat/x" },
                { "__typename": "BaseRefChangedEvent",
                  "createdAt": "2026-09-05T00:00:00Z", "previousRefName": "v4.3-dev" },
                { "__typename": "ReopenedEvent", "createdAt": "2026-09-06T00:00:00Z" },
                {}
            ] },
            "comments": { "pageInfo": { "hasPreviousPage": false }, "nodes": [
                { "id": "IC_engine", "createdAt": "2026-09-02T00:00:00Z",
                  "author": { "login": "github-actions", "__typename": "Bot" } },
                { "id": "IC_person", "createdAt": "2026-09-02T00:00:00Z",
                  "author": { "login": "github-actions", "__typename": "User" } },
                { "id": "IC_ghost", "createdAt": "2026-09-02T00:00:00Z", "author": null }
            ] }
        })
    }

    /// Only the engine's App account's comments are read further: a person
    /// can paste the record marker, and an account named like the App is a
    /// different account.
    #[test]
    fn stage_evidence_keeps_events_in_order_and_only_the_engines_comments() {
        let (evidence, engine_comments) = parse_evidence_node(&evidence_node(), 42).unwrap();
        assert_eq!(engine_comments, vec!["IC_engine"]);
        let at = |s: &str| s.parse::<DateTime<Utc>>().unwrap();
        assert_eq!(
            evidence.events,
            vec![
                PrEvent::ConvertedToDraft {
                    at: at("2026-09-02T00:00:00Z")
                },
                PrEvent::ReadyForReview {
                    at: at("2026-09-03T00:00:00Z")
                },
                PrEvent::BaseChanged {
                    at: at("2026-09-04T00:00:00Z"),
                    from: "feat/x".into()
                },
                PrEvent::BaseChanged {
                    at: at("2026-09-05T00:00:00Z"),
                    from: "v4.3-dev".into()
                },
                PrEvent::Reopened {
                    at: at("2026-09-06T00:00:00Z")
                },
            ],
            "an event the query did not ask for is skipped"
        );
        assert!(evidence.events_complete && evidence.comments_complete);
        assert!(evidence.comments.is_empty(), "histories come separately");

        let mut node = evidence_node();
        node["timelineItems"]["pageInfo"]["hasPreviousPage"] = json!(true);
        node["comments"]["pageInfo"] = json!({});
        let (evidence, _) = parse_evidence_node(&node, 42).unwrap();
        assert!(!evidence.events_complete, "older events exist");
        assert!(!evidence.comments_complete, "unknown is not complete");
    }

    #[test]
    fn a_comment_history_holds_every_revision_and_who_wrote_it() {
        let bot = json!({ "login": "github-actions", "__typename": "Bot" });
        let unedited = parse_comment_history(&json!({
            "createdAt": "2026-09-02T00:00:00Z", "author": bot, "body": "original",
            "userContentEdits": { "pageInfo": { "hasNextPage": false }, "nodes": [] }
        }))
        .unwrap();
        assert_eq!(unedited.author.as_deref(), Some(ENGINE_LOGIN));
        assert_eq!(unedited.revisions.len(), 1);
        assert_eq!(unedited.revisions[0].editor.as_deref(), Some(ENGINE_LOGIN));
        assert_eq!(unedited.revisions[0].body.as_deref(), Some("original"));
        assert!(unedited.complete);

        // Once edited, GitHub lists the original among the revisions; the
        // current body is the newest of them and is not read twice.
        let edited = parse_comment_history(&json!({
            "createdAt": "2026-09-02T00:00:00Z", "author": bot, "body": "admin's",
            "userContentEdits": { "pageInfo": { "hasNextPage": true }, "nodes": [
                { "editedAt": "2026-09-04T00:00:00Z", "diff": "admin's",
                  "editor": { "login": "an-admin", "__typename": "User" } },
                { "editedAt": "2026-09-03T00:00:00Z", "diff": null, "editor": bot },
                { "editedAt": "2026-09-02T00:00:00Z", "diff": "original", "editor": bot }
            ] }
        }))
        .unwrap();
        let editors: Vec<Option<&str>> = edited
            .revisions
            .iter()
            .map(|r| r.editor.as_deref())
            .collect();
        assert_eq!(
            editors,
            vec![Some(ENGINE_LOGIN), Some(ENGINE_LOGIN), Some("an-admin")],
            "oldest first"
        );
        assert_eq!(edited.revisions[0].body.as_deref(), Some("original"));
        assert_eq!(edited.revisions[1].body, None, "a deleted revision");
        assert!(!edited.complete, "older revisions exist");
    }

    /// Each engine comment's history reaches its own PR; a PR deleted since the
    /// list was read is left out, and a comment deleted since it was listed
    /// leaves its PR's records incomplete rather than silently shorter.
    #[test]
    fn evidence_pages_join_each_history_to_its_pr() {
        let mut other = evidence_node();
        other["number"] = json!(43);
        other["comments"]["nodes"] = json!([
            { "id": "IC_gone", "createdAt": "2026-09-02T00:00:00Z",
              "author": { "login": "github-actions", "__typename": "Bot" } }
        ]);
        let asked = [
            ("PR_42".to_string(), 42),
            ("PR_deleted".to_string(), 44),
            ("PR_43".to_string(), 43),
        ];
        let mut out = HashMap::new();
        let resp = json!({ "data": { "nodes": [evidence_node(), null, other] } });
        let histories = add_evidence_page(&mut out, &asked, &resp).unwrap();
        assert_eq!(
            histories,
            vec![("IC_engine".to_string(), 42), ("IC_gone".to_string(), 43)]
        );
        assert_eq!(out.len(), 2);

        let resp = json!({ "data": { "nodes": [{
            "createdAt": "2026-09-02T00:00:00Z", "body": "b",
            "author": { "login": "github-actions", "__typename": "Bot" },
            "userContentEdits": { "pageInfo": { "hasNextPage": false }, "nodes": [] }
        }, null] } });
        add_history_page(&mut out, &histories, &resp).unwrap();
        assert_eq!(out[&42].comments.len(), 1);
        assert!(out[&42].comments_complete);
        assert!(out[&43].comments.is_empty());
        assert!(!out[&43].comments_complete);
    }

    #[test]
    fn only_a_missing_node_is_tolerated_in_a_nodes_query() {
        let missing = json!({ "type": "NOT_FOUND", "path": ["nodes", 1] });
        let limited = json!({ "type": "RATE_LIMITED" });
        let other = json!({ "message": "Something went wrong" });
        // A field inside a node that could not be resolved is not a deleted
        // node: reading on would quietly drop what that field held.
        let inside = json!({ "type": "NOT_FOUND", "path": ["nodes", 1, "editor"] });
        assert!(only_missing_nodes(&[missing.clone(), missing.clone()]));
        assert!(!only_missing_nodes(&[missing.clone(), limited]));
        assert!(!only_missing_nodes(&[missing.clone(), other]));
        assert!(!only_missing_nodes(&[missing, inside]));
    }

    #[test]
    fn a_nodes_response_must_answer_every_id() {
        let resp = json!({ "data": { "nodes": [null, {}] } });
        assert_eq!(nodes_of(&resp, 2).unwrap().len(), 2);
        assert!(nodes_of(&resp, 3).is_err());
        assert!(nodes_of(&json!({ "data": null }), 1).is_err());
    }
}
