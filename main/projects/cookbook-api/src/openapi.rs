use utoipa::OpenApi;

use crate::{
    error::ErrorResponse,
    handlers::{
        categories::{CategoryCount, StatsResponse},
        images::ImageResponse,
        ingredients::IngredientSummary,
        meal_plan::{DayPlan, MealPlanRequest, MealPlanResponse},
    },
    models::{
        ingredient::Ingredient,
        nutrition::Nutrition,
        pagination::PagedResult,
        recipe::{RecipeDetail, RecipeRow, RecipeSummary},
        step::Step,
    },
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "菜谱 Cookbook API",
        version = "1.0.0",
        description = "基于 Axum + SQLite 的菜谱查询服务，收录约 700 道中餐菜谱。",
        contact(name = "Cookbook", email = "dev2jake@gmail.com"),
    ),
    paths(
        crate::handlers::recipes::list,
        crate::handlers::recipes::get,
        crate::handlers::recipes::random,
        crate::handlers::recipes::similar,
        crate::handlers::recipes::search,
        crate::handlers::recipes::by_ingredients,
        crate::handlers::images::upload,
        crate::handlers::images::delete,
        crate::handlers::categories::list,
        crate::handlers::categories::stats,
        crate::handlers::ingredients::list,
        crate::handlers::ingredients::suggest,
        crate::handlers::meal_plan::generate,
    ),
    components(
        schemas(
            ErrorResponse,
            ImageResponse,
            RecipeSummary,
            RecipeDetail,
            RecipeRow,
            Ingredient,
            Step,
            Nutrition,
            CategoryCount,
            StatsResponse,
            IngredientSummary,
            MealPlanRequest,
            DayPlan,
            MealPlanResponse,
            PagedResult<RecipeSummary>,
            PagedResult<IngredientSummary>,
        )
    ),
    tags(
        (name = "菜谱", description = "菜谱查询、搜索、相似推荐"),
        (name = "分类与统计", description = "分类列表和全局统计"),
        (name = "食材", description = "食材列表和模糊搜索"),
        (name = "餐单", description = "自动生成每日餐单"),
        (name = "图片管理", description = "上传、替换、删除菜谱封面图"),
    )
)]
pub struct ApiDoc;
