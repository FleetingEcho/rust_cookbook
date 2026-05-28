use sqlx::FromRow;

/// 数据库 Issue 行
#[derive(Debug, FromRow)]
pub struct Issue {
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

/// 数据库 Comment 行
#[derive(Debug, FromRow)]
pub struct Comment {
    pub id: i64,
    pub issue_id: i64,
    pub author: String,
    pub body: String,
    pub created_at: String,
}

/// 数据库 Label 行
#[derive(Debug, FromRow)]
pub struct Label {
    pub id: i64,
    pub name: String,
    pub color: String,
}

/// 数据库 Attachment 行
#[derive(Debug, FromRow)]
pub struct Attachment {
    pub id: i64,
    pub issue_id: i64,
    pub original_filename: String,
    pub stored_filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub created_at: String,
}
