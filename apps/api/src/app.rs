use std::time::Duration;

use axum::{response::Html, routing::get, Json, Router};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::modules;

pub fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .merge(modules::health::router())
        .merge(modules::recipes::router(pool))
        .route("/api/openapi.json", get(openapi))
        .route("/docs", get(docs))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}

async fn docs() -> Html<&'static str> {
    Html(include_str!("../docs/swagger.html"))
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(openapi_document())
}

pub fn openapi_document() -> utoipa::openapi::OpenApi {
    let mut doc = modules::health::openapi();
    doc.merge(modules::recipes::openapi());
    doc
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
