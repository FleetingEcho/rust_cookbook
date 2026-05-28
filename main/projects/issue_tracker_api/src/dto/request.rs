use serde::Deserialize;
use validator::Validate;

/// CreateIssue 请求 DTO（camelCase 输入）
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueRequest {
    #[validate(length(min = 1, max = 200, message = "title is required"))]
    pub title: String,

    #[validate(length(min = 1, message = "description is required"))]
    pub description: String,

    pub priority: String,

    pub issue_type: String,

    pub assignee: Option<String>,

    #[validate(length(min = 1, message = "created_by is required"))]
    pub created_by: String,

    pub label_ids: Option<Vec<i64>>,
}

/// UpdateIssue 请求 DTO（所有字段可选）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIssueRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub issue_type: Option<String>,
    pub assignee: Option<Option<String>>,
    pub label_ids: Option<Vec<i64>>,
}

/// 列表查询参数（含分页）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueQueryParams {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub issue_type: Option<String>,
    pub label_id: Option<i64>,
    pub search: Option<String>,

    /// 每页数量，默认 20
    #[serde(default = "default_limit")]
    pub limit: i64,

    /// 偏移量，默认 0
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 { 5 }
fn default_offset() -> i64 { 0 }

/// CreateComment 请求 DTO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, message = "author is required"))]
    pub author: String,

    #[validate(length(min = 1, message = "body is required"))]
    pub body: String,
}

/// CreateLabel 请求 DTO
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateLabelRequest {
    #[validate(length(min = 1, message = "name is required"))]
    pub name: String,

    #[validate(length(min = 1, message = "color is required"))]
    pub color: String,
}
