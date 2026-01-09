//! Hive - Terminal UI for monitoring AI agent sessions
//!
//! A production-grade TUI for monitoring multiple tmux sessions running AI agents,
//! with integrated file exploration and git awareness.

mod app;
mod config;
mod event;
mod files;
mod tmux;
mod ui;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::load_config()?;

    // Run the application
    app::run(config).await
}
