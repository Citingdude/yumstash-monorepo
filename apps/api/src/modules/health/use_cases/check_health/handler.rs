use axum::Json;

use super::super::super::models::HealthResponse;

#[utoipa::path(
    tag = "Health",
    get,
    path = "/api/health",
    responses((status = 200, description = "API is running", body = HealthResponse))
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into() })
}
