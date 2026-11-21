//! The verdicts pushed since a snapshot, shown over it.
//!
//! The snapshot view is kept as stored; what is served is that view with,
//! for each repository, what was pushed after its data was read. A pushed
//! verdict replaces only what the engine's verdict decides — the PR's
//! stage, engine state, draft flag, blockers, asks and objectors, the
//! reviews owed on it, and its author's open-slot count — through the
//! analyzer's own functions. A PR missing from a newer list of open PRs is
//! hidden. Everything else, and every PR nothing newer was pushed for,
//! stays as the snapshot has it. A newer snapshot needs no clean-up: what
//! was pushed before it is simply older.

use crate::view::View;
use chrono::{DateTime, Utc};
use pr_hygiene::dashboard::{self as board, Owed, PersonOut, PrOut, SinceBasis, Stage};
use pr_hygiene::model::PolicyState;
use std::collections::{HashMap, HashSet};

/// The newest verdict pushed for one PR.
#[derive(Debug, Clone)]
pub struct StoredVerdict {
    pub repo: String,
    pub number: u64,
    pub read_at: DateTime<Utc>,
    pub verdict: PolicyState,
    /// When a push first showed the PR in its current stage, unless that
    /// was the PR's first push.
    pub stage_seen_at: Option<DateTime<Utc>>,
}

/// One repository's open PRs as of the newest push.
#[derive(Debug, Clone)]
pub struct OpenList {
    pub repo: String,
    pub read_at: DateTime<Utc>,
    pub numbers: HashSet<u64>,
}

/// `view` with what was pushed after each repository's data was read,
/// with lateness as of `now`.
pub fn overlay(
    view: &View,
    verdicts: &[StoredVerdict],
    open: &[OpenList],
    now: DateTime<Utc>,
) -> View {
    let data_as_of: HashMap<&str, DateTime<Utc>> = view
        .repos
        .iter()
        .filter_map(|r| Some((r.repo.as_str(), r.data_as_of?)))
        .collect();
    let newer = |repo: &str, read_at: DateTime<Utc>| {
        data_as_of.get(repo).is_some_and(|as_of| read_at > *as_of)
    };
    let open: HashMap<&str, &HashSet<u64>> = open
        .iter()
        .filter(|o| newer(&o.repo, o.read_at))
        .map(|o| (o.repo.as_str(), &o.numbers))
        .collect();
    // By the PR's key, `owner/name#number`.
    let verdicts: HashMap<String, &StoredVerdict> = verdicts
        .iter()
        .filter(|v| newer(&v.repo, v.read_at))
        .map(|v| (format!("{}#{}", v.repo, v.number), v))
        .collect();
    let mut out = view.clone();
    if open.is_empty() && verdicts.is_empty() {
        return out;
    }

    let mut hidden = vec![];
    let mut changed = vec![];
    out.prs.retain_mut(|p| {
        if open
            .get(p.repo.as_str())
            .is_some_and(|numbers| !numbers.contains(&p.number))
        {
            hidden.push(p.clone());
            return false;
        }
        let pushed = verdicts.get(&p.key);
        if let (Some(v), Some(as_of)) = (pushed, data_as_of.get(p.repo.as_str())) {
            let was = p.stage;
            apply(p, v, *as_of, &view.stages, now);
            changed.push((was, p.clone(), *v));
        }
        true
    });

    for p in &hidden {
        forget_owed(&mut out.people, &p.key);
        if let Some(author) = person(&mut out.people, p.author.as_deref()) {
            author.authored.retain(|k| *k != p.key);
            if holds_slot(p.stage) {
                take_slot(author, &p.repo);
            }
        }
    }
    for (was, p, v) in changed {
        forget_owed(&mut out.people, &p.key);
        for reviewer in &v.verdict.reviewers {
            let Some((areas, rereview)) = board::part(&v.verdict, reviewer) else {
                continue;
            };
            if let Some(person) = person(&mut out.people, Some(reviewer)) {
                person.owes.push(Owed {
                    pr: p.key.clone(),
                    areas,
                    rereview,
                });
                person.owes.sort_by(|a, b| a.pr.cmp(&b.pr));
            }
        }
        if holds_slot(was) != holds_slot(p.stage) {
            if let Some(author) = person(&mut out.people, p.author.as_deref()) {
                if holds_slot(p.stage) {
                    *author.wip.entry(p.repo.clone()).or_default() += 1;
                } else {
                    take_slot(author, &p.repo);
                }
            }
        }
    }
    out
}

/// Replace what the engine's verdict decides on `p`, whose repository's
/// data was read at `data_as_of`.
fn apply(
    p: &mut PrOut,
    v: &StoredVerdict,
    data_as_of: DateTime<Utc>,
    stages: &[board::StageOut],
    now: DateTime<Utc>,
) {
    let state = &v.verdict;
    // Not the snapshot's draft flag: the verdict is newer, and a draft
    // marked ready since must leave Draft. The engine only rules on PRs to
    // governed branches.
    let stage = board::stage(false, true, true, Some(state));
    // Entered its stage after the snapshot: at the push that first showed
    // it there. Still in the snapshot's stage: when the snapshot says. Moved
    // since, but first seen there by this PR's first push: at that push.
    let since = match v.stage_seen_at.filter(|at| *at > data_as_of) {
        Some(at) => Some(at),
        None if stage == p.stage => None,
        None => Some(v.read_at),
    };
    let out = stages.iter().find(|s| s.stage == stage);
    if let Some(since) = since {
        if out.is_some_and(|s| s.entry_recorded) {
            p.since = Some(since);
            p.since_basis = Some(SinceBasis::Engine);
            p.lateness =
                board::lateness_at(p.since, p.since_basis, out.and_then(|s| s.late_hours), now);
        } else {
            // A stage whose entry is never recorded, such as a running
            // build: dated from the PR's opening and never late, as the
            // analyzer has it.
            p.since = p.created_at;
            p.since_basis = p.created_at.map(|_| SinceBasis::Opened);
            p.lateness = None;
        }
    }
    p.stage = stage;
    p.engine_state = Some(state.state.clone());
    p.draft = state.state == "draft";
    p.next_action = state.blockers.first().cloned();
    p.blockers = state.blockers.clone();
    p.asks = if stage == Stage::Review {
        board::asks(state)
    } else {
        vec![]
    };
    p.objectors = state.objectors.clone();
}

/// Whether a PR in `stage` counts against its author's open-slot limit.
fn holds_slot(stage: Stage) -> bool {
    !matches!(stage, Stage::NotGoverned | Stage::Draft)
}

fn take_slot(person: &mut PersonOut, repo: &str) {
    if let Some(n) = person.wip.get_mut(repo) {
        *n = n.saturating_sub(1);
        if *n == 0 {
            person.wip.remove(repo);
        }
    }
}

fn forget_owed(people: &mut [PersonOut], key: &str) {
    for person in people {
        person.owes.retain(|o| o.pr != key);
    }
}

fn person<'a>(people: &'a mut [PersonOut], login: Option<&str>) -> Option<&'a mut PersonOut> {
    let login = login?;
    people
        .iter_mut()
        .find(|p| p.login.eq_ignore_ascii_case(login))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::fixture;
    use crate::view::{assemble, is_good, repo_part, Kept};

    fn view() -> View {
        let mut d = fixture();
        for r in &mut d.repos {
            r.engine_state_available = true;
        }
        let kept = d
            .repos
            .iter()
            .filter(|r| is_good(r))
            .map(|r| {
                (
                    r.repo.clone(),
                    Kept {
                        good_at: d.generated_at,
                        data: repo_part(&d, r),
                    },
                )
            })
            .collect();
        assemble(&d, 1, d.generated_at, &kept)
    }

    fn mergeable(read_at: DateTime<Utc>) -> StoredVerdict {
        StoredVerdict {
            repo: "dashpay/platform".into(),
            number: 3000,
            read_at,
            verdict: PolicyState {
                state: "ready-to-merge".into(),
                ..PolicyState::default()
            },
            stage_seen_at: None,
        }
    }

    /// Deployed before anything is pushed, the service serves what it
    /// served before, to the byte.
    #[test]
    fn with_nothing_pushed_the_view_is_unchanged() {
        let v = view();
        let out = overlay(&v, &[], &[], Utc::now());
        assert_eq!(
            serde_json::to_string(&out).unwrap(),
            serde_json::to_string(&v).unwrap()
        );
    }

    /// What was read before the snapshot's data is already in it.
    #[test]
    fn a_push_not_newer_than_the_data_changes_nothing() {
        let v = view();
        let as_of = v.repos[0].data_as_of.unwrap();
        let open = OpenList {
            repo: "dashpay/platform".into(),
            read_at: as_of,
            numbers: HashSet::new(),
        };
        let out = overlay(&v, &[mergeable(as_of)], &[open], Utc::now());
        assert_eq!(out, v);
    }

    /// Thresholds come from the view's stages and time runs to `now`.
    #[test]
    fn a_moved_pr_is_late_by_its_new_stage_from_the_push() {
        let v = view();
        let as_of = v.repos[0].data_as_of.unwrap();
        let read_at = as_of + chrono::TimeDelta::hours(1);
        let now = read_at + chrono::TimeDelta::hours(30);
        let out = overlay(&v, &[mergeable(read_at)], &[], now);
        let p = out.pr("dashpay/platform#3000").unwrap();
        assert_eq!(p.stage, Stage::Mergeable);
        assert_eq!(p.since, Some(read_at));
        assert_eq!(p.lateness, Some(board::Lateness::Late), "mergeable: 24h");
    }
}
