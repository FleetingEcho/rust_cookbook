use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::FromRow;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::{error::AppError, AppState};

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct CategoryCount {
    /// 分类名
    pub name: String,
    /// 该分类的菜谱数量
    pub count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    /// 菜谱总数
    pub total_recipes: i64,
    /// 平均热量（大卡）
    pub avg_calories: Option<f64>,
    /// 按分类的菜谱数量
    pub by_category: HashMap<String, i64>,
    /// 按数据来源的菜谱数量
    pub sources: HashMap<String, i64>,
}

/// 获取分类列表
#[utoipa::path(
    get,
    path = "/api/v1/categories",
    responses(
        (status = 200, description = "分类列表（按菜谱数量降序）", body = Vec<CategoryCount>),
    ),
    tag = "分类与统计"
)]
pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryCount>>, AppError> {
    let cats = sqlx::query_as::<_, CategoryCount>(
        "SELECT category AS name, COUNT(*) AS count \
         FROM recipes GROUP BY category ORDER BY count DESC",
    )
    .fetch_all(&state.db)
    .await?;
    Ok(Json(cats))
}

/// 获取全局统计数据
#[utoipa::path(
    get,
    path = "/api/v1/stats",
    responses(
        (status = 200, description = "全局统计", body = StatsResponse),
        (status = 500, description = "服务器错误", body = ErrorResponse),
    ),
    tag = "分类与统计"
)]
pub async fn stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, AppError> {
    let (total, avg_calories): (i64, Option<f64>) = sqlx::query_as(
        "SELECT COUNT(*), ROUND(AVG(calories), 1) FROM recipes",
    )
    .fetch_one(&state.db)
    .await?;

    let by_category: Vec<(String, i64)> = sqlx::query_as(
        "SELECT category, COUNT(*) FROM recipes GROUP BY category ORDER BY COUNT(*) DESC",
    )
    .fetch_all(&state.db)
    .await?;

    let by_source: Vec<(String, i64)> = sqlx::query_as(
        "SELECT source, COUNT(*) FROM recipes GROUP BY source",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(StatsResponse {
        total_recipes: total,
        avg_calories,
        by_category: by_category.into_iter().collect(),
        sources: by_source.into_iter().collect(),
    }))
}
