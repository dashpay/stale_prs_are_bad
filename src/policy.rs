//! The shared review policy: the registry of governed repositories, each
//! repository's area/owner map, and the review engine's exported per-PR state.
//!
//! A policy is the review engine's: it is read with the engine's JSON reader
//! and accepted only when the engine's `validate_policy` accepts it, and the
//! branches it governs are the engine's `governs`. The fields the board reads
//! are taken from that accepted value, so the board cannot route or count
//! slots by a policy the engine would refuse.

use anyhow::{bail, Context, Result};
use pr_hygiene_engine::policy as engine;
use pr_hygiene_engine::pycompat::object::{get_or, getitem, iterate, EMPTY_LIST, NONE};
use pr_hygiene_engine::pycompat::{py_loads, PyDict, PyValue};
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

/// One repository's policy (`policies/<name>.json`) as the engine accepted
/// it, with the fields the dashboard reads. Only [`Policy::parse`] makes
/// one, so every `Policy` is one the engine accepts.
#[derive(Debug, Clone)]
pub struct Policy {
    /// The policy as the engine reads it.
    value: PyValue,
    repository: String,
    fallback: Roster,
    areas: Vec<Area>,
    bot_authors: Vec<String>,
    max_active_prs: u32,
}

impl Policy {
    /// Read a policy file's text with the engine's JSON reader and accept it
    /// only when the engine's `validate_policy` does.
    pub fn parse(text: &str) -> Result<Policy> {
        let value = py_loads(text).context("not JSON the engine reads")?;
        engine::validate_policy(&value)?;
        targets_compile(&value)?;
        let repository = string(getitem(&value, "repository")?)?;
        let fallback = Roster::read(getitem(&value, "fallback")?)?;
        let areas = match getitem(&value, "areas")? {
            PyValue::List(items) => items.iter().map(Area::read).collect::<Result<_>>()?,
            other => bail!("areas is not a list: {other:?}"),
        };
        let bot_authors = strings(get_or(&value, "bot_authors", &EMPTY_LIST)?)?;
        let max_active_prs = match getitem(&value, "max_active_prs")? {
            PyValue::Int(n) => n.as_i64().and_then(|n| u32::try_from(n).ok()),
            _ => None,
        }
        .context("max_active_prs is not a count")?;
        Ok(Policy {
            value,
            repository,
            fallback,
            areas,
            bot_authors,
            max_active_prs,
        })
    }

    /// Whether PRs into `branch` are under this policy: the engine's
    /// `governs`. A target is a branch name or a pattern in the form GitHub
    /// branch rules use, where `*` matches any run of characters except `/`.
    pub fn governs(&self, branch: &str) -> bool {
        engine::governs(&self.value, &PyValue::Str(branch.to_owned()))
            .expect("every target compiled when the policy was parsed")
    }

    /// The roster for files no area claims.
    pub fn fallback(&self) -> &Roster {
        &self.fallback
    }

    /// The areas, in policy order.
    pub fn areas(&self) -> &[Area] {
        &self.areas
    }

    /// Accounts that open PRs with no person behind them.
    pub fn bot_authors(&self) -> &[String] {
        &self.bot_authors
    }

    /// Open PRs per author that hold a review slot.
    pub fn max_active_prs(&self) -> u32 {
        self.max_active_prs
    }
}

/// Refuse a policy with a target the engine cannot match branches against.
/// The engine compiles a target the first time it is matched, refuses one
/// too large to compile, and stops at the first target that matches; asking
/// about each target alone compiles every one, so such a policy is refused
/// here rather than when a branch is looked up.
fn targets_compile(value: &PyValue) -> Result<()> {
    for target in iterate(getitem(value, "target_branches")?)? {
        let target = target.into_owned();
        let alone = PyDict::from_iter([(
            "target_branches".to_owned(),
            PyValue::List(vec![target.clone()].into()),
        )]);
        engine::governs(&PyValue::Dict(alone), &target)?;
    }
    Ok(())
}

/// A string the engine's validation accepted.
fn string(value: &PyValue) -> Result<String> {
    match value {
        PyValue::Str(s) => Ok(s.clone()),
        other => bail!("expected a string, found {other:?}"),
    }
}

/// A list of strings the engine's validation accepted.
fn strings(value: &PyValue) -> Result<Vec<String>> {
    match value {
        PyValue::List(items) => items.iter().map(string).collect(),
        other => bail!("expected a list of strings, found {other:?}"),
    }
}

#[derive(Debug, Clone)]
pub struct Roster {
    pub owners: Vec<String>,
    pub reviewers: Vec<String>,
}

impl Roster {
    fn read(roster: &PyValue) -> Result<Roster> {
        Ok(Roster {
            owners: strings(getitem(roster, "owners")?)?,
            reviewers: strings(getitem(roster, "reviewers")?)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub id: String,
    /// Literal directory prefixes. `""` covers the whole repository.
    pub paths: Vec<String>,
    pub owners: Vec<String>,
    pub reviewers: Vec<String>,
    /// The area lists open questions about who owns it: its roster is not
    /// authoritative yet.
    pub unresolved: bool,
}

impl Area {
    fn read(area: &PyValue) -> Result<Area> {
        Ok(Area {
            id: string(getitem(area, "id")?)?,
            paths: strings(getitem(area, "paths")?)?,
            owners: strings(getitem(area, "owners")?)?,
            reviewers: strings(getitem(area, "reviewers")?)?,
            unresolved: get_or(area, "unresolved", &NONE)?.truthy(),
        })
    }
}

/// Load the registry and every policy it references, in registry order.
/// A policy the engine refuses is an error: routing from a policy the engine
/// would reject must not silently produce a different answer.
pub fn load_registry(policies_root: &Path) -> Result<Vec<(String, Policy)>> {
    let registry_path = policies_root.join(REGISTRY_FILE);
    let text = std::fs::read_to_string(&registry_path)
        .with_context(|| format!("reading registry {}", registry_path.display()))?;
    let registry: Registry = serde_json::from_str(&text)
        .with_context(|| format!("parsing registry {}", registry_path.display()))?;
    let mut seen: HashSet<String> = HashSet::new();
    registry
        .repositories
        .into_iter()
        .map(|entry| {
            if !seen.insert(entry.repository.to_ascii_lowercase()) {
                bail!(
                    "registry {} lists {} more than once",
                    registry_path.display(),
                    entry.repository
                );
            }
            if entry.policy.contains('/')
                || entry.policy.contains('\\')
                || entry.policy.starts_with('.')
            {
                bail!(
                    "registry {} names policy {:?} outside the policies directory",
                    registry_path.display(),
                    entry.policy
                );
            }
            let path = policies_root.join(&entry.policy);
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading policy {}", path.display()))?;
            let policy = Policy::parse(&text)
                .with_context(|| format!("invalid policy {}", path.display()))?;
            if !policy.repository.eq_ignore_ascii_case(&entry.repository) {
                bail!(
                    "policy {} is for {} but the registry lists it for {}",
                    path.display(),
                    policy.repository,
                    entry.repository
                );
            }
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

/// Map changed files onto policy areas by the engine's rule: a file belongs
/// to the first area with a prefix of its path, and a file no area claims
/// falls back to the repository default roster. The engine accepts no
/// policy whose prefixes nest, so no file can belong to a second area.
pub fn route(policy: &Policy, changed_files: &[String]) -> Routing {
    let mut matched: Vec<usize> = Vec::new();
    let mut fallback_used = false;
    for file in changed_files {
        let area = policy.areas.iter().position(|area| {
            area.paths
                .iter()
                .any(|prefix| file.starts_with(prefix.as_str()))
        });
        match area {
            Some(idx) if !matched.contains(&idx) => matched.push(idx),
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
                    unresolved: area.unresolved,
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

/// One row of the engine's `report --format json` output.
#[derive(Debug, Deserialize)]
struct EngineRow {
    repository: String,
    number: u64,
    #[serde(flatten)]
    state: PolicyState,
}

#[derive(Debug, Deserialize)]
struct EngineExport {
    pull_requests: Vec<EngineRow>,
}

/// Read one repository's engine export, keyed by PR number. Every row must
/// belong to `repo`; a file written for another repository is rejected whole.
pub fn load_engine_state(path: &Path, repo: &str) -> Result<HashMap<u64, PolicyState>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading engine state {}", path.display()))?;
    let export: EngineExport = serde_json::from_str(&text)
        .with_context(|| format!("parsing engine state {}", path.display()))?;
    export
        .pull_requests
        .into_iter()
        .map(|row| {
            if !row.repository.eq_ignore_ascii_case(repo) {
                bail!(
                    "engine state {} row #{} belongs to {} not {repo}",
                    path.display(),
                    row.number,
                    row.repository
                );
            }
            Ok((row.number, row.state))
        })
        .collect()
}

/// Load the engine export of every registered repository from `dir`
/// (`<name>.json` per repository). A missing or invalid file is logged and
/// leaves that repository out of the map, which the board renders as
/// "engine state unavailable". `None` or a directory that does not exist is
/// an error only in the latter case: a typo must not silently blank every
/// verdict, while running without exports at all is a supported mode.
pub fn load_engine_states(
    dir: Option<&Path>,
    registry: &[(String, Policy)],
) -> Result<HashMap<String, HashMap<u64, PolicyState>>> {
    let Some(dir) = dir else {
        tracing::warn!(
            "no --policy-state directory; engine state unavailable for every repository"
        );
        return Ok(HashMap::new());
    };
    if !dir.is_dir() {
        bail!("--policy-state {} is not a directory", dir.display());
    }
    let mut out = HashMap::new();
    for (repo, _) in registry {
        let name = repo.rsplit('/').next().unwrap_or(repo);
        match load_engine_state(&dir.join(format!("{name}.json")), repo) {
            Ok(state) => {
                out.insert(repo.clone(), state);
            }
            Err(e) => tracing::warn!(%repo, "engine state unavailable: {e:#}"),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse(policy: serde_json::Value) -> Policy {
        Policy::parse(&policy.to_string()).unwrap_or_else(|e| panic!("{e:#}"))
    }

    fn refusal(policy: serde_json::Value) -> String {
        match Policy::parse(&policy.to_string()) {
            Ok(_) => panic!("accepted {policy}"),
            Err(e) => format!("{e:#}"),
        }
    }

    /// A policy the engine accepts with `areas` as its areas.
    fn with_areas(areas: serde_json::Value) -> serde_json::Value {
        json!({
            "version": 1, "repository": "dashpay/example", "max_active_prs": 5,
            "target_branches": ["dev"],
            "fallback": {"owners": ["fallback-owner"], "reviewers": ["fallback-reviewer"]},
            "areas": areas
        })
    }

    /// The same policy with `targets` as its target branches.
    fn with_targets(targets: &[&str]) -> serde_json::Value {
        let mut policy = with_areas(json!([]));
        policy["target_branches"] = json!(targets);
        policy
    }

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|s| s.to_string()).collect()
    }

    fn example() -> serde_json::Value {
        with_areas(json!([
            {"id": "drive", "paths": ["packages/rs-drive/"], "owners": ["qe"], "reviewers": ["shumkov"]},
            {"id": "drive-abci", "paths": ["packages/rs-drive-abci/"], "owners": ["lklimek"], "reviewers": []},
            {"id": "sdk", "paths": ["packages/rs-sdk/"], "owners": ["shumkov"], "reviewers": ["qe"]}
        ]))
    }

    fn policy() -> Policy {
        parse(example())
    }

    fn ids(routing: &Routing) -> Vec<&str> {
        routing.areas.iter().map(|a| a.id.as_str()).collect()
    }

    #[test]
    fn target_branches_match_like_github_branch_rules() {
        let p = parse(with_targets(&["v*-dev", "develop"]));
        for b in ["v4.2-dev", "v5.1-dev", "v6.0-dev", "develop"] {
            assert!(p.governs(b), "{b}");
        }
        for b in [
            "master",
            "feat/v5-dev",
            "v5/x-dev",
            "v5.1-dev-old",
            "developer",
        ] {
            assert!(!p.governs(b), "{b}");
        }
        let any = parse(with_targets(&["*"]));
        assert!(
            !any.governs(""),
            "a PR with no known base is never governed"
        );
    }

    /// `*` is the one pattern: it matches within a path segment, never
    /// across `/`, and every other character is itself. Anything GitHub's
    /// rules read as a pattern besides `*` would match no branch the way it
    /// reads there, so a policy using one is refused.
    #[test]
    fn star_stays_within_a_segment_and_no_other_pattern_is_accepted() {
        let cases: [(&str, &[&str], &[&str]); 5] = [
            (
                "release/*",
                &["release/v1", "release/"],
                &["release/v1/hotfix", "release", "releases/v1"],
            ),
            ("*", &["main", "v5.0-dev"], &["feat/x", "/", ""]),
            (
                "v*-dev",
                &["v5.0-dev", "v-dev"],
                &["v5/0-dev", "v5.0-dev/x"],
            ),
            ("a*b*c", &["abc", "aXbYc"], &["a/b/c", "aXbYc/"]),
            ("v.+", &["v.+"], &["vX+", "v.."]),
        ];
        for (target, governed, not) in cases {
            let p = parse(with_targets(&[target]));
            for b in governed {
                assert!(p.governs(b), "{target} should govern {b:?}");
            }
            for b in not {
                assert!(!p.governs(b), "{target} should not govern {b:?}");
            }
        }
        for target in [
            "v?-dev",
            "v[0-9]-dev",
            "v]-dev",
            "v{5,6}-dev",
            "v}-dev",
            r"v\d-dev",
            "v**-dev",
            "**",
        ] {
            let err = refusal(with_targets(&[target]));
            assert!(
                err.contains("Target branch patterns support only `*`"),
                "{target}: {err}"
            );
        }
    }

    #[test]
    fn each_file_routes_to_the_one_area_claiming_its_directory() {
        let r = route(&policy(), &files(&["packages/rs-drive-abci/src/lib.rs"]));
        assert_eq!(ids(&r), vec!["drive-abci"]);
        assert!(!r.fallback_used);

        let r = route(&policy(), &files(&["packages/rs-drive/src/lib.rs"]));
        assert_eq!(ids(&r), vec!["drive"]);
    }

    #[test]
    fn empty_prefix_covers_whole_repo() {
        let p = parse(with_areas(json!([
            {"id": "everything", "paths": [""], "owners": ["root"], "reviewers": []}
        ])));
        let r = route(&p, &files(&["README.md", "packages/rs-sdk/x.rs"]));
        assert_eq!(ids(&r), vec!["everything"]);
        assert!(!r.fallback_used);

        // `""` is a prefix of every path, so beside another area it nests:
        // no area can be carved out of a catch-all.
        let err = refusal(with_areas(json!([
            {"id": "sdk", "paths": ["packages/rs-sdk/"], "owners": ["a"], "reviewers": []},
            {"id": "everything", "paths": [""], "owners": ["root"], "reviewers": []}
        ])));
        assert!(err.contains("Overlapping directory prefixes"), "{err}");
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
        let mut p = example();
        p["areas"][0]["unresolved"] = json!(["Owner: unknown"]);
        let r = route(&parse(p), &files(&["packages/rs-drive/x.rs"]));
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

    /// The fields the board reads come from the policy the engine accepted,
    /// including what the engine allows beside them (an area's `metadata`)
    /// and the defaults of what a policy may leave out.
    #[test]
    fn policy_json_reads_the_fields_the_board_uses() {
        let p = parse(json!({
            "version": 1,
            "repository": "dashpay/x",
            "max_active_prs": 5,
            "target_branches": ["dev"],
            "fallback": {"owners": ["a"], "reviewers": []},
            "areas": [{"id": "core", "paths": ["core/"], "owners": ["b"], "reviewers": [],
                       "metadata": {"contributors": ["c"]}}]
        }));
        assert_eq!(p.fallback().owners, vec!["a"]);
        assert_eq!(p.areas()[0].id, "core");
        assert!(!p.areas()[0].unresolved);
        assert!(p.bot_authors().is_empty());
        assert_eq!(p.max_active_prs(), 5);
    }

    #[test]
    fn engine_export_parses_rows() {
        let dir = std::env::temp_dir().join(format!("pr-hygiene-engine-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("platform.json");
        std::fs::write(
            &path,
            r#"{"generated_at": "2026-09-11T00:00:00Z", "pull_requests": [
                {"repository": "dashpay/platform", "number": 7, "state": "waiting-bots", "status": "pending",
                 "blockers": ["CodeRabbit review pending"], "reviewers": ["shumkov"],
                 "areas": ["dpp"], "admitted_at": null, "ready_since": null},
                {"repository": "dashpay/platform", "number": 9, "state": "ready-to-merge", "status": "success"}
            ]}"#,
        )
        .unwrap();
        let map = load_engine_state(&path, "dashpay/platform").unwrap();
        let wrong_repo = load_engine_state(&path, "dashpay/grovedb").unwrap_err();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(
            wrong_repo
                .to_string()
                .contains("belongs to dashpay/platform"),
            "got: {wrong_repo}"
        );
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
        let err = load_engine_state(&path, "dashpay/x").unwrap_err();
        assert!(err.to_string().contains("parsing engine state"));
        // An export without the rows array is unavailable, not "no verdicts".
        std::fs::write(&path, "{}").unwrap();
        assert!(load_engine_state(&path, "dashpay/x").is_err());
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(load_engine_state(&dir.join("missing.json"), "dashpay/x").is_err());
    }

    #[test]
    fn engine_states_skip_broken_files_but_reject_missing_dir() {
        let dir = std::env::temp_dir().join(format!("pr-hygiene-states-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let registry = vec![
            ("dashpay/platform".to_string(), policy()),
            ("dashpay/grovedb".to_string(), policy()),
        ];
        std::fs::write(
            dir.join("platform.json"),
            r#"{"pull_requests": [{"repository": "dashpay/platform", "number": 1, "state": "ready-for-human"}]}"#,
        )
        .unwrap();
        let states = load_engine_states(Some(&dir), &registry).unwrap();
        assert_eq!(states["dashpay/platform"][&1].state, "ready-for-human");
        assert!(
            !states.contains_key("dashpay/grovedb"),
            "no file → unavailable"
        );
        assert!(load_engine_states(None, &registry).unwrap().is_empty());
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(
            load_engine_states(Some(&dir), &registry).is_err(),
            "typo'd dir is fatal"
        );
    }

    fn one_area(paths: &[&str]) -> serde_json::Value {
        with_areas(json!([{"id": "x", "paths": paths, "owners": ["a"], "reviewers": []}]))
    }

    #[test]
    fn validation_rejects_missing_trailing_slash() {
        let err = refusal(one_area(&["packages/rs-sdk"]));
        assert!(err.contains("Invalid literal directory prefix"), "{err}");
        for bad in ["/packages/", "packages/../x/", "./x/", "a//b/", "a b/"] {
            let err = refusal(one_area(&[bad]));
            assert!(
                err.contains("Invalid literal directory prefix"),
                "{bad:?}: {err}"
            );
        }
        for good in ["", "packages/rs-sdk/", "a/b-c.d_e/"] {
            Policy::parse(&one_area(&[good]).to_string())
                .unwrap_or_else(|e| panic!("{good:?}: {e:#}"));
        }
    }

    #[test]
    fn validation_rejects_nested_prefixes_but_allows_siblings() {
        let err = refusal(with_areas(json!([
            {"id": "drive", "paths": ["packages/rs-drive/"], "owners": ["a"], "reviewers": []},
            {"id": "abci", "paths": ["packages/rs-drive/abci/"], "owners": ["b"], "reviewers": []}
        ])));
        assert!(err.contains("Overlapping directory prefixes"), "{err}");

        let siblings = parse(with_areas(json!([
            {"id": "drive", "paths": ["packages/rs-drive/"], "owners": ["a"], "reviewers": []},
            {"id": "abci", "paths": ["packages/rs-drive-abci/"], "owners": ["b"], "reviewers": []}
        ])));
        // A shared string prefix is not a directory prefix: only abci matches.
        let r = route(&siblings, &files(&["packages/rs-drive-abci/src/x.rs"]));
        assert_eq!(ids(&r), vec!["abci"]);
        assert!(!r.fallback_used);
    }

    /// An area without owners is accepted only with its open ownership
    /// questions listed; reviewers alone do not stand in for an owner.
    #[test]
    fn validation_requires_owners_or_unresolved() {
        let area = |owners: &[&str], reviewers: &[&str], unresolved: Option<&[&str]>| {
            let mut area =
                json!({"id": "x", "paths": ["x/"], "owners": owners, "reviewers": reviewers});
            if let Some(unresolved) = unresolved {
                area["unresolved"] = json!(unresolved);
            }
            with_areas(json!([area]))
        };
        let reason = "Missing owners require an explicit unresolved identity";
        assert!(refusal(area(&[], &[], None)).contains(reason));
        assert!(refusal(area(&[], &["r"], None)).contains(reason));
        assert!(refusal(area(&[], &["r"], Some(&[]))).contains(reason));
        parse(area(&[], &[], Some(&["Owner: unknown"])));
        parse(area(&[], &["r"], Some(&["Owner: unknown"])));
        parse(area(&["o"], &[], None));
    }

    #[test]
    fn registry_rejects_duplicate_and_invalid_entries() {
        let dir = std::env::temp_dir().join(format!("pr-hygiene-registry-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let good = r#"{"version": 1, "repository": "dashpay/x", "max_active_prs": 5, "target_branches": ["dev"],
                       "fallback": {"owners": ["a"], "reviewers": []},
                       "areas": [{"id": "core", "paths": ["core/"], "owners": ["b"], "reviewers": []}]}"#;
        std::fs::write(dir.join("x.json"), good).unwrap();
        std::fs::write(
            dir.join("repositories.json"),
            r#"{"version": 1, "repositories": [
                {"repository": "dashpay/x", "policy": "x.json"},
                {"repository": "dashpay/x", "policy": "x.json"}]}"#,
        )
        .unwrap();
        let err = load_registry(&dir).unwrap_err().to_string();
        assert!(err.contains("more than once"), "{err}");

        std::fs::write(
            dir.join("repositories.json"),
            r#"{"version": 1, "repositories": [{"repository": "dashpay/x", "policy": "x.json"}]}"#,
        )
        .unwrap();
        let loaded = load_registry(&dir).unwrap();
        assert_eq!(loaded[0].0, "dashpay/x");

        std::fs::write(dir.join("x.json"), good.replace("core/", "core")).unwrap();
        let err = format!("{:#}", load_registry(&dir).unwrap_err());
        assert!(err.contains("invalid policy"), "{err}");

        std::fs::write(dir.join("x.json"), "{").unwrap();
        let err = format!("{:#}", load_registry(&dir).unwrap_err());
        assert!(err.contains("invalid policy"), "{err}");
        assert!(err.contains("not JSON"), "{err}");

        std::fs::write(
            dir.join("x.json"),
            good.replace("dashpay/x", "dashpay/other"),
        )
        .unwrap();
        let err = format!("{:#}", load_registry(&dir).unwrap_err());
        assert!(err.contains("registry lists it for"), "{err}");

        std::fs::write(dir.join("x.json"), good).unwrap();
        std::fs::write(
            dir.join("repositories.json"),
            r#"{"version": 1, "repositories": [{"repository": "dashpay/x", "policy": "../x.json"}]}"#,
        )
        .unwrap();
        let err = format!("{:#}", load_registry(&dir).unwrap_err());
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(err.contains("outside the policies directory"), "{err}");
    }

    /// Load `policy` as the only entry of a registry, from a scratch
    /// directory named after `case`.
    fn load_one(case: &str, policy: &serde_json::Value) -> Result<Vec<(String, Policy)>> {
        let dir =
            std::env::temp_dir().join(format!("pr-hygiene-refused-{case}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("x.json"), policy.to_string()).unwrap();
        std::fs::write(
            dir.join(REGISTRY_FILE),
            r#"{"version": 1, "repositories": [{"repository": "dashpay/x", "policy": "x.json"}]}"#,
        )
        .unwrap();
        let loaded = load_registry(&dir);
        std::fs::remove_dir_all(&dir).unwrap();
        loaded
    }

    /// A policy the engine accepts, which each case below breaks in one way.
    fn engine_valid() -> serde_json::Value {
        serde_json::json!({
            "version": 1, "repository": "dashpay/x", "max_active_prs": 5,
            "target_branches": ["dev"],
            "fallback": {"owners": ["a"], "reviewers": []},
            "areas": [{"id": "core", "paths": ["core/"], "owners": ["b"], "reviewers": []}]
        })
    }

    /// The board routes, counts slots and decides what is governed from the
    /// policy the engine enforces. A policy the engine refuses gets no
    /// verdicts at all, so the board must refuse it too rather than show
    /// routing and slots the engine never applies. Each case names the
    /// engine's own reason.
    #[test]
    fn a_policy_the_engine_refuses_is_refused_at_load() {
        load_one("valid", &engine_valid()).expect("the base case is valid");
        type Break = fn(&mut serde_json::Value);
        let cases: [(&str, Break, &str); 13] = [
            (
                "reviewers-without-owners",
                |p| {
                    p["areas"][0]["owners"] = serde_json::json!([]);
                    p["areas"][0]["reviewers"] = serde_json::json!(["c"]);
                },
                "Missing owners require an explicit unresolved identity",
            ),
            (
                "unknown-field",
                |p| p["owner"] = serde_json::json!("a"),
                "Unknown or missing policy fields",
            ),
            (
                "no-target-branches",
                |p| {
                    p.as_object_mut().unwrap().remove("target_branches");
                },
                "Unknown or missing policy fields",
            ),
            (
                "question-mark",
                |p| p["target_branches"] = serde_json::json!(["v?-dev"]),
                "Target branch patterns support only `*`",
            ),
            (
                "double-star",
                |p| p["target_branches"] = serde_json::json!(["v**-dev"]),
                "Target branch patterns support only `*`",
            ),
            (
                "duplicate-target",
                |p| p["target_branches"] = serde_json::json!(["dev", "dev"]),
                "Invalid target branches",
            ),
            (
                "owner-also-reviewer",
                |p| p["areas"][0]["reviewers"] = serde_json::json!(["B"]),
                "Owner and reviewer roles overlap",
            ),
            (
                "area-id-case",
                |p| p["areas"][0]["id"] = serde_json::json!("Core"),
                "Invalid or duplicate area id",
            ),
            (
                "three-slots",
                |p| p["max_active_prs"] = serde_json::json!(3),
                "Expected five active slots",
            ),
            (
                "version-two",
                |p| p["version"] = serde_json::json!(2),
                "Unsupported policy version",
            ),
            (
                "machine-owner",
                |p| p["bot_authors"] = serde_json::json!(["b"]),
                "A machine author cannot own or review: b",
            ),
            (
                "no-fallback-owner",
                |p| p["fallback"]["owners"] = serde_json::json!([]),
                "Expected handle list",
            ),
            (
                "excluded-identity",
                |p| p["areas"][0]["owners"] = serde_json::json!(["dependabot"]),
                "Duplicate or excluded identity",
            ),
        ];
        for (case, break_it, reason) in cases {
            let mut policy = engine_valid();
            break_it(&mut policy);
            let err = match load_one(case, &policy) {
                Ok(_) => panic!("{case}: accepted a policy the engine refuses ({reason})"),
                Err(e) => format!("{e:#}"),
            };
            assert!(err.contains("invalid policy"), "{case}: {err}");
            assert!(err.contains(reason), "{case}: {err}");
        }
    }

    /// A registered policy: its repository, its file's text, the value the
    /// engine reads from it, and the board's policy.
    struct Registered {
        repo: String,
        text: String,
        raw: PyValue,
        policy: Policy,
    }

    /// Every registered policy.
    fn registered() -> Vec<Registered> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("policies");
        let registry: Registry =
            serde_json::from_str(&std::fs::read_to_string(root.join(REGISTRY_FILE)).unwrap())
                .unwrap();
        let loaded = load_registry(&root).expect("every registered policy loads");
        assert_eq!(loaded.len(), registry.repositories.len());
        registry
            .repositories
            .into_iter()
            .zip(loaded)
            .map(|(entry, (repo, policy))| {
                let text = std::fs::read_to_string(root.join(&entry.policy)).unwrap();
                let raw = py_loads(&text).unwrap();
                Registered {
                    repo,
                    text,
                    raw,
                    policy,
                }
            })
            .collect()
    }

    fn targets(raw: &PyValue) -> Vec<String> {
        strings(getitem(raw, "target_branches").unwrap()).unwrap()
    }

    /// The board accepts a registered policy, or any variation of one,
    /// exactly when the engine does. Most variations break a rule the
    /// engine checks, and the board refuses them with the engine's reason;
    /// for the ones the engine accepts, this holds what the board adds on
    /// top — compiling the targets and reading the typed fields — to never
    /// refuse what the engine accepts.
    #[test]
    fn every_registered_policy_is_accepted_or_refused_as_the_engine_does() {
        type Vary = fn(&mut serde_json::Value);
        let variations: [(&str, Vary); 11] = [
            ("as written", |_| {}),
            ("unknown field", |p| p["owner"] = json!("a")),
            ("no areas", |p| {
                p.as_object_mut().unwrap().remove("areas");
            }),
            ("no area at all", |p| p["areas"] = json!([])),
            ("question mark", |p| {
                p["target_branches"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!("v?"))
            }),
            ("double star", |p| {
                p["target_branches"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!("**"))
            }),
            ("prefix without slash", |p| {
                p["areas"][0]["paths"] = json!(["no-slash"])
            }),
            ("nested prefix", |p| {
                let nested = format!("{}nested/", p["areas"][0]["paths"][0].as_str().unwrap());
                let last = p["areas"].as_array().unwrap().len() - 1;
                p["areas"][last]["paths"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(nested));
            }),
            ("four slots", |p| p["max_active_prs"] = json!(4)),
            ("owner as machine", |p| {
                p["bot_authors"] = json!([p["fallback"]["owners"][0].clone()])
            }),
            ("area named fallback", |p| {
                p["areas"][0]["id"] = json!("fallback")
            }),
        ];
        for Registered { repo, text, .. } in registered() {
            let original: serde_json::Value = serde_json::from_str(&text).unwrap();
            for (name, vary) in variations {
                let mut policy = original.clone();
                vary(&mut policy);
                let text = policy.to_string();
                let engine = engine::validate_policy(&py_loads(&text).unwrap());
                let board = Policy::parse(&text);
                assert_eq!(
                    board.is_ok(),
                    engine.is_ok(),
                    "{repo}, {name}: board {:?}, engine {engine:?}",
                    board.err()
                );
                if let (Err(board), Err(engine)) = (board, engine) {
                    let board = format!("{board:#}");
                    assert!(
                        board.contains(&engine.to_string()),
                        "{repo}, {name}: {board}"
                    );
                }
            }
        }
    }

    /// Branches the registered policies are asked about, besides each
    /// target and the branches a target's `*` could stand for.
    const BRANCHES: [&str; 14] = [
        "",
        "master",
        "main",
        "develop",
        "dev",
        "v5.0-dev",
        "v-dev",
        "v5/x-dev",
        "feat/v5-dev",
        "v5.1-dev-old",
        "developer",
        "V5.0-DEV",
        "v5.0-dev/",
        " dev",
    ];

    /// The board asks the engine which branches a policy governs, on the
    /// value the engine read from the policy's file. This holds it there:
    /// a matcher of the board's own, or a policy value the board reshaped,
    /// would answer differently on some of these branches.
    #[test]
    fn every_registered_policy_governs_the_branches_the_engine_does() {
        for Registered {
            repo, raw, policy, ..
        } in registered()
        {
            let mut branches: Vec<String> = BRANCHES.iter().map(|b| b.to_string()).collect();
            for target in &targets(&raw) {
                for stand_in in ["", "x", "5.0", "a/b", "/"] {
                    branches.push(target.replace('*', stand_in));
                }
                branches.push(target.clone());
                branches.push(format!("feat/{target}"));
            }
            for branch in branches {
                let engine = engine::governs(&raw, &PyValue::Str(branch.clone())).unwrap();
                assert_eq!(policy.governs(&branch), engine, "{repo}: {branch:?}");
            }
        }
    }

    /// The areas the engine's `evaluate` records for an open PR into `base`
    /// that changes `files`: the engine routes before anything about the
    /// people or the reviews can stop it.
    fn engine_areas(raw: &PyValue, base: &str, files: &[String]) -> Vec<String> {
        let pr = json!({
            "number": 1, "author": "someone", "head": "a".repeat(40), "base": base,
            "base_sha": "b".repeat(40), "created_at": "2026-01-01T00:00:00Z",
            "draft": false, "state": "open",
            "files": files.iter().map(|f| json!({"filename": f})).collect::<Vec<_>>(),
            "reviews": [], "comments": [], "threads": [], "permissions": {},
            "complete": true, "build": null
        });
        let verdict = engine::evaluate(
            raw,
            &py_loads(&pr.to_string()).unwrap(),
            &PyValue::None,
            &PyValue::Str("2026-01-02T00:00:00Z".into()),
            &PyValue::None,
        )
        .unwrap();
        let Some(PyValue::List(areas)) = verdict.get("areas") else {
            panic!("no areas in {verdict:?}");
        };
        assert!(
            !areas.is_empty(),
            "the engine stopped before routing: {verdict:?}"
        );
        strings(&PyValue::List(areas.clone())).unwrap()
    }

    /// The board's routing is not the engine's code, so it is held to the
    /// engine's answer: for every registered policy, each file — one in
    /// every area, one beside every area's directory, and some no area
    /// claims — lands in the areas the engine's verdict names.
    #[test]
    fn every_registered_policy_routes_files_where_the_engine_does() {
        for Registered {
            repo, raw, policy, ..
        } in registered()
        {
            let base = targets(&raw)[0].replace('*', "x");
            assert!(policy.governs(&base), "{repo}: {base}");
            let mut paths = files(&["README.md", "Cargo.toml", ".github/workflows/ci.yml"]);
            for area in policy.areas() {
                for prefix in &area.paths {
                    paths.push(format!("{prefix}x.rs"));
                    paths.push(format!("{prefix}deep/er/y.rs"));
                    paths.push(format!("{}-sibling/z.rs", prefix.trim_end_matches('/')));
                }
            }
            let board = |files: &[String]| {
                let routing = route(&policy, files);
                let mut ids: Vec<String> = routing.areas.into_iter().map(|a| a.id).collect();
                if routing.fallback_used {
                    ids.push("fallback".into());
                }
                ids.sort();
                ids
            };
            for path in &paths {
                let one = std::slice::from_ref(path);
                assert_eq!(board(one), engine_areas(&raw, &base, one), "{repo}: {path}");
            }
            assert_eq!(board(&paths), engine_areas(&raw, &base, &paths), "{repo}");
        }
    }
}
