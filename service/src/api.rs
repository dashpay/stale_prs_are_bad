//! The public read API, `/api/v1`. Everything is answered from the latest
//! view in the store: no request reaches GitHub, and an unknown login or PR
//! is a 404, never a lookup.
//!
//! Each response carries an ETag naming the snapshot (and this service's
//! version) it was built from; the same snapshot always gives the same
//! bytes, so a matching `If-None-Match` is answered 304.

use crate::app::{ApiError, AppState};
use crate::digest;
use crate::snapshot::{is_login, is_repo};
use crate::view::View;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use pr_hygiene::dashboard::{Lateness, PrOut, Stage};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;

/// Headers on every public response: readable from any origin, never a
/// cookie, cacheable when it succeeded.
pub async fn public_headers(mut res: Response) -> Response {
    let cacheable = res.status().is_success() || res.status() == StatusCode::NOT_MODIFIED;
    let h = res.headers_mut();
    h.remove(header::SET_COOKIE);
    h.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    h.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("ETag"),
    );
    h.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    h.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static(if cacheable {
            "public, max-age=60"
        } else {
            "no-store"
        }),
    );
    res
}

fn etag(view: &View) -> String {
    format!("\"{}-{}\"", view.version, env!("CARGO_PKG_VERSION"))
}

/// 304 when the client already holds this snapshot's response.
fn not_modified(headers: &HeaderMap, view: &View) -> Option<Response> {
    let tag = etag(view);
    let matches = headers
        .get_all(header::IF_NONE_MATCH)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .map(str::trim)
        .any(|t| t == "*" || t == tag || t.strip_prefix("W/") == Some(tag.as_str()));
    matches.then(|| with_etag(StatusCode::NOT_MODIFIED.into_response(), &tag))
}

fn with_etag(mut res: Response, tag: &str) -> Response {
    if let Ok(value) = HeaderValue::from_str(tag) {
        res.headers_mut().insert(header::ETAG, value);
    }
    res
}

fn json_response(view: &View, body: serde_json::Value) -> Response {
    with_etag(axum::Json(body).into_response(), &etag(view))
}

fn text_response(status: StatusCode, body: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
        .into_response()
}

fn query<T>(q: Result<Query<T>, QueryRejection>) -> Result<T, ApiError> {
    q.map(|Query(q)| q)
        .map_err(|e| ApiError::bad_request(e.body_text()))
}

/// For endpoints that take no parameters: any parameter is a mistake.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoParams {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrsQuery {
    repo: Option<String>,
    area: Option<String>,
    stage: Option<Stage>,
    author: Option<String>,
    reviewer: Option<String>,
    late: Option<bool>,
}

impl PrsQuery {
    fn check(&self) -> Result<(), ApiError> {
        if self.repo.as_deref().is_some_and(|r| !is_repo(r)) {
            return Err(ApiError::bad_request("repo: expected owner/name"));
        }
        for (name, login) in [("author", &self.author), ("reviewer", &self.reviewer)] {
            if login.as_deref().is_some_and(|l| !is_login(l)) {
                return Err(ApiError::bad_request(format!("{name}: not a GitHub login")));
            }
        }
        Ok(())
    }
}

/// `GET /api/v1/prs`: open PRs, filtered. Every filter given must match.
pub async fn prs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    q: Result<Query<PrsQuery>, QueryRejection>,
) -> Result<Response, ApiError> {
    let q = query(q)?;
    q.check()?;
    let view = state.view().await?;
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    let owed: Option<HashSet<&str>> = q.reviewer.as_deref().map(|login| {
        view.person(login)
            .map(|p| p.owes.iter().map(|o| o.pr.as_str()).collect())
            .unwrap_or_default()
    });
    let eq = |a: &str, b: &Option<String>| b.as_deref().is_none_or(|b| a.eq_ignore_ascii_case(b));
    let prs: Vec<&PrOut> = view
        .prs
        .iter()
        .filter(|p| eq(&p.repo, &q.repo))
        .filter(|p| q.area.as_ref().is_none_or(|a| p.areas.contains(a)))
        .filter(|p| q.stage.is_none_or(|s| p.stage == s))
        .filter(|p| {
            q.author.as_deref().is_none_or(|a| {
                p.author
                    .as_deref()
                    .is_some_and(|x| x.eq_ignore_ascii_case(a))
            })
        })
        .filter(|p| owed.as_ref().is_none_or(|o| o.contains(p.key.as_str())))
        .filter(|p| {
            q.late.is_none_or(|late| {
                late == matches!(p.lateness, Some(Lateness::Late | Lateness::VeryLate))
            })
        })
        .collect();
    Ok(json_response(
        &view,
        json!({
            "generated_at": view.generated_at,
            "stale_repos": view.stale_repos(),
            "prs": prs,
        }),
    ))
}

/// `GET /api/v1/prs/{owner}/{repo}/{number}`: one PR with the stage changes
/// recorded for it.
pub async fn pr(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    path: Result<Path<(String, String, u64)>, PathRejection>,
    q: Result<Query<NoParams>, QueryRejection>,
) -> Result<Response, ApiError> {
    query(q)?;
    let Path((owner, name, number)) =
        path.map_err(|_| ApiError::bad_request("expected /prs/{owner}/{repo}/{number}"))?;
    let repo = format!("{owner}/{name}");
    if !is_repo(&repo) {
        return Err(ApiError::bad_request("repo: expected owner/name"));
    }
    let view = state.view().await?;
    let Some(pr) = view
        .prs
        .iter()
        .find(|p| p.number == number && p.repo.eq_ignore_ascii_case(&repo))
    else {
        return Err(ApiError::not_found(format!(
            "{repo}#{number} is not an open PR here"
        )));
    };
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    let stored_repo = pr.repo.clone();
    let as_of = view.version;
    let changes = state
        .read(move |reader| reader.stage_changes(&stored_repo, number, as_of))
        .await?;
    Ok(json_response(
        &view,
        json!({
            "generated_at": view.generated_at,
            "stale": view.repo(&pr.repo).is_some_and(|r| r.stale),
            "pr": pr,
            "stage_changes": changes,
        }),
    ))
}

/// `GET /api/v1/people`: everyone, by name. What each owes and holds — and
/// nothing that ranks them.
pub async fn people(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    q: Result<Query<NoParams>, QueryRejection>,
) -> Result<Response, ApiError> {
    query(q)?;
    let view = state.view().await?;
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    let mut people: Vec<_> = view.people.iter().collect();
    people.sort_by_key(|p| p.login.to_ascii_lowercase());
    Ok(json_response(
        &view,
        json!({
            "generated_at": view.generated_at,
            "stale_repos": view.stale_repos(),
            "people": people,
        }),
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Format {
    Json,
    Text,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonQuery {
    format: Option<Format>,
}

/// `GET /api/v1/people/{login}`: what one person owes, oldest first, and
/// what they hold; `?format=text` is the digest.
pub async fn person(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(login): Path<String>,
    q: Result<Query<PersonQuery>, QueryRejection>,
) -> Result<Response, ApiError> {
    let format = query(q)?.format.unwrap_or(Format::Json);
    if !is_login(&login) {
        return Err(ApiError::bad_request("not a GitHub login"));
    }
    let view = state.view().await?;
    let Some(person) = view.person(&login) else {
        return Ok(match format {
            Format::Text => text_response(
                StatusCode::NOT_FOUND,
                format!(
                    "PR Hygiene digest v{}: {login} is not in the PR Hygiene data\n",
                    digest::VERSION
                ),
            ),
            Format::Json => ApiError::not_found(format!("{login} is not in the PR Hygiene data"))
                .into_response(),
        });
    };
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    if format == Format::Text {
        let body = digest::person(&view, person);
        return Ok(with_etag(text_response(StatusCode::OK, body), &etag(&view)));
    }
    let owes: Vec<_> = digest::owed_oldest_first(&view, person)
        .into_iter()
        .map(|(o, pr)| json!({ "pr": pr, "areas": o.areas, "rereview": o.rereview }))
        .collect();
    Ok(json_response(
        &view,
        json!({
            "generated_at": view.generated_at,
            "stale_repos": view.stale_repos(),
            "person": person,
            "owes": owes,
            "authored": digest::authored(&view, person),
        }),
    ))
}

/// `GET /api/v1/repos`: each repository's freshness and mode.
pub async fn repos(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    q: Result<Query<NoParams>, QueryRejection>,
) -> Result<Response, ApiError> {
    query(q)?;
    let view = state.view().await?;
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    Ok(json_response(
        &view,
        json!({
            "generated_at": view.generated_at,
            "received_at": view.received_at,
            "commit": view.commit,
            "repos": view.repos,
        }),
    ))
}

/// `GET /api/v1/stages`: every stage in display order, whose move it is and
/// when it is late.
pub async fn stages(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    q: Result<Query<NoParams>, QueryRejection>,
) -> Result<Response, ApiError> {
    query(q)?;
    let view = state.view().await?;
    if let Some(res) = not_modified(&headers, &view) {
        return Ok(res);
    }
    Ok(json_response(
        &view,
        json!({
            "idle_days": view.idle_days,
            "stages": view.stages,
        }),
    ))
}
