use axum::{
    extract::{Multipart, Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{error::AppError, AppState};

const IMAGES_DIR: &str = "public/images";
const MAX_SIZE: usize = 10 * 1024 * 1024; // 10 MB
const ALLOWED_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif"];

#[derive(Debug, Serialize, ToSchema)]
pub struct ImageResponse {
    /// 图片访问路径，如 /images/upload_abc123.jpg
    pub url: String,
}

/// 上传或替换菜谱封面图（multipart/form-data，字段名 image）
#[utoipa::path(
    post,
    path = "/api/v1/recipes/{id}/image",
    params(("id" = i64, Path, description = "菜谱 ID")),
    request_body(content_type = "multipart/form-data", content = String),
    responses(
        (status = 200, description = "上传成功，返回图片路径", body = ImageResponse),
        (status = 400, description = "文件类型不支持或超出大小限制", body = crate::error::ErrorResponse),
        (status = 404, description = "菜谱不存在", body = crate::error::ErrorResponse),
    ),
    tag = "图片管理"
)]
pub async fn upload(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ImageResponse>, AppError> {
    // Verify recipe exists
    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM recipes WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    if exists.is_none() {
        return Err(AppError::NotFound);
    }

    // Pull the image field from multipart
    let mut file_bytes: Option<(Vec<u8>, String)> = None;
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("multipart error: {e}"))
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name != "image" {
            continue;
        }

        let filename = field
            .file_name()
            .unwrap_or("upload.jpg")
            .to_string();

        let ext = filename
            .rsplit('.')
            .next()
            .unwrap_or("jpg")
            .to_lowercase();

        if !ALLOWED_EXTS.contains(&ext.as_str()) {
            return Err(AppError::BadRequest(format!(
                "unsupported file type .{ext}; allowed: jpg, jpeg, png, webp, gif"
            )));
        }

        let data = field.bytes().await.map_err(|e| {
            AppError::BadRequest(format!("failed to read upload: {e}"))
        })?;

        if data.len() > MAX_SIZE {
            return Err(AppError::Validation(format!(
                "file too large ({} MB); max 10 MB",
                data.len() / 1024 / 1024
            )));
        }

        file_bytes = Some((data.to_vec(), ext));
        break;
    }

    let (data, ext) = file_bytes.ok_or_else(|| {
        AppError::BadRequest("missing `image` field in multipart form".to_string())
    })?;

    // Delete old image if it was a user-uploaded one (prefix "upload_")
    let old_url: Option<String> =
        sqlx::query_scalar("SELECT cover_image FROM recipes WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?
            .flatten();

    if let Some(old) = &old_url {
        if let Some(fname) = old.strip_prefix("/images/") {
            if fname.starts_with("upload_") {
                let old_path = format!("{IMAGES_DIR}/{fname}");
                let _ = tokio::fs::remove_file(&old_path).await;
            }
        }
    }

    // Ensure upload directory exists
    tokio::fs::create_dir_all(IMAGES_DIR).await.ok();

    // Save new file
    let new_filename = format!("upload_{}_{}.{}", id, Uuid::new_v4().simple(), ext);
    let new_path = format!("{IMAGES_DIR}/{new_filename}");
    tokio::fs::write(&new_path, &data).await.map_err(|e| {
        AppError::Internal(format!("failed to save image: {e}"))
    })?;

    let url = format!("/images/{new_filename}");

    // Update DB
    sqlx::query("UPDATE recipes SET cover_image = ? WHERE id = ?")
        .bind(&url)
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(ImageResponse { url }))
}

/// 删除菜谱封面图
#[utoipa::path(
    delete,
    path = "/api/v1/recipes/{id}/image",
    params(("id" = i64, Path, description = "菜谱 ID")),
    responses(
        (status = 200, description = "删除成功"),
        (status = 404, description = "菜谱不存在", body = crate::error::ErrorResponse),
    ),
    tag = "图片管理"
)]
pub async fn delete(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let url: Option<String> =
        sqlx::query_scalar("SELECT cover_image FROM recipes WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?
            .flatten();

    if url.is_none() {
        return Err(AppError::NotFound);
    }

    // Only delete file if it's a user upload
    if let Some(ref u) = url {
        if let Some(fname) = u.strip_prefix("/images/") {
            if fname.starts_with("upload_") {
                let _ = tokio::fs::remove_file(format!("{IMAGES_DIR}/{fname}")).await;
            }
        }
    }

    sqlx::query("UPDATE recipes SET cover_image = NULL WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetUrlBody {
    /// 封面图的完整 URL（CDN、GitHub raw、Pixabay 等）
    pub url: String,
}

/// 直接用 URL 设置菜谱封面图（不上传文件）
#[utoipa::path(
    put,
    path = "/api/v1/recipes/{id}/image",
    params(("id" = i64, Path, description = "菜谱 ID")),
    request_body = SetUrlBody,
    responses(
        (status = 200, description = "设置成功，返回图片 URL", body = ImageResponse),
        (status = 400, description = "URL 为空", body = crate::error::ErrorResponse),
        (status = 404, description = "菜谱不存在", body = crate::error::ErrorResponse),
    ),
    tag = "图片管理"
)]
pub async fn set_url(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(body): Json<SetUrlBody>,
) -> Result<Json<ImageResponse>, AppError> {
    if body.url.trim().is_empty() {
        return Err(AppError::BadRequest("url must not be empty".to_string()));
    }

    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM recipes WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await?;
    if exists.is_none() {
        return Err(AppError::NotFound);
    }

    // Clean up old user-uploaded file if being replaced
    let old: Option<String> =
        sqlx::query_scalar("SELECT cover_image FROM recipes WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await?
            .flatten();
    if let Some(ref u) = old {
        if let Some(fname) = u.strip_prefix("/images/") {
            if fname.starts_with("upload_") {
                let _ = tokio::fs::remove_file(format!("{IMAGES_DIR}/{fname}")).await;
            }
        }
    }

    sqlx::query("UPDATE recipes SET cover_image = ? WHERE id = ?")
        .bind(&body.url)
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(ImageResponse { url: body.url }))
}
