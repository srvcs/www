use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use srvcs_www::{health, router, telemetry};
use tower::ServiceExt;

async fn status_of(uri: &str) -> StatusCode {
    let app = router(telemetry::metrics_handle_for_tests());
    app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

#[tokio::test]
async fn index_ok() {
    assert_eq!(status_of("/").await, StatusCode::OK);
}

#[tokio::test]
async fn index_serves_json_identity_to_api_callers() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let content_type = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    assert!(content_type.starts_with("application/json"));

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["service"], "srvcs-www");
}

#[tokio::test]
async fn healthz_ok() {
    assert_eq!(status_of("/healthz").await, StatusCode::OK);
}

#[tokio::test]
async fn readyz_reflects_state() {
    health::set_ready(true);
    assert_eq!(status_of("/readyz").await, StatusCode::OK);
}

#[tokio::test]
async fn metrics_ok() {
    assert_eq!(status_of("/metrics").await, StatusCode::OK);
}

#[tokio::test]
async fn openapi_ok() {
    assert_eq!(status_of("/openapi.json").await, StatusCode::OK);
}

#[tokio::test]
async fn index_serves_html_to_browsers() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header(header::ACCEPT, "text/html")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let content_type = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    assert!(content_type.starts_with("text/html"));

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(html.contains("srvcs.cloud"));
    assert!(html.contains("Building focused, composable services"));
    assert!(html.contains("Flake-backed runtime"));
    assert!(html.contains("The distributed standard library."));
    assert!(html.contains("serviceSearch"));
    assert!(html.contains("Dependency graph"));
    assert!(html.contains("services.json"));
    assert!(html.contains("Bugs &amp; service proposals"));
    assert!(html.contains("Questions &amp; architecture debates"));
    assert!(html.contains("Service Proposal awaiting ARB review"));
    assert!(html.contains(r#"<link rel="canonical" href="https://srvcs.cloud/" />"#));
    assert!(html.contains(r#"<meta property="og:type" content="website" />"#));
    assert!(html.contains(r#"<meta property="og:image:width" content="1200" />"#));
    assert!(html.contains(r#"<meta name="twitter:card" content="summary_large_image" />"#));
    assert!(html.contains(
        r#"<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />"#
    ));
    assert!(html.contains(r#"<link rel="manifest" href="/site.webmanifest" />"#));
    assert!(html.contains("new Date().getFullYear()"));
}

#[tokio::test]
async fn logo_asset_ok() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/assets/srvcs-logo.png")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get(header::CONTENT_TYPE).unwrap(),
        "image/png"
    );
}

#[tokio::test]
async fn social_metadata_assets_ok() {
    let app = router(telemetry::metrics_handle_for_tests());
    for (uri, content_type) in [
        ("/assets/srvcs-social.png", "image/png"),
        ("/apple-touch-icon.png", "image/png"),
        (
            "/site.webmanifest",
            "application/manifest+json; charset=utf-8",
        ),
        ("/services.json", "application/json; charset=utf-8"),
    ] {
        let res = app
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK, "{uri}");
        assert_eq!(
            res.headers().get(header::CONTENT_TYPE).unwrap(),
            content_type,
            "{uri}"
        );
    }
}

#[tokio::test]
async fn service_catalog_ok() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/services.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["schemaVersion"], 1);
    assert_eq!(json["serviceCount"], 201);
    assert!(json["services"].as_array().unwrap().len() >= 201);
    assert!(json["graph"]["edges"].as_array().unwrap().len() >= 300);
}

#[tokio::test]
async fn generates_request_id_when_absent() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        res.headers().contains_key("x-request-id"),
        "response must carry a generated x-request-id"
    );
}
