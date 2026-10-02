//! What a snapshot must look like to be stored: the analyzer's schema, with
//! every name in a known format and every text bounded; and, when it
//! arrives with a token, bound to the run that minted the token.

use crate::oidc::{Verified, CLOCK_SKEW};
use chrono::TimeDelta;
use pr_hygiene::dashboard::{Dashboard, PersonOut, PrOut, SCHEMA_VERSION};
use std::collections::HashSet;
use std::time::Duration;

const MAX_REPOS: usize = 64;
const MAX_PRS: usize = 10_000;
const MAX_PEOPLE: usize = 10_000;
/// Per-PR and per-person lists: approvers, blockers, areas, owed reviews.
const MAX_LIST: usize = 1_000;
/// GitHub caps a title at 256 characters.
const MAX_TITLE: usize = 1_024;
/// The engine's blockers and next actions.
const MAX_TEXT: usize = 4_096;
/// Diagnostics (`fetch_error`, `stage_times_error`) carry whatever an HTTP
/// error body said; they are cut to this rather than refused, so a long
/// error page cannot cost the whole snapshot.
const MAX_DIAGNOSTIC: usize = 500;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Invalid(pub String);

fn invalid<T>(why: impl Into<String>) -> Result<T, Invalid> {
    Err(Invalid(why.into()))
}

/// Read and check a snapshot. Diagnostics are shortened; nothing else is
/// changed.
pub fn parse(body: &[u8]) -> Result<Dashboard, Invalid> {
    if body.is_empty() {
        return invalid("empty body");
    }
    let mut d: Dashboard =
        serde_json::from_slice(body).map_err(|e| Invalid(format!("not a snapshot: {e}")))?;
    shorten_diagnostics(&mut d);
    validate(&d)?;
    Ok(d)
}

fn shorten_diagnostics(d: &mut Dashboard) {
    for r in &mut d.repos {
        for text in [&mut r.fetch_error, &mut r.stage_times_error]
            .into_iter()
            .flatten()
        {
            if text.chars().count() > MAX_DIAGNOSTIC {
                *text = text.chars().take(MAX_DIAGNOSTIC).collect::<String>() + "…";
            }
        }
    }
}

/// The token's run produced this snapshot: the same commit, generated
/// during that run — no earlier than the job could have started, no later
/// than the token was minted. A captured snapshot replayed with a later
/// token fails the first; a future-dated one, which would make every honest
/// snapshot after it "not newer", fails the second.
pub fn bind(d: &Dashboard, token: &Verified, job_timeout: Duration) -> Result<(), Invalid> {
    match &d.commit {
        Some(commit) if commit.eq_ignore_ascii_case(&token.sha) => {}
        Some(_) => return invalid("snapshot commit is not the commit of the posting run"),
        None => return invalid("snapshot names no commit"),
    }
    let delta = |d: Duration| TimeDelta::from_std(d).unwrap_or(TimeDelta::MAX);
    if d.generated_at < token.issued_at - delta(job_timeout) {
        return invalid("snapshot was generated before the posting run's job could have started");
    }
    if d.generated_at > token.issued_at + delta(CLOCK_SKEW) {
        return invalid("snapshot is dated after its token was minted");
    }
    Ok(())
}

pub fn validate(d: &Dashboard) -> Result<(), Invalid> {
    if d.schema_version != SCHEMA_VERSION {
        return invalid(format!(
            "schema_version {} is not {SCHEMA_VERSION}",
            d.schema_version
        ));
    }
    if let Some(commit) = &d.commit {
        // A full id when posted (it must equal the token's); an abbreviated
        // one is accepted from a local import.
        if !(7..=64).contains(&commit.len()) || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
            return invalid("commit is not a commit id");
        }
    }
    if !(0..=3_650).contains(&d.idle_days) {
        return invalid("idle_days out of range");
    }
    if d.repos.is_empty() || d.repos.len() > MAX_REPOS {
        return invalid("repos: expected 1 to 64");
    }
    let mut repos = HashSet::new();
    for r in &d.repos {
        if !is_repo(&r.repo) {
            return invalid(format!("repos: {:?} is not owner/name", r.repo));
        }
        if !repos.insert(r.repo.to_ascii_lowercase()) {
            return invalid(format!("repos: {} listed twice", r.repo));
        }
    }
    let known = |repo: &str| repos.contains(&repo.to_ascii_lowercase());
    bounded("prs", d.prs.len(), MAX_PRS)?;
    let mut keys = HashSet::new();
    for p in &d.prs {
        pr(p, &known)?;
        if !keys.insert(p.key.clone()) {
            return invalid(format!("prs: {} listed twice", p.key));
        }
    }
    bounded("people", d.people.len(), MAX_PEOPLE)?;
    let mut logins = HashSet::new();
    for p in &d.people {
        person(p, &known)?;
        if !logins.insert(p.login.to_ascii_lowercase()) {
            return invalid(format!("people: {} listed twice", p.login));
        }
    }
    Ok(())
}

fn pr(p: &PrOut, known: &impl Fn(&str) -> bool) -> Result<(), Invalid> {
    let at = || format!("prs[{}]", p.key);
    if !known(&p.repo) || p.key != format!("{}#{}", p.repo, p.number) {
        return invalid(format!("{}: key or repository not in repos", at()));
    }
    if p.number == 0 || i64::try_from(p.number).is_err() {
        return invalid(format!("{}: number out of range", at()));
    }
    text(&at(), "title", &p.title, MAX_TITLE)?;
    if let Some(author) = &p.author {
        login(&at(), author)?;
    }
    if let Some(base) = &p.base {
        text(&at(), "base", base, 255)?;
    }
    if let Some(state) = &p.engine_state {
        if state.is_empty()
            || state.len() > 64
            || !state.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
        {
            return invalid(format!("{}: engine_state is not a state", at()));
        }
    }
    if let Some(next) = &p.next_action {
        text(&at(), "next_action", next, MAX_TEXT)?;
    }
    bounded(&at(), p.blockers.len(), MAX_LIST)?;
    for b in &p.blockers {
        text(&at(), "blockers", b, MAX_TEXT)?;
    }
    bounded(&at(), p.asks.len(), MAX_LIST)?;
    for ask in &p.asks {
        area(&at(), &ask.area)?;
        bounded(&at(), ask.approvers.len(), MAX_LIST)?;
        for a in &ask.approvers {
            login(&at(), a)?;
        }
    }
    bounded(&at(), p.objectors.len(), MAX_LIST)?;
    for o in &p.objectors {
        login(&at(), o)?;
    }
    bounded(&at(), p.areas.len(), MAX_LIST)?;
    for a in &p.areas {
        area(&at(), a)?;
    }
    Ok(())
}

fn person(p: &PersonOut, known: &impl Fn(&str) -> bool) -> Result<(), Invalid> {
    login("people", &p.login)?;
    let at = || format!("people[{}]", p.login);
    let pr_key = |key: &str| -> Result<(), Invalid> {
        match key.rsplit_once('#') {
            Some((repo, n)) if known(repo) && n.parse::<u64>().is_ok() => Ok(()),
            _ => invalid(format!(
                "{}: {key:?} is not a PR of a listed repository",
                at()
            )),
        }
    };
    bounded(&at(), p.owes.len(), MAX_LIST)?;
    for owed in &p.owes {
        pr_key(&owed.pr)?;
        bounded(&at(), owed.areas.len(), MAX_LIST)?;
        for part in &owed.areas {
            area(&at(), &part.area)?;
            bounded(&at(), part.others.len(), MAX_LIST)?;
            for other in &part.others {
                login(&at(), other)?;
            }
        }
    }
    bounded(&at(), p.authored.len(), MAX_LIST)?;
    for key in &p.authored {
        pr_key(key)?;
    }
    for repo in p.wip.keys() {
        if !known(repo) {
            return invalid(format!("{}: wip names an unlisted repository", at()));
        }
    }
    for (repo, areas) in &p.areas {
        if !known(repo) {
            return invalid(format!("{}: areas name an unlisted repository", at()));
        }
        bounded(&at(), areas.len(), MAX_LIST)?;
        for a in areas {
            area(&at(), a)?;
        }
    }
    Ok(())
}

fn bounded(at: &str, len: usize, max: usize) -> Result<(), Invalid> {
    if len > max {
        return invalid(format!("{at}: more than {max} entries"));
    }
    Ok(())
}

fn text(at: &str, field: &str, value: &str, max: usize) -> Result<(), Invalid> {
    if value.chars().count() > max {
        return invalid(format!("{at}: {field} longer than {max} characters"));
    }
    Ok(())
}

/// A GitHub login, or a GitHub App's `name[bot]`.
pub fn is_login(s: &str) -> bool {
    let name = s.strip_suffix("[bot]").unwrap_or(s);
    (1..=39).contains(&name.len()) && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

fn login(at: &str, s: &str) -> Result<(), Invalid> {
    if !is_login(s) {
        return invalid(format!("{at}: {s:?} is not a GitHub login"));
    }
    Ok(())
}

/// A policy's area id (`[a-z0-9][a-z0-9-]*`), or `fallback`.
fn area(at: &str, s: &str) -> Result<(), Invalid> {
    let ok = (1..=100).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-');
    if !ok {
        return invalid(format!("{at}: {s:?} is not an area id"));
    }
    Ok(())
}

/// `owner/name` as GitHub spells repositories.
pub fn is_repo(s: &str) -> bool {
    let Some((owner, name)) = s.split_once('/') else {
        return false;
    };
    (1..=39).contains(&owner.len())
        && owner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        && (1..=100).contains(&name.len())
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        && name != "."
        && name != ".."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logins_repos_and_areas_have_github_shapes() {
        assert!(is_login("QuantumExplorer"));
        assert!(is_login("dependabot[bot]"));
        assert!(!is_login(""));
        assert!(!is_login("a b"));
        assert!(!is_login("<!channel>"));
        assert!(!is_login(&"x".repeat(40)));
        assert!(is_repo("dashpay/platform"));
        assert!(is_repo("dashpay/rust-dashcore"));
        assert!(!is_repo("dashpay"));
        assert!(!is_repo("dashpay/.."));
        assert!(!is_repo("dash pay/platform"));
        assert!(area("t", "swift-sdk").is_ok());
        assert!(area("t", "fallback").is_ok());
        assert!(area("t", "Swift").is_err());
        assert!(area("t", "-x").is_err());
    }

    #[test]
    fn a_long_error_page_is_cut_not_refused() {
        let mut d: Dashboard = serde_json::from_value(serde_json::json!({
            "schema_version": 1, "generated_at": "2026-10-01T12:00:00Z", "commit": null,
            "repos": [{"repo": "dashpay/platform", "engine_state_available": true,
                       "fetch_error": "x".repeat(10_000), "stage_times_error": null, "slot_limit": 5}],
            "idle_days": 14, "stages": [], "prs": [], "people": []
        }))
        .unwrap();
        shorten_diagnostics(&mut d);
        let error = d.repos[0].fetch_error.as_deref().unwrap();
        assert_eq!(error.chars().count(), MAX_DIAGNOSTIC + 1);
        assert!(validate(&d).is_ok());
    }
}
