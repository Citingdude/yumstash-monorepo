use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use super::super::super::{
    models::{Recipe, RecipeInput},
    repository,
};
use super::super::database_error;
use crate::error::{error, ApiError, ErrorResponse};

#[utoipa::path(
    tag = "Recipes",
    post,
    path = "/api/recipes",
    request_body = RecipeInput,
    responses(
        (status = 201, body = Recipe),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn create(
    State(pool): State<PgPool>,
    Json(input): Json<RecipeInput>,
) -> Result<(StatusCode, Json<Recipe>), ApiError> {
    input
        .validate()
        .map_err(|message| error(StatusCode::BAD_REQUEST, message))?;
    Ok((
        StatusCode::CREATED,
        Json(
            repository::create(&pool, &input)
                .await
                .map_err(database_error)?,
        ),
    ))
}
