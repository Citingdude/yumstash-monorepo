use axum::{routing::get, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
struct HealthResponse {
    status: String,
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses((status = 200, description = "API is running", body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into() })
}

#[derive(OpenApi)]
#[openapi(paths(health), components(schemas(HealthResponse)))]
struct HealthApi;

pub fn router() -> Router {
    Router::new().route("/api/health", get(health))
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    HealthApi::openapi()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_is_available() {
        let response = router()
            .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
