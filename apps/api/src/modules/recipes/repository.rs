use super::models::{Recipe, RecipeInput};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>(
        "SELECT id, title, description, ingredients, instructions FROM recipes ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await
}
pub async fn get(pool: &PgPool, id: i32) -> Result<Option<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>(
        "SELECT id, title, description, ingredients, instructions FROM recipes WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
pub async fn create(pool: &PgPool, input: &RecipeInput) -> Result<Recipe, sqlx::Error> {
    sqlx::query_as::<_, Recipe>("INSERT INTO recipes (title, description, ingredients, instructions) VALUES ($1, $2, $3, $4) RETURNING id, title, description, ingredients, instructions")
        .bind(input.title.trim()).bind(&input.description).bind(&input.ingredients).bind(&input.instructions).fetch_one(pool).await
}
pub async fn update(
    pool: &PgPool,
    id: i32,
    input: &RecipeInput,
) -> Result<Option<Recipe>, sqlx::Error> {
    sqlx::query_as::<_, Recipe>("UPDATE recipes SET title = $2, description = $3, ingredients = $4, instructions = $5 WHERE id = $1 RETURNING id, title, description, ingredients, instructions")
        .bind(id).bind(input.title.trim()).bind(&input.description).bind(&input.ingredients).bind(&input.instructions).fetch_optional(pool).await
}
pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM recipes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected()
        > 0)
}
