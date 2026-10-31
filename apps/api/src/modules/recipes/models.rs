use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct Recipe {
    pub(super) id: i32,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) ingredients: String,
    pub(super) instructions: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecipeInput {
    pub(super) title: String,
    pub(super) description: String,
    pub(super) ingredients: String,
    pub(super) instructions: String,
}

impl RecipeInput {
    pub(super) fn validate(&self) -> Result<(), &'static str> {
        if self.title.trim().is_empty() {
            return Err("Title is required");
        }
        if self.title.chars().count() > 200 {
            return Err("Title must be at most 200 characters");
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "model_tests.rs"]
mod tests;
