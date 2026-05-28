use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};

pub fn safe_file_name(name: &str) -> String {
    name.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

pub fn stored_file_name(original: &str) -> String {
    format!("{}-{}", uuid::Uuid::new_v4(), safe_file_name(original))
}

pub fn ensure_inside_upload_dir(upload_dir: &Path, stored_name: &str) -> AppResult<PathBuf> {
    if stored_name.contains('/') || stored_name.contains('\\') || stored_name.contains("..") {
        return Err(AppError::BadRequest("invalid file name".to_string()));
    }
    Ok(upload_dir.join(stored_name))
}
