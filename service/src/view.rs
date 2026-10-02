//! What the API serves: every repository's latest good data, assembled into
//! one view when a snapshot is stored.
//!
//! A run that could not read a repository must not blank it. Each
//! repository's part of a snapshot — its PRs, and each person's part in it —
//! is kept as the last good data for that repository, and the view is put
//! together from those parts.
//!
//! People are recomputed by merging their per-repository parts rather than
//! by re-deriving them from the PRs. Every part of a person is keyed by
//! repository in the analyzer's output (an owed review and an authored PR by
//! their `owner/name#n` key; `wip` and `areas` by repository), so a part is
//! a filter and a merge is a union. Nothing the snapshot lacks is needed — no
//! engine reviewer list, no policies — and nothing of the engine's "your part"
//! rule is reimplemented here: when every repository is read, the merged
//! people equal the analyzer's own. A failed repository's owed reviews,
//! authored PRs, open-slot counts and areas all come from the same snapshot
//! as its PRs, so they agree with each other.

use chrono::{DateTime, Utc};
use pr_hygiene::dashboard::{Dashboard, Kind, PersonOut, PrOut, RepoOut, Role, StageOut};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};

/// A repository counts as read only when both its PRs and the engine's
/// verdicts were: without either, its stages are unknown.
pub fn is_good(r: &RepoOut) -> bool {
    r.fetch_error.is_none() && r.engine_state_available
}

/// One repository's part of a snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoData {
    pub status: RepoOut,
    pub prs: Vec<PrOut>,
    /// Each person's part in this repository only.
    pub people: Vec<PersonOut>,
}

/// The repository named by a PR key, `owner/name#n`.
pub fn repo_of(key: &str) -> &str {
    key.rsplit_once('#').map_or(key, |(repo, _)| repo)
}

pub fn repo_part(d: &Dashboard, status: &RepoOut) -> RepoData {
    let repo = status.repo.as_str();
    RepoData {
        status: status.clone(),
        prs: d.prs.iter().filter(|p| p.repo == repo).cloned().collect(),
        people: d
            .people
            .iter()
            .filter_map(|p| person_part(p, repo))
            .collect(),
    }
}

fn person_part(p: &PersonOut, repo: &str) -> Option<PersonOut> {
    let part = PersonOut {
        login: p.login.clone(),
        kind: p.kind,
        roles: p.roles.clone(),
        owes: p
            .owes
            .iter()
            .filter(|o| repo_of(&o.pr) == repo)
            .cloned()
            .collect(),
        authored: p
            .authored
            .iter()
            .filter(|k| repo_of(k) == repo)
            .cloned()
            .collect(),
        wip: p
            .wip
            .get(repo)
            .map(|n| (repo.to_string(), *n))
            .into_iter()
            .collect(),
        areas: p
            .areas
            .get(repo)
            .map(|a| (repo.to_string(), a.clone()))
            .into_iter()
            .collect(),
    };
    let empty = part.owes.is_empty()
        && part.authored.is_empty()
        && part.wip.is_empty()
        && part.areas.is_empty();
    (!empty).then_some(part)
}

/// People over the kept parts. Who is a bot, and how a login is spelled,
/// come from the latest snapshot when it knows the person: it reflects the
/// current policies. `author-bot` follows from the merged authored PRs.
pub fn merge_people(
    latest: &[PersonOut],
    parts: impl IntoIterator<Item = PersonOut>,
) -> Vec<PersonOut> {
    let mut by_login: BTreeMap<String, PersonOut> = BTreeMap::new();
    for part in parts {
        match by_login.entry(part.login.to_ascii_lowercase()) {
            Entry::Vacant(e) => {
                e.insert(part);
            }
            Entry::Occupied(mut e) => {
                let p = e.get_mut();
                p.owes.extend(part.owes);
                p.authored.extend(part.authored);
                p.wip.extend(part.wip);
                p.areas.extend(part.areas);
                for role in part.roles {
                    if !p.roles.contains(&role) {
                        p.roles.push(role);
                    }
                }
            }
        }
    }
    let latest: HashMap<String, &PersonOut> = latest
        .iter()
        .map(|p| (p.login.to_ascii_lowercase(), p))
        .collect();
    let mut out: Vec<PersonOut> = by_login
        .into_iter()
        .map(|(key, mut p)| {
            if let Some(now) = latest.get(&key) {
                p.login.clone_from(&now.login);
                p.kind = now.kind;
            }
            // A review bot is one by its login, which every part shares.
            let review_bot = p.roles.contains(&Role::ReviewBot);
            p.roles.clear();
            if p.kind == Kind::Bot {
                if !p.authored.is_empty() {
                    p.roles.push(Role::AuthorBot);
                }
                if review_bot {
                    p.roles.push(Role::ReviewBot);
                }
            }
            p.owes.sort_by(|a, b| a.pr.cmp(&b.pr));
            p.authored.sort();
            p
        })
        .collect();
    out.sort_by_key(|p| p.login.to_ascii_lowercase());
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// The verdicts shown are the engine's, as it posted them on each PR;
    /// this service writes nothing to GitHub.
    Shadow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoView {
    pub repo: String,
    pub mode: Mode,
    /// The latest run could not read this repository: what is shown is its
    /// last good data, from `data_as_of`.
    pub stale: bool,
    /// When the data shown was generated; `null` when it never was.
    pub data_as_of: Option<DateTime<Utc>>,
    /// When the latest run, good or not, was generated.
    pub checked_at: DateTime<Utc>,
    /// Why the latest run could not read the PRs.
    pub fetch_error: Option<String>,
    /// Whether the latest run had the engine's verdicts.
    pub engine_state_available: bool,
    /// Why some stage entry times in the data shown could not be read.
    pub stage_times_error: Option<String>,
    pub slot_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    /// The id of the snapshot this view was assembled at.
    pub version: i64,
    pub generated_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub commit: Option<String>,
    pub idle_days: i64,
    pub stages: Vec<StageOut>,
    pub repos: Vec<RepoView>,
    pub prs: Vec<PrOut>,
    pub people: Vec<PersonOut>,
}

impl View {
    pub fn stale_repos(&self) -> Vec<&str> {
        self.repos
            .iter()
            .filter(|r| r.stale)
            .map(|r| r.repo.as_str())
            .collect()
    }

    pub fn person(&self, login: &str) -> Option<&PersonOut> {
        self.people
            .iter()
            .find(|p| p.login.eq_ignore_ascii_case(login))
    }

    pub fn pr(&self, key: &str) -> Option<&PrOut> {
        self.prs.iter().find(|p| p.key == key)
    }

    pub fn repo(&self, repo: &str) -> Option<&RepoView> {
        self.repos.iter().find(|r| r.repo == repo)
    }
}

/// The last good data kept for one repository.
#[derive(Debug, Clone)]
pub struct Kept {
    pub good_at: DateTime<Utc>,
    pub data: RepoData,
}

/// The view as of snapshot `d`: its repositories, each shown from `kept`.
pub fn assemble(
    d: &Dashboard,
    version: i64,
    received_at: DateTime<Utc>,
    kept: &HashMap<String, Kept>,
) -> View {
    let mut prs = vec![];
    let mut parts = vec![];
    let mut repos = vec![];
    for status in &d.repos {
        let shown = kept.get(&status.repo);
        repos.push(RepoView {
            repo: status.repo.clone(),
            mode: Mode::Shadow,
            stale: !is_good(status),
            data_as_of: shown.map(|k| k.good_at),
            checked_at: d.generated_at,
            fetch_error: status.fetch_error.clone(),
            engine_state_available: status.engine_state_available,
            stage_times_error: shown.and_then(|k| k.data.status.stage_times_error.clone()),
            slot_limit: status.slot_limit,
        });
        if let Some(k) = shown {
            prs.extend(k.data.prs.iter().cloned());
            parts.extend(k.data.people.iter().cloned());
        }
    }
    prs.sort_by(|a, b| a.repo.cmp(&b.repo).then(a.number.cmp(&b.number)));
    View {
        version,
        generated_at: d.generated_at,
        received_at,
        commit: d.commit.clone(),
        idle_days: d.idle_days,
        stages: d.stages.clone(),
        repos,
        prs,
        people: merge_people(&d.people, parts),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::fixture;

    fn all_good(mut d: Dashboard) -> Dashboard {
        for r in &mut d.repos {
            r.engine_state_available = true;
            r.fetch_error = None;
        }
        d
    }

    fn kept_from(d: &Dashboard) -> HashMap<String, Kept> {
        d.repos
            .iter()
            .filter(|r| is_good(r))
            .map(|r| {
                (
                    r.repo.clone(),
                    Kept {
                        good_at: d.generated_at,
                        data: repo_part(d, r),
                    },
                )
            })
            .collect()
    }

    /// The merge must not be a second opinion: with every repository read,
    /// the people and PRs served are exactly the analyzer's.
    #[test]
    fn with_every_repository_read_the_view_is_the_analyzers_own() {
        let d = all_good(fixture());
        let view = assemble(&d, 1, d.generated_at, &kept_from(&d));
        assert_eq!(view.people, d.people);
        assert_eq!(view.prs, d.prs);
        assert!(view.stale_repos().is_empty());
    }

    #[test]
    fn a_persons_part_is_only_what_lives_in_that_repository() {
        let d = all_good(fixture());
        let someone = d
            .people
            .iter()
            .find(|p| p.areas.len() > 1 || p.authored.len() > 1)
            .expect("the fixture has someone in more than one place");
        for r in &d.repos {
            if let Some(part) = person_part(someone, &r.repo) {
                assert!(part.owes.iter().all(|o| repo_of(&o.pr) == r.repo));
                assert!(part.authored.iter().all(|k| repo_of(k) == r.repo));
                assert!(part
                    .wip
                    .keys()
                    .chain(part.areas.keys())
                    .all(|k| *k == r.repo));
            }
        }
    }

    #[test]
    fn a_bot_is_an_author_bot_only_while_it_holds_a_pr() {
        let bot = |authored: Vec<&str>| PersonOut {
            login: "thepastaclaw".into(),
            kind: Kind::Bot,
            roles: vec![Role::AuthorBot, Role::ReviewBot],
            owes: vec![],
            authored: authored.into_iter().map(String::from).collect(),
            wip: BTreeMap::new(),
            areas: BTreeMap::new(),
        };
        let merged = merge_people(&[], vec![bot(vec![])]);
        assert_eq!(merged[0].roles, vec![Role::ReviewBot]);
        let merged = merge_people(&[], vec![bot(vec!["dashpay/platform#1"])]);
        assert_eq!(merged[0].roles, vec![Role::AuthorBot, Role::ReviewBot]);
    }
}
