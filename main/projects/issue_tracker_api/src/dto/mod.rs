pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

// ── From 转换：models::Issue → response types ─────────────────────────────────

use crate::models;

impl From<models::Issue> for IssueSummary {
    fn from(i: models::Issue) -> Self {
        IssueSummary {
            id: i.id,
            title: i.title,
            description: i.description,
            status: i.status,
            priority: i.priority,
            issue_type: i.issue_type,
            assignee: i.assignee,
            created_by: i.created_by,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}

impl From<models::Comment> for CommentResponse {
    fn from(c: models::Comment) -> Self {
        CommentResponse {
            id: c.id,
            issue_id: c.issue_id,
            author: c.author,
            body: c.body,
            created_at: c.created_at,
        }
    }
}

impl From<models::Label> for LabelResponse {
    fn from(l: models::Label) -> Self {
        LabelResponse {
            id: l.id,
            name: l.name,
            color: l.color,
        }
    }
}

impl From<models::Attachment> for AttachmentResponse {
    fn from(a: models::Attachment) -> Self {
        AttachmentResponse {
            id: a.id,
            issue_id: a.issue_id,
            original_filename: a.original_filename,
            stored_filename: a.stored_filename,
            content_type: a.content_type,
            size_bytes: a.size_bytes,
            created_at: a.created_at,
        }
    }
}
