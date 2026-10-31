use axum::{extract::State, Json};
use sqlx::PgPool;

use super::super::super::{models::Recipe, repository};
use super::super::database_error;
use crate::error::{ApiError, ErrorResponse};

#[utoipa::path(
    tag = "Recipes", 
    get,
    path = "/api/recipes", 
    responses(
        (status = 200, body = [Recipe]), 
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn list(State(pool): State<PgPool>) -> Result<Json<Vec<Recipe>>, ApiError> {
    Ok(Json(
        repository::list(&pool)
            .await
            .map_err(database_error)?,
    ))
}
