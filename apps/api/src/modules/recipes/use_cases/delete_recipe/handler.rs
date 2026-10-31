use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use super::super::super::repository;
use super::super::{database_error, not_found};
use crate::error::{ApiError, ErrorResponse};

#[utoipa::path(
    tag = "Recipes",
    delete,
    path = "/api/recipes/{id}",
    params(("id" = i32, Path)),
    responses(
        (status = 204),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    if repository::delete(&pool, id)
        .await
        .map_err(database_error)?
    {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(not_found())
    }
}
