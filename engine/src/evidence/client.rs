//! The rules every answer is read by, whatever fetched it: `GitHub._run`,
//! `GitHub.request` and `GitHub.pages`.

use super::error::ReadError;
use super::transport::{Call, Method, Reply, Transport, TransportError};
use crate::pycompat::text::{py_slice, py_strip};
use crate::pycompat::{py_loads, PyErr, PyValue};
use std::time::Duration;

/// How long the client waits before asking a transiently failed call again.
pub const RETRY_DELAY: Duration = Duration::from_secs(2);

/// Waiting, made a value so that a test of the retry does not wait.
pub trait Sleep {
    fn sleep(&mut self, duration: Duration);
}

/// Waits on the current thread.
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreadSleep;

impl Sleep for ThreadSleep {
    fn sleep(&mut self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

/// Does not wait: for a replay, where the retry still happens and the wait
/// would only slow it down.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoSleep;

impl Sleep for NoSleep {
    fn sleep(&mut self, _duration: Duration) {}
}

/// A transport, read by Python's rules. It can move to another thread
/// whenever its transport can.
pub struct Client<T> {
    transport: T,
    sleep: Box<dyn Sleep + Send>,
}

impl<T: Transport> Client<T> {
    /// A client that waits for real between a failure and its retry.
    pub fn new(transport: T) -> Self {
        Self::with_sleep(transport, ThreadSleep)
    }

    pub fn with_sleep(transport: T, sleep: impl Sleep + Send + 'static) -> Self {
        Client {
            transport,
            sleep: Box::new(sleep),
        }
    }

    pub fn transport(&self) -> &T {
        &self.transport
    }

    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }

    /// `GitHub._run`: the answer to `call`, read as JSON.
    ///
    /// - A failure worth one more try — transient, of an idempotent call —
    ///   is asked once more, [`RETRY_DELAY`] later: one flaky answer would
    ///   otherwise mark a pull request an error under a required check until
    ///   its next event. A `POST` is never asked twice: it could write twice.
    /// - A failed GraphQL query whose answer still holds a `data` object is
    ///   an answer: GraphQL answers with a usable payload and an errors
    ///   array when only part of a query resolved, and the caller decides
    ///   which of those errors it tolerates. Every other failure, and every
    ///   failed REST call, is an error.
    /// - A call that never completed is an error, and is not asked again.
    /// - Empty output (nothing but whitespace) is `None`; output that is not
    ///   JSON is an error.
    pub fn run(&mut self, call: &Call) -> Result<PyValue, ReadError> {
        let mut first = true;
        loop {
            let failure = match self.transport.call(call) {
                Ok(reply) => return read_reply(reply),
                Err(TransportError::Refused(why)) => return Err(ReadError::Refused(why)),
                Err(TransportError::Failed(failure)) => failure,
            };
            let Some(status) = failure.status else {
                return Err(ReadError::github(
                    "GitHub API command unavailable or timed out",
                ));
            };
            if first && failure.transient && call.method().idempotent() {
                first = false;
                self.sleep.sleep(RETRY_DELAY);
                continue;
            }
            if call.is_graphql() && !py_strip(&failure.body).is_empty() {
                match py_loads(&failure.body) {
                    Ok(body) => {
                        if let PyValue::Dict(entries) = &body {
                            if matches!(entries.get("data"), Some(PyValue::Dict(_))) {
                                return Ok(body);
                            }
                        }
                    }
                    Err(PyErr::Value(_)) => {}
                    Err(other) => return Err(other.into()),
                }
            }
            let detail = py_slice(py_strip(&failure.detail), None, Some(300));
            let mut message = format!("GitHub API command failed (exit {status})");
            if !detail.is_empty() {
                message.push_str(": ");
                message.push_str(detail);
            }
            return Err(ReadError::GitHub(message));
        }
    }

    /// `GitHub.request(method, path, payload)`.
    pub fn request(
        &mut self,
        method: Method,
        path: &str,
        payload: Option<PyValue>,
    ) -> Result<PyValue, ReadError> {
        self.run(&Call::Rest {
            method,
            path: path.to_owned(),
            body: payload,
            paginate: false,
        })
    }

    /// `GitHub.request("POST", "graphql", {"query": query, "variables": variables})`.
    pub fn graphql(&mut self, query: &str, variables: PyValue) -> Result<PyValue, ReadError> {
        self.run(&Call::Graphql {
            query: query.to_owned(),
            variables,
        })
    }

    /// `GitHub.pages(path)`: every item on every page of a REST listing, a
    /// hundred to a page unless the path says otherwise.
    pub fn pages(&mut self, path: &str) -> Result<Vec<PyValue>, ReadError> {
        let mut path = path.to_owned();
        let separator = if path.contains('?') { '&' } else { '?' };
        if !path.contains("per_page=") {
            path.push(separator);
            path.push_str("per_page=100");
        }
        let answer = self.run(&Call::Rest {
            method: Method::Get,
            path,
            body: None,
            paginate: true,
        })?;
        let expected = || ReadError::github("Expected paginated GitHub list");
        let PyValue::List(pages) = answer else {
            return Err(expected());
        };
        let mut items = Vec::new();
        for page in pages {
            let PyValue::List(page) = page else {
                return Err(expected());
            };
            items.extend(page);
        }
        Ok(items)
    }
}

fn read_reply(reply: Reply) -> Result<PyValue, ReadError> {
    match reply {
        Reply::Text(text) => read_text(&text),
        Reply::Pages(pages) => Ok(PyValue::List(
            pages
                .iter()
                .map(|page| read_text(page))
                .collect::<Result<Vec<_>, _>>()?
                .into(),
        )),
    }
}

/// One body: `None` when there is nothing but whitespace, else its JSON.
fn read_text(text: &str) -> Result<PyValue, ReadError> {
    if py_strip(text).is_empty() {
        return Ok(PyValue::None);
    }
    match py_loads(text) {
        Ok(value) => Ok(value),
        Err(PyErr::Value(_)) => Err(ReadError::github("GitHub API returned invalid JSON")),
        Err(other) => Err(other.into()),
    }
}
