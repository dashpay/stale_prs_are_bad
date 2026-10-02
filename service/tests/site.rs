//! The page, served from the service's own origin, reading the same data the
//! analyzer writes — so the page that ran on GitHub Pages runs unchanged.

mod common;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use common::*;
use pr_hygiene::dashboard::Dashboard;
use tower::ServiceExt;

#[tokio::test]
async fn the_page_reads_the_merged_view_in_the_analyzers_own_shape() {
    let app = app();
    let sent = snapshot(5);
    ingest(&app, &sent).await;
    let res = get(&app, "/dashboard.json").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key(header::ETAG));
    let got: Dashboard =
        serde_json::from_value(json_body(res).await).expect("the analyzer's contract");
    assert_eq!(got.schema_version, sent.schema_version);
    assert_eq!(got.prs.len(), sent.prs.len());
    assert_eq!(got.people.len(), sent.people.len());
}

#[tokio::test]
async fn the_page_is_served_with_its_security_headers_and_nothing_outside_it() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("index.html"),
        "<!doctype html><title>PR Hygiene</title>",
    )
    .unwrap();
    let outside = dir.path().parent().unwrap().join("outside-the-site.txt");
    std::fs::write(&outside, "secret").unwrap();
    let site = pr_hygiene_service::app::site(dir.path().to_path_buf());

    let res = site
        .clone()
        .oneshot(Request::get("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let csp = res.headers()[header::CONTENT_SECURITY_POLICY]
        .to_str()
        .unwrap();
    // A meta tag cannot forbid framing; the header can.
    assert!(csp.contains("frame-ancestors 'none'"), "{csp}");
    assert!(csp.contains("require-trusted-types-for 'script'"), "{csp}");
    assert_eq!(res.headers()[header::X_CONTENT_TYPE_OPTIONS], "nosniff");
    assert_eq!(res.headers()[header::REFERRER_POLICY], "no-referrer");

    for path in ["/../outside-the-site.txt", "/%2e%2e/outside-the-site.txt"] {
        let res = site
            .clone()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_ne!(res.status(), StatusCode::OK, "{path} escaped the site");
    }
    std::fs::remove_file(outside).unwrap();
}
