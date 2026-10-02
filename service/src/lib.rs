//! The read-only PR Hygiene service. The scheduled workflow posts the
//! analyzer's snapshot to `/ingest` with a GitHub OIDC token; the service
//! keeps each repository's last good data in SQLite and serves it, publicly,
//! under `/api/v1`. It holds no GitHub credential and never calls GitHub,
//! except to fetch GitHub's public token-signing keys.

pub mod snapshot;
pub mod store;
pub mod view;

#[cfg(test)]
pub(crate) mod testdata {
    use chrono::TimeDelta;
    use pr_hygiene::dashboard::Dashboard;

    /// The analyzer's own end-to-end output, as its snapshot test records
    /// it: the real contract, two repositories, one without engine data.
    pub fn fixture() -> Dashboard {
        let snap = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../tests/snapshots/end_to_end__dashboard.snap"
        ))
        .unwrap();
        let body = snap.splitn(3, "---\n").nth(2).expect("insta front matter");
        serde_json::from_str(body).unwrap()
    }

    pub fn minutes(n: i64) -> TimeDelta {
        TimeDelta::minutes(n)
    }
}
