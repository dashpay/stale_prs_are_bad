//! The data behind the interactive dashboard: every open PR with the stage it
//! is in — whose move it is — and for how long, every person and bot with the
//! reviews they owe and the PRs they hold. Everything that carries meaning is
//! decided here; the page only filters, sorts and draws.

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::config::Config;
use crate::model::{PolicyState, ScoredPr};
use crate::policy::Policy;
use crate::renderer::RepoStatus;

/// Bumped on any change a page built for the previous shape would misread.
pub const SCHEMA_VERSION: u32 = 1;

/// The review engine's own review bots (`pr_review.policy.BOTS`). They
/// review PRs and may open their own; a policy may not name them at all.
const ENGINE_REVIEW_BOTS: &[&str] = &["thepastaclaw", "coderabbitai", "coderabbitai[bot]"];

#[derive(Debug, Serialize)]
pub struct Dashboard {
    pub schema_version: u32,
    pub generated_at: DateTime<Utc>,
    pub commit: Option<String>,
    pub repos: Vec<RepoOut>,
    pub prs: Vec<PrOut>,
    pub people: Vec<PersonOut>,
}

#[derive(Debug, Serialize)]
pub struct RepoOut {
    pub repo: String,
    pub engine_state_available: bool,
    pub fetch_error: Option<String>,
}

/// Whose move a PR is waiting on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
    fn key(self) -> &'static str {
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

/// Where `since` comes from. Only an engine timestamp is precise enough to
/// call a PR late; "opened" is shown as context and never coloured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SinceBasis {
    Engine,
    Opened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lateness {
    Ok,
    Late,
    VeryLate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    Human,
    Bot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    AuthorBot,
    ReviewBot,
}

/// An area still waiting for an approval: anyone listed may give it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ask {
    pub area: String,
    pub approvers: Vec<String>,
}

#[derive(Debug, Serialize)]
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
    pub stage: Stage,
    pub engine_state: Option<String>,
    /// What unblocks it, in the engine's words: its first blocker.
    pub next_action: Option<String>,
    pub blockers: Vec<String>,
    pub since: Option<DateTime<Utc>>,
    pub since_basis: Option<SinceBasis>,
    pub lateness: Option<Lateness>,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AreaPart {
    pub area: String,
    pub others: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Owed {
    pub pr: String,
    pub areas: Vec<AreaPart>,
    /// Their own objection is still open: re-review or resolve it.
    pub rereview: bool,
}

#[derive(Debug, Serialize)]
pub struct PersonOut {
    pub login: String,
    pub kind: Kind,
    pub roles: Vec<Role>,
    pub owes: Vec<Owed>,
    pub authored: Vec<String>,
    /// Open, non-draft PRs on governed branches per repository — the five
    /// review slots are per repository.
    pub wip: BTreeMap<String, u32>,
}

pub struct Inputs<'a> {
    pub scored: &'a [ScoredPr],
    pub engine: &'a HashMap<String, HashMap<u64, PolicyState>>,
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
            })
            .collect(),
        prs,
        people,
    }
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
    let (since, since_basis) = match engine_since(stage, state) {
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
        stage,
        engine_state: state.map(|p| p.state.clone()),
        next_action: state.and_then(|p| p.blockers.first().cloned()),
        blockers: state.map(|p| p.blockers.clone()).unwrap_or_default(),
        since,
        since_basis,
        lateness: lateness(stage, since, since_basis, inp),
        asks: state.map(asks).unwrap_or_default(),
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
    let since = engine_since(stage, Some(state));
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
        stage,
        engine_state: Some(state.state.clone()),
        next_action: state.blockers.first().cloned(),
        blockers: state.blockers.clone(),
        since,
        since_basis,
        lateness: lateness(stage, since, since_basis, inp),
        asks: asks(state),
        objectors: state.objectors.clone(),
        areas: state.approvals.iter().map(|a| a.area.clone()).collect(),
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

/// When the current stage started, where the engine records it. Today that is
/// the review cycle's `ready_since`.
fn engine_since(stage: Stage, state: Option<&PolicyState>) -> Option<DateTime<Utc>> {
    if stage != Stage::Review {
        return None;
    }
    state?
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
        });
        key
    };
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
        let governed = !matches!(p.stage, Stage::NotGoverned | Stage::Draft);
        if p.tracked && governed {
            *person.wip.entry(p.repo.clone()).or_default() += 1;
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
        let cfg = Config::default();
        let policies = policies();
        build(&Inputs {
            scored,
            engine,
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

    /// The board's old queue dropped every PR its buckets called stale — and
    /// every platform PR on a development branch other than the default was.
    /// The engine's own reviewer list is the queue now.
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
}
