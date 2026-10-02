//! SQLite: the snapshots received, each repository's last good data, the
//! view assembled from them, and every stage change observed.
//!
//! One writer, in `BEGIN IMMEDIATE` transactions, so the "is it newer"
//! check and the write cannot interleave with another ingest. The API reads
//! through its own read-only connection.

use crate::view::{self, Kept, RepoData, View};
use anyhow::Context;
use chrono::{DateTime, SecondsFormat, TimeDelta, Utc};
use pr_hygiene::dashboard::{Dashboard, PrOut, SinceBasis, Stage};
use rusqlite::{
    params, Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

/// Raw snapshots are kept this long, for debugging and replaying a view.
pub const RAW_RETENTION: TimeDelta = TimeDelta::days(30);

const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

/// Bumped with every change to the tables below.
const SCHEMA_VERSION: i64 = 1;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS snapshots (
    id            INTEGER PRIMARY KEY,
    received_at   TEXT NOT NULL,
    generated_at  TEXT NOT NULL,
    commit_sha    TEXT,
    -- The snapshot as received; cleared once older than the retention.
    raw           TEXT
);
-- Each repository's part of the latest snapshot that read it.
CREATE TABLE IF NOT EXISTS repos (
    repo          TEXT PRIMARY KEY,
    snapshot_id   INTEGER NOT NULL REFERENCES snapshots (id),
    good_at       TEXT NOT NULL,
    data          TEXT NOT NULL
);
-- One row each time a PR is first seen in a stage, engine state or
-- recorded entry time.
CREATE TABLE IF NOT EXISTS stage_changes (
    id            INTEGER PRIMARY KEY,
    repo          TEXT NOT NULL,
    number        INTEGER NOT NULL,
    stage         TEXT NOT NULL,
    engine_state  TEXT,
    -- When the PR entered the stage, where that is recorded; otherwise
    -- only `observed_at` (the generation time of the snapshot that first
    -- showed it) bounds it.
    since         TEXT,
    observed_at   TEXT NOT NULL,
    snapshot_id   INTEGER NOT NULL REFERENCES snapshots (id)
);
CREATE INDEX IF NOT EXISTS stage_changes_by_pr ON stage_changes (repo, number, id);
-- What the API serves, assembled at each ingest.
CREATE TABLE IF NOT EXISTS view (
    id            INTEGER PRIMARY KEY CHECK (id = 1),
    version       INTEGER NOT NULL,
    body          TEXT NOT NULL
);
";

/// Fixed-width UTC, so the text sorts as the time does.
fn ts(t: DateTime<Utc>) -> String {
    t.to_rfc3339_opts(SecondsFormat::Micros, true)
}

fn parse_ts(s: &str) -> anyhow::Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)
        .with_context(|| format!("stored time {s:?}"))?
        .with_timezone(&Utc))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Stored {
        snapshot: i64,
        stale: Vec<String>,
        stage_changes: usize,
    },
    /// Not generated after the latest snapshot stored: a replay, or an
    /// older run arriving late. Nothing was written.
    NotNewer { latest: DateTime<Utc> },
}

pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open (creating if need be) the database for writing.
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        conn.busy_timeout(BUSY_TIMEOUT)?;
        let mode: String =
            conn.pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get(0))?;
        anyhow::ensure!(
            mode.eq_ignore_ascii_case("wal"),
            "{}: WAL is not available (journal_mode={mode}); is it on a network file system?",
            path.display()
        );
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "foreign_keys", true)?;
        let version: i64 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;
        anyhow::ensure!(
            version <= SCHEMA_VERSION,
            "{}: schema version {version} is newer than this service ({SCHEMA_VERSION})",
            path.display()
        );
        conn.execute_batch(SCHEMA)?;
        conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        Ok(Self { conn })
    }

    /// Store a checked snapshot and the view it makes: repositories it read
    /// replace their kept data and record their stage changes; those it
    /// could not read keep theirs, untouched.
    pub fn ingest(
        &mut self,
        d: &Dashboard,
        raw: &str,
        received_at: DateTime<Utc>,
    ) -> anyhow::Result<Outcome> {
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let latest: Option<String> = tx
            .query_row(
                "SELECT generated_at FROM snapshots ORDER BY id DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let latest = latest.as_deref().map(parse_ts).transpose()?;
        if let Some(latest) = latest.filter(|latest| d.generated_at <= *latest) {
            return Ok(Outcome::NotNewer { latest });
        }
        tx.execute(
            "INSERT INTO snapshots (received_at, generated_at, commit_sha, raw)
             VALUES (?1, ?2, ?3, ?4)",
            params![ts(received_at), ts(d.generated_at), d.commit, raw],
        )?;
        let id = tx.last_insert_rowid();

        let mut kept = load_kept(&tx)?;
        let mut stage_changes = 0;
        for status in d.repos.iter().filter(|r| view::is_good(r)) {
            let data = view::repo_part(d, status);
            stage_changes += record_stage_changes(&tx, id, d.generated_at, &data.prs)?;
            tx.execute(
                "INSERT INTO repos (repo, snapshot_id, good_at, data) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (repo) DO UPDATE SET
                     snapshot_id = excluded.snapshot_id,
                     good_at = excluded.good_at,
                     data = excluded.data",
                params![
                    status.repo,
                    id,
                    ts(d.generated_at),
                    serde_json::to_string(&data)?
                ],
            )?;
            kept.insert(
                status.repo.clone(),
                Kept {
                    good_at: d.generated_at,
                    data,
                },
            );
        }
        // Every snapshot lists every governed repository; one it no longer
        // lists has left the registry, and its data goes with it.
        let listed: HashSet<&str> = d.repos.iter().map(|r| r.repo.as_str()).collect();
        let gone: Vec<String> = kept
            .keys()
            .filter(|r| !listed.contains(r.as_str()))
            .cloned()
            .collect();
        for repo in &gone {
            tx.execute("DELETE FROM repos WHERE repo = ?1", [repo])?;
            kept.remove(repo);
        }

        let view = view::assemble(d, id, received_at, &kept);
        tx.execute(
            "INSERT INTO view (id, version, body) VALUES (1, ?1, ?2)
             ON CONFLICT (id) DO UPDATE SET version = excluded.version, body = excluded.body",
            params![id, serde_json::to_string(&view)?],
        )?;
        tx.execute(
            "UPDATE snapshots SET raw = NULL WHERE raw IS NOT NULL AND received_at < ?1",
            [ts(received_at - RAW_RETENTION)],
        )?;
        tx.commit()?;
        Ok(Outcome::Stored {
            snapshot: id,
            stale: view.stale_repos().into_iter().map(String::from).collect(),
            stage_changes,
        })
    }
}

fn load_kept(tx: &Transaction<'_>) -> anyhow::Result<HashMap<String, Kept>> {
    let mut stmt = tx.prepare("SELECT repo, good_at, data FROM repos")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut kept = HashMap::new();
    for row in rows {
        let (repo, good_at, data) = row?;
        let data: RepoData =
            serde_json::from_str(&data).with_context(|| format!("kept data of {repo}"))?;
        kept.insert(
            repo,
            Kept {
                good_at: parse_ts(&good_at)?,
                data,
            },
        );
    }
    Ok(kept)
}

/// A row when a PR is first seen, or seen in another stage or engine state,
/// or with a recorded entry time other than the one last recorded (it left
/// and re-entered between two snapshots, or its entry has only now been
/// read). An entry time that could not be read this time is no change: the
/// PR has not moved, and recording it would make rows of read failures.
fn record_stage_changes(
    tx: &Transaction<'_>,
    snapshot_id: i64,
    observed_at: DateTime<Utc>,
    prs: &[PrOut],
) -> anyhow::Result<usize> {
    let mut last = tx.prepare_cached(
        "SELECT stage, engine_state, since FROM stage_changes
         WHERE repo = ?1 AND number = ?2 ORDER BY id DESC LIMIT 1",
    )?;
    let mut insert = tx.prepare_cached(
        "INSERT INTO stage_changes (repo, number, stage, engine_state, since, observed_at, snapshot_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    let mut written = 0;
    for pr in prs {
        let number = i64::try_from(pr.number).context("PR number out of range")?;
        let stage = pr.stage.key();
        let since = pr
            .since
            .filter(|_| pr.since_basis == Some(SinceBasis::Engine))
            .map(ts);
        let previous: Option<(String, Option<String>, Option<String>)> = last
            .query_row(params![pr.repo, number], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .optional()?;
        let changed = match previous {
            None => true,
            Some((was_stage, was_state, was_since)) => {
                was_stage != stage
                    || was_state != pr.engine_state
                    || (since.is_some() && since != was_since)
            }
        };
        if changed {
            insert.execute(params![
                pr.repo,
                number,
                stage,
                pr.engine_state,
                since,
                ts(observed_at),
                snapshot_id
            ])?;
            written += 1;
        }
    }
    Ok(written)
}

/// One recorded stage change of a PR.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StageChange {
    pub stage: Stage,
    pub engine_state: Option<String>,
    /// When the PR entered the stage, where recorded.
    pub since: Option<DateTime<Utc>>,
    /// The generation time of the snapshot that first showed it.
    pub observed_at: DateTime<Utc>,
}

/// The API's connection: read-only, so nothing it does can write.
pub struct Reader {
    conn: Connection,
}

impl Reader {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_context(|| format!("opening {} read-only", path.display()))?;
        conn.busy_timeout(BUSY_TIMEOUT)?;
        conn.pragma_update(None, "query_only", true)?;
        Ok(Self { conn })
    }

    pub fn ping(&self) -> anyhow::Result<()> {
        self.conn
            .query_row("SELECT count(*) FROM view", [], |row| row.get::<_, i64>(0))?;
        Ok(())
    }

    /// The version of the view stored, if any.
    pub fn version(&self) -> anyhow::Result<Option<i64>> {
        Ok(self
            .conn
            .query_row("SELECT version FROM view WHERE id = 1", [], |row| {
                row.get(0)
            })
            .optional()?)
    }

    pub fn view(&self) -> anyhow::Result<Option<View>> {
        let body: Option<String> = self
            .conn
            .query_row("SELECT body FROM view WHERE id = 1", [], |row| row.get(0))
            .optional()?;
        body.map(|b| serde_json::from_str(&b).context("stored view"))
            .transpose()
    }

    pub fn stage_changes(&self, repo: &str, number: u64) -> anyhow::Result<Vec<StageChange>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT stage, engine_state, since, observed_at FROM stage_changes
             WHERE repo = ?1 AND number = ?2 ORDER BY id",
        )?;
        let Ok(number) = i64::try_from(number) else {
            return Ok(vec![]);
        };
        let rows = stmt.query_map(params![repo, number], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        let mut out = vec![];
        for row in rows {
            let (stage, engine_state, since, observed_at) = row?;
            out.push(StageChange {
                stage: serde_json::from_value(serde_json::Value::String(stage))
                    .context("stored stage")?,
                engine_state,
                since: since.as_deref().map(parse_ts).transpose()?,
                observed_at: parse_ts(&observed_at)?,
            });
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::{fixture, minutes};
    use pr_hygiene::dashboard::Stage;

    const PLATFORM: &str = "dashpay/platform";
    const DASHCORE: &str = "dashpay/rust-dashcore";

    struct Db {
        _dir: tempfile::TempDir,
        store: Store,
        reader: Reader,
    }

    fn db() -> Db {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.sqlite3");
        let store = Store::open(&path).unwrap();
        let reader = Reader::open(&path).unwrap();
        Db {
            _dir: dir,
            store,
            reader,
        }
    }

    /// The fixture with both repositories read, generated `at` minutes in.
    fn snapshot(at: i64) -> Dashboard {
        let mut d = fixture();
        for r in &mut d.repos {
            r.engine_state_available = true;
        }
        d.generated_at += minutes(at);
        d
    }

    fn ingest(db: &mut Db, d: &Dashboard) -> Outcome {
        let raw = serde_json::to_string(d).unwrap();
        db.store.ingest(d, &raw, d.generated_at).unwrap()
    }

    fn stored(o: &Outcome) -> (Vec<String>, usize) {
        match o {
            Outcome::Stored {
                stale,
                stage_changes,
                ..
            } => (stale.clone(), *stage_changes),
            Outcome::NotNewer { .. } => panic!("not stored: {o:?}"),
        }
    }

    fn count(db: &Db, sql: &str) -> i64 {
        db.reader.conn.query_row(sql, [], |row| row.get(0)).unwrap()
    }

    #[test]
    fn stage_changes_are_written_only_when_something_changes() {
        let mut db = db();
        let first = snapshot(0);
        let (_, written) = stored(&ingest(&mut db, &first));
        assert_eq!(written, first.prs.len(), "first sighting of every PR");

        let (_, written) = stored(&ingest(&mut db, &snapshot(15)));
        assert_eq!(written, 0, "nothing moved: no rows");

        let mut moved = snapshot(30);
        let pr = moved.prs.iter_mut().find(|p| p.repo == PLATFORM).unwrap();
        let number = pr.number;
        pr.stage = Stage::Mergeable;
        pr.engine_state = Some("ready-to-merge".into());
        pr.since_basis = Some(SinceBasis::Engine);
        pr.since = Some(moved.generated_at - minutes(5));
        let (_, written) = stored(&ingest(&mut db, &moved));
        assert_eq!(written, 1, "one PR moved: one row");

        // Its entry time could not be read this time: it has not moved.
        let mut unread = snapshot(45);
        let same = unread.prs.iter_mut().find(|p| p.number == number).unwrap();
        same.stage = Stage::Mergeable;
        same.engine_state = Some("ready-to-merge".into());
        same.since_basis = Some(SinceBasis::Opened);
        let (_, written) = stored(&ingest(&mut db, &unread));
        assert_eq!(written, 0, "a lost entry time is not a move");

        let history = db.reader.stage_changes(PLATFORM, number).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].stage, Stage::Mergeable);
        assert_eq!(history[1].since, Some(moved.generated_at - minutes(5)));
        assert_eq!(history[1].observed_at, moved.generated_at);
    }

    #[test]
    fn a_failed_repository_keeps_its_data_and_records_nothing() {
        let mut db = db();
        let good = snapshot(0);
        ingest(&mut db, &good);
        let platform_rows = "SELECT count(*) FROM stage_changes WHERE repo = 'dashpay/platform'";
        let rows_before = count(&db, platform_rows);

        // The next run cannot fetch platform. What it still says about
        // platform — here an engine-only PR in another state — is not data.
        let mut failed = snapshot(15);
        failed.repos[0].fetch_error = Some("HTTP 502".into());
        let pr = failed.prs.iter_mut().find(|p| p.number == 9100).unwrap();
        pr.stage = Stage::Blocked;
        pr.engine_state = Some("configuration-error".into());
        let (stale, _) = stored(&ingest(&mut db, &failed));
        assert_eq!(stale, vec![PLATFORM.to_string()]);
        assert_eq!(
            count(&db, platform_rows),
            rows_before,
            "a repository that was not read records no stage changes"
        );

        let view = db.reader.view().unwrap().unwrap();
        let kept: Vec<_> = view.prs.iter().filter(|p| p.repo == PLATFORM).collect();
        let before: Vec<_> = good.prs.iter().filter(|p| p.repo == PLATFORM).collect();
        assert_eq!(kept, before, "its last good PRs are still served");
        let repo = view.repo(PLATFORM).unwrap();
        assert!(repo.stale);
        assert_eq!(repo.data_as_of, Some(good.generated_at));
        assert_eq!(repo.fetch_error.as_deref(), Some("HTTP 502"));
        assert!(!view.repo(DASHCORE).unwrap().stale);
    }

    #[test]
    fn a_snapshot_not_newer_than_the_latest_is_ignored() {
        let mut db = db();
        let d = snapshot(0);
        ingest(&mut db, &d);
        assert_eq!(
            ingest(&mut db, &d),
            Outcome::NotNewer {
                latest: d.generated_at
            }
        );
        assert_eq!(
            ingest(&mut db, &snapshot(-15)),
            Outcome::NotNewer {
                latest: d.generated_at
            }
        );
        assert_eq!(count(&db, "SELECT count(*) FROM snapshots"), 1);
    }

    #[test]
    fn a_repository_never_read_is_listed_stale_with_no_data() {
        let mut db = db();
        // As in the fixture: no engine export for rust-dashcore.
        let d = fixture();
        let (stale, _) = stored(&ingest(&mut db, &d));
        assert_eq!(stale, vec![DASHCORE.to_string()]);
        let view = db.reader.view().unwrap().unwrap();
        assert!(view.prs.iter().all(|p| p.repo != DASHCORE));
        assert_eq!(view.repo(DASHCORE).unwrap().data_as_of, None);
    }

    #[test]
    fn raw_snapshots_are_cleared_after_the_retention() {
        let mut db = db();
        let old = snapshot(0);
        ingest(&mut db, &old);
        let mut later = snapshot(1);
        later.generated_at = old.generated_at + RAW_RETENTION + minutes(1);
        ingest(&mut db, &later);
        assert_eq!(
            count(&db, "SELECT count(*) FROM snapshots WHERE raw IS NULL"),
            1
        );
        assert_eq!(
            count(&db, "SELECT count(*) FROM snapshots"),
            2,
            "metadata stays"
        );
    }
}
