use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use utoipa::OpenApi;

use crate::error::ErrorResponse;

use super::{
    models::{Recipe, RecipeInput},
    use_cases,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        use_cases::list_recipes::handler::list,
        use_cases::get_recipe::handler::get_one,
        use_cases::create_recipe::handler::create,
        use_cases::update_recipe::handler::update,
        use_cases::delete_recipe::handler::delete
    ),
    components(schemas(Recipe, RecipeInput, ErrorResponse))
)]
struct RecipesApi;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/recipes", get(use_cases::list_recipes::handler::list))
        .route(
            "/api/recipes",
            post(use_cases::create_recipe::handler::create),
        )
        .route(
            "/api/recipes/{id}",
            get(use_cases::get_recipe::handler::get_one),
        )
        .route(
            "/api/recipes/{id}",
            put(use_cases::update_recipe::handler::update),
        )
        .route(
            "/api/recipes/{id}",
            delete(use_cases::delete_recipe::handler::delete),
        )
        .with_state(pool)
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    RecipesApi::openapi()
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
