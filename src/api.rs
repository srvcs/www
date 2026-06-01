use axum::{
    http::{header, HeaderMap},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

const INDEX_HTML: &str = include_str!("../static/index.html");
const LOGO_PNG: &[u8] = include_bytes!("../static/assets/srvcs-logo.png");
const SOCIAL_PNG: &[u8] = include_bytes!("../static/assets/srvcs-social.png");
const APPLE_TOUCH_ICON_PNG: &[u8] = include_bytes!("../static/apple-touch-icon.png");
const SITE_WEBMANIFEST: &str = include_str!("../static/site.webmanifest");
const SERVICES_JSON: &str = include_str!("../static/services.json");
const ROBOTS_TXT: &str = "User-agent: *\nAllow: /\n\nSitemap: https://srvcs.cloud/sitemap.xml\n";
const SITEMAP_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://srvcs.cloud/</loc>
  </url>
</urlset>
"#;

#[derive(Serialize, ToSchema)]
pub struct Info {
    pub service: &'static str,
}

#[utoipa::path(get, path = "/", responses((status = 200, body = Info)))]
pub async fn index(headers: HeaderMap) -> Response {
    if wants_html(&headers) {
        Html(INDEX_HTML).into_response()
    } else {
        Json(Info {
            service: "srvcs-www",
        })
        .into_response()
    }
}

fn wants_html(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|accept| {
            accept
                .split(',')
                .any(|part| part.trim().starts_with("text/html"))
        })
}

pub async fn logo_png() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        LOGO_PNG,
    )
}

pub async fn social_png() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        SOCIAL_PNG,
    )
}

pub async fn apple_touch_icon_png() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        APPLE_TOUCH_ICON_PNG,
    )
}

pub async fn site_webmanifest() -> impl IntoResponse {
    (
        [
            (
                header::CONTENT_TYPE,
                "application/manifest+json; charset=utf-8",
            ),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        SITE_WEBMANIFEST,
    )
}

#[utoipa::path(
    get,
    path = "/services.json",
    responses((status = 200, description = "Service catalog and dependency graph JSON"))
)]
pub async fn services_json() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        SERVICES_JSON,
    )
}

pub async fn robots_txt() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        ROBOTS_TXT,
    )
}

pub async fn sitemap_xml() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "application/xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        SITEMAP_XML,
    )
}

#[derive(OpenApi)]
#[openapi(paths(index, services_json), components(schemas(Info)))]
pub struct ApiDoc;

/// Serve OpenAPI document
pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_documents_index() {
        assert!(ApiDoc::openapi().paths.paths.contains_key("/"));
    }

    #[tokio::test]
    async fn index_returns_service_info() {
        let response = index(HeaderMap::new()).await;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
