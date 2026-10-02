//! Each signed-in person's own speed, and the inputs it is computed from.
//!
//! The inputs are recorded at ingest, in the snapshot's own transaction,
//! keyed by GitHub user id, only from repositories the snapshot read, and
//! never under the id of anyone who opted out:
//!
//! - `asks`: a person asked to review a PR — while it waits on review, as an
//!   approver of an area still unapproved or as an objector — from when to
//!   when, and how it ended: `answered` (their own decisive review),
//!   `covered` (others' reviews did without theirs: a co-approver cleared
//!   their area, and the PR waits on others, is mergeable or merged) or
//!   `left` (back to its author, closed unmerged, or gone).
//! - `turns`: a PR's time in its author's stage (self-review, answering an
//!   objection, a failed build). A run broken only by bots or a build is one
//!   turn, timed by its stretches in the author's stage alone.
//! - `merges`: when a merged PR was first ready for review, and merged.
//! - `reviews`: decisive reviews, by reviewer id.
//! - `pr_authors`: each PR's author id.
//!
//! **Ids.** The engine asks for logins. A login is tied to an account id
//! only by an event GitHub reports the id with, in the same snapshot: the
//! person's own decisive review on that PR, or a PR they authored. Within
//! one snapshot GitHub names each login's current account, so the tie is
//! exact; across snapshots it would not be — a login can be renamed and
//! then registered by someone else — so no tie is carried from one snapshot
//! to the next. Until one is made, an open ask is known by its login only,
//! is part of no one's speed, and is deleted if it ends that way. The one
//! thing a later tie carries across a rename is an ask still open on the
//! login: whoever holds it when the tie is made takes the ask from its
//! start. Opt-outs keep ids only, so an opted-out person's open asks may be
//! held by login too, until a tie to their id deletes them.
//!
//! **Timing.** Snapshots are minutes apart, so most changes are dated by
//! the engine's record of when a PR entered its stage, and otherwise to
//! within one snapshot. An interval is counted but not timed when its start
//! is not known: it was already running when first seen (a repository read
//! for the first time, or again after a gap) without a recorded start, or
//! it began before the last look, which already showed the PR in that
//! stage, without being seen. One already open when a snapshot fails to
//! read its repository is not timed either: what happened in the gap was
//! not seen. After the gap the repository is seen afresh, as on a first
//! read. A PR whose stage the engine did not give (`unknown`) is not seen
//! either: what is open on it stays open, as it was.

use crate::store::{parse_ts, ts};
use crate::view;
use anyhow::Context;
use chrono::{DateTime, Datelike, Utc};
use pr_hygiene::dashboard::{
    ClosedPr, Dashboard, DecisiveReview, PrOut, SinceBasis, Stage, Verdict,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};

/// The most months a speed answer lists, the current one included.
pub const MAX_MONTHS: i32 = 12;

/// Each median rolls over this many months: the month and the two before.
const WINDOW: i32 = 3;

/// Fewer values than this make no median: one or two PRs say more about
/// those PRs than about the person.
const MIN_N: usize = 3;

// --- recording ---

/// GitHub's ids fit SQLite's integers; one that did not would be no id.
fn sql_id(id: u64) -> Option<i64> {
    i64::try_from(id).ok()
}

fn opted_out(tx: &Transaction<'_>) -> anyhow::Result<HashSet<i64>> {
    let mut stmt = tx.prepare_cached("SELECT user_id FROM opt_outs")?;
    let ids = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;
    Ok(ids)
}

/// The PR's recorded entry into its stage, if the entry is recorded.
fn recorded(p: &PrOut) -> Option<DateTime<Utc>> {
    p.since
        .filter(|_| p.since_basis == Some(SinceBasis::Engine))
}

/// Who a PR asks for a review now, by lowercase login: while it waits on
/// review, the approvers of each area still unapproved, and everyone whose
/// objection stands.
fn asked(p: &PrOut) -> BTreeSet<String> {
    if p.stage != Stage::Review {
        return BTreeSet::new();
    }
    p.asks
        .iter()
        .flat_map(|a| &a.approvers)
        .chain(&p.objectors)
        .map(|login| login.to_ascii_lowercase())
        .collect()
}

fn verdict(v: Verdict) -> &'static str {
    match v {
        Verdict::Approved => "approved",
        Verdict::ChangesRequested => "changes-requested",
        Verdict::Dismissed => "dismissed",
    }
}

/// How the last look at a repository bears on what this one shows.
#[derive(Debug, Clone, Copy)]
enum Look {
    /// Seen as if for the first time: never read for speed before, or not
    /// read in some snapshot since it last was.
    Fresh,
    /// Read last in the snapshot generated at this time, and nothing missed
    /// since.
    After(DateTime<Utc>),
}

/// Logins this snapshot ties to account ids through the PRs' authors, open
/// and closed. A login seen with two ids (renamed mid-run) is tied to none.
struct Authors(HashMap<String, Option<i64>>);

impl Authors {
    fn of(d: &Dashboard) -> Self {
        let mut ids: HashMap<String, Option<i64>> = HashMap::new();
        let open = d
            .prs
            .iter()
            .filter_map(|p| Some((p.author.as_deref()?, p.author_id?)));
        let closed = d
            .closed
            .iter()
            .filter_map(|c| Some((c.author.as_deref()?, c.author_id?)));
        for (login, id) in open.chain(closed) {
            let id = sql_id(id);
            ids.entry(login.to_ascii_lowercase())
                .and_modify(|seen| {
                    if *seen != id {
                        *seen = None;
                    }
                })
                .or_insert(id);
        }
        Self(ids)
    }

    /// The account `login` names in this snapshot: by its decisive reviews
    /// on the PR (`reviews`), else by a PR it authored.
    fn resolve(&self, login: &str, reviews: &[DecisiveReview]) -> Option<i64> {
        let mut by_review = reviews
            .iter()
            .filter(|r| r.reviewer.eq_ignore_ascii_case(login))
            .map(|r| sql_id(r.reviewer_id));
        if let Some(first) = by_review.next() {
            return if by_review.all(|id| id == first) {
                first
            } else {
                None
            };
        }
        self.0.get(&login.to_ascii_lowercase()).copied().flatten()
    }
}

/// The closed PRs of the repositories whose open and closed PRs were both
/// read, by repository and number.
fn closed_read(d: &Dashboard) -> HashMap<(&str, u64), &ClosedPr> {
    let read: HashSet<&str> = d
        .repos
        .iter()
        .filter(|r| r.fetch_error.is_none() && r.closed_error.is_none())
        .map(|r| r.repo.as_str())
        .collect();
    d.closed
        .iter()
        .filter(|c| read.contains(c.repo.as_str()))
        .map(|c| ((c.repo.as_str(), c.number), c))
        .collect()
}

/// What a local import added from a snapshot's closed PRs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClosedRecorded {
    pub merges: usize,
    pub reviews: usize,
}

/// Record what snapshot `d` says about people's speed, in the transaction
/// that stores it. Repositories it read move their open asks and turns on;
/// those it could not read leave theirs open and no longer timed; those it
/// no longer lists end theirs.
pub(crate) fn record(tx: &Transaction<'_>, d: &Dashboard) -> anyhow::Result<()> {
    let opted_out = opted_out(tx)?;
    let closed = closed_read(d);
    closed_facts(tx, closed.values().copied(), &opted_out, None)?;
    let authors = Authors::of(d);
    let mut listed = HashSet::new();
    for r in &d.repos {
        listed.insert(r.repo.as_str());
        if !view::is_good(r) {
            lose_sight(tx, &r.repo)?;
            continue;
        }
        let prs: HashMap<u64, &PrOut> = d
            .prs
            .iter()
            .filter(|p| p.repo == r.repo)
            .map(|p| (p.number, p))
            .collect();
        open_facts(tx, d.generated_at, prs.values().copied(), &opted_out)?;
        let pass = Pass {
            repo: &r.repo,
            look: look(tx, &r.repo)?,
            at: d.generated_at,
            closed: closed
                .iter()
                .filter(|((repo, _), _)| *repo == r.repo)
                .map(|((_, number), c)| (*number, *c))
                .collect(),
            prs,
            closed_read: r.closed_error.is_none(),
            authors: &authors,
            opted_out: &opted_out,
        };
        pass.asks(tx)?;
        pass.turns(tx)?;
        tx.execute(
            "INSERT INTO speed_reads (repo, read_at, stale) VALUES (?1, ?2, 0)
             ON CONFLICT (repo) DO UPDATE SET read_at = excluded.read_at, stale = 0",
            params![r.repo, ts(d.generated_at)],
        )?;
    }
    // A repository that left the registry is no longer read: what was open
    // on it ends here, as if each PR had gone.
    let tracked: Vec<String> = tx
        .prepare_cached("SELECT repo FROM speed_reads")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<_, _>>()?;
    for repo in tracked.iter().filter(|r| !listed.contains(r.as_str())) {
        let pass = Pass {
            repo,
            look: Look::Fresh,
            at: d.generated_at,
            prs: HashMap::new(),
            closed: HashMap::new(),
            closed_read: true,
            authors: &authors,
            opted_out: &opted_out,
        };
        pass.asks(tx)?;
        pass.turns(tx)?;
        tx.execute("DELETE FROM speed_reads WHERE repo = ?1", [repo])?;
    }
    Ok(())
}

/// Record the merges and reviews of `d`'s closed PRs alone: for a
/// snapshot that is not newer than the latest, such as a year's backfill
/// imported after posting began. These are dated facts no later snapshot
/// changes; nothing open is moved by an older snapshot. Those dated before
/// `kept_from` are past the retention already, and not recorded.
pub(crate) fn record_closed(
    tx: &Transaction<'_>,
    d: &Dashboard,
    kept_from: DateTime<Utc>,
) -> anyhow::Result<ClosedRecorded> {
    let opted_out = opted_out(tx)?;
    closed_facts(
        tx,
        closed_read(d).into_values(),
        &opted_out,
        Some(kept_from),
    )
}

fn author_seen(
    tx: &Transaction<'_>,
    repo: &str,
    number: i64,
    author: i64,
    at: DateTime<Utc>,
) -> anyhow::Result<()> {
    // A PR's author never changes; when it was last seen does, and dates
    // the row for retention.
    tx.prepare_cached(
        "INSERT INTO pr_authors (repo, number, author_id, seen_at) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (repo, number) DO UPDATE SET seen_at = max(seen_at, excluded.seen_at)",
    )?
    .execute(params![repo, number, author, ts(at)])?;
    Ok(())
}

fn reviews_seen(
    tx: &Transaction<'_>,
    repo: &str,
    number: i64,
    reviews: &[DecisiveReview],
    opted_out: &HashSet<i64>,
) -> anyhow::Result<usize> {
    let mut insert = tx.prepare_cached(
        "INSERT OR IGNORE INTO reviews (repo, number, reviewer_id, state, at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    let mut added = 0;
    for r in reviews {
        let Some(reviewer) = sql_id(r.reviewer_id).filter(|id| !opted_out.contains(id)) else {
            continue;
        };
        added += insert.execute(params![repo, number, reviewer, verdict(r.state), ts(r.at)])?;
    }
    Ok(added)
}

fn open_facts<'a>(
    tx: &Transaction<'_>,
    at: DateTime<Utc>,
    prs: impl Iterator<Item = &'a PrOut>,
    opted_out: &HashSet<i64>,
) -> anyhow::Result<()> {
    for p in prs {
        let number = i64::try_from(p.number).context("PR number out of range")?;
        if let Some(author) = p
            .author_id
            .and_then(sql_id)
            .filter(|id| !opted_out.contains(id))
        {
            author_seen(tx, &p.repo, number, author, at)?;
        }
        reviews_seen(tx, &p.repo, number, &p.reviews, opted_out)?;
    }
    Ok(())
}

/// A closed PR's merge and reviews. Its author is not kept apart from the
/// merge: nothing open needs it.
fn closed_facts<'a>(
    tx: &Transaction<'_>,
    closed: impl Iterator<Item = &'a ClosedPr>,
    opted_out: &HashSet<i64>,
    kept_from: Option<DateTime<Utc>>,
) -> anyhow::Result<ClosedRecorded> {
    let kept = |t: DateTime<Utc>| kept_from.is_none_or(|from| t >= from);
    let mut added = ClosedRecorded::default();
    let mut merge = tx.prepare_cached(
        "INSERT OR IGNORE INTO merges (repo, number, author_id, ready_at, merged_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    for c in closed {
        let number = i64::try_from(c.number).context("PR number out of range")?;
        let reviews: Vec<DecisiveReview> =
            c.reviews.iter().filter(|r| kept(r.at)).cloned().collect();
        added.reviews += reviews_seen(tx, &c.repo, number, &reviews, opted_out)?;
        let Some(author) = c
            .author_id
            .and_then(sql_id)
            .filter(|id| !opted_out.contains(id))
        else {
            continue;
        };
        if let (Some(ready), Some(merged)) = (c.ready_at, c.merged_at) {
            if ready <= merged && kept(merged) {
                added.merges +=
                    merge.execute(params![c.repo, number, author, ts(ready), ts(merged)])?;
            }
        }
    }
    Ok(added)
}

fn look(tx: &Transaction<'_>, repo: &str) -> anyhow::Result<Look> {
    let row: Option<(String, bool)> = tx
        .prepare_cached("SELECT read_at, stale FROM speed_reads WHERE repo = ?1")?
        .query_row([repo], |row| Ok((row.get(0)?, row.get(1)?)))
        .optional()?;
    Ok(match row {
        Some((at, false)) => Look::After(parse_ts(&at)?),
        _ => Look::Fresh,
    })
}

/// The repository could not be read: what is open on it goes unseen until
/// it is read again, so none of it is timed, and that read sees it afresh.
fn lose_sight(tx: &Transaction<'_>, repo: &str) -> anyhow::Result<()> {
    if tx.execute("UPDATE speed_reads SET stale = 1 WHERE repo = ?1", [repo])? == 0 {
        // Never read for speed: nothing of it is open.
        return Ok(());
    }
    tx.execute(
        "UPDATE asks SET timed = 0 WHERE repo = ?1 AND ended_at IS NULL",
        [repo],
    )?;
    tx.execute(
        "UPDATE turns SET timed = 0 WHERE repo = ?1 AND ended_at IS NULL",
        [repo],
    )?;
    Ok(())
}

/// Where a PR stands in this snapshot.
#[derive(Clone, Copy)]
enum Seen<'a> {
    Open(&'a PrOut),
    /// Merged or closed. The closed list is read after the open one, so a
    /// PR in both closed while the run read.
    Closed(&'a ClosedPr),
    /// In neither list, though the closed PRs were read: gone some other way.
    Gone,
    /// In neither list, and the closed PRs could not be read: whether and
    /// when it closed is not known yet, so what is open on it stays open.
    Unknown,
}

impl<'a> Seen<'a> {
    fn reviews(&self) -> &'a [DecisiveReview] {
        match self {
            Seen::Open(p) => &p.reviews,
            Seen::Closed(c) => &c.reviews,
            Seen::Gone | Seen::Unknown => &[],
        }
    }
}

struct OpenAsk {
    id: i64,
    person: Option<i64>,
    login: String,
    asked_at: DateTime<Utc>,
}

struct OpenTurn {
    id: i64,
    started_at: DateTime<Utc>,
    resumed_at: Option<DateTime<Utc>>,
    paused_at: Option<DateTime<Utc>>,
}

/// One repository's part of one snapshot, against what is open on it.
struct Pass<'a> {
    repo: &'a str,
    look: Look,
    at: DateTime<Utc>,
    prs: HashMap<u64, &'a PrOut>,
    closed: HashMap<u64, &'a ClosedPr>,
    closed_read: bool,
    authors: &'a Authors,
    opted_out: &'a HashSet<i64>,
}

impl<'a> Pass<'a> {
    fn seen(&self, number: u64) -> Seen<'a> {
        if let Some(c) = self.closed.get(&number) {
            Seen::Closed(c)
        } else if let Some(p) = self.prs.get(&number) {
            if p.stage == Stage::Unknown {
                // No verdict this time: one snapshot's gap in the engine's
                // export must not end, or split, what is open on the PR.
                Seen::Unknown
            } else {
                Seen::Open(p)
            }
        } else if self.closed_read {
            Seen::Gone
        } else {
            Seen::Unknown
        }
    }

    /// When an interval this snapshot shows for the first time began, and
    /// whether that is known:
    /// - the PR's recorded entry into its stage, when after the last look,
    ///   or with no last look to be after;
    /// - else the last look, when it showed the PR in another stage: the
    ///   interval began since, and this look bounds it as closely as any
    ///   change between two looks. A failed build is the usual case: the
    ///   engine dates it from when the build began, which the last look
    ///   already showed running;
    /// - else it began some time not seen: counted, not timed. The last look
    ///   (or this one, with none) stands in for the start, so a review
    ///   between the two still ends it.
    fn began(&self, tx: &Transaction<'_>, p: &PrOut) -> anyhow::Result<(DateTime<Utc>, bool)> {
        Ok(match (self.look, recorded(p)) {
            (Look::After(prev), Some(entered)) if entered > prev => (entered, true),
            (Look::After(prev), _) => {
                let was = self.stage_at(tx, p.number, prev)?;
                let moved =
                    was.is_some_and(|was| was != p.stage.key() && was != Stage::Unknown.key());
                (prev, moved)
            }
            (Look::Fresh, Some(entered)) => (entered, true),
            (Look::Fresh, None) => (self.at, false),
        })
    }

    /// The PR's stage as of the snapshot generated at `at`, as recorded.
    fn stage_at(
        &self,
        tx: &Transaction<'_>,
        number: u64,
        at: DateTime<Utc>,
    ) -> anyhow::Result<Option<String>> {
        Ok(tx
            .prepare_cached(
                "SELECT stage FROM stage_changes
                 WHERE repo = ?1 AND number = ?2 AND observed_at <= ?3
                 ORDER BY id DESC LIMIT 1",
            )?
            .query_row(
                params![
                    self.repo,
                    i64::try_from(number).context("PR number out of range")?,
                    ts(at)
                ],
                |row| row.get(0),
            )
            .optional()?)
    }

    /// When the PR left the stage it was last seen in: when it closed; its
    /// recorded entry into the stage it is in now, when after the last
    /// look; otherwise this look.
    fn left(&self, seen: &Seen<'_>) -> DateTime<Utc> {
        match (seen, self.look) {
            (Seen::Closed(c), _) => c.closed_at,
            (Seen::Open(p), Look::After(prev)) => {
                recorded(p).filter(|t| *t > prev).unwrap_or(self.at)
            }
            (Seen::Open(p), Look::Fresh) => recorded(p).unwrap_or(self.at),
            (Seen::Gone | Seen::Unknown, _) => self.at,
        }
    }

    fn asks(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        let mut open: HashMap<u64, Vec<OpenAsk>> = HashMap::new();
        {
            let mut stmt = tx.prepare_cached(
                "SELECT id, number, person_id, login, asked_at FROM asks
                 WHERE repo = ?1 AND ended_at IS NULL",
            )?;
            let rows = stmt.query_map([self.repo], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })?;
            for row in rows {
                let (id, number, person, login, asked_at) = row?;
                let number = u64::try_from(number).context("stored PR number")?;
                open.entry(number).or_default().push(OpenAsk {
                    id,
                    person,
                    login,
                    asked_at: parse_ts(&asked_at)?,
                });
            }
        }
        let numbers: BTreeSet<u64> = self.prs.keys().chain(open.keys()).copied().collect();
        for number in numbers {
            let seen = self.seen(number);
            if matches!(seen, Seen::Unknown) {
                continue;
            }
            let asked = match seen {
                Seen::Open(p) => asked(p),
                _ => BTreeSet::new(),
            };
            let reviews = seen.reviews();
            let mut kept = BTreeSet::new();
            for ask in open.remove(&number).unwrap_or_default() {
                if !asked.contains(&ask.login) {
                    self.end_ask(tx, &ask, &seen)?;
                    continue;
                }
                kept.insert(ask.login.clone());
                if ask.person.is_some() {
                    continue;
                }
                match self.authors.resolve(&ask.login, reviews) {
                    Some(id) if self.opted_out.contains(&id) => {
                        tx.execute("DELETE FROM asks WHERE id = ?1", [ask.id])?;
                    }
                    Some(id) => {
                        tx.execute("UPDATE asks SET person_id = ?2 WHERE id = ?1", [ask.id, id])?;
                    }
                    None => {}
                }
            }
            let Seen::Open(p) = seen else { continue };
            let mut insert = tx.prepare_cached(
                "INSERT INTO asks (repo, number, person_id, login, asked_at, timed)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )?;
            for login in asked.difference(&kept) {
                let person = self.authors.resolve(login, reviews);
                if person.is_some_and(|id| self.opted_out.contains(&id)) {
                    continue;
                }
                let (asked_at, timed) = self.began(tx, p)?;
                insert.execute(params![
                    self.repo,
                    i64::try_from(number).context("PR number out of range")?,
                    person,
                    login,
                    ts(asked_at),
                    timed
                ])?;
            }
        }
        Ok(())
    }

    /// The person no longer asked, the ask ends: answered by their first
    /// decisive review since they were asked; covered when others' reviews
    /// did without theirs (the PR still waits on review, now on others, or
    /// is approved and mergeable, or merged); left otherwise (back to its
    /// author, closed unmerged, gone).
    fn end_ask(&self, tx: &Transaction<'_>, ask: &OpenAsk, seen: &Seen<'_>) -> anyhow::Result<()> {
        let reviews = seen.reviews();
        let person = ask
            .person
            .or_else(|| self.authors.resolve(&ask.login, reviews));
        // By id, not login: a review by whoever holds the login now is not
        // an answer from the account that was asked.
        let answer = person.and_then(|id| {
            reviews
                .iter()
                .filter(|r| sql_id(r.reviewer_id) == Some(id) && r.at > ask.asked_at)
                .min_by_key(|r| r.at)
        });
        let (outcome, ended_at) = match (answer, seen) {
            (Some(r), _) => ("answered", r.at),
            (None, Seen::Open(p)) if matches!(p.stage, Stage::Review | Stage::Mergeable) => {
                ("covered", self.at)
            }
            (None, Seen::Closed(c)) if c.merged_at.is_some() => ("covered", c.closed_at),
            (None, Seen::Closed(c)) => ("left", c.closed_at),
            (None, _) => ("left", self.at),
        };
        match person.filter(|id| !self.opted_out.contains(id)) {
            Some(id) => tx.execute(
                "UPDATE asks SET person_id = ?2, login = NULL, ended_at = ?3, outcome = ?4
                 WHERE id = ?1",
                params![ask.id, id, ts(ended_at.max(ask.asked_at)), outcome],
            )?,
            None => tx.execute("DELETE FROM asks WHERE id = ?1", [ask.id])?,
        };
        Ok(())
    }

    /// The PR's author id: from the PR, else as recorded when it was seen
    /// with one.
    fn author(&self, tx: &Transaction<'_>, p: &PrOut) -> anyhow::Result<Option<i64>> {
        if let Some(id) = p.author_id.and_then(sql_id) {
            return Ok(Some(id));
        }
        Ok(tx
            .prepare_cached("SELECT author_id FROM pr_authors WHERE repo = ?1 AND number = ?2")?
            .query_row(
                params![
                    self.repo,
                    i64::try_from(p.number).context("PR number out of range")?
                ],
                |row| row.get(0),
            )
            .optional()?)
    }

    fn turns(&self, tx: &Transaction<'_>) -> anyhow::Result<()> {
        let mut open: HashMap<u64, OpenTurn> = HashMap::new();
        {
            let mut stmt = tx.prepare_cached(
                "SELECT id, number, started_at, resumed_at, paused_at FROM turns
                 WHERE repo = ?1 AND ended_at IS NULL",
            )?;
            let rows = stmt.query_map([self.repo], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })?;
            for row in rows {
                let (id, number, started, resumed, paused) = row?;
                open.insert(
                    u64::try_from(number).context("stored PR number")?,
                    OpenTurn {
                        id,
                        started_at: parse_ts(&started)?,
                        resumed_at: resumed.as_deref().map(parse_ts).transpose()?,
                        paused_at: paused.as_deref().map(parse_ts).transpose()?,
                    },
                );
            }
        }
        let numbers: BTreeSet<u64> = self.prs.keys().chain(open.keys()).copied().collect();
        for number in numbers {
            let seen = self.seen(number);
            let turn = open.remove(&number);
            let stage = match seen {
                Seen::Open(p) => Some(p.stage),
                Seen::Unknown => continue,
                Seen::Closed(_) | Seen::Gone => None,
            };
            match (turn, seen) {
                (None, Seen::Open(p)) if stage == Some(Stage::SelfReview) => {
                    let Some(author) = self.author(tx, p)? else {
                        continue;
                    };
                    if self.opted_out.contains(&author) {
                        continue;
                    }
                    let (started, timed) = self.began(tx, p)?;
                    tx.prepare_cached(
                        "INSERT INTO turns (repo, number, author_id, started_at, resumed_at, timed)
                         VALUES (?1, ?2, ?3, ?4, ?4, ?5)",
                    )?
                    .execute(params![
                        self.repo,
                        i64::try_from(number).context("PR number out of range")?,
                        author,
                        ts(started),
                        timed
                    ])?;
                }
                (None, _) => {}
                // Back from bots or a build: the same turn goes on.
                (Some(t), Seen::Open(p)) if stage == Some(Stage::SelfReview) => {
                    if t.resumed_at.is_none() {
                        let from = t.paused_at.unwrap_or(t.started_at);
                        let resumed = self.began(tx, p)?.0.max(from);
                        tx.execute(
                            "UPDATE turns SET resumed_at = ?2 WHERE id = ?1",
                            params![t.id, ts(resumed)],
                        )?;
                    }
                }
                // A bot or a build has it: the turn pauses.
                (Some(t), seen) if matches!(stage, Some(Stage::Bots | Stage::Ci)) => {
                    if let Some(from) = t.resumed_at {
                        let to = self.left(&seen).max(from);
                        tx.execute(
                            "UPDATE turns SET author_secs = author_secs + ?2,
                                 resumed_at = NULL, paused_at = ?3
                             WHERE id = ?1",
                            params![t.id, (to - from).num_seconds(), ts(to)],
                        )?;
                    }
                }
                // Anywhere else, or closed: the turn is over.
                (Some(t), seen) => {
                    let (secs, ended) = match t.resumed_at {
                        Some(from) => {
                            let to = self.left(&seen).max(from);
                            ((to - from).num_seconds(), to)
                        }
                        None => (0, t.paused_at.unwrap_or(t.started_at)),
                    };
                    tx.execute(
                        "UPDATE turns SET author_secs = author_secs + ?2,
                             resumed_at = NULL, ended_at = ?3
                         WHERE id = ?1",
                        params![t.id, secs, ts(ended)],
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// Delete every speed input of `user_id`.
pub(crate) fn forget(tx: &Transaction<'_>, user_id: i64) -> anyhow::Result<usize> {
    let mut deleted = 0;
    for sql in [
        "DELETE FROM asks WHERE person_id = ?1",
        "DELETE FROM turns WHERE author_id = ?1",
        "DELETE FROM merges WHERE author_id = ?1",
        "DELETE FROM reviews WHERE reviewer_id = ?1",
        "DELETE FROM pr_authors WHERE author_id = ?1",
    ] {
        deleted += tx.execute(sql, [user_id])?;
    }
    Ok(deleted)
}

/// Delete the open asks known only by `login`: they may be its holder's,
/// and no id ties them to anyone yet.
pub(crate) fn forget_login(tx: &Transaction<'_>, login: &str) -> anyhow::Result<usize> {
    Ok(tx.execute(
        "DELETE FROM asks WHERE person_id IS NULL AND login = ?1",
        [login.to_ascii_lowercase()],
    )?)
}

/// Delete speed inputs dated before `cutoff`: an ask by when it was made
/// (the month it counts in), a turn by when it ended or, still open, when
/// it began. An ask or turn left open on an abandoned PR goes too; if the
/// PR is still in that stage, the next ingest opens a fresh one, untimed.
pub(crate) fn purge(tx: &Transaction<'_>, cutoff: DateTime<Utc>) -> anyhow::Result<usize> {
    let cutoff = ts(cutoff);
    let mut deleted = 0;
    for sql in [
        "DELETE FROM asks WHERE asked_at < ?1",
        "DELETE FROM turns WHERE ended_at < ?1",
        "DELETE FROM turns WHERE ended_at IS NULL AND started_at < ?1",
        "DELETE FROM merges WHERE merged_at < ?1",
        "DELETE FROM reviews WHERE at < ?1",
        "DELETE FROM pr_authors WHERE seen_at < ?1",
    ] {
        deleted += tx.execute(sql, [&cutoff])?;
    }
    Ok(deleted)
}

// --- computing ---

/// One ask of the person's.
#[derive(Debug, Clone, PartialEq)]
struct AskRow {
    asked_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
    answered: bool,
    timed: bool,
}

/// One turn of the person's, over and timed.
#[derive(Debug, Clone, PartialEq)]
struct TurnRow {
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    author_secs: i64,
}

/// One merged PR of the person's.
#[derive(Debug, Clone, PartialEq)]
struct MergeRow {
    ready_at: DateTime<Utc>,
    merged_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
struct Inputs {
    asks: Vec<AskRow>,
    turns: Vec<TurnRow>,
    merges: Vec<MergeRow>,
}

/// A person's speed: rolling three-month medians by month, oldest first.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Speed {
    /// The earliest data the months rest on; `null` with none.
    pub since: Option<DateTime<Utc>>,
    pub months: Vec<MonthSpeed>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MonthSpeed {
    /// `YYYY-MM`.
    pub month: String,
    pub review_wait: ReviewWait,
    pub your_turn: Median,
    pub cycle_time: Median,
}

/// The asks made of the person in the window: how many, how many they
/// answered, how many are still open, and the median wait of those answered
/// and timed. Covered and left asks are counted, never timed, so leaving
/// hard PRs to others does not look fast.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReviewWait {
    pub asked: usize,
    pub answered: usize,
    pub open: usize,
    pub median_hours: Option<f64>,
    pub n: usize,
}

/// A median in hours, `null` when the window holds fewer than three values;
/// `n` is the window's count either way.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Median {
    pub median_hours: Option<f64>,
    pub n: usize,
}

fn hours(from: DateTime<Utc>, to: DateTime<Utc>) -> f64 {
    (to - from).num_seconds() as f64 / 3600.0
}

fn median(mut values: Vec<f64>) -> Median {
    let n = values.len();
    if n < MIN_N {
        return Median {
            median_hours: None,
            n,
        };
    }
    values.sort_by(f64::total_cmp);
    let mid = n / 2;
    let m = if n % 2 == 1 {
        values[mid]
    } else {
        (values[mid - 1] + values[mid]) / 2.0
    };
    Median {
        median_hours: Some((m * 10.0).round() / 10.0),
        n,
    }
}

/// Months counted from year 0, so that consecutive months differ by one.
fn month_of(t: DateTime<Utc>) -> i32 {
    t.year() * 12 + i32::try_from(t.month0()).unwrap_or(0)
}

fn month_label(m: i32) -> String {
    format!("{:04}-{:02}", m.div_euclid(12), m.rem_euclid(12) + 1)
}

fn load(conn: &Connection, user_id: i64) -> anyhow::Result<Inputs> {
    let mut inputs = Inputs::default();
    let mut stmt = conn.prepare_cached(
        "SELECT asked_at, ended_at, outcome, timed FROM asks WHERE person_id = ?1",
    )?;
    let rows = stmt.query_map([user_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, bool>(3)?,
        ))
    })?;
    for row in rows {
        let (asked_at, ended_at, outcome, timed) = row?;
        inputs.asks.push(AskRow {
            asked_at: parse_ts(&asked_at)?,
            ended_at: ended_at.as_deref().map(parse_ts).transpose()?,
            answered: outcome.as_deref() == Some("answered"),
            timed,
        });
    }
    let mut stmt = conn.prepare_cached(
        "SELECT started_at, ended_at, author_secs FROM turns
         WHERE author_id = ?1 AND ended_at IS NOT NULL AND timed = 1",
    )?;
    let rows = stmt.query_map([user_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
        ))
    })?;
    for row in rows {
        let (started_at, ended_at, author_secs) = row?;
        inputs.turns.push(TurnRow {
            started_at: parse_ts(&started_at)?,
            ended_at: parse_ts(&ended_at)?,
            author_secs,
        });
    }
    let mut stmt =
        conn.prepare_cached("SELECT ready_at, merged_at FROM merges WHERE author_id = ?1")?;
    let rows = stmt.query_map([user_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (ready_at, merged_at) = row?;
        inputs.merges.push(MergeRow {
            ready_at: parse_ts(&ready_at)?,
            merged_at: parse_ts(&merged_at)?,
        });
    }
    Ok(inputs)
}

/// The person's speed as of `now`. An ask falls in the month it was made
/// (so a month's asks read as asked, answered and still open); a turn in
/// the month it ended; a merged PR in the month it merged. Months run from
/// the first with data to the current one, the last twelve at most.
fn compute(inputs: &Inputs, now: DateTime<Utc>) -> Speed {
    let current = month_of(now);
    let first = inputs
        .asks
        .iter()
        .map(|a| month_of(a.asked_at))
        .chain(inputs.turns.iter().map(|t| month_of(t.ended_at)))
        .chain(inputs.merges.iter().map(|m| month_of(m.merged_at)))
        .filter(|m| *m <= current)
        .min();
    let Some(first) = first else {
        return Speed {
            since: None,
            months: vec![],
        };
    };
    let first = first.max(current - (MAX_MONTHS - 1));
    let in_window =
        |month: i32, t: DateTime<Utc>| (month - (WINDOW - 1)..=month).contains(&month_of(t));
    let months = (first..=current)
        .map(|month| {
            let asks: Vec<&AskRow> = inputs
                .asks
                .iter()
                .filter(|a| in_window(month, a.asked_at))
                .collect();
            let waits = median(
                asks.iter()
                    .filter(|a| a.answered && a.timed)
                    .filter_map(|a| Some(hours(a.asked_at, a.ended_at?)))
                    .collect(),
            );
            MonthSpeed {
                month: month_label(month),
                review_wait: ReviewWait {
                    asked: asks.len(),
                    answered: asks.iter().filter(|a| a.answered).count(),
                    open: asks.iter().filter(|a| a.ended_at.is_none()).count(),
                    median_hours: waits.median_hours,
                    n: waits.n,
                },
                your_turn: median(
                    inputs
                        .turns
                        .iter()
                        .filter(|t| in_window(month, t.ended_at))
                        .map(|t| t.author_secs as f64 / 3600.0)
                        .collect(),
                ),
                cycle_time: median(
                    inputs
                        .merges
                        .iter()
                        .filter(|m| in_window(month, m.merged_at))
                        .map(|m| hours(m.ready_at, m.merged_at))
                        .collect(),
                ),
            }
        })
        .collect();
    // Every row some listed month's window holds, by when its data starts.
    let used = |t: DateTime<Utc>| (first - (WINDOW - 1)..=current).contains(&month_of(t));
    let since = inputs
        .asks
        .iter()
        .filter(|a| used(a.asked_at))
        .map(|a| a.asked_at)
        .chain(
            inputs
                .turns
                .iter()
                .filter(|t| used(t.ended_at))
                .map(|t| t.started_at),
        )
        .chain(
            inputs
                .merges
                .iter()
                .filter(|m| used(m.merged_at))
                .map(|m| m.ready_at),
        )
        .min();
    Speed { since, months }
}

pub(crate) fn speed(conn: &Connection, user_id: i64, now: DateTime<Utc>) -> anyhow::Result<Speed> {
    Ok(compute(&load(conn, user_id)?, now))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{Imported, Outcome, Reader, Store};
    use pr_hygiene::dashboard::{Ask, Kind, RepoOut, SCHEMA_VERSION};
    use rusqlite::types::FromSql;

    const REPO: &str = "dashpay/platform";
    const ALICE: u64 = 1001;
    const BOB: u64 = 2002;
    const CAROL: u64 = 3003;
    const DAVE: u64 = 4004;

    fn t(s: &str) -> DateTime<Utc> {
        s.parse().unwrap()
    }

    /// A time on 2 March 2026.
    fn mar2(hhmm: &str) -> String {
        format!("2026-03-02T{hhmm}:00Z")
    }

    struct Db {
        _dir: tempfile::TempDir,
        path: std::path::PathBuf,
        store: Store,
        reader: Reader,
    }

    fn db() -> Db {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("speed.sqlite3");
        let store = Store::open(&path).unwrap();
        let reader = Reader::open(&path).unwrap();
        Db {
            _dir: dir,
            path,
            store,
            reader,
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct AskSeen {
        number: i64,
        person: Option<i64>,
        asked_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
        outcome: Option<String>,
        timed: bool,
    }

    impl Db {
        fn ingest(&mut self, d: &Dashboard) {
            let raw = serde_json::to_string(d).unwrap();
            let outcome = self.store.ingest(d, &raw, None, d.generated_at).unwrap();
            assert!(matches!(outcome, Outcome::Stored { .. }), "{outcome:?}");
        }

        fn one<T: FromSql>(&self, sql: &str) -> T {
            let conn = Connection::open(&self.path).unwrap();
            conn.query_row(sql, [], |row| row.get(0)).unwrap()
        }

        fn asks(&self) -> Vec<AskSeen> {
            let conn = Connection::open(&self.path).unwrap();
            let mut stmt = conn
                .prepare(
                    "SELECT number, person_id, asked_at, ended_at, outcome, timed
                     FROM asks ORDER BY id",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(AskSeen {
                        number: row.get(0)?,
                        person: row.get(1)?,
                        asked_at: parse_ts(&row.get::<_, String>(2)?).unwrap(),
                        ended_at: row
                            .get::<_, Option<String>>(3)?
                            .map(|s| parse_ts(&s).unwrap()),
                        outcome: row.get(4)?,
                        timed: row.get(5)?,
                    })
                })
                .unwrap()
                .map(Result::unwrap)
                .collect();
            rows
        }

        fn asks_on(&self, number: i64) -> Vec<AskSeen> {
            self.asks()
                .into_iter()
                .filter(|a| a.number == number)
                .collect()
        }

        /// Every row of every speed input naming `id`.
        fn rows_of(&self, id: u64) -> i64 {
            self.one(&format!(
                "SELECT (SELECT count(*) FROM asks WHERE person_id = {id})
                      + (SELECT count(*) FROM turns WHERE author_id = {id})
                      + (SELECT count(*) FROM merges WHERE author_id = {id})
                      + (SELECT count(*) FROM reviews WHERE reviewer_id = {id})
                      + (SELECT count(*) FROM pr_authors WHERE author_id = {id})"
            ))
        }

        fn speed(&self, id: u64, now: &str) -> Speed {
            self.reader.speed(id as i64, t(now)).unwrap()
        }
    }

    fn repo(name: &str) -> RepoOut {
        RepoOut {
            repo: name.into(),
            engine_state_available: true,
            fetch_error: None,
            stage_times_error: None,
            slot_limit: 5,
            closed_error: None,
        }
    }

    fn snap(at: &str, prs: Vec<PrOut>) -> Dashboard {
        snap_closed(at, prs, vec![])
    }

    fn snap_closed(at: &str, prs: Vec<PrOut>, closed: Vec<ClosedPr>) -> Dashboard {
        Dashboard {
            schema_version: SCHEMA_VERSION,
            generated_at: t(at),
            commit: None,
            repos: vec![repo(REPO)],
            idle_days: 14,
            stages: vec![],
            prs,
            people: vec![],
            closed,
        }
    }

    /// The repository could not be read in this snapshot.
    fn failed(at: &str) -> Dashboard {
        let mut d = snap(at, vec![]);
        d.repos[0].fetch_error = Some("HTTP 502".into());
        d
    }

    /// Alice's PR `number`, in `stage` since `since` as the engine records.
    fn pr(number: u64, stage: Stage, since: &str) -> PrOut {
        PrOut {
            key: format!("{REPO}#{number}"),
            repo: REPO.into(),
            number,
            title: format!("PR {number}"),
            author: Some("alice".into()),
            author_kind: Kind::Human,
            draft: false,
            base: Some("master".into()),
            created_at: None,
            updated_at: None,
            idle: false,
            stage,
            engine_state: None,
            next_action: None,
            blockers: vec![],
            since: Some(t(since)),
            since_basis: Some(SinceBasis::Engine),
            lateness: None,
            asks: vec![],
            objectors: vec![],
            areas: vec![],
            unresolved_comments: 0,
            ci_failing: false,
            merge_conflict: false,
            changes_requested: false,
            tracked: true,
            author_id: Some(ALICE),
            reviews: vec![],
        }
    }

    /// In review since `since`, waiting on one area anyone of `approvers`
    /// may approve.
    fn in_review(number: u64, since: &str, approvers: &[&str]) -> PrOut {
        let mut p = pr(number, Stage::Review, since);
        p.asks = vec![area("drive", approvers)];
        p
    }

    fn area(name: &str, approvers: &[&str]) -> Ask {
        Ask {
            area: name.into(),
            approvers: approvers.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// A PR `login` authored, so the snapshot ties `login` to `id`.
    fn authored_by(number: u64, login: &str, id: u64) -> PrOut {
        let mut p = pr(number, Stage::Draft, &mar2("00:00"));
        p.author = Some(login.into());
        p.author_id = Some(id);
        p
    }

    fn review(login: &str, id: u64, state: Verdict, at: &str) -> DecisiveReview {
        DecisiveReview {
            reviewer: login.into(),
            reviewer_id: id,
            state,
            at: t(at),
        }
    }

    fn closed(number: u64, merged: bool, at: &str, reviews: Vec<DecisiveReview>) -> ClosedPr {
        ClosedPr {
            key: format!("{REPO}#{number}"),
            repo: REPO.into(),
            number,
            author: Some("alice".into()),
            author_id: Some(ALICE),
            created_at: t("2026-01-01T00:00:00Z"),
            ready_at: Some(t("2026-03-01T00:00:00Z")),
            merged_at: merged.then(|| t(at)),
            closed_at: t(at),
            reviews,
        }
    }

    // --- the ask lifecycle ---

    /// Bob is asked when the PR enters review, as the engine records it,
    /// and answers with a review; the wait runs from one to the other, and
    /// the review is also what ties the login "bob" to his account.
    #[test]
    fn an_ask_is_answered_by_the_persons_first_decisive_review() {
        let mut db = db();
        db.ingest(&snap(
            &mar2("10:00"),
            vec![in_review(1, &mar2("09:00"), &["bob"])],
        ));
        let open = db.asks();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].person, None, "no event has named bob's account yet");
        assert_eq!(open[0].asked_at, t(&mar2("09:00")));

        db.ingest(&snap(
            &mar2("10:15"),
            vec![in_review(1, &mar2("09:00"), &["bob"])],
        ));
        let mut done = pr(1, Stage::Mergeable, &mar2("10:20"));
        done.reviews = vec![review("bob", BOB, Verdict::Approved, &mar2("10:20"))];
        db.ingest(&snap(&mar2("10:30"), vec![done]));

        assert_eq!(
            db.asks(),
            vec![AskSeen {
                number: 1,
                person: Some(BOB as i64),
                asked_at: t(&mar2("09:00")),
                ended_at: Some(t(&mar2("10:20"))),
                outcome: Some("answered".into()),
                timed: true,
            }]
        );
        let m = &db.speed(BOB, "2026-03-20T00:00:00Z").months[0];
        assert_eq!(
            (
                m.review_wait.asked,
                m.review_wait.answered,
                m.review_wait.open
            ),
            (1, 1, 0)
        );
        assert_eq!(m.review_wait.n, 1);
    }

    /// A co-approver's review clears the area Bob was asked about: he is no
    /// longer asked though the PR still waits on review. Counted, not timed
    /// — or leaving PRs to others would look fast.
    #[test]
    fn an_ask_cleared_by_a_co_approver_is_covered_and_not_timed() {
        let mut db = db();
        let mut asking = in_review(2, &mar2("09:00"), &["bob", "carol"]);
        asking.asks.push(area("dpp", &["dave"]));
        db.ingest(&snap(
            &mar2("10:00"),
            vec![asking, authored_by(9, "bob", BOB)],
        ));
        assert_eq!(db.asks().len(), 3);

        let mut cleared = in_review(2, &mar2("09:00"), &[]);
        cleared.asks = vec![area("dpp", &["dave"])];
        cleared.reviews = vec![review("carol", CAROL, Verdict::Approved, &mar2("10:05"))];
        db.ingest(&snap(
            &mar2("10:15"),
            vec![cleared, authored_by(9, "bob", BOB)],
        ));

        let by = |id: u64| {
            db.asks()
                .into_iter()
                .find(|a| a.person == Some(id as i64))
                .unwrap()
        };
        assert_eq!(by(BOB).outcome.as_deref(), Some("covered"));
        assert_eq!(by(BOB).ended_at, Some(t(&mar2("10:15"))));
        assert_eq!(by(CAROL).outcome.as_deref(), Some("answered"));
        assert_eq!(by(CAROL).ended_at, Some(t(&mar2("10:05"))));
        let dave: Vec<_> = db
            .asks()
            .into_iter()
            .filter(|a| a.person.is_none())
            .collect();
        assert_eq!(dave.len(), 1, "dave is still asked, and not yet named");
        assert_eq!(dave[0].ended_at, None);

        let w = &db.speed(BOB, "2026-03-20T00:00:00Z").months[0].review_wait;
        assert_eq!((w.asked, w.answered, w.open, w.n), (1, 0, 0, 0));
    }

    /// Unanswered, an ask is covered when others' reviews did without the
    /// person's — the PR approved and mergeable, or merged — and left when
    /// the PR went back to its author or closed unmerged.
    #[test]
    fn an_unanswered_ask_ends_covered_or_left_by_where_the_pr_went() {
        let mut db = db();
        let bob = authored_by(9, "bob", BOB);
        db.ingest(&snap(
            &mar2("10:00"),
            [3, 4, 5, 14, 15, 16]
                .into_iter()
                .map(|n| in_review(n, &mar2("09:00"), &["bob"]))
                .chain([bob.clone()])
                .collect(),
        ));
        // 3: back to its author. 4: closed unmerged. 5: approved by bob and
        // merged minutes later, between two snapshots: only the closed list
        // shows his review. 14: approved by others. 15: merged on others'
        // approvals. 16: merged while the run read, so in both lists; the
        // closed one is the later read.
        db.ingest(&snap_closed(
            &mar2("10:15"),
            vec![
                pr(3, Stage::SelfReview, &mar2("10:02")),
                pr(14, Stage::Mergeable, &mar2("10:05")),
                in_review(16, &mar2("09:00"), &["bob"]),
                bob,
            ],
            vec![
                closed(4, false, &mar2("10:10"), vec![]),
                closed(
                    5,
                    true,
                    &mar2("10:12"),
                    vec![review("bob", BOB, Verdict::Approved, &mar2("10:08"))],
                ),
                closed(15, true, &mar2("10:11"), vec![]),
                closed(
                    16,
                    true,
                    &mar2("10:16"),
                    vec![review("bob", BOB, Verdict::Approved, &mar2("10:14"))],
                ),
            ],
        ));
        let outcome = |n: i64| {
            let asks = db.asks_on(n);
            assert_eq!(asks.len(), 1, "#{n}");
            (asks[0].outcome.clone().unwrap(), asks[0].ended_at.unwrap())
        };
        assert_eq!(outcome(3), ("left".into(), t(&mar2("10:15"))));
        assert_eq!(outcome(4), ("left".into(), t(&mar2("10:10"))));
        assert_eq!(outcome(5), ("answered".into(), t(&mar2("10:08"))));
        assert_eq!(outcome(14), ("covered".into(), t(&mar2("10:15"))));
        assert_eq!(outcome(15), ("covered".into(), t(&mar2("10:11"))));
        assert_eq!(outcome(16), ("answered".into(), t(&mar2("10:14"))));
    }

    /// The engine may list one person twice — an approver and an objector,
    /// spelled two ways: one ask.
    #[test]
    fn a_person_asked_twice_over_on_one_pr_has_one_ask() {
        let mut db = db();
        let mut p = in_review(17, &mar2("09:00"), &["Bob"]);
        p.objectors = vec!["bob".into()];
        let bob = authored_by(9, "BOB", BOB);
        db.ingest(&snap(&mar2("10:00"), vec![p.clone(), bob.clone()]));
        db.ingest(&snap(&mar2("10:15"), vec![p, bob]));
        let asks = db.asks_on(17);
        assert_eq!(asks.len(), 1);
        assert_eq!(asks[0].person, Some(BOB as i64));
    }

    /// For one snapshot the engine gives the PR no verdict: nothing is
    /// known of it then, and what was open on it goes on, still timed.
    #[test]
    fn a_pr_without_a_verdict_for_a_snapshot_keeps_its_ask_and_turn() {
        let mut db = db();
        let bob = authored_by(9, "bob", BOB);
        let unknown = |n: u64| {
            let mut p = pr(n, Stage::Unknown, &mar2("01:00"));
            p.since_basis = Some(SinceBasis::Opened);
            p
        };
        let before = vec![
            in_review(53, &mar2("09:00"), &["bob"]),
            pr(54, Stage::SelfReview, &mar2("09:00")),
            bob.clone(),
        ];
        db.ingest(&snap(&mar2("10:00"), before.clone()));
        db.ingest(&snap(
            &mar2("10:15"),
            vec![unknown(53), unknown(54), bob.clone()],
        ));
        db.ingest(&snap(&mar2("10:30"), before));
        let mut answered = pr(53, Stage::Mergeable, &mar2("10:40"));
        answered.reviews = vec![review("bob", BOB, Verdict::Approved, &mar2("10:40"))];
        db.ingest(&snap(
            &mar2("10:45"),
            vec![answered, pr(54, Stage::Review, &mar2("10:35")), bob],
        ));
        let asks = db.asks_on(53);
        assert_eq!(asks.len(), 1, "one ask, not one before and one after");
        assert_eq!(asks[0].outcome.as_deref(), Some("answered"));
        assert!(asks[0].timed);
        assert_eq!(asks[0].asked_at, t(&mar2("09:00")));
        assert_eq!(asks[0].ended_at, Some(t(&mar2("10:40"))));
        assert_eq!(db.one::<i64>("SELECT count(*) FROM turns"), 1);
        assert_eq!(db.one::<i64>("SELECT author_secs FROM turns"), 95 * 60);
        assert!(db.one::<bool>("SELECT timed FROM turns"));
    }

    /// Bob requests changes, the author answers, and the PR comes back to
    /// review waiting on Bob's objection: a second ask, timed from the
    /// PR's return, which his earlier review does not answer.
    #[test]
    fn a_reviewer_asked_twice_on_one_pr_has_two_asks() {
        let mut db = db();
        db.ingest(&snap(
            &mar2("09:00"),
            vec![in_review(6, &mar2("08:00"), &["bob"])],
        ));
        let changes = review("bob", BOB, Verdict::ChangesRequested, &mar2("09:30"));
        let mut back = pr(6, Stage::SelfReview, &mar2("09:30"));
        back.objectors = vec!["bob".into()];
        back.reviews = vec![changes.clone()];
        db.ingest(&snap(&mar2("10:00"), vec![back]));

        let mut again = pr(6, Stage::Review, &mar2("10:30"));
        again.objectors = vec!["bob".into()];
        again.reviews = vec![changes.clone()];
        db.ingest(&snap(&mar2("11:00"), vec![again.clone()]));
        assert_eq!(db.asks_on(6).len(), 2);
        assert_eq!(
            db.asks_on(6)[1].person,
            Some(BOB as i64),
            "his earlier review on this PR names him"
        );
        db.ingest(&snap(&mar2("11:15"), vec![again]));

        let mut approved = pr(6, Stage::Mergeable, &mar2("11:45"));
        approved.reviews = vec![
            changes,
            review("bob", BOB, Verdict::Approved, &mar2("11:45")),
        ];
        db.ingest(&snap(&mar2("12:00"), vec![approved]));

        let asks = db.asks_on(6);
        let waits: Vec<_> = asks
            .iter()
            .map(|a| (a.outcome.as_deref(), hours(a.asked_at, a.ended_at.unwrap())))
            .collect();
        assert_eq!(
            waits,
            [(Some("answered"), 1.5), (Some("answered"), 1.25)],
            "each ask is answered by the first review after it began"
        );
        assert!(asks.iter().all(|a| a.timed));
    }

    /// An ask already running when its repository is first read, with no
    /// recorded start, has an unknown wait; so has one that began before
    /// the last look without being seen. Both are counted, neither timed.
    #[test]
    fn an_ask_without_a_known_start_is_counted_and_not_timed() {
        let mut db = db();
        let mut unrecorded = in_review(7, &mar2("08:00"), &["bob"]);
        unrecorded.since_basis = Some(SinceBasis::Opened);
        let bob = authored_by(9, "bob", BOB);
        db.ingest(&snap(
            &mar2("10:00"),
            vec![
                unrecorded.clone(),
                in_review(8, &mar2("09:00"), &["bob"]),
                in_review(10, &mar2("09:00"), &["carol"]),
                bob.clone(),
            ],
        ));
        // Bob is now also asked on 10, which has been in review since
        // before the last look.
        db.ingest(&snap(
            &mar2("10:15"),
            vec![
                unrecorded,
                in_review(8, &mar2("09:00"), &["bob"]),
                in_review(10, &mar2("09:00"), &["carol", "bob"]),
                bob.clone(),
            ],
        ));
        let answered = |n: u64| {
            let mut p = pr(n, Stage::Mergeable, &mar2("10:20"));
            p.reviews = vec![review("bob", BOB, Verdict::Approved, &mar2("10:20"))];
            p
        };
        db.ingest(&snap(
            &mar2("10:30"),
            vec![answered(7), answered(8), answered(10), bob],
        ));
        let timed: Vec<_> = db
            .asks()
            .into_iter()
            .filter(|a| a.person == Some(BOB as i64))
            .map(|a| (a.number, a.outcome.unwrap(), a.timed))
            .collect();
        assert_eq!(
            timed,
            [
                (7, "answered".to_string(), false),
                (8, "answered".to_string(), true),
                (10, "answered".to_string(), false),
            ]
        );
        let bobs_on_10 = db
            .asks_on(10)
            .into_iter()
            .find(|a| a.person == Some(BOB as i64))
            .unwrap();
        assert_eq!(
            bobs_on_10.asked_at,
            t(&mar2("10:00")),
            "the last look stands in for the start, so a review after it still answers"
        );
        let w = &db.speed(BOB, "2026-03-20T00:00:00Z").months[0].review_wait;
        assert_eq!((w.asked, w.answered, w.n), (3, 3, 1));
    }

    /// While a repository cannot be read, what happens on it is not seen:
    /// an ask or a turn open across the gap is counted, not timed.
    #[test]
    fn intervals_across_a_stale_repositorys_gap_are_not_timed() {
        let mut db = db();
        let bob = authored_by(9, "bob", BOB);
        db.ingest(&snap(
            &mar2("10:00"),
            vec![
                in_review(11, &mar2("09:00"), &["bob"]),
                pr(12, Stage::SelfReview, &mar2("09:00")),
                bob.clone(),
            ],
        ));
        db.ingest(&failed(&mar2("10:15")));
        db.ingest(&failed(&mar2("10:30")));
        let mut answered = pr(11, Stage::Mergeable, &mar2("10:20"));
        answered.reviews = vec![review("bob", BOB, Verdict::Approved, &mar2("10:20"))];
        db.ingest(&snap(
            &mar2("10:45"),
            vec![answered, pr(12, Stage::Review, &mar2("10:25")), bob.clone()],
        ));
        let ask = &db.asks_on(11)[0];
        assert_eq!(ask.outcome.as_deref(), Some("answered"));
        assert!(!ask.timed);
        assert!(!db.one::<bool>("SELECT timed FROM turns WHERE number = 12"));

        // Read again without a gap, the next ones are timed.
        db.ingest(&snap(
            &mar2("11:00"),
            vec![in_review(13, &mar2("10:50"), &["bob"]), bob.clone()],
        ));
        let mut answered = pr(13, Stage::Mergeable, &mar2("11:05"));
        answered.reviews = vec![review("bob", BOB, Verdict::Approved, &mar2("11:05"))];
        db.ingest(&snap(&mar2("11:15"), vec![answered, bob]));
        assert!(db.asks_on(13)[0].timed);
    }

    // --- ids ---

    /// "bob" is renamed "robert", and someone else registers "bob". An ask
    /// for "bob" after that is the new account's, tied by the new account's
    /// own review — never by what "bob" meant before.
    #[test]
    fn a_login_is_tied_to_an_account_only_by_the_same_snapshots_events() {
        const NEW_BOB: u64 = 9999;
        let mut db = db();
        db.ingest(&snap(
            &mar2("10:00"),
            vec![
                in_review(20, &mar2("09:00"), &["bob"]),
                authored_by(29, "bob", BOB),
            ],
        ));
        assert_eq!(db.asks_on(20)[0].person, Some(BOB as i64));

        // The rename: the old account's PR now shows its new login.
        db.ingest(&snap(
            &mar2("10:30"),
            vec![
                in_review(21, &mar2("10:20"), &["bob"]),
                in_review(22, &mar2("10:20"), &["bob"]),
                authored_by(29, "robert", BOB),
            ],
        ));
        assert_eq!(
            db.asks_on(21)[0].person,
            None,
            "not BOB: no lookup by login"
        );
        assert_eq!(db.asks_on(22)[0].person, None);

        // New bob answers 21; 22 is covered, and nothing ever named its
        // "bob": it belongs to no one who could be named, and goes.
        let mut answered = pr(21, Stage::Mergeable, &mar2("10:40"));
        answered.reviews = vec![review("bob", NEW_BOB, Verdict::Approved, &mar2("10:40"))];
        let mut covered = in_review(22, &mar2("10:20"), &[]);
        covered.asks = vec![area("dpp", &["dave"])];
        db.ingest(&snap(
            &mar2("10:45"),
            vec![answered, covered, authored_by(29, "robert", BOB)],
        ));

        let numbers = |id: u64| -> Vec<i64> {
            db.asks()
                .into_iter()
                .filter(|a| a.person == Some(id as i64))
                .map(|a| a.number)
                .collect()
        };
        assert_eq!(numbers(BOB), [20]);
        assert_eq!(numbers(NEW_BOB), [21]);
        assert!(
            db.asks_on(22).iter().all(|a| a.ended_at.is_none()),
            "the covered ask known only by login was deleted"
        );
    }

    // --- opting out ---

    #[test]
    fn an_opted_out_person_is_never_recorded() {
        let mut db = db();
        db.store.opt_out(BOB as i64, "bob", Utc::now()).unwrap();
        let mut bobs = authored_by(30, "bob", BOB);
        bobs.stage = Stage::SelfReview;
        let mut reviewed = in_review(31, &mar2("09:00"), &["bob", "carol"]);
        reviewed.reviews = vec![
            review("bob", BOB, Verdict::ChangesRequested, &mar2("08:00")),
            review("carol", CAROL, Verdict::ChangesRequested, &mar2("08:00")),
        ];
        let mut merged = closed(
            32,
            true,
            &mar2("09:30"),
            vec![review("bob", BOB, Verdict::Approved, &mar2("09:20"))],
        );
        merged.author = Some("bob".into());
        merged.author_id = Some(BOB);
        db.ingest(&snap_closed(
            &mar2("10:00"),
            vec![bobs, reviewed, pr(33, Stage::SelfReview, &mar2("09:00"))],
            vec![merged, closed(34, true, &mar2("09:40"), vec![])],
        ));
        assert_eq!(db.rows_of(BOB), 0, "no row of any kind names bob");
        assert_eq!(
            db.asks().len(),
            1,
            "only carol's ask: bob's is known by his review, and left out"
        );
        assert!(db.rows_of(ALICE) > 0, "alice's turn, merge and authorship");
        assert!(db.rows_of(CAROL) > 0);
    }

    /// Opting out forgets every speed input of the person; deleting the
    /// account does too, but without an opt-out recording goes on.
    #[test]
    fn opting_out_or_deleting_forgets_every_row_of_the_person() {
        let mut db = db();
        let mut reviewed = in_review(41, &mar2("09:00"), &["carol", "dave"]);
        reviewed.reviews = vec![review("carol", CAROL, Verdict::Dismissed, &mar2("08:00"))];
        let both = |at: &str| {
            snap_closed(
                at,
                vec![pr(40, Stage::SelfReview, &mar2("09:00")), reviewed.clone()],
                vec![closed(
                    42,
                    true,
                    &mar2("09:30"),
                    vec![review("carol", CAROL, Verdict::Approved, &mar2("09:20"))],
                )],
            )
        };
        db.ingest(&both(&mar2("10:00")));
        assert!(db.rows_of(ALICE) >= 3, "turn, merge, authorship");
        assert!(db.rows_of(CAROL) >= 3, "ask, reviews");
        let dave_by_login = "SELECT count(*) FROM asks WHERE login = 'dave'";
        assert_eq!(db.one::<i64>(dave_by_login), 1);

        db.store.opt_out(CAROL as i64, "carol", Utc::now()).unwrap();
        db.store.delete_user(ALICE as i64, "alice").unwrap();
        db.store.delete_user(DAVE as i64, "Dave").unwrap();
        assert_eq!(db.rows_of(CAROL), 0);
        assert_eq!(db.rows_of(ALICE), 0);
        assert_eq!(
            db.one::<i64>(dave_by_login),
            0,
            "an open ask known only by the login signed in with goes too"
        );

        db.ingest(&both(&mar2("10:15")));
        assert_eq!(db.rows_of(CAROL), 0, "the opt-out is honoured from then on");
        assert!(db.rows_of(ALICE) > 0, "a deletion alone is no opt-out");
    }

    // --- turns and merges ---

    /// Alice's turn: self-review, a push the bots and a build look at, a
    /// failed build back to her, then review. One turn, timed by her two
    /// stretches only.
    #[test]
    fn a_turn_runs_across_bot_and_build_bounces_timing_only_the_authors_stretches() {
        let mut db = db();
        db.ingest(&snap(
            &mar2("10:00"),
            vec![pr(50, Stage::SelfReview, &mar2("09:00"))],
        ));
        db.ingest(&snap(
            &mar2("10:15"),
            vec![pr(50, Stage::Bots, &mar2("10:10"))],
        ));
        let mut building = pr(50, Stage::Ci, &mar2("01:00"));
        building.since_basis = Some(SinceBasis::Opened);
        db.ingest(&snap(&mar2("10:30"), vec![building]));
        db.ingest(&snap(
            &mar2("10:45"),
            vec![pr(50, Stage::SelfReview, &mar2("10:40"))],
        ));
        db.ingest(&snap(
            &mar2("11:00"),
            vec![pr(50, Stage::Review, &mar2("10:50"))],
        ));

        assert_eq!(db.one::<i64>("SELECT count(*) FROM turns"), 1, "one turn");
        let conn = Connection::open(&db.path).unwrap();
        let (secs, started, ended, timed): (i64, String, String, bool) = conn
            .query_row(
                "SELECT author_secs, started_at, ended_at, timed FROM turns",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .unwrap();
        assert_eq!(secs, (70 + 10) * 60, "09:00–10:10 and 10:40–10:50");
        assert_eq!(parse_ts(&started).unwrap(), t(&mar2("09:00")));
        assert_eq!(parse_ts(&ended).unwrap(), t(&mar2("10:50")));
        assert!(timed);
        let m = &db.speed(ALICE, "2026-03-20T00:00:00Z").months[0];
        assert_eq!(m.your_turn.n, 1);
    }

    /// Review, a push, bots, a build seen running, and the build fails:
    /// the engine dates the failure from when the build began, before the
    /// last look, which showed it building. The turn began since that look,
    /// and is timed from it.
    #[test]
    fn a_turn_begun_by_a_failed_build_is_timed_from_the_last_look() {
        let mut db = db();
        db.ingest(&snap(
            &mar2("10:00"),
            vec![pr(52, Stage::Review, &mar2("09:00"))],
        ));
        db.ingest(&snap(
            &mar2("10:15"),
            vec![pr(52, Stage::Bots, &mar2("10:05"))],
        ));
        let mut building = pr(52, Stage::Ci, &mar2("01:00"));
        building.since_basis = Some(SinceBasis::Opened);
        db.ingest(&snap(&mar2("10:30"), vec![building]));
        db.ingest(&snap(
            &mar2("10:45"),
            vec![pr(52, Stage::SelfReview, &mar2("10:20"))],
        ));
        db.ingest(&snap(
            &mar2("11:00"),
            vec![pr(52, Stage::Review, &mar2("10:55"))],
        ));
        let conn = Connection::open(&db.path).unwrap();
        let (secs, started, timed): (i64, String, bool) = conn
            .query_row(
                "SELECT author_secs, started_at, timed FROM turns",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert!(timed);
        assert_eq!(parse_ts(&started).unwrap(), t(&mar2("10:30")));
        assert_eq!(secs, 25 * 60, "10:30 to 10:55");
    }

    #[test]
    fn a_turn_already_running_without_a_recorded_start_is_not_timed() {
        let mut db = db();
        let mut unrecorded = pr(51, Stage::SelfReview, &mar2("01:00"));
        unrecorded.since_basis = Some(SinceBasis::Opened);
        db.ingest(&snap(&mar2("10:00"), vec![unrecorded]));
        db.ingest(&snap(
            &mar2("10:15"),
            vec![pr(51, Stage::Review, &mar2("10:10"))],
        ));
        assert!(!db.one::<bool>("SELECT timed FROM turns"));
        assert_eq!(
            db.speed(ALICE, "2026-03-20T00:00:00Z").months,
            [],
            "nothing timed to show"
        );
    }

    #[test]
    fn merges_are_recorded_from_merged_prs_with_their_ready_and_merge_times() {
        let mut db = db();
        let mut draft_only = closed(62, false, &mar2("09:00"), vec![]);
        draft_only.ready_at = None;
        db.ingest(&snap_closed(
            &mar2("10:00"),
            vec![],
            vec![
                closed(60, true, &mar2("09:00"), vec![]),
                closed(61, false, &mar2("09:00"), vec![]),
                draft_only,
            ],
        ));
        assert_eq!(db.one::<i64>("SELECT count(*) FROM merges"), 1);
        assert_eq!(db.one::<i64>("SELECT number FROM merges"), 60);
        let c = &db.speed(ALICE, "2026-03-20T00:00:00Z").months[0].cycle_time;
        assert_eq!(c.n, 1);
    }

    /// A PR gone from the open list, while the closed list could not be
    /// read, may or may not have closed: what is open on it waits.
    #[test]
    fn a_pr_gone_while_closed_prs_could_not_be_read_keeps_its_ask_open() {
        let mut db = db();
        let bob = authored_by(9, "bob", BOB);
        db.ingest(&snap(
            &mar2("10:00"),
            vec![in_review(70, &mar2("09:00"), &["bob"]), bob.clone()],
        ));
        let mut unread = snap(&mar2("10:15"), vec![bob.clone()]);
        unread.repos[0].closed_error = Some("HTTP 502".into());
        db.ingest(&unread);
        assert_eq!(db.asks_on(70)[0].ended_at, None);
        db.ingest(&snap_closed(
            &mar2("10:30"),
            vec![bob],
            vec![closed(
                70,
                true,
                &mar2("10:10"),
                vec![review("bob", BOB, Verdict::Approved, &mar2("10:05"))],
            )],
        ));
        assert_eq!(db.asks_on(70)[0].outcome.as_deref(), Some("answered"));
    }

    #[test]
    fn a_repository_dropped_from_the_registry_ends_what_was_open_on_it() {
        let mut db = db();
        db.ingest(&snap(
            &mar2("10:00"),
            vec![
                in_review(80, &mar2("09:00"), &["bob"]),
                pr(81, Stage::SelfReview, &mar2("09:00")),
                authored_by(9, "bob", BOB),
            ],
        ));
        let mut moved = snap(&mar2("10:15"), vec![]);
        moved.repos = vec![repo("dashpay/other")];
        db.ingest(&moved);
        assert_eq!(db.asks_on(80)[0].outcome.as_deref(), Some("left"));
        assert_eq!(
            db.one::<i64>("SELECT count(*) FROM turns WHERE ended_at IS NULL"),
            0
        );
        assert_eq!(
            db.one::<i64>("SELECT count(*) FROM speed_reads WHERE repo = 'dashpay/platform'"),
            0
        );
    }

    // --- retention and import ---

    #[test]
    fn speed_inputs_are_purged_thirteen_months_after_their_time() {
        let mut db = db();
        db.ingest(&snap_closed(
            &mar2("10:00"),
            vec![pr(90, Stage::SelfReview, &mar2("09:00"))],
            vec![closed(
                91,
                true,
                &mar2("09:30"),
                vec![review("bob", BOB, Verdict::Approved, &mar2("09:20"))],
            )],
        ));
        // A year on, PR 90 is still in self-review, abandoned; 92 is new.
        db.ingest(&snap(
            "2027-03-01T10:00:00Z",
            vec![
                pr(90, Stage::SelfReview, &mar2("09:00")),
                pr(92, Stage::SelfReview, "2027-03-01T09:00:00Z"),
            ],
        ));
        let rows = db.rows_of(ALICE) + db.rows_of(BOB);
        let purged = db.store.purge_expired(t("2027-04-01T00:00:00Z")).unwrap();
        assert_eq!(purged.speed_inputs, 0, "13 months on, all but a day: kept");
        let purged = db.store.purge_expired(t("2027-04-03T00:00:00Z")).unwrap();
        assert_eq!(db.rows_of(BOB), 0, "his review on 91");
        assert_eq!(
            db.one::<i64>("SELECT number FROM turns"),
            92,
            "the turn left open on 90 for over 13 months goes; 92's stays"
        );
        assert_eq!(db.one::<i64>("SELECT count(*) FROM merges"), 0);
        assert_eq!(
            db.rows_of(ALICE),
            3,
            "92's turn, and both PRs' authorship, seen a month ago"
        );
        assert_eq!(purged.speed_inputs as i64, rows - 3);
    }

    /// A year of closed PRs is read once and imported after posting has
    /// begun, so it is older than the latest snapshot. It is not stored as
    /// a snapshot, but its merges and reviews are — except those already
    /// past the 13 months kept.
    #[test]
    fn an_older_backfill_import_adds_its_closed_prs_merges_and_reviews() {
        let mut db = db();
        db.ingest(&snap(&mar2("10:00"), vec![]));
        let mut year_ago = closed(
            100,
            true,
            "2025-03-10T00:00:00Z",
            vec![review(
                "bob",
                BOB,
                Verdict::Approved,
                "2025-03-09T00:00:00Z",
            )],
        );
        year_ago.ready_at = Some(t("2025-03-05T00:00:00Z"));
        let mut too_old = closed(
            101,
            true,
            "2025-01-10T00:00:00Z",
            vec![review(
                "bob",
                BOB,
                Verdict::Approved,
                "2025-01-09T00:00:00Z",
            )],
        );
        too_old.ready_at = Some(t("2025-01-05T00:00:00Z"));
        let backfill = snap_closed(&mar2("09:00"), vec![], vec![year_ago, too_old]);
        let raw = serde_json::to_string(&backfill).unwrap();
        let imported = db.store.import(&backfill, &raw, t(&mar2("10:30"))).unwrap();
        assert!(matches!(imported.outcome, Outcome::NotNewer { .. }));
        assert_eq!(
            imported,
            Imported {
                outcome: imported.outcome.clone(),
                closed: Some(ClosedRecorded {
                    merges: 1,
                    reviews: 1
                }),
            }
        );
        assert_eq!(db.one::<i64>("SELECT count(*) FROM snapshots"), 1);
        let speed = db.speed(ALICE, "2026-03-20T00:00:00Z");
        assert_eq!(speed.since, Some(t("2025-03-05T00:00:00Z")));
        assert_eq!(speed.months[0].month, "2025-04");
        assert_eq!(
            speed.months[0].cycle_time.n, 1,
            "March 2025 is in its window"
        );
    }

    // --- computing ---

    fn merge_in(month: &str, hours: i64) -> MergeRow {
        let merged_at = t(&format!("{month}-15T00:00:00Z"));
        MergeRow {
            ready_at: merged_at - chrono::TimeDelta::hours(hours),
            merged_at,
        }
    }

    /// Each month's median rolls over it and the two before, and one or two
    /// values make none.
    #[test]
    fn medians_roll_over_three_months_and_need_three_values() {
        let inputs = Inputs {
            merges: vec![
                merge_in("2026-01", 10),
                merge_in("2026-02", 20),
                merge_in("2026-03", 31),
            ],
            ..Inputs::default()
        };
        let speed = compute(&inputs, t("2026-06-10T00:00:00Z"));
        let cycle: Vec<_> = speed
            .months
            .iter()
            .map(|m| (m.month.as_str(), m.cycle_time.median_hours, m.cycle_time.n))
            .collect();
        assert_eq!(
            cycle,
            [
                ("2026-01", None, 1),
                ("2026-02", None, 2),
                ("2026-03", Some(20.0), 3),
                ("2026-04", None, 2),
                ("2026-05", None, 1),
                ("2026-06", None, 0),
            ]
        );
        assert_eq!(median(vec![1.0, 2.0, 4.0, 8.0]).median_hours, Some(3.0));
    }

    #[test]
    fn months_run_oldest_first_to_the_current_one_twelve_at_most() {
        let inputs = Inputs {
            asks: vec![AskRow {
                asked_at: t("2025-05-03T00:00:00Z"),
                ended_at: Some(t("2025-05-04T00:00:00Z")),
                answered: true,
                timed: true,
            }],
            merges: vec![merge_in("2025-01", 5), merge_in("2026-06", 5)],
            ..Inputs::default()
        };
        let speed = compute(&inputs, t("2026-06-10T00:00:00Z"));
        let months: Vec<&str> = speed.months.iter().map(|m| m.month.as_str()).collect();
        assert_eq!(
            months,
            [
                "2025-07", "2025-08", "2025-09", "2025-10", "2025-11", "2025-12", "2026-01",
                "2026-02", "2026-03", "2026-04", "2026-05", "2026-06"
            ]
        );
        assert_eq!(
            speed.months[0].review_wait.asked, 1,
            "May 2025 is in July's window"
        );
        assert_eq!(
            speed.since,
            Some(t("2025-05-03T00:00:00Z")),
            "January 2025 is in no listed month's window"
        );
        assert_eq!(
            compute(&Inputs::default(), t("2026-06-10T00:00:00Z")).months,
            []
        );
    }

    #[test]
    fn an_answer_is_shaped_as_the_page_reads_it() {
        let inputs = Inputs {
            asks: vec![
                AskRow {
                    asked_at: t("2026-09-01T00:00:00Z"),
                    ended_at: Some(t("2026-09-01T18:30:00Z")),
                    answered: true,
                    timed: true,
                },
                AskRow {
                    asked_at: t("2026-09-02T00:00:00Z"),
                    ended_at: None,
                    answered: false,
                    timed: true,
                },
            ],
            ..Inputs::default()
        };
        let speed = compute(&inputs, t("2026-09-20T00:00:00Z"));
        assert_eq!(
            serde_json::to_value(&speed).unwrap(),
            serde_json::json!({
                "since": "2026-09-01T00:00:00Z",
                "months": [{
                    "month": "2026-09",
                    "review_wait": {"asked": 2, "answered": 1, "open": 1, "median_hours": null, "n": 1},
                    "your_turn": {"median_hours": null, "n": 0},
                    "cycle_time": {"median_hours": null, "n": 0},
                }]
            })
        );
    }
}
