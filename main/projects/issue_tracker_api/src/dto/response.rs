use serde::Serialize;

// ── 通用分页响应包装 ──────────────────────────────────────────────────────────

/// 分页响应
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

// ── Issue ─────────────────────────────────────────────────────────────────────

/// Issue 摘要（列表用，不含 labels/comments/attachments）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueSummary {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub issue_type: String,
    pub assignee: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Issue 详情（含 labels/comments/attachments）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueDetailResponse {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub issue_type: String,
    pub assignee: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<LabelResponse>,
    pub comments: Vec<CommentResponse>,
    pub attachments: Vec<AttachmentResponse>,
}

// ── Comment ───────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponse {
    pub id: i64,
    pub issue_id: i64,
    pub author: String,
    pub body: String,
    pub created_at: String,
}

// ── Label ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelResponse {
    pub id: i64,
    pub name: String,
    pub color: String,
}

// ── Attachment ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentResponse {
    pub id: i64,
    pub issue_id: i64,
    pub original_filename: String,
    pub stored_filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub created_at: String,
}

// ── Health ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}
