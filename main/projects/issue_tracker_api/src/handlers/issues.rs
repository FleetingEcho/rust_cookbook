use axum::{
    extract::{Path, Query, State},
    Json,
};
use sqlx::{QueryBuilder, Row, Sqlite};

use validator::Validate;

use crate::{
    dto::{
        CreateIssueRequest, IssueDetailResponse, IssueQueryParams, IssueSummary, PaginatedResponse,
        UpdateIssueRequest,
    },
    error::{AppError, AppResult},
    models::Issue,
    state::AppState,
};

pub async fn list_issues(
    State(state): State<AppState>,
    Query(query): Query<IssueQueryParams>,
) -> AppResult<Json<PaginatedResponse<IssueSummary>>> {
    // ── 构建计数查询 ─────────────────────────────────────────────────────
    let mut count_builder =
        QueryBuilder::<Sqlite>::new("SELECT COUNT(DISTINCT issues.id) FROM issues");
    if query.label_id.is_some() {
        count_builder.push(" JOIN issue_labels ON issues.id = issue_labels.issue_id");
    }
    count_builder.push(" WHERE 1 = 1");
    if let Some(ref status) = query.status {
        count_builder.push(" AND status = ").push_bind(status);
    }
    if let Some(ref priority) = query.priority {
        count_builder.push(" AND priority = ").push_bind(priority);
    }
    if let Some(ref issue_type) = query.issue_type {
        count_builder
            .push(" AND issue_type = ")
            .push_bind(issue_type);
    }
    if let Some(label_id) = query.label_id {
        count_builder
            .push(" AND issue_labels.label_id = ")
            .push_bind(label_id);
    }
    if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        count_builder
            .push(" AND (title LIKE ")
            .push_bind(pattern.clone())
            .push(" OR description LIKE ")
            .push_bind(pattern)
            .push(")");
    }

    let total: i64 = count_builder
        .build()
        .fetch_one(&state.db)
        .await?
        .get::<i64, _>(0);

    // ── 构建数据查询 ─────────────────────────────────────────────────────
    let mut builder = QueryBuilder::<Sqlite>::new("SELECT DISTINCT issues.* FROM issues");
    if query.label_id.is_some() {
        builder.push(" JOIN issue_labels ON issues.id = issue_labels.issue_id");
    }
    builder.push(" WHERE 1 = 1");
    if let Some(ref status) = query.status {
        builder.push(" AND status = ").push_bind(status);
    }
    if let Some(ref priority) = query.priority {
        builder.push(" AND priority = ").push_bind(priority);
    }
    if let Some(ref issue_type) = query.issue_type {
        builder.push(" AND issue_type = ").push_bind(issue_type);
    }
    if let Some(label_id) = query.label_id {
        builder
            .push(" AND issue_labels.label_id = ")
            .push_bind(label_id);
    }
    if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        builder
            .push(" AND (title LIKE ")
            .push_bind(pattern.clone())
            .push(" OR description LIKE ")
            .push_bind(pattern)
            .push(")");
    }
    builder.push(" ORDER BY updated_at DESC, id DESC");
    builder.push(" LIMIT ").push_bind(query.limit);
    builder.push(" OFFSET ").push_bind(query.offset);

    let issues: Vec<IssueSummary> = builder
        .build_query_as::<Issue>()
        .fetch_all(&state.db)
        .await?
        .into_iter()
        .map(IssueSummary::from)
        .collect();

    Ok(Json(PaginatedResponse {
        items: issues,
        total,
        limit: query.limit,
        offset: query.offset,
    }))
}

pub async fn create_issue(
    State(state): State<AppState>,
    Json(input): Json<CreateIssueRequest>,
) -> AppResult<Json<IssueDetailResponse>> {
    input
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let mut tx = state.db.begin().await?;
    let id = sqlx::query(
        "INSERT INTO issues (title, description, priority, issue_type, assignee, created_by) VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
    )
    .bind(input.title.trim())
    .bind(input.description.trim())
    .bind(input.priority)
    .bind(input.issue_type)
    .bind(input.assignee)
    .bind(input.created_by.trim())
    .fetch_one(&mut *tx)
    .await?
    .get::<i64, _>("id");

    if let Some(label_ids) = input.label_ids {
        for label_id in label_ids {
            sqlx::query("INSERT OR IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
                .bind(id)
                .bind(label_id)
                .execute(&mut *tx)
                .await?;
        }
    }
    tx.commit().await?;
    get_issue(State(state), Path(id)).await
}

pub async fn get_issue(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<IssueDetailResponse>> {
    let issue = fetch_issue(&state, id).await?;
    let labels = super::labels::labels_for_issue(&state, id).await?;
    let comments = super::comments::comments_for_issue(&state, id).await?;
    let attachments = super::attachments::attachments_for_issue(&state, id).await?;

    Ok(Json(IssueDetailResponse {
        id: issue.id,
        title: issue.title,
        description: issue.description,
        status: issue.status,
        priority: issue.priority,
        issue_type: issue.issue_type,
        assignee: issue.assignee,
        created_by: issue.created_by,
        created_at: issue.created_at,
        updated_at: issue.updated_at,
        labels: labels.into_iter().map(Into::into).collect(),
        comments: comments.into_iter().map(Into::into).collect(),
        attachments: attachments.into_iter().map(Into::into).collect(),
    }))
}

pub async fn update_issue(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateIssueRequest>,
) -> AppResult<Json<IssueDetailResponse>> {
    fetch_issue(&state, id).await?;
    let current = fetch_issue(&state, id).await?;
    let title = input.title.unwrap_or(current.title);
    let description = input.description.unwrap_or(current.description);
    let status = input.status.unwrap_or(current.status);
    let priority = input.priority.unwrap_or(current.priority);
    let issue_type = input.issue_type.unwrap_or(current.issue_type);
    let assignee = input.assignee.unwrap_or(current.assignee);
    validate_issue_fields(&title, &description, &priority, &issue_type)?;
    validate_status(&status)?;

    let mut tx = state.db.begin().await?;
    sqlx::query(
        "UPDATE issues SET title = ?, description = ?, status = ?, priority = ?, issue_type = ?, assignee = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(title.trim())
    .bind(description.trim())
    .bind(status)
    .bind(priority)
    .bind(issue_type)
    .bind(assignee)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    if let Some(label_ids) = input.label_ids {
        sqlx::query("DELETE FROM issue_labels WHERE issue_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        for label_id in label_ids {
            sqlx::query("INSERT OR IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
                .bind(id)
                .bind(label_id)
                .execute(&mut *tx)
                .await?;
        }
    }
    tx.commit().await?;
    get_issue(State(state), Path(id)).await
}

pub async fn delete_issue(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    fetch_issue(&state, id).await?;
    sqlx::query("DELETE FROM issues WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

pub async fn fetch_issue(state: &AppState, id: i64) -> AppResult<Issue> {
    sqlx::query_as::<_, Issue>("SELECT * FROM issues WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(Into::into)
}

// ── 校验函数（用于 update 场景，因为 UpdateIssue 字段全可选） ──────────────

pub fn validate_issue_fields(
    title: &str,
    description: &str,
    priority: &str,
    issue_type: &str,
) -> AppResult<()> {
    if title.trim().is_empty() {
        return Err(AppError::BadRequest("title is required".to_string()));
    }
    if description.trim().is_empty() {
        return Err(AppError::BadRequest("description is required".to_string()));
    }
    if !matches!(priority, "low" | "medium" | "high") {
        return Err(AppError::BadRequest(
            "priority must be low, medium, or high".to_string(),
        ));
    }
    if !matches!(issue_type, "bug" | "feature" | "task" | "question") {
        return Err(AppError::BadRequest(
            "issue_type must be bug, feature, task, or question".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_status(status: &str) -> AppResult<()> {
    if matches!(status, "open" | "in_progress" | "closed") {
        Ok(())
    } else {
        Err(AppError::BadRequest(
            "status must be open, in_progress, or closed".to_string(),
        ))
    }
}

// ── 测试 ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_issue_fields_ok() {
        assert!(validate_issue_fields("title", "desc", "high", "bug").is_ok());
    }

    #[test]
    fn validate_issue_fields_empty_title() {
        let err = validate_issue_fields("", "desc", "high", "bug").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn validate_issue_fields_bad_priority() {
        let err = validate_issue_fields("title", "desc", "urgent", "bug").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }

    #[test]
    fn validate_status_ok() {
        assert!(validate_status("open").is_ok());
        assert!(validate_status("in_progress").is_ok());
        assert!(validate_status("closed").is_ok());
    }

    #[test]
    fn validate_status_bad() {
        let err = validate_status("deleted").unwrap_err();
        assert!(matches!(err, AppError::BadRequest(_)));
    }
}
