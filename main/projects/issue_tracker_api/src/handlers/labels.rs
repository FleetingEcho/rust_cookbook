use axum::{
    extract::{Path, State},
    Json,
};

use validator::Validate;

use crate::{
    dto::{CreateLabelRequest, LabelResponse},
    error::{AppError, AppResult},
    models::Label,
    state::AppState,
};

pub async fn labels_for_issue(state: &AppState, issue_id: i64) -> AppResult<Vec<Label>> {
    sqlx::query_as::<_, Label>(
        "SELECT labels.* FROM labels JOIN issue_labels ON labels.id = issue_labels.label_id WHERE issue_labels.issue_id = ? ORDER BY labels.name",
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

pub async fn list_labels(State(state): State<AppState>) -> AppResult<Json<Vec<LabelResponse>>> {
    let labels: Vec<LabelResponse> = sqlx::query_as::<_, Label>("SELECT * FROM labels ORDER BY name")
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(labels))
}

pub async fn create_label(
    State(state): State<AppState>,
    Json(input): Json<CreateLabelRequest>,
) -> AppResult<Json<LabelResponse>> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let label =
        sqlx::query_as::<_, Label>("INSERT INTO labels (name, color) VALUES (?, ?) RETURNING *")
            .bind(input.name.trim())
            .bind(input.color.trim())
            .fetch_one(&state.db)
            .await?;
    Ok(Json(label.into()))
}

pub async fn add_issue_label(
    State(state): State<AppState>,
    Path((issue_id, label_id)): Path<(i64, i64)>,
) -> AppResult<Json<serde_json::Value>> {
    super::issues::fetch_issue(&state, issue_id).await?;
    sqlx::query_as::<_, Label>("SELECT * FROM labels WHERE id = ?")
        .bind(label_id)
        .fetch_one(&state.db)
        .await?;
    sqlx::query("INSERT OR IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
        .bind(issue_id)
        .bind(label_id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({ "linked": true })))
}

pub async fn remove_issue_label(
    State(state): State<AppState>,
    Path((issue_id, label_id)): Path<(i64, i64)>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM issue_labels WHERE issue_id = ? AND label_id = ?")
        .bind(issue_id)
        .bind(label_id)
        .execute(&state.db)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("issue label link not found".to_string()));
    }
    Ok(Json(serde_json::json!({ "deleted": true })))
}
