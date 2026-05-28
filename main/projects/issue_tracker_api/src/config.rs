use std::{env, net::SocketAddr, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub upload_dir: PathBuf,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let bind_addr = env::var("ISSUE_TRACKER_BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3001".to_string())
            .parse()
            .expect("ISSUE_TRACKER_BIND_ADDR must be a valid socket address");

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db_path = crate_dir.join("data/issue_tracker.db");
            format!("sqlite://{}", db_path.display())
        });

        let upload_dir = env::var("ISSUE_TRACKER_UPLOAD_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| crate_dir.join("storage/uploads"));

        let api_key =
            env::var("ISSUE_TRACKER_API_KEY").unwrap_or_else(|_| "dev-secret".to_string());

        Self {
            bind_addr,
            database_url,
            upload_dir,
            api_key,
        }
    }
}
