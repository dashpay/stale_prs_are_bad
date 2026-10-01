//! When an open PR entered the stage it is in, from what GitHub and the review
//! engine record about it. Nothing here guesses: where the record does not
//! reach back to the entry, the answer is `None` and the PR shows its age.
//!
//! Two records are read. GitHub's timeline says when a PR became a draft or
//! changed base. The engine's record comment says which state the engine was
//! in each time it wrote it: the engine keeps its record in its newest
//! comment, rewrites it in place when the state changes and posts a fresh
//! comment when the move passes to someone else, and GitHub keeps every
//! revision of every comment.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// The account the review engine writes as. A comment or a revision by anyone
/// else is not the engine's record, whatever it says.
pub const ENGINE_LOGIN: &str = "github-actions[bot]";

/// The engine's record marker (`pr_review.github.STATE_MARKER`).
const RECORD_MARKER: &str = "<!-- platform-pr-review-state-v1";

/// What GitHub and the engine record about one PR's stage changes.
#[derive(Debug, Clone)]
pub struct Evidence {
    pub number: u64,
    pub created_at: DateTime<Utc>,
    pub is_draft: bool,
    pub base: String,
    /// Draft toggles and base changes, oldest first.
    pub events: Vec<PrEvent>,
    /// False when the PR has older events than `events` holds.
    pub events_complete: bool,
    /// The engine account's comments with every revision of each.
    pub comments: Vec<CommentHistory>,
    /// False when the PR has older comments than were read.
    pub comments_complete: bool,
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
    /// Every revision, the original included, oldest first.
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
/// else the PR's opening once every event is known. Walking back from now,
/// each event is undone — a base change by its own `from`, since a renamed
/// branch moves PRs without one — and an event that contradicts the state
/// after it means the timeline cannot be trusted. A reopened PR stands where
/// it does since it was reopened.
pub fn standing_since(
    e: &Evidence,
    governs: impl Fn(&str) -> bool,
) -> Option<(Standing, DateTime<Utc>)> {
    let now = standing(e.is_draft, &e.base, &governs);
    let (mut draft, mut base) = (e.is_draft, e.base.as_str());
    for event in e.events.iter().rev() {
        match event {
            PrEvent::ConvertedToDraft { .. } if draft => draft = false,
            PrEvent::ReadyForReview { .. } if !draft => draft = true,
            PrEvent::BaseChanged { from, .. } => base = from,
            PrEvent::Reopened { at } => return Some((now, *at)),
            PrEvent::ConvertedToDraft { .. } | PrEvent::ReadyForReview { .. } => return None,
        }
        if standing(draft, base, &governs) != now {
            return Some((now, event.at()));
        }
    }
    e.events_complete.then_some((now, e.created_at))
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
/// The engine writes nothing while a PR is closed or off the governed
/// branches, and on its return rewrites the record only if the state changed:
/// a run of records can span time the PR was away. When every record is in
/// the current stage, the first one is taken as the entry: the engine writes
/// its first record when the move first passes to somebody. Where an earlier
/// engine's records were deleted, that understates the time spent in the
/// stage; it never overstates it.
pub fn engine_since(e: &Evidence, state: &str, governs: impl Fn(&str) -> bool) -> Recorded {
    let Some(records) = records(e) else {
        return Recorded::Unreadable;
    };
    let mut entry = None;
    for (at, recorded) in records.iter().rev() {
        // A revision that can no longer be read may have held any state.
        let Some(recorded) = recorded.as_deref() else {
            return Recorded::Unreadable;
        };
        if !same_stage(recorded, state) {
            break;
        }
        entry = Some(*at);
    }
    let Some(entry) = entry else {
        return Recorded::Not;
    };
    match standing_since(e, governs) {
        Some((Standing::Governed, back)) => Recorded::Since(entry.max(back)),
        // The timeline has the PR out of the engine's hands: one of the two
        // reads is stale.
        Some(_) => Recorded::Not,
        None => Recorded::Unreadable,
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

/// Every record the engine wrote on this PR, oldest first, with `None` for a
/// revision of a record comment GitHub no longer shows; `None` altogether
/// when some may not have been read.
fn records(e: &Evidence) -> Option<Vec<(DateTime<Utc>, Option<String>)>> {
    if !e.comments_complete {
        return None;
    }
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
            return None;
        }
        out.extend(
            comment
                .revisions
                .iter()
                .filter(|r| engine(&r.editor))
                .filter_map(|r| match r.body.as_deref() {
                    None => Some((r.at, None)),
                    Some(body) => Some((r.at, Some(record_state(body, e.number)?))),
                }),
        );
    }
    // Stable: revisions written in the same second keep the order written.
    out.sort_by_key(|(at, _)| *at);
    Some(out)
}

/// The engine state a comment body records for PR `number`, read the way the
/// engine reads its own (`pr_review.github.parse_controller_state`): one
/// marker, a JSON object on the marker's line, naming this PR. Anything else —
/// a quote, a malformed record, another PR's — is not a record.
fn record_state(body: &str, number: u64) -> Option<String> {
    #[derive(Deserialize)]
    struct Record {
        number: u64,
        state: String,
    }
    if body.matches(RECORD_MARKER).count() != 1 {
        return None;
    }
    let (_, rest) = body.split_once(RECORD_MARKER)?;
    let line = rest.strip_prefix(' ')?.split(['\r', '\n']).next()?;
    let json = &line[..line.rfind("} -->")? + 1];
    let record: Record = serde_json::from_str(json).ok()?;
    (record.number == number && !record.state.is_empty()).then_some(record.state)
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
        format!(
            "<!-- platform-pr-review-state-v1 {{\"admitted_at\":null,\"context\":\"c\",\
             \"evidence\":\"e\",\"head\":\"h\",\"number\":{number},\"ready_since\":null,\
             \"state\":\"{state}\",\"version\":1}} -->\n\
             <!-- pr-hygiene-diff-v1 {{\"number\":{number}}} -->\n\
             <!-- pr-hygiene:move state={state} sha=h -->\nYour move."
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
            comments_complete: true,
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
        assert_eq!(
            record_state(&record("ready-for-human"), PR).as_deref(),
            Some("ready-for-human")
        );
        // A standing comment of the earlier engine: the record and a pointer.
        let standing = "<!-- platform-pr-review-state-v1 {\"number\":5203,\"state\":\"waiting-bots\"} -->\n\nPR Hygiene: the checklist is in the description.";
        assert_eq!(record_state(standing, PR).as_deref(), Some("waiting-bots"));
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

    #[test]
    fn records_that_may_not_all_have_been_read_say_nothing() {
        let mut e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
        ])]);
        e.comments_complete = false;
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable,
            "older comments unread"
        );

        let mut e = evidence(vec![engine_comment(&[
            (t(2, 0), "waiting-bots"),
            (t(3, 0), "waiting-author"),
        ])]);
        e.comments[0].complete = false;
        assert_eq!(
            engine_since(&e, "waiting-author", governs),
            Recorded::Unreadable,
            "older revisions unread"
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
                },
                PrEvent::BaseChanged {
                    at: t(3, 0),
                    from: "v5.0-dev".into(),
                },
                // Between two branches nobody governs: still not governed.
                PrEvent::BaseChanged {
                    at: t(4, 0),
                    from: "v6.1-dev".into(),
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
            },
            PrEvent::BaseChanged {
                at: t(5, 0),
                from: "feat/x".into(),
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
}
