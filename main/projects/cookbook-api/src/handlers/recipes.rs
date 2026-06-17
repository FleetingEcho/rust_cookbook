use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use utoipa::IntoParams;

use crate::{
    error::AppError,
    models::{
        ingredient::Ingredient,
        nutrition::Nutrition,
        pagination::{PagedResult, PaginationParams},
        recipe::{RecipeDetail, RecipeRow, RecipeSummary},
        step::Step,
    },
    AppState,
};

// ─── list ────────────────────────────────────────────────────────────────────

// serde_urlencoded does not support #[serde(flatten)], so pagination fields are
// inlined directly rather than nested via PaginationParams.
#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListParams {
    /// 页码（默认 1）
    #[serde(default = "default_page")]
    #[param(default = 1, minimum = 1)]
    pub page: i64,
    /// 每页数量（默认 20，上限 100）
    #[serde(default = "default_per_page")]
    #[param(default = 20, minimum = 1, maximum = 100)]
    pub per_page: i64,
    /// 分类过滤（如 水产、早餐）
    pub category: Option<String>,
    /// 难度下限（1-5）
    pub difficulty_min: Option<i64>,
    /// 难度上限（1-5）
    pub difficulty_max: Option<i64>,
    /// 热量下限（大卡）
    pub min_calories: Option<f64>,
    /// 热量上限（大卡）
    pub max_calories: Option<f64>,
    /// 精确难度过滤（1-5，与 difficulty_min/max 互斥）
    pub difficulty: Option<i64>,
    /// 来源过滤（HowToCook / CookLikeHOC）
    pub source: Option<String>,
    /// 图片过滤：true = 只看有图，false = 只看无图
    pub has_image: Option<bool>,
    /// 排序字段（calories / difficulty / name，默认 id）
    pub sort_by: Option<String>,
    /// 排序方向（asc / desc，默认 asc）
    pub order: Option<String>,
}

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 20 }

impl ListParams {
    fn validate(&self) -> Result<(), AppError> {
        if self.page < 1 {
            return Err(AppError::Validation("page must be >= 1".to_string()));
        }
        if self.per_page < 1 || self.per_page > 100 {
            return Err(AppError::Validation(
                "per_page must be between 1 and 100".to_string(),
            ));
        }
        Ok(())
    }
    fn offset(&self) -> i64 { (self.page - 1) * self.per_page }
    fn as_pagination(&self) -> PaginationParams {
        PaginationParams { page: self.page, per_page: self.per_page }
    }
}

/// 获取菜谱列表（支持分页、过滤、排序）
#[utoipa::path(
    get,
    path = "/api/v1/recipes",
    params(ListParams),
    responses(
        (status = 200, description = "分页菜谱列表", body = inline(PagedResult<RecipeSummary>)),
        (status = 422, description = "参数校验失败", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<PagedResult<RecipeSummary>>, AppError> {
    params.validate()?;

    let total: i64 = {
        let mut qb: QueryBuilder<Sqlite> =
            QueryBuilder::new("SELECT COUNT(*) FROM recipes r WHERE 1=1");
        push_filters(&mut qb, &params);
        qb.build_query_scalar().fetch_one(&state.db).await?
    };

    let sort_col = match params.sort_by.as_deref() {
        Some("calories") => "r.calories",
        Some("difficulty") => "r.difficulty",
        Some("name") => "r.name",
        _ => "r.id",
    };
    let order_dir = if params.order.as_deref() == Some("desc") { "DESC" } else { "ASC" };

    let data = {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
             COUNT(i.id) AS ingredient_count \
             FROM recipes r LEFT JOIN ingredients i ON i.recipe_id = r.id WHERE 1=1",
        );
        push_filters(&mut qb, &params);
        qb.push(format!(
            " GROUP BY r.id ORDER BY {sort_col} {order_dir} LIMIT "
        ));
        qb.push_bind(params.per_page);
        qb.push(" OFFSET ");
        qb.push_bind(params.offset());
        qb.build_query_as::<RecipeSummary>()
            .fetch_all(&state.db)
            .await?
    };

    Ok(Json(PagedResult::new(data, total, &params.as_pagination())))
}

fn push_filters(qb: &mut QueryBuilder<Sqlite>, params: &ListParams) {
    if let Some(cat) = &params.category {
        qb.push(" AND r.category = ");
        qb.push_bind(cat.clone());
    }
    if let Some(d) = params.difficulty {
        qb.push(" AND r.difficulty = ");
        qb.push_bind(d);
    } else {
        if let Some(min) = params.difficulty_min {
            qb.push(" AND r.difficulty >= ");
            qb.push_bind(min);
        }
        if let Some(max) = params.difficulty_max {
            qb.push(" AND r.difficulty <= ");
            qb.push_bind(max);
        }
    }
    if let Some(min) = params.min_calories {
        qb.push(" AND r.calories >= ");
        qb.push_bind(min);
    }
    if let Some(max) = params.max_calories {
        qb.push(" AND r.calories <= ");
        qb.push_bind(max);
    }
    if let Some(src) = &params.source {
        qb.push(" AND r.source = ");
        qb.push_bind(src.clone());
    }
    match params.has_image {
        Some(true)  => { qb.push(" AND r.cover_image IS NOT NULL"); }
        Some(false) => { qb.push(" AND r.cover_image IS NULL"); }
        None => {}
    }
}

// ─── get ─────────────────────────────────────────────────────────────────────

/// 获取菜谱详情（含食材、步骤、营养、标签）
#[utoipa::path(
    get,
    path = "/api/v1/recipes/{id}",
    params(("id" = i64, Path, description = "菜谱 ID")),
    responses(
        (status = 200, description = "菜谱详情", body = RecipeDetail),
        (status = 404, description = "菜谱不存在", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<RecipeDetail>, AppError> {
    fetch_detail(&state.db, id).await.map(Json)
}

// ─── random ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct RandomParams {
    /// 分类过滤（可选）
    pub category: Option<String>,
    /// 热量上限（可选）
    pub max_calories: Option<f64>,
}

/// 随机获取一道菜谱
#[utoipa::path(
    get,
    path = "/api/v1/recipes/random",
    params(RandomParams),
    responses(
        (status = 200, description = "随机菜谱详情", body = RecipeDetail),
        (status = 404, description = "没有符合条件的菜谱", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn random(
    State(state): State<AppState>,
    Query(params): Query<RandomParams>,
) -> Result<Json<RecipeDetail>, AppError> {
    let mut qb: QueryBuilder<Sqlite> =
        QueryBuilder::new("SELECT id FROM recipes WHERE 1=1");
    if let Some(cat) = &params.category {
        qb.push(" AND category = ");
        qb.push_bind(cat.clone());
    }
    if let Some(max) = params.max_calories {
        qb.push(" AND calories <= ");
        qb.push_bind(max);
    }
    qb.push(" ORDER BY RANDOM() LIMIT 1");

    let id: Option<i64> = qb.build_query_scalar().fetch_optional(&state.db).await?;
    let id = id.ok_or(AppError::NotFound)?;
    fetch_detail(&state.db, id).await.map(Json)
}

// ─── similar ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SimilarParams {
    /// 返回条数（默认 5，上限 20）
    #[serde(default = "default_limit")]
    #[param(default = 5, minimum = 1, maximum = 20)]
    pub limit: i64,
}

fn default_limit() -> i64 { 5 }

/// 获取相似菜谱（同分类，按食材重合度排序）
#[utoipa::path(
    get,
    path = "/api/v1/recipes/{id}/similar",
    params(
        ("id" = i64, Path, description = "菜谱 ID"),
        SimilarParams,
    ),
    responses(
        (status = 200, description = "相似菜谱列表", body = Vec<RecipeSummary>),
        (status = 404, description = "菜谱不存在", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn similar(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<SimilarParams>,
) -> Result<Json<Vec<RecipeSummary>>, AppError> {
    let category: Option<String> =
        sqlx::query_scalar("SELECT category FROM recipes WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;
    let category = category.ok_or(AppError::NotFound)?;

    let limit = params.limit.clamp(1, 20);

    let data = sqlx::query_as::<_, RecipeSummary>(
        "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
         COUNT(DISTINCT i.id) AS ingredient_count \
         FROM recipes r \
         LEFT JOIN ingredients i ON i.recipe_id = r.id \
         WHERE r.id != ? AND r.category = ? \
         GROUP BY r.id \
         ORDER BY ( \
             SELECT COUNT(*) FROM ingredients i2 \
             WHERE i2.recipe_id = r.id \
               AND i2.name IN (SELECT name FROM ingredients WHERE recipe_id = ?) \
         ) DESC \
         LIMIT ?",
    )
    .bind(id)
    .bind(&category)
    .bind(id)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(data))
}

// ─── search (FTS5 trigram) ────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchParams {
    /// 查询词（必填）。≥3 字符使用 FTS5，<3 字符自动 fallback 到菜名 LIKE 匹配
    pub q: Option<String>,
    #[serde(default = "default_page")]
    #[param(default = 1, minimum = 1)]
    pub page: i64,
    #[serde(default = "default_per_page")]
    #[param(default = 20, minimum = 1, maximum = 100)]
    pub per_page: i64,
}

/// 全文搜索菜谱（FTS5 trigram）
#[utoipa::path(
    get,
    path = "/api/v1/recipes/search",
    params(SearchParams),
    responses(
        (status = 200, description = "搜索结果（按相关度排序）", body = inline(PagedResult<RecipeSummary>)),
        (status = 422, description = "参数校验失败", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<PagedResult<RecipeSummary>>, AppError> {
    let q = params.q.as_deref().unwrap_or("").trim().to_string();
    if q.is_empty() {
        return Err(AppError::Validation("q is required".to_string()));
    }
    if params.page < 1 {
        return Err(AppError::Validation("page must be >= 1".to_string()));
    }
    if params.per_page < 1 || params.per_page > 100 {
        return Err(AppError::Validation(
            "per_page must be between 1 and 100".to_string(),
        ));
    }

    let offset = (params.page - 1) * params.per_page;
    let pg = PaginationParams { page: params.page, per_page: params.per_page };

    // FTS5 trigram requires at least 3 characters. Fall back to LIKE for shorter queries.
    if q.chars().count() < 3 {
        let pattern = format!("%{q}%");
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM recipes WHERE name LIKE ?",
        )
        .bind(&pattern)
        .fetch_one(&state.db)
        .await?;

        let data = sqlx::query_as::<_, RecipeSummary>(
            "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
             COUNT(i.id) AS ingredient_count \
             FROM recipes r LEFT JOIN ingredients i ON i.recipe_id = r.id \
             WHERE r.name LIKE ? GROUP BY r.id ORDER BY r.name LIMIT ? OFFSET ?",
        )
        .bind(&pattern)
        .bind(params.per_page)
        .bind(offset)
        .fetch_all(&state.db)
        .await?;

        return Ok(Json(PagedResult::new(data, total, &pg)));
    }

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM recipes_fts WHERE recipes_fts MATCH ?",
    )
    .bind(&q)
    .fetch_one(&state.db)
    .await?;

    // Pull ranked IDs from FTS5 first, then join for full recipe data.
    let data = sqlx::query_as::<_, RecipeSummary>(
        "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
         COUNT(i.id) AS ingredient_count \
         FROM ( \
             SELECT recipe_id FROM recipes_fts WHERE recipes_fts MATCH ? \
             ORDER BY rank LIMIT ? OFFSET ? \
         ) fts \
         JOIN recipes r ON r.id = fts.recipe_id \
         LEFT JOIN ingredients i ON i.recipe_id = r.id \
         GROUP BY r.id",
    )
    .bind(&q)
    .bind(params.per_page)
    .bind(offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PagedResult::new(data, total, &pg)))
}

// ─── by-ingredients ───────────────────────────────────────────────────────────

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ByIngredientsParams {
    /// 逗号分隔的食材名（支持模糊匹配，如 "豆腐,鸡蛋"）
    pub ingredients: Option<String>,
    /// 匹配模式：any（含其中之一）/ all（全部包含），默认 any
    #[serde(default = "default_match_mode")]
    pub r#match: String,
    #[serde(default = "default_page")]
    #[param(default = 1, minimum = 1)]
    pub page: i64,
    #[serde(default = "default_per_page")]
    #[param(default = 20, minimum = 1, maximum = 100)]
    pub per_page: i64,
}

fn default_match_mode() -> String { "any".to_string() }

/// 按食材反查菜谱
#[utoipa::path(
    get,
    path = "/api/v1/recipes/by-ingredients",
    params(ByIngredientsParams),
    responses(
        (status = 200, description = "含指定食材的菜谱列表", body = inline(PagedResult<RecipeSummary>)),
        (status = 422, description = "参数校验失败", body = ErrorResponse),
    ),
    tag = "菜谱"
)]
pub async fn by_ingredients(
    State(state): State<AppState>,
    Query(params): Query<ByIngredientsParams>,
) -> Result<Json<PagedResult<RecipeSummary>>, AppError> {
    let ing_list: Vec<String> = params
        .ingredients
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if ing_list.is_empty() {
        return Err(AppError::Validation(
            "ingredients is required (comma-separated)".to_string(),
        ));
    }
    if params.page < 1 {
        return Err(AppError::Validation("page must be >= 1".to_string()));
    }
    if params.per_page < 1 || params.per_page > 100 {
        return Err(AppError::Validation(
            "per_page must be between 1 and 100".to_string(),
        ));
    }

    let join_op = if params.r#match == "all" { "AND" } else { "OR" };
    // Build: EXISTS(...) AND/OR EXISTS(...) ...
    let exists_clauses: String = ing_list
        .iter()
        .map(|_| "EXISTS (SELECT 1 FROM ingredients WHERE recipe_id = r.id AND name LIKE ?)")
        .collect::<Vec<_>>()
        .join(&format!(" {join_op} "));
    let patterns: Vec<String> = ing_list.iter().map(|s| format!("%{s}%")).collect();

    let count_sql = format!(
        "SELECT COUNT(*) FROM recipes r WHERE {exists_clauses}"
    );
    let total: i64 = {
        let mut q = sqlx::query_scalar::<_, i64>(&count_sql);
        for p in &patterns {
            q = q.bind(p.clone());
        }
        q.fetch_one(&state.db).await?
    };

    let offset = (params.page - 1) * params.per_page;
    let data_sql = format!(
        "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
         COUNT(DISTINCT i.id) AS ingredient_count \
         FROM recipes r \
         LEFT JOIN ingredients i ON i.recipe_id = r.id \
         WHERE {exists_clauses} \
         GROUP BY r.id ORDER BY ingredient_count DESC \
         LIMIT ? OFFSET ?"
    );
    let data = {
        let mut q = sqlx::query_as::<_, RecipeSummary>(&data_sql);
        for p in &patterns {
            q = q.bind(p.clone());
        }
        q.bind(params.per_page).bind(offset).fetch_all(&state.db).await?
    };

    let pg = PaginationParams { page: params.page, per_page: params.per_page };
    Ok(Json(PagedResult::new(data, total, &pg)))
}

// ─── shared helper ───────────────────────────────────────────────────────────

async fn fetch_detail(db: &SqlitePool, id: i64) -> Result<RecipeDetail, AppError> {
    let recipe = sqlx::query_as::<_, RecipeRow>(
        "SELECT id, name, description, category, difficulty, calories, cover_image, \
         source, source_path, created_at FROM recipes WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(AppError::NotFound)?;

    let ingredients = sqlx::query_as::<_, Ingredient>(
        "SELECT id, recipe_id, name, amount, unit, note FROM ingredients \
         WHERE recipe_id = ? ORDER BY id",
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let steps = sqlx::query_as::<_, Step>(
        "SELECT id, recipe_id, step_order, content, image_url FROM steps \
         WHERE recipe_id = ? ORDER BY step_order",
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let nutrition = sqlx::query_as::<_, Nutrition>(
        "SELECT id, recipe_id, protein_g, fat_g, carbs_g, sodium_mg \
         FROM nutrition WHERE recipe_id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    let tags: Vec<String> =
        sqlx::query_scalar("SELECT tag FROM tags WHERE recipe_id = ? ORDER BY tag")
            .bind(id)
            .fetch_all(db)
            .await?;

    Ok(RecipeDetail {
        recipe,
        ingredients,
        steps,
        nutrition,
        tags,
    })
}
