//! Session data structures

use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Represents a tmux session
#[derive(Debug, Clone)]
pub struct Session {
    /// Session name
    pub name: String,
    /// Session ID
    pub id: String,
    /// Project root directory
    pub project_root: PathBuf,
    /// Current status
    pub status: SessionStatus,
    /// Resource usage
    pub resource_usage: ResourceUsage,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// Process ID of the main pane
    pub pane_pid: Option<u32>,
}

/// Session status
#[derive(Debug, Clone)]
pub enum SessionStatus {
    /// Session is actively running
    Running {
        /// Optional progress information
        progress: Option<Progress>,
    },
    /// Session is idle
    Idle {
        /// When the session became idle
        since: DateTime<Utc>,
    },
    /// Session has an error
    Error {
        /// Error message
        message: String,
    },
    /// Session completed successfully
    Completed {
        /// When the session completed
        at: DateTime<Utc>,
    },
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Idle { since: Utc::now() }
    }
}

/// Progress information for a running session
#[derive(Debug, Clone)]
pub struct Progress {
    /// Current step
    pub current: u32,
    /// Total steps
    pub total: u32,
    /// Optional label
    pub label: Option<String>,
}

impl Progress {
    /// Calculate progress percentage (0.0 - 1.0)
    pub fn percentage(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.current as f32 / self.total as f32
        }
    }
}

/// Resource usage information
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: f32,
    /// Memory usage in MB
    pub memory_mb: u32,
}
