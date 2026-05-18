use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::model::Snapshot;

/// Where snapshots live, relative to the repo root.
pub const HISTORY_DIR: &str = ".pr-hygiene/history";
pub const AUTHOR_CACHE: &str = ".pr-hygiene/authors.json";

pub fn load_author_cache(path: &Path) -> Result<HashMap<String, NaiveDate>> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let map: HashMap<String, NaiveDate> =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(map)
}

pub fn save_author_cache(path: &Path, cache: &HashMap<String, NaiveDate>) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(cache).context("serialising author cache")?;
    std::fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

fn snapshot_path(dir: &Path, date: NaiveDate) -> PathBuf {
    dir.join(format!("{}.json", date.format("%Y-%m-%d")))
}

/// Look for a snapshot ~7 days before `today` (window: 5..=9 days), preferring closer dates.
pub fn load_previous(dir: &Path, today: NaiveDate) -> Result<Option<Snapshot>> {
    // Order of preference: 7, 6, 8, 5, 9.
    let offsets: [i64; 5] = [7, 6, 8, 5, 9];
    for off in offsets {
        let candidate = today - Duration::days(off);
        let path = snapshot_path(dir, candidate);
        if path.exists() {
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let snap: Snapshot = serde_json::from_str(&text)
                .with_context(|| format!("parsing {}", path.display()))?;
            tracing::debug!(date = %candidate, "loaded previous snapshot for delta");
            return Ok(Some(snap));
        }
    }
    Ok(None)
}

pub fn write_snapshot(dir: &Path, snap: &Snapshot) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)
        .with_context(|| format!("creating directory {}", dir.display()))?;
    let path = snapshot_path(dir, snap.date);
    let json = serde_json::to_string_pretty(snap).context("serialising snapshot")?;
    std::fs::write(&path, json).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

/// Delete snapshot files whose date is older than `today - retention_days`.
/// Returns the number of files removed.
pub fn prune(dir: &Path, today: NaiveDate, retention_days: i64) -> Result<usize> {
    if !dir.exists() {
        return Ok(0);
    }
    let cutoff = today - Duration::days(retention_days);
    let mut removed = 0;
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let Ok(date) = NaiveDate::parse_from_str(stem, "%Y-%m-%d") else {
            continue;
        };
        if date < cutoff {
            std::fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
            removed += 1;
        }
    }
    Ok(removed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AuthorSnapshot, PrSnapshot, Summary};
    use tempdir_minimal::TempDir;

    fn snap(date: NaiveDate) -> Snapshot {
        Snapshot {
            date,
            summary: Summary {
                open_prs: 0,
                prs_needing_author_action: 0,
                total_unresolved: 0,
            },
            per_author: vec![],
            per_pr: vec![],
        }
    }

    #[test]
    fn write_then_load_roundtrips() {
        let tmp = TempDir::new();
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        let week_ago = today - Duration::days(7);
        let s = Snapshot {
            date: week_ago,
            summary: Summary {
                open_prs: 5,
                prs_needing_author_action: 2,
                total_unresolved: 7,
            },
            per_author: vec![AuthorSnapshot {
                login: "alice".into(),
                total_open_prs: 3,
                prs_needing_author_action: 1,
                total_unresolved: 2,
                total_score: 1.5,
                oldest_stale_pr_days: 4.0,
            }],
            per_pr: vec![PrSnapshot {
                number: 1,
                author: Some("alice".into()),
                score: 1.5,
                unresolved_total: 2,
                by_severity: Default::default(),
                by_source: Default::default(),
                needs_author_action: true,
            }],
        };
        write_snapshot(tmp.path(), &s).unwrap();
        let loaded = load_previous(tmp.path(), today).unwrap().unwrap();
        assert_eq!(loaded.summary.open_prs, 5);
        assert_eq!(loaded.per_author[0].login, "alice");
    }

    #[test]
    fn load_previous_handles_no_history() {
        let tmp = TempDir::new();
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        assert!(load_previous(tmp.path(), today).unwrap().is_none());
    }

    #[test]
    fn load_previous_falls_back_within_window() {
        let tmp = TempDir::new();
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        // Write a snapshot for 8 days ago (the 7-day-ago file doesn't exist).
        write_snapshot(tmp.path(), &snap(today - Duration::days(8))).unwrap();
        let loaded = load_previous(tmp.path(), today).unwrap().unwrap();
        assert_eq!(loaded.date, today - Duration::days(8));
    }

    #[test]
    fn load_previous_returns_none_outside_window() {
        let tmp = TempDir::new();
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        // 15 days ago is outside the 5..=9 window.
        write_snapshot(tmp.path(), &snap(today - Duration::days(15))).unwrap();
        assert!(load_previous(tmp.path(), today).unwrap().is_none());
    }

    #[test]
    fn prune_removes_old_files_only() {
        let tmp = TempDir::new();
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        write_snapshot(tmp.path(), &snap(today - Duration::days(100))).unwrap();
        write_snapshot(tmp.path(), &snap(today - Duration::days(95))).unwrap();
        write_snapshot(tmp.path(), &snap(today - Duration::days(50))).unwrap();
        let removed = prune(tmp.path(), today, 90).unwrap();
        assert_eq!(removed, 2);
        let remaining: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        assert_eq!(remaining.len(), 1);
        assert!(remaining[0]
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .contains("2026"));
    }

    #[test]
    fn prune_handles_missing_dir() {
        let tmp = TempDir::new();
        let missing = tmp.path().join("does-not-exist");
        let today = NaiveDate::from_ymd_opt(2026, 5, 19).unwrap();
        assert_eq!(prune(&missing, today, 90).unwrap(), 0);
    }

    // Minimal tempdir replacement so we don't pull in a heavy dev-dep.
    mod tempdir_minimal {
        use std::path::{Path, PathBuf};
        pub struct TempDir(PathBuf);
        impl TempDir {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicU64, Ordering};
                static COUNTER: AtomicU64 = AtomicU64::new(0);
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let n = COUNTER.fetch_add(1, Ordering::SeqCst);
                let p = std::env::temp_dir().join(format!("pr-hygiene-{nanos}-{n}"));
                std::fs::create_dir_all(&p).unwrap();
                Self(p)
            }
            pub fn path(&self) -> &Path {
                &self.0
            }
        }
        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }
}
