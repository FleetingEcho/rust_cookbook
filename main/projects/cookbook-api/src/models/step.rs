use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Step {
    pub id: i64,
    pub recipe_id: i64,
    pub step_order: i64,
    pub content: Option<String>,
    pub image_url: Option<String>,
}
