//! When an open PR entered the stage it is in, from what GitHub and the review
//! engine record about it. Nothing here guesses: where the record does not
//! reach back to the entry, there is no answer and the PR shows its age.
//!
//! Two records are read. GitHub's timeline says when a PR became a draft,
//! changed base or was reopened. The engine's record comment says which state
//! the engine judged the PR in, and on which head, each time it wrote it: the
//! engine keeps its record in its newest comment, rewrites it in place when
//! what it holds changes and posts a fresh comment when the move passes to
//! someone else, and GitHub keeps every revision of every comment.
//!
//! The records start when the engine started governing a repository, and an
//! earlier engine's were deleted when it changed how it comments. For a PR
//! older than its surviving records, the first of them may come after the
//! true entry: its time in the stage is then a lower bound.

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

/// The account the review engine writes as. A comment or a revision by anyone
/// else is not the engine's record, whatever it says.
pub const ENGINE_LOGIN: &str = "github-actions[bot]";

/// The engine's record marker (`pr_review.github.STATE_MARKER`).
pub const RECORD_MARKER: &str = "<!-- platform-pr-review-state-v1";

/// The diff record the engine writes beside it (`pr_review.github.DIFF_MARKER`).
const DIFF_MARKER: &str = "<!-- pr-hygiene-diff-v1";

/// What GitHub and the engine record about a repository's PRs.
#[derive(Debug, Clone, Default)]
pub struct RepoEvidence {
    pub prs: HashMap<u64, Evidence>,
    /// What could not be read, when something could not: the PRs it leaves
    /// without evidence show their age.
    pub error: Option<String>,
}

/// What GitHub and the engine record about one PR's stage changes.
#[derive(Debug, Clone)]
pub struct Evidence {
    pub number: u64,
    pub created_at: DateTime<Utc>,
    pub is_draft: bool,
    pub base: String,
    /// Draft toggles, base changes and reopenings, oldest first.
    pub events: Vec<PrEvent>,
    /// False when the PR has older events than `events` holds.
    pub events_complete: bool,
    /// The engine's record comments with every revision of each.
    pub comments: Vec<CommentHistory>,
    /// How much of the PR's comment list was read.
    pub comments_read: Coverage,
}

/// How much of a PR's comment list was read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    All,
    /// The comments written from then on. Older ones were not read; the
    /// engine writes to its newest comments, so a record in one of them is
    /// older still.
    Since(DateTime<Utc>),
    /// Some comments could not be read at all.
    Partial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrEvent {
    ConvertedToDraft {
        at: DateTime<Utc>,
    },
    ReadyForReview {
        at: DateTime<Utc>,
    },
    /// A new base, set by hand or by GitHub when the old base was merged.
    BaseChanged {
        at: DateTime<Utc>,
        from: String,
        to: String,
    },
    Reopened {
        at: DateTime<Utc>,
    },
}

impl PrEvent {
    fn at(&self) -> DateTime<Utc> {
        match self {
            PrEvent::ConvertedToDraft { at }
            | PrEvent::ReadyForReview { at }
            | PrEvent::BaseChanged { at, .. }
            | PrEvent::Reopened { at } => *at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommentHistory {
    pub author: Option<String>,
    /// Every revision read, the original included, oldest first.
    pub revisions: Vec<Revision>,
    /// False when the comment has older revisions than were read.
    pub complete: bool,
}

#[derive(Debug, Clone)]
pub struct Revision {
    pub at: DateTime<Utc>,
    /// Who wrote this revision: an admin may edit the engine's comment.
    pub editor: Option<String>,
    /// `None` when GitHub no longer shows the revision.
    pub body: Option<String>,
}

/// Where a PR stands before any engine verdict applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    Draft,
    /// Open, against a branch no policy governs.
    Ungoverned,
    Governed,
}

fn standing(draft: bool, base: &str, governs: &impl Fn(&str) -> bool) -> Standing {
    if draft {
        Standing::Draft
    } else if governs(base) {
        Standing::Governed
    } else {
        Standing::Ungoverned
    }
}

/// The PR's standing now and when it began: the newest event that changed it,
/// else the PR's opening once every event is known. Bases are judged by
/// today's policy.
pub fn standing_since(
    e: &Evidence,
    governs: impl Fn(&str) -> bool,
) -> Option<(Standing, DateTime<Utc>)> {
    let now = standing(e.is_draft, &e.base, &governs);
    match last_change(e, &governs, None) {
        Change::At(at) => Some((now, at)),
        Change::None => Some((now, e.created_at)),
        Change::Unknown => None,
    }
}

enum Change {
    At(DateTime<Utc>),
    /// No change among the events considered: with every event read, none
    /// since the PR was opened.
    None,
    Unknown,
}

/// The newest change to the PR's standing among its events newer than
/// `after`, walking back from now and undoing each event in turn. A reopened
/// PR stands where it does since it was reopened. An event that contradicts
/// the state after it, or unread events that may hold the change, leave it
/// unknown. So does a base renamed across the governed line: a rename moves
/// a branch's PRs without an event, so the base an event set may not be the
/// one the PR is on now, and when it moved is not recorded.
fn last_change(
    e: &Evidence,
    governs: &impl Fn(&str) -> bool,
    after: Option<DateTime<Utc>>,
) -> Change {
    let now = standing(e.is_draft, &e.base, governs);
    let (mut draft, mut base) = (e.is_draft, e.base.as_str());
    for event in e.events.iter().rev() {
        if after.is_some_and(|after| event.at() <= after) {
            return Change::None;
        }
        match event {
            PrEvent::Reopened { at } => return Change::At(*at),
            PrEvent::ConvertedToDraft { .. } if draft => draft = false,
            PrEvent::ReadyForReview { .. } if !draft => draft = true,
            PrEvent::ConvertedToDraft { .. } | PrEvent::ReadyForReview { .. } => {
                return Change::Unknown
            }
            PrEvent::BaseChanged { from, to, .. } => {
                if standing(draft, to, governs) != standing(draft, base, governs) {
                    return Change::Unknown;
                }
                base = from;
            }
        }
        if standing(draft, base, governs) != now {
            return Change::At(event.at());
        }
    }
    if e.events_complete {
        Change::None
    } else {
        Change::Unknown
    }
}

/// What the engine's records say about when a PR entered its stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recorded {
    /// It entered the stage then.
    Since(DateTime<Utc>),
    /// The engine has not recorded the PR in this stage (yet).
    Not,
    /// Records that could hold the entry were not read, or can no longer be.
    Unreadable,
}

/// When the PR entered the stage its current engine `state` belongs to: the
/// first record of the newest unbroken run of records in that stage, or the
/// time the PR last came under the engine, if later.
///
/// - The bots review per diff, and the engine times them per diff: their
///   stage starts over at a head whose diff the newest record does not carry.
/// - The engine writes nothing while a PR is closed or off the governed
///   branches, and on its return rewrites the record only if the state
///   changed: a run of records can span time the PR was away.
/// - When every record read is in the current stage and every record was
///   read, the first is the entry: the engine writes its first record when
///   the move first passes to somebody (see the module note on PRs older
///   than their records).
/// - A build wait is its own stage here, whatever the build's outcome: the
///   author's time on a failed build counts from the start of the build
///   wait, overstated by however long the build ran before it failed.
pub fn engine_since(e: &Evidence, state: &str, governs: impl Fn(&str) -> bool) -> Recorded {
    let Some((records, unread_until)) = records(e) else {
        return Recorded::Unreadable;
    };
    let mut entry = None;
    let mut carried: Option<&[String]> = None;
    let mut started = false;
    for record in records.iter().rev() {
        // Before the oldest record read, unread ones may lie in between.
        if unread_until.is_some_and(|until| record.at < until) {
            return Recorded::Unreadable;
        }
        // A revision that can no longer be read may have said anything.
        let Some(said) = &record.said else {
            return Recorded::Unreadable;
        };
        let carried = carried.get_or_insert(said.carried.as_slice());
        let new_diff = state == "waiting-bots" && !carried.contains(&said.head);
        if !same_stage(&said.state, state) || new_diff {
            started = true;
            break;
        }
        entry = Some(record.at);
    }
    if !started && unread_until.is_some() {
        return Recorded::Unreadable;
    }
    let Some(entry) = entry else {
        return Recorded::Not;
    };
    if standing(e.is_draft, &e.base, &governs) != Standing::Governed {
        // The timeline has the PR out of the engine's hands: one of the two
        // reads is stale.
        return Recorded::Not;
    }
    match last_change(e, &governs, Some(entry)) {
        Change::At(back) => Recorded::Since(back),
        Change::None => Recorded::Since(entry),
        Change::Unknown => Recorded::Unreadable,
    }
}

/// Engine states that are one stage for the time spent in it: an author who
/// owes a self-review and one who owes an answer both have the move. A build
/// is its own: the record does not say whether it was running or had failed.
pub fn same_stage(a: &str, b: &str) -> bool {
    fn class(state: &str) -> &str {
        match state {
            "waiting-self-review" => "waiting-author",
            other => other,
        }
    }
    class(a) == class(b)
}

struct Record {
    at: DateTime<Utc>,
    /// `None` for a revision GitHub no longer shows.
    said: Option<Said>,
}

/// What one revision of the engine's record says.
struct Said {
    state: String,
    /// The head it judged.
    head: String,
    /// The heads whose diff is this one's: what the bots said about any of
    /// them still stands. Lowercase, `head` among them.
    carried: Vec<String>,
}

/// Every record the engine wrote on this PR that was read, oldest first, and
/// the time before which more may exist unread; `None` when some could not be
/// read at all.
fn records(e: &Evidence) -> Option<(Vec<Record>, Option<DateTime<Utc>>)> {
    let mut unread_until = match e.comments_read {
        Coverage::All => None,
        Coverage::Since(from) => Some(from),
        Coverage::Partial => return None,
    };
    let engine = |login: &Option<String>| {
        login
            .as_deref()
            .is_some_and(|l| l.eq_ignore_ascii_case(ENGINE_LOGIN))
    };
    let mut out = vec![];
    for comment in e.comments.iter().filter(|c| engine(&c.author)) {
        // The engine's other comments (bot nudges) carry no record and hide
        // none. One whose revision can no longer be read may have.
        let may_record = comment.revisions.iter().any(|r| {
            r.body
                .as_deref()
                .map_or(engine(&r.editor), |b| b.contains(RECORD_MARKER))
        });
        if !may_record {
            continue;
        }
        if !comment.complete {
            // Its unread revisions are older than the oldest one read.
            let oldest = comment.revisions.iter().map(|r| r.at).min()?;
            unread_until = unread_until.max(Some(oldest));
        }
        out.extend(
            comment
                .revisions
                .iter()
                .filter(|r| engine(&r.editor))
                .filter_map(|r| match r.body.as_deref() {
                    None => Some(Record {
                        at: r.at,
                        said: None,
                    }),
                    Some(body) => Some(Record {
                        at: r.at,
                        said: Some(read_record(body, e.number)?),
                    }),
                }),
        );
    }
    // Stable: revisions written in the same second keep the order written.
    out.sort_by_key(|r| r.at);
    Some((out, unread_until))
}

/// The record a comment body holds for PR `number`, read the way the engine
/// reads its own (`pr_review.github.parse_controller_state` and
/// `parse_controller_diff`): one marker, a JSON object on the marker's line,
/// naming this PR. Anything else — a quote, a malformed record, another PR's
/// — is not a record. A diff record that does not read leaves only the head.
fn read_record(body: &str, number: u64) -> Option<Said> {
    #[derive(Deserialize)]
    struct State {
        number: u64,
        state: String,
        head: String,
    }
    #[derive(Deserialize)]
    struct Diff {
        number: u64,
        diff_heads: Vec<String>,
    }
    let record: State = serde_json::from_str(marked_json(body, RECORD_MARKER)?).ok()?;
    if record.number != number || record.state.is_empty() || record.head.is_empty() {
        return None;
    }
    let head = record.head.to_ascii_lowercase();
    let mut carried: Vec<String> = marked_json(body, DIFF_MARKER)
        .and_then(|json| serde_json::from_str::<Diff>(json).ok())
        .filter(|diff| diff.number == number)
        .map(|diff| diff.diff_heads)
        .unwrap_or_default()
        .iter()
        .map(|h| h.to_ascii_lowercase())
        .collect();
    if !carried.contains(&head) {
        carried.push(head.clone());
    }
    Some(Said {
        state: record.state,
        head,
        carried,
    })
}

/// The JSON object on the line of a marker that appears exactly once.
fn marked_json<'a>(body: &'a str, marker: &str) -> Option<&'a str> {
    if body.matches(marker).count() != 1 {
        return None;
    }
    let (_, rest) = body.split_once(marker)?;
    let line = rest.strip_prefix(' ')?.split(['\r', '\n']).next()?;
    Some(&line[..line.rfind("} -->")? + 1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    const PR: u64 = 5203;

    fn t(day: u32, hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 9, day, hour, 0, 0).unwrap()
    }

    fn record(state: &str) -> String {
        record_for(PR, state)
    }

    /// A body as the engine writes it: the record, the diff beside it, then
    /// the words for the person whose move it is.
    fn record_for(number: u64, state: &str) -> String {
        record_on(number, state, "h", &["h"])
    }

    /// A record for `head`, beside a diff record saying which heads share
    /// its diff.
    fn record_on(number: u64, state: &str, head: &str, carried: &[&str]) -> String {
        let carried = serde_json::to_string(carried).unwrap();
        format!(
            "<!-- platform-pr-review-state-v1 {{\"admitted_at\":null,\"context\":\"c\",\
             \"evidence\":\"e\",\"head\":\"{head}\",\"number\":{number},\"ready_since\":null,\
             \"state\":\"{state}\",\"version\":1}} -->\n\
             <!-- pr-hygiene-diff-v1 {{\"diff_heads\":{carried},\"number\":{number}}} -->\n\
             <!-- pr-hygiene:move state={state} sha={head} -->\nYour move."
        )
    }

    fn by(login: &str, at: DateTime<Utc>, body: &str) -> Revision {
        Revision {
            at,
            editor: Some(login.into()),
            body: Some(body.into()),
        }
    }

    /// An engine comment: its revisions, each written by the engine.
    fn engine_comment(revisions: &[(DateTime<Utc>, &str)]) -> CommentHistory {
        CommentHistory {
            author: Some(ENGINE_LOGIN.into()),
            revisions: revisions
                .iter()
                .map(|(at, state)| by(ENGINE_LOGIN, *at, &record(state)))
                .collect(),
            complete: true,
        }
    }

    fn evidence(comments: Vec<CommentHistory>) -> Evidence {
        Evidence {
            number: PR,
            created_at: t(1, 0),
            is_draft: false,
            base: "v5.0-dev".into(),
            events: vec![],
            events_complete: true,
            comments,
            comments_read: Coverage::All,
        }
    }

    #[test]
    fn a_stage_left_and_re_entered_starts_at_the_last_entry() {
        // Self-review, a push sends it back to the bots, then self-review
        // again: the time owed is from the second self-review, not the first.
        let e = evidence(vec![
            engine_comment(&[(t(2, 0), "waiting-self-review"), (t(3, 0), "waiting-bots")]),
            engine_comment(&[(t(4, 0), "waiting-self-review")]),
        ]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(4, 0))
        );
        assert_eq!(
            engine_since(&e, "waiting-bots", governs),
            Recorded::Not,
            "the latest record is not the state asked about: not recorded yet"
        );
    }

    #[test]
    fn rewriting_the_record_in_the_same_stage_keeps_the_entry() {
        // A refresh for a new head or new words rewrites the record in place;
        // a fresh move comment for a new head in the same move is no new stage.
        let e = evidence(vec![
            engine_comment(&[
                (t(2, 0), "waiting-bots"),
                (t(3, 0), "waiting-author"),
                (t(3, 5), "waiting-author"),
            ]),
            engine_comment(&[(t(4, 0), "waiting-author"), (t(5, 0), "waiting-author")]),
        ]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
    }

    #[test]
    fn owing_a_self_review_or_an_answer_is_one_stage() {
        let e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
            (t(4, 0), "waiting-self-review"),
        ])]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(3, 0))
        );
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
    }

    #[test]
    fn a_build_is_its_own_stage_entry() {
        // Whether a recorded build was running or had failed is not in the
        // record, so a failed build's time starts with the build wait.
        let e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-self-review"),
            (t(3, 0), "waiting-build"),
        ])]);
        assert_eq!(
            engine_since(&e, "waiting-build", governs),
            Recorded::Since(t(3, 0))
        );
        assert!(!same_stage("waiting-build", "waiting-author"));
    }

    #[test]
    fn the_first_record_is_the_entry_when_nothing_came_before() {
        let e = evidence(vec![engine_comment(&[(t(2, 0), "waiting-self-review")])]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(2, 0))
        );
    }

    #[test]
    fn without_engine_comments_nothing_is_recorded() {
        let e = evidence(vec![]);
        assert_eq!(engine_since(&e, "waiting-bots", governs), Recorded::Not);
    }

    #[test]
    fn a_marker_in_a_persons_comment_is_ignored() {
        // Anyone can paste the marker; only the engine account writes records.
        let mut forged = engine_comment(&[(t(5, 0), "waiting-self-review")]);
        forged.author = Some("mallory".into());
        let e = evidence(vec![
            engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "waiting-self-review")]),
            forged,
        ]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(3, 0))
        );

        // An engine account named by GraphQL without its `[bot]` suffix is a
        // different account: the fetcher always names the App account in full.
        let mut bare = engine_comment(&[(t(6, 0), "waiting-bots")]);
        bare.author = Some("github-actions".into());
        let e = evidence(vec![
            engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "waiting-self-review")]),
            bare,
        ]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(3, 0))
        );
    }

    #[test]
    fn a_persons_edit_of_the_engine_comment_is_ignored() {
        let mut comment =
            engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "waiting-self-review")]);
        comment
            .revisions
            .push(by("an-admin", t(4, 0), &record("ready-to-merge")));
        let e = evidence(vec![comment]);
        assert_eq!(
            engine_since(&e, "waiting-self-review", governs),
            Recorded::Since(t(3, 0))
        );
        assert_eq!(engine_since(&e, "ready-to-merge", governs), Recorded::Not);
    }

    #[test]
    fn a_malformed_record_is_ignored_not_fatal() {
        let mut comment = engine_comment(&[(t(2, 0), "waiting-bots"), (t(4, 0), "waiting-author")]);
        for body in [
            "<!-- platform-pr-review-state-v1 {\"number\":5203,\"state\": -->".to_string(),
            "<!-- platform-pr-review-state-v1 not json -->".to_string(),
            record("ready-to-merge")
                .replace(" -->\n<!-- pr-hygiene-diff", "\n<!-- pr-hygiene-diff"),
            format!("{}\n{}", record("ready-to-merge"), record("ready-to-merge")),
            record_for(4000, "ready-to-merge"),
        ] {
            comment.revisions.push(by(ENGINE_LOGIN, t(5, 0), &body));
        }
        let e = evidence(vec![comment]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(4, 0))
        );
    }

    #[test]
    fn a_revision_github_no_longer_shows_may_have_held_any_state() {
        let mut comment = engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
            (t(5, 0), "waiting-author"),
        ]);
        comment.revisions.insert(
            2,
            Revision {
                at: t(4, 0),
                editor: Some(ENGINE_LOGIN.into()),
                body: None,
            },
        );
        let e = evidence(vec![comment.clone()]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable
        );
        // Older than the record of the stage before, it hides nothing.
        comment.revisions[2].at = t(1, 12);
        comment.revisions.sort_by_key(|r| r.at);
        let e = evidence(vec![comment]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
    }

    #[test]
    fn records_written_in_the_same_second_keep_their_order() {
        // GitHub times revisions to the second: the later write of a second
        // is the newer state.
        let e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
            (t(3, 0), "waiting-bots"),
        ])]);
        assert_eq!(
            engine_since(&e, "waiting-bots", governs),
            Recorded::Since(t(3, 0))
        );
        assert_eq!(engine_since(&e, "waiting-author", governs), Recorded::Not);
    }

    #[test]
    fn a_record_is_read_as_the_engine_reads_it() {
        let record_state = |body: &str, number| read_record(body, number).map(|r| r.state);
        assert_eq!(
            record_state(&record("ready-for-human"), PR).as_deref(),
            Some("ready-for-human")
        );
        // A standing comment of the earlier engine: the record and a pointer.
        let standing = "<!-- platform-pr-review-state-v1 {\"head\":\"h\",\"number\":5203,\"state\":\"waiting-bots\"} -->\n\nPR Hygiene: the checklist is in the description.";
        assert_eq!(record_state(standing, PR).as_deref(), Some("waiting-bots"));
        // The engine's own schema names the head judged: without it, no record.
        assert_eq!(
            record_state(&standing.replace("\"head\":\"h\",", ""), PR),
            None
        );
        assert_eq!(record_state("no marker at all", PR), None);
        assert_eq!(record_state(&record_for(4000, "waiting-bots"), PR), None);
        assert_eq!(
            record_state(
                "<!-- platform-pr-review-state-v1 {\"number\":5203,\"state\":\"\"} -->",
                PR
            ),
            None
        );
    }

    /// A long-lived PR's record comment can outgrow one page of revisions,
    /// and its comments one page of comments. What was read still answers
    /// whenever the stage's start lies within it.
    #[test]
    fn unread_records_matter_only_where_the_walk_needs_them() {
        let read = || {
            engine_comment(&[
                (t(2, 0), "waiting-bots"),
                (t(3, 0), "waiting-author"),
                (t(4, 0), "waiting-author"),
            ])
        };
        // Older revisions unread: the change into the stage is among those
        // read, so it stands.
        let mut e = evidence(vec![read()]);
        e.comments[0].complete = false;
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
        // The walk reaches the oldest revision read still in the stage.
        let mut e = evidence(vec![read()]);
        e.comments[0].revisions.remove(0);
        e.comments[0].complete = false;
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable
        );
        // The same for comments older than those read.
        let mut e = evidence(vec![read()]);
        e.comments_read = Coverage::Since(t(1, 0));
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
        e.comments_read = Coverage::Since(t(2, 12));
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable,
            "the record before the stage may have unread neighbours"
        );
        e.comments_read = Coverage::Partial;
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable,
            "comments that could not be read at all"
        );

        // Another of the engine account's comments, a nudge with a long
        // history of its own, hides no record.
        let mut e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
        ])]);
        e.comments.push(CommentHistory {
            author: Some(ENGINE_LOGIN.into()),
            revisions: vec![by(ENGINE_LOGIN, t(1, 0), "<!-- pr-hygiene-nudge v1 -->")],
            complete: false,
        });
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
    }

    fn timeline(is_draft: bool, base: &str, events: Vec<PrEvent>) -> Evidence {
        Evidence {
            is_draft,
            base: base.into(),
            events,
            ..evidence(vec![])
        }
    }

    fn governs(branch: &str) -> bool {
        branch == "v5.0-dev"
    }

    #[test]
    fn a_draft_dates_from_its_last_conversion() {
        let e = timeline(
            true,
            "v5.0-dev",
            vec![
                PrEvent::ConvertedToDraft { at: t(2, 0) },
                PrEvent::ReadyForReview { at: t(3, 0) },
                PrEvent::ConvertedToDraft { at: t(4, 0) },
            ],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Draft, t(4, 0)))
        );
    }

    #[test]
    fn a_pr_opened_as_draft_dates_from_its_opening() {
        let e = timeline(true, "v5.0-dev", vec![]);
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Draft, t(1, 0)))
        );
        // Its base changing while a draft does not make it any less a draft.
        let e = timeline(
            true,
            "v5.0-dev",
            vec![PrEvent::BaseChanged {
                at: t(3, 0),
                from: "feat/x".into(),
                to: "v5.0-dev".into(),
            }],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Draft, t(1, 0)))
        );
    }

    #[test]
    fn leaving_the_governed_branches_dates_from_the_base_change() {
        let e = timeline(
            false,
            "v6.0-dev",
            vec![
                PrEvent::BaseChanged {
                    at: t(2, 0),
                    from: "v4.0-dev".into(),
                    to: "v5.0-dev".into(),
                },
                PrEvent::BaseChanged {
                    at: t(3, 0),
                    from: "v5.0-dev".into(),
                    to: "v6.1-dev".into(),
                },
                // Between two branches nobody governs: still not governed.
                PrEvent::BaseChanged {
                    at: t(4, 0),
                    from: "v6.1-dev".into(),
                    to: "v6.0-dev".into(),
                },
            ],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Ungoverned, t(3, 0)))
        );
    }

    #[test]
    fn a_draft_marked_ready_off_the_governed_branches_dates_from_ready() {
        let e = timeline(
            false,
            "feat/x",
            vec![PrEvent::ReadyForReview { at: t(5, 0) }],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Ungoverned, t(5, 0)))
        );
        // Never moved: off the governed branches since it was opened.
        let e = timeline(false, "feat/x", vec![]);
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Ungoverned, t(1, 0)))
        );
    }

    #[test]
    fn an_unread_or_contradictory_timeline_says_nothing() {
        // The fetched events never change the standing and older ones exist.
        let mut e = timeline(
            false,
            "feat/x",
            vec![PrEvent::BaseChanged {
                at: t(3, 0),
                from: "feat/y".into(),
                to: "feat/x".into(),
            }],
        );
        e.events_complete = false;
        assert_eq!(standing_since(&e, governs), None);
        // A change found among the fetched events stands regardless.
        e.events.push(PrEvent::ReadyForReview { at: t(4, 0) });
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Ungoverned, t(4, 0)))
        );
        // A draft whose newest draft event says it was made ready.
        let e = timeline(
            true,
            "v5.0-dev",
            vec![PrEvent::ReadyForReview { at: t(3, 0) }],
        );
        assert_eq!(standing_since(&e, governs), None);
    }

    #[test]
    fn a_reopened_pr_stands_where_it_does_since_it_was_reopened() {
        let e = timeline(
            true,
            "v5.0-dev",
            vec![
                PrEvent::ConvertedToDraft { at: t(2, 0) },
                PrEvent::Reopened { at: t(4, 0) },
            ],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Draft, t(4, 0)))
        );
    }

    /// The engine is silent while a PR is closed or off the governed
    /// branches, and on its return writes nothing if the state is unchanged.
    #[test]
    fn time_away_from_the_engine_is_not_time_in_its_stage() {
        let records = || engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "ready-for-human")]);
        let mut reopened = evidence(vec![records()]);
        reopened.events = vec![PrEvent::Reopened { at: t(6, 0) }];
        assert_eq!(
            engine_since(&reopened, "ready-for-human", governs),
            Recorded::Since(t(6, 0))
        );

        let mut away = evidence(vec![records()]);
        away.events = vec![
            PrEvent::BaseChanged {
                at: t(4, 0),
                from: "v5.0-dev".into(),
                to: "feat/x".into(),
            },
            PrEvent::BaseChanged {
                at: t(5, 0),
                from: "feat/x".into(),
                to: "v5.0-dev".into(),
            },
        ];
        assert_eq!(
            engine_since(&away, "ready-for-human", governs),
            Recorded::Since(t(5, 0))
        );

        // A draft now, by GitHub's later read: the engine's state is stale.
        let mut drafted = evidence(vec![records()]);
        drafted.is_draft = true;
        drafted.events = vec![PrEvent::ConvertedToDraft { at: t(7, 0) }];
        assert_eq!(
            engine_since(&drafted, "ready-for-human", governs),
            Recorded::Not
        );

        // When it last came under the engine cannot be read.
        let mut unread = evidence(vec![records()]);
        unread.events_complete = false;
        assert_eq!(
            engine_since(&unread, "ready-for-human", governs),
            Recorded::Unreadable
        );
    }

    #[test]
    fn a_record_comment_whose_every_revision_is_gone_may_hide_a_state() {
        let gone = CommentHistory {
            author: Some(ENGINE_LOGIN.into()),
            revisions: vec![Revision {
                at: t(4, 0),
                editor: Some(ENGINE_LOGIN.into()),
                body: None,
            }],
            complete: true,
        };
        let e = evidence(vec![
            engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "waiting-author")]),
            gone,
        ]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable
        );
    }

    /// What the bots said about a head stands for every head carrying its
    /// diff (a merge of the base, a rebase): their wait starts over only
    /// with a new diff. Each push re-records the state for the new head.
    #[test]
    fn the_bots_are_timed_per_diff_not_per_run() {
        let comment = |records: &[(DateTime<Utc>, &str, &[&str])]| CommentHistory {
            author: Some(ENGINE_LOGIN.into()),
            revisions: records
                .iter()
                .map(|(at, head, carried)| {
                    by(
                        ENGINE_LOGIN,
                        *at,
                        &record_on(PR, "waiting-bots", head, carried),
                    )
                })
                .collect(),
            complete: true,
        };
        // Pushes a, then b (a new diff), then c (b rebased, same diff).
        let e = evidence(vec![
            engine_comment(&[(t(1, 12), "waiting-author")]),
            comment(&[
                (t(2, 0), "a", &["a"]),
                (t(3, 0), "b", &["b"]),
                (t(4, 0), "c", &["b", "c"]),
            ]),
        ]);
        assert_eq!(
            engine_since(&e, "waiting-bots", governs),
            Recorded::Since(t(3, 0))
        );
        // Heads compare as hex, in any case.
        let e = evidence(vec![comment(&[
            (t(2, 0), "a", &["a"]),
            (t(3, 0), "B", &["B"]),
            (t(4, 0), "c", &["b", "C"]),
        ])]);
        assert_eq!(
            engine_since(&e, "waiting-bots", governs),
            Recorded::Since(t(3, 0))
        );
        // Without a diff record beside it, a record carries only its head.
        let e = evidence(vec![comment(&[(t(2, 0), "a", &["a"])])]);
        let mut bare = e.clone();
        bare.comments[0].revisions[0].body = Some(
            record_on(PR, "waiting-bots", "a", &["a"])
                .replace("<!-- pr-hygiene-diff-v1", "<!-- another-marker"),
        );
        bare.comments[0].revisions.push(by(
            ENGINE_LOGIN,
            t(3, 0),
            &record_on(PR, "waiting-bots", "b", &["b"])
                .replace("<!-- pr-hygiene-diff-v1", "<!-- another-marker"),
        ));
        assert_eq!(
            engine_since(&bare, "waiting-bots", governs),
            Recorded::Since(t(3, 0))
        );
        // Other stages are the author's or the reviewers' whatever the head.
        let e = evidence(vec![
            engine_comment(&[(t(2, 0), "waiting-bots")]),
            CommentHistory {
                author: Some(ENGINE_LOGIN.into()),
                revisions: vec![
                    by(
                        ENGINE_LOGIN,
                        t(3, 0),
                        &record_on(PR, "waiting-author", "a", &["a"]),
                    ),
                    by(
                        ENGINE_LOGIN,
                        t(4, 0),
                        &record_on(PR, "waiting-author", "b", &["b"]),
                    ),
                ],
                complete: true,
            },
        ]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
    }

    #[test]
    fn a_revision_with_no_editor_is_not_the_engines() {
        let mut comment = engine_comment(&[(t(2, 0), "waiting-bots"), (t(3, 0), "waiting-author")]);
        comment.revisions[1].editor = None;
        let e = evidence(vec![comment]);
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Not,
            "only the waiting-bots record is the engine's"
        );
        assert_eq!(
            engine_since(&e, "waiting-bots", governs),
            Recorded::Since(t(2, 0))
        );
    }

    fn moved(at: DateTime<Utc>, from: &str, to: &str) -> PrEvent {
        PrEvent::BaseChanged {
            at,
            from: from.into(),
            to: to.into(),
        }
    }

    /// A renamed branch takes its PRs with it and leaves no event: an event's
    /// date belongs to the base the PR is on now only if that is the base the
    /// event set.
    #[test]
    fn a_renamed_base_does_not_lend_its_date_across_the_governed_line() {
        // Moved onto feat/old-name (not governed) long ago, which has since
        // been renamed v5.0-dev (governed): when it became governed is not
        // recorded.
        let e = timeline(
            false,
            "v5.0-dev",
            vec![moved(t(2, 0), "feat/x", "feat/old-name")],
        );
        assert_eq!(standing_since(&e, governs), None);
        // A rename on the same side of the line changes nothing.
        let e = timeline(
            false,
            "feat/new-name",
            vec![moved(t(2, 0), "v5.0-dev", "feat/old-name")],
        );
        assert_eq!(
            standing_since(&e, governs),
            Some((Standing::Ungoverned, t(2, 0)))
        );
        // The engine's records show the PR governed after the event: a rename
        // before them cannot have cut the stage short.
        let mut e = evidence(vec![engine_comment(&[(t(3, 0), "waiting-author")])]);
        e.events = vec![moved(t(2, 0), "feat/x", "feat/old-name")];
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Since(t(3, 0))
        );
        // After them it could have: moved away, then renamed back in.
        e.events = vec![moved(t(4, 0), "v5.0-dev", "feat/old-name")];
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable
        );
    }
}
