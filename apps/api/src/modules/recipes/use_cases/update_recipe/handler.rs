use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use super::super::super::{
    models::{Recipe, RecipeInput},
    repository,
};
use super::super::{database_error, not_found};
use crate::error::{error, ApiError, ErrorResponse};

#[utoipa::path(
    tag = "Recipes",
    put,
    path = "/api/recipes/{id}",
    params(("id" = i32, Path)),
    request_body = RecipeInput,
    responses(
        (status = 200, body = Recipe),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<RecipeInput>,
) -> Result<Json<Recipe>, ApiError> {
    input
        .validate()
        .map_err(|message| error(StatusCode::BAD_REQUEST, message))?;
    Ok(Json(
        repository::update(&pool, id, &input)
            .await
            .map_err(database_error)?
            .ok_or_else(not_found)?,
    ))
}
