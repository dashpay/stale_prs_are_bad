//! The shared review policy: the registry of governed repositories, each
//! repository's area/owner map, and the review engine's exported per-PR state.
//!
//! A policy is accepted only when it passes the same checks as the review
//! engine's `validate_policy` in `pr_review/policy.py`, with the same
//! messages, so the board never routes or counts slots by a policy the
//! engine would refuse. The engine stays the authority: the reusable
//! workflow validates every policy with it before any of them is used.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

use crate::model::PolicyState;

pub const REGISTRY_FILE: &str = "repositories.json";

/// The review bots, by every login they write under: the engine's `BOTS`.
/// A policy may not name them, and the board never counts them as people.
pub const BOTS: [&str; 4] = [
    "thepastaclaw",
    "coderabbitai",
    "coderabbitai[bot]",
    "claudbot[bot]",
];

/// The review bots a policy may require: the engine's `REVIEW_BOTS`.
const REVIEW_BOTS: [&str; 2] = ["thepastaclaw", "coderabbitai"];

/// Handles no policy may name besides the bots: two people, and machines.
const EXCLUDED: [&str; 4] = ["strophy", "silvanassss", "copilot", "dependabot"];

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

/// One repository's policy (`policies/<name>.json`) with the fields the
/// dashboard reads. Only [`Policy::parse`] makes one, so every `Policy` is
/// one the engine's rules accept.
#[derive(Debug, Clone)]
pub struct Policy {
    repository: String,
    target_branches: Vec<String>,
    fallback: Roster,
    areas: Vec<Area>,
    bot_authors: Vec<String>,
    max_active_prs: u32,
}

impl Policy {
    /// Read a policy file's text and accept it only when it passes the
    /// engine's checks.
    pub fn parse(text: &str) -> Result<Policy> {
        let value: Value = serde_json::from_str(text).context("not JSON")?;
        validate_policy(&value)?;
        // Validation has checked the shape of every field read below.
        let typed: Typed = serde_json::from_value(value).context("reading a validated policy")?;
        Ok(Policy {
            repository: typed.repository,
            target_branches: typed.target_branches,
            fallback: typed.fallback,
            areas: typed
                .areas
                .into_iter()
                .map(|area| Area {
                    id: area.id,
                    paths: area.paths,
                    owners: area.owners,
                    reviewers: area.reviewers,
                    unresolved: !area.unresolved.is_empty(),
                })
                .collect(),
            bot_authors: typed.bot_authors,
            max_active_prs: typed.max_active_prs,
        })
    }

    /// Whether PRs into `branch` are under this policy, by the engine's
    /// `governs`: a target is a branch name or a pattern in the form GitHub
    /// branch rules use, where `*` matches any run of characters except `/`.
    /// A PR with no known base is never governed.
    pub fn governs(&self, branch: &str) -> bool {
        !branch.is_empty()
            && self
                .target_branches
                .iter()
                .any(|target| target_matches(target, branch))
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

/// The fields of a policy `validate_policy` accepted.
#[derive(Deserialize)]
struct Typed {
    repository: String,
    target_branches: Vec<String>,
    fallback: Roster,
    areas: Vec<TypedArea>,
    #[serde(default)]
    bot_authors: Vec<String>,
    max_active_prs: u32,
}

#[derive(Deserialize)]
struct TypedArea {
    id: String,
    paths: Vec<String>,
    owners: Vec<String>,
    reviewers: Vec<String>,
    #[serde(default)]
    unresolved: Vec<String>,
}

/// Whether `target` matches the whole of `branch`, `*` standing for any run
/// of characters except `/`. Since `*` never matches `/`, the two match
/// exactly when they have as many `/`-separated segments and each segment
/// matches its counterpart.
fn target_matches(target: &str, branch: &str) -> bool {
    let targets: Vec<&str> = target.split('/').collect();
    let branches: Vec<&str> = branch.split('/').collect();
    targets.len() == branches.len()
        && targets
            .iter()
            .zip(&branches)
            .all(|(target, branch)| segment_matches(target, branch))
}

/// Whether a `/`-free `pattern` matches the whole of `text`, `*` standing for
/// any run of characters. The usual greedy match: on a mismatch, the last
/// `*` takes one more character, which keeps it linear in practice and
/// quadratic at worst.
fn segment_matches(pattern: &str, text: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();
    let (mut p, mut t) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while t < text.len() {
        if p < pattern.len() && pattern[p] == '*' {
            star = Some((p, t));
            p += 1;
        } else if p < pattern.len() && pattern[p] == text[t] {
            p += 1;
            t += 1;
        } else if let Some((star_p, star_t)) = star {
            star = Some((star_p, star_t + 1));
            p = star_p + 1;
            t = star_t + 1;
        } else {
            return false;
        }
    }
    pattern[p..].iter().all(|&c| c == '*')
}

/// `validate_policy(policy)` of `pr_review/policy.py`, without the check of
/// each area's directory in a checkout: the first problem the engine would
/// find, with its message. The board refuses at least what the engine
/// refuses.
fn validate_policy(policy: &Value) -> Result<()> {
    const KEYS: [&str; 6] = [
        "version",
        "repository",
        "fallback",
        "max_active_prs",
        "target_branches",
        "areas",
    ];
    const OPTIONAL: [&str; 3] = ["required_bots", "bot_timeouts", "bot_authors"];
    let allowed: Vec<&str> = KEYS.iter().chain(&OPTIONAL).copied().collect();
    let policy = fields(policy, &allowed, &KEYS)?;
    let no_handles = Value::Array(Vec::new());
    let bot_authors = policy.get("bot_authors").unwrap_or(&no_handles);
    handles(bot_authors, false)?;

    if let Some(timeouts) = policy.get("bot_timeouts").filter(|t| !t.is_null()) {
        let both = ["nudge_after_hours", "waive_after_hours"];
        let timeouts = fields(timeouts, &both, &both)?;
        if !timeouts.values().all(|hours| is_int(hours, 1..=168)) {
            bail!("bot_timeouts must be whole hours between 1 and 168");
        }
    }

    if let Some(bots) = policy.get("required_bots") {
        let accepted = bots.as_array().is_some_and(|bots| {
            let mut seen = HashSet::new();
            bots.iter().all(|bot| {
                bot.as_str()
                    .is_some_and(|bot| REVIEW_BOTS.contains(&bot) && seen.insert(bot))
            })
        });
        if !accepted {
            bail!(
                "required_bots must be a subset of {}",
                REVIEW_BOTS.join(", ")
            );
        }
    }

    if !is_int(&policy["version"], 1..=1) {
        bail!("Unsupported policy version");
    }
    if !policy["repository"].as_str().is_some_and(is_repository) {
        bail!("Invalid repository");
    }
    if !is_int(&policy["max_active_prs"], 5..=5) {
        bail!("Expected five active slots");
    }

    let Some(branches) = texts(&policy["target_branches"]).filter(|b| !b.is_empty()) else {
        bail!("Invalid target branches");
    };
    if branches.iter().collect::<HashSet<_>>().len() != branches.len() {
        bail!("Invalid target branches");
    }
    // `*` is the only pattern `governs` understands; anything else GitHub's
    // rules accept would be read literally and match no branch at all.
    if branches.iter().any(|b| has_branch_syntax(b)) {
        bail!("Target branch patterns support only `*`");
    }

    let fallback = fields(
        &policy["fallback"],
        &["owners", "reviewers"],
        &["owners", "reviewers"],
    )?;
    handles(&fallback["owners"], true)?;
    handles(&fallback["reviewers"], false)?;

    let Some(areas) = policy["areas"].as_array() else {
        bail!("Expected areas list");
    };
    let mut ids: HashSet<&str> = HashSet::new();
    let mut prefixes: Vec<&str> = Vec::new();
    for area in areas {
        let area = fields(
            area,
            &[
                "id",
                "paths",
                "owners",
                "reviewers",
                "unresolved",
                "metadata",
            ],
            &["id", "paths", "owners", "reviewers"],
        )?;
        let id = area["id"]
            .as_str()
            .filter(|id| is_area_id(id) && !ids.contains(id) && *id != "fallback");
        let Some(id) = id else {
            bail!("Invalid or duplicate area id");
        };
        ids.insert(id);
        handles(&area["owners"], false)?;
        if !truthy(&area["owners"]) && !area.get("unresolved").is_some_and(truthy) {
            bail!("Missing owners require an explicit unresolved identity");
        }
        handles(&area["reviewers"], false)?;
        if !handle_set(&area["owners"]).is_disjoint(&handle_set(&area["reviewers"])) {
            bail!("Owner and reviewer roles overlap");
        }
        if area.get("unresolved").is_some_and(|u| texts(u).is_none()) {
            bail!("Invalid unresolved identities");
        }
        if area.get("metadata").is_some_and(|m| !m.is_object()) {
            bail!("Invalid area metadata");
        }
        let Some(paths) = area["paths"].as_array().filter(|p| !p.is_empty()) else {
            bail!("Expected literal directory prefixes");
        };
        for prefix in paths {
            let Some(prefix) = prefix.as_str().filter(|p| is_directory_prefix(p)) else {
                bail!("Invalid literal directory prefix");
            };
            if prefixes
                .iter()
                .any(|other| prefix.starts_with(other) || other.starts_with(prefix))
            {
                bail!("Overlapping directory prefixes");
            }
            prefixes.push(prefix);
        }
    }

    // A machine author needs no attestation because the approval it cannot
    // do without stands in for one. Owning an area it would need neither,
    // and the owner exemption would merge its pull requests unread.
    let machines = handle_set(bot_authors);
    let mut named = handle_set(&fallback["owners"]);
    named.extend(handle_set(&fallback["reviewers"]));
    for area in areas {
        named.extend(handle_set(&area["owners"]));
        named.extend(handle_set(&area["reviewers"]));
    }
    let both: Vec<&str> = machines.intersection(&named).map(String::as_str).collect();
    if !both.is_empty() {
        bail!("A machine author cannot own or review: {}", both.join(", "));
    }
    Ok(())
}

/// `_fields(value, allowed, required)`: an object with every required key
/// and none but the allowed ones.
fn fields<'a>(
    value: &'a Value,
    allowed: &[&str],
    required: &[&str],
) -> Result<&'a Map<String, Value>> {
    match value.as_object() {
        Some(map)
            if map.keys().all(|key| allowed.contains(&key.as_str()))
                && required.iter().all(|key| map.contains_key(*key)) =>
        {
            Ok(map)
        }
        _ => bail!("Unknown or missing policy fields"),
    }
}

/// `_handles(values, nonempty)`: a list of distinct GitHub handles, none of
/// them a bot or an excluded identity.
fn handles(values: &Value, nonempty: bool) -> Result<()> {
    let Some(items) = values
        .as_array()
        .filter(|items| !nonempty || !items.is_empty())
    else {
        bail!("Expected handle list");
    };
    let mut seen = HashSet::new();
    for handle in items {
        let Some(handle) = handle.as_str().filter(|h| is_handle(h)) else {
            bail!("Malformed GitHub handle");
        };
        let folded = handle.to_ascii_lowercase();
        if EXCLUDED.contains(&folded.as_str())
            || BOTS.contains(&folded.as_str())
            || !seen.insert(folded)
        {
            bail!("Duplicate or excluded identity");
        }
    }
    Ok(())
}

/// The lowercased handles of a list `handles` accepted.
fn handle_set(values: &Value) -> BTreeSet<String> {
    values
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_ascii_lowercase)
        .collect()
}

/// A list of strings, each with something besides whitespace.
fn texts(value: &Value) -> Option<Vec<&str>> {
    value
        .as_array()?
        .iter()
        .map(|x| x.as_str().filter(|s| !python_strip(s).is_empty()))
        .collect()
}

/// Python's `str.strip()`: Unicode whitespace plus the four ASCII
/// separators (`\x1c`–`\x1f`) Python also counts as space.
fn python_strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || ('\x1c'..='\x1f').contains(&c))
}

/// `type(value) is int` and within `range`: `true` and `5.0` are not ints.
fn is_int(value: &Value, range: std::ops::RangeInclusive<i64>) -> bool {
    value.as_i64().is_some_and(|n| range.contains(&n))
}

/// Python's truthiness of a JSON value.
fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|n| n != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(items) => !items.is_empty(),
        Value::Object(map) => !map.is_empty(),
    }
}

/// `[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?`, whole.
fn is_handle(handle: &str) -> bool {
    let bytes = handle.as_bytes();
    (1..=39).contains(&bytes.len())
        && bytes
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'-')
        && bytes[0] != b'-'
        && bytes[bytes.len() - 1] != b'-'
}

/// `[\w.-]+/[\w.-]+`, whole, with `\w` read as ASCII: Python's `\w` also
/// takes other Unicode letters and digits, which no GitHub name holds, so
/// this refuses at least what the engine refuses.
fn is_repository(repository: &str) -> bool {
    let part = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'))
    };
    repository
        .split_once('/')
        .is_some_and(|(owner, name)| part(owner) && part(name))
}

/// `[a-z0-9][a-z0-9-]*`, whole.
fn is_area_id(id: &str) -> bool {
    id.bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && id.bytes().next().is_some_and(|b| b != b'-')
}

/// `re.search(r'[?\[\]{}\\]|\*\*', branch)`: branch-rule syntax besides `*`.
fn has_branch_syntax(branch: &str) -> bool {
    branch.contains("**") || branch.contains(['?', '[', ']', '{', '}', '\\'])
}

/// `""` (the whole repository) or one or more `segment/` parts made of
/// `[A-Za-z0-9_.-]`, none of them `.` or `..`.
fn is_directory_prefix(prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    let Some(body) = prefix.strip_suffix('/') else {
        return false;
    };
    body.split('/').all(|segment| {
        !segment.is_empty()
            && segment != "."
            && segment != ".."
            && segment
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'))
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct Roster {
    pub owners: Vec<String>,
    pub reviewers: Vec<String>,
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

    /// Every registered policy: its repository, its file's text and the
    /// board's policy.
    fn registered() -> Vec<(String, String, Policy)> {
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
                (repo, text, policy)
            })
            .collect()
    }

    /// Every registered policy is accepted as written, and each variation
    /// that breaks one of the engine's rules is refused with the engine's
    /// reason, whatever the policy's own shape.
    #[test]
    fn every_registered_policy_is_accepted_or_refused_as_the_engine_does() {
        type Vary = fn(&mut serde_json::Value);
        let variations: [(&str, Vary, Option<&str>); 11] = [
            ("as written", |_| {}, None),
            (
                "unknown field",
                |p| p["owner"] = json!("a"),
                Some("Unknown or missing policy fields"),
            ),
            (
                "no areas",
                |p| {
                    p.as_object_mut().unwrap().remove("areas");
                },
                Some("Unknown or missing policy fields"),
            ),
            ("no area at all", |p| p["areas"] = json!([]), None),
            (
                "question mark",
                |p| {
                    p["target_branches"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!("v?"))
                },
                Some("Target branch patterns support only `*`"),
            ),
            (
                "double star",
                |p| {
                    p["target_branches"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!("**"))
                },
                Some("Target branch patterns support only `*`"),
            ),
            (
                "prefix without slash",
                |p| p["areas"][0]["paths"] = json!(["no-slash"]),
                Some("Invalid literal directory prefix"),
            ),
            (
                "nested prefix",
                |p| {
                    let nested = format!("{}nested/", p["areas"][0]["paths"][0].as_str().unwrap());
                    let last = p["areas"].as_array().unwrap().len() - 1;
                    p["areas"][last]["paths"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!(nested));
                },
                Some("Overlapping directory prefixes"),
            ),
            (
                "four slots",
                |p| p["max_active_prs"] = json!(4),
                Some("Expected five active slots"),
            ),
            (
                "owner as machine",
                |p| p["bot_authors"] = json!([p["fallback"]["owners"][0].clone()]),
                Some("A machine author cannot own or review"),
            ),
            (
                "area named fallback",
                |p| p["areas"][0]["id"] = json!("fallback"),
                Some("Invalid or duplicate area id"),
            ),
        ];
        for (repo, text, _) in registered() {
            let original: serde_json::Value = serde_json::from_str(&text).unwrap();
            for (name, vary, reason) in variations {
                let mut policy = original.clone();
                vary(&mut policy);
                match (Policy::parse(&policy.to_string()), reason) {
                    (Ok(_), None) => {}
                    (Ok(_), Some(reason)) => panic!("{repo}, {name}: accepted, not {reason}"),
                    (Err(e), None) => panic!("{repo}, {name}: refused: {e:#}"),
                    (Err(e), Some(reason)) => {
                        let e = format!("{e:#}");
                        assert!(e.contains(reason), "{repo}, {name}: {e}");
                    }
                }
            }
        }
    }

    /// Each registered target governs exactly the branches its `*` can
    /// stand for: any run without `/`, nothing more, nothing nested.
    #[test]
    fn every_registered_policy_governs_its_targets_and_nothing_nested() {
        for (repo, _, policy) in registered() {
            for target in &policy.target_branches {
                for stand_in in ["", "x", "5.0"] {
                    let branch = target.replace('*', stand_in);
                    assert_eq!(
                        policy.governs(&branch),
                        !branch.is_empty(),
                        "{repo}: {branch:?}"
                    );
                }
                if target.contains('*') {
                    let branch = target.replace('*', "a/b");
                    assert!(!policy.governs(&branch), "{repo}: {branch:?}");
                }
                for branch in [format!("feat/{target}"), format!("{target}/x")] {
                    let branch = branch.replace('*', "x");
                    assert!(!policy.governs(&branch), "{repo}: {branch:?}");
                }
            }
            assert!(!policy.governs(""), "{repo}");
        }
    }
}
