use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::dashboard::{ClosedPr, ClosedRead, DecisiveReview, PrFacts, Verdict};
use crate::model::{
    LastCommit, Mergeable, RawComment, RawPr, RawThread, Review, ReviewState, StatusState,
};
use crate::stages::{
    CommentHistory, Coverage, Evidence, PrEvent, RepoEvidence, Revision, ENGINE_LOGIN,
    RECORD_MARKER,
};

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
query PrHygieneList($owner: String!, $name: String!, $cursor: String, $threads: Int!, $comments: Int!,
                    $decisive: Int!) {
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
        author { login __typename ... on User { databaseId } ... on Bot { databaseId } }
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
        decisiveReviews: reviews(states: [APPROVED, CHANGES_REQUESTED, DISMISSED], last: $decisive) {
          pageInfo { hasPreviousPage }
          nodes { state submittedAt
                  author { login __typename ... on User { databaseId } ... on Bot { databaseId } } }
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

/// When each PR changed draft state or base or was reopened, and which
/// comments the engine account wrote — not what they say. Kept out of the PR
/// list query, the heaviest one; this costs one point a request.
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
          ... on BaseRefChangedEvent { createdAt previousRefName currentRefName }
          ... on AutomaticBaseChangeSucceededEvent { createdAt oldBase newBase }
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

/// What the engine account's comments say now, to find its record comments
/// among them (it also posts bot nudges, and other workflows post as it).
const ENGINE_COMMENTS_QUERY: &str = r#"
query PrHygieneEngineComments($ids: [ID!]!) {
  rateLimit { remaining resetAt cost }
  nodes(ids: $ids) {
    ... on IssueComment { createdAt lastEditedAt body author { login __typename } }
  }
}
"#;

/// Every revision of an edited record comment, newest first, the original
/// among them.
const COMMENT_HISTORY_QUERY: &str = r#"
query PrHygieneCommentHistory($ids: [ID!]!) {
  rateLimit { remaining resetAt cost }
  nodes(ids: $ids) {
    ... on IssueComment {
      userContentEdits(first: 100) {
        pageInfo { hasNextPage }
        nodes { editedAt diff editor { login __typename } }
      }
    }
  }
}
"#;

/// Nodes per stage-evidence request. A batch that fails is tried again in
/// halves before its nodes are given up.
const EVIDENCE_NODES_PER_REQUEST: usize = 50;

/// Decisive reviews read per PR: the newest this many. Each run reads them
/// afresh, so the newest are the ones not yet seen. A PR with more is logged
/// rather than paged: the most any PR of the five repositories had in a year
/// was 70.
pub const DECISIVE_REVIEWS: u32 = 100;

/// Merged and closed PRs, most recently updated first. Closing a PR updates
/// it, so once a page reaches PRs last updated before the window, no later
/// page holds one closed within it. Not a search: a search stops at 1 000
/// results, which a year of a busy repository can pass. The cursor holds the
/// last update read, so a PR updated while the list is being read moves
/// ahead of it and is missed by that read; a later run, whose window still
/// holds it, reads it.
const CLOSED_LIST_QUERY: &str = r#"
query PrHygieneClosed($owner: String!, $name: String!, $cursor: String, $decisive: Int!) {
  rateLimit { remaining resetAt cost }
  repository(owner: $owner, name: $name) {
    pullRequests(states: [MERGED, CLOSED], first: 50, after: $cursor,
                 orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo { endCursor hasNextPage }
      nodes {
        number isDraft createdAt updatedAt mergedAt closedAt
        author { login __typename ... on User { databaseId } ... on Bot { databaseId } }
        timelineItems(first: 1, itemTypes: [READY_FOR_REVIEW_EVENT, CONVERT_TO_DRAFT_EVENT]) {
          nodes { __typename ... on ReadyForReviewEvent { createdAt } }
        }
        decisiveReviews: reviews(states: [APPROVED, CHANGES_REQUESTED, DISMISSED], last: $decisive) {
          pageInfo { hasPreviousPage }
          nodes { state submittedAt
                  author { login __typename ... on User { databaseId } ... on Bot { databaseId } } }
        }
      }
    }
  }
}
"#;

pub struct Fetcher {
    client: Client,
    endpoint: String,
    /// Rate-limit points GitHub charged for the queries answered so far.
    points: AtomicU64,
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
            points: AtomicU64::new(0),
        })
    }

    /// Rate-limit points GitHub charged for the queries answered so far.
    pub fn points(&self) -> u64 {
        self.points.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Fetch every open PR in `owner/name`, fully populated.
    /// Returns the PRs, (node_id, number) pairs for follow-up queries, the
    /// repository's default branch name (used to drive stale-branch detection)
    /// and what GitHub attaches to each PR, by number.
    pub async fn fetch_all_open_prs(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<(
        Vec<RawPr>,
        Vec<(String, u64)>,
        Option<String>,
        HashMap<u64, PrFacts>,
    )> {
        let mut out: Vec<RawPr> = Vec::new();
        let mut node_ids: Vec<(String, u64)> = Vec::new();
        let mut facts: HashMap<u64, PrFacts> = HashMap::new();
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
                "decisive": DECISIVE_REVIEWS,
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
                facts.insert(parsed.pr.number, parsed.facts);
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

        Ok((out, node_ids, default_branch, facts))
    }

    /// The PRs of `owner/name` merged or closed since `since`, or why they
    /// could not be read. Never a failed run: the open PRs stand without them.
    pub async fn fetch_closed_prs(
        &self,
        owner: &str,
        name: &str,
        since: DateTime<Utc>,
    ) -> ClosedRead {
        self.closed_prs(owner, name, since)
            .await
            .map_err(|e| shorten(format!("{e:#}")))
    }

    async fn closed_prs(
        &self,
        owner: &str,
        name: &str,
        since: DateTime<Utc>,
    ) -> Result<Vec<ClosedPr>> {
        let repo = format!("{owner}/{name}");
        let mut out = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let vars = json!({
                "owner": owner,
                "name": name,
                "cursor": cursor,
                "decisive": DECISIVE_REVIEWS,
            });
            let resp = self.execute(CLOSED_LIST_QUERY, vars).await?;
            let conn = resp
                .pointer("/data/repository/pullRequests")
                .ok_or_else(|| anyhow!("response missing data.repository.pullRequests"))?;
            let nodes = conn
                .get("nodes")
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow!("pullRequests.nodes missing"))?;
            let (closed, past_window) = closed_since(nodes, &repo, since)?;
            out.extend(closed);
            let has_next = conn
                .pointer("/pageInfo/hasNextPage")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| anyhow!("pullRequests.pageInfo missing"))?;
            if past_window || !has_next {
                return Ok(out);
            }
            cursor = Some(
                conn.pointer("/pageInfo/endCursor")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow!("a next page with no cursor"))?
                    .to_string(),
            );
        }
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

    /// What GitHub and the engine record about each PR's stage changes, for
    /// the (node_id, number) pairs of the main fetch. What cannot be read is
    /// lost for its own PRs only, and said in the result's `error`: a PR whose
    /// comments could not be read keeps what its timeline says.
    pub async fn fetch_stage_evidence(&self, prs: &[(String, u64)]) -> RepoEvidence {
        let mut reads = StageReads::default();
        let ids: Vec<String> = prs.iter().map(|(id, _)| id.clone()).collect();
        let nodes = self.read_nodes(STAGE_EVIDENCE_QUERY, &ids).await;
        reads.prs(prs, nodes);
        let nodes = self
            .read_nodes(ENGINE_COMMENTS_QUERY, &reads.engine_comment_ids())
            .await;
        reads.comments(nodes);
        let nodes = self
            .read_nodes(COMMENT_HISTORY_QUERY, &reads.edited_comment_ids())
            .await;
        reads.histories(nodes);
        reads.finish()
    }

    /// Each id's node, `null` for one deleted meanwhile, or why it could not
    /// be read. A batch that fails — a GraphQL timeout on a page too large to
    /// answer in time is the usual cause — is tried again in halves before its
    /// nodes are given up.
    async fn read_nodes(&self, query: &str, ids: &[String]) -> Vec<NodeRead> {
        let mut out = Vec::with_capacity(ids.len());
        for batch in ids.chunks(EVIDENCE_NODES_PER_REQUEST) {
            let halves = match self.nodes_page(query, batch).await {
                Ok(nodes) => {
                    out.extend(nodes.into_iter().map(Ok));
                    continue;
                }
                Err(e) if batch.len() > 1 => {
                    tracing::warn!(nodes = batch.len(), "retrying in halves: {e:#}");
                    batch.split_at(batch.len() / 2)
                }
                Err(e) => {
                    out.push(Err(format!("{e:#}")));
                    continue;
                }
            };
            for half in [halves.0, halves.1] {
                match self.nodes_page(query, half).await {
                    Ok(nodes) => out.extend(nodes.into_iter().map(Ok)),
                    Err(e) => {
                        let why = format!("{e:#}");
                        out.extend(half.iter().map(|_| Err(why.clone())));
                    }
                }
            }
        }
        out
    }

    async fn nodes_page(&self, query: &str, ids: &[String]) -> Result<Vec<Value>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let resp = self.execute_nodes(query, json!({ "ids": ids })).await?;
        Ok(nodes_of(&resp, ids.len())?.clone())
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
                        self.charge(&value);
                        return Ok(value);
                    }
                    bail!("graphql errors: {errors:?}");
                }
            }
            self.charge(&value);
            return Ok(value);
        }
    }

    /// Count what an answered query cost. One that does not ask its cost is
    /// a single-node read, which GitHub charges its minimum, one point.
    fn charge(&self, value: &Value) {
        let cost = value
            .pointer("/data/rateLimit/cost")
            .and_then(|v| v.as_u64())
            .unwrap_or(1);
        self.points.fetch_add(cost, Ordering::Relaxed);
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
    pub facts: PrFacts,
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
    let facts = PrFacts {
        author_id: node.get("author").and_then(actor_id),
        reviews: decisive_reviews(node, repo, number),
    };

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
        facts,
    })
}

/// The PRs on one page of the closed list that were closed since `since`,
/// and whether the page reaches PRs last updated before it. The list runs
/// from the most recently updated, and closing a PR updates it: past such a
/// PR no page holds one closed since. A PR closed long ago and touched
/// since — a comment, a label — is on the page and left out.
pub fn closed_since(
    nodes: &[Value],
    repo: &str,
    since: DateTime<Utc>,
) -> Result<(Vec<ClosedPr>, bool)> {
    let mut closed = Vec::new();
    let mut past_window = false;
    for node in nodes {
        if parse_datetime(node, "updatedAt")? < since {
            past_window = true;
            continue;
        }
        let pr = parse_closed_node(node, repo)?;
        if pr.closed_at >= since {
            closed.push(pr);
        }
    }
    Ok((closed, past_window))
}

/// A merged or closed PR from the closed list.
pub fn parse_closed_node(node: &Value, repo: &str) -> Result<ClosedPr> {
    let number = node
        .get("number")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow!("missing number"))?;
    let parse = || -> Result<ClosedPr> {
        let merged_at = match node.get("mergedAt") {
            Some(Value::Null) => None,
            Some(_) => Some(parse_datetime(node, "mergedAt")?),
            None => bail!("missing mergedAt"),
        };
        Ok(ClosedPr {
            key: format!("{repo}#{number}"),
            repo: repo.to_string(),
            number,
            author: author_login(node),
            author_id: node.get("author").and_then(actor_id),
            created_at: parse_datetime(node, "createdAt")?,
            ready_at: ready_at(node)?,
            merged_at,
            closed_at: parse_datetime(node, "closedAt")?,
            reviews: decisive_reviews(node, repo, number),
        })
    };
    parse().with_context(|| format!("{repo}#{number}"))
}

/// When a closed PR was first ready for review. Its first draft toggle says
/// how it was opened: marked ready means it was opened as a draft, and was
/// first ready then; made a draft means it was opened ready. With no toggle
/// it was opened as it is now: ready, or a draft it never left.
fn ready_at(node: &Value) -> Result<Option<DateTime<Utc>>> {
    let first = node
        .pointer("/timelineItems/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("timelineItems.nodes missing"))?
        .first();
    let opened_as_draft = match first {
        Some(event) => {
            event.get("__typename").and_then(|v| v.as_str()) == Some("ReadyForReviewEvent")
        }
        None => node
            .get("isDraft")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("missing isDraft"))?,
    };
    match (opened_as_draft, first) {
        (false, _) => parse_datetime(node, "createdAt").map(Some),
        (true, Some(ready)) => parse_datetime(ready, "createdAt").map(Some),
        (true, None) => Ok(None),
    }
}

/// A PR's decisive reviews, oldest first, each by an account GitHub gives an
/// id: a deleted account's review has no one to join it to. One that cannot
/// be read is left out with a warning rather than costing the PR, or the
/// repository, the rest of what was read. Absent from the node (a fixture
/// older than the field), there are none.
fn decisive_reviews(node: &Value, repo: &str, number: u64) -> Vec<DecisiveReview> {
    let Some(conn) = node.get("decisiveReviews") else {
        return vec![];
    };
    if conn
        .pointer("/pageInfo/hasPreviousPage")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        tracing::warn!(
            "{repo}#{number}: more than {DECISIVE_REVIEWS} decisive reviews; only the newest read"
        );
    }
    let nodes = conn.get("nodes").and_then(|v| v.as_array());
    if nodes.is_none() {
        tracing::warn!("{repo}#{number}: decisive reviews unreadable: no nodes");
    }
    nodes
        .into_iter()
        .flatten()
        .filter_map(|review| {
            let author = review.get("author").unwrap_or(&Value::Null);
            let (Some(reviewer), Some(reviewer_id)) = (actor_login(author), actor_id(author))
            else {
                tracing::debug!("{repo}#{number}: a decisive review by no identified account");
                return None;
            };
            let state = match review.get("state").and_then(|v| v.as_str()) {
                Some("APPROVED") => Verdict::Approved,
                Some("CHANGES_REQUESTED") => Verdict::ChangesRequested,
                Some("DISMISSED") => Verdict::Dismissed,
                other => {
                    tracing::warn!("{repo}#{number}: not a decisive review state: {other:?}");
                    return None;
                }
            };
            match parse_datetime(review, "submittedAt") {
                Ok(at) => Some(DecisiveReview {
                    reviewer,
                    reviewer_id,
                    state,
                    at,
                }),
                Err(e) => {
                    tracing::warn!("{repo}#{number}: decisive review unreadable: {e:#}");
                    None
                }
            }
        })
        .collect()
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

/// A node as read: the node (`null` when deleted meanwhile), or why it could
/// not be read.
pub type NodeRead = std::result::Result<Value, String>;

/// Stage evidence assembled from its three reads, each a list of nodes in the
/// order their ids were asked for: the PRs, then the engine account's comments
/// on them, then the revisions of the record comments among those that were
/// edited. A read that fails costs only its own PRs, and says so.
#[derive(Default)]
pub struct StageReads {
    evidence: HashMap<u64, Evidence>,
    asked: usize,
    /// The engine account's comments, each with its PR.
    engine_comments: Vec<(String, u64)>,
    /// Edited record comments awaiting their revisions, with PR and author.
    edited: Vec<(String, u64, Option<String>)>,
    /// PRs a failed read left without evidence or with comments unread.
    failed: BTreeSet<u64>,
    reason: Option<String>,
}

impl StageReads {
    pub fn prs(&mut self, asked: &[(String, u64)], nodes: Vec<NodeRead>) {
        self.asked += asked.len();
        for ((_, number), node) in zip_reads(asked, nodes) {
            let parsed = node
                .map_err(|why| format!("#{number}: {why}"))
                .and_then(|node| {
                    if node.is_null() {
                        return Ok(None);
                    }
                    parse_evidence_node(&node, *number)
                        .map(Some)
                        .map_err(|e| format!("#{number}: {e:#}"))
                });
            match parsed {
                Ok(Some((evidence, comments))) => {
                    self.engine_comments
                        .extend(comments.into_iter().map(|id| (id, *number)));
                    self.evidence.insert(*number, evidence);
                }
                Ok(None) => {}
                Err(why) => self.fail(*number, why),
            }
        }
    }

    pub fn engine_comment_ids(&self) -> Vec<String> {
        self.engine_comments
            .iter()
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn comments(&mut self, nodes: Vec<NodeRead>) {
        let asked = std::mem::take(&mut self.engine_comments);
        for ((id, number), node) in zip_reads(&asked, nodes) {
            let parsed = node
                .map_err(|why| format!("#{number}: {why}"))
                .and_then(|node| {
                    if node.is_null() {
                        return Ok(None);
                    }
                    parse_engine_comment(&node)
                        .map(Some)
                        .map_err(|e| format!("#{number} comment {id}: {e:#}"))
                });
            match parsed {
                Ok(Some(EngineComment::Other)) => {}
                Ok(Some(EngineComment::Unedited(history))) => {
                    if let Some(evidence) = self.evidence.get_mut(number) {
                        evidence.comments.push(history);
                    }
                }
                Ok(Some(EngineComment::Edited { author })) => {
                    self.edited.push((id.clone(), *number, author));
                }
                // Deleted since it was listed: gone from the record, as the
                // next run will find it.
                Ok(None) => {}
                Err(why) => {
                    self.comments_unread(*number);
                    self.fail(*number, why);
                }
            }
        }
    }

    pub fn edited_comment_ids(&self) -> Vec<String> {
        self.edited.iter().map(|(id, _, _)| id.clone()).collect()
    }

    pub fn histories(&mut self, nodes: Vec<NodeRead>) {
        let asked = std::mem::take(&mut self.edited);
        for ((id, number, author), node) in zip_reads(&asked, nodes) {
            let parsed = node
                .map_err(|why| format!("#{number}: {why}"))
                .and_then(|node| {
                    if node.is_null() {
                        return Ok(None);
                    }
                    parse_revisions(&node, author.clone())
                        .map(Some)
                        .map_err(|e| format!("#{number} comment {id}: {e:#}"))
                });
            match parsed {
                Ok(Some(history)) => {
                    if let Some(evidence) = self.evidence.get_mut(number) {
                        evidence.comments.push(history);
                    }
                }
                Ok(None) => {}
                Err(why) => {
                    self.comments_unread(*number);
                    self.fail(*number, why);
                }
            }
        }
    }

    pub fn finish(self) -> RepoEvidence {
        let error = self.reason.map(|reason| {
            format!(
                "{} of {} PRs could not be read: {}",
                self.failed.len(),
                self.asked,
                shorten(reason)
            )
        });
        RepoEvidence {
            prs: self.evidence,
            error,
        }
    }

    fn comments_unread(&mut self, number: u64) {
        if let Some(evidence) = self.evidence.get_mut(&number) {
            evidence.comments_read = Coverage::Partial;
        }
    }

    fn fail(&mut self, number: u64, why: String) {
        tracing::warn!(pr = number, "stage evidence: {why}");
        self.failed.insert(number);
        self.reason.get_or_insert(why);
    }
}

/// How much of a failed read's reason is kept for the page.
const MAX_REASON_CHARS: usize = 300;

/// A failed read's reason, cut to what the page keeps: an HTTP error can
/// carry a whole error page.
fn shorten(reason: String) -> String {
    if reason.chars().count() > MAX_REASON_CHARS {
        reason.chars().take(MAX_REASON_CHARS).chain(['…']).collect()
    } else {
        reason
    }
}

/// Pair each id asked for with its node; a response that does not answer
/// every id answers none.
fn zip_reads<T>(asked: &[T], nodes: Vec<NodeRead>) -> impl Iterator<Item = (&T, NodeRead)> {
    let nodes = if nodes.len() == asked.len() {
        nodes
    } else {
        let why = format!("{} nodes for {} ids", nodes.len(), asked.len());
        vec![Err(why); asked.len()]
    };
    asked.iter().zip(nodes)
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

/// One PR's draft, base and reopen events, oldest first, and the ids of the
/// engine account's comments on it, read separately.
fn parse_evidence_node(node: &Value, number: u64) -> Result<(Evidence, Vec<String>)> {
    let events = node
        .pointer("/timelineItems/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("timelineItems.nodes missing"))?
        .iter()
        .filter_map(|n| parse_stage_event(n).transpose())
        .collect::<Result<Vec<_>>>()?;
    let comments = node
        .pointer("/comments/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("comments.nodes missing"))?;
    let engine_comments = comments
        .iter()
        .filter(|c| author_login(c).is_some_and(|a| a.eq_ignore_ascii_case(ENGINE_LOGIN)))
        .map(|c| string_field(c, "id"))
        .collect::<Result<Vec<_>>>()?;
    let earlier = |connection: &str| {
        node.pointer(&format!("/{connection}/pageInfo/hasPreviousPage"))
            .and_then(|v| v.as_bool())
    };
    let comments_read = match earlier("comments") {
        Some(false) => Coverage::All,
        // Only the newest comments were read: from the oldest of them on.
        Some(true) => comments
            .iter()
            .map(|c| parse_datetime(c, "createdAt"))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .min()
            .map_or(Coverage::Partial, Coverage::Since),
        None => Coverage::Partial,
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
            events_complete: earlier("timelineItems") == Some(false),
            comments: vec![],
            comments_read,
        },
        engine_comments,
    ))
}

/// A draft, base or reopen event; `None` for an event type the query did not
/// ask for.
fn parse_stage_event(node: &Value) -> Result<Option<PrEvent>> {
    let typename = node.get("__typename").and_then(|v| v.as_str());
    let at = || parse_datetime(node, "createdAt");
    Ok(Some(match typename {
        Some("ConvertToDraftEvent") => PrEvent::ConvertedToDraft { at: at()? },
        Some("ReadyForReviewEvent") => PrEvent::ReadyForReview { at: at()? },
        Some("BaseRefChangedEvent") => PrEvent::BaseChanged {
            at: at()?,
            from: string_field(node, "previousRefName")?,
            to: string_field(node, "currentRefName")?,
        },
        Some("AutomaticBaseChangeSucceededEvent") => PrEvent::BaseChanged {
            at: at()?,
            from: string_field(node, "oldBase")?,
            to: string_field(node, "newBase")?,
        },
        Some("ReopenedEvent") => PrEvent::Reopened { at: at()? },
        _ => return Ok(None),
    }))
}

/// One of the engine account's comments, as it reads now.
enum EngineComment {
    /// Not a record comment: a bot nudge, or another workflow's.
    Other,
    /// A record comment never edited: its body is its only revision.
    Unedited(CommentHistory),
    /// A record comment whose revisions are read next.
    Edited { author: Option<String> },
}

fn parse_engine_comment(node: &Value) -> Result<EngineComment> {
    let body = string_field(node, "body")?;
    if !body.contains(RECORD_MARKER) {
        return Ok(EngineComment::Other);
    }
    let author = author_login(node);
    // Read as never edited only where GitHub says so: a body taken for the
    // whole history would give an entry from an incomplete read.
    match node.get("lastEditedAt") {
        Some(Value::Null) => {}
        Some(_) => return Ok(EngineComment::Edited { author }),
        None => bail!("missing lastEditedAt"),
    }
    Ok(EngineComment::Unedited(CommentHistory {
        revisions: vec![Revision {
            at: parse_datetime(node, "createdAt")?,
            editor: author.clone(),
            body: Some(body),
        }],
        author,
        complete: true,
    }))
}

/// Every revision of an edited comment, the original among them, oldest
/// first, each with who wrote it.
fn parse_revisions(node: &Value, author: Option<String>) -> Result<CommentHistory> {
    let edits = node
        .pointer("/userContentEdits/nodes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("userContentEdits.nodes missing"))?;
    if edits.is_empty() {
        bail!("an edited comment lists no revisions");
    }
    // GitHub lists them newest first.
    let revisions = edits
        .iter()
        .rev()
        .map(|e| {
            Ok(Revision {
                at: parse_datetime(e, "editedAt")?,
                editor: e.get("editor").and_then(actor_login),
                body: e.get("diff").and_then(|v| v.as_str()).map(str::to_string),
            })
        })
        .collect::<Result<Vec<_>>>()?;
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

/// An actor's numeric account id, which GraphQL gives a User and a Bot (the
/// same id REST gives `name[bot]`); `None` for an actor of another kind.
fn actor_id(actor: &Value) -> Option<u64> {
    actor.get("databaseId")?.as_u64()
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
            ENGINE_COMMENTS_QUERY,
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
                  "createdAt": "2026-09-04T00:00:00Z", "oldBase": "feat/x", "newBase": "v4.3-dev" },
                { "__typename": "BaseRefChangedEvent", "createdAt": "2026-09-05T00:00:00Z",
                  "previousRefName": "v4.3-dev", "currentRefName": "v5.0-dev" },
                { "__typename": "ReopenedEvent", "createdAt": "2026-09-06T00:00:00Z" },
                {}
            ] },
            "comments": { "pageInfo": { "hasPreviousPage": false }, "nodes": [
                { "id": "IC_engine", "createdAt": "2026-09-02T00:00:00Z",
                  "author": { "login": "github-actions", "__typename": "Bot" } },
                { "id": "IC_person", "createdAt": "2026-09-03T00:00:00Z",
                  "author": { "login": "github-actions", "__typename": "User" } },
                { "id": "IC_ghost", "createdAt": "2026-09-04T00:00:00Z", "author": null }
            ] }
        })
    }

    fn at(s: &str) -> DateTime<Utc> {
        s.parse().unwrap()
    }

    /// Only the engine's App account's comments are read further: a person
    /// can paste the record marker, and an account named like the App is a
    /// different account.
    #[test]
    fn stage_evidence_keeps_events_in_order_and_only_the_engines_comments() {
        let (evidence, engine_comments) = parse_evidence_node(&evidence_node(), 42).unwrap();
        assert_eq!(engine_comments, vec!["IC_engine"]);
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
                    from: "feat/x".into(),
                    to: "v4.3-dev".into(),
                },
                PrEvent::BaseChanged {
                    at: at("2026-09-05T00:00:00Z"),
                    from: "v4.3-dev".into(),
                    to: "v5.0-dev".into(),
                },
                PrEvent::Reopened {
                    at: at("2026-09-06T00:00:00Z")
                },
            ],
            "an event the query did not ask for is skipped"
        );
        assert!(evidence.events_complete);
        assert_eq!(evidence.comments_read, Coverage::All);
        assert!(evidence.comments.is_empty(), "comments come separately");

        // Only the newest comments read: from the oldest of them on.
        let mut node = evidence_node();
        node["timelineItems"]["pageInfo"]["hasPreviousPage"] = json!(true);
        node["comments"]["pageInfo"]["hasPreviousPage"] = json!(true);
        let (evidence, _) = parse_evidence_node(&node, 42).unwrap();
        assert!(!evidence.events_complete, "older events exist");
        assert_eq!(
            evidence.comments_read,
            Coverage::Since(at("2026-09-02T00:00:00Z"))
        );
        node["comments"]["pageInfo"] = json!({});
        let (evidence, _) = parse_evidence_node(&node, 42).unwrap();
        assert_eq!(
            evidence.comments_read,
            Coverage::Partial,
            "unknown is partial"
        );
    }

    fn engine_comment_node(body: &str, edited: bool) -> Value {
        json!({
            "createdAt": "2026-09-02T00:00:00Z",
            "lastEditedAt": if edited { json!("2026-09-04T00:00:00Z") } else { Value::Null },
            "body": body,
            "author": { "login": "github-actions", "__typename": "Bot" }
        })
    }

    const RECORD: &str = "<!-- platform-pr-review-state-v1 {\"head\":\"h\",\"number\":42,\"state\":\"waiting-bots\"} -->";

    /// Revisions are read only for the engine's record comments, and only
    /// where there is more than the body already read.
    #[test]
    fn only_edited_record_comments_have_their_revisions_read() {
        let nudge =
            parse_engine_comment(&engine_comment_node("<!-- pr-hygiene-nudge v1 -->", true));
        assert!(matches!(nudge.unwrap(), EngineComment::Other));
        let EngineComment::Unedited(history) =
            parse_engine_comment(&engine_comment_node(RECORD, false)).unwrap()
        else {
            panic!("an unedited record comment is its own history");
        };
        assert_eq!(history.author.as_deref(), Some(ENGINE_LOGIN));
        assert_eq!(history.revisions.len(), 1);
        assert_eq!(history.revisions[0].editor.as_deref(), Some(ENGINE_LOGIN));
        assert_eq!(history.revisions[0].body.as_deref(), Some(RECORD));
        assert!(history.complete);
        assert!(matches!(
            parse_engine_comment(&engine_comment_node(RECORD, true)).unwrap(),
            EngineComment::Edited { .. }
        ));
        // Not said to be unedited is not read as unedited.
        let mut unsure = engine_comment_node(RECORD, false);
        unsure.as_object_mut().unwrap().remove("lastEditedAt");
        assert!(parse_engine_comment(&unsure).is_err());
    }

    #[test]
    fn a_comment_history_holds_every_revision_and_who_wrote_it() {
        let bot = json!({ "login": "github-actions", "__typename": "Bot" });
        let edited = parse_revisions(
            &json!({ "userContentEdits": { "pageInfo": { "hasNextPage": true }, "nodes": [
                { "editedAt": "2026-09-05T00:00:00Z", "diff": "ghost's", "editor": null },
                { "editedAt": "2026-09-04T00:00:00Z", "diff": "admin's",
                  "editor": { "login": "an-admin", "__typename": "User" } },
                { "editedAt": "2026-09-03T00:00:00Z", "diff": null, "editor": bot },
                { "editedAt": "2026-09-02T00:00:00Z", "diff": "original", "editor": bot }
            ] } }),
            Some(ENGINE_LOGIN.into()),
        )
        .unwrap();
        let editors: Vec<Option<&str>> = edited
            .revisions
            .iter()
            .map(|r| r.editor.as_deref())
            .collect();
        assert_eq!(
            editors,
            vec![
                Some(ENGINE_LOGIN),
                Some(ENGINE_LOGIN),
                Some("an-admin"),
                None
            ],
            "oldest first; an editor GitHub no longer names is nobody's"
        );
        assert_eq!(edited.revisions[0].body.as_deref(), Some("original"));
        assert_eq!(edited.revisions[1].body, None, "a deleted revision");
        assert!(!edited.complete, "older revisions exist");
        assert!(
            parse_revisions(
                &json!({ "userContentEdits": { "pageInfo": { "hasNextPage": false }, "nodes": [] } }),
                None
            )
            .is_err(),
            "an edited comment with no revisions is not read as never edited"
        );
    }

    /// A PR the reads could not make sense of costs that PR alone, and is
    /// named in what the page is told.
    #[test]
    fn a_malformed_pr_costs_only_itself() {
        let mut broken = evidence_node();
        broken.as_object_mut().unwrap().remove("createdAt");
        let mut reads = StageReads::default();
        reads.prs(
            &[
                ("PR_42".into(), 42),
                ("PR_43".into(), 43),
                ("PR_44".into(), 44),
            ],
            vec![Ok(evidence_node()), Ok(broken), Ok(Value::Null)],
        );
        assert_eq!(reads.engine_comment_ids(), vec!["IC_engine"]);
        let evidence = reads.finish();
        assert_eq!(evidence.prs.len(), 1, "#43 malformed, #44 deleted");
        let error = evidence.error.unwrap();
        assert!(
            error.starts_with("1 of 3 PRs could not be read: #43"),
            "{error}"
        );

        // A response that does not answer every id answers none.
        let mut reads = StageReads::default();
        reads.prs(&[("PR_42".into(), 42)], vec![]);
        assert!(reads.finish().error.is_some());
    }

    /// A GraphQL endpoint on localhost answering each request with what
    /// `answer` makes of its body, recording the bodies it was sent.
    fn serve(
        answer: impl Fn(&Value) -> Value + Send + 'static,
    ) -> (String, std::sync::Arc<std::sync::Mutex<Vec<Value>>>) {
        use std::io::{BufRead, BufReader, Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/graphql", listener.local_addr().unwrap());
        let seen = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let log = seen.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap() == 0 || line == "\r\n" {
                        break;
                    }
                    if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        length = v.trim().parse().unwrap();
                    }
                }
                let mut body = vec![0; length];
                reader.read_exact(&mut body).unwrap();
                let request: Value = serde_json::from_slice(&body).unwrap();
                let reply = answer(&request).to_string();
                log.lock().unwrap().push(request);
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\
                     content-length: {}\r\nconnection: close\r\n\r\n{reply}",
                    reply.len()
                )
                .unwrap();
            }
        });
        (url, seen)
    }

    fn ids(request: &Value) -> Vec<String> {
        serde_json::from_value(request["variables"]["ids"].clone()).unwrap()
    }

    /// GitHub answers a `nodes` query naming a deleted node with that node
    /// `null` and a NOT_FOUND error for its index: an answer, not a failure.
    #[tokio::test]
    async fn a_deleted_node_is_an_answer_not_a_failure() {
        let (url, _) = serve(|_| {
            json!({
                "data": { "nodes": [null, { "body": "b" }] },
                "errors": [{ "type": "NOT_FOUND", "path": ["nodes", 0], "message": "Not Found" }]
            })
        });
        let fetcher = Fetcher::new("t").unwrap().with_endpoint(url);
        let read = fetcher
            .read_nodes(ENGINE_COMMENTS_QUERY, &["IC_gone".into(), "IC_here".into()])
            .await;
        assert_eq!(read, vec![Ok(Value::Null), Ok(json!({ "body": "b" }))]);
        assert!(
            fetcher
                .execute(PR_MERGEABLE_QUERY, json!({}))
                .await
                .is_err(),
            "outside a nodes query the same error fails the request"
        );
    }

    /// A page GitHub cannot answer in time fails whole; in halves it mostly
    /// can. A half that still fails costs its own PRs their comments, not
    /// their timeline, and not the other PRs anything.
    #[tokio::test]
    async fn a_failed_batch_is_retried_in_halves_and_costs_only_its_own_prs() {
        let (url, seen) = serve(|request| {
            let query = request["query"].as_str().unwrap();
            let ids = ids(request);
            if query.contains("PrHygieneStageEvidence") {
                let nodes: Vec<Value> = ids
                    .iter()
                    .map(|id| {
                        let n: u64 = id.trim_start_matches("PR_").parse().unwrap();
                        let mut node = evidence_node();
                        node["number"] = json!(n);
                        node["comments"]["nodes"][0]["id"] = json!(format!("IC_{n}"));
                        node
                    })
                    .collect();
                json!({ "data": { "nodes": nodes } })
            } else if ids.iter().any(|id| id == "IC_43") {
                json!({ "data": null, "errors": [{ "message":
                    "Something went wrong while executing your query. This may be the result of a timeout." }] })
            } else if query.contains("PrHygieneEngineComments") {
                let nodes: Vec<Value> = ids
                    .iter()
                    .map(|_| engine_comment_node(RECORD, true))
                    .collect();
                json!({ "data": { "nodes": nodes } })
            } else {
                let nodes: Vec<Value> = ids
                    .iter()
                    .map(|_| json!({ "userContentEdits": { "pageInfo": { "hasNextPage": false }, "nodes": [
                        { "editedAt": "2026-09-02T00:00:00Z", "diff": RECORD,
                          "editor": { "login": "github-actions", "__typename": "Bot" } }
                    ] } }))
                    .collect();
                json!({ "data": { "nodes": nodes } })
            }
        });
        let fetcher = Fetcher::new("t").unwrap().with_endpoint(url);
        let evidence = fetcher
            .fetch_stage_evidence(&[("PR_42".into(), 42), ("PR_43".into(), 43)])
            .await;
        let asked: Vec<Vec<String>> = seen.lock().unwrap().iter().map(ids).collect();
        assert_eq!(
            asked,
            vec![
                vec!["PR_42", "PR_43"],
                vec!["IC_42", "IC_43"],
                vec!["IC_42"],
                vec!["IC_43"],
                vec!["IC_42"],
            ]
        );
        let read = &evidence.prs[&42];
        assert_eq!(read.comments.len(), 1);
        assert_eq!(read.comments_read, Coverage::All);
        let unread = &evidence.prs[&43];
        assert_eq!(unread.comments_read, Coverage::Partial);
        assert_eq!(unread.events.len(), 5, "its timeline still stands");
        let error = evidence.error.unwrap();
        assert!(
            error.starts_with("1 of 2 PRs could not be read: #43"),
            "{error}"
        );
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

    fn user(login: &str, id: u64) -> Value {
        json!({ "login": login, "__typename": "User", "databaseId": id })
    }

    fn bot(login: &str, id: u64) -> Value {
        json!({ "login": login, "__typename": "Bot", "databaseId": id })
    }

    fn review(state: &str, at: &str, author: Value) -> Value {
        json!({ "state": state, "submittedAt": at, "author": author })
    }

    fn open_node(author: Value, reviews: Vec<Value>) -> Value {
        json!({
            "id": "PR_9", "number": 9, "title": "t", "url": "u", "isDraft": false,
            "createdAt": "2026-09-01T00:00:00Z", "updatedAt": "2026-09-02T00:00:00Z",
            "author": author,
            "reviewThreads": { "pageInfo": { "hasNextPage": false }, "nodes": [] },
            "decisiveReviews": { "pageInfo": { "hasPreviousPage": false }, "nodes": reviews }
        })
    }

    /// A closed PR, opened at `created` and closed at `closed`, last
    /// updated at `updated`, with no draft toggle.
    fn closed_node(number: u64, created: &str, closed: &str, updated: &str) -> Value {
        json!({
            "number": number, "isDraft": false,
            "createdAt": created, "updatedAt": updated,
            "mergedAt": closed, "closedAt": closed,
            "author": user("alice", 1001),
            "timelineItems": { "nodes": [] },
            "decisiveReviews": { "pageInfo": { "hasPreviousPage": false }, "nodes": [] }
        })
    }

    /// Speed joins people on GitHub's account ids, which a login cannot
    /// inherit. A GitHub App account's id is the one REST gives `name[bot]`,
    /// and it comes with the name REST gives it.
    #[test]
    fn a_users_and_a_bots_ids_are_read_from_the_pr_and_its_reviews() {
        let node = open_node(
            bot("dependabot", 49699333),
            vec![
                review("APPROVED", "2026-09-03T00:00:00Z", user("alice", 1001)),
                review(
                    "CHANGES_REQUESTED",
                    "2026-09-04T00:00:00Z",
                    bot("coderabbitai", 136622811),
                ),
            ],
        );
        let parsed = parse_pr_node(&node, "dashpay/platform").unwrap();
        assert_eq!(parsed.pr.author.as_deref(), Some("dependabot[bot]"));
        assert_eq!(parsed.facts.author_id, Some(49699333));
        assert_eq!(
            parsed.facts.reviews,
            vec![
                DecisiveReview {
                    reviewer: "alice".into(),
                    reviewer_id: 1001,
                    state: Verdict::Approved,
                    at: at("2026-09-03T00:00:00Z"),
                },
                DecisiveReview {
                    reviewer: "coderabbitai[bot]".into(),
                    reviewer_id: 136622811,
                    state: Verdict::ChangesRequested,
                    at: at("2026-09-04T00:00:00Z"),
                },
            ]
        );
        let user_pr = parse_pr_node(&open_node(user("alice", 1001), vec![]), "dashpay/platform");
        assert_eq!(user_pr.unwrap().facts.author_id, Some(1001));
        // A deleted account's PR names no one, and no id is made up for it.
        let ghost = parse_pr_node(&open_node(Value::Null, vec![]), "dashpay/platform").unwrap();
        assert_eq!(ghost.facts, PrFacts::default());

        let mut closed = closed_node(
            7,
            "2026-09-01T00:00:00Z",
            "2026-09-02T00:00:00Z",
            "2026-09-02T00:00:00Z",
        );
        closed["author"] = bot("dependabot", 49699333);
        let closed = parse_closed_node(&closed, "dashpay/platform").unwrap();
        assert_eq!(closed.author.as_deref(), Some("dependabot[bot]"));
        assert_eq!(closed.author_id, Some(49699333));
        for query in [PR_LIST_QUERY, CLOSED_LIST_QUERY] {
            assert!(
                asks(
                    query,
                    "... on User { databaseId } ... on Bot { databaseId }"
                ),
                "both account kinds give their id"
            );
        }
    }

    /// Whether a query holds `selection`, however either is laid out.
    fn asks(query: &str, selection: &str) -> bool {
        let squash = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
        squash(query).contains(&squash(selection))
    }

    /// Only decisions are kept, each joined to an account. The analyzer's own
    /// review read, which counts comments as reviewer activity, is a separate
    /// read and stays as it was.
    #[test]
    fn only_decisive_reviews_by_identified_accounts_are_kept() {
        let mannequin = json!({ "login": "imported", "__typename": "Mannequin" });
        let node = open_node(
            user("carol", 3003),
            vec![
                review("DISMISSED", "2026-09-03T00:00:00Z", user("bob", 4004)),
                review("APPROVED", "2026-09-04T00:00:00Z", Value::Null),
                review("APPROVED", "2026-09-05T00:00:00Z", mannequin),
                review("COMMENTED", "2026-09-06T00:00:00Z", user("bob", 4004)),
                json!({ "state": "APPROVED", "submittedAt": null, "author": user("bob", 4004) }),
                review("APPROVED", "2026-09-07T00:00:00Z", user("bob", 4004)),
            ],
        );
        let reviews = parse_pr_node(&node, "dashpay/platform")
            .unwrap()
            .facts
            .reviews;
        let kept: Vec<(Verdict, DateTime<Utc>)> = reviews.iter().map(|r| (r.state, r.at)).collect();
        assert_eq!(
            kept,
            vec![
                (Verdict::Dismissed, at("2026-09-03T00:00:00Z")),
                (Verdict::Approved, at("2026-09-07T00:00:00Z")),
            ],
            "a deleted or id-less reviewer, a comment and an unreadable review are left out"
        );
        // Asked for by state, so bots' comments cannot push decisions off the
        // page; the newest are read, being the ones not yet seen.
        let decisive =
            "decisiveReviews: reviews(states: [APPROVED, CHANGES_REQUESTED, DISMISSED], \
                        last: $decisive)";
        assert!(asks(PR_LIST_QUERY, decisive));
        assert!(asks(CLOSED_LIST_QUERY, decisive));
        assert!(asks(
            PR_LIST_QUERY,
            "reviews(first: 50) { nodes { state author { login __typename } submittedAt } }"
        ));
    }

    /// A PR's cycle runs from when it was first ready for review: a draft is
    /// not yet asking anyone for anything.
    #[test]
    fn a_closed_pr_is_first_ready_when_opened_unless_opened_as_a_draft() {
        let mut node = closed_node(
            7,
            "2026-09-01T00:00:00Z",
            "2026-09-10T00:00:00Z",
            "2026-09-10T00:00:00Z",
        );
        let ready = |node: &Value| {
            parse_closed_node(node, "dashpay/platform")
                .unwrap()
                .ready_at
        };
        assert_eq!(
            ready(&node),
            Some(at("2026-09-01T00:00:00Z")),
            "opened ready"
        );

        // Opened as a draft: its first toggle marks it ready, and later
        // toggles do not move that.
        node["timelineItems"]["nodes"] = json!([
            { "__typename": "ReadyForReviewEvent", "createdAt": "2026-09-04T00:00:00Z" },
            { "__typename": "ConvertToDraftEvent" },
            { "__typename": "ReadyForReviewEvent", "createdAt": "2026-09-06T00:00:00Z" }
        ]);
        assert_eq!(ready(&node), Some(at("2026-09-04T00:00:00Z")));

        // Opened ready, made a draft later and closed as one: ready when opened.
        node["isDraft"] = json!(true);
        node["timelineItems"]["nodes"] = json!([{ "__typename": "ConvertToDraftEvent" }]);
        assert_eq!(ready(&node), Some(at("2026-09-01T00:00:00Z")));

        // Opened as a draft and closed as one, never ready.
        node["timelineItems"]["nodes"] = json!([]);
        assert_eq!(ready(&node), None);

        // Not knowing the timeline is not knowing when it was ready.
        node.as_object_mut().unwrap().remove("timelineItems");
        assert!(parse_closed_node(&node, "dashpay/platform").is_err());
        assert!(asks(
            CLOSED_LIST_QUERY,
            "timelineItems(first: 1, itemTypes: [READY_FOR_REVIEW_EVENT, CONVERT_TO_DRAFT_EVENT])"
        ));
    }

    /// The list runs from the most recently updated. A PR closed long ago
    /// and commented on since is on it but not in the window; past the first
    /// PR updated before the window there is nothing more to read.
    #[test]
    fn the_window_cuts_by_update_to_stop_and_by_close_to_keep() {
        let since = at("2026-09-01T00:00:00Z");
        let nodes = vec![
            closed_node(
                3,
                "2026-08-20T00:00:00Z",
                "2026-09-05T00:00:00Z",
                "2026-09-06T00:00:00Z",
            ),
            closed_node(
                2,
                "2021-01-01T00:00:00Z",
                "2021-01-02T00:00:00Z",
                "2026-09-04T00:00:00Z",
            ),
            closed_node(
                1,
                "2026-08-01T00:00:00Z",
                "2026-09-01T00:00:00Z",
                "2026-09-01T00:00:00Z",
            ),
        ];
        let (closed, past) = closed_since(&nodes, "dashpay/platform", since).unwrap();
        let numbers: Vec<u64> = closed.iter().map(|c| c.number).collect();
        assert_eq!(numbers, vec![3, 1], "closed at the window's start is in it");
        assert!(!past, "every PR on the page was updated within the window");

        let mut older = nodes.clone();
        older.push(closed_node(
            0,
            "2026-08-01T00:00:00Z",
            "2026-08-31T23:59:59Z",
            "2026-08-31T23:59:59Z",
        ));
        let (closed, past) = closed_since(&older, "dashpay/platform", since).unwrap();
        assert_eq!(closed.len(), 2);
        assert!(past);
    }

    fn closed_page(nodes: Vec<Value>, next: Option<&str>) -> Value {
        json!({ "data": {
            "rateLimit": { "remaining": 4999, "resetAt": "2026-09-10T00:00:00Z", "cost": 3 },
            "repository": { "pullRequests": {
                "pageInfo": { "hasNextPage": next.is_some(), "endCursor": next },
                "nodes": nodes
            } }
        } })
    }

    /// Paging stops at the first page that reaches before the window, though
    /// GitHub has more: a year's backfill reads a year, not the repository's
    /// whole history. What it costs is counted.
    #[tokio::test]
    async fn the_closed_list_is_paged_until_it_reaches_before_the_window() {
        let (url, seen) = serve(|request| {
            let node = |n, updated| closed_node(n, "2026-08-01T00:00:00Z", updated, updated);
            match request["variables"]["cursor"].as_str() {
                None => closed_page(vec![node(3, "2026-09-05T00:00:00Z")], Some("c1")),
                Some("c1") => closed_page(
                    vec![
                        node(2, "2026-09-02T00:00:00Z"),
                        node(1, "2026-08-02T00:00:00Z"),
                    ],
                    Some("c2"),
                ),
                other => {
                    json!({ "errors": [{ "message": format!("read past the window: {other:?}") }] })
                }
            }
        });
        let fetcher = Fetcher::new("t").unwrap().with_endpoint(url);
        let closed = fetcher
            .fetch_closed_prs("dashpay", "platform", at("2026-09-01T00:00:00Z"))
            .await
            .unwrap();
        let numbers: Vec<u64> = closed.iter().map(|c| c.number).collect();
        assert_eq!(numbers, vec![3, 2]);
        assert_eq!(closed[0].key, "dashpay/platform#3");
        let requests = seen.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0]["variables"]["decisive"], DECISIVE_REVIEWS);
        assert_eq!(fetcher.points(), 6, "what GitHub reported each page cost");
    }

    /// A repository whose whole closed history fits in the window ends where
    /// GitHub's list ends, as a full answer.
    #[tokio::test]
    async fn a_closed_list_shorter_than_the_window_is_read_to_its_end() {
        let (url, seen) = serve(|_| {
            let updated = "2026-09-05T00:00:00Z";
            closed_page(
                vec![closed_node(1, "2026-09-04T00:00:00Z", updated, updated)],
                None,
            )
        });
        let fetcher = Fetcher::new("t").unwrap().with_endpoint(url);
        let closed = fetcher
            .fetch_closed_prs("dashpay", "platform", at("2026-09-01T00:00:00Z"))
            .await
            .unwrap();
        assert_eq!(closed.len(), 1);
        assert_eq!(seen.lock().unwrap().len(), 1);
    }

    /// A list cut short would read as fewer PRs closed: a page that fails
    /// costs the repository its whole closed list, and says why, rather than
    /// failing the run or passing on part of the list.
    #[tokio::test]
    async fn a_closed_list_that_fails_part_way_is_an_error_not_a_shorter_list() {
        let (url, _) = serve(|request| match request["variables"]["cursor"].as_str() {
            None => closed_page(
                vec![closed_node(
                    3,
                    "2026-08-01T00:00:00Z",
                    "2026-09-05T00:00:00Z",
                    "2026-09-05T00:00:00Z",
                )],
                Some("c1"),
            ),
            _ => json!({ "data": null, "errors": [{ "message": format!(
                "Something went wrong while executing your query. {}", "x".repeat(1000)) }] }),
        });
        let fetcher = Fetcher::new("t").unwrap().with_endpoint(url);
        let error = fetcher
            .fetch_closed_prs("dashpay", "platform", at("2026-09-01T00:00:00Z"))
            .await
            .unwrap_err();
        assert!(error.contains("Something went wrong"), "{error}");
        assert_eq!(
            error.chars().count(),
            MAX_REASON_CHARS + 1,
            "an error page is cut"
        );
    }
}
