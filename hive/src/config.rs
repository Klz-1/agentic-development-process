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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.general.refresh_rate_ms > 0);
        assert!(config.sessions.output_buffer_lines > 0);
        assert!(!config.files.show_hidden);
        assert!(config.files.git_status);
    }

    #[test]
    fn test_layout_order_default() {
        let order = LayoutOrder::default();
        assert_eq!(order, LayoutOrder::SessionsOutputFiles);
    }

    #[test]
    fn test_layout_order_panel_order() {
        assert_eq!(LayoutOrder::SessionsOutputFiles.panel_order(), [0, 1, 2]);
        assert_eq!(LayoutOrder::OutputSessionsFiles.panel_order(), [1, 0, 2]);
        assert_eq!(LayoutOrder::FilesOutputSessions.panel_order(), [2, 1, 0]);
        assert_eq!(LayoutOrder::SessionsFilesOutput.panel_order(), [0, 2, 1]);
    }

    #[test]
    fn test_alerts_config_default() {
        let alerts = AlertsConfig::default();
        assert!(alerts.visual);
        assert!(!alerts.sound);
    }

    #[test]
    fn test_parse_config_toml() {
        let toml = r#"
            [general]
            refresh_rate_ms = 250
            layout_order = "output-sessions-files"

            [sessions]
            show_resource_usage = false
            output_buffer_lines = 500

            [files]
            show_hidden = true
            git_status = false

            [alerts]
            visual = true
            sound = true
        "#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.general.refresh_rate_ms, 250);
        assert_eq!(config.general.layout_order, LayoutOrder::OutputSessionsFiles);
        assert!(!config.sessions.show_resource_usage);
        assert_eq!(config.sessions.output_buffer_lines, 500);
        assert!(config.files.show_hidden);
        assert!(!config.files.git_status);
        assert!(config.alerts.visual);
        assert!(config.alerts.sound);
    }

    #[test]
    fn test_parse_partial_config() {
        // Only specify some fields, rest should be defaults
        let toml = r#"
            [general]
            refresh_rate_ms = 100
        "#;

        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.general.refresh_rate_ms, 100);
        // Check defaults are applied
        assert!(config.sessions.show_resource_usage);
        assert!(!config.files.show_hidden);
    }

    #[test]
    fn test_load_config_missing_file() {
        // Should return default config when file doesn't exist
        let result = load_config();
        assert!(result.is_ok());
    }
}
