use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Ingredient {
    pub id: i64,
    pub recipe_id: i64,
    pub name: String,
    pub amount: Option<String>,
    pub unit: Option<String>,
    pub note: Option<String>,
}
