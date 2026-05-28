mod app;
mod config;
mod db;
mod dto;
mod error;
mod handlers;
mod middleware;
mod models;
mod state;
mod storage;

#[cfg(test)]
mod testing;

use anyhow::Context;
use config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "issue_tracker_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let state = state::AppState::new(&config).await?;
    let app = app::build_app(state, config.api_key.clone());
    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
        .await
        .with_context(|| format!("failed to bind {}", config.bind_addr))?;

    tracing::info!("issue tracker api listening on http://{}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
