//! A stateful fake of GitHub: one repository's pull requests, comments,
//! statuses, labels, descriptions and reviewer requests, held as state,
//! read the way GitHub answers each route the engine reads, and changed by
//! each write the way GitHub changes it.
//!
//! The corpus answers every write with a success and every later read as
//! GitHub was before the run, so it cannot show what the engine does with
//! its own writes read back, nor with a write that fails. This can: a
//! second run over the state the first left, a label that 404s, a reviewer
//! request refused with 422, a status that is not acknowledged.
//!
//! What it answers is modelled on GitHub's documented answers and on the
//! shapes the recordings hold, and has not been checked against GitHub's
//! answers to real writes: what a write returns, how an edit moves a
//! comment's times, that removing a label the pull request does not wear
//! is a 404, that an edit leaving the text as it was is no edit. Its state
//! holds exactly the fields those reads carry, every answer is made from
//! it by one function per route (`Fake::rest`, `Fake::graphql`), and ids
//! and times come from one counter and one clock, so a recording of real
//! writes and the reads after them can be set against it answer by answer.

use crate::support::*;
use pr_hygiene_engine::conformance::gh_printed;
use pr_hygiene_engine::evidence::replay::transient;
use pr_hygiene_engine::evidence::{Failure, FailureClass, Method};
use pr_hygiene_engine::pycompat::text::py_lower;
use std::collections::BTreeMap;

pub const REPO: &str = "dashpay/platform";
/// The engine's identity, as REST spells it.
pub const ENGINE: &str = "github-actions[bot]";

/// A comment, as GitHub keeps it.
#[derive(Debug, Clone)]
pub struct Comment {
    pub id: i64,
    /// The author's login as REST spells it: `name[bot]` for an app.
    pub author: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    /// When its text was last edited, if ever.
    pub last_edited_at: Option<String>,
    /// Who last edited it, as REST spells the login.
    pub editor: Option<String>,
}

impl Comment {
    pub fn new(id: i64, author: &str, body: &str, at: &str) -> Self {
        Comment {
            id,
            author: author.into(),
            body: body.into(),
            created_at: at.into(),
            updated_at: at.into(),
            last_edited_at: None,
            editor: None,
        }
    }

    /// The same comment, edited by `editor` at `at`.
    pub fn edited(mut self, editor: &str, at: &str) -> Self {
        self.updated_at = at.into();
        self.last_edited_at = Some(at.into());
        self.editor = Some(editor.into());
        self
    }
}

/// A pull request, as GitHub keeps it.
#[derive(Debug, Clone)]
pub struct Pr {
    pub number: i64,
    pub author: String,
    /// Whether GitHub marks the author's account a bot.
    pub bot_author: bool,
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub head: String,
    pub base: String,
    pub base_sha: String,
    pub created_at: String,
    pub draft: bool,
    pub state: String,
    /// The changed files, as the files route answers them.
    pub files: Vec<Value>,
    /// The reviews, as the reviews route answers them.
    pub reviews: Vec<Value>,
    pub comments: Vec<Comment>,
    /// The review threads, as GraphQL answers them.
    pub threads: Vec<Value>,
    pub requested_reviewers: Vec<String>,
    /// `(event, created_at)` of its timeline: `closed`, `convert_to_draft`.
    pub timeline: Vec<(String, String)>,
    /// The head's checks as GraphQL answers them; none at all is green.
    pub checks: Option<Vec<Value>>,
}

impl Pr {
    /// `fixture()`'s pull request: one file in `drive`, both review bots'
    /// final word on the head, and the author's attestation.
    pub fn new(number: i64, author: &str, head: &str) -> Self {
        Pr {
            number,
            author: author.into(),
            bot_author: false,
            title: format!("PR {number}"),
            body: "Some text.".into(),
            labels: Vec::new(),
            assignees: Vec::new(),
            head: head.into(),
            base: "v4.2-dev".into(),
            base_sha: "b".repeat(40),
            created_at: "2026-09-10T00:00:00Z".into(),
            draft: false,
            state: "open".into(),
            files: vec![json!({"filename": "packages/drive/a.rs", "status": "modified"})],
            reviews: vec![
                review(
                    10 * number + 1,
                    "thepastaclaw",
                    "COMMENTED",
                    head,
                    "2026-09-11T10:00:00Z",
                    &format!("<!-- thepastaclaw-review-phase v1 phase=final sha={head} -->"),
                ),
                review(
                    10 * number + 2,
                    "coderabbitai[bot]",
                    "APPROVED",
                    head,
                    "2026-09-11T10:00:00Z",
                    "",
                ),
            ],
            comments: vec![Comment::new(
                100 * number + 3,
                author,
                &format!("/self-reviewed {head}"),
                "2026-09-11T11:00:00Z",
            )],
            threads: Vec::new(),
            requested_reviewers: Vec::new(),
            timeline: Vec::new(),
            checks: None,
        }
    }
}

/// A review, as the reviews route answers it.
pub fn review(id: i64, user: &str, state: &str, commit: &str, at: &str, body: &str) -> Value {
    json!({"id": id, "user": {"login": user}, "state": state, "commit_id": commit,
           "submitted_at": at, "body": body})
}

/// A review thread, as GraphQL answers it: opened by `author` with
/// `opening`, and each later voice.
pub fn thread(id: &str, resolved: bool, opening: &str, voices: &[(&str, &str)]) -> Value {
    let comments: Vec<Value> = voices
        .iter()
        .map(|(login, at)| json!({"author": {"login": login}, "createdAt": at}))
        .collect();
    json!({"id": id, "isResolved": resolved, "opening": {"nodes": [{"body": opening}]},
           "comments": {"nodes": comments}})
}

/// What a failing call answers, as `gh` reports it.
#[derive(Debug, Clone)]
pub enum Refusal {
    /// GitHub answers this HTTP status and message, and `gh` exits 1.
    Http(u16, String),
    /// The call succeeds with nothing in the answer.
    Empty,
}

/// A refusal, for calls whose method is `method` and whose route under the
/// repository (or `graphql`) starts with `route`.
#[derive(Debug, Clone)]
struct Rule {
    method: Method,
    route: String,
    refusal: Refusal,
    /// Refuse only this many times; `None` for always.
    times: Option<usize>,
}

/// A change made to the state when a call is made: what another run, a
/// reviewer or a push does while the engine works.
pub type Hook = Box<dyn FnMut(&mut State, &Call)>;

/// A write that reached the fake: its method, its route under the
/// repository, and its body.
#[derive(Debug, Clone)]
pub struct Written {
    pub method: Method,
    pub route: String,
    pub body: Option<PyValue>,
}

impl Written {
    /// The route with its method, as a test names it: `POST statuses/…`.
    pub fn named(&self) -> String {
        format!("{} {}", self.method, self.route)
    }

    /// A field of the body.
    pub fn field(&self, key: &str) -> &PyValue {
        field(self.body.as_ref().expect("a body"), key)
    }
}

/// The repository's state.
pub struct State {
    pub now: String,
    pub prs: BTreeMap<i64, Pr>,
    /// Each commit's statuses, newest first.
    pub statuses: BTreeMap<String, Vec<Value>>,
    /// `(login, level)` of everyone the collaborator listing names.
    pub collaborators: Vec<(String, String)>,
    /// The labels the repository defines.
    pub labels: Vec<String>,
    next_id: i64,
}

impl State {
    pub fn pr(&mut self, number: i64) -> &mut Pr {
        self.prs
            .get_mut(&number)
            .expect("a pull request the fake holds")
    }

    fn id(&mut self) -> i64 {
        self.next_id += 1;
        self.next_id
    }

    /// A status the engine posted, as the statuses route lists it.
    pub fn engine_status(&mut self, head: &str, state: &str, description: &str, at: &str) {
        let id = self.id();
        self.statuses.entry(head.into()).or_default().insert(
            0,
            json!({"id": id, "state": state, "context": "PR Hygiene", "description": description,
                   "target_url": null, "creator": {"login": ENGINE, "type": "Bot"},
                   "created_at": at, "updated_at": at}),
        );
    }
}

/// The fake: the state, the calls made of it, and the refusals and hooks a
/// test arranged.
pub struct Fake {
    pub state: State,
    pub calls: Vec<Call>,
    pub written: Vec<Written>,
    /// Whether it answers as `gh api` prints GitHub's answers, as the
    /// Python engine reads them, rather than as GitHub sends them.
    pub prints_as_gh: bool,
    rules: Vec<Rule>,
    hooks: Vec<Hook>,
}

fn level_flags(level: &str) -> Value {
    let rank = ["pull", "triage", "push", "maintain", "admin"];
    let at = match level {
        "admin" => 4,
        "maintain" => 3,
        "write" => 2,
        "triage" => 1,
        _ => 0,
    };
    let mut flags = serde_json::Map::new();
    for (i, name) in rank.iter().enumerate() {
        flags.insert((*name).into(), Value::Bool(i <= at));
    }
    Value::Object(flags)
}

/// A login as GraphQL spells it, and whether it is an app's.
fn graphql_actor(login: &str) -> Value {
    match login.strip_suffix("[bot]") {
        Some(bare) => json!({"login": bare, "__typename": "Bot"}),
        None => json!({"login": login, "__typename": "User"}),
    }
}

fn rest_user(login: &str) -> Value {
    let kind = if login.ends_with("[bot]") {
        "Bot"
    } else {
        "User"
    };
    json!({"login": login, "type": kind})
}

impl Fake {
    /// The repository at `now`, with the policy fixture's three people able
    /// to write and every label the engine sets defined.
    pub fn new(now: &str) -> Self {
        Fake {
            state: State {
                now: now.into(),
                prs: BTreeMap::new(),
                statuses: BTreeMap::new(),
                collaborators: ["owner", "reviewer", "fallback"]
                    .iter()
                    .map(|login| ((*login).into(), "write".into()))
                    .collect(),
                labels: [
                    "waiting-bots",
                    "waiting-self-review",
                    "too-many-open-prs",
                    "ready-for-human",
                    "bot-review-skipped",
                ]
                .iter()
                .map(|label| (*label).into())
                .collect(),
                next_id: 1_000,
            },
            calls: Vec::new(),
            written: Vec::new(),
            prints_as_gh: false,
            rules: Vec::new(),
            hooks: Vec::new(),
        }
    }

    pub fn add(&mut self, pr: Pr) -> &mut Self {
        self.state.prs.insert(pr.number, pr);
        self
    }

    pub fn pr(&mut self, number: i64) -> &mut Pr {
        self.state.pr(number)
    }

    /// Refuse every call of `method` to a route starting with `route`.
    pub fn refuse(&mut self, method: Method, route: &str, refusal: Refusal) -> &mut Self {
        self.rules.push(Rule {
            method,
            route: route.into(),
            refusal,
            times: None,
        });
        self
    }

    /// Refuse the next such call only.
    pub fn refuse_once(&mut self, method: Method, route: &str, refusal: Refusal) -> &mut Self {
        self.rules.push(Rule {
            method,
            route: route.into(),
            refusal,
            times: Some(1),
        });
        self
    }

    /// Change the state whenever a call is made, before it is answered.
    pub fn on_call(&mut self, hook: impl FnMut(&mut State, &Call) + 'static) -> &mut Self {
        self.hooks.push(Box::new(hook));
        self
    }

    /// The writes made, in order.
    pub fn writes(&self) -> Vec<String> {
        self.written.iter().map(Written::named).collect()
    }

    /// Forget the calls and writes made so far, keeping the state.
    pub fn forget_calls(&mut self) {
        self.calls.clear();
        self.written.clear();
    }

    /// The REST answer for a pull request.
    fn pull_json(pr: &Pr) -> Value {
        let mut paths: Vec<&str> = pr
            .files
            .iter()
            .filter_map(|f| f["filename"].as_str())
            .collect();
        paths.sort();
        paths.dedup();
        json!({
            "number": pr.number,
            "user": {"login": pr.author, "type": if pr.bot_author { "Bot" } else { "User" }},
            "body": pr.body,
            "labels": pr.labels.iter().map(|name| json!({"name": name})).collect::<Vec<_>>(),
            "assignees": pr.assignees.iter().map(|login| json!({"login": login})).collect::<Vec<_>>(),
            "head": {"sha": pr.head}, "base": {"ref": pr.base, "sha": pr.base_sha},
            "created_at": pr.created_at, "draft": pr.draft, "state": pr.state,
            "html_url": format!("https://github.com/{REPO}/pull/{}", pr.number), "title": pr.title,
            "changed_files": paths.len(),
            "requested_reviewers": pr.requested_reviewers.iter()
                .map(|login| json!({"login": login, "type": "User"})).collect::<Vec<_>>(),
        })
    }

    fn comment_rest(comment: &Comment) -> Value {
        json!({"id": comment.id, "user": rest_user(&comment.author), "body": comment.body,
               "created_at": comment.created_at, "updated_at": comment.updated_at})
    }

    fn comment_node(comment: &Comment) -> Value {
        json!({"databaseId": comment.id, "body": comment.body, "createdAt": comment.created_at,
               "updatedAt": comment.updated_at, "lastEditedAt": comment.last_edited_at,
               "author": graphql_actor(&comment.author),
               "editor": comment.editor.as_deref().map(graphql_actor)})
    }

    /// The latest closing or conversion to draft, as `timelineItems(last:1)`
    /// answers it.
    fn lifecycle(pr: &Pr) -> Vec<Value> {
        pr.timeline
            .iter()
            .rfind(|(event, _)| event == "closed" || event == "convert_to_draft")
            .map(|(_, at)| vec![json!({"createdAt": at})])
            .unwrap_or_default()
    }

    fn graphql(&mut self, query: &str, variables: &PyValue) -> Result<Value, String> {
        let number = || match variables {
            PyValue::Dict(v) => match v.get("number") {
                Some(PyValue::Int(n)) => n.as_i64().ok_or("a number"),
                _ => Err("no number"),
            },
            _ => Err("no variables"),
        };
        let graph = |pull: Value| json!({"data": {"repository": {"pullRequest": pull}}});
        if query.contains("fragment history") {
            let alias = regex::Regex::new(r"pr([0-9]+): pullRequest").expect("a pattern");
            let mut repository = serde_json::Map::new();
            let mut errors = Vec::new();
            for found in alias.captures_iter(query) {
                let n: i64 = found[1].parse().map_err(|_| "a number")?;
                let key = format!("pr{n}");
                match self.state.prs.get(&n) {
                    Some(pr) => {
                        let nodes: Vec<Value> = pr
                            .comments
                            .iter()
                            .skip(pr.comments.len().saturating_sub(100))
                            .map(Self::comment_node)
                            .collect();
                        repository.insert(
                            key,
                            json!({"number": n,
                            "comments": {"totalCount": pr.comments.len(), "nodes": nodes},
                            "timelineItems": {"nodes": Self::lifecycle(pr)}}),
                        );
                    }
                    None => {
                        repository.insert(key.clone(), Value::Null);
                        errors.push(json!({"type": "NOT_FOUND", "path": ["repository", key],
                            "message": format!("Could not resolve to a PullRequest with the number of {n}.")}));
                    }
                }
            }
            let mut answer = json!({"data": {"repository": repository}});
            if !errors.is_empty() {
                answer["errors"] = Value::Array(errors);
            }
            return Ok(answer);
        }
        let n = number()?;
        let Some(pr) = self.state.prs.get(&n) else {
            return Ok(graph(Value::Null));
        };
        if query.contains("comments(first:100, after:$after)") {
            let start = match variables {
                PyValue::Dict(v) => match v.get("after") {
                    Some(PyValue::Str(after)) => after[1..].parse().map_err(|_| "a cursor")?,
                    _ => 0,
                },
                _ => 0,
            };
            let page: Vec<Value> = pr
                .comments
                .iter()
                .skip(start)
                .take(100)
                .map(Self::comment_node)
                .collect();
            let more = start + 100 < pr.comments.len();
            return Ok(graph(json!({"comments": {"totalCount": pr.comments.len(),
                "pageInfo": {"hasNextPage": more, "endCursor": more.then(|| format!("c{}", start + 100))},
                "nodes": page}})));
        }
        if query.contains("reviewThreads") {
            return Ok(graph(
                json!({"reviewThreads": {"totalCount": pr.threads.len(),
                "pageInfo": {"hasNextPage": false, "endCursor": null}, "nodes": pr.threads}}),
            ));
        }
        if query.contains("statusCheckRollup") {
            let rollup = pr.checks.as_ref().map(|nodes| {
                json!({"contexts": {"totalCount": nodes.len(),
                "pageInfo": {"hasNextPage": false, "endCursor": null}, "nodes": nodes}})
            });
            return Ok(graph(
                json!({"commits": {"nodes": [{"commit": {"oid": pr.head,
                "statusCheckRollup": rollup}}]}}),
            ));
        }
        Err(format!("a query the fake does not answer: {query}"))
    }

    /// The answer to a REST call, `Ok(None)` for an empty one, or the
    /// status and message GitHub refuses it with.
    fn rest(
        &mut self,
        method: Method,
        route: &str,
        body: Option<&PyValue>,
    ) -> Result<Option<Value>, (u16, String)> {
        let not_found = || (404, "Not Found".to_owned());
        let path = route.split('?').next().unwrap_or("");
        let parts: Vec<&str> = path.split('/').collect();
        let number = |at: usize| -> Result<i64, (u16, String)> {
            parts
                .get(at)
                .and_then(|n| n.parse().ok())
                .ok_or_else(not_found)
        };
        let text_of = |key: &str| -> Result<String, (u16, String)> {
            match body {
                Some(PyValue::Dict(fields)) => match fields.get(key) {
                    Some(PyValue::Str(s)) => Ok(s.clone()),
                    _ => Err((422, format!("{key} is missing"))),
                },
                _ => Err((422, "no body".into())),
            }
        };
        let now = self.state.now.clone();
        match (method, parts.as_slice()) {
            (Method::Get, ["pulls"]) => {
                let open: Vec<Value> = self
                    .state
                    .prs
                    .values()
                    .filter(|pr| pr.state == "open")
                    .map(Self::pull_json)
                    .collect();
                Ok(Some(json!([open])))
            }
            (Method::Get, ["pulls", _]) => {
                let pr = self.state.prs.get(&number(1)?).ok_or_else(not_found)?;
                Ok(Some(Self::pull_json(pr)))
            }
            (Method::Get, ["pulls", _, "files"]) => {
                let pr = self.state.prs.get(&number(1)?).ok_or_else(not_found)?;
                Ok(Some(json!([pr.files])))
            }
            (Method::Get, ["pulls", _, "reviews"]) => {
                let pr = self.state.prs.get(&number(1)?).ok_or_else(not_found)?;
                Ok(Some(json!([pr.reviews])))
            }
            (Method::Get, ["issues", _, "timeline"]) => {
                let pr = self.state.prs.get(&number(1)?).ok_or_else(not_found)?;
                let events: Vec<Value> = pr
                    .timeline
                    .iter()
                    .map(|(event, at)| json!({"event": event, "created_at": at}))
                    .collect();
                Ok(Some(json!([events])))
            }
            (Method::Get, ["issues", _, "comments"]) => {
                let pr = self.state.prs.get(&number(1)?).ok_or_else(not_found)?;
                let comments: Vec<Value> = pr.comments.iter().map(Self::comment_rest).collect();
                Ok(Some(json!([comments])))
            }
            (Method::Get, ["commits", sha, "statuses"]) => Ok(Some(json!([self
                .state
                .statuses
                .get(*sha)
                .cloned()
                .unwrap_or_default()]))),
            (Method::Get, ["collaborators"]) => {
                let listed: Vec<Value> = self
                    .state
                    .collaborators
                    .iter()
                    .map(
                        |(login, level)| json!({"login": login, "permissions": level_flags(level)}),
                    )
                    .collect();
                Ok(Some(json!([listed])))
            }
            (Method::Get, ["collaborators", login, "permission"]) => {
                let level = self
                    .state
                    .collaborators
                    .iter()
                    .find(|(known, _)| py_lower(known) == py_lower(login))
                    .map(|(_, level)| level.clone())
                    .unwrap_or_else(|| "read".into());
                let legacy = if level == "maintain" {
                    "write".into()
                } else {
                    level.clone()
                };
                Ok(Some(json!({"permission": legacy,
                    "user": {"login": login, "permissions": level_flags(&level)}})))
            }
            (Method::Get, ["labels", name]) => {
                if self.state.labels.iter().any(|label| label == name) {
                    Ok(Some(json!({"name": name})))
                } else {
                    Err(not_found())
                }
            }
            (Method::Post, ["statuses", sha]) => {
                let id = self.state.id();
                let field = |key: &str| match body {
                    Some(PyValue::Dict(fields)) => {
                        fields.get(key).map(dump_value).unwrap_or(Value::Null)
                    }
                    _ => Value::Null,
                };
                let status = json!({"id": id, "state": field("state"), "context": field("context"),
                    "description": field("description"), "target_url": field("target_url"),
                    "creator": {"login": ENGINE, "type": "Bot"}, "created_at": now, "updated_at": now});
                self.state
                    .statuses
                    .entry((*sha).into())
                    .or_default()
                    .insert(0, status.clone());
                Ok(Some(status))
            }
            (Method::Post, ["issues", _, "comments"]) => {
                let text = text_of("body")?;
                let id = self.state.id();
                let comment = Comment::new(id, ENGINE, &text, &now);
                let answer = Self::comment_rest(&comment);
                self.state
                    .prs
                    .get_mut(&number(1)?)
                    .ok_or_else(not_found)?
                    .comments
                    .push(comment);
                Ok(Some(answer))
            }
            (Method::Get, ["issues", "comments", id]) => {
                // One comment by its id, wherever it stands; GitHub answers
                // 404 for one that was deleted.
                let id: i64 = id.parse().map_err(|_| not_found())?;
                self.state
                    .prs
                    .values()
                    .flat_map(|pr| &pr.comments)
                    .find(|comment| comment.id == id)
                    .map(|comment| Some(Self::comment_rest(comment)))
                    .ok_or_else(|| (404, "Not Found".to_owned()))
            }
            (Method::Patch, ["issues", "comments", id]) => {
                let text = text_of("body")?;
                let id: i64 = id.parse().map_err(|_| not_found())?;
                let comment = self
                    .state
                    .prs
                    .values_mut()
                    .flat_map(|pr| pr.comments.iter_mut())
                    .find(|comment| comment.id == id)
                    .ok_or_else(not_found)?;
                // GitHub counts an edit that changes nothing as no edit.
                if comment.body != text {
                    comment.body = text;
                    comment.updated_at = now.clone();
                    comment.last_edited_at = Some(now.clone());
                    comment.editor = Some(ENGINE.into());
                }
                Ok(Some(Self::comment_rest(comment)))
            }
            (Method::Delete, ["issues", "comments", id]) => {
                let id: i64 = id.parse().map_err(|_| not_found())?;
                let held = self
                    .state
                    .prs
                    .values_mut()
                    .find(|pr| pr.comments.iter().any(|comment| comment.id == id))
                    .ok_or_else(not_found)?;
                held.comments.retain(|comment| comment.id != id);
                Ok(None)
            }
            (Method::Post, ["issues", _, "labels"]) => {
                let names: Vec<String> = match body {
                    Some(PyValue::Dict(fields)) => match fields.get("labels") {
                        Some(PyValue::List(names)) => {
                            names.iter().map(|n| text(n).to_owned()).collect()
                        }
                        _ => return Err((422, "labels is missing".into())),
                    },
                    _ => return Err((422, "no body".into())),
                };
                let n = number(1)?;
                // A label the repository does not have is created by the
                // write, in a default colour.
                for name in &names {
                    if !self.state.labels.contains(name) {
                        self.state.labels.push(name.clone());
                    }
                }
                let pr = self.state.prs.get_mut(&n).ok_or_else(not_found)?;
                for name in names {
                    if !pr.labels.contains(&name) {
                        pr.labels.push(name);
                    }
                }
                Ok(Some(json!(pr
                    .labels
                    .iter()
                    .map(|n| json!({"name": n}))
                    .collect::<Vec<_>>())))
            }
            (Method::Delete, ["issues", _, "labels", name]) => {
                let pr = self.state.prs.get_mut(&number(1)?).ok_or_else(not_found)?;
                if !pr.labels.iter().any(|label| label == name) {
                    return Err((404, "Label does not exist".into()));
                }
                pr.labels.retain(|label| label != name);
                Ok(Some(json!(pr
                    .labels
                    .iter()
                    .map(|n| json!({"name": n}))
                    .collect::<Vec<_>>())))
            }
            (Method::Patch, ["pulls", _]) => {
                let text = text_of("body")?;
                let pr = self.state.prs.get_mut(&number(1)?).ok_or_else(not_found)?;
                pr.body = text;
                Ok(Some(Self::pull_json(pr)))
            }
            (Method::Post, ["pulls", _, "requested_reviewers"]) => {
                let users: Vec<String> = match body {
                    Some(PyValue::Dict(fields)) => match fields.get("reviewers") {
                        Some(PyValue::List(users)) => {
                            users.iter().map(|u| text(u).to_owned()).collect()
                        }
                        _ => return Err((422, "reviewers is missing".into())),
                    },
                    _ => return Err((422, "no body".into())),
                };
                let pr = self.state.prs.get_mut(&number(1)?).ok_or_else(not_found)?;
                if users
                    .iter()
                    .any(|user| py_lower(user) == py_lower(&pr.author))
                {
                    return Err((
                        422,
                        "Review cannot be requested from pull request author.".into(),
                    ));
                }
                for user in users {
                    if !pr.requested_reviewers.contains(&user) {
                        pr.requested_reviewers.push(user);
                    }
                }
                Ok(Some(Self::pull_json(pr)))
            }
            _ => Err((
                404,
                format!("a route the fake does not answer: {method} {route}"),
            )),
        }
    }
}

/// A value the engine sent, as JSON.
fn dump_value(value: &PyValue) -> Value {
    serde_json::from_str(&dump(value)).expect("the engine writes JSON")
}

/// `gh`'s report of a refused call.
fn failed(status: u16, message: &str) -> TransportError {
    let stderr = format!("gh: {message} (HTTP {status})");
    TransportError::Failed(Failure {
        transient: transient(&stderr, ""),
        status: Some(1),
        body: String::new(),
        detail: stderr,
        class: FailureClass::Http {
            code: status,
            rate_limited: false,
        },
    })
}

impl Transport for Fake {
    fn call(&mut self, call: &Call) -> Result<Reply, TransportError> {
        self.calls.push(call.clone());
        let mut hooks = std::mem::take(&mut self.hooks);
        for hook in &mut hooks {
            hook(&mut self.state, call);
        }
        self.hooks = hooks;
        let prefix = format!("repos/{REPO}/");
        let (method, route, body) = match call {
            Call::Rest {
                method, path, body, ..
            } => {
                let Some(route) = path.strip_prefix(&prefix) else {
                    return Err(TransportError::Refused(format!(
                        "outside the repository: {path}"
                    )));
                };
                (*method, route.to_owned(), body.clone())
            }
            Call::Graphql { .. } => (Method::Post, "graphql".to_owned(), None),
        };
        let write = !matches!(method, Method::Get) && route != "graphql";
        if let Some(at) = self
            .rules
            .iter()
            .position(|rule| rule.method == method && route.starts_with(&rule.route))
        {
            let rule = &mut self.rules[at];
            let refusal = rule.refusal.clone();
            if let Some(times) = &mut rule.times {
                *times -= 1;
                if *times == 0 {
                    self.rules.remove(at);
                }
            }
            if write {
                self.written.push(Written {
                    method,
                    route: route.clone(),
                    body: body.clone(),
                });
            }
            return match refusal {
                Refusal::Http(status, message) => Err(failed(status, &message)),
                Refusal::Empty => Ok(Reply::Text(String::new())),
            };
        }
        let answer = match call {
            Call::Graphql { query, variables } => self
                .graphql(query, variables)
                .map(Some)
                .map_err(|why| (400, why)),
            Call::Rest { .. } => {
                if write {
                    self.written.push(Written {
                        method,
                        route: route.clone(),
                        body: body.clone(),
                    });
                }
                self.rest(method, &route, body.as_ref())
            }
        };
        match answer {
            Ok(Some(value)) if self.prints_as_gh => Ok(Reply::Text(gh_printed(&value.to_string()))),
            Ok(Some(value)) => Ok(Reply::Text(value.to_string())),
            Ok(None) => Ok(Reply::Text(String::new())),
            Err((status, message)) => Err(failed(status, &message)),
        }
    }
}
