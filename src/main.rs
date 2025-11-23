use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod arxiv;
mod config;
mod core;
mod db;
mod embeddings;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "arxiv_tui=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting arXiv TUI");

    // Load configuration
    let cfg = config::Config::load()?;
    tracing::info!("Configuration loaded");

    // Initialize application
    let mut app = app::App::new(cfg).await?;
    tracing::info!("Application initialized");

    // Run the TUI
    app.run().await?;

    tracing::info!("Shutting down");
    Ok(())
}
