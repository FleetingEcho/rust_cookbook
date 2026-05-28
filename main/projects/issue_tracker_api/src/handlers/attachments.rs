use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::Response,
    Json,
};

use crate::{
    dto::AttachmentResponse,
    error::{AppError, AppResult},
    models::Attachment,
    state::AppState,
    storage,
};

pub async fn attachments_for_issue(state: &AppState, issue_id: i64) -> AppResult<Vec<Attachment>> {
    sqlx::query_as::<_, Attachment>(
        "SELECT * FROM attachments WHERE issue_id = ? ORDER BY created_at DESC, id DESC",
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .map_err(Into::into)
}

pub async fn list_attachments(
    State(state): State<AppState>,
    Path(issue_id): Path<i64>,
) -> AppResult<Json<Vec<AttachmentResponse>>> {
    super::issues::fetch_issue(&state, issue_id).await?;
    let items: Vec<AttachmentResponse> = attachments_for_issue(&state, issue_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(items))
}

pub async fn upload_attachment(
    State(state): State<AppState>,
    Path(issue_id): Path<i64>,
    mut multipart: Multipart,
) -> AppResult<Json<AttachmentResponse>> {
    super::issues::fetch_issue(&state, issue_id).await?;

    let field = multipart
        .next_field()
        .await?
        .ok_or_else(|| AppError::BadRequest("no file part in multipart form".to_string()))?;

    let original_filename = field
        .file_name()
        .map(ToString::to_string)
        .unwrap_or_else(|| "unnamed".to_string());

    let content_type = field
        .content_type()
        .map(ToString::to_string)
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let data = field.bytes().await?;
    let size_bytes = data.len() as i64;

    let stored_name = storage::stored_file_name(&original_filename);
    let file_path = storage::ensure_inside_upload_dir(&state.upload_dir, &stored_name)?;
    tokio::fs::write(&file_path, &data).await?;

    let attachment = sqlx::query_as::<_, Attachment>(
        "INSERT INTO attachments (issue_id, original_filename, stored_filename, content_type, size_bytes) VALUES (?, ?, ?, ?, ?) RETURNING *",
    )
    .bind(issue_id)
    .bind(&original_filename)
    .bind(&stored_name)
    .bind(&content_type)
    .bind(size_bytes)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(attachment.into()))
}

pub async fn download_attachment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Response<Body>> {
    let attachment = sqlx::query_as::<_, Attachment>("SELECT * FROM attachments WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    let stored_path =
        storage::ensure_inside_upload_dir(&state.upload_dir, &attachment.stored_filename)?;
    let data = tokio::fs::read(&stored_path).await?;

    let content_type = mime_guess::from_path(&attachment.original_filename)
        .first_or_octet_stream()
        .to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type).unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}\"",
            attachment.original_filename
        ))
        .unwrap(),
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", content_type)
        .header(
            "content-disposition",
            format!("attachment; filename=\"{}\"", attachment.original_filename),
        )
        .body(Body::from(data))
        .unwrap())
}

pub async fn delete_attachment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    let attachment = sqlx::query_as::<_, Attachment>("SELECT * FROM attachments WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await?;

    sqlx::query("DELETE FROM attachments WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    let stored_path =
        storage::ensure_inside_upload_dir(&state.upload_dir, &attachment.stored_filename)?;
    let _ = tokio::fs::remove_file(&stored_path).await;

    Ok(Json(serde_json::json!({ "deleted": true })))
}
