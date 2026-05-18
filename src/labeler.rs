use anyhow::{bail, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde_json::json;
use std::time::Duration;

use crate::model::AnalyzedPr;

const REST_API: &str = "https://api.github.com";
const USER_AGENT_STR: &str = concat!("pr-hygiene/", env!("CARGO_PKG_VERSION"));
const LABEL_COLOR: &str = "d4c5f9";
const LABEL_DESCRIPTION: &str =
    "Auto-applied by pr-hygiene: unresolved review feedback awaits the author.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelDiff {
    pub to_add: Vec<u64>,
    pub to_remove: Vec<u64>,
}

/// Compute the set of PRs that need the label added vs removed.
/// Pure function; testable without HTTP.
pub fn compute_diff<'a, I>(prs: I, label: &str) -> LabelDiff
where
    I: IntoIterator<Item = &'a AnalyzedPr>,
{
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();
    for pr in prs {
        let has_label = pr.raw.labels.iter().any(|l| l.eq_ignore_ascii_case(label));
        match (pr.needs_author_action, has_label) {
            (true, false) => to_add.push(pr.raw.number),
            (false, true) => to_remove.push(pr.raw.number),
            _ => {}
        }
    }
    to_add.sort_unstable();
    to_remove.sort_unstable();
    LabelDiff { to_add, to_remove }
}

pub struct Labeler {
    client: Client,
    base: String,
}

impl Labeler {
    pub fn new(token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("token contains invalid header characters")?,
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .context("building HTTP client")?;
        Ok(Self {
            client,
            base: REST_API.to_string(),
        })
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_base(mut self, base: impl Into<String>) -> Self {
        self.base = base.into();
        self
    }

    pub async fn ensure_label(&self, owner: &str, name: &str, label: &str) -> Result<()> {
        let get_url = format!("{}/repos/{owner}/{name}/labels/{label}", self.base);
        let res = self
            .client
            .get(&get_url)
            .send()
            .await
            .with_context(|| format!("GET {get_url}"))?;
        if res.status().is_success() {
            return Ok(());
        }
        if res.status() != StatusCode::NOT_FOUND {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("checking label {label}: HTTP {status}: {body}");
        }
        let create_url = format!("{}/repos/{owner}/{name}/labels", self.base);
        let body = json!({
            "name": label,
            "color": LABEL_COLOR,
            "description": LABEL_DESCRIPTION,
        });
        let res = self
            .client
            .post(&create_url)
            .json(&body)
            .send()
            .await
            .with_context(|| format!("POST {create_url}"))?;
        if !res.status().is_success() && res.status() != StatusCode::UNPROCESSABLE_ENTITY {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("creating label {label}: HTTP {status}: {body}");
        }
        Ok(())
    }

    pub async fn add_label(
        &self,
        owner: &str,
        name: &str,
        pr_number: u64,
        label: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/repos/{owner}/{name}/issues/{pr_number}/labels",
            self.base
        );
        let res = self
            .client
            .post(&url)
            .json(&json!({ "labels": [label] }))
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("adding label to PR #{pr_number}: HTTP {status}: {body}");
        }
        Ok(())
    }

    pub async fn remove_label(
        &self,
        owner: &str,
        name: &str,
        pr_number: u64,
        label: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/repos/{owner}/{name}/issues/{pr_number}/labels/{label}",
            self.base
        );
        let res = self
            .client
            .delete(&url)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;
        if !res.status().is_success() && res.status() != StatusCode::NOT_FOUND {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("removing label from PR #{pr_number}: HTTP {status}: {body}");
        }
        Ok(())
    }

    pub async fn apply(
        &self,
        owner: &str,
        name: &str,
        diff: &LabelDiff,
        label: &str,
    ) -> Result<()> {
        if diff.to_add.is_empty() && diff.to_remove.is_empty() {
            return Ok(());
        }
        self.ensure_label(owner, name, label).await?;
        for pr in &diff.to_add {
            tracing::info!(pr, label, "adding label");
            self.add_label(owner, name, *pr, label).await?;
        }
        for pr in &diff.to_remove {
            tracing::info!(pr, label, "removing label");
            self.remove_label(owner, name, *pr, label).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Mergeable, RawPr};
    use chrono::{TimeZone, Utc};

    fn pr(number: u64, labels: Vec<&str>, needs_action: bool) -> AnalyzedPr {
        AnalyzedPr {
            raw: RawPr {
                number,
                title: "t".into(),
                url: "u".into(),
                author: Some("alice".into()),
                created_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
                updated_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
                is_draft: false,
                mergeable: Mergeable::Mergeable,
                labels: labels.into_iter().map(|s| s.to_string()).collect(),
                last_commit: None,
                reviews: vec![],
                threads: vec![],
            },
            unresolved_threads: vec![],
            days_since_author_push: 0.0,
            days_since_last_reviewer_activity: 0.0,
            needs_author_action: needs_action,
            has_merge_conflict: false,
            changes_requested: false,
            ci_failing: false,
        }
    }

    #[test]
    fn add_when_action_needed_and_label_absent() {
        let prs = vec![pr(1, vec![], true)];
        let diff = compute_diff(&prs, "needs-author-action");
        assert_eq!(diff.to_add, vec![1]);
        assert!(diff.to_remove.is_empty());
    }

    #[test]
    fn remove_when_action_not_needed_but_label_present() {
        let prs = vec![pr(1, vec!["needs-author-action"], false)];
        let diff = compute_diff(&prs, "needs-author-action");
        assert!(diff.to_add.is_empty());
        assert_eq!(diff.to_remove, vec![1]);
    }

    #[test]
    fn noop_when_state_matches() {
        let prs = vec![
            pr(1, vec!["needs-author-action"], true),
            pr(2, vec![], false),
        ];
        let diff = compute_diff(&prs, "needs-author-action");
        assert!(diff.to_add.is_empty());
        assert!(diff.to_remove.is_empty());
    }

    #[test]
    fn case_insensitive_label_match() {
        let prs = vec![pr(1, vec!["Needs-Author-Action"], false)];
        let diff = compute_diff(&prs, "needs-author-action");
        assert_eq!(diff.to_remove, vec![1]);
    }

    #[test]
    fn mixed_set() {
        let prs = vec![
            pr(1, vec![], true),                       // add
            pr(2, vec!["needs-author-action"], false), // remove
            pr(3, vec!["needs-author-action"], true),  // no-op
            pr(4, vec![], false),                      // no-op
        ];
        let diff = compute_diff(&prs, "needs-author-action");
        assert_eq!(diff.to_add, vec![1]);
        assert_eq!(diff.to_remove, vec![2]);
    }
}
