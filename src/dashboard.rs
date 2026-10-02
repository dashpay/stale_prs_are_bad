//! The data behind the interactive dashboard: every open PR with the stage it
//! is in — whose move it is — and for how long, every person and bot with the
//! reviews they owe and the PRs they hold. Everything that carries meaning is
//! decided here; the page only filters, sorts and draws.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::config::Config;
use crate::model::{PolicyState, ScoredPr};
use crate::policy::Policy;
use crate::renderer::RepoStatus;
use crate::stages::{self, Recorded, RepoEvidence, Standing};

/// Bumped on any change a page built for the previous shape would misread.
pub const SCHEMA_VERSION: u32 = 1;

/// The engine's limit when a policy predates `max_active_prs` (it requires 5).
const DEFAULT_SLOT_LIMIT: u32 = 5;

/// The review engine's own review bots (`pr_review.policy.BOTS`). They
/// review PRs and may open their own; a policy may not name them at all.
const ENGINE_REVIEW_BOTS: &[&str] = &["thepastaclaw", "coderabbitai", "coderabbitai[bot]"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    pub schema_version: u32,
    pub generated_at: DateTime<Utc>,
    pub commit: Option<String>,
    pub repos: Vec<RepoOut>,
    /// Days without an update after which a PR counts as `idle`.
    pub idle_days: i64,
    /// Every stage in display order, with whose move it is and when it is
    /// late, so the page never has to know either.
    pub stages: Vec<StageOut>,
    pub prs: Vec<PrOut>,
    pub people: Vec<PersonOut>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageOut {
    pub stage: Stage,
    pub owner: Owner,
    /// Hours to late and to very late; absent for a stage that is never late.
    pub late_hours: Option<[f64; 2]>,
    /// Whether the time a PR entered this stage can be read from a record.
    /// Where it is not, or a PR's own record does not reach back to its
    /// entry, the PR shows its age instead (`since_basis: opened`) and is
    /// never called late.
    pub entry_recorded: bool,
    /// False for work parked outside the review flow — drafts, PRs off the
    /// governed branches, PRs with no verdict — which no one is asked to move
    /// today; counting them as "the author's move" would bury the real asks.
    pub in_flow: bool,
}

/// Whose move a stage waits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Owner {
    Author,
    Reviewers,
    Bots,
    Maintainers,
    Nobody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoOut {
    pub repo: String,
    pub engine_state_available: bool,
    pub fetch_error: Option<String>,
    /// Why some PRs' entry into their stage could not be read. Those show their
    /// age instead and are not called late, except a review, which keeps the
    /// engine's own start; what GitHub's timeline says still stands. `null`
    /// when nothing failed.
    pub stage_times_error: Option<String>,
    /// Open review slots per author in this repository (the policy's
    /// `max_active_prs`); `wip` above it is over the limit.
    pub slot_limit: u32,
}

/// Whose move a PR is waiting on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stage {
    Draft,
    NotGoverned,
    Bots,
    SelfReview,
    Ci,
    Queued,
    Review,
    Mergeable,
    Blocked,
    Unknown,
}

impl Stage {
    /// Display order: earliest in a PR's life first, then the stages that
    /// leave the flow.
    pub const ALL: [Stage; 10] = [
        Stage::Draft,
        Stage::Bots,
        Stage::SelfReview,
        Stage::Ci,
        Stage::Queued,
        Stage::Review,
        Stage::Mergeable,
        Stage::Blocked,
        Stage::NotGoverned,
        Stage::Unknown,
    ];

    pub fn owner(self) -> Owner {
        match self {
            Stage::Draft | Stage::SelfReview | Stage::Mergeable | Stage::NotGoverned => {
                Owner::Author
            }
            Stage::Review => Owner::Reviewers,
            Stage::Bots => Owner::Bots,
            Stage::Blocked => Owner::Maintainers,
            // Queued waits for the author's other PRs to merge, which is
            // their reviewers' move as much as anyone's; CI and Unknown wait
            // on nobody in particular.
            Stage::Ci | Stage::Queued | Stage::Unknown => Owner::Nobody,
        }
    }

    /// Whether a PR's entry into this stage can be read from a record: GitHub's
    /// timeline for a draft or a base change, the engine's record comment for
    /// the rest. Not for a running build: a re-run on the same head leaves the
    /// record as it was, so the build wait it shows may have ended long ago.
    fn entry_recorded(self) -> bool {
        !matches!(self, Stage::Ci | Stage::Unknown)
    }

    pub fn key(self) -> &'static str {
        match self {
            Stage::Draft => "draft",
            Stage::NotGoverned => "not-governed",
            Stage::Bots => "bots",
            Stage::SelfReview => "self-review",
            Stage::Ci => "ci",
            Stage::Queued => "queued",
            Stage::Review => "review",
            Stage::Mergeable => "mergeable",
            Stage::Blocked => "blocked",
            Stage::Unknown => "unknown",
        }
    }
}

/// Where `since` comes from. Only a recorded entry into the stage is precise
/// enough to call a PR late; "opened" is shown as context and never coloured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SinceBasis {
    /// When the PR entered its stage, as recorded: by the engine's record
    /// comment, or by GitHub's timeline for a draft or a base change.
    Engine,
    /// When the PR was opened: its entry into the stage is not recorded.
    Opened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lateness {
    Ok,
    Late,
    VeryLate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    Human,
    Bot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    AuthorBot,
    ReviewBot,
}

/// An area still waiting for an approval: anyone listed may give it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ask {
    pub area: String,
    pub approvers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrOut {
    /// `owner/name#number`.
    pub key: String,
    pub repo: String,
    pub number: u64,
    pub title: String,
    pub author: Option<String>,
    pub author_kind: Kind,
    pub draft: bool,
    pub base: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    /// Last activity of any kind; drives "close or revive" for idle drafts
    /// and PRs off the governed branches.
    pub updated_at: Option<DateTime<Utc>>,
    /// Untouched for `idle_days`: a candidate to close or revive.
    pub idle: bool,
    pub stage: Stage,
    pub engine_state: Option<String>,
    /// What unblocks it, in the engine's words: its first blocker.
    pub next_action: Option<String>,
    pub blockers: Vec<String>,
    pub since: Option<DateTime<Utc>>,
    pub since_basis: Option<SinceBasis>,
    pub lateness: Option<Lateness>,
    /// Only while the PR waits on review: earlier the engine's approvals
    /// describe areas nobody has been asked about yet.
    pub asks: Vec<Ask>,
    pub objectors: Vec<String>,
    pub areas: Vec<String>,
    pub unresolved_comments: u32,
    pub ci_failing: bool,
    pub merge_conflict: bool,
    pub changes_requested: bool,
    /// False when the PR is known only from the engine's export: the
    /// dashboard's own filters (skip labels, excluded authors, the grace
    /// period for new contributors) left it out, but its reviewers still owe it.
    pub tracked: bool,
}

/// One area of one PR that a person may approve, and who else may instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AreaPart {
    pub area: String,
    pub others: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owed {
    pub pr: String,
    pub areas: Vec<AreaPart>,
    /// Their own objection is still open: re-review or resolve it.
    pub rereview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonOut {
    pub login: String,
    pub kind: Kind,
    pub roles: Vec<Role>,
    pub owes: Vec<Owed>,
    pub authored: Vec<String>,
    /// Open, non-draft PRs on governed branches per repository, whether or
    /// not the board itself tracks them — the engine's five review slots are
    /// counted that way, per repository.
    pub wip: BTreeMap<String, u32>,
    /// Areas this person owns or reviews per repository, from the policies;
    /// `fallback` is the files no area claims.
    pub areas: BTreeMap<String, Vec<String>>,
}

pub struct Inputs<'a> {
    pub scored: &'a [ScoredPr],
    pub engine: &'a HashMap<String, HashMap<u64, PolicyState>>,
    /// What GitHub and the engine record about each PR's stage changes, per
    /// repository and number; a PR missing here shows its age.
    pub evidence: &'a HashMap<String, RepoEvidence>,
    pub policies: &'a HashMap<String, Policy>,
    pub repos: &'a [RepoStatus],
    pub cfg: &'a Config,
    pub now: DateTime<Utc>,
    pub commit: Option<&'a str>,
}

pub fn build(inp: &Inputs<'_>) -> Dashboard {
    let mut prs: Vec<PrOut> = inp.scored.iter().map(|s| tracked_pr(s, inp)).collect();
    let seen: HashSet<String> = prs.iter().map(|p| p.key.clone()).collect();
    for (repo, rows) in inp.engine {
        for (number, state) in rows {
            let key = format!("{repo}#{number}");
            if !seen.contains(&key) {
                prs.push(engine_only_pr(repo, *number, state, inp));
            }
        }
    }
    prs.sort_by(|a, b| a.repo.cmp(&b.repo).then(a.number.cmp(&b.number)));

    let people = people(&prs, inp);
    Dashboard {
        schema_version: SCHEMA_VERSION,
        generated_at: inp.now,
        commit: inp.commit.map(str::to_string),
        repos: inp
            .repos
            .iter()
            .map(|r| RepoOut {
                repo: r.repo.clone(),
                engine_state_available: r.engine_state_available,
                fetch_error: r.fetch_error.clone(),
                stage_times_error: inp.evidence.get(&r.repo).and_then(|e| e.error.clone()),
                slot_limit: inp
                    .policies
                    .get(&r.repo)
                    .map_or(DEFAULT_SLOT_LIMIT, |p| p.max_active_prs),
            })
            .collect(),
        idle_days: inp.cfg.idle_days,
        stages: Stage::ALL
            .iter()
            .map(|&stage| StageOut {
                stage,
                owner: stage.owner(),
                late_hours: inp.cfg.lateness_hours.get(stage.key()).copied(),
                entry_recorded: stage.entry_recorded(),
                in_flow: !matches!(stage, Stage::Draft | Stage::NotGoverned | Stage::Unknown),
            })
            .collect(),
        prs,
        people,
    }
}

/// Refuse a `lateness_hours` key that names no stage, or thresholds out of
/// order: a typo would otherwise leave a stage silently never late.
pub fn validate_lateness(cfg: &Config) -> anyhow::Result<()> {
    for (key, [late, very_late]) in &cfg.lateness_hours {
        if !Stage::ALL.iter().any(|s| s.key() == key) {
            anyhow::bail!("lateness_hours: {key:?} is not a stage");
        }
        if !(late.is_finite() && very_late.is_finite() && 0.0 < *late && late <= very_late) {
            anyhow::bail!("lateness_hours.{key}: expected finite hours, 0 < late <= very late");
        }
    }
    Ok(())
}

fn engine_available(inp: &Inputs<'_>, repo: &str) -> bool {
    inp.engine.contains_key(repo)
}

fn tracked_pr(s: &ScoredPr, inp: &Inputs<'_>) -> PrOut {
    let raw = &s.pr.raw;
    let governed = inp
        .policies
        .get(&raw.repo)
        .is_some_and(|p| p.governs(&raw.base_ref));
    let state = s.policy_state.as_ref();
    let stage = stage(
        raw.is_draft,
        governed,
        engine_available(inp, &raw.repo),
        state,
    );
    let (since, since_basis) = match entered(stage, state, &raw.repo, raw.number, inp) {
        Some(t) => (Some(t), Some(SinceBasis::Engine)),
        None => (Some(raw.created_at), Some(SinceBasis::Opened)),
    };
    PrOut {
        key: format!("{}#{}", raw.repo, raw.number),
        repo: raw.repo.clone(),
        number: raw.number,
        title: raw.title.clone(),
        author: raw.author.clone(),
        author_kind: raw
            .author
            .as_deref()
            .map_or(Kind::Human, |a| kind(a, inp.policies)),
        draft: raw.is_draft,
        base: Some(raw.base_ref.clone()),
        created_at: Some(raw.created_at),
        updated_at: Some(raw.updated_at),
        idle: (inp.now - raw.updated_at).num_days() >= inp.cfg.idle_days,
        stage,
        engine_state: state.map(|p| p.state.clone()),
        next_action: state.and_then(|p| p.blockers.first().cloned()),
        blockers: state.map(|p| p.blockers.clone()).unwrap_or_default(),
        since,
        since_basis,
        lateness: lateness(stage, since, since_basis, inp),
        asks: state
            .filter(|_| stage == Stage::Review)
            .map(asks)
            .unwrap_or_default(),
        objectors: state.map(|p| p.objectors.clone()).unwrap_or_default(),
        areas: s.areas.clone(),
        unresolved_comments: s.unresolved_total,
        ci_failing: s.pr.ci_failing,
        merge_conflict: s.pr.has_merge_conflict,
        changes_requested: s.pr.changes_requested,
        tracked: true,
    }
}

/// A PR the engine governs but the dashboard's own filters left out. The
/// export only lists open PRs on governed branches.
fn engine_only_pr(repo: &str, number: u64, state: &PolicyState, inp: &Inputs<'_>) -> PrOut {
    let draft = state.state == "draft";
    let stage = stage(draft, true, true, Some(state));
    let since = entered(stage, Some(state), repo, number, inp);
    let since_basis = since.map(|_| SinceBasis::Engine);
    let author = (!state.author.is_empty()).then(|| state.author.clone());
    PrOut {
        key: format!("{repo}#{number}"),
        repo: repo.to_string(),
        number,
        title: state.title.clone(),
        author_kind: author
            .as_deref()
            .map_or(Kind::Human, |a| kind(a, inp.policies)),
        author,
        draft,
        base: None,
        created_at: None,
        updated_at: None,
        idle: false,
        stage,
        engine_state: Some(state.state.clone()),
        next_action: state.blockers.first().cloned(),
        blockers: state.blockers.clone(),
        since,
        since_basis,
        lateness: lateness(stage, since, since_basis, inp),
        asks: if stage == Stage::Review {
            asks(state)
        } else {
            vec![]
        },
        objectors: state.objectors.clone(),
        areas: state.areas.clone(),
        unresolved_comments: 0,
        ci_failing: false,
        merge_conflict: false,
        changes_requested: false,
        tracked: false,
    }
}

/// Collapse a PR's facts into whose move it is. A draft is its author's
/// whatever the branch; a PR off the governed branches has no engine verdict
/// to read; a governed PR with no verdict is Unknown, never guessed.
pub fn stage(
    draft: bool,
    governed: bool,
    engine_available: bool,
    state: Option<&PolicyState>,
) -> Stage {
    if draft {
        return Stage::Draft;
    }
    if !governed {
        return Stage::NotGoverned;
    }
    let Some(state) = state.filter(|_| engine_available) else {
        return Stage::Unknown;
    };
    match state.state.as_str() {
        "draft" => Stage::Draft,
        "waiting-bots" => Stage::Bots,
        "waiting-author" | "waiting-self-review" => Stage::SelfReview,
        // A failed build is the author's to fix; a running one is nobody's.
        "waiting-build" if state.build() == Some("failed") => Stage::SelfReview,
        "waiting-build" => Stage::Ci,
        "too-many-open-prs" => Stage::Queued,
        "ready-for-human" => Stage::Review,
        "ready-to-merge" => Stage::Mergeable,
        "configuration-error" => Stage::Blocked,
        _ => Stage::Unknown,
    }
}

/// When the PR entered its current stage, where that is recorded. A draft or
/// a PR off the governed branches dates from GitHub's timeline; an engine
/// stage from the engine's records, checked against the PR's state in the
/// export. Where the records could not be read, only a review cycle has a
/// start: the engine's own `ready_since`. Where they were read and do not
/// show the PR in review, that `ready_since` is the time of the export, not
/// of the review.
fn entered(
    stage: Stage,
    state: Option<&PolicyState>,
    repo: &str,
    number: u64,
    inp: &Inputs<'_>,
) -> Option<DateTime<Utc>> {
    if !stage.entry_recorded() {
        return None;
    }
    let evidence = inp.evidence.get(repo).and_then(|e| e.prs.get(&number));
    let policy = inp.policies.get(repo);
    let governs = |base: &str| policy.is_some_and(|p| p.governs(base));
    let standing = match stage {
        Stage::Draft => Standing::Draft,
        Stage::NotGoverned => Standing::Ungoverned,
        _ => {
            let state = state?;
            return match evidence.map(|e| stages::engine_since(e, &state.state, governs)) {
                Some(Recorded::Since(at)) => Some(at),
                Some(Recorded::Not) => None,
                None | Some(Recorded::Unreadable) if stage == Stage::Review => ready_since(state),
                None | Some(Recorded::Unreadable) => None,
            };
        }
    };
    // The timeline's own idea of where the PR stands must be the stage's: a
    // PR changed between the two reads has no entry worth showing.
    stages::standing_since(evidence?, governs)
        .filter(|(now, _)| *now == standing)
        .map(|(_, at)| at)
}

fn ready_since(state: &PolicyState) -> Option<DateTime<Utc>> {
    state
        .ready_since
        .as_deref()
        .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
        .map(|t| t.with_timezone(&Utc))
}

fn lateness(
    stage: Stage,
    since: Option<DateTime<Utc>>,
    basis: Option<SinceBasis>,
    inp: &Inputs<'_>,
) -> Option<Lateness> {
    if basis != Some(SinceBasis::Engine) {
        return None;
    }
    let [late, very_late] = *inp.cfg.lateness_hours.get(stage.key())?;
    let hours = (inp.now - since?).num_seconds() as f64 / 3600.0;
    Some(if hours >= very_late {
        Lateness::VeryLate
    } else if hours >= late {
        Lateness::Late
    } else {
        Lateness::Ok
    })
}

/// The areas still waiting for an approval, in the engine's order.
fn asks(state: &PolicyState) -> Vec<Ask> {
    state
        .approvals
        .iter()
        .filter(|a| !a.owned && a.approved_by.is_empty())
        .map(|a| Ask {
            area: a.area.clone(),
            approvers: a.approvers.clone(),
        })
        .collect()
}

/// What one person is asked for on a PR ready for review — the engine's
/// `your_part` rule: each unapproved area they may approve, with who else may
/// instead, and any re-review their own objection is waiting on.
pub fn part(state: &PolicyState, login: &str) -> Option<(Vec<AreaPart>, bool)> {
    if state.state != "ready-for-human"
        || !state
            .reviewers
            .iter()
            .any(|r| r.eq_ignore_ascii_case(login))
    {
        return None;
    }
    let areas = asks(state)
        .into_iter()
        .filter(|a| a.approvers.iter().any(|x| x.eq_ignore_ascii_case(login)))
        .map(|a| AreaPart {
            others: a
                .approvers
                .into_iter()
                .filter(|x| !x.eq_ignore_ascii_case(login))
                .collect(),
            area: a.area,
        })
        .collect();
    let rereview = state
        .objectors
        .iter()
        .any(|o| o.eq_ignore_ascii_case(login));
    Some((areas, rereview))
}

/// Human or bot. A bot is a GitHub App account (`name[bot]`), an account a
/// policy names under `bot_authors`, or one of the engine's review bots.
pub fn kind(login: &str, policies: &HashMap<String, Policy>) -> Kind {
    let named = policies
        .values()
        .flat_map(|p| &p.bot_authors)
        .any(|b| b.eq_ignore_ascii_case(login));
    if login.ends_with("[bot]") || named || is_review_bot(login) {
        Kind::Bot
    } else {
        Kind::Human
    }
}

fn is_review_bot(login: &str) -> bool {
    ENGINE_REVIEW_BOTS
        .iter()
        .any(|b| b.eq_ignore_ascii_case(login))
}

fn people(prs: &[PrOut], inp: &Inputs<'_>) -> Vec<PersonOut> {
    // Keyed case-insensitively; the first spelling seen is displayed.
    let mut by_login: BTreeMap<String, PersonOut> = BTreeMap::new();
    let mut entry = |login: &str| -> String {
        let key = login.to_ascii_lowercase();
        by_login.entry(key.clone()).or_insert_with(|| PersonOut {
            login: login.to_string(),
            kind: kind(login, inp.policies),
            roles: vec![],
            owes: vec![],
            authored: vec![],
            wip: BTreeMap::new(),
            areas: BTreeMap::new(),
        });
        key
    };
    // Everyone a policy names is a person on the board, with their areas,
    // even with nothing owed or open today.
    let mut rostered: Vec<(String, String, String)> = vec![];
    for (repo, policy) in inp.policies {
        let rosters = std::iter::once((
            "fallback",
            &policy.fallback.owners,
            &policy.fallback.reviewers,
        ))
        .chain(
            policy
                .areas
                .iter()
                .map(|a| (a.id.as_str(), &a.owners, &a.reviewers)),
        );
        for (area, owners, reviewers) in rosters {
            for login in owners.iter().chain(reviewers) {
                rostered.push((entry(login), repo.clone(), area.to_string()));
            }
        }
    }
    let mut authored: Vec<(String, &PrOut)> = vec![];
    for p in prs {
        if let Some(author) = &p.author {
            authored.push((entry(author), p));
        }
    }
    let mut owed: Vec<(String, Owed)> = vec![];
    for (repo, rows) in inp.engine {
        for (number, state) in rows {
            for reviewer in &state.reviewers {
                if let Some((areas, rereview)) = part(state, reviewer) {
                    owed.push((
                        entry(reviewer),
                        Owed {
                            pr: format!("{repo}#{number}"),
                            areas,
                            rereview,
                        },
                    ));
                }
            }
        }
    }
    for (key, p) in authored {
        let person = by_login.get_mut(&key).expect("just inserted");
        person.authored.push(p.key.clone());
        if !matches!(p.stage, Stage::NotGoverned | Stage::Draft) {
            *person.wip.entry(p.repo.clone()).or_default() += 1;
        }
    }
    for (key, repo, area) in rostered {
        let areas = by_login
            .get_mut(&key)
            .expect("just inserted")
            .areas
            .entry(repo)
            .or_default();
        if !areas.contains(&area) {
            areas.push(area);
        }
    }
    for (key, o) in owed {
        by_login.get_mut(&key).expect("just inserted").owes.push(o);
    }
    let mut out: Vec<PersonOut> = by_login.into_values().collect();
    for person in &mut out {
        if person.kind == Kind::Bot {
            if !person.authored.is_empty() {
                person.roles.push(Role::AuthorBot);
            }
            if is_review_bot(&person.login) {
                person.roles.push(Role::ReviewBot);
            }
        }
        person.owes.sort_by(|a, b| a.pr.cmp(&b.pr));
        person.authored.sort();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        AnalyzedPr, AreaApproval, BySeverity, BySource, ChecklistItem, Mergeable, RawPr,
    };
    use crate::stages::Evidence;
    use chrono::TimeZone;

    const REPO: &str = "dashpay/platform";

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 1, 12, 0, 0).unwrap()
    }

    fn policies() -> HashMap<String, Policy> {
        HashMap::from([(
            REPO.to_string(),
            Policy {
                repository: REPO.into(),
                target_branches: vec!["v*-dev".into()],
                bot_authors: vec!["Claudius-Maginificent".into()],
                ..Policy::default()
            },
        )])
    }

    fn scored(number: u64, author: &str, base: &str, state: Option<PolicyState>) -> ScoredPr {
        ScoredPr {
            pr: AnalyzedPr {
                raw: RawPr {
                    repo: REPO.into(),
                    number,
                    title: format!("PR {number}"),
                    url: format!("https://github.com/{REPO}/pull/{number}"),
                    author: Some(author.into()),
                    created_at: now() - chrono::Duration::days(30),
                    updated_at: now(),
                    is_draft: false,
                    mergeable: Mergeable::Mergeable,
                    labels: vec![],
                    last_commit: None,
                    reviews: vec![],
                    threads: vec![],
                    requested_reviewers: vec![],
                    base_ref: base.into(),
                    changed_files: vec![],
                    changed_files_truncated: false,
                },
                unresolved_threads: vec![],
                days_since_author_push: 0.0,
                days_since_last_reviewer_activity: 0.0,
                needs_author_action: false,
                has_merge_conflict: false,
                changes_requested: false,
                ci_failing: false,
                is_deferred: false,
                is_stale: base != "v5.0-dev",
                stale_reasons: vec![],
            },
            score: 0.0,
            oldest_thread_age_days: 0.0,
            unresolved_by_severity: BySeverity::default(),
            unresolved_by_source: BySource::default(),
            unresolved_total: 0,
            routed_reviewers: vec![],
            areas: vec![],
            unresolved_areas: vec![],
            routing_unavailable: None,
            policy_state: state,
        }
    }

    fn engine_state(state: &str) -> PolicyState {
        PolicyState {
            state: state.into(),
            ..PolicyState::default()
        }
    }

    fn ready(reviewers: &[&str], approvals: Vec<AreaApproval>, ready_since: &str) -> PolicyState {
        PolicyState {
            state: "ready-for-human".into(),
            reviewers: reviewers.iter().map(|s| s.to_string()).collect(),
            approvals,
            ready_since: Some(ready_since.into()),
            title: "From the engine".into(),
            author: "newcomer".into(),
            ..PolicyState::default()
        }
    }

    fn area(id: &str, approvers: &[&str], approved_by: &[&str]) -> AreaApproval {
        AreaApproval {
            area: id.into(),
            approvers: approvers.iter().map(|s| s.to_string()).collect(),
            approved_by: approved_by.iter().map(|s| s.to_string()).collect(),
            owned: false,
        }
    }

    fn board(
        scored: &[ScoredPr],
        engine: &HashMap<String, HashMap<u64, PolicyState>>,
    ) -> Dashboard {
        board_with(scored, engine, vec![])
    }

    fn board_with(
        scored: &[ScoredPr],
        engine: &HashMap<String, HashMap<u64, PolicyState>>,
        evidence: Vec<Evidence>,
    ) -> Dashboard {
        let cfg = Config::default();
        let policies = policies();
        let evidence = HashMap::from([(
            REPO.to_string(),
            RepoEvidence {
                prs: evidence.into_iter().map(|e| (e.number, e)).collect(),
                error: None,
            },
        )]);
        build(&Inputs {
            scored,
            engine,
            evidence: &evidence,
            policies: &policies,
            repos: &[],
            cfg: &cfg,
            now: now(),
            commit: None,
        })
    }

    fn person<'a>(d: &'a Dashboard, login: &str) -> &'a PersonOut {
        d.people
            .iter()
            .find(|p| p.login.eq_ignore_ascii_case(login))
            .unwrap_or_else(|| panic!("no {login}"))
    }

    #[test]
    fn every_engine_state_has_an_owner() {
        let failed = PolicyState {
            state: "waiting-build".into(),
            checklist: vec![ChecklistItem {
                item: "build".into(),
                state: Some(serde_json::json!("failed")),
            }],
            ..PolicyState::default()
        };
        let cases = [
            ("draft", Stage::Draft),
            ("waiting-bots", Stage::Bots),
            ("waiting-author", Stage::SelfReview),
            ("waiting-self-review", Stage::SelfReview),
            ("waiting-build", Stage::Ci),
            ("too-many-open-prs", Stage::Queued),
            ("ready-for-human", Stage::Review),
            ("ready-to-merge", Stage::Mergeable),
            ("configuration-error", Stage::Blocked),
            ("a-state-from-a-newer-engine", Stage::Unknown),
        ];
        for (state, expected) in cases {
            assert_eq!(
                stage(false, true, true, Some(&engine_state(state))),
                expected,
                "{state}"
            );
        }
        assert_eq!(
            stage(false, true, true, Some(&failed)),
            Stage::SelfReview,
            "a failed build is the author's move"
        );
        assert_eq!(stage(true, false, false, None), Stage::Draft);
        assert_eq!(stage(false, false, true, None), Stage::NotGoverned);
        assert_eq!(
            stage(false, true, true, None),
            Stage::Unknown,
            "governed, export present, PR missing from it: the engine could not read it"
        );
        assert_eq!(
            stage(false, true, false, Some(&engine_state("ready-for-human"))),
            Stage::Unknown,
            "no export for the repository: nothing is guessed"
        );
    }

    /// What a reviewer owes is the engine's own reviewer list, whatever bucket
    /// the board puts the PR in — a PR on a development branch other than
    /// the default is "stale" to the board and still waiting for review.
    #[test]
    fn a_reviewer_owes_what_the_engine_asks_of_them_with_their_areas() {
        let state = ready(
            &["bob", "carol", "dave"],
            vec![
                area("dpp", &["bob", "carol"], &[]),
                area("drive", &["dave"], &["dave"]),
                area("fallback", &["Bob"], &[]),
            ],
            "2026-09-30T12:00:00Z",
        );
        let scored = vec![scored(4801, "alice", "v4.3-dev", Some(state.clone()))];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(4801, state)]))]);
        let d = board(&scored, &engine);
        let bob = person(&d, "bob");
        assert_eq!(
            bob.owes,
            vec![Owed {
                pr: format!("{REPO}#4801"),
                areas: vec![
                    AreaPart {
                        area: "dpp".into(),
                        others: vec!["carol".into()]
                    },
                    AreaPart {
                        area: "fallback".into(),
                        others: vec![]
                    },
                ],
                rereview: false,
            }]
        );
        assert!(
            person(&d, "dave").owes[0].areas.is_empty(),
            "an area they already approved asks nothing of them"
        );
        let pr = &d.prs[0];
        assert_eq!(pr.stage, Stage::Review);
        assert_eq!(pr.asks.len(), 2, "drive is approved: only dpp and fallback");
    }

    #[test]
    fn a_pr_the_board_filtered_out_still_reaches_its_reviewers() {
        // A new contributor's PR is held back from the board's own analysis
        // for a grace period; its reviewers still owe it.
        let state = ready(
            &["bob"],
            vec![area("dpp", &["bob"], &[])],
            "2026-09-30T12:00:00Z",
        );
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(77, state)]))]);
        let d = board(&[], &engine);
        assert_eq!(person(&d, "bob").owes[0].pr, format!("{REPO}#77"));
        let pr = &d.prs[0];
        assert!(!pr.tracked);
        assert_eq!(pr.title, "From the engine");
        assert_eq!(pr.author.as_deref(), Some("newcomer"));
        assert_eq!(pr.stage, Stage::Review);
    }

    #[test]
    fn an_objector_owes_a_re_review() {
        let mut state = ready(&["bob"], vec![], "2026-09-30T12:00:00Z");
        state.objectors = vec!["Bob".into()];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(1, state)]))]);
        let d = board(&[], &engine);
        assert!(person(&d, "bob").owes[0].rereview);
    }

    #[test]
    fn bots_are_never_people() {
        let scored = vec![
            scored(1, "thepastaclaw", "v5.0-dev", None),
            scored(2, "PastaPastaPasta", "v5.0-dev", None),
            scored(3, "Claudius-Maginificent", "v5.0-dev", None),
            scored(4, "dependabot[bot]", "v5.0-dev", None),
        ];
        let engine = HashMap::from([(REPO.to_string(), HashMap::new())]);
        let d = board(&scored, &engine);
        let claw = person(&d, "thepastaclaw");
        assert_eq!(claw.kind, Kind::Bot);
        assert_eq!(claw.roles, vec![Role::AuthorBot, Role::ReviewBot]);
        assert_eq!(person(&d, "PastaPastaPasta").kind, Kind::Human);
        assert_eq!(person(&d, "Claudius-Maginificent").kind, Kind::Bot);
        assert_eq!(person(&d, "dependabot[bot]").kind, Kind::Bot);
        assert_eq!(d.prs[0].author_kind, Kind::Bot);
        assert_eq!(d.prs[1].author_kind, Kind::Human);
    }

    #[test]
    fn lateness_comes_only_from_a_recorded_stage_start() {
        let two_days = ready(&["bob"], vec![], "2026-09-29T12:00:00Z");
        let four_days = ready(&["bob"], vec![], "2026-09-27T12:00:00Z");
        let scored = vec![
            scored(1, "alice", "v5.0-dev", Some(two_days.clone())),
            scored(2, "alice", "v5.0-dev", Some(four_days.clone())),
            // Opened thirty days ago, waiting on bots since we don't know when:
            // its age is no measure of how late the bots are.
            scored(3, "alice", "v5.0-dev", Some(engine_state("waiting-bots"))),
        ];
        let engine = HashMap::from([(
            REPO.to_string(),
            HashMap::from([
                (1, two_days),
                (2, four_days),
                (3, engine_state("waiting-bots")),
            ]),
        )]);
        let d = board(&scored, &engine);
        assert_eq!(d.prs[0].lateness, Some(Lateness::Late));
        assert_eq!(d.prs[1].lateness, Some(Lateness::VeryLate));
        assert_eq!(d.prs[2].since_basis, Some(SinceBasis::Opened));
        assert_eq!(d.prs[2].lateness, None);
    }

    #[test]
    fn wip_counts_open_governed_non_draft_prs_per_repository() {
        let mut draft = scored(3, "alice", "v5.0-dev", Some(engine_state("draft")));
        draft.pr.raw.is_draft = true;
        let scored = vec![
            scored(1, "alice", "v5.0-dev", Some(engine_state("waiting-bots"))),
            scored(2, "alice", "feat/x", None),
            draft,
        ];
        let engine = HashMap::from([(
            REPO.to_string(),
            HashMap::from([
                (1, engine_state("waiting-bots")),
                (3, engine_state("draft")),
            ]),
        )]);
        let d = board(&scored, &engine);
        let alice = person(&d, "alice");
        assert_eq!(alice.wip.get(REPO), Some(&1));
        assert_eq!(alice.authored.len(), 3);
    }

    #[test]
    fn serializes_with_schema_version_and_kebab_case() {
        let d = board(&[], &HashMap::new());
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["schema_version"], SCHEMA_VERSION);
        assert_eq!(
            serde_json::to_value(Stage::SelfReview).unwrap(),
            "self-review"
        );
        assert_eq!(serde_json::to_value(Role::ReviewBot).unwrap(), "review-bot");
    }

    #[test]
    fn only_areas_still_waiting_are_asked_and_a_stranded_one_is_shown() {
        let mut owned = area("swift-sdk", &["alice"], &[]);
        owned.owned = true;
        let state = ready(
            &["bob"],
            vec![
                owned,
                area("dpp", &["bob"], &["bob"]),
                area("drive", &[], &[]),
                area("fallback", &["bob"], &[]),
            ],
            "2026-09-30T12:00:00Z",
        );
        let scored = vec![scored(1, "alice", "v5.0-dev", Some(state.clone()))];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(1, state)]))]);
        let d = board(&scored, &engine);
        let asked: Vec<&str> = d.prs[0].asks.iter().map(|a| a.area.as_str()).collect();
        // The author owns swift-sdk and dpp is approved: neither asks anyone.
        // Nobody may approve drive, which must show rather than read as met.
        assert_eq!(asked, vec!["drive", "fallback"]);
        assert!(d.prs[0].asks[0].approvers.is_empty());
        let bob = &person(&d, "bob").owes[0];
        assert_eq!(bob.areas.len(), 1, "bob is asked for fallback only");
        assert_eq!(bob.areas[0].area, "fallback");
    }

    #[test]
    fn nothing_is_owed_or_asked_before_the_pr_is_ready_for_review() {
        // The engine fills approvals from the self-review gate on; listing them
        // earlier would ask reviewers for something not theirs to do yet.
        let mut state = ready(
            &["bob"],
            vec![area("dpp", &["bob"], &[])],
            "2026-09-30T12:00:00Z",
        );
        state.state = "waiting-build".into();
        let scored = vec![scored(1, "alice", "v5.0-dev", Some(state.clone()))];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(1, state)]))]);
        let d = board(&scored, &engine);
        assert!(d.prs[0].asks.is_empty());
        assert!(d.people.iter().all(|p| p.owes.is_empty()));
    }

    #[test]
    fn wip_counts_every_governed_pr_the_engine_counts() {
        // The board leaves out skip-labelled PRs and new contributors' PRs;
        // the engine's five slots count them, so the warning must too.
        let mut untracked = engine_state("too-many-open-prs");
        untracked.author = "alice".into();
        let mut draft = engine_state("draft");
        draft.author = "alice".into();
        let engine = HashMap::from([(
            REPO.to_string(),
            HashMap::from([(9, untracked), (10, draft)]),
        )]);
        let d = board(&[], &engine);
        assert_eq!(person(&d, "alice").wip.get(REPO), Some(&1));
    }

    #[test]
    fn late_starts_at_the_threshold() {
        let day = ready(&["bob"], vec![], "2026-09-30T12:00:00Z");
        let three_days = ready(&["bob"], vec![], "2026-09-28T12:00:00Z");
        let scored = vec![
            scored(1, "alice", "v5.0-dev", Some(day.clone())),
            scored(2, "alice", "v5.0-dev", Some(three_days.clone())),
        ];
        let engine =
            HashMap::from([(REPO.to_string(), HashMap::from([(1, day), (2, three_days)]))]);
        let d = board(&scored, &engine);
        assert_eq!(d.prs[0].lateness, Some(Lateness::Late));
        assert_eq!(d.prs[1].lateness, Some(Lateness::VeryLate));
    }

    #[test]
    fn a_policy_names_its_bots_in_any_case() {
        assert_eq!(kind("claudius-maginificent", &policies()), Kind::Bot);
    }

    /// Mirrors the engine's own list; a review bot added there and not here
    /// would show as a person.
    #[test]
    fn engine_review_bots_match_the_engine() {
        let engine =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/pr_review/policy.py"))
                .unwrap();
        let line = engine
            .lines()
            .find(|l| l.starts_with("BOTS = {"))
            .expect("BOTS in pr_review/policy.py");
        let mut theirs: Vec<&str> = line.split('\'').skip(1).step_by(2).collect();
        theirs.sort();
        let mut ours = ENGINE_REVIEW_BOTS.to_vec();
        ours.sort();
        assert_eq!(theirs, ours);
    }

    #[test]
    fn everyone_a_policy_names_is_on_the_board_with_their_areas() {
        let mut policies = policies();
        let p = policies.get_mut(REPO).unwrap();
        p.fallback.owners = vec!["QuantumExplorer".into()];
        p.areas = vec![crate::policy::Area {
            id: "dpp".into(),
            owners: vec!["quantumexplorer".into()],
            reviewers: vec!["shumkov".into()],
            ..Default::default()
        }];
        let cfg = Config::default();
        let d = build(&Inputs {
            scored: &[],
            engine: &HashMap::new(),
            evidence: &HashMap::new(),
            policies: &policies,
            repos: &[],
            cfg: &cfg,
            now: now(),
            commit: None,
        });
        let qe = person(&d, "QuantumExplorer");
        assert_eq!(
            qe.areas.get(REPO),
            Some(&vec!["fallback".to_string(), "dpp".to_string()])
        );
        assert_eq!(
            person(&d, "shumkov").areas.get(REPO),
            Some(&vec!["dpp".to_string()])
        );
    }

    #[test]
    fn the_stage_legend_says_whose_move_and_what_is_measurable() {
        let d = board(&[], &HashMap::new());
        assert_eq!(d.stages.len(), Stage::ALL.len());
        let review = d.stages.iter().find(|s| s.stage == Stage::Review).unwrap();
        assert_eq!(review.owner, Owner::Reviewers);
        assert_eq!(review.late_hours, Some([24.0, 72.0]));
        assert!(review.entry_recorded);
        let queued = d.stages.iter().find(|s| s.stage == Stage::Queued).unwrap();
        assert_eq!(queued.late_hours, None, "queued is never late per PR");
        assert!(queued.entry_recorded, "never late, still timed");
        let unrecorded: Vec<Stage> = d
            .stages
            .iter()
            .filter(|s| !s.entry_recorded)
            .map(|s| s.stage)
            .collect();
        assert_eq!(unrecorded, vec![Stage::Ci, Stage::Unknown]);
        let parked: Vec<Stage> = d
            .stages
            .iter()
            .filter(|s| !s.in_flow)
            .map(|s| s.stage)
            .collect();
        assert_eq!(
            parked,
            vec![Stage::Draft, Stage::NotGoverned, Stage::Unknown]
        );
    }

    #[test]
    fn a_misspelled_or_inverted_threshold_is_refused() {
        let mut cfg = Config::default();
        assert!(validate_lateness(&cfg).is_ok());
        cfg.lateness_hours
            .insert("self_review".into(), [72.0, 168.0]);
        assert!(validate_lateness(&cfg).is_err());
        cfg.lateness_hours.remove("self_review");
        cfg.lateness_hours.insert("review".into(), [72.0, 24.0]);
        assert!(validate_lateness(&cfg).is_err());
        // An infinite threshold is never reached: the stage would never be late.
        for hours in [
            [24.0, f64::INFINITY],
            [f64::INFINITY, f64::INFINITY],
            [f64::NAN, 72.0],
        ] {
            cfg.lateness_hours.insert("review".into(), hours);
            assert!(validate_lateness(&cfg).is_err(), "{hours:?}");
        }
    }

    /// What GitHub and the engine recorded about PR `number`: opened thirty
    /// days ago against `v5.0-dev`, with the engine's records written the
    /// given numbers of hours before now.
    fn recorded(number: u64, records: &[(i64, &str)]) -> Evidence {
        Evidence {
            number,
            created_at: now() - chrono::Duration::days(30),
            is_draft: false,
            base: "v5.0-dev".into(),
            events: vec![],
            events_complete: true,
            comments: vec![stages::CommentHistory {
                author: Some(stages::ENGINE_LOGIN.into()),
                revisions: records
                    .iter()
                    .map(|(hours_ago, state)| stages::Revision {
                        at: now() - chrono::Duration::hours(*hours_ago),
                        editor: Some(stages::ENGINE_LOGIN.into()),
                        body: Some(format!(
                            "<!-- platform-pr-review-state-v1 \
                             {{\"head\":\"h\",\"number\":{number},\"state\":\"{state}\"}} -->\n\
                             Your move."
                        )),
                    })
                    .collect(),
                complete: true,
            }],
            comments_read: stages::Coverage::All,
        }
    }

    #[test]
    fn every_engine_stage_is_timed_from_the_engines_records() {
        let cases = [
            (1, "waiting-bots", 30, Some(Lateness::Late)),
            (2, "waiting-author", 200, Some(Lateness::VeryLate)),
            (3, "too-many-open-prs", 500, None),
        ];
        let engine: HashMap<u64, PolicyState> = cases
            .iter()
            .map(|(n, state, _, _)| (*n, engine_state(state)))
            .collect();
        let scored: Vec<ScoredPr> = cases
            .iter()
            .map(|(n, state, _, _)| scored(*n, "alice", "v5.0-dev", Some(engine_state(state))))
            .collect();
        // Each was in another stage before it entered its own.
        let evidence = cases
            .iter()
            .map(|(n, state, hours, _)| {
                recorded(*n, &[(hours + 10, "ready-for-human"), (*hours, state)])
            })
            .collect();
        let d = board_with(
            &scored,
            &HashMap::from([(REPO.to_string(), engine)]),
            evidence,
        );
        for (pr, (_, state, hours, late)) in d.prs.iter().zip(cases) {
            assert_eq!(pr.since_basis, Some(SinceBasis::Engine), "{state}");
            assert_eq!(pr.since, Some(now() - chrono::Duration::hours(hours)));
            // Queued is timed but never late per PR.
            assert_eq!(pr.lateness, late, "{state}");
        }
    }

    #[test]
    fn a_state_the_engine_has_not_recorded_shows_the_pr_age() {
        // The export says ready for review; the engine's last record is the
        // self-review before it. When review began is not known — the
        // export's `ready_since` for such a PR is the time of the export.
        let state = ready(&["bob"], vec![], "2026-10-01T11:59:00Z");
        let scored = vec![scored(1, "alice", "v5.0-dev", Some(state.clone()))];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(1, state)]))]);
        let d = board_with(
            &scored,
            &engine,
            vec![recorded(1, &[(48, "waiting-self-review")])],
        );
        assert_eq!(d.prs[0].stage, Stage::Review);
        assert_eq!(d.prs[0].since_basis, Some(SinceBasis::Opened));
        assert_eq!(d.prs[0].since, Some(now() - chrono::Duration::days(30)));
        assert_eq!(d.prs[0].lateness, None);
    }

    #[test]
    fn a_review_whose_records_could_not_all_be_read_keeps_the_engines_start() {
        // Unread records may well show the review starting when the engine
        // says; they are no evidence against it.
        let state = ready(&["bob"], vec![], "2026-09-29T12:00:00Z");
        let scored = vec![scored(1, "alice", "v5.0-dev", Some(state.clone()))];
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(1, state)]))]);
        let mut unread = recorded(1, &[(48, "ready-for-human")]);
        unread.comments_read = stages::Coverage::Partial;
        let d = board_with(&scored, &engine, vec![unread]);
        assert_eq!(d.prs[0].since_basis, Some(SinceBasis::Engine));
        assert_eq!(d.prs[0].since, Some(now() - chrono::Duration::days(2)));
        assert_eq!(d.prs[0].lateness, Some(Lateness::Late));
    }

    #[test]
    fn a_running_build_is_never_timed_from_the_build_wait() {
        // Failed two days ago and re-run just now on the same head: the record
        // still says the build wait began two days ago. A failed build is the
        // author's move from the start of that wait.
        let records = &[(60, "waiting-author"), (48, "waiting-build")];
        let failed = PolicyState {
            checklist: vec![ChecklistItem {
                item: "build".into(),
                state: Some(serde_json::json!("failed")),
            }],
            ..engine_state("waiting-build")
        };
        let engine = HashMap::from([(
            REPO.to_string(),
            HashMap::from([(1, engine_state("waiting-build")), (2, failed.clone())]),
        )]);
        let d = board_with(
            &[
                scored(1, "alice", "v5.0-dev", Some(engine_state("waiting-build"))),
                scored(2, "alice", "v5.0-dev", Some(failed)),
            ],
            &engine,
            vec![recorded(1, records), recorded(2, records)],
        );
        assert_eq!(d.prs[0].stage, Stage::Ci);
        assert_eq!(d.prs[0].since_basis, Some(SinceBasis::Opened));
        assert_eq!(d.prs[0].lateness, None);
        assert_eq!(d.prs[1].stage, Stage::SelfReview);
        assert_eq!(d.prs[1].since, Some(now() - chrono::Duration::hours(48)));
        assert_eq!(d.prs[1].since_basis, Some(SinceBasis::Engine));
    }

    #[test]
    fn a_pr_the_board_filtered_out_is_timed_too() {
        let state = engine_state("waiting-bots");
        let engine = HashMap::from([(REPO.to_string(), HashMap::from([(77, state)]))]);
        let d = board_with(
            &[],
            &engine,
            vec![recorded(
                77,
                &[(60, "waiting-author"), (30, "waiting-bots")],
            )],
        );
        let pr = &d.prs[0];
        assert!(!pr.tracked);
        assert_eq!(pr.since, Some(now() - chrono::Duration::hours(30)));
        assert_eq!(pr.lateness, Some(Lateness::Late));
    }

    #[test]
    fn drafts_and_prs_off_the_governed_branches_date_from_github_events() {
        let at = |hours: i64| now() - chrono::Duration::hours(hours);
        let mut draft = scored(1, "alice", "v5.0-dev", None);
        draft.pr.raw.is_draft = true;
        let mut draft_evidence = recorded(1, &[]);
        draft_evidence.is_draft = true;
        draft_evidence.events = vec![
            stages::PrEvent::ReadyForReview { at: at(100) },
            stages::PrEvent::ConvertedToDraft { at: at(50) },
        ];
        let mut off_evidence = recorded(2, &[]);
        off_evidence.base = "feat/x".into();
        off_evidence.events = vec![stages::PrEvent::BaseChanged {
            at: at(20),
            from: "v5.0-dev".into(),
            to: "feat/x".into(),
        }];
        // GitHub's timeline says this one is a draft; the PR list said it was
        // not. One of the two reads is stale: no entry is shown.
        let mut stale = recorded(3, &[]);
        stale.is_draft = true;
        stale.base = "feat/x".into();
        let engine = HashMap::from([(REPO.to_string(), HashMap::new())]);
        let d = board_with(
            &[
                draft,
                scored(2, "alice", "feat/x", None),
                scored(3, "alice", "feat/x", None),
            ],
            &engine,
            vec![draft_evidence, off_evidence, stale],
        );
        assert_eq!(d.prs[0].stage, Stage::Draft);
        assert_eq!(d.prs[0].since, Some(at(50)));
        assert_eq!(d.prs[0].since_basis, Some(SinceBasis::Engine));
        assert_eq!(d.prs[0].lateness, None, "no draft is late");
        assert_eq!(d.prs[1].stage, Stage::NotGoverned);
        assert_eq!(d.prs[1].since, Some(at(20)));
        assert_eq!(d.prs[1].since_basis, Some(SinceBasis::Engine));
        assert_eq!(d.prs[2].stage, Stage::NotGoverned);
        assert_eq!(d.prs[2].since_basis, Some(SinceBasis::Opened));
    }

    /// The states timed as one stage must be one stage on the board, or a PR
    /// could carry its entry into a stage it is no longer in.
    #[test]
    fn states_timed_as_one_stage_are_one_stage() {
        let states = [
            "draft",
            "waiting-bots",
            "waiting-author",
            "waiting-self-review",
            "waiting-build",
            "too-many-open-prs",
            "ready-for-human",
            "ready-to-merge",
            "configuration-error",
        ];
        for a in states {
            for b in states {
                if stages::same_stage(a, b) {
                    assert_eq!(
                        stage(false, true, true, Some(&engine_state(a))),
                        stage(false, true, true, Some(&engine_state(b))),
                        "{a} and {b}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_null_in_the_engine_export_reads_as_empty() {
        let state: PolicyState = serde_json::from_value(serde_json::json!({
            "state": "ready-for-human", "title": null, "author": null, "areas": null,
            "approvals": [{"area": "dpp", "approvers": null, "approved_by": null}],
            "objectors": null, "ready_since": null, "checklist": null
        }))
        .unwrap();
        assert_eq!(state.title, "");
        assert!(state.approvals[0].approvers.is_empty());
    }
}
