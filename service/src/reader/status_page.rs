//! [`StatusPage`]: the review system's public status page, read as
//! `pr_review/telemetry.py`'s `fetch` reads it.
//!
//! The page is a third party's, and never authoritative: whatever goes
//! wrong, the answer is "nothing to say" (`None`), and the engine carries
//! on. It is read through a client of its own that never carries the App's
//! token, follows no redirect, and gives up after Python's ten seconds.

use super::USER_AGENT;
use pr_hygiene_engine::pycompat::ops::py_eq;
use pr_hygiene_engine::pycompat::{py_loads, PyInt, PyValue};
use reqwest::StatusCode;
use std::fmt;
use std::time::Duration;
use tokio::runtime::{Handle, RuntimeFlavor};

/// `telemetry.SOURCE`.
pub const SOURCE: &str = "https://thepastaclaw.github.io/review-system/data/status.json";

/// `telemetry.LIMIT`: the most of the page read. Python reads one byte
/// more, so that a larger page reads as JSON cut short and is refused.
pub const LIMIT: usize = 2 * 1024 * 1024;

/// `fetch`'s timeout.
const TIMEOUT: Duration = Duration::from_secs(10);

/// The status page, read on the service's runtime from the engine's
/// blocking thread, as [`HttpTransport`](super::HttpTransport) is.
pub struct StatusPage {
    client: reqwest::Client,
    runtime: Handle,
    url: String,
}

impl fmt::Debug for StatusPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusPage")
            .field("url", &self.url)
            .finish_non_exhaustive()
    }
}

impl StatusPage {
    pub fn new(runtime: Handle) -> anyhow::Result<Self> {
        Self::at(runtime, SOURCE)
    }

    /// Another URL, for testing the read itself.
    #[cfg(test)]
    pub(crate) fn with_url(runtime: Handle, url: &str) -> anyhow::Result<Self> {
        Self::at(runtime, url)
    }

    fn at(runtime: Handle, url: &str) -> anyhow::Result<Self> {
        anyhow::ensure!(
            runtime.runtime_flavor() == RuntimeFlavor::MultiThread,
            "the status page is read on a multi-thread runtime, from the engine's thread"
        );
        let client = reqwest::Client::builder()
            .timeout(TIMEOUT)
            // `telemetry.py` refuses redirects.
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(USER_AGENT)
            .build()?;
        Ok(Self {
            client,
            runtime,
            url: url.to_owned(),
        })
    }

    /// `telemetry.fetch()`: the page's payload, or `None`. One attempt; any
    /// problem at all is `None`. Blocks the calling thread, which must be
    /// the engine's blocking thread, never a task on the runtime.
    pub fn fetch(&self) -> PyValue {
        self.runtime.block_on(self.read()).unwrap_or(PyValue::None)
    }

    async fn read(&self) -> Option<PyValue> {
        let mut response = self.client.get(&self.url).send().await.ok()?;
        if response.status() != StatusCode::OK {
            return None;
        }
        // `response.read(LIMIT + 1)`.
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.ok()? {
            let room = LIMIT + 1 - bytes.len();
            bytes.extend_from_slice(&chunk[..chunk.len().min(room)]);
            if bytes.len() > LIMIT {
                break;
            }
        }
        // `.decode('utf-8', 'replace')`, then `json.loads`.
        let payload = py_loads(&String::from_utf8_lossy(&bytes)).ok()?;
        let PyValue::Dict(fields) = &payload else {
            return None;
        };
        let version = fields.get("schema_version")?;
        py_eq(version, &PyValue::Int(PyInt::from(1))).then_some(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::mock::{self, blocking, json_answer, Seen};
    use axum::http::{header, StatusCode as Code};
    use axum::response::IntoResponse;

    async fn fetch(url: String) -> PyValue {
        let page = StatusPage::with_url(Handle::current(), &url).unwrap();
        blocking(move || page.fetch()).await
    }

    fn is_dict(value: &PyValue) -> bool {
        matches!(value, PyValue::Dict(_))
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn the_payload_is_read_without_any_credential() {
        let page = mock::serve(|_: &Seen| {
            json_answer(r#"{"schema_version": 1, "data_as_of": "2026-10-02T12:00:00Z"}"#)
        })
        .await;
        assert!(is_dict(&fetch(format!("{}/status.json", page.url)).await));
        let seen = &page.seen()[0];
        assert_eq!(seen.path(), "/status.json");
        assert!(
            !seen.headers.contains_key(header::AUTHORIZATION),
            "the status page never sees a token"
        );
        assert!(!seen.headers.contains_key(header::COOKIE));
    }

    /// `payload.get('schema_version') != 1`, as Python compares: `1.0` and
    /// `true` are 1, anything else or nothing is not.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn only_schema_version_one_is_read() {
        for (body, read) in [
            (r#"{"schema_version": 1.0}"#, true),
            (r#"{"schema_version": true}"#, true),
            (r#"{"schema_version": 2}"#, false),
            (r#"{"schema_version": "1"}"#, false),
            (r#"{"data_as_of": "x"}"#, false),
            (r#"[{"schema_version": 1}]"#, false),
            (r#"{"schema_version": 1"#, false),
            ("", false),
        ] {
            let page = mock::serve(move |_: &Seen| json_answer(body)).await;
            assert_eq!(is_dict(&fetch(page.url.clone()).await), read, "{body}");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn any_problem_is_nothing_to_say() {
        let refused = mock::serve(|_: &Seen| Code::NOT_FOUND.into_response()).await;
        assert!(matches!(fetch(refused.url.clone()).await, PyValue::None));
        let accepted =
            mock::serve(|_: &Seen| (Code::ACCEPTED, r#"{"schema_version": 1}"#).into_response())
                .await;
        assert!(matches!(fetch(accepted.url.clone()).await, PyValue::None));
        assert!(matches!(
            fetch("http://127.0.0.1:9/".into()).await,
            PyValue::None
        ));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_redirect_is_not_followed() {
        let page = mock::serve(|seen: &Seen| {
            if seen.path() == "/moved" {
                return json_answer(r#"{"schema_version": 1}"#);
            }
            let to = format!("{}/moved", seen.origin());
            (Code::FOUND, [(header::LOCATION, to)]).into_response()
        })
        .await;
        assert!(matches!(fetch(page.url.clone()).await, PyValue::None));
        assert_eq!(page.seen().len(), 1);
    }

    /// A page over the limit reads as JSON cut short, unless what fits is
    /// itself the whole document: Python reads one byte past the limit and
    /// parses what it read.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_page_over_the_limit_is_cut_where_python_cuts_it() {
        let large = format!(r#"{{"schema_version": 1, "pad": "{}"}}"#, "x".repeat(LIMIT));
        let page = mock::serve(move |_: &Seen| json_answer(&large)).await;
        assert!(matches!(fetch(page.url.clone()).await, PyValue::None));
        let padded = format!(r#"{{"schema_version": 1}}{}"#, " ".repeat(LIMIT));
        let page = mock::serve(move |_: &Seen| json_answer(&padded)).await;
        assert!(is_dict(&fetch(page.url.clone()).await));
    }
}
