use std::time::Duration;

use axum::{routing::get, Json, Router};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::features;

pub fn router() -> Router {
    Router::new()
        .merge(features::health::router())
        .route("/api/openapi.json", get(openapi))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(features::health::openapi())
}
