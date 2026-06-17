use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

use super::{ingredient::Ingredient, nutrition::Nutrition, step::Step};

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct RecipeSummary {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub difficulty: Option<i64>,
    pub calories: Option<f64>,
    pub cover_image: Option<String>,
    pub source: String,
    pub ingredient_count: i64,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct RecipeRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub difficulty: Option<i64>,
    pub calories: Option<f64>,
    pub cover_image: Option<String>,
    pub source: String,
    pub source_path: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecipeDetail {
    #[serde(flatten)]
    pub recipe: RecipeRow,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<Step>,
    pub nutrition: Option<Nutrition>,
    pub tags: Vec<String>,
}
