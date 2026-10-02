//! Stand-ins for GitHub on local ports, and what the reader's tests share.

use super::app::{GivenToken, TokenSource};
use crate::config::Secret;
use axum::body::Bytes;
use axum::extract::Request;
use axum::http::{header, HeaderMap, Method};
use axum::response::{IntoResponse, Response};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

/// A request as the stand-in received it.
#[derive(Debug, Clone)]
pub struct Seen {
    pub method: Method,
    pub path_and_query: String,
    pub headers: HeaderMap,
    pub body: String,
}

impl Seen {
    /// The origin the request was sent to: the stand-in's own.
    pub fn origin(&self) -> String {
        let host = self
            .headers
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .unwrap_or_default();
        format!("http://{host}")
    }

    /// The path, without its query.
    pub fn path(&self) -> &str {
        self.path_and_query.split('?').next().unwrap_or_default()
    }
}

/// A stand-in for GitHub: its origin, and every request it was sent.
pub struct Mock {
    pub url: String,
    seen: Arc<Mutex<Vec<Seen>>>,
}

impl Mock {
    pub fn seen(&self) -> Vec<Seen> {
        self.seen.lock().unwrap().clone()
    }
}

/// A stand-in on 127.0.0.1 answering each request with `answer`.
pub async fn serve(answer: impl Fn(&Seen) -> Response + Send + Sync + 'static) -> Mock {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let log = seen.clone();
    let answer = Arc::new(answer);
    let app = axum::Router::new().fallback(move |req: Request| {
        let (log, answer) = (log.clone(), answer.clone());
        async move {
            let (parts, body) = req.into_parts();
            let body: Bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
            let seen = Seen {
                method: parts.method,
                path_and_query: parts
                    .uri
                    .path_and_query()
                    .map(|p| p.as_str().to_owned())
                    .unwrap_or_default(),
                headers: parts.headers,
                body: String::from_utf8_lossy(&body).into_owned(),
            };
            log.lock().unwrap().push(seen.clone());
            answer(&seen)
        }
    });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await });
    Mock {
        url: format!("http://{addr}"),
        seen,
    }
}

/// A JSON answer with status 200.
pub fn json_answer(body: &str) -> Response {
    (
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body.to_owned(),
    )
        .into_response()
}

/// The `gh api` command a read became: its arguments and its stdin.
type GhCommand = (Vec<String>, Option<String>);

/// One answer `gh` gave a recorded read.
#[derive(Clone)]
struct GhAnswer {
    exit: i64,
    stdout: String,
    stderr: String,
}

/// The reads of a recording's `calls.jsonl`, by `gh api` command.
fn recorded_reads(calls: &str) -> std::collections::HashMap<GhCommand, Vec<GhAnswer>> {
    let mut reads: std::collections::HashMap<GhCommand, Vec<GhAnswer>> = Default::default();
    for line in calls.lines().filter(|line| !line.is_empty()) {
        let entry: serde_json::Value = serde_json::from_str(line).unwrap();
        if entry["kind"] != "read" {
            continue;
        }
        let args = entry["args"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a.as_str().unwrap().to_owned())
            .collect();
        let stdin = entry["stdin"].as_str().map(str::to_owned);
        let text = |key: &str| entry[key].as_str().unwrap_or_default().to_owned();
        let answer = GhAnswer {
            // `gh` that could not run, or ran out of time, answered nothing.
            exit: entry["exit"].as_i64().unwrap_or(1),
            stdout: text("stdout"),
            stderr: text("stderr"),
        };
        reads.entry((args, stdin)).or_default().push(answer);
    }
    reads
}

/// GitHub as a boundary recording saw it: each read the recording holds,
/// answered as `gh` answered Python. A REST route by its path and query; a
/// listing page by page, each page linked to the next as GitHub links them;
/// a GraphQL query by its body. The answers to one read in the order
/// recorded, the last again once they run out. A read `gh` reported as
/// failed answers with the HTTP status its message names. Anything the
/// recording does not hold is a 404, and anything that is not a read a 405.
pub fn as_recorded(calls: &str) -> impl Fn(&Seen) -> Response + Send + Sync + 'static {
    use axum::http::StatusCode;
    use pr_hygiene_engine::pycompat::{py_dumps, py_loads, PyValue};
    let reads = recorded_reads(calls);
    let served: Mutex<std::collections::HashMap<GhCommand, usize>> = Mutex::default();
    // The next answer to `command`, or with `again`, the one last given.
    let take = move |command: &GhCommand, again: bool| -> Option<GhAnswer> {
        let answers = reads.get(command)?;
        let mut served = served.lock().unwrap();
        let next = served.entry(command.clone()).or_insert(0);
        if !again {
            *next += 1;
        }
        let at = next.saturating_sub(1).min(answers.len() - 1);
        Some(answers[at].clone())
    };
    let failed = |answer: &GhAnswer| {
        let code = answer
            .stderr
            .split("(HTTP ")
            .nth(1)
            .and_then(|rest| rest.get(..3))
            .and_then(|code| code.parse::<u16>().ok())
            .and_then(|code| StatusCode::from_u16(code).ok())
            .unwrap_or(StatusCode::BAD_GATEWAY);
        let message = serde_json::json!({ "message": answer.stderr }).to_string();
        (code, message).into_response()
    };
    move |seen: &Seen| {
        let words = |words: &[&str]| words.iter().map(|w| (*w).to_owned()).collect::<Vec<_>>();
        if (seen.method.as_str(), seen.path()) == ("POST", "/graphql") {
            let command = (
                words(&["--method", "POST", "graphql", "--input", "-"]),
                Some(seen.body.clone()),
            );
            return match take(&command, false) {
                Some(answer) if answer.exit == 0 || !answer.stdout.trim().is_empty() => {
                    json_answer(&answer.stdout)
                }
                Some(answer) => failed(&answer),
                None => StatusCode::NOT_FOUND.into_response(),
            };
        }
        if seen.method != Method::GET {
            return StatusCode::METHOD_NOT_ALLOWED.into_response();
        }
        let asked = seen.path_and_query.trim_start_matches('/');
        let (route, page) = match asked.rsplit_once("&page=") {
            Some((route, page)) => (route, page.parse::<usize>().unwrap_or(0)),
            None => (asked, 1),
        };
        if page == 1 {
            if let Some(answer) = take(&(words(&["--method", "GET", route]), None), false) {
                return match answer.exit {
                    0 => json_answer(&answer.stdout),
                    _ => failed(&answer),
                };
            }
        }
        let listing = (
            words(&["--method", "GET", route, "--paginate", "--slurp"]),
            None,
        );
        let Some(answer) = take(&listing, page > 1) else {
            return StatusCode::NOT_FOUND.into_response();
        };
        if answer.exit != 0 {
            return failed(&answer);
        }
        let Ok(PyValue::List(pages)) = py_loads(&answer.stdout) else {
            return StatusCode::BAD_GATEWAY.into_response();
        };
        let Some(this) = page.checked_sub(1).and_then(|at| pages.get(at)) else {
            return StatusCode::NOT_FOUND.into_response();
        };
        let mut response = json_answer(&py_dumps(this, false, None, None).unwrap());
        if page < pages.len() {
            let link = format!(
                "<{}/{route}&page={}>; rel=\"next\"",
                seen.origin(),
                page + 1
            );
            response
                .headers_mut()
                .insert(header::LINK, link.parse().unwrap());
        }
        response
    }
}

/// A stand-in that writes raw bytes: the answers no well-behaved server
/// gives.
pub struct Raw {
    pub url: String,
    connections: Arc<AtomicUsize>,
}

impl Raw {
    /// How many connections it has accepted.
    pub fn connections(&self) -> usize {
        self.connections.load(Ordering::SeqCst)
    }
}

/// The bytes a raw stand-in writes back for a request's bytes.
type RawAnswer = fn(&[u8]) -> Vec<u8>;

/// Plain threads and sockets: what it writes is exactly what is sent.
fn raw_with(answer: Option<RawAnswer>) -> Raw {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let connections = Arc::new(AtomicUsize::new(0));
    let count = connections.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else {
                return;
            };
            count.fetch_add(1, Ordering::SeqCst);
            std::thread::spawn(move || {
                let mut request = Vec::new();
                let mut buf = [0u8; 4096];
                while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                    match stream.read(&mut buf) {
                        Ok(0) | Err(_) => return,
                        Ok(n) => request.extend_from_slice(&buf[..n]),
                    }
                }
                match answer {
                    Some(answer) => {
                        let _ = stream.write_all(&answer(&request));
                        let _ = stream.shutdown(std::net::Shutdown::Both);
                    }
                    // Holds the connection open and says nothing.
                    None => std::thread::sleep(std::time::Duration::from_secs(30)),
                }
            });
        }
    });
    Raw {
        url: format!("http://{addr}"),
        connections,
    }
}

/// A stand-in answering every request with `answer`'s bytes, then closing
/// the connection.
pub async fn raw(answer: RawAnswer) -> Raw {
    raw_with(Some(answer))
}

/// A stand-in that accepts every connection and never answers.
pub async fn raw_silent() -> Raw {
    raw_with(None)
}

/// One token for every request.
pub fn fixed_token(token: &str) -> Arc<dyn TokenSource> {
    Arc::new(GivenToken::new(Secret::new(token)))
}

/// `f` on a blocking thread, as the engine runs.
pub async fn blocking<R: Send + 'static>(f: impl FnOnce() -> R + Send + 'static) -> R {
    tokio::task::spawn_blocking(f).await.unwrap()
}

/// An App key, made once per test run.
pub struct TestKey {
    pub private: rsa::RsaPrivateKey,
    /// The private key as GitHub issues it: PKCS#1 PEM.
    pub pem: String,
    /// The public key, PKCS#1 DER, to check signatures with.
    pub public_der: Vec<u8>,
}

pub fn test_key() -> &'static TestKey {
    use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
    static KEY: OnceLock<TestKey> = OnceLock::new();
    KEY.get_or_init(|| {
        let private = rsa::RsaPrivateKey::new(&mut rand::thread_rng(), 2048).unwrap();
        let pem = private.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();
        let public_der = private
            .to_public_key()
            .to_pkcs1_der()
            .unwrap()
            .as_bytes()
            .to_vec();
        TestKey {
            private,
            pem,
            public_der,
        }
    })
}
