use axum::{routing::get, Router};
use utoipa::OpenApi;

use super::{models::HealthResponse, use_cases};

#[derive(OpenApi)]
#[openapi(
    paths(use_cases::check_health::handler::health),
    components(schemas(HealthResponse))
)]
struct HealthApi;

pub fn router() -> Router {
    Router::new().route("/api/health", get(use_cases::check_health::handler::health))
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    HealthApi::openapi()
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
