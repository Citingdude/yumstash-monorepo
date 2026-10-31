use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;

use super::super::super::{models::Recipe, repository};
use super::super::{database_error, not_found};
use crate::error::{ApiError, ErrorResponse};

#[utoipa::path(
    tag = "Recipes",
    get,
    path = "/api/recipes/{id}",
    params(("id" = i32, Path)),
    responses(
        (status = 200, body = Recipe),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Recipe>, ApiError> {
    Ok(Json(
        repository::get(&pool, id)
            .await
            .map_err(database_error)?
            .ok_or_else(not_found)?,
    ))
}
