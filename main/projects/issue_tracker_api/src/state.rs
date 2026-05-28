use std::path::PathBuf;

use sqlx::SqlitePool;

use crate::{config::Config, db, error::AppResult};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub upload_dir: PathBuf,
}

impl AppState {
    pub async fn new(config: &Config) -> AppResult<Self> {
        tokio::fs::create_dir_all(&config.upload_dir).await?;
        let db = db::connect(&config.database_url).await?;
        Ok(Self {
            db,
            upload_dir: config.upload_dir.clone(),
        })
    }
}
