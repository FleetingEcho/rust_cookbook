use axum::{
    extract::{Path, State},
    Json,
};

use validator::Validate;

use crate::{
    dto::{CommentResponse, CreateCommentRequest},
    error::{AppError, AppResult},
    models::Comment,
    state::AppState,
};

pub async fn comments_for_issue(state: &AppState, issue_id: i64) -> AppResult<Vec<Comment>> {
    sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE issue_id = ? ORDER BY created_at ASC, id ASC",
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

pub async fn list_comments(
    State(state): State<AppState>,
    Path(issue_id): Path<i64>,
) -> AppResult<Json<Vec<CommentResponse>>> {
    super::issues::fetch_issue(&state, issue_id).await?;
    let comments: Vec<CommentResponse> = comments_for_issue(&state, issue_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(comments))
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path(issue_id): Path<i64>,
    Json(input): Json<CreateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    input
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    super::issues::fetch_issue(&state, issue_id).await?;
    let comment = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (issue_id, author, body) VALUES (?, ?, ?) RETURNING *",
    )
    .bind(issue_id)
    .bind(input.author.trim())
    .bind(input.body.trim())
    .fetch_one(&state.db)
    .await?;
    Ok(Json(comment.into()))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM comments WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("comment not found".to_string()));
    }
    Ok(Json(serde_json::json!({ "deleted": true })))
}
