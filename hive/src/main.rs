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

use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::Write;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Command line arguments
struct Args {
    /// Enable debug logging to file
    debug: bool,
    /// Show version and exit
    version: bool,
    /// Show help and exit
    help: bool,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        Self {
            debug: args.iter().any(|a| a == "--debug" || a == "-d"),
            version: args.iter().any(|a| a == "--version" || a == "-V"),
            help: args.iter().any(|a| a == "--help" || a == "-h"),
        }
    }
}

fn print_help() {
    println!(
        r#"hive - Terminal UI for monitoring AI agent sessions

USAGE:
    hive [OPTIONS]

OPTIONS:
    -d, --debug      Enable debug logging to ~/.local/share/hive/hive.log
    -V, --version    Print version information
    -h, --help       Print this help message

KEYBINDINGS:
    Tab/Shift+Tab    Switch between panels
    ↑/↓ or j/k       Navigate within panel
    Enter            Primary action (attach, expand, select)
    ?                Show help overlay
    /                Open fuzzy finder
    e                Open file in $EDITOR (Files panel)
    .                Toggle hidden files (Files panel)
    p                Toggle auto-scroll (Output panel)
    q                Quit

CONFIGURATION:
    Config file: ~/.config/hive/config.toml

For more information, visit: https://github.com/youruser/hive
"#
    );
}

fn print_version() {
    println!("hive {}", env!("CARGO_PKG_VERSION"));
}

/// Initialize logging with optional file output
fn init_logging(debug: bool) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            if debug {
                EnvFilter::new("hive=debug,warn")
            } else {
                EnvFilter::new("warn")
            }
        });

    if debug {
        // Create log directory
        let log_dir = directories::ProjectDirs::from("", "", "hive")
            .map(|dirs| dirs.data_dir().to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        std::fs::create_dir_all(&log_dir)
            .context("Failed to create log directory")?;

        let log_path = log_dir.join("hive.log");

        // Open log file
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .context("Failed to open log file")?;

        // Write startup marker
        let mut file = log_file.try_clone()?;
        writeln!(file, "\n--- Hive started at {} ---", chrono::Local::now())?;

        // Set up file logging
        let file_layer = fmt::layer()
            .with_writer(move || log_file.try_clone().unwrap())
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(file_layer)
            .init();

        eprintln!("Debug logging enabled: {}", log_path.display());
    } else {
        // Simple stderr logging for warnings/errors only
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().with_writer(std::io::stderr))
            .init();
    }

    Ok(())
}

/// Check system requirements and provide helpful error messages
fn check_requirements() -> Result<()> {
    // Check if tmux is available
    match std::process::Command::new("tmux")
        .arg("-V")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                tracing::info!("Found {}", version.trim());
            }
        }
        Err(e) => {
            tracing::warn!("tmux not found: {}. Session monitoring will be limited.", e);
            eprintln!(
                "Warning: tmux not found. Install tmux for full functionality.\n\
                 On macOS: brew install tmux\n\
                 On Ubuntu: sudo apt install tmux\n"
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.help {
        print_help();
        return Ok(());
    }

    if args.version {
        print_version();
        return Ok(());
    }

    // Initialize logging
    init_logging(args.debug)?;

    tracing::info!("Hive {} starting", env!("CARGO_PKG_VERSION"));

    // Check system requirements
    check_requirements()?;

    // Load configuration with helpful error message
    let config = config::load_config()
        .context("Failed to load configuration. Check ~/.config/hive/config.toml for syntax errors.")?;

    tracing::debug!("Configuration loaded: {:?}", config.general);

    // Run the application with panic handler
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tokio::runtime::Handle::current().block_on(async {
            app::run(config).await
        })
    }));

    match result {
        Ok(Ok(())) => {
            tracing::info!("Hive exited normally");
            Ok(())
        }
        Ok(Err(e)) => {
            tracing::error!("Application error: {:?}", e);
            Err(e).context("Hive encountered an error")
        }
        Err(panic) => {
            // Try to extract panic message
            let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            tracing::error!("Panic: {}", msg);
            eprintln!("\nHive crashed unexpectedly: {}", msg);
            eprintln!("Please report this issue with the log file from ~/.local/share/hive/hive.log");

            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_empty() {
        // This would need to be tested differently since it reads from env
        // Just ensure the struct can be created
        let args = Args {
            debug: false,
            version: false,
            help: false,
        };
        assert!(!args.debug);
    }
}
