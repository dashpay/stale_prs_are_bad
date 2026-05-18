use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use std::time::Duration;

use crate::model::{
    LastCommit, Mergeable, RawComment, RawPr, RawThread, Review, ReviewState, StatusState,
};

const GITHUB_API: &str = "https://api.github.com/graphql";
const USER_AGENT_STR: &str = concat!("pr-hygiene/", env!("CARGO_PKG_VERSION"));
const THREADS_PER_PAGE: u32 = 100;
const COMMENTS_PER_THREAD: u32 = 50;
const MAX_RETRIES: u32 = 5;

const PR_LIST_QUERY: &str = r#"
query PrHygieneList($owner: String!, $name: String!, $cursor: String, $threads: Int!, $comments: Int!) {
  rateLimit { remaining resetAt cost }
  repository(owner: $owner, name: $name) {
    pullRequests(states: OPEN, first: 50, after: $cursor,
                 orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo { endCursor hasNextPage }
      nodes {
        id number title url isDraft mergeable createdAt updatedAt
        baseRefName
        files(first: 50) { nodes { path } }
        author { login }
        labels(first: 20) { nodes { name } }
        commits(last: 1) {
          nodes { commit {
            committedDate
            statusCheckRollup { state }
          } }
        }
        reviews(first: 50) {
          nodes { state author { login } submittedAt }
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
              nodes { author { login } createdAt body }
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
            nodes { author { login } createdAt body }
          }
        }
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

    /// Fetch every open PR in the target repo, fully populated.
    /// Returns the PRs plus a list of (node_id, number) pairs (used for follow-up queries).
    pub async fn fetch_all_open_prs(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<(Vec<RawPr>, Vec<(String, u64)>)> {
        let mut out: Vec<RawPr> = Vec::new();
        let mut node_ids: Vec<(String, u64)> = Vec::new();
        let mut paginated_threads: Vec<(String, u64)> = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let vars = json!({
                "owner": owner,
                "name": name,
                "cursor": cursor,
                "threads": THREADS_PER_PAGE,
                "comments": COMMENTS_PER_THREAD,
            });
            let resp = self.execute(PR_LIST_QUERY, vars).await?;
            let pr_conn = resp
                .pointer("/data/repository/pullRequests")
                .ok_or_else(|| anyhow!("response missing data.repository.pullRequests"))?;
            let nodes = pr_conn
                .get("nodes")
                .and_then(|v| v.as_array())
                .ok_or_else(|| anyhow!("pullRequests.nodes missing"))?;
            for node in nodes {
                let (raw, node_id, thread_has_more, _thread_cursor) =
                    parse_pr_node(node).context("parsing PR node")?;
                node_ids.push((node_id.clone(), raw.number));
                if thread_has_more {
                    paginated_threads.push((node_id, raw.number));
                }
                out.push(raw);
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

        Ok((out, node_ids))
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
        let body = json!({ "query": query, "variables": variables });
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            let res = self
                .client
                .post(&self.endpoint)
                .json(&body)
                .send()
                .await
                .with_context(|| format!("POST to {}", self.endpoint))?;
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
            let text = res.text().await.context("reading response body")?;

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

            let value: Value = serde_json::from_str(&text)
                .with_context(|| format!("parsing JSON response: {text}"))?;
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
                    bail!("graphql errors: {errors:?}");
                }
            }
            return Ok(value);
        }
    }
}

fn backoff_secs(attempt: u32) -> u64 {
    // 2, 4, 8, 16, 32 ...
    1u64 << attempt.min(6)
}

/// Parse a PR node from GraphQL JSON. Returns (pr, node_id, threads_have_more, threads_end_cursor).
pub fn parse_pr_node(node: &Value) -> Result<(RawPr, String, bool, Option<String>)> {
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
    let author = node
        .get("author")
        .and_then(|a| a.get("login"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

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
    let threads_end_cursor = page_info
        .and_then(|p| p.get("endCursor"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok((
        RawPr {
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
        },
        id,
        threads_have_more,
        threads_end_cursor,
    ))
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

fn parse_comment(node: &Value) -> Result<RawComment> {
    let author = node
        .get("author")
        .and_then(|a| a.get("login"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
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
    let author = node
        .get("author")
        .and_then(|a| a.get("login"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
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
        let (pr, id, more, _) = parse_pr_node(&node).unwrap();
        assert_eq!(pr.number, 42);
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
}
