//! The shared review policy: the registry of governed repositories, each
//! repository's area/owner map, and the review engine's exported per-PR state.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::model::PolicyState;

pub const REGISTRY_FILE: &str = "repositories.json";

/// `policies/repositories.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct Registry {
    pub repositories: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistryEntry {
    /// `owner/name`.
    pub repository: String,
    /// Policy file name, relative to the policies root.
    pub policy: String,
}

/// One repository's policy (`policies/<name>.json`). Only the fields the
/// dashboard routes on are modelled; everything else is ignored.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Policy {
    #[serde(default)]
    pub fallback: Roster,
    #[serde(default)]
    pub areas: Vec<Area>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Roster {
    #[serde(default)]
    pub owners: Vec<String>,
    #[serde(default)]
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Area {
    pub id: String,
    /// Literal directory prefixes. `""` covers the whole repository.
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub owners: Vec<String>,
    #[serde(default)]
    pub reviewers: Vec<String>,
    /// Open questions about who owns the area. Non-empty means the area's
    /// roster is not authoritative yet.
    #[serde(default)]
    pub unresolved: Vec<String>,
}

/// Load the registry and every policy it references, in registry order.
pub fn load_registry(policies_root: &Path) -> Result<Vec<(String, Policy)>> {
    let registry_path = policies_root.join(REGISTRY_FILE);
    let text = std::fs::read_to_string(&registry_path)
        .with_context(|| format!("reading registry {}", registry_path.display()))?;
    let registry: Registry = serde_json::from_str(&text)
        .with_context(|| format!("parsing registry {}", registry_path.display()))?;
    registry
        .repositories
        .into_iter()
        .map(|entry| {
            let path = policies_root.join(&entry.policy);
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading policy {}", path.display()))?;
            let policy: Policy = serde_json::from_str(&text)
                .with_context(|| format!("parsing policy {}", path.display()))?;
            Ok((entry.repository, policy))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AreaMatch {
    pub id: String,
    pub owners: Vec<String>,
    pub reviewers: Vec<String>,
    pub unresolved: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Routing {
    /// Matched areas in policy order, each at most once.
    pub areas: Vec<AreaMatch>,
    /// True when at least one changed file matched no area.
    pub fallback_used: bool,
}

/// Map changed files onto policy areas. Each file goes to the area owning
/// its longest matching path prefix; a file no area claims falls back to the
/// repository default roster.
pub fn route(policy: &Policy, changed_files: &[String]) -> Routing {
    let mut matched: Vec<usize> = Vec::new();
    let mut fallback_used = false;
    for file in changed_files {
        let best = policy
            .areas
            .iter()
            .enumerate()
            .filter_map(|(idx, area)| {
                area.paths
                    .iter()
                    .filter(|prefix| file.starts_with(prefix.as_str()))
                    .map(|prefix| prefix.len())
                    .max()
                    .map(|len| (len, idx))
            })
            // Ties can't happen (prefixes don't overlap); prefer the earlier area anyway.
            .max_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
        match best {
            Some((_, idx)) if !matched.contains(&idx) => matched.push(idx),
            Some(_) => {}
            None => fallback_used = true,
        }
    }
    matched.sort_unstable();
    Routing {
        areas: matched
            .into_iter()
            .map(|idx| {
                let area = &policy.areas[idx];
                AreaMatch {
                    id: area.id.clone(),
                    owners: area.owners.clone(),
                    reviewers: area.reviewers.clone(),
                    unresolved: !area.unresolved.is_empty(),
                }
            })
            .collect(),
        fallback_used,
    }
}

/// Everyone the policy makes responsible for reviewing a PR: owners and
/// reviewers of each matched area, plus the fallback roster when a file
/// matched no area. The author never reviews their own PR, and anyone who
/// already submitted a review (any state) has done their job. Deduplicated
/// case-insensitively, first spelling wins.
pub fn candidate_reviewers(
    policy: &Policy,
    routing: &Routing,
    author: Option<&str>,
    already_reviewed: &HashSet<String>,
) -> Vec<String> {
    let rosters = routing
        .areas
        .iter()
        .map(|a| (&a.owners, &a.reviewers))
        .chain(
            routing
                .fallback_used
                .then_some((&policy.fallback.owners, &policy.fallback.reviewers)),
        );
    let mut out: Vec<String> = Vec::new();
    for (owners, reviewers) in rosters {
        for login in owners.iter().chain(reviewers) {
            let lc = login.to_ascii_lowercase();
            if author.is_some_and(|a| a.eq_ignore_ascii_case(login)) {
                continue;
            }
            if already_reviewed.contains(&lc) {
                continue;
            }
            if !out.iter().any(|r| r.eq_ignore_ascii_case(login)) {
                out.push(login.clone());
            }
        }
    }
    out
}

/// One row of the engine's `sync --format json` output.
#[derive(Debug, Deserialize)]
struct EngineRow {
    number: u64,
    #[serde(flatten)]
    state: PolicyState,
}

#[derive(Debug, Deserialize)]
struct EngineExport {
    #[serde(default)]
    pull_requests: Vec<EngineRow>,
}

/// Read one repository's engine export, keyed by PR number.
pub fn load_engine_state(path: &Path) -> Result<HashMap<u64, PolicyState>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading engine state {}", path.display()))?;
    let export: EngineExport = serde_json::from_str(&text)
        .with_context(|| format!("parsing engine state {}", path.display()))?;
    Ok(export
        .pull_requests
        .into_iter()
        .map(|row| (row.number, row.state))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn area(id: &str, paths: &[&str], owners: &[&str], reviewers: &[&str]) -> Area {
        Area {
            id: id.into(),
            paths: paths.iter().map(|s| s.to_string()).collect(),
            owners: owners.iter().map(|s| s.to_string()).collect(),
            reviewers: reviewers.iter().map(|s| s.to_string()).collect(),
            unresolved: vec![],
        }
    }

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|s| s.to_string()).collect()
    }

    fn policy() -> Policy {
        Policy {
            fallback: Roster {
                owners: vec!["fallback-owner".into()],
                reviewers: vec!["fallback-reviewer".into()],
            },
            areas: vec![
                area("drive", &["packages/rs-drive/"], &["qe"], &["shumkov"]),
                area(
                    "drive-abci",
                    &["packages/rs-drive/abci/"],
                    &["lklimek"],
                    &[],
                ),
                area("sdk", &["packages/rs-sdk/"], &["shumkov"], &["qe"]),
            ],
        }
    }

    fn ids(routing: &Routing) -> Vec<&str> {
        routing.areas.iter().map(|a| a.id.as_str()).collect()
    }

    #[test]
    fn longest_prefix_wins() {
        let r = route(&policy(), &files(&["packages/rs-drive/abci/src/lib.rs"]));
        assert_eq!(ids(&r), vec!["drive-abci"]);
        assert!(!r.fallback_used);

        let r = route(&policy(), &files(&["packages/rs-drive/src/lib.rs"]));
        assert_eq!(ids(&r), vec!["drive"]);
    }

    #[test]
    fn empty_prefix_covers_whole_repo() {
        let mut p = policy();
        p.areas.push(area("everything", &[""], &["root"], &[]));
        let r = route(&p, &files(&["README.md", "packages/rs-sdk/x.rs"]));
        assert_eq!(ids(&r), vec!["sdk", "everything"]);
        assert!(!r.fallback_used);
    }

    #[test]
    fn unmatched_file_uses_fallback() {
        let r = route(
            &policy(),
            &files(&["packages/rs-sdk/x.rs", "docs/README.md"]),
        );
        assert_eq!(ids(&r), vec!["sdk"]);
        assert!(r.fallback_used);
    }

    #[test]
    fn no_files_matches_nothing() {
        let r = route(&policy(), &[]);
        assert!(r.areas.is_empty());
        assert!(!r.fallback_used);
    }

    #[test]
    fn matched_area_appears_once_in_policy_order() {
        let r = route(
            &policy(),
            &files(&[
                "packages/rs-sdk/a.rs",
                "packages/rs-drive/a.rs",
                "packages/rs-sdk/b.rs",
            ]),
        );
        assert_eq!(ids(&r), vec!["drive", "sdk"]);
    }

    #[test]
    fn unresolved_area_is_flagged() {
        let mut p = policy();
        p.areas[0].unresolved = vec!["Owner: unknown".into()];
        let r = route(&p, &files(&["packages/rs-drive/x.rs"]));
        assert!(r.areas[0].unresolved);
        assert_eq!(r.areas[0].owners, vec!["qe"]);
    }

    fn reviewed(logins: &[&str]) -> HashSet<String> {
        logins.iter().map(|s| s.to_ascii_lowercase()).collect()
    }

    #[test]
    fn candidates_are_owners_union_reviewers_of_every_matched_area() {
        let p = policy();
        let r = route(
            &p,
            &files(&["packages/rs-drive/a.rs", "packages/rs-sdk/b.rs"]),
        );
        let c = candidate_reviewers(&p, &r, Some("alice"), &reviewed(&[]));
        // drive: qe + shumkov; sdk: shumkov + qe — deduplicated, area order kept.
        assert_eq!(c, vec!["qe", "shumkov"]);
    }

    #[test]
    fn candidates_include_fallback_roster_only_when_used() {
        let p = policy();
        let r = route(&p, &files(&["packages/rs-sdk/b.rs"]));
        let c = candidate_reviewers(&p, &r, Some("alice"), &reviewed(&[]));
        assert_eq!(c, vec!["shumkov", "qe"]);

        let r = route(&p, &files(&["packages/rs-sdk/b.rs", "Cargo.toml"]));
        let c = candidate_reviewers(&p, &r, Some("alice"), &reviewed(&[]));
        assert_eq!(
            c,
            vec!["shumkov", "qe", "fallback-owner", "fallback-reviewer"]
        );
    }

    #[test]
    fn author_is_excluded_case_insensitively() {
        let p = policy();
        let r = route(&p, &files(&["packages/rs-drive/a.rs"]));
        let c = candidate_reviewers(&p, &r, Some("Shumkov"), &reviewed(&[]));
        assert_eq!(c, vec!["qe"]);
    }

    #[test]
    fn already_reviewed_is_excluded() {
        let p = policy();
        let r = route(&p, &files(&["packages/rs-drive/a.rs"]));
        let c = candidate_reviewers(&p, &r, Some("alice"), &reviewed(&["QE"]));
        assert_eq!(c, vec!["shumkov"]);
        let c = candidate_reviewers(&p, &r, Some("alice"), &reviewed(&["qe", "shumkov"]));
        assert!(c.is_empty(), "everyone responsible already reviewed");
    }

    #[test]
    fn policy_json_tolerates_unknown_fields() {
        let json = r#"{
            "version": 1,
            "repository": "dashpay/x",
            "max_active_prs": 5,
            "target_branches": ["dev"],
            "fallback": {"owners": ["a"], "reviewers": []},
            "areas": [{"id": "core", "paths": ["core/"], "owners": ["b"], "reviewers": [],
                       "metadata": {"contributors": ["c"]}}]
        }"#;
        let p: Policy = serde_json::from_str(json).unwrap();
        assert_eq!(p.fallback.owners, vec!["a"]);
        assert_eq!(p.areas[0].id, "core");
        assert!(p.areas[0].unresolved.is_empty());
    }

    #[test]
    fn engine_export_parses_rows() {
        let dir = std::env::temp_dir().join(format!("pr-hygiene-engine-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("platform.json");
        std::fs::write(
            &path,
            r#"{"generated_at": "2026-09-11T00:00:00Z", "pull_requests": [
                {"number": 7, "state": "waiting-bots", "status": "pending",
                 "blockers": ["CodeRabbit review pending"], "reviewers": ["shumkov"],
                 "areas": ["dpp"], "admitted_at": null, "ready_since": null},
                {"number": 9, "state": "ready-to-merge", "status": "success"}
            ]}"#,
        )
        .unwrap();
        let map = load_engine_state(&path).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(map[&7].state, "waiting-bots");
        assert_eq!(map[&7].blockers, vec!["CodeRabbit review pending"]);
        assert!(!map[&7].is_ready_for_human());
        assert!(map[&9].blockers.is_empty());
        assert!(map[&9].is_ready_for_human());
    }

    #[test]
    fn engine_export_rejects_garbage() {
        let dir = std::env::temp_dir().join(format!("pr-hygiene-garbage-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("x.json");
        std::fs::write(&path, "not json").unwrap();
        let err = load_engine_state(&path).unwrap_err();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(err.to_string().contains("parsing engine state"));
        assert!(load_engine_state(&dir.join("missing.json")).is_err());
    }
}
