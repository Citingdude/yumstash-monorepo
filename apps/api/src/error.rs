use axum::{http::StatusCode, Json};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    message: String,
}

pub type ApiError = (StatusCode, Json<ErrorResponse>);

pub fn error(status: StatusCode, message: &str) -> ApiError {
    (status, Json(ErrorResponse { message: message.into() }))
}
