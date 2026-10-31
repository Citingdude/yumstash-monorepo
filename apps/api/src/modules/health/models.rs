use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub(super) struct HealthResponse {
    pub(super) status: String,
}
