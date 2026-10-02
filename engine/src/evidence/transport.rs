//! The seam between the engine and GitHub: one [`Call`] out, one [`Reply`]
//! or one failure back. Everything above it is the engine's own reading of
//! the answer; everything below it is how the answer was fetched.

use crate::pycompat::PyValue;
use std::collections::VecDeque;
use std::fmt;

/// An HTTP method the engine uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
        }
    }

    /// Whether asking twice can do nothing asking once would not: the only
    /// calls the client asks again after a transient failure. A `POST`
    /// could write twice, and GraphQL is a `POST`.
    pub fn idempotent(self) -> bool {
        !matches!(self, Method::Post)
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One request to GitHub.
#[derive(Debug, Clone)]
pub enum Call {
    /// A REST route under `https://api.github.com/`, without a leading `/`,
    /// with its query string as written. `body` is the JSON sent, if any.
    /// `paginate` asks for every page, answered as a list of pages.
    Rest {
        method: Method,
        path: String,
        body: Option<PyValue>,
        paginate: bool,
    },
    /// A GraphQL document and its variables, sent as one `POST`.
    Graphql { query: String, variables: PyValue },
}

impl Call {
    pub fn method(&self) -> Method {
        match self {
            Call::Rest { method, .. } => *method,
            Call::Graphql { .. } => Method::Post,
        }
    }

    pub fn is_graphql(&self) -> bool {
        matches!(self, Call::Graphql { .. })
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Call::Rest {
                method,
                path,
                paginate,
                ..
            } => write!(
                f,
                "{method} {path}{}",
                if *paginate { " (every page)" } else { "" }
            ),
            Call::Graphql { query, .. } => {
                let first = query.lines().next().unwrap_or_default();
                write!(f, "POST graphql {first}")
            }
        }
    }
}

/// What a call that completed answered: the text of its body, not yet
/// read as JSON. Reading it is the client's, so that every transport's
/// answer is read by the same rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    /// The whole answer. For a paginated call made through `gh api
    /// --paginate --slurp`, that is already the list of pages.
    Text(String),
    /// A paginated call's pages, each its own body, in order: read as the
    /// list `gh api --paginate --slurp` would have printed.
    Pages(Vec<String>),
}

/// A call that completed and failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// Whether asking again could plausibly succeed: a gateway error, a
    /// timeout reported by the far side, a truncated answer. The client
    /// asks again only for an idempotent call.
    pub transient: bool,
    /// The status the failure ended with, which the client's error quotes
    /// as `GitHub API command failed (exit N)`, Python's words for `gh`'s
    /// exit status. `None` when the call never completed: the command could
    /// not run, or ran out of its own time. That is never asked again.
    pub status: Option<i32>,
    /// What came back as the answer, if anything: a failed GraphQL query
    /// can still carry data in it.
    pub body: String,
    /// What the transport said about the failure.
    pub detail: String,
}

impl Failure {
    /// A call that never completed.
    pub fn unavailable() -> Self {
        Failure {
            transient: false,
            status: None,
            body: String::new(),
            detail: String::new(),
        }
    }
}

/// Why a transport gave no reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    /// The call was made, or tried, and failed.
    Failed(Failure),
    /// The transport would not make the call at all. Never asked again and
    /// never read as evidence: it stops the run.
    Refused(String),
}

impl From<Failure> for TransportError {
    fn from(failure: Failure) -> Self {
        TransportError::Failed(failure)
    }
}

/// Something that answers calls.
pub trait Transport {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError>;
}

impl<T: Transport + ?Sized> Transport for &mut T {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        (**self).call(call)
    }
}

impl<T: Transport + ?Sized> Transport for Box<T> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        (**self).call(call)
    }
}

/// A function that answers calls, which is how a test routes each call to
/// the answer it means.
pub struct FromFn<F>(pub F);

impl<F> Transport for FromFn<F>
where
    F: FnMut(&Call) -> Result<Reply, TransportError>,
{
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        (self.0)(call)
    }
}

/// A transport that answers from a queue, in order, and keeps every call it
/// was asked. Asked once more than it holds, it refuses.
#[derive(Debug, Default)]
pub struct Scripted {
    answers: VecDeque<Result<Reply, TransportError>>,
    calls: Vec<Call>,
}

impl Scripted {
    pub fn new(answers: impl IntoIterator<Item = Result<Reply, TransportError>>) -> Self {
        Scripted {
            answers: answers.into_iter().collect(),
            calls: Vec::new(),
        }
    }

    /// Every call made, in order.
    pub fn calls(&self) -> &[Call] {
        &self.calls
    }

    /// How many answers are still queued.
    pub fn remaining(&self) -> usize {
        self.answers.len()
    }
}

impl Transport for Scripted {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        self.calls.push(call.clone());
        self.answers.pop_front().unwrap_or_else(|| {
            Err(TransportError::Refused(format!(
                "no answer left for {call}"
            )))
        })
    }
}
