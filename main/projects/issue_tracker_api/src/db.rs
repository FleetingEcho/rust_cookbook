use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::error::AppResult;

pub async fn connect(database_url: &str) -> AppResult<SqlitePool> {
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    sqlx::raw_sql(include_str!("../migrations/0001_init.sql"))
        .execute(&pool)
        .await?;
    Ok(pool)
}
