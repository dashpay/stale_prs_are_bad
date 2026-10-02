//! The read-only PR Hygiene service. The scheduled workflow posts the
//! analyzer's snapshot to `/ingest` with a GitHub OIDC token; the service
//! keeps each repository's last good data in SQLite and serves it, publicly,
//! under `/api/v1`. It holds no credential with access to any repository.
//! It calls GitHub only to fetch GitHub's public token-signing keys and,
//! when sign-in is on, to sign people in through an App with no permissions.
//!
//! `reader` is how the review engine will read GitHub, through a read-only
//! App, from a process of its own: the server never holds that App's key.
//! `differential` holds the engine to the Python engine's recorded runs by
//! reading the same pull requests live, through `reader`, for the
//! scheduled differential job (`src/bin/differential-live.rs`).

pub mod api;
pub mod app;
pub mod auth;
pub mod config;
pub mod differential;
pub mod digest;
pub mod github;
pub mod oidc;
pub mod reader;
pub mod snapshot;
pub mod speed;
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
