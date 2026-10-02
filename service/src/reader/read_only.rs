//! [`ReadOnly`]: a transport that lets through only what reads.
//!
//! The reader App's permissions are the first lock: its token can only
//! read. This is the second, in the service's own code, so that neither a
//! bug in the engine nor a mistaken grant can turn a read-only run into a
//! write. It sits between the engine's client and the HTTP transport, so
//! every call the client makes passes it — each call of a GraphQL
//! listing's pages, and the first page of a REST listing, whose later pages
//! the transport follows only on the same origin and the same listing, one
//! repository's, and only by `GET`. The transport is made only behind this
//! layer ([`read_only_transport`](super::read_only_transport)).
//!
//! A read is:
//!
//! - a `GET`, with no body, of a route under `repos/{owner}/{repo}/` for
//!   a repository it was given, every segment of that route plain — letters,
//!   digits, `-._~` and percent escapes, none of which decodes to `.`, `..`,
//!   `/`, `\` or `%` — and its query likewise;
//! - a GraphQL document that is exactly one of the engine's own queries
//!   ([`queries`]), every character as the engine writes it, asking about a
//!   repository it was given.
//!
//! Anything else is refused with a [`TransportError::Refused`] that begins
//! with [`NOT_A_READ`], and is never sent. The client does not ask a
//! refused call again, and the run stops.

use pr_hygiene_engine::evidence::queries;
use pr_hygiene_engine::evidence::{Call, Method, Reply, Transport, TransportError};
use pr_hygiene_engine::pycompat::{PyInt, PyValue};

/// How every refusal begins.
pub const NOT_A_READ: &str = "Not a read, so not sent: ";

/// A transport that passes reads to `T` and refuses everything else.
#[derive(Debug)]
pub struct ReadOnly<T> {
    inner: T,
    /// `owner/name` of each repository it may read, lower-cased: GitHub's
    /// names are case-insensitive.
    repositories: Vec<String>,
    refused: usize,
}

/// A repository identity that is not `owner/name`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0:?} is not a repository identity owner/name")]
pub struct NotARepository(pub String);

impl<T> ReadOnly<T> {
    /// Reads of `repositories`, each `owner/name`, through `inner`.
    pub fn new(
        inner: T,
        repositories: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, NotARepository> {
        let repositories = repositories
            .into_iter()
            .map(|repository| {
                let repository = repository.as_ref();
                let valid = repository
                    .split_once('/')
                    .is_some_and(|(owner, name)| identity_part(owner) && identity_part(name));
                if valid {
                    Ok(repository.to_ascii_lowercase())
                } else {
                    Err(NotARepository(repository.to_owned()))
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(ReadOnly {
            inner,
            repositories,
            refused: 0,
        })
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    /// How many calls it has refused.
    pub fn refused(&self) -> usize {
        self.refused
    }

    fn allows(&self, owner: &str, name: &str) -> bool {
        let wanted = format!("{owner}/{name}").to_ascii_lowercase();
        self.repositories.contains(&wanted)
    }

    /// Why `call` is not a read, if it is not.
    pub fn check(&self, call: &Call) -> Result<(), String> {
        match call {
            Call::Rest {
                method: Method::Get,
                path,
                body: None,
                ..
            } => self.check_route(path),
            Call::Rest {
                method: Method::Get,
                path,
                ..
            } => Err(format!("a GET with a body: {path}")),
            Call::Rest { method, path, .. } => Err(format!("{method} {path}")),
            Call::Graphql { query, variables } => {
                if !is_engine_query(query) {
                    return Err(format!(
                        "a GraphQL document that is not one of the engine's queries: {call}"
                    ));
                }
                match graphql_repository(variables) {
                    Some((owner, name)) if self.allows(owner, name) => Ok(()),
                    Some((owner, name)) => Err(format!(
                        "a GraphQL query about {owner}/{name}, which it may not read"
                    )),
                    None => Err("a GraphQL query naming no repository".into()),
                }
            }
        }
    }

    fn check_route(&self, path: &str) -> Result<(), String> {
        let (route, query) = match path.split_once('?') {
            Some((route, query)) => (route, Some(query)),
            None => (path, None),
        };
        let mut segments = route.split('/');
        let (Some("repos"), Some(owner), Some(name)) =
            (segments.next(), segments.next(), segments.next())
        else {
            return Err(format!("GET {path}, outside repos/{{owner}}/{{repo}}/"));
        };
        if !self.allows(owner, name) {
            return Err(format!("GET {path}, outside the repositories it may read"));
        }
        let rest: Vec<&str> = segments.collect();
        if rest.is_empty() || !rest.iter().all(|segment| plain_segment(segment)) {
            return Err(format!("GET {path}, not a plain route"));
        }
        if query.is_some_and(|query| !plain_query(query)) {
            return Err(format!("GET {path}, not a plain query"));
        }
        Ok(())
    }
}

impl<T: Transport> Transport for ReadOnly<T> {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        if let Err(why) = self.check(call) {
            self.refused += 1;
            tracing::warn!(%why, "the reader refused a call that is not a read");
            return Err(TransportError::Refused(format!("{NOT_A_READ}{why}")));
        }
        self.inner.call(call)
    }
}

/// A part of `owner/name`, as the engine accepts one.
fn identity_part(part: &str) -> bool {
    !part.is_empty()
        && part != "."
        && part != ".."
        && part
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'))
}

/// One segment of a route: unreserved characters and percent escapes, and
/// not a segment a server could read as a step up or across.
fn plain_segment(segment: &str) -> bool {
    let Some(decoded) = percent_decoded(segment, |b| {
        b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~')
    }) else {
        return false;
    };
    !decoded.is_empty()
        && decoded != b"."
        && decoded != b".."
        && !decoded.contains(&b'/')
        && !decoded.contains(&b'\\')
        && !decoded.contains(&b'%')
}

/// A query of plain `name=value` pairs.
fn plain_query(query: &str) -> bool {
    percent_decoded(query, |b| {
        b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'=' | b'&')
    })
    .is_some()
}

/// The bytes `text` stands for, when it holds only bytes `plain` accepts
/// and well-formed percent escapes.
fn percent_decoded(text: &str, plain: impl Fn(u8) -> bool) -> Option<Vec<u8>> {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'%' => {
                let hex = bytes.get(at + 1..at + 3)?;
                if !hex.iter().all(u8::is_ascii_hexdigit) {
                    return None;
                }
                let digit = |b: u8| (b as char).to_digit(16).unwrap_or_default() as u8;
                decoded.push(digit(hex[0]) * 16 + digit(hex[1]));
                at += 3;
            }
            b if plain(b) => {
                decoded.push(b);
                at += 1;
            }
            _ => return None,
        }
    }
    Some(decoded)
}

/// Whether `query` is, character for character, one of the documents the
/// engine sends.
fn is_engine_query(query: &str) -> bool {
    [queries::BUILD, queries::THREADS, queries::COMMENT_PAGES].contains(&query)
        || is_histories(query)
}

/// Whether `query` is the engine's batched history query for the pull
/// requests it names: exactly what [`queries::histories`] writes for them,
/// in ascending order with none twice, as the engine asks.
fn is_histories(query: &str) -> bool {
    const SELECTION: &str = "pullRequest(number:";
    let mut numbers: Vec<i64> = Vec::new();
    for (at, _) in query.match_indices(SELECTION) {
        let digits = query[at + SELECTION.len()..]
            .split(')')
            .next()
            .unwrap_or_default();
        if digits.is_empty() || digits.len() > 18 || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return false;
        }
        let Ok(number) = digits.parse::<i64>() else {
            return false;
        };
        if numbers.last().is_some_and(|last| *last >= number) {
            return false;
        }
        numbers.push(number);
    }
    let numbers: Vec<PyInt> = numbers.into_iter().map(PyInt::from).collect();
    !numbers.is_empty() && queries::histories(&numbers) == query
}

/// The repository a GraphQL query's variables name: every engine query
/// reads `repository(owner:$owner, name:$repo)`.
fn graphql_repository(variables: &PyValue) -> Option<(&str, &str)> {
    let PyValue::Dict(variables) = variables else {
        return None;
    };
    match (variables.get("owner"), variables.get("repo")) {
        (Some(PyValue::Str(owner)), Some(PyValue::Str(name))) => Some((owner, name)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr_hygiene_engine::evidence::Scripted;
    use pr_hygiene_engine::pycompat::PyDict;

    const REPO: &str = "dashpay/platform";

    /// A read-only layer over a transport that answers anything: what
    /// reaches it was let through.
    fn guard() -> ReadOnly<Scripted> {
        let answers = (0..32).map(|_| Ok(Reply::Text("{}".into())));
        ReadOnly::new(Scripted::new(answers), [REPO, "dashpay/tenderdash"]).unwrap()
    }

    fn rest(method: Method, path: &str, body: Option<PyValue>, paginate: bool) -> Call {
        Call::Rest {
            method,
            path: path.into(),
            body,
            paginate,
        }
    }

    fn get(path: &str) -> Call {
        rest(Method::Get, path, None, false)
    }

    fn variables(repository: &str) -> PyValue {
        let (owner, name) = repository.split_once('/').unwrap();
        let mut variables = PyDict::new();
        variables.insert("owner".into(), PyValue::Str(owner.into()));
        variables.insert("repo".into(), PyValue::Str(name.into()));
        variables.insert("number".into(), PyValue::Int(PyInt::from(7)));
        PyValue::Dict(variables)
    }

    fn graphql(query: &str) -> Call {
        Call::Graphql {
            query: query.into(),
            variables: variables(REPO),
        }
    }

    /// The call is refused, counted, and never reaches the transport.
    #[track_caller]
    fn refused(call: Call) -> String {
        let mut guard = guard();
        let Err(TransportError::Refused(why)) = guard.call(&call) else {
            panic!("let through: {call}")
        };
        assert!(why.starts_with(NOT_A_READ), "{why}");
        assert_eq!(guard.refused(), 1);
        assert!(guard.inner().calls().is_empty(), "sent: {call}");
        why
    }

    #[track_caller]
    fn let_through(call: Call) {
        let mut guard = guard();
        assert_eq!(guard.call(&call), Ok(Reply::Text("{}".into())), "{call}");
        assert_eq!(guard.inner().calls().len(), 1);
        assert_eq!(guard.refused(), 0);
    }

    #[test]
    fn every_read_the_engine_makes_passes() {
        for path in [
            "repos/dashpay/platform/pulls/7",
            "repos/dashpay/platform/pulls?state=open&per_page=100",
            "repos/dashpay/platform/pulls/7/files?per_page=100",
            "repos/dashpay/platform/issues/7/timeline?per_page=100",
            "repos/dashpay/platform/collaborators?affiliation=all&per_page=100",
            "repos/dashpay/platform/collaborators/coderabbitai%5Bbot%5D/permission",
            "repos/dashpay/platform/commits/0123abc/statuses?per_page=100",
            "repos/dashpay/platform/labels/ready-for-human",
            "repos/DashPay/Platform/pulls/7",
            "repos/dashpay/tenderdash/pulls/1",
        ] {
            let_through(get(path));
            let_through(rest(Method::Get, path, None, true));
        }
        for query in [queries::BUILD, queries::THREADS, queries::COMMENT_PAGES] {
            let_through(graphql(query));
        }
        let numbers: Vec<PyInt> = [1, 2, 4818].into_iter().map(PyInt::from).collect();
        let_through(graphql(&queries::histories(&numbers)));
        let_through(graphql(&queries::histories(&[PyInt::from(5)])));
    }

    #[test]
    fn a_rest_call_other_than_a_get_is_refused() {
        let body = || Some(PyValue::Dict(PyDict::new()));
        for method in [Method::Post, Method::Put, Method::Patch, Method::Delete] {
            refused(rest(
                method,
                "repos/dashpay/platform/statuses/abc",
                body(),
                false,
            ));
            refused(rest(
                method,
                "repos/dashpay/platform/issues/comments/5",
                None,
                false,
            ));
            refused(rest(method, "repos/dashpay/platform/pulls", None, true));
        }
        assert_eq!(
            refused(rest(
                Method::Get,
                "repos/dashpay/platform/pulls/7",
                body(),
                false
            )),
            format!("{NOT_A_READ}a GET with a body: repos/dashpay/platform/pulls/7")
        );
    }

    #[test]
    fn a_route_outside_the_allowed_repositories_is_refused() {
        for path in [
            "repos/dashpay/dash/pulls/7",
            "repos/other/platform/pulls/7",
            "repos/dashpay/platform",
            "repos/dashpay/platform/",
            "repos/dashpay",
            "user",
            "user/repos",
            "app/installations/1/access_tokens",
            "orgs/dashpay/members",
            "graphql",
            "/repos/dashpay/platform/pulls/7",
            "https://api.github.com/repos/dashpay/platform/pulls/7",
            "https://evil.example/repos/dashpay/platform/pulls/7",
            "//evil.example/repos/dashpay/platform/pulls/7",
            "repos/dashpay/platform/../dash/pulls/7",
            "repos/dashpay/platform/%2e%2e/dash/pulls",
            "repos/dashpay/platform/%2E/pulls",
            "repos/dashpay/platform/pulls%2f7",
            "repos/dashpay/platform/pulls%5c7",
            // Encoded twice: `..` to whatever decodes it a second time.
            "repos/dashpay/platform/%252e%252e/dash/pulls",
            "repos/dashpay/platform//pulls",
            "repos/dashpay/platform/pulls/7#frag",
            "repos/dashpay/platform/pulls/7?state=open#frag",
            "repos/dashpay/platform/pulls?x=%zz",
            "repos/dashpay/platform/pulls?redirect=https://evil.example",
            "repos/dashpay/platform/pulls\\7",
            "repos/dashpay/platform/pulls/7 HTTP/1.1",
        ] {
            refused(get(path));
        }
    }

    #[test]
    fn a_graphql_mutation_is_refused() {
        refused(graphql(
            "mutation { addComment(input: {subjectId: \"x\", body: \"hi\"}) { clientMutationId } }",
        ));
        // A query with a mutation after it, and one hidden in a history
        // query's selection.
        refused(graphql(&format!("{}\nmutation {{ x }}", queries::BUILD)));
        let numbers = [PyInt::from(1)];
        let history = queries::histories(&numbers);
        refused(graphql(&history.replace(
            "{ ...history }",
            "{ ...history } } } mutation { deleteIssue(input:{issueId:\"x\"}) { clientMutationId } } query { a {",
        )));
    }

    /// The exact text, or nothing: one character more, less or different
    /// is another document.
    #[test]
    fn a_query_one_character_off_is_refused() {
        for query in [queries::BUILD, queries::THREADS, queries::COMMENT_PAGES] {
            refused(graphql(&query.replacen("100", "101", 1)));
            refused(graphql(&format!("{query} ")));
            refused(graphql(&query[..query.len() - 1]));
        }
        let numbers = [PyInt::from(1), PyInt::from(2)];
        let history = queries::histories(&numbers);
        refused(graphql(&history.replacen("last:100", "last:99", 1)));
        refused(graphql(&history.replacen(
            "pr2: pullRequest",
            "pr3: pullRequest",
            1,
        )));
        // Not as the engine asks: out of order, twice, zero-padded.
        let reversed = queries::histories(&[PyInt::from(2), PyInt::from(1)]);
        refused(graphql(&reversed));
        let twice = queries::histories(&[PyInt::from(2), PyInt::from(2)]);
        refused(graphql(&twice));
        refused(graphql(&history.replacen("number:1)", "number:01)", 1)));
        refused(graphql(&queries::histories(&[])));
    }

    #[test]
    fn a_query_about_another_repository_is_refused() {
        let why = refused(Call::Graphql {
            query: queries::BUILD.into(),
            variables: variables("dashpay/dash"),
        });
        assert!(why.contains("dashpay/dash"), "{why}");
        refused(Call::Graphql {
            query: queries::BUILD.into(),
            variables: PyValue::Dict(PyDict::new()),
        });
    }

    #[test]
    fn only_owner_name_identities_are_accepted() {
        assert!(ReadOnly::new((), ["dashpay/platform", "a.b/c_d-e"]).is_ok());
        for bad in [
            "dashpay",
            "dashpay/",
            "/platform",
            "a/b/c",
            "../x",
            "a/..",
            "a b/c",
            "",
        ] {
            assert_eq!(
                ReadOnly::new((), [bad]).map(|_| ()),
                Err(NotARepository(bad.into())),
                "{bad}"
            );
        }
    }
}
