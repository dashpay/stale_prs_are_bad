//! GitHub, as far as sign-in needs it: exchange the code GitHub sent back
//! for a user token, ask once whose token it is, and revoke it. Behind a
//! trait, so tests sign in against a fake.
//!
//! The sign-in App has no permissions, so the token could read only public
//! data; it is still revoked at once, because nothing here needs it again.

use crate::config::{Secret, SignInConfig};
use crate::oidc::BoxFuture;
use reqwest::header::ACCEPT;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

/// Where the browser is sent to sign in.
pub const AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const API_URL: &str = "https://api.github.com";

/// A REST API version GitHub supports with no end date announced; `id` and
/// `login` of `/user` are the same in every version.
const API_VERSION: &str = "2026-03-10";

/// GitHub answers each call with a few hundred bytes; a far larger answer
/// is not GitHub's, and is not read.
pub const MAX_RESPONSE_BYTES: usize = 64 * 1024;

/// Per call. A sign-in makes three in a row and two database writes, all
/// inside the service's 30 s limit on a request.
const CALL_TIMEOUT: Duration = Duration::from_secs(6);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Who signed in, as GitHub says.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub login: String,
}

/// Why a call failed. Nothing here quotes GitHub's free text, a token or
/// the client secret, so it can be logged as it is.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GithubError {
    #[error("GitHub answered HTTP {0}")]
    Status(u16),
    /// The `error` code of a token answer, e.g. `bad_verification_code`;
    /// GitHub reports a refused exchange this way, with HTTP 200.
    #[error("GitHub refused the exchange: {0}")]
    Refused(String),
    #[error("unexpected answer from GitHub: {0}")]
    Malformed(&'static str),
    #[error("GitHub's answer is larger than {MAX_RESPONSE_BYTES} bytes")]
    TooLarge,
    #[error("could not reach GitHub: {0}")]
    Transport(String),
}

/// The three calls of a sign-in.
pub trait SignInApi: Send + Sync {
    /// Trade the code GitHub sent back, with the PKCE verifier its
    /// challenge was made from, for a user access token.
    fn exchange<'a>(
        &'a self,
        code: &'a str,
        verifier: &'a str,
    ) -> BoxFuture<'a, Result<Secret, GithubError>>;

    /// The account the token belongs to.
    fn user<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<GithubUser, GithubError>>;

    /// Revoke the token.
    fn revoke<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<(), GithubError>>;
}

/// GitHub itself, at its fixed URLs.
pub struct GithubSignIn {
    client: reqwest::Client,
    client_id: String,
    client_secret: Secret,
    redirect_uri: String,
    token_url: String,
    api_url: String,
}

impl GithubSignIn {
    pub fn new(cfg: &SignInConfig) -> anyhow::Result<Self> {
        Self::at(cfg, TOKEN_URL, API_URL)
    }

    /// Other URLs, for testing the calls themselves; production code has no
    /// way to send the client secret anywhere but GitHub.
    #[cfg(test)]
    fn with_urls(cfg: &SignInConfig, token_url: &str, api_url: &str) -> anyhow::Result<Self> {
        Self::at(cfg, token_url, api_url)
    }

    fn at(cfg: &SignInConfig, token_url: &str, api_url: &str) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(CALL_TIMEOUT)
            .connect_timeout(CONNECT_TIMEOUT)
            // A redirect would carry the request, credentials and all,
            // somewhere else; none is expected, so none is followed.
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("pr-hygiene-service")
            .build()?;
        Ok(Self {
            client,
            client_id: cfg.client_id.clone(),
            client_secret: cfg.client_secret.clone(),
            redirect_uri: cfg.redirect_uri(),
            token_url: token_url.to_string(),
            api_url: api_url.to_string(),
        })
    }
}

impl SignInApi for GithubSignIn {
    fn exchange<'a>(
        &'a self,
        code: &'a str,
        verifier: &'a str,
    ) -> BoxFuture<'a, Result<Secret, GithubError>> {
        Box::pin(async move {
            // In the body, never the URL: an error can quote its URL, and
            // this one would then carry the code, the verifier and the secret.
            let form = [
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.expose()),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("code_verifier", verifier),
            ];
            let res = self
                .client
                .post(&self.token_url)
                .header(ACCEPT, "application/json")
                .form(&form)
                .send()
                .await
                .map_err(transport)?;
            let body = success_body(res).await?;
            #[derive(Deserialize)]
            struct Answer {
                access_token: Option<String>,
                error: Option<String>,
            }
            let answer: Answer = serde_json::from_slice(&body)
                .map_err(|_| GithubError::Malformed("the token answer is not JSON"))?;
            if let Some(error) = answer.error {
                return Err(GithubError::Refused(error_code(&error)));
            }
            match answer.access_token {
                Some(token) if !token.is_empty() => Ok(Secret::new(token)),
                _ => Err(GithubError::Malformed("the token answer has no token")),
            }
        })
    }

    fn user<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<GithubUser, GithubError>> {
        Box::pin(async move {
            let res = self
                .client
                .get(format!("{}/user", self.api_url))
                .bearer_auth(token.expose())
                .header(ACCEPT, "application/vnd.github+json")
                .header("X-GitHub-Api-Version", API_VERSION)
                .send()
                .await
                .map_err(transport)?;
            let body = success_body(res).await?;
            serde_json::from_slice(&body)
                .map_err(|_| GithubError::Malformed("the user has no numeric id and login"))
        })
    }

    fn revoke<'a>(&'a self, token: &'a Secret) -> BoxFuture<'a, Result<(), GithubError>> {
        Box::pin(async move {
            // The client id is checked at start-up to need no escaping.
            let res = self
                .client
                .delete(format!(
                    "{}/applications/{}/token",
                    self.api_url, self.client_id
                ))
                .basic_auth(&self.client_id, Some(self.client_secret.expose()))
                .header(ACCEPT, "application/vnd.github+json")
                .header("X-GitHub-Api-Version", API_VERSION)
                .json(&json!({ "access_token": token.expose() }))
                .send()
                .await
                .map_err(transport)?;
            if !res.status().is_success() {
                return Err(GithubError::Status(res.status().as_u16()));
            }
            Ok(())
        })
    }
}

/// The error without its URL: messages stay free of anything a URL could
/// ever carry.
fn transport(e: reqwest::Error) -> GithubError {
    GithubError::Transport(e.without_url().to_string())
}

/// The body of a 2xx answer, read up to the limit.
async fn success_body(mut res: reqwest::Response) -> Result<Vec<u8>, GithubError> {
    if !res.status().is_success() {
        return Err(GithubError::Status(res.status().as_u16()));
    }
    if res
        .content_length()
        .is_some_and(|n| n > MAX_RESPONSE_BYTES as u64)
    {
        return Err(GithubError::TooLarge);
    }
    let mut body = Vec::new();
    while let Some(chunk) = res.chunk().await.map_err(transport)? {
        if body.len() + chunk.len() > MAX_RESPONSE_BYTES {
            return Err(GithubError::TooLarge);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

/// GitHub's error codes are short snake_case words; anything else is not
/// repeated, not even in a log.
fn error_code(s: &str) -> String {
    let plain =
        (1..=64).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_lowercase() || b == b'_');
    if plain {
        s.to_string()
    } else {
        "unrecognised".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Bytes;
    use axum::extract::Request;
    use axum::http::{HeaderMap, Method, StatusCode};
    use axum::response::{IntoResponse, Response};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    struct Seen {
        method: Method,
        path: String,
        query: Option<String>,
        headers: HeaderMap,
        body: String,
    }

    /// A stand-in for GitHub on a local port, answering every request with
    /// `answer` and keeping what it was sent.
    async fn github(answer: fn() -> Response) -> (String, Arc<Mutex<Vec<Seen>>>) {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let log = seen.clone();
        let app = axum::Router::new().fallback(move |req: Request| {
            let log = log.clone();
            async move {
                let (parts, body) = req.into_parts();
                let body: Bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
                log.lock().unwrap().push(Seen {
                    method: parts.method,
                    path: parts.uri.path().to_string(),
                    query: parts.uri.query().map(String::from),
                    headers: parts.headers,
                    body: String::from_utf8_lossy(&body).into_owned(),
                });
                answer()
            }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await });
        (format!("http://{addr}"), seen)
    }

    fn cfg() -> SignInConfig {
        SignInConfig {
            client_id: "Iv23liTEST".into(),
            client_secret: Secret::new("the-client-secret"),
            origin: "https://hygiene.example.org".into(),
        }
    }

    async fn client(answer: fn() -> Response) -> (GithubSignIn, Arc<Mutex<Vec<Seen>>>) {
        let (url, seen) = github(answer).await;
        let api = GithubSignIn::with_urls(&cfg(), &format!("{url}/login/oauth/access_token"), &url)
            .unwrap();
        (api, seen)
    }

    fn json_answer(body: &'static str) -> Response {
        (
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response()
    }

    #[tokio::test]
    async fn the_code_is_exchanged_in_the_body_with_the_verifier_and_fixed_callback() {
        let (api, seen) = client(|| {
            json_answer(r#"{"access_token":"ghu_abc","token_type":"bearer","scope":""}"#)
        })
        .await;
        let token = api.exchange("the-code", "the-verifier").await.unwrap();
        assert_eq!(token.expose(), "ghu_abc");
        let seen = seen.lock().unwrap()[0].clone();
        assert_eq!(seen.method, Method::POST);
        assert_eq!(seen.path, "/login/oauth/access_token");
        assert_eq!(seen.query, None, "nothing secret in the URL");
        assert_eq!(seen.headers["accept"], "application/json");
        let form: Vec<(String, String)> =
            reqwest::Url::parse(&format!("http://form.invalid/?{}", seen.body))
                .unwrap()
                .query_pairs()
                .into_owned()
                .collect();
        let get = |k: &str| form.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
        assert_eq!(get("client_id"), Some("Iv23liTEST"));
        assert_eq!(get("client_secret"), Some("the-client-secret"));
        assert_eq!(get("code"), Some("the-code"));
        assert_eq!(get("code_verifier"), Some("the-verifier"));
        assert_eq!(
            get("redirect_uri"),
            Some("https://hygiene.example.org/auth/callback")
        );
    }

    /// GitHub refuses a bad code with HTTP 200 and an `error` field; that
    /// is a failure, and its description is not carried into the error.
    #[tokio::test]
    async fn an_error_in_a_200_answer_is_a_failure() {
        let (api, _) = client(|| {
            json_answer(
                r#"{"error":"bad_verification_code","error_description":"<b>sneaky</b>","error_uri":"https://evil.example"}"#,
            )
        })
        .await;
        let err = api.exchange("c", "v").await.unwrap_err();
        assert_eq!(err, GithubError::Refused("bad_verification_code".into()));

        let (api, _) = client(|| json_answer(r#"{"error":"<script>alert(1)</script>"}"#)).await;
        let err = api.exchange("c", "v").await.unwrap_err();
        assert_eq!(err, GithubError::Refused("unrecognised".into()));

        let (api, _) = client(|| json_answer(r#"{"token_type":"bearer"}"#)).await;
        assert!(matches!(
            api.exchange("c", "v").await,
            Err(GithubError::Malformed(_))
        ));
    }

    /// The client secret goes to GitHub and nowhere else, even if GitHub
    /// (or something posing as it) answers with a redirect.
    #[tokio::test]
    async fn a_redirect_is_not_followed() {
        let (api, seen) = client(|| {
            (
                StatusCode::TEMPORARY_REDIRECT,
                [(axum::http::header::LOCATION, "/elsewhere")],
            )
                .into_response()
        })
        .await;
        assert_eq!(
            api.exchange("c", "v").await.unwrap_err(),
            GithubError::Status(307)
        );
        assert_eq!(seen.lock().unwrap().len(), 1, "one request, not two");
    }

    #[tokio::test]
    async fn an_oversized_answer_is_not_read() {
        let (api, _) = client(|| " ".repeat(MAX_RESPONSE_BYTES + 1).into_response()).await;
        assert_eq!(
            api.user(&Secret::new("t")).await.unwrap_err(),
            GithubError::TooLarge
        );
    }

    #[tokio::test]
    async fn the_user_is_read_with_the_token_and_only_id_and_login_kept() {
        let (api, seen) = client(|| {
            json_answer(r#"{"id":42,"login":"alice","name":"Alice","email":"a@example.org"}"#)
        })
        .await;
        let user = api.user(&Secret::new("ghu_abc")).await.unwrap();
        assert_eq!(
            user,
            GithubUser {
                id: 42,
                login: "alice".into()
            }
        );
        let seen = seen.lock().unwrap()[0].clone();
        assert_eq!((seen.method, seen.path.as_str()), (Method::GET, "/user"));
        assert_eq!(seen.headers["authorization"], "Bearer ghu_abc");
        assert!(seen.headers.contains_key("user-agent"));

        let (api, _) = client(|| StatusCode::UNAUTHORIZED.into_response()).await;
        assert_eq!(
            api.user(&Secret::new("t")).await.unwrap_err(),
            GithubError::Status(401)
        );
    }

    #[tokio::test]
    async fn the_token_is_revoked_with_the_client_credentials() {
        let (api, seen) = client(|| StatusCode::NO_CONTENT.into_response()).await;
        api.revoke(&Secret::new("ghu_abc")).await.unwrap();
        let seen = seen.lock().unwrap()[0].clone();
        assert_eq!(seen.method, Method::DELETE);
        assert_eq!(seen.path, "/applications/Iv23liTEST/token");
        use base64::Engine;
        let basic =
            base64::engine::general_purpose::STANDARD.encode("Iv23liTEST:the-client-secret");
        assert_eq!(
            seen.headers["authorization"],
            format!("Basic {basic}").as_str()
        );
        let body: serde_json::Value = serde_json::from_str(&seen.body).unwrap();
        assert_eq!(body, json!({ "access_token": "ghu_abc" }));

        let (api, _) = client(|| StatusCode::UNPROCESSABLE_ENTITY.into_response()).await;
        assert_eq!(
            api.revoke(&Secret::new("t")).await.unwrap_err(),
            GithubError::Status(422)
        );
    }
}
