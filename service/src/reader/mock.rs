//! Stand-ins for GitHub on local ports, and what the reader's tests share.

use super::app::{AppAuthError, TokenSource};
use crate::config::Secret;
use crate::oidc::BoxFuture;
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
struct Fixed(Secret);

impl TokenSource for Fixed {
    fn token(&self) -> BoxFuture<'_, Result<Secret, AppAuthError>> {
        Box::pin(async move { Ok(self.0.clone()) })
    }
}

pub fn fixed_token(token: &str) -> Arc<dyn TokenSource> {
    Arc::new(Fixed(Secret::new(token)))
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
