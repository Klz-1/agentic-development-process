//! Configuration loading and defaults

use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub sessions: SessionsConfig,
    pub files: FilesConfig,
    pub keybindings: KeybindingsConfig,
    pub theme: ThemeConfig,
    pub alerts: AlertsConfig,
}

/// Panel layout order options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutOrder {
    /// Sessions | Output | Files (default)
    #[default]
    SessionsOutputFiles,
    /// Output | Sessions | Files
    OutputSessionsFiles,
    /// Files | Output | Sessions
    FilesOutputSessions,
    /// Sessions | Files | Output
    SessionsFilesOutput,
}

impl LayoutOrder {
    /// Get the panel order as indices (0=sessions, 1=output, 2=files)
    pub fn panel_order(&self) -> [usize; 3] {
        match self {
            Self::SessionsOutputFiles => [0, 1, 2],
            Self::OutputSessionsFiles => [1, 0, 2],
            Self::FilesOutputSessions => [2, 1, 0],
            Self::SessionsFilesOutput => [0, 2, 1],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Root directory for projects
    pub project_root: PathBuf,
    /// Refresh rate in milliseconds
    pub refresh_rate_ms: u64,
    /// Default panel layout style
    pub default_layout: String,
    /// Panel arrangement order
    pub layout_order: LayoutOrder,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            project_root: directories::UserDirs::new()
                .map(|dirs| dirs.home_dir().to_path_buf())
                .unwrap_or_default()
                .join("projects"),
            refresh_rate_ms: 500,
            default_layout: "three-panel".to_string(),
            layout_order: LayoutOrder::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SessionsConfig {
    /// Show CPU/memory usage
    pub show_resource_usage: bool,
    /// Show progress indicators
    pub show_progress: bool,
    /// Number of output lines to buffer
    pub output_buffer_lines: usize,
}

impl Default for SessionsConfig {
    fn default() -> Self {
        Self {
            show_resource_usage: true,
            show_progress: true,
            output_buffer_lines: 1000,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct FilesConfig {
    /// Show hidden files
    pub show_hidden: bool,
    /// Show git status
    pub git_status: bool,
    /// Enable syntax highlighting
    pub syntax_highlighting: bool,
    /// Lines to show in preview
    pub preview_lines: usize,
}

impl Default for FilesConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            git_status: true,
            syntax_highlighting: true,
            preview_lines: 20,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct KeybindingsConfig {
    pub quit: String,
    pub help: String,
    pub fuzzy_find: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ThemeConfig {
    pub accent: Option<String>,
    pub error: Option<String>,
    pub success: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AlertsConfig {
    pub visual: bool,
    pub sound: bool,
}

impl Default for AlertsConfig {
    fn default() -> Self {
        Self {
            visual: true,
            sound: false,
        }
    }
}

/// Load configuration from file or use defaults
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();

    if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

/// Get the configuration file path
fn get_config_path() -> PathBuf {
    directories::ProjectDirs::from("", "", "hive")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .unwrap_or_else(|| PathBuf::from("~/.config/hive/config.toml"))
}
