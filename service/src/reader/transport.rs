//! [`HttpTransport`]: the engine's calls made over HTTPS, as `gh api` made
//! them for Python.
//!
//! The same requests: a REST route under the API's origin, its query as
//! written, a body exactly as Python's `json.dumps` wrote it to `gh`'s
//! stdin; GraphQL as one `POST /graphql`. The same headers: those `gh api`
//! sends (`GH_DEBUG=api`, gh 2.101), the API version above all, since REST
//! answers change shape between versions:
//!
//! ```text
//! Accept: */*
//! Authorization: token <installation token>
//! Content-Type: application/json; charset=utf-8
//! Time-Zone: UTC
//! X-GitHub-Api-Version: 2022-11-28
//! ```
//!
//! `gh` sends the machine's own time zone, which GitHub reads only when
//! creating commits; the service's clock is UTC. Its user agent names `gh`;
//! this one names the service.
//!
//! The same pages: a paginated call follows each answer's `Link:
//! rel="next"`, adding `per_page=100` as `gh` does when the route has none,
//! and answers with the pages in order, which the client reads as the list
//! `gh api --paginate --slurp` prints. A next page is followed only on the
//! same origin and the same listing.
//!
//! Failures are sorted by what happened, not by what an error's text
//! contains: a gateway error (502, 503, 504), a timeout, a connection reset
//! or closed early, or an answer cut short or unreadable is worth one more
//! try; anything GitHub refused (every 4xx, and every other status) is not.
//! A GraphQL answer that carries errors is a failure, as `gh` reports it,
//! with the answer as its body: the client decides whether the data in it
//! is an answer. Each failure reports `gh`'s exit status, 1, which the
//! client quotes as Python did, and says what went wrong in its own words.
//!
//! No redirect is followed: a request and its token go to the URL asked for
//! and nowhere else.

use super::app::TokenSource;
use super::{github_client, API_URL, API_VERSION};
use pr_hygiene_engine::evidence::replay::gh_arguments;
use pr_hygiene_engine::evidence::{Call, Failure, Method, Reply, Transport, TransportError};
use pr_hygiene_engine::pycompat::text::py_strip;
use pr_hygiene_engine::pycompat::{py_loads, PyErr, PyValue};
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, LINK};
use reqwest::{StatusCode, Url};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

/// How long one request may take, its whole answer read. Each page of a
/// paginated call has its own.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// The largest answer read. A page of a hundred changed files, patches
/// included, is a few megabytes at most.
pub const MAX_ANSWER_BYTES: usize = 32 * 1024 * 1024;

/// The most pages one call reads: a hundred thousand items, where the
/// largest listing the engine reads holds a few thousand. A listing that
/// never ends is an error, not a loop.
pub const MAX_PAGES: usize = 1000;

/// `gh`'s exit status for every failure it reports.
const GH_FAILED: i32 = 1;

/// The headers `gh api` sends with every request, its token aside.
const ACCEPT_ANY: &str = "*/*";
const JSON_CONTENT: &str = "application/json; charset=utf-8";
const TIME_ZONE: &str = "UTC";

/// The engine's [`Transport`] over HTTPS, on the service's runtime.
///
/// # The engine's thread and the service's runtime
///
/// The engine is synchronous: a reconciliation runs on a blocking thread of
/// its own, in `tokio::task::spawn_blocking`. [`Transport::call`] blocks
/// that thread on [`Handle::block_on`] while the request runs on the
/// service's runtime, through its async HTTP client: there is no second
/// runtime and no blocking client.
///
/// So `call` must only ever be made from such a thread. Made from a task on
/// the runtime itself, `block_on` panics at once, as tokio refuses to block
/// one of its workers: a mistake shows on its first call, never as a
/// deadlock.
pub struct HttpTransport {
    client: reqwest::Client,
    runtime: Handle,
    tokens: Arc<dyn TokenSource>,
    /// The API's origin, with no path: every request goes to it.
    base: Url,
}

impl fmt::Debug for HttpTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpTransport")
            .field("base", &self.base.as_str())
            .finish_non_exhaustive()
    }
}

impl HttpTransport {
    /// Calls to GitHub's API on `runtime`, each with a token from `tokens`.
    pub fn new(runtime: Handle, tokens: Arc<dyn TokenSource>) -> anyhow::Result<Self> {
        Self::at(runtime, tokens, API_URL, REQUEST_TIMEOUT)
    }

    /// Another origin and timeout, for testing the calls themselves;
    /// production code has no way to send the token anywhere but GitHub.
    #[cfg(test)]
    pub(crate) fn with_origin(
        runtime: Handle,
        tokens: Arc<dyn TokenSource>,
        origin: &str,
        timeout: Duration,
    ) -> anyhow::Result<Self> {
        Self::at(runtime, tokens, origin, timeout)
    }

    fn at(
        runtime: Handle,
        tokens: Arc<dyn TokenSource>,
        origin: &str,
        timeout: Duration,
    ) -> anyhow::Result<Self> {
        let base = Url::parse(origin)?;
        anyhow::ensure!(
            base.origin().ascii_serialization() == origin,
            "{origin} is not an origin"
        );
        Ok(Self {
            client: github_client(timeout)?,
            runtime,
            tokens,
            base,
        })
    }

    /// The URL of `path`, a route under the origin with its query as
    /// written, or a refusal when the URL would not carry it as written:
    /// `..` or `.` resolved away, a character the parser re-encodes, a
    /// fragment, another authority.
    fn url(&self, path: &str) -> Result<Url, TransportError> {
        let refuse = || {
            TransportError::Refused(format!(
                "A route that would not reach GitHub as written: {path}"
            ))
        };
        let origin = self.base.origin().ascii_serialization();
        let url = Url::parse(&format!("{origin}/{path}")).map_err(|_| refuse())?;
        let (route, query) = match path.split_once('?') {
            Some((route, query)) => (route, Some(query)),
            None => (path, None),
        };
        let exact = url.origin() == self.base.origin()
            && url.username().is_empty()
            && url.password().is_none()
            && url.fragment().is_none()
            && url.path().strip_prefix('/') == Some(route)
            && url.query() == query;
        if exact {
            Ok(url)
        } else {
            Err(refuse())
        }
    }

    async fn answer(&self, call: &Call) -> Result<Reply, TransportError> {
        // The body `gh` was given on stdin, byte for byte.
        let (_, body) =
            gh_arguments(call).map_err(|error| TransportError::Refused(error.to_string()))?;
        match call {
            Call::Graphql { .. } => {
                let answer = self.send(Method::Post, self.url("graphql")?, body).await?;
                graphql(answer)
            }
            Call::Rest {
                method,
                path,
                paginate: false,
                ..
            } => {
                let answer = self.send(*method, self.url(path)?, body).await?;
                Ok(Reply::Text(completed(answer)?))
            }
            Call::Rest {
                method,
                path,
                paginate: true,
                ..
            } => {
                if *method != Method::Get {
                    return Err(TransportError::Refused(format!(
                        "Only a GET is read page by page: {call}"
                    )));
                }
                self.pages(path, body).await
            }
        }
    }

    /// Every page of a listing, from `path` on. Only the first request
    /// carries `body`, as with `gh`.
    async fn pages(&self, path: &str, mut body: Option<String>) -> Result<Reply, TransportError> {
        let first = self.url(&with_per_page(path))?;
        let mut url = first.clone();
        let mut pages = Vec::new();
        loop {
            let answer = self.send(Method::Get, url, body.take()).await?;
            let next = answer.next.clone();
            pages.push(completed(answer)?);
            let Some(next) = next else {
                return Ok(Reply::Pages(pages));
            };
            if pages.len() >= MAX_PAGES {
                return Err(failure(
                    false,
                    String::new(),
                    format!("a listing of more than {MAX_PAGES} pages"),
                )
                .into());
            }
            url = self.next_page(&first, &next)?;
        }
    }

    /// The page a `Link: rel="next"` names, if it may be followed: on the
    /// same origin, carrying no credentials, and the same listing as the
    /// first page.
    fn next_page(&self, first: &Url, link: &str) -> Result<Url, TransportError> {
        let refuse =
            |why: String| TransportError::Refused(format!("Next page not followed: {why}"));
        let next = Url::parse(link).map_err(|_| refuse("not an absolute URL".into()))?;
        if next.origin() != self.base.origin() {
            return Err(refuse(format!(
                "on {}, not {}",
                next.origin().ascii_serialization(),
                self.base.origin().ascii_serialization()
            )));
        }
        if !next.username().is_empty() || next.password().is_some() || next.fragment().is_some() {
            return Err(refuse("a URL with credentials or a fragment".into()));
        }
        if !same_listing(first.path(), next.path()) {
            return Err(refuse(format!(
                "{} is not the listing {} asked for",
                next.path(),
                first.path()
            )));
        }
        Ok(next)
    }

    /// One request, and its whole answer, whatever its status; a failure
    /// when there is no complete answer to read.
    async fn send(
        &self,
        method: Method,
        url: Url,
        body: Option<String>,
    ) -> Result<Answer, Failure> {
        let token = self.tokens.token().await.map_err(|error| {
            failure(
                error.transient(),
                String::new(),
                format!("no installation token: {error}"),
            )
        })?;
        let mut authorization = HeaderValue::from_str(&format!("token {}", token.expose()))
            .map_err(|_| {
                failure(
                    false,
                    String::new(),
                    "an installation token unfit for a header",
                )
            })?;
        authorization.set_sensitive(true);
        let mut request = self
            .client
            .request(reqwest_method(method), url)
            .header(ACCEPT, ACCEPT_ANY)
            .header(AUTHORIZATION, authorization)
            .header(CONTENT_TYPE, JSON_CONTENT)
            .header("Time-Zone", TIME_ZONE)
            .header("X-GitHub-Api-Version", API_VERSION);
        if let Some(body) = body {
            request = request.body(body);
        }
        let mut response = request
            .send()
            .await
            .map_err(|error| failure(is_transient(&error), String::new(), describe(error)))?;
        let status = response.status();
        // `gh` reads the first `Link` header, and the first `next` in it.
        let next = response
            .headers()
            .get(LINK)
            .and_then(|value| value.to_str().ok())
            .and_then(next_link)
            .map(str::to_owned);
        let too_large = || {
            failure(
                false,
                String::new(),
                format!("an answer larger than {MAX_ANSWER_BYTES} bytes"),
            )
        };
        if response
            .content_length()
            .is_some_and(|n| n > MAX_ANSWER_BYTES as u64)
        {
            return Err(too_large());
        }
        let mut bytes = Vec::new();
        loop {
            match response.chunk().await {
                Ok(Some(chunk)) => {
                    if bytes.len() + chunk.len() > MAX_ANSWER_BYTES {
                        return Err(too_large());
                    }
                    bytes.extend_from_slice(&chunk);
                }
                Ok(None) => break,
                // Cut short: the connection closed or stalled mid-answer.
                Err(error) => return Err(failure(true, String::new(), describe(error))),
            }
        }
        let text = String::from_utf8(bytes)
            .map_err(|_| failure(true, String::new(), "an answer that is not UTF-8"))?;
        Ok(Answer { status, text, next })
    }
}

impl Transport for HttpTransport {
    /// Blocks the calling thread until the call is answered. Only ever
    /// called from a blocking thread: see [`HttpTransport`].
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        self.runtime.block_on(self.answer(call))
    }
}

/// What came back for one request.
struct Answer {
    status: StatusCode,
    text: String,
    /// The page after this one, as the `Link` header names it.
    next: Option<String>,
}

fn failure(transient: bool, body: String, detail: impl Into<String>) -> Failure {
    Failure {
        transient,
        status: Some(GH_FAILED),
        body,
        detail: detail.into(),
    }
}

/// The text of a successful answer; a failure for any other status, or for
/// an answer that is not JSON.
fn completed(answer: Answer) -> Result<String, Failure> {
    if !answer.status.is_success() {
        return Err(refused(answer));
    }
    readable(&answer.text)?;
    Ok(answer.text)
}

/// A complete answer GitHub gave with a status other than success.
fn refused(answer: Answer) -> Failure {
    let code = answer.status.as_u16();
    let detail = if answer.status.is_redirection() {
        format!("a redirect, not followed (HTTP {code})")
    } else {
        match message(&answer.text) {
            Some(message) => format!("{message} (HTTP {code})"),
            None => format!("HTTP {code}"),
        }
    };
    failure(matches!(code, 502..=504), answer.text, detail)
}

/// GitHub's own words for a refusal: the `message` of its answer.
fn message(text: &str) -> Option<String> {
    match py_loads(text) {
        Ok(PyValue::Dict(fields)) => match fields.get("message") {
            Some(PyValue::Str(message)) if !message.is_empty() => Some(message.clone()),
            _ => None,
        },
        _ => None,
    }
}

/// An answer's JSON, read as the client will read it: `None` when it holds
/// nothing, or when reading it raises something other than a `ValueError`,
/// which the client then raises itself. Text that is not JSON is an answer
/// cut short or garbled on the way, worth one more try.
fn readable(text: &str) -> Result<Option<PyValue>, Failure> {
    if py_strip(text).is_empty() {
        return Ok(None);
    }
    match py_loads(text) {
        Ok(value) => Ok(Some(value)),
        Err(PyErr::Value(_)) => Err(failure(
            true,
            text.to_owned(),
            "an answer that is not JSON, cut short or garbled",
        )),
        Err(_) => Ok(None),
    }
}

/// A GraphQL answer: a failure when it carries errors, as `gh` reports one,
/// with the answer as its body for the client to decide on.
fn graphql(answer: Answer) -> Result<Reply, TransportError> {
    if !answer.status.is_success() {
        return Err(refused(answer).into());
    }
    let errors = readable(&answer.text)?.as_ref().and_then(graphql_errors);
    match errors {
        Some(messages) => Err(failure(false, answer.text, messages).into()),
        None => Ok(Reply::Text(answer.text)),
    }
}

/// The messages of a GraphQL answer's errors, one per line, when it
/// carries any.
fn graphql_errors(answer: &PyValue) -> Option<String> {
    let PyValue::Dict(fields) = answer else {
        return None;
    };
    match fields.get("errors") {
        Some(PyValue::List(errors)) if !errors.is_empty() => Some(
            errors
                .iter()
                .filter_map(|error| match error {
                    PyValue::Dict(error) => match error.get("message") {
                        Some(PyValue::Str(message)) => Some(message.as_str()),
                        _ => None,
                    },
                    PyValue::Str(message) => Some(message.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        Some(PyValue::Str(error)) if !error.is_empty() => Some(error.clone()),
        _ => None,
    }
}

/// `path` with `per_page=100`, as `gh api --paginate` asks for a listing
/// whose route names no page size.
fn with_per_page(path: &str) -> String {
    match path.split_once('?') {
        Some((_, query)) if query.split('&').any(|pair| pair.starts_with("per_page=")) => {
            path.to_owned()
        }
        Some((_, "")) => format!("{path}per_page=100"),
        Some(_) => format!("{path}&per_page=100"),
        None => format!("{path}?per_page=100"),
    }
}

/// The first `rel="next"` URL of a `Link` header: `<url>; rel="next", …`.
fn next_link(header: &str) -> Option<&str> {
    let mut rest = header;
    while let Some(start) = rest.find('<') {
        let after = &rest[start + 1..];
        let end = after.find('>')?;
        let (url, tail) = (&after[..end], &after[end + 1..]);
        let rel = tail
            .strip_prefix(';')
            .map(str::trim_start)
            .and_then(|params| params.strip_prefix("rel=\""))
            .and_then(|rel| rel.split('"').next());
        if rel == Some("next") {
            return Some(url);
        }
        rest = tail;
    }
    None
}

/// Whether `next` is a page of the listing `first` is: the same path, or,
/// for a repository's listing `/repos/{owner}/{repo}/{route}`, the same
/// route under that repository's id, `/repositories/{id}/{route}`, which
/// is how GitHub links a repository's later pages.
fn same_listing(first: &str, next: &str) -> bool {
    if first == next {
        return true;
    }
    let Some(mut parts) = first
        .strip_prefix("/repos/")
        .map(|rest| rest.splitn(3, '/'))
    else {
        return false;
    };
    let (Some(owner), Some(name), Some(route)) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    if let Some(rest) = next.strip_prefix("/repositories/") {
        return rest.split_once('/').is_some_and(|(id, tail)| {
            !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) && tail == route
        });
    }
    let Some(mut parts) = next.strip_prefix("/repos/").map(|rest| rest.splitn(3, '/')) else {
        return false;
    };
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(o), Some(n), Some(r))
            if o.eq_ignore_ascii_case(owner) && n.eq_ignore_ascii_case(name) && r == route
    )
}

fn reqwest_method(method: Method) -> reqwest::Method {
    match method {
        Method::Get => reqwest::Method::GET,
        Method::Post => reqwest::Method::POST,
        Method::Put => reqwest::Method::PUT,
        Method::Patch => reqwest::Method::PATCH,
        Method::Delete => reqwest::Method::DELETE,
    }
}

/// Whether a request that got no complete answer is worth one more try: it
/// timed out, its answer was cut short, or its connection broke after it
/// was made. A connection that could not be made at all — refused, a name
/// that does not resolve, a TLS handshake that failed — is not.
pub(super) fn is_transient(error: &reqwest::Error) -> bool {
    use std::io::ErrorKind;
    if error.is_timeout() || error.is_body() || error.is_decode() {
        return true;
    }
    let mut cause = std::error::Error::source(error);
    while let Some(inner) = cause {
        if let Some(io) = inner.downcast_ref::<std::io::Error>() {
            if matches!(
                io.kind(),
                ErrorKind::ConnectionReset
                    | ErrorKind::ConnectionAborted
                    | ErrorKind::BrokenPipe
                    | ErrorKind::UnexpectedEof
            ) {
                return true;
            }
        }
        cause = inner.source();
    }
    error.is_request() && !error.is_connect()
}

/// What went wrong, with its causes, and without the URL: nothing a request
/// carried is repeated.
pub(super) fn describe(error: reqwest::Error) -> String {
    let error = error.without_url();
    let mut text = error.to_string();
    let mut cause = std::error::Error::source(&error);
    while let Some(inner) = cause {
        text.push_str(": ");
        text.push_str(&inner.to_string());
        cause = inner.source();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::mock::{self, blocking, fixed_token, json_answer, Seen};
    use axum::http::{header, StatusCode as Code};
    use axum::response::IntoResponse;
    use pr_hygiene_engine::evidence::queries;
    use pr_hygiene_engine::evidence::{Client, NoSleep, ReadError};
    use pr_hygiene_engine::pycompat::{PyDict, PyInt};
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn transport(origin: &str) -> HttpTransport {
        transport_with_timeout(origin, Duration::from_secs(5))
    }

    fn transport_with_timeout(origin: &str, timeout: Duration) -> HttpTransport {
        HttpTransport::with_origin(Handle::current(), fixed_token("ghs_test"), origin, timeout)
            .unwrap()
    }

    fn get(path: &str) -> Call {
        Call::Rest {
            method: Method::Get,
            path: path.into(),
            body: None,
            paginate: false,
        }
    }

    fn graphql_call(query: &str) -> Call {
        let mut variables = PyDict::new();
        variables.insert("owner".into(), PyValue::Str("dashpay".into()));
        variables.insert("repo".into(), PyValue::Str("platform".into()));
        variables.insert("number".into(), PyValue::Int(PyInt::from(7)));
        variables.insert("cursor".into(), PyValue::None);
        Call::Graphql {
            query: query.into(),
            variables: PyValue::Dict(variables),
        }
    }

    /// Calls made as the engine makes them: through its client, with the
    /// retry's wait left out, on a blocking thread.
    async fn run(
        transport: HttpTransport,
        f: impl FnOnce(&mut Client<HttpTransport>) -> Result<PyValue, ReadError> + Send + 'static,
    ) -> Result<PyValue, ReadError> {
        blocking(move || f(&mut Client::with_sleep(transport, NoSleep))).await
    }

    async fn call(transport: HttpTransport, call: Call) -> Result<Reply, TransportError> {
        blocking(move || {
            let mut transport = transport;
            transport.call(&call)
        })
        .await
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn requests_carry_the_headers_gh_sends() {
        let github = mock::serve(|_: &Seen| json_answer("{}")).await;
        let reply = call(
            transport(&github.url),
            get("repos/dashpay/platform/pulls/7"),
        )
        .await;
        assert_eq!(reply, Ok(Reply::Text("{}".into())));
        let seen = github.seen();
        let headers = &seen[0].headers;
        assert_eq!(seen[0].path_and_query, "/repos/dashpay/platform/pulls/7");
        assert_eq!(headers["accept"], "*/*");
        assert_eq!(headers["authorization"], "token ghs_test");
        assert_eq!(headers["content-type"], "application/json; charset=utf-8");
        assert_eq!(headers["time-zone"], "UTC");
        assert_eq!(headers["x-github-api-version"], "2022-11-28");
        assert_eq!(headers["user-agent"], "pr-hygiene-service");
    }

    /// The body is what Python gave `gh` on stdin: `json.dumps` of the
    /// document, default separators, keys in the engine's order.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_graphql_query_is_one_post_with_pythons_body() {
        let github = mock::serve(|_: &Seen| json_answer(r#"{"data": {"x": 1}}"#)).await;
        let reply = call(transport(&github.url), graphql_call(queries::THREADS)).await;
        assert_eq!(reply, Ok(Reply::Text(r#"{"data": {"x": 1}}"#.into())));
        let seen = &github.seen()[0];
        assert_eq!(
            (seen.method.as_str(), seen.path_and_query.as_str()),
            ("POST", "/graphql")
        );
        let (_, stdin) = gh_arguments(&graphql_call(queries::THREADS)).unwrap();
        assert_eq!(Some(seen.body.clone()), stdin);
        assert!(seen.body.starts_with(r#"{"query": "query($owner:String!"#));
    }

    /// Three pages, linked as GitHub links them — the later ones under the
    /// repository's id — read as the list `gh --paginate --slurp` prints.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn every_page_is_read_in_order() {
        let github = mock::serve(|seen: &Seen| {
            let origin = seen.origin();
            let page = |body: &str, next: Option<&str>| {
                let mut response = json_answer(body);
                if let Some(next) = next {
                    let link = format!(
                        "<{origin}{next}>; rel=\"next\", <{origin}/repositories/9/pulls?state=open&per_page=100&page=3>; rel=\"last\""
                    );
                    response
                        .headers_mut()
                        .insert(header::LINK, link.parse().unwrap());
                }
                response
            };
            match seen.path_and_query.as_str() {
                "/repos/dashpay/platform/pulls?state=open&per_page=100" => page(
                    r#"[{"number": 1}]"#,
                    Some("/repositories/9/pulls?state=open&per_page=100&page=2"),
                ),
                "/repositories/9/pulls?state=open&per_page=100&page=2" => page(
                    r#"[{"number": 2}]"#,
                    Some("/repositories/9/pulls?state=open&per_page=100&page=3"),
                ),
                "/repositories/9/pulls?state=open&per_page=100&page=3" => page(r#"[{"number": 3}]"#, None),
                _ => Code::NOT_FOUND.into_response(),
            }
        })
        .await;
        let items = run(transport(&github.url), |client| {
            client
                .pages("repos/dashpay/platform/pulls?state=open")
                .map(|items| PyValue::List(items.into()))
        })
        .await
        .unwrap();
        let numbers: Vec<String> = match items {
            PyValue::List(items) => items
                .iter()
                .map(|item| match item {
                    PyValue::Dict(fields) => format!("{:?}", fields.get("number")),
                    other => format!("{other:?}"),
                })
                .collect(),
            other => panic!("{other:?}"),
        };
        assert_eq!(numbers.len(), 3, "{numbers:?}");
        assert_eq!(github.seen().len(), 3);

        // The transport's own answer: the pages, each its own body. Asked
        // with no page size, it asks for a hundred, as `gh` does.
        let reply = call(
            transport(&github.url),
            Call::Rest {
                method: Method::Get,
                path: "repos/dashpay/platform/pulls?state=open".into(),
                body: None,
                paginate: true,
            },
        )
        .await;
        assert_eq!(
            reply,
            Ok(Reply::Pages(vec![
                r#"[{"number": 1}]"#.into(),
                r#"[{"number": 2}]"#.into(),
                r#"[{"number": 3}]"#.into()
            ]))
        );
    }

    #[test]
    fn a_page_size_is_added_as_gh_adds_it() {
        assert_eq!(
            with_per_page("repos/a/b/pulls"),
            "repos/a/b/pulls?per_page=100"
        );
        assert_eq!(
            with_per_page("repos/a/b/pulls?state=open"),
            "repos/a/b/pulls?state=open&per_page=100"
        );
        assert_eq!(
            with_per_page("repos/a/b/pulls?per_page=5"),
            "repos/a/b/pulls?per_page=5"
        );
        assert_eq!(
            with_per_page("repos/a/b/pulls?"),
            "repos/a/b/pulls?per_page=100"
        );
    }

    #[test]
    fn the_next_link_is_the_first_marked_next() {
        let header = r#"<https://api.github.com/x?page=1>; rel="prev", <https://api.github.com/x?page=3>; rel="next", <https://api.github.com/x?page=9>; rel="last""#;
        assert_eq!(next_link(header), Some("https://api.github.com/x?page=3"));
        assert_eq!(
            next_link(r#"<https://api.github.com/x?page=9>; rel="last""#),
            None
        );
        assert_eq!(next_link("garbage"), None);
    }

    #[test]
    fn a_next_page_must_be_the_same_listing() {
        let first = "/repos/dashpay/platform/pulls";
        assert!(same_listing(first, "/repositories/424232911/pulls"));
        assert!(same_listing(first, "/repos/DashPay/Platform/pulls"));
        assert!(!same_listing(first, "/repositories/424232911/issues"));
        assert!(!same_listing(first, "/repositories/x1/pulls"));
        assert!(!same_listing(first, "/repos/dashpay/other/pulls"));
        assert!(!same_listing(first, "/user/repos"));
        assert!(same_listing("/user/repos", "/user/repos"));
        assert!(!same_listing("/user/repos", "/user/orgs"));
    }

    /// A next link to another origin would carry the token there. It is
    /// refused, never fetched, and stops the read.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_next_link_to_another_origin_is_refused_and_never_fetched() {
        let elsewhere = mock::serve(|_: &Seen| json_answer("[]")).await;
        let foreign = format!("{}/repositories/9/pulls?page=2", elsewhere.url);
        let github = mock::serve(move |_: &Seen| {
            let mut response = json_answer("[1]");
            let link = format!("<{foreign}>; rel=\"next\"");
            response
                .headers_mut()
                .insert(header::LINK, link.parse().unwrap());
            response
        })
        .await;
        let error = run(transport(&github.url), |client| {
            client
                .pages("repos/dashpay/platform/pulls")
                .map(|_| PyValue::None)
        })
        .await
        .unwrap_err();
        let ReadError::Refused(why) = error else {
            panic!("{error:?}")
        };
        assert!(
            why.starts_with("Next page not followed: on http://127.0.0.1:"),
            "{why}"
        );
        assert_eq!(github.seen().len(), 1, "not asked again");
        assert!(
            elsewhere.seen().is_empty(),
            "the other origin never contacted"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_next_link_to_another_listing_is_refused() {
        let github = mock::serve(|seen: &Seen| {
            let mut response = json_answer("[1]");
            let link = format!(
                "<{}/repos/dashpay/platform/collaborators>; rel=\"next\"",
                seen.origin()
            );
            response
                .headers_mut()
                .insert(header::LINK, link.parse().unwrap());
            response
        })
        .await;
        let error = run(transport(&github.url), |client| {
            client
                .pages("repos/dashpay/platform/pulls")
                .map(|_| PyValue::None)
        })
        .await
        .unwrap_err();
        assert!(
            matches!(&error, ReadError::Refused(why) if why.contains("is not the listing")),
            "{error:?}"
        );
        assert_eq!(github.seen().len(), 1);
    }

    /// A gateway error is asked again once, and the second answer stands.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_502_is_asked_again_once_through_the_client() {
        let count = AtomicUsize::new(0);
        let github = mock::serve(move |_: &Seen| match count.fetch_add(1, Ordering::SeqCst) {
            0 => (Code::BAD_GATEWAY, "<html>bad gateway</html>").into_response(),
            _ => json_answer(r#"{"number": 7}"#),
        })
        .await;
        let answer = run(transport(&github.url), |client| {
            client.request(Method::Get, "repos/dashpay/platform/pulls/7", None)
        })
        .await
        .unwrap();
        assert!(matches!(answer, PyValue::Dict(_)), "{answer:?}");
        assert_eq!(github.seen().len(), 2);
    }

    /// A 404 is GitHub's answer, not a failure on the way: never asked
    /// again, and reported in Python's words with GitHub's.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_404_is_not_asked_again() {
        let github = mock::serve(|_: &Seen| {
            (
                Code::NOT_FOUND,
                [(header::CONTENT_TYPE, "application/json")],
                r#"{"message": "Not Found", "documentation_url": "https://docs.github.com/rest"}"#,
            )
                .into_response()
        })
        .await;
        let error = run(transport(&github.url), |client| {
            client.request(Method::Get, "repos/dashpay/platform/labels/missing", None)
        })
        .await
        .unwrap_err();
        assert_eq!(
            error,
            ReadError::GitHub("GitHub API command failed (exit 1): Not Found (HTTP 404)".into())
        );
        assert_eq!(github.seen().len(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn only_a_gateway_error_is_transient() {
        for (code, transient) in [
            (500, false),
            (501, false),
            (502, true),
            (503, true),
            (504, true),
            (403, false),
            (429, false),
        ] {
            let github =
                mock::serve(move |_: &Seen| (Code::from_u16(code).unwrap(), "{}").into_response())
                    .await;
            let Err(TransportError::Failed(failure)) =
                call(transport(&github.url), get("repos/a/b/pulls/1")).await
            else {
                panic!("{code} succeeded")
            };
            assert_eq!(failure.transient, transient, "{code}");
            assert_eq!(failure.status, Some(1));
        }
    }

    /// An answer cut short by the connection closing is worth one more try,
    /// and gets it.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_truncated_answer_is_transient() {
        let raw = mock::raw(|_| {
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 100\r\n\r\n[{\"number\": 1".to_vec()
        })
        .await;
        let reply = call(transport(&raw.url), get("repos/a/b/pulls/1")).await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(failure.transient, "{failure:?}");
        let before = raw.connections();
        let error = run(transport(&raw.url), |client| {
            client.request(Method::Get, "repos/a/b/pulls/1", None)
        })
        .await
        .unwrap_err();
        assert!(matches!(error, ReadError::GitHub(_)), "{error:?}");
        assert_eq!(raw.connections() - before, 2, "asked once more");
    }

    /// A connection closed before any answer came is `gh`'s `EOF`, which
    /// Python asked again.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_connection_closed_before_answering_is_transient() {
        let raw = mock::raw(|_| Vec::new()).await;
        let reply = call(transport(&raw.url), get("repos/a/b/pulls/1")).await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(failure.transient, "{failure:?}");
        assert_eq!(failure.status, Some(1));
    }

    /// A complete answer that is not JSON was garbled on the way.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_unreadable_answer_is_transient_and_an_empty_one_is_none() {
        let github = mock::serve(|seen: &Seen| match seen.path_and_query.as_str() {
            "/repos/a/b/pulls/1" => json_answer(r#"[{"number": 1"#),
            _ => (Code::NO_CONTENT, "").into_response(),
        })
        .await;
        let reply = call(transport(&github.url), get("repos/a/b/pulls/1")).await;
        assert!(
            matches!(&reply, Err(TransportError::Failed(f)) if f.transient),
            "{reply:?}"
        );
        let answer = run(transport(&github.url), |client| {
            client.request(Method::Get, "repos/a/b/pulls/2", None)
        })
        .await;
        assert!(matches!(answer, Ok(PyValue::None)), "{answer:?}");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_timeout_is_transient() {
        let silent = mock::raw_silent().await;
        let reply = call(
            transport_with_timeout(&silent.url, Duration::from_millis(200)),
            get("repos/a/b/pulls/1"),
        )
        .await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(failure.transient, "{failure:?}");
        assert_eq!(failure.status, Some(1), "a completed failure: asked again");
    }

    /// A connection that cannot be made is not a flaky answer.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_refused_connection_is_not_transient() {
        let reply = call(transport("http://127.0.0.1:9"), get("repos/a/b/pulls/1")).await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(!failure.transient, "{failure:?}");
        assert!(
            !failure.detail.contains("127.0.0.1"),
            "no URL: {}",
            failure.detail
        );
    }

    /// GraphQL answers a query that only partly resolved with its data and
    /// an errors array. The transport reports it as `gh` did; the client
    /// hands the data to the reader, which decides.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_graphql_partial_answer_reaches_the_client() {
        const PARTIAL: &str = r#"{"data": {"repository": {"pr1": {"number": 1}, "pr2": null}}, "errors": [{"type": "NOT_FOUND", "path": ["repository", "pr2"], "message": "Could not resolve to a PullRequest with the number of 2."}]}"#;
        let github = mock::serve(|_: &Seen| json_answer(PARTIAL)).await;
        let numbers = [PyInt::from(1), PyInt::from(2)];
        let query = queries::histories(&numbers);
        let reply = call(transport(&github.url), graphql_call(&query)).await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(!failure.transient);
        assert_eq!(failure.body, PARTIAL);
        assert_eq!(
            failure.detail,
            "Could not resolve to a PullRequest with the number of 2."
        );
        let answer = run(transport(&github.url), move |client| {
            client.graphql(&query, PyValue::Dict(PyDict::new()))
        })
        .await
        .unwrap();
        assert!(
            matches!(&answer, PyValue::Dict(fields) if fields.contains_key("errors")),
            "the whole answer, errors and all: {answer:?}"
        );
        assert_eq!(github.seen().len(), 2, "a POST is never asked twice");

        // Errors and no data: no answer.
        let github = mock::serve(|_: &Seen| {
            json_answer(r#"{"data": null, "errors": [{"message": "Something went wrong"}]}"#)
        })
        .await;
        let error = run(transport(&github.url), |client| {
            client.graphql(queries::BUILD, PyValue::Dict(PyDict::new()))
        })
        .await
        .unwrap_err();
        assert_eq!(
            error,
            ReadError::GitHub("GitHub API command failed (exit 1): Something went wrong".into())
        );
    }

    /// A redirect would carry the request, token and all, somewhere else.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_redirect_is_not_followed() {
        let github = mock::serve(|seen: &Seen| {
            let to = format!("{}/repos/a/b/pulls/2", seen.origin());
            (Code::FOUND, [(header::LOCATION, to)]).into_response()
        })
        .await;
        let reply = call(transport(&github.url), get("repos/a/b/pulls/1")).await;
        let Err(TransportError::Failed(failure)) = reply else {
            panic!("{reply:?}")
        };
        assert!(!failure.transient);
        assert_eq!(failure.detail, "a redirect, not followed (HTTP 302)");
        assert_eq!(github.seen().len(), 1, "one request, not two");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn a_route_that_would_change_on_the_way_is_refused_unsent() {
        let github = mock::serve(|_: &Seen| json_answer("{}")).await;
        for path in [
            "repos/a/b/../c/pulls",
            "repos/a/b/./pulls",
            "repos/a/b/pulls#x",
            "repos/a/b/pulls with space",
            "repos/a\\b/pulls",
        ] {
            let reply = call(transport(&github.url), get(path)).await;
            assert!(
                matches!(&reply, Err(TransportError::Refused(why)) if why.contains("as written")),
                "{path}: {reply:?}"
            );
        }
        assert!(github.seen().is_empty());
    }

    /// The engine's thread blocks on the service's runtime; the runtime's
    /// own workers stay free to serve, here the stand-in for GitHub itself.
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn the_engines_thread_blocks_and_the_runtime_still_serves() {
        let github = mock::serve(|_: &Seen| json_answer("[]")).await;
        let calls = (0..4).map(|n| {
            let transport = transport(&github.url);
            tokio::task::spawn_blocking(move || {
                let mut transport = transport;
                transport.call(&get(&format!("repos/a/b/pulls/{n}")))
            })
        });
        for reply in futures_join(calls).await {
            assert_eq!(reply, Ok(Reply::Text("[]".into())));
        }
        assert_eq!(github.seen().len(), 4);
    }

    async fn futures_join<T>(handles: impl Iterator<Item = tokio::task::JoinHandle<T>>) -> Vec<T> {
        let mut out = Vec::new();
        for handle in handles.collect::<Vec<_>>() {
            out.push(handle.await.unwrap());
        }
        out
    }

    /// Called on the runtime itself, the transport panics at once rather
    /// than block a worker the request needs.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[should_panic(expected = "Cannot start a runtime from within a runtime")]
    async fn called_on_the_runtime_it_panics_rather_than_deadlocks() {
        let mut transport = transport("http://127.0.0.1:9");
        let _ = transport.call(&get("repos/a/b/pulls/1"));
    }
}
