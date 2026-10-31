pub(super) mod create_recipe;
pub(super) mod delete_recipe;
pub(super) mod get_recipe;
pub(super) mod list_recipes;
pub(super) mod update_recipe;

use crate::error::{error, ApiError};
use axum::http::StatusCode;

fn database_error(err: sqlx::Error) -> ApiError {
    tracing::error!(error = %err, "recipe database operation failed");
    error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Could not access recipes",
    )
}

fn not_found() -> ApiError {
    error(StatusCode::NOT_FOUND, "Recipe not found")
}
