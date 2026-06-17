use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers::{categories, images, ingredients, meal_plan, recipes},
    openapi::ApiDoc,
    AppState,
};

pub fn build(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest_service("/images", ServeDir::new("public/images"))
        .route("/health", get(health))
        .nest("/api/v1", api_router())
        .with_state(state)
}

fn api_router() -> Router<AppState> {
    Router::new()
        // recipes
        .route("/recipes", get(recipes::list))
        .route("/recipes/random", get(recipes::random))
        .route("/recipes/search", get(recipes::search))
        .route("/recipes/by-ingredients", get(recipes::by_ingredients))
        .route("/recipes/:id", get(recipes::get))
        .route("/recipes/:id/similar", get(recipes::similar))
        .route("/recipes/:id/image", post(images::upload).put(images::set_url).delete(images::delete))
        // categories + stats
        .route("/categories", get(categories::list))
        .route("/stats", get(categories::stats))
        // ingredients
        .route("/ingredients", get(ingredients::list))
        .route("/ingredients/suggest", get(ingredients::suggest))
        // meal plan
        .route("/meal-plan", post(meal_plan::generate))
}

async fn health() -> &'static str {
    "ok"
}
