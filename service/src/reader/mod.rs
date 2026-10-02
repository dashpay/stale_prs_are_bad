//! The engine's way to GitHub: the HTTP side of the review engine's
//! [`Transport`](pr_hygiene_engine::evidence::Transport), and what it needs.
//!
//! - [`HttpTransport`]: the engine's calls made over HTTPS as `gh api` made
//!   them for Python — the same requests, headers and pages — with its
//!   failures sorted into the ones worth one more try and the rest.
//! - [`ReadOnly`]: a second layer that lets through only what reads: a
//!   `GET` under an allowed repository, or one of the engine's own GraphQL
//!   queries. Anything else is refused, never sent.
//! - [`InstallationTokens`]: the reader App's installation token, asked for
//!   with read permissions only, cached, and renewed before it expires.
//! - [`StatusPage`]: the review system's public status page, through a
//!   client that never carries the App's token.
//!
//! The engine is synchronous and runs on a blocking thread; every request
//! here is made on the service's own runtime from that thread
//! (see [`HttpTransport`]).

pub mod app;
pub mod read_only;
pub mod status_page;
pub mod transport;

#[cfg(test)]
mod end_to_end;
#[cfg(test)]
pub(crate) mod mock;

pub use app::{AppAuthError, AppKey, InstallationTokens, TokenSource};
pub use read_only::{ReadOnly, NOT_A_READ};
pub use status_page::StatusPage;
pub use transport::HttpTransport;

/// GitHub's API: the only origin the reader sends its token to.
pub const API_URL: &str = "https://api.github.com";

/// The REST API version the reader asks for: the one `gh api` sends, under
/// which the engine's recordings were made. REST answers change shape
/// between versions, and the engine reads them as recorded.
pub const API_VERSION: &str = "2022-11-28";

const USER_AGENT: &str = "pr-hygiene-service";

/// How long a connection to GitHub may take to open.
const CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

/// A client for GitHub's API: `timeout` per request, and no redirect
/// followed, so that a request and its token reach the URL asked for and
/// nowhere else.
fn github_client(timeout: std::time::Duration) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(timeout)
        .connect_timeout(CONNECT_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .user_agent(USER_AGENT)
        .build()
}
