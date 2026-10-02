//! [`GitHub`]: every read `pr_review/github.py` makes, and the snapshot of
//! one pull request that `evaluate` decides from.
//!
//! Reads are sequential. Python's run takes four snapshots at once over
//! shared caches, and one of them can find the collaborator listing marked
//! read before another has read it; the port does not reproduce that race.
//! It reads the listing once, then asks per person for anyone it does not
//! name, as a run of Python's taken one snapshot at a time does.

use super::builds::{build_verdict, Build};
use super::client::Client;
use super::error::{KeyOrTypeAs, PyClass, ReadError};
use super::py::{
    get, get_or_empty, is_engine, item, iterate, iterate_or_empty, login, or_default, str_method,
    text, Read,
};
use super::queries;
use super::records::{
    parse_controller_diff, parse_controller_state, utc_timestamp, EDITOR_UNKNOWN,
};
use super::rules::{finding_severities, BOT_LOGINS};
use super::transport::{Method, Transport};
use crate::pycompat::hashlib::sha256_hexdigest;
use crate::pycompat::ops::{
    py_compare_sequences, py_eq, py_eq_str, py_hashable, py_in_str_set, py_same_element, Compare,
};
use crate::pycompat::text::{py_lower, py_strip};
use crate::pycompat::urllib::py_quote;
use crate::pycompat::{py_min_by, PyDateTime, PyDict, PyErr, PyInt, PyList, PyValue};
use indexmap::IndexMap;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

/// A pull request's comments and its latest inactive transition, as
/// [`GitHub::histories`] reads them for many pull requests at once.
#[derive(Debug, Clone)]
pub struct History {
    /// Each comment in the shape every comment reader shares, with who last
    /// edited it and when.
    pub comments: Vec<PyValue>,
    /// When it was last closed or converted to a draft.
    pub lifecycle_at: Option<String>,
}

/// The reads of one repository, and the caches one run keeps.
///
/// Neither a person's access nor a commit's own status history changes
/// under one reconciliation, and asking again for every pull request was a
/// large part of the Python engine's traffic against the organisation's
/// limit. [`GitHub::forget_cached_access`] clears all of them before the
/// checks made immediately before writing.
pub struct GitHub<T> {
    repo: String,
    owner: String,
    name: String,
    root: String,
    client: Client<T>,
    /// Access by lowercased login; `None` where the answer is unknown.
    permissions: IndexMap<String, Option<String>>,
    statuses: IndexMap<String, Vec<PyValue>>,
    builds: IndexMap<String, Build>,
    listed: bool,
    diagnostics: Vec<String>,
}

fn str_value(text: impl Into<String>) -> PyValue {
    PyValue::Str(text.into())
}

fn optional(text: Option<String>) -> PyValue {
    text.map_or(PyValue::None, PyValue::Str)
}

fn dict(entries: PyDict) -> PyValue {
    PyValue::Dict(entries)
}

fn list(items: Vec<PyValue>) -> PyValue {
    PyValue::List(PyList::from(items))
}

/// `_unique(items, key, label)`: the same key twice is the listing
/// drifting under the read.
fn unique<K: Eq + Hash>(keys: impl IntoIterator<Item = K>, label: &str) -> Read<()> {
    let mut seen = HashSet::new();
    for key in keys {
        if !seen.insert(key) {
            return Err(ReadError::github(format!(
                "Duplicate {label}; pagination may have changed during collection"
            )));
        }
    }
    Ok(())
}

/// The id of an item already checked to carry an `int` one.
fn int_id(entry: &PyValue) -> Option<PyInt> {
    match entry {
        PyValue::Dict(fields) => match fields.get("id") {
            Some(PyValue::Int(id)) => Some(id.clone()),
            _ => None,
        },
        _ => None,
    }
}

/// `_capability(granted)`: a collaborator's level from the capability flags,
/// highest first. The flags are read rather than the role's name because a
/// custom organisation role carries a name this policy has never heard of
/// but still says plainly whether its holder can push.
fn capability(granted: &PyValue) -> Option<String> {
    let PyValue::Dict(flags) = granted else {
        return None;
    };
    ["admin", "maintain", "push", "triage", "pull"]
        .into_iter()
        .find(|level| matches!(flags.get(*level), Some(PyValue::Bool(true))))
        .map(|level| match level {
            "push" => "write".to_owned(),
            "pull" => "read".to_owned(),
            other => other.to_owned(),
        })
}

/// `_graphql_login(author)`: REST reports an app as `name[bot]` and GraphQL
/// reports the bare name. The engine finds its own record by that login, so
/// a bare one would make its state invisible and it would open a second
/// report on every pull request.
fn graphql_login(author: &PyValue) -> Read<String> {
    if !matches!(author, PyValue::Dict(_)) {
        return Err(ReadError::github("Missing comment author"));
    }
    let name = text(get(author, "login")?, "comment author")?;
    let bot = get(author, "__typename")?.is_some_and(|t| py_eq_str(t, "Bot"));
    Ok(if bot && !name.ends_with("[bot]") {
        format!("{name}[bot]")
    } else {
        name.to_owned()
    })
}

/// `_graphql_comment(comment)`: a comment as GraphQL answers it, in the
/// shape every comment reader shares.
///
/// Who last wrote it, not only when: a comment edited by somebody other
/// than its author is that person speaking. Whether it was edited at all is
/// the edit time GitHub records, not the update time, which also moves when
/// a comment is hidden; the field is required, so a query that stopped
/// asking for it fails rather than reading every comment as unedited. A
/// comment whose author GitHub answers as nobody — a deleted account — is
/// the `ghost` user's, as the listing names it, rather than a failure of the
/// whole history read for every pull request beside it.
fn graphql_comment(comment: &PyValue) -> Read<PyValue> {
    let edited_at = item(comment, "lastEditedAt")?;
    let author = item(comment, "author")?;
    let mut out = PyDict::new();
    out.insert("id".into(), item(comment, "databaseId")?.clone());
    let user = match author {
        PyValue::None => "ghost".to_owned(),
        author => graphql_login(author)?,
    };
    out.insert("user".into(), str_value(user));
    out.insert("body".into(), item(comment, "body")?.clone());
    out.insert(
        "created_at".into(),
        str_value(text(
            Some(item(comment, "createdAt")?),
            "comment creation time",
        )?),
    );
    out.insert(
        "updated_at".into(),
        str_value(text(
            Some(item(comment, "updatedAt")?),
            "comment update time",
        )?),
    );
    let edited = match edited_at {
        PyValue::None => PyValue::None,
        when => str_value(text(Some(when), "comment edit time")?),
    };
    out.insert("edited_at".into(), edited);
    let editor = match or_default(get(comment, "editor")?) {
        Some(_) => str_value(graphql_login(item(comment, "editor")?)?),
        None => PyValue::None,
    };
    out.insert("edited_by".into(), editor);
    Ok(dict(out))
}

/// Comments read from either route: every body a string, every id an
/// `int`, no id twice.
fn checked_comments(comments: &[PyValue]) -> Read<()> {
    let valid = |c: &PyValue| {
        matches!(item(c, "body"), Ok(PyValue::Str(_)))
            && matches!(item(c, "id"), Ok(PyValue::Int(_)))
    };
    if !comments.iter().all(valid) {
        return Err(ReadError::github("Invalid comment identity or body"));
    }
    unique(comments.iter().filter_map(int_id), "comment")
}

/// `errors = response.get("errors") or []`, where an error that is anything
/// but a pull request not found is a real failure: a pull request the
/// listing saw but GraphQL can no longer resolve answers null for its own
/// alias while the rest answer normally.
fn only_not_found(response: &PyValue, message: &str) -> Read<()> {
    match or_default(get(response, "errors")?) {
        None => Ok(()),
        Some(PyValue::List(errors)) => {
            let tolerated = errors.iter().all(|error| match error {
                PyValue::Dict(fields) => fields
                    .get("type")
                    .is_some_and(|t| py_eq_str(t, "NOT_FOUND")),
                _ => false,
            });
            if tolerated {
                Ok(())
            } else {
                Err(ReadError::github(message))
            }
        }
        Some(_) => Err(ReadError::github(message)),
    }
}

/// Whether a GraphQL answer is a dict whose `data` is one.
fn has_data(response: &PyValue) -> Read<bool> {
    Ok(matches!(response, PyValue::Dict(_))
        && matches!(get(response, "data")?, Some(PyValue::Dict(_))))
}

/// Whether a GraphQL answer is a dict that carries no errors.
fn clean(response: &PyValue) -> Read<bool> {
    Ok(matches!(response, PyValue::Dict(_)) && or_default(get(response, "errors")?).is_none())
}

/// `GitHub._pr(raw)`: a pull request's identity, from any route that reads
/// one. Every read carries the same fields, assignees beside labels: a
/// field one read can supply and another cannot is how three defects in a
/// day began.
pub fn pr_identity(raw: &PyValue) -> Result<PyValue, ReadError> {
    identity(raw)
        .key_or_type_as("Incomplete PR identity")
        .map(dict)
}

fn identity(raw: &PyValue) -> Read<PyDict> {
    let mut result = PyDict::new();
    result.insert("number".into(), item(raw, "number")?.clone());
    result.insert("author".into(), str_value(login(item(raw, "user")?)?));
    let bot = get_or_empty(get(raw, "user")?, "type")?.is_some_and(|t| py_eq_str(t, "Bot"));
    result.insert("author_is_bot".into(), PyValue::Bool(bot));
    let body = or_default(get(raw, "body")?)
        .cloned()
        .unwrap_or_else(|| str_value(""));
    result.insert("body".into(), body);
    let mut labels = Vec::new();
    for label in iterate_or_empty(get(raw, "labels")?)? {
        labels.push(str_value(text(Some(item(&label, "name")?), "label name")?));
    }
    result.insert("labels".into(), list(labels));
    let mut assignees = Vec::new();
    for user in iterate_or_empty(get(raw, "assignees")?)? {
        assignees.push(str_value(login(&user)?));
    }
    result.insert("assignees".into(), list(assignees));
    let head = text(Some(item(item(raw, "head")?, "sha")?), "head SHA")?;
    result.insert("head".into(), str_value(head));
    let base = item(raw, "base")?;
    result.insert(
        "base".into(),
        str_value(text(Some(item(base, "ref")?), "base branch")?),
    );
    let base = item(raw, "base")?;
    result.insert(
        "base_sha".into(),
        str_value(text(Some(item(base, "sha")?), "base SHA")?),
    );
    result.insert(
        "created_at".into(),
        str_value(text(Some(item(raw, "created_at")?), "PR creation time")?),
    );
    result.insert("draft".into(), item(raw, "draft")?.clone());
    result.insert("state".into(), item(raw, "state")?.clone());
    result.insert(
        "url".into(),
        str_value(text(Some(item(raw, "html_url")?), "PR URL")?),
    );
    result.insert("title".into(), item(raw, "title")?.clone());
    let numbered = matches!(&result["number"], PyValue::Int(n) if *n >= PyInt::from(1));
    if !numbered || !matches!(result["draft"], PyValue::Bool(_)) {
        return Err(ReadError::github("Invalid PR number or draft state"));
    }
    if !py_in_str_set(&result["state"], &["open", "closed"])?
        || !matches!(result["title"], PyValue::Str(_))
    {
        return Err(ReadError::github("Invalid PR state or title"));
    }
    Ok(result)
}

/// `users.update(values)`: every element added once, as a set holds it.
fn add_users(users: &mut Vec<PyValue>, values: &PyValue) -> Read<()> {
    for value in iterate(values)? {
        py_hashable(&value)?;
        if !users.iter().any(|known| py_same_element(known, &value)) {
            users.push(value.into_owned());
        }
    }
    Ok(())
}

/// `sorted(users)`, each then lowered: strings in code point order. A set
/// that mixes strings with anything else cannot be sorted; one that holds
/// no string at all sorts, and its first element has no `lower`.
fn sorted_logins(users: Vec<PyValue>) -> Read<Vec<String>> {
    let mut logins = Vec::with_capacity(users.len());
    let mut other = None;
    for user in &users {
        match user {
            PyValue::Str(login) => logins.push(login.clone()),
            value => other = other.or(Some(value)),
        }
    }
    if let Some(value) = other {
        let numeric =
            |v: &PyValue| matches!(v, PyValue::Int(_) | PyValue::Float(_) | PyValue::Bool(_));
        if users.len() == 1 || users.iter().all(numeric) {
            str_method(value, "lower")?;
        }
        return Err(ReadError::exception(
            PyClass::TypeError,
            "'<' not supported between the users being sorted",
        ));
    }
    logins.sort();
    Ok(logins)
}

impl<T: Transport> GitHub<T> {
    /// `GitHub(repo)`: the reads of `owner/name`, through `client`.
    pub fn new(repo: &str, client: Client<T>) -> Result<Self, ReadError> {
        let part = |p: &str| {
            !p.is_empty()
                && p.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'))
        };
        let parts: Vec<&str> = repo.split('/').collect();
        if parts.len() != 2 || !parts.iter().all(|p| part(p)) {
            return Err(ReadError::github("Expected repository identity owner/name"));
        }
        if parts.iter().any(|p| *p == "." || *p == "..") {
            return Err(ReadError::github("Invalid repository identity"));
        }
        Ok(GitHub {
            repo: repo.to_owned(),
            owner: parts[0].to_owned(),
            name: parts[1].to_owned(),
            root: format!("repos/{repo}"),
            client,
            permissions: IndexMap::new(),
            statuses: IndexMap::new(),
            builds: IndexMap::new(),
            listed: false,
            diagnostics: Vec::new(),
        })
    }

    pub fn repo(&self) -> &str {
        &self.repo
    }

    pub fn client(&self) -> &Client<T> {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut Client<T> {
        &mut self.client
    }

    /// What the reader would have printed to its log, oldest first.
    pub fn take_diagnostics(&mut self) -> Vec<String> {
        std::mem::take(&mut self.diagnostics)
    }

    /// GraphQL variables: the repository, then `extra` in order.
    fn variables(&self, extra: Vec<(&str, PyValue)>) -> PyValue {
        let mut entries = PyDict::new();
        entries.insert("owner".into(), str_value(self.owner.as_str()));
        entries.insert("repo".into(), str_value(self.name.as_str()));
        for (key, value) in extra {
            entries.insert(key.into(), value);
        }
        dict(entries)
    }

    /// `GitHub.pull(number)`: one pull request's identity.
    pub fn pull(&mut self, number: &PyInt) -> Result<PyValue, ReadError> {
        let raw =
            self.client
                .request(Method::Get, &format!("{}/pulls/{number}", self.root), None)?;
        pr_identity(&raw)
    }

    /// `GitHub.open_prs()`: every open pull request's identity.
    pub fn open_prs(&mut self) -> Result<Vec<PyValue>, ReadError> {
        let raws = self
            .client
            .pages(&format!("{}/pulls?state=open", self.root))?;
        let prs = raws.iter().map(pr_identity).collect::<Read<Vec<_>>>()?;
        let numbers = prs.iter().filter_map(|pr| match pr {
            PyValue::Dict(fields) => match fields.get("number") {
                Some(PyValue::Int(n)) => Some(n.clone()),
                _ => None,
            },
            _ => None,
        });
        unique(numbers, "PR")?;
        Ok(prs)
    }

    /// `GitHub.comments(number)`: the comments from the REST listing.
    ///
    /// That route carries no editor, and says so: who rewrote an edited
    /// comment is unknown there, so the record reader refuses comments read
    /// from it, since a forged record would pass for one nobody edited.
    pub fn comments(&mut self, number: &PyInt) -> Result<Vec<PyValue>, ReadError> {
        let raws = self
            .client
            .pages(&format!("{}/issues/{number}/comments", self.root))?;
        let read = || -> Read<Vec<PyValue>> {
            let mut comments = Vec::with_capacity(raws.len());
            for raw in &raws {
                let mut comment = PyDict::new();
                comment.insert("id".into(), item(raw, "id")?.clone());
                comment.insert("user".into(), str_value(login(item(raw, "user")?)?));
                comment.insert("body".into(), item(raw, "body")?.clone());
                comment.insert(
                    "created_at".into(),
                    str_value(text(
                        Some(item(raw, "created_at")?),
                        "comment creation time",
                    )?),
                );
                comment.insert(
                    "updated_at".into(),
                    str_value(text(Some(item(raw, "updated_at")?), "comment update time")?),
                );
                comment.insert("edited_by".into(), str_value(EDITOR_UNKNOWN));
                comments.push(dict(comment));
            }
            checked_comments(&comments)?;
            Ok(comments)
        };
        read().key_or_type_as("Incomplete issue comments")
    }

    /// `GitHub.activity(number)`: when the pull request was last closed or
    /// converted to a draft, from its whole timeline. Finds slot releases
    /// even when the close/reopen or draft events themselves were missed.
    pub fn activity(&mut self, number: &PyInt) -> Result<Option<String>, ReadError> {
        let events = self
            .client
            .pages(&format!("{}/issues/{number}/timeline", self.root))?;
        let mut latest: Option<(PyDateTime, String)> = None;
        for event in &events {
            let kind = match event {
                PyValue::Dict(fields) => fields.get("event"),
                _ => None,
            };
            let Some(PyValue::Str(kind)) = kind else {
                return Err(ReadError::github("Incomplete issue timeline"));
            };
            if kind != "closed" && kind != "convert_to_draft" {
                continue;
            }
            let value = text(get(event, "created_at")?, "inactive lifecycle timestamp")?;
            let invalid = || ReadError::github("Invalid inactive lifecycle timestamp");
            let instant = match PyDateTime::fromisoformat(&value.replace('Z', "+00:00")) {
                Ok(instant) if instant.utcoffset().is_some() => instant,
                Ok(_) | Err(PyErr::Value(_)) => return Err(invalid()),
                Err(other) => return Err(other.into()),
            };
            // `max(transitions)` over `(instant, text)`: equal instants fall
            // to the larger text, and equal pairs keep the first.
            let later = match &latest {
                None => true,
                Some((kept, kept_text)) => match instant.py_cmp(kept)? {
                    Ordering::Greater => true,
                    Ordering::Less => false,
                    Ordering::Equal => value > kept_text.as_str(),
                },
            };
            if later {
                latest = Some((instant, value.to_owned()));
            }
        }
        Ok(latest.map(|(_, value)| value))
    }

    /// `GitHub.histories(numbers)`: comments and the latest inactive
    /// transition for many pull requests in one query. A pull request that
    /// no longer resolves is left out.
    pub fn histories(&mut self, numbers: &[PyInt]) -> Result<IndexMap<PyInt, History>, ReadError> {
        if numbers.is_empty() {
            return Ok(IndexMap::new());
        }
        let mut wanted = numbers.to_vec();
        wanted.sort();
        wanted.dedup();
        let query = queries::histories(&wanted);
        let variables = self.variables(Vec::new());
        let response = self.client.graphql(&query, variables)?;
        if !has_data(&response)? {
            return Err(ReadError::github("GraphQL history query failed"));
        }
        only_not_found(&response, "GraphQL history query failed")?;
        let repository = match get(item(&response, "data")?, "repository")? {
            Some(repository @ PyValue::Dict(_)) => repository,
            _ => {
                return Err(ReadError::github(
                    "GraphQL history query returned no repository",
                ))
            }
        };
        let mut histories = IndexMap::new();
        for number in &wanted {
            let node = get(repository, &queries::history_alias(number))?;
            let Some(node) = node.filter(|node| !matches!(node, PyValue::None)) else {
                continue;
            };
            let read = self
                .history_of(number, node)
                .key_or_type_as("Incomplete pull request history")?;
            if let Some(history) = read {
                histories.insert(number.clone(), history);
            }
        }
        Ok(histories)
    }

    fn history_of(&mut self, number: &PyInt, node: &PyValue) -> Read<Option<History>> {
        let connection = item(node, "comments")?;
        let total = item(connection, "totalCount")?;
        let nodes = item(connection, "nodes")?;
        let (PyValue::Int(total), PyValue::List(window)) = (total, nodes) else {
            return Err(ReadError::github("Incomplete comment connection"));
        };
        // Older than the window asked for: read all of it rather than miss
        // the engine's own record, and still with who last edited each
        // comment, without which a forged record cannot be told from the
        // engine's own.
        let paged;
        let nodes: &[PyValue] = if *total > PyInt::from(window.len() as i64) {
            match self.comment_pages(number)? {
                Some(all) => {
                    paged = all;
                    &paged
                }
                None => return Ok(None),
            }
        } else {
            window
        };
        let comments = nodes
            .iter()
            .map(graphql_comment)
            .collect::<Read<Vec<_>>>()?;
        checked_comments(&comments)?;
        let events = item(item(node, "timelineItems")?, "nodes")?;
        let PyValue::List(events) = events else {
            return Err(ReadError::github("Incomplete pull request timeline"));
        };
        let lifecycle_at = match events.first() {
            Some(event) => Some(
                text(
                    Some(item(event, "createdAt")?),
                    "inactive lifecycle timestamp",
                )?
                .to_owned(),
            ),
            None => None,
        };
        Ok(Some(History {
            comments,
            lifecycle_at,
        }))
    }

    /// `GitHub._comment_pages(number)`: every comment on one pull request,
    /// a page at a time, each with its editor; `None` when the pull request
    /// no longer resolves.
    ///
    /// The pages must hold the whole conversation. A comment posted during
    /// the read lands on the last page and is counted by that page's total,
    /// so the last total is what they are held to; one deleted from a page
    /// already read leaves more read than that, a superset with nothing
    /// missing, and is kept. Fewer than the total means part of the
    /// conversation was not read, and the read is refused.
    fn comment_pages(&mut self, number: &PyInt) -> Read<Option<Vec<PyValue>>> {
        let mut nodes = Vec::new();
        let mut after = PyValue::None;
        let mut total;
        loop {
            let variables = self.variables(vec![
                ("number", PyValue::Int(number.clone())),
                ("after", after.clone()),
            ]);
            let response = self.client.graphql(queries::COMMENT_PAGES, variables)?;
            if !has_data(&response)? {
                return Err(ReadError::github("GraphQL comment query failed"));
            }
            only_not_found(&response, "GraphQL comment query failed")?;
            let page = (|| -> Read<Option<[PyValue; 4]>> {
                let pull = item(item(item(&response, "data")?, "repository")?, "pullRequest")?;
                if matches!(pull, PyValue::None) {
                    return Ok(None);
                }
                let connection = item(pull, "comments")?;
                let total = item(connection, "totalCount")?.clone();
                let page = item(connection, "nodes")?.clone();
                let more = item(item(connection, "pageInfo")?, "hasNextPage")?.clone();
                let cursor = item(item(connection, "pageInfo")?, "endCursor")?.clone();
                Ok(Some([total, page, more, cursor]))
            })()
            .key_or_type_as("Incomplete comment page")?;
            let Some([count, page, more, cursor]) = page else {
                return Ok(None);
            };
            let incomplete = || ReadError::github("Incomplete comment page");
            let (PyValue::Int(count), PyValue::List(page), PyValue::Bool(more)) =
                (count, page, more)
            else {
                return Err(incomplete());
            };
            // More to read must come with somewhere new to read it from, or
            // the same page could be asked for again for ever.
            if more
                && (page.is_empty()
                    || !matches!(&cursor, PyValue::Str(c) if !c.is_empty())
                    || py_eq(&cursor, &after))
            {
                return Err(incomplete());
            }
            total = count;
            nodes.extend(page);
            if !more {
                break;
            }
            after = cursor;
        }
        if PyInt::from(nodes.len() as i64) < total {
            return Err(ReadError::github(
                "Comment pages hold fewer comments than the conversation",
            ));
        }
        Ok(Some(nodes))
    }

    /// `GitHub.threads(number)`: every review thread, with everyone who
    /// spoke in it and the severity of a review bot's findings — not their
    /// text, which CodeRabbit appends to once a finding is addressed and
    /// which would otherwise read as the evidence changing under a write.
    pub fn threads(&mut self, number: &PyInt) -> Result<Vec<PyValue>, ReadError> {
        let mut cursor = PyValue::None;
        let mut seen: HashSet<String> = HashSet::new();
        let mut results = Vec::new();
        let mut emptied = 0usize;
        let mut total: Option<PyInt> = None;
        loop {
            let variables = self.variables(vec![
                ("number", PyValue::Int(number.clone())),
                ("cursor", cursor.clone()),
            ]);
            let response = self.client.graphql(queries::THREADS, variables)?;
            if !clean(&response)? {
                return Err(ReadError::github("GraphQL review-thread query failed"));
            }
            let next = thread_page(&response, &mut total, &mut results, &mut emptied, &mut seen)
                .key_or_type_as("Incomplete review-thread evidence")?;
            match next {
                Some(next) => cursor = str_value(next),
                None => break,
            }
        }
        let read = PyInt::from((results.len() + emptied) as i64);
        if total.as_ref() != Some(&read) {
            return Err(ReadError::github("Incomplete review-thread list"));
        }
        let ids = results.iter().filter_map(|thread| match get(thread, "id") {
            Ok(Some(PyValue::Str(id))) => Some(id.clone()),
            _ => None,
        });
        unique(ids, "review thread")?;
        Ok(results)
    }

    /// `GitHub.build_state(number, head)`: this head's build. A partial
    /// answer is never read as no checks, and a head that moved under the
    /// read is unfinished, never a stale green. Cached per head.
    pub fn build_state(&mut self, number: &PyInt, head: &str) -> Result<Build, ReadError> {
        if let Some(verdict) = self.builds.get(head) {
            return Ok(*verdict);
        }
        let mut nodes = Vec::new();
        let mut cursor = PyValue::None;
        loop {
            let variables = self.variables(vec![
                ("number", PyValue::Int(number.clone())),
                ("cursor", cursor.clone()),
            ]);
            let response = self.client.graphql(queries::BUILD, variables)?;
            if !clean(&response)? {
                return Err(ReadError::github("Build state query failed"));
            }
            let commits = (|| -> Read<&PyValue> {
                let pull = item(item(item(&response, "data")?, "repository")?, "pullRequest")?;
                item(item(pull, "commits")?, "nodes")
            })()
            .key_or_type_as("Build state unavailable")?;
            if !commits.truthy() {
                return Err(ReadError::github("Build state unavailable"));
            }
            let commit = item(first(commits)?, "commit")?;
            if !get(commit, "oid")?.is_some_and(|oid| py_eq_str(oid, head)) {
                return Ok(Build::Running);
            }
            let rollup = match get(commit, "statusCheckRollup")? {
                None | Some(PyValue::None) => {
                    // No checks at all, and no error to explain it away.
                    // Several governed repositories have none.
                    self.builds.insert(head.to_owned(), Build::Green);
                    return Ok(Build::Green);
                }
                Some(rollup) => rollup,
            };
            let connection = item(rollup, "contexts")?;
            let page = item(connection, "nodes")?;
            let info = item(connection, "pageInfo")?;
            let (PyValue::List(page), PyValue::Dict(_)) = (page, info) else {
                return Err(ReadError::github("Incomplete check connection"));
            };
            nodes.extend(page.iter().cloned());
            if or_default(get(info, "hasNextPage")?).is_none() {
                break;
            }
            cursor = get(info, "endCursor")?.cloned().unwrap_or(PyValue::None);
            if !cursor.truthy() {
                return Err(ReadError::github("Check pagination did not advance"));
            }
        }
        let verdict = build_verdict(&nodes)?;
        self.builds.insert(head.to_owned(), verdict);
        Ok(verdict)
    }

    /// `GitHub.snapshot(number, policy, history)`: the full evidence for one
    /// pull request, with its keys in Python's order.
    ///
    /// `history` is the comments and latest inactive transition already read
    /// for it by [`GitHub::histories`]; without it both are read here, the
    /// comments by the same batched route, so every read of one pull request
    /// sees the same comments with the same fields. `policy` is one
    /// `validate_policy` accepted.
    pub fn snapshot(
        &mut self,
        number: &PyInt,
        policy: &PyValue,
        history: Option<&History>,
    ) -> Result<PyValue, ReadError> {
        self.take_snapshot(number, policy, history)
            .key_or_type_as("Incomplete PR snapshot")
            .map(dict)
    }

    fn take_snapshot(
        &mut self,
        number: &PyInt,
        policy: &PyValue,
        history: Option<&History>,
    ) -> Read<PyDict> {
        let raw =
            self.client
                .request(Method::Get, &format!("{}/pulls/{number}", self.root), None)?;
        let PyValue::Dict(mut result) = pr_identity(&raw)? else {
            return Err(ReadError::github("Incomplete PR identity"));
        };
        let count = item(&raw, "changed_files")?;
        if !matches!(count, PyValue::Int(c) if *c >= PyInt::from(0) && *c <= PyInt::from(3000)) {
            return Err(ReadError::github(
                "Changed-file count unavailable or above GitHub's 3000-file limit",
            ));
        }
        let files = self
            .client
            .pages(&format!("{}/pulls/{number}/files", self.root))?;
        // Against the paths, not the entries: a file whose type changed is
        // listed twice, removed and added, while GitHub counts the path once.
        let mut paths = HashSet::new();
        for file in &files {
            paths.insert(text(Some(item(file, "filename")?), "changed-file path")?);
        }
        if !matches!(count, PyValue::Int(c) if *c == PyInt::from(paths.len() as i64)) {
            return Err(ReadError::github("Incomplete changed-file list"));
        }
        let mut changed = Vec::with_capacity(files.len());
        for file in &files {
            changed.push(normalized_file(file)?);
        }
        // The same path twice is the listing drifting under the read, unless
        // it is one path changing type: removed and added.
        unique(
            changed.iter().map(|file| {
                let field = |key| match get(file, key) {
                    Ok(Some(PyValue::Str(s))) => Some(s.clone()),
                    _ => None,
                };
                (field("filename"), field("status"))
            }),
            "changed file",
        )?;
        result.insert("files".into(), list(changed.clone()));

        let mut reviews = Vec::new();
        for review in self
            .client
            .pages(&format!("{}/pulls/{number}/reviews", self.root))?
        {
            let state = text(Some(item(&review, "state")?), "review state")?;
            if ![
                "APPROVED",
                "CHANGES_REQUESTED",
                "COMMENTED",
                "DISMISSED",
                "PENDING",
            ]
            .contains(&state)
            {
                return Err(ReadError::github("Unknown review state"));
            }
            if state == "PENDING" {
                continue;
            }
            let mut entry = PyDict::new();
            entry.insert("id".into(), item(&review, "id")?.clone());
            entry.insert("user".into(), str_value(login(item(&review, "user")?)?));
            entry.insert("state".into(), str_value(state));
            entry.insert(
                "commit_id".into(),
                str_value(text(Some(item(&review, "commit_id")?), "review commit")?),
            );
            entry.insert(
                "submitted_at".into(),
                str_value(text(Some(item(&review, "submitted_at")?), "review time")?),
            );
            entry.insert("body".into(), item(&review, "body")?.clone());
            reviews.push(dict(entry));
        }
        let valid = |r: &PyValue| {
            matches!(get(r, "id"), Ok(Some(PyValue::Int(_))))
                && matches!(get(r, "body"), Ok(Some(PyValue::Str(_))))
        };
        if !reviews.iter().all(valid) {
            return Err(ReadError::github("Invalid review identity or body"));
        }
        unique(reviews.iter().filter_map(int_id), "review")?;
        result.insert("reviews".into(), list(reviews.clone()));

        // One route for comments, always: the batched one, which names who
        // last edited each. Two routes, one able to supply a field the other
        // cannot, gave one pull request two different answers in one run.
        let comments = match history {
            Some(history) => history.comments.clone(),
            None => self
                .histories(std::slice::from_ref(number))?
                .shift_remove(number)
                .map(|history| history.comments)
                .unwrap_or_default(),
        };
        result.insert("comments".into(), list(comments.clone()));
        let threads = self.threads(number)?;
        result.insert("threads".into(), list(threads.clone()));
        let lifecycle_at = match history {
            Some(history) => history.lifecycle_at.clone(),
            None => self.activity(number)?,
        };
        result.insert("lifecycle_at".into(), optional(lifecycle_at));
        let head = match result.get("head") {
            Some(PyValue::Str(head)) => head.clone(),
            _ => return Err(ReadError::github("Missing or invalid head SHA")),
        };
        let seen = self.head_seen_at(&head)?;
        result.insert("head_seen_at".into(), optional(seen));
        let build = self.build_state(number, &head)?;
        result.insert("build".into(), str_value(build.as_str()));
        let ready = self.ready_published(&head)?;
        result.insert("ready_published".into(), PyValue::Bool(ready));
        let mut requested = Vec::new();
        for user in iterate(item(&raw, "requested_reviewers")?)? {
            requested.push(str_value(login(&user)?));
        }
        result.insert("requested_reviewers".into(), list(requested));
        let mut labels = Vec::new();
        for label in iterate(item(&raw, "labels")?)? {
            labels.push(str_value(text(Some(item(&label, "name")?), "label name")?));
        }
        result.insert("labels".into(), list(labels));

        let record = parse_controller_state(&comments)?;
        if let Some(record) = &record {
            if !py_eq(
                item(&record.state, "number")?,
                &PyValue::Int(number.clone()),
            ) {
                return Err(ReadError::github("Controller state belongs to another PR"));
            }
        }
        let (state, comment_id) = match record {
            Some(record) => (record.state, record.comment_id),
            None => (PyValue::None, PyValue::None),
        };
        result.insert("controller_state".into(), state);
        result.insert("controller_comment_id".into(), comment_id);
        let diff = parse_controller_diff(&comments, number)?;
        result.insert("controller_diff".into(), diff.unwrap_or(PyValue::None));

        let users = users_to_vouch_for(policy, &changed, &reviews, &threads, &comments)?;
        let mut permissions = PyDict::new();
        for user in sorted_logins(users)? {
            let lowered = py_lower(&user);
            if BOT_LOGINS.contains(&lowered.as_str()) || lowered.ends_with("[bot]") {
                continue;
            }
            // Absent from the collaborator list means no write access. The
            // per-person route would answer "read" for a stranger on a
            // public repository; every decision here asks only whether
            // someone can write, so the two agree where it counts.
            let level = self.permission(&user)?;
            permissions.insert(user, optional(level));
        }
        result.insert("permissions".into(), dict(permissions));
        result.insert("repo".into(), str_value(self.repo.as_str()));
        result.insert("complete".into(), PyValue::Bool(true));
        Ok(result)
    }

    /// `GitHub.access()`: every collaborator's level, read once per
    /// reconciliation. The listing is marked read before it is read, so a
    /// listing that failed is not asked again in the same run.
    pub fn access(&mut self) -> Result<&IndexMap<String, Option<String>>, ReadError> {
        if !self.listed {
            self.listed = true;
            let entries = self
                .client
                .pages(&format!("{}/collaborators?affiliation=all", self.root))?;
            for entry in &entries {
                let name = login(entry)?;
                let granted = get(entry, "permissions")?;
                let Some(granted @ PyValue::Dict(_)) = granted else {
                    return Err(ReadError::github(
                        "Collaborator listing is missing its permissions",
                    ));
                };
                self.permissions
                    .insert(py_lower(&name), capability(granted));
            }
        }
        Ok(&self.permissions)
    }

    /// `GitHub.permission(login)`: one person's level, asking directly when
    /// the listing did not name them. `None` means the answer is unknown,
    /// which is not the same as `none`.
    ///
    /// The listing answers for the collaborators a token can see, and a
    /// repository-scoped token does not enumerate the people who reach a
    /// repository through the organisation. An answer that cannot be read is
    /// remembered for the run: asking again on every snapshot turns one
    /// unreachable answer into hundreds of requests, and a rate limit into a
    /// loop. The pre-write re-read clears it.
    pub fn permission(&mut self, login: &str) -> Result<Option<String>, ReadError> {
        let key = py_lower(login);
        if let Some(level) = self.permissions.get(&key) {
            return Ok(level.clone());
        }
        self.access()?;
        if let Some(level) = self.permissions.get(&key) {
            return Ok(level.clone());
        }
        let path = format!("{}/collaborators/{}/permission", self.root, py_quote(login));
        let answer = match self.client.request(Method::Get, &path, None) {
            Ok(answer) => answer,
            Err(ReadError::GitHub(error)) => {
                self.diagnostics
                    .push(format!("Could not read access for {login}: {error}"));
                self.permissions.insert(key, None);
                return Ok(None);
            }
            Err(other) => return Err(other),
        };
        // The same capability flags the listing reads, not the legacy role
        // string beside them.
        let granted = match &answer {
            PyValue::Dict(fields) => match fields.get("user") {
                Some(user) => get(user, "permissions")?,
                None => None,
            },
            _ => None,
        };
        let mut level = granted.and_then(capability);
        if level.is_none() {
            if let PyValue::Dict(fields) = &answer {
                if let Some(legacy) = fields.get("permission") {
                    if py_in_str_set(legacy, &["admin", "write", "read", "none"])? {
                        level = match legacy {
                            PyValue::Str(legacy) => Some(legacy.clone()),
                            _ => None,
                        };
                    }
                }
            }
        }
        if level.is_none() {
            self.diagnostics
                .push(format!("Unreadable access answer for {login}"));
        }
        self.permissions.insert(key, level.clone());
        Ok(level)
    }

    /// `GitHub.forget_cached_access()`: read access, statuses and builds
    /// afresh, for the checks made immediately before writing.
    pub fn forget_cached_access(&mut self) {
        self.permissions.clear();
        self.statuses.clear();
        self.builds.clear();
        self.listed = false;
    }

    /// `GitHub._head_statuses(head)`: a commit's statuses, read once per run.
    fn head_statuses(&mut self, head: &str) -> Read<&[PyValue]> {
        if !self.statuses.contains_key(head) {
            let listed = self.client.pages(&format!(
                "{}/commits/{}/statuses",
                self.root,
                py_quote(head)
            ))?;
            self.statuses.insert(head.to_owned(), listed);
        }
        Ok(self.statuses.get(head).map_or(&[], Vec::as_slice))
    }

    /// `GitHub.head_seen_at(head)`: when the engine first published a status
    /// for this head. Statuses cannot be edited or deleted, so this is a
    /// time no author can move.
    pub fn head_seen_at(&mut self, head: &str) -> Result<Option<String>, ReadError> {
        let ours = engine_statuses(self.head_statuses(head)?)?;
        let mut stamps = Vec::with_capacity(ours.len());
        for status in ours {
            stamps.push(text(
                Some(item(status, "created_at")?),
                "status creation time",
            )?);
        }
        if stamps.iter().any(|stamp| !utc_timestamp(stamp)) {
            return Err(ReadError::github("Unexpected status timestamp format"));
        }
        Ok(py_min_by(stamps, |a, b| a.cmp(b)).map(str::to_owned))
    }

    /// `GitHub.ready_published(head)`: whether a human has ever been asked
    /// to review this head — a record no later run can take back, unlike
    /// the record comment, which every run rewrites.
    pub fn ready_published(&mut self, head: &str) -> Result<bool, ReadError> {
        for status in self.head_statuses(head)? {
            if engine_status(status)?
                && get(status, "description")?.is_some_and(|d| py_eq_str(d, "ready-for-human"))
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// `GitHub.latest_state_from_status(head)`: the description of the
    /// newest status the engine published for this head, or `None`. For a
    /// pull request with no record comment yet, the status is the only
    /// trace of its state.
    pub fn latest_state_from_status(&mut self, head: &str) -> Result<PyValue, ReadError> {
        // Every status is checked before any two are compared, as Python
        // lists the engine's own before taking the newest.
        let mine = engine_statuses(self.head_statuses(head)?)?;
        let mut kept: Option<(&PyValue, [PyValue; 2])> = None;
        for status in mine {
            let created = or_default(get(status, "created_at")?)
                .cloned()
                .unwrap_or_else(|| str_value(""));
            let id = or_default(get(status, "id")?)
                .cloned()
                .unwrap_or(PyValue::Int(PyInt::from(0)));
            let key = [created, id];
            let later = match &kept {
                None => true,
                Some((_, best)) => py_compare_sequences(&key, Compare::Gt, best)?,
            };
            if later {
                kept = Some((status, key));
            }
        }
        match kept {
            None => Ok(PyValue::None),
            Some((status, _)) => Ok(get(status, "description")?
                .cloned()
                .unwrap_or(PyValue::None)),
        }
    }
}

/// `commits[0]` of a value already known to be true.
fn first(commits: &PyValue) -> Read<&PyValue> {
    match commits {
        PyValue::List(items) => items
            .first()
            .ok_or_else(|| ReadError::github("Build state unavailable")),
        PyValue::Dict(_) => Err(ReadError::exception(PyClass::KeyError, "0")),
        // `"abc"[0]` is `"a"`, which then has no `["commit"]`.
        _ => Err(ReadError::exception(
            PyClass::TypeError,
            "commits is not a list",
        )),
    }
}

/// The statuses the engine posted, every one checked, in the order listed.
fn engine_statuses(statuses: &[PyValue]) -> Read<Vec<&PyValue>> {
    let mut ours = Vec::new();
    for status in statuses {
        if engine_status(status)? {
            ours.push(status);
        }
    }
    Ok(ours)
}

/// A status the engine posted under one of its identities.
fn engine_status(status: &PyValue) -> Read<bool> {
    if !get(status, "context")?.is_some_and(|c| py_eq_str(c, "PR Hygiene")) {
        return Ok(false);
    }
    is_engine(get_or_empty(get(status, "creator")?, "login")?)
}

/// A changed file as the snapshot keeps it: what it is, what happened to
/// it, what it holds and the digest of its patch.
///
/// The patch, digested, because what the file holds is not enough: a
/// conflict resolved by keeping your own side leaves the file byte for byte
/// what the reviewer saw while the patch against the moved base now also
/// undoes what the base did. Where GitHub sends no patch nothing is carried,
/// except for a file the pull request adds, which the base does not have, so
/// nothing the base did can hide in its patch and its blob decides it.
fn normalized_file(file: &PyValue) -> Read<PyValue> {
    let mut normalized = PyDict::new();
    normalized.insert(
        "filename".into(),
        str_value(text(Some(item(file, "filename")?), "changed-file path")?),
    );
    let status = get(file, "status")?;
    let renamed = status.is_some_and(|s| py_eq_str(s, "renamed"));
    if renamed && or_default(get(file, "previous_filename")?).is_none() {
        return Err(ReadError::github("Renamed file is missing its source path"));
    }
    if let PyValue::Dict(fields) = file {
        if fields.contains_key("previous_filename") {
            normalized.insert(
                "previous_filename".into(),
                str_value(text(
                    Some(item(file, "previous_filename")?),
                    "rename source path",
                )?),
            );
        }
    }
    if let Some(PyValue::Str(status)) = status {
        normalized.insert("status".into(), str_value(status.as_str()));
    }
    if let Some(PyValue::Str(sha)) = get(file, "sha")? {
        if !sha.is_empty() {
            normalized.insert("content".into(), str_value(sha.as_str()));
        }
    }
    match get(file, "patch")? {
        Some(PyValue::Str(patch)) => {
            normalized.insert(
                "shape".into(),
                str_value(sha256_hexdigest(patch.as_bytes())),
            );
        }
        _ if status.is_some_and(|s| py_eq_str(s, "added")) => {
            normalized.insert("shape".into(), str_value("added"));
        }
        _ => {}
    }
    Ok(dict(normalized))
}

/// One page of review threads, appended to `results`; the cursor of the
/// next page, or `None` after the last.
fn thread_page(
    response: &PyValue,
    total: &mut Option<PyInt>,
    results: &mut Vec<PyValue>,
    emptied: &mut usize,
    seen: &mut HashSet<String>,
) -> Read<Option<String>> {
    let pull = item(item(item(response, "data")?, "repository")?, "pullRequest")?;
    let connection = item(pull, "reviewThreads")?;
    let count = match item(connection, "totalCount")? {
        PyValue::Int(count)
            if *count >= PyInt::from(0) && total.as_ref().is_none_or(|known| known == count) =>
        {
            count.clone()
        }
        _ => {
            return Err(ReadError::github(
                "Review thread count unavailable or changed during collection",
            ))
        }
    };
    *total = Some(count);
    let PyValue::List(nodes) = item(connection, "nodes")? else {
        return Err(ReadError::github("Missing review thread nodes"));
    };
    for node in nodes.iter() {
        let comments = item(item(node, "comments")?, "nodes")?;
        let opening = item(item(node, "opening")?, "nodes")?;
        let (PyValue::List(comments), PyValue::List(opening)) = (comments, opening) else {
            return Err(ReadError::github("Incomplete review thread"));
        };
        if !matches!(item(node, "isResolved")?, PyValue::Bool(_)) {
            return Err(ReadError::github("Incomplete review thread"));
        }
        if comments.is_empty() {
            // Every comment in the thread was deleted; nothing remains to resolve.
            *emptied += 1;
            continue;
        }
        let opening_body = match opening.first() {
            Some(first) => match item(first, "body")? {
                PyValue::Str(body) => body.clone(),
                _ => return Err(ReadError::github("Incomplete review thread")),
            },
            None => return Err(ReadError::github("Incomplete review thread")),
        };
        // Whoever opened the thread names it; whoever spoke in it can be
        // objecting.
        let author = login(item(&comments[0], "author")?)?;
        let mut thread = PyDict::new();
        thread.insert(
            "id".into(),
            str_value(text(Some(item(node, "id")?), "thread identity")?),
        );
        thread.insert("is_resolved".into(), item(node, "isResolved")?.clone());
        thread.insert("author".into(), str_value(author.as_str()));
        thread.insert(
            "created_at".into(),
            str_value(text(
                Some(item(&comments[0], "createdAt")?),
                "thread creation time",
            )?),
        );
        let mut voices = Vec::with_capacity(comments.len());
        for comment in comments.iter() {
            let mut voice = PyDict::new();
            voice.insert("user".into(), str_value(login(item(comment, "author")?)?));
            voice.insert(
                "created_at".into(),
                str_value(text(
                    Some(item(comment, "createdAt")?),
                    "thread comment time",
                )?),
            );
            voices.push(dict(voice));
        }
        thread.insert("voices".into(), list(voices));
        let severities = finding_severities(&author, &opening_body)
            .into_iter()
            .map(str_value)
            .collect();
        thread.insert("severities".into(), list(severities));
        results.push(dict(thread));
    }
    let info = item(connection, "pageInfo")?;
    let PyValue::Bool(more) = item(info, "hasNextPage")? else {
        return Err(ReadError::github("Missing review-thread pagination state"));
    };
    if !more {
        return Ok(None);
    }
    let cursor = text(Some(item(info, "endCursor")?), "review-thread cursor")?.to_owned();
    if !seen.insert(cursor.clone()) {
        return Err(ReadError::github(
            "Review-thread pagination did not advance",
        ));
    }
    if seen.len() >= 100 {
        return Err(ReadError::github(
            "Review-thread pagination exceeds collection bound",
        ));
    }
    Ok(Some(cursor))
}

/// Everyone whose access the snapshot reads: the owners and reviewers of
/// every area a changed path falls in (the fallback where none claims it),
/// every reviewer, every thread's author, and whoever told the engine to
/// stop waiting for the bots, since that has to be someone it can vouch for.
fn users_to_vouch_for(
    policy: &PyValue,
    files: &[PyValue],
    reviews: &[PyValue],
    threads: &[PyValue],
    comments: &[PyValue],
) -> Read<Vec<PyValue>> {
    let fallback = item(policy, "fallback")?;
    let wrapped;
    let fallback = if matches!(fallback, PyValue::Dict(_)) {
        fallback
    } else {
        let mut owners = PyDict::new();
        owners.insert("owners".into(), fallback.clone());
        wrapped = dict(owners);
        &wrapped
    };
    let mut groups: Vec<&PyValue> = Vec::new();
    for file in files {
        let filename = item(file, "filename")?;
        let previous = get(file, "previous_filename")?.unwrap_or(filename);
        let mut paths = vec![filename];
        if !py_eq(previous, filename) {
            paths.push(previous);
        }
        for path in paths {
            groups.push(area_of(policy, path)?.unwrap_or(fallback));
        }
    }
    let mut users = Vec::new();
    let empty = PyValue::List(PyList::new());
    for group in groups {
        add_users(&mut users, get(group, "owners")?.unwrap_or(&empty))?;
        add_users(&mut users, get(group, "reviewers")?.unwrap_or(&empty))?;
    }
    let field = |entry: &PyValue, key: &str| -> Read<PyValue> { Ok(item(entry, key)?.clone()) };
    let mut named = Vec::new();
    for review in reviews {
        named.push(field(review, "user")?);
    }
    for thread in threads {
        named.push(field(thread, "author")?);
    }
    for comment in comments {
        let body = str_method(item(comment, "body")?, "strip")?;
        if py_strip(body) == "/skip-bots" {
            named.push(field(comment, "user")?);
        }
    }
    add_users(&mut users, &PyValue::List(PyList::from(named)))?;
    Ok(users)
}

/// `next((area for area in policy["areas"] if any(path.startswith(prefix)
/// for prefix in area["paths"])), None)`.
fn area_of<'a>(policy: &'a PyValue, path: &PyValue) -> Read<Option<&'a PyValue>> {
    let type_error = |detail: &str| ReadError::exception(PyClass::TypeError, detail);
    for area in iterate(item(policy, "areas")?)? {
        // Only a list's items can be dicts: a dict's keys and a string's
        // characters have no `["paths"]`, and raise here.
        let prefixes = iterate(item(&area, "paths")?)?;
        for prefix in prefixes {
            let PyValue::Str(prefix) = prefix.as_ref() else {
                return Err(type_error(
                    "startswith first arg must be str or a tuple of str",
                ));
            };
            if str_method(path, "startswith")?.starts_with(prefix.as_str()) {
                return match area {
                    Cow::Borrowed(area) => Ok(Some(area)),
                    Cow::Owned(_) => Err(type_error("an area is not a dict")),
                };
            }
        }
    }
    Ok(None)
}
