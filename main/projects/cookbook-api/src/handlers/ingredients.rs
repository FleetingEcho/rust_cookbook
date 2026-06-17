use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

use crate::{
    error::AppError,
    models::pagination::{PagedResult, PaginationParams},
    AppState,
};

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct IngredientSummary {
    /// 食材名称
    pub name: String,
    /// 出现在多少道菜谱中
    pub recipe_count: i64,
}

// ─── list ────────────────────────────────────────────────────────────────────

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 50 }

#[derive(Deserialize, IntoParams)]
pub struct ListParams {
    /// 页码（从 1 开始）
    #[serde(default = "default_page")]
    #[param(default = 1, minimum = 1)]
    pub page: i64,
    /// 每页数量（1-100）
    #[serde(default = "default_per_page")]
    #[param(default = 50, minimum = 1, maximum = 100)]
    pub per_page: i64,
}


/// 获取食材列表（按出现次数降序）
#[utoipa::path(
    get,
    path = "/api/v1/ingredients",
    params(ListParams),
    responses(
        (status = 200, description = "分页食材列表", body = inline(PagedResult<IngredientSummary>)),
        (status = 422, description = "参数校验失败", body = ErrorResponse),
    ),
    tag = "食材"
)]
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PagedResult<IngredientSummary>>, AppError> {
    if params.page < 1 {
        return Err(AppError::Validation("page must be >= 1".to_string()));
    }
    if params.per_page < 1 || params.per_page > 100 {
        return Err(AppError::Validation(
            "per_page must be between 1 and 100".to_string(),
        ));
    }

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(DISTINCT name) FROM ingredients")
            .fetch_one(&state.db)
            .await?;

    let offset = (params.page - 1) * params.per_page;
    let data = sqlx::query_as::<_, IngredientSummary>(
        "SELECT name, COUNT(DISTINCT recipe_id) AS recipe_count \
         FROM ingredients GROUP BY name ORDER BY recipe_count DESC, name \
         LIMIT ? OFFSET ?",
    )
    .bind(params.per_page)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    let pg = PaginationParams { page: params.page, per_page: params.per_page };
    Ok(Json(PagedResult::new(data, total, &pg)))
}

// ─── suggest ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
pub struct SuggestParams {
    /// 搜索关键词（食材名模糊匹配）
    pub q: Option<String>,
    /// 返回条数上限
    #[param(default = 10, minimum = 1, maximum = 50)]
    pub limit: Option<i64>,
}

/// 食材名模糊搜索（自动补全）
#[utoipa::path(
    get,
    path = "/api/v1/ingredients/suggest",
    params(SuggestParams),
    responses(
        (status = 200, description = "匹配的食材列表", body = Vec<IngredientSummary>),
    ),
    tag = "食材"
)]
pub async fn suggest(
    State(state): State<AppState>,
    Query(params): Query<SuggestParams>,
) -> Result<Json<Vec<IngredientSummary>>, AppError> {
    let q = params.q.as_deref().unwrap_or("").trim().to_string();
    if q.is_empty() {
        return Ok(Json(vec![]));
    }
    let limit = params.limit.unwrap_or(10).clamp(1, 50);
    let pattern = format!("%{q}%");

    let data = sqlx::query_as::<_, IngredientSummary>(
        "SELECT name, COUNT(DISTINCT recipe_id) AS recipe_count \
         FROM ingredients WHERE name LIKE ? \
         GROUP BY name ORDER BY recipe_count DESC \
         LIMIT ?",
    )
    .bind(pattern)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(data))
}
