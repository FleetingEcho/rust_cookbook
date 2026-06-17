use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{error::AppError, models::recipe::RecipeSummary, AppState};

/// 餐单生成请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct MealPlanRequest {
    /// 天数（1-14，默认 7）
    #[serde(default = "default_days")]
    pub days: usize,
    /// 用餐人数（仅记录，默认 2）
    #[serde(default = "default_people")]
    pub people: u32,
    /// 每道菜热量上限（大卡）
    pub max_calories_per_meal: Option<f64>,
    /// 难度上限（1-5）
    pub max_difficulty: Option<i64>,
    /// 要求同时具有的标签（如 "低卡"、"新手友好"）
    pub tags: Option<Vec<String>>,
}

fn default_days() -> usize { 7 }
fn default_people() -> u32 { 2 }

/// 单日餐单
#[derive(Debug, Serialize, ToSchema)]
pub struct DayPlan {
    /// 第几天（从 1 开始）
    pub day: usize,
    /// 早餐（1 道）
    pub breakfast: RecipeSummary,
    /// 午餐（2 道）
    pub lunch: Vec<RecipeSummary>,
    /// 晚餐（2 道）
    pub dinner: Vec<RecipeSummary>,
}

/// 餐单响应
#[derive(Debug, Serialize, ToSchema)]
pub struct MealPlanResponse {
    pub days: Vec<DayPlan>,
    pub people: u32,
}

/// 生成每日餐单
#[utoipa::path(
    post,
    path = "/api/v1/meal-plan",
    request_body = MealPlanRequest,
    responses(
        (status = 200, description = "生成的餐单", body = MealPlanResponse),
        (status = 422, description = "参数校验失败", body = ErrorResponse),
    ),
    tag = "餐单"
)]
pub async fn generate(
    State(state): State<AppState>,
    Json(req): Json<MealPlanRequest>,
) -> Result<Json<MealPlanResponse>, AppError> {
    if req.days == 0 || req.days > 14 {
        return Err(AppError::Validation("days must be between 1 and 14".to_string()));
    }

    let breakfasts = fetch_pool(&state, "早餐", &req).await?;
    let main_dishes = fetch_main_pool(&state, &req).await?;

    let needed_breakfasts = req.days;
    let needed_mains = req.days * 2 * 2;

    if breakfasts.len() < needed_breakfasts {
        return Err(AppError::Validation(format!(
            "not enough breakfast recipes ({} available, {} needed)",
            breakfasts.len(), needed_breakfasts
        )));
    }
    if main_dishes.len() < needed_mains {
        return Err(AppError::Validation(format!(
            "not enough main dish recipes ({} available, {} needed for {} days)",
            main_dishes.len(), needed_mains, req.days
        )));
    }

    let bf_pool = breakfasts;
    let main_pool = main_dishes;

    let mut day_plans = Vec::with_capacity(req.days);
    let mut bf_idx = 0;
    let mut main_idx = 0;

    for day in 1..=req.days {
        let breakfast = bf_pool[bf_idx].clone();
        bf_idx += 1;

        let lunch = vec![main_pool[main_idx].clone(), main_pool[main_idx + 1].clone()];
        main_idx += 2;

        let dinner = vec![main_pool[main_idx].clone(), main_pool[main_idx + 1].clone()];
        main_idx += 2;

        day_plans.push(DayPlan { day, breakfast, lunch, dinner });
    }

    Ok(Json(MealPlanResponse { days: day_plans, people: req.people }))
}

async fn fetch_pool(
    state: &AppState,
    category: &str,
    req: &MealPlanRequest,
) -> Result<Vec<RecipeSummary>, AppError> {
    let mut conditions = vec![format!("r.category = '{category}'")];
    if let Some(max_cal) = req.max_calories_per_meal {
        conditions.push(format!("(r.calories IS NULL OR r.calories <= {max_cal})"));
    }
    if let Some(max_diff) = req.max_difficulty {
        conditions.push(format!("(r.difficulty IS NULL OR r.difficulty <= {max_diff})"));
    }
    let where_clause = conditions.join(" AND ");
    let tag_join = build_tag_join(req);
    let sql = format!(
        "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
         COUNT(DISTINCT i.id) AS ingredient_count \
         FROM recipes r LEFT JOIN ingredients i ON i.recipe_id = r.id \
         {tag_join} WHERE {where_clause} GROUP BY r.id ORDER BY RANDOM()"
    );
    sqlx::query_as::<_, RecipeSummary>(&sql)
        .fetch_all(&state.db)
        .await
        .map_err(AppError::Db)
}

async fn fetch_main_pool(
    state: &AppState,
    req: &MealPlanRequest,
) -> Result<Vec<RecipeSummary>, AppError> {
    let main_cats = "'荤菜','素菜','水产','炒菜','蒸菜','炖菜','卤菜'";
    let mut conditions = vec![format!("r.category IN ({main_cats})")];
    if let Some(max_cal) = req.max_calories_per_meal {
        conditions.push(format!("(r.calories IS NULL OR r.calories <= {max_cal})"));
    }
    if let Some(max_diff) = req.max_difficulty {
        conditions.push(format!("(r.difficulty IS NULL OR r.difficulty <= {max_diff})"));
    }
    let where_clause = conditions.join(" AND ");
    let tag_join = build_tag_join(req);
    let sql = format!(
        "SELECT r.id, r.name, r.category, r.difficulty, r.calories, r.cover_image, r.source, \
         COUNT(DISTINCT i.id) AS ingredient_count \
         FROM recipes r LEFT JOIN ingredients i ON i.recipe_id = r.id \
         {tag_join} WHERE {where_clause} GROUP BY r.id ORDER BY RANDOM()"
    );
    sqlx::query_as::<_, RecipeSummary>(&sql)
        .fetch_all(&state.db)
        .await
        .map_err(AppError::Db)
}

fn build_tag_join(req: &MealPlanRequest) -> String {
    match &req.tags {
        Some(tags) if !tags.is_empty() => tags
            .iter()
            .enumerate()
            .map(|(i, tag)| {
                let safe = tag.replace('\'', "''");
                format!("JOIN tags t{i} ON t{i}.recipe_id = r.id AND t{i}.tag = '{safe}'")
            })
            .collect::<Vec<_>>()
            .join(" "),
        _ => String::new(),
    }
}
