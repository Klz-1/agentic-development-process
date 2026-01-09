//! tmux client - IPC with tmux server
//!
//! This module provides async interaction with the tmux server using
//! `tokio::process::Command` for non-blocking I/O. All command execution
//! uses the argument array form (not shell strings) to prevent injection.

use super::session::{ResourceUsage, Session, SessionStatus};
use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};
use tokio::process::Command;
use tokio::sync::Mutex;

/// Idle threshold in seconds - sessions with no activity for this long are considered idle
const IDLE_THRESHOLD_SECS: i64 = 60;

/// Cache duration for process info to avoid excessive polling
const PROCESS_CACHE_DURATION: Duration = Duration::from_secs(2);

/// Cached process information
struct ProcessCache {
    system: System,
    last_refresh: Instant,
    /// Cached resource usage by PID
    cache: HashMap<u32, (ResourceUsage, Instant)>,
}

impl ProcessCache {
    fn new() -> Self {
        Self {
            system: System::new(),
            last_refresh: Instant::now() - PROCESS_CACHE_DURATION,
            cache: HashMap::new(),
        }
    }

    fn get_resource_usage(&mut self, pid: u32) -> ResourceUsage {
        let now = Instant::now();

        // Check if we have a recent cached value
        if let Some((usage, cached_at)) = self.cache.get(&pid) {
            if now.duration_since(*cached_at) < PROCESS_CACHE_DURATION {
                return usage.clone();
            }
        }

        // Refresh system info if needed
        if now.duration_since(self.last_refresh) >= PROCESS_CACHE_DURATION {
            self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
            self.last_refresh = now;
        }

        // Get process info
        let usage = if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            ResourceUsage {
                cpu_percent: process.cpu_usage(),
                memory_mb: (process.memory() / (1024 * 1024)) as u32,
            }
        } else {
            ResourceUsage::default()
        };

        // Cache the result
        self.cache.insert(pid, (usage.clone(), now));
        usage
    }

    fn process_exists(&mut self, pid: u32) -> bool {
        let now = Instant::now();

        // Refresh system info if needed
        if now.duration_since(self.last_refresh) >= PROCESS_CACHE_DURATION {
            self.system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
            self.last_refresh = now;
        }

        self.system.process(Pid::from_u32(pid)).is_some()
    }
}

/// Client for interacting with tmux
pub struct TmuxClient {
    /// Cached process information
    process_cache: Arc<Mutex<ProcessCache>>,
}

impl TmuxClient {
    /// Create a new tmux client
    pub fn new() -> Self {
        Self {
            process_cache: Arc::new(Mutex::new(ProcessCache::new())),
        }
    }

    /// Check if tmux server is running
    pub async fn is_server_running(&self) -> bool {
        let output = Command::new("tmux")
            .args(["list-sessions"])
            .output()
            .await;

        matches!(output, Ok(o) if o.status.success())
    }

    /// List all tmux sessions
    pub async fn list_sessions(&self) -> Result<Vec<Session>> {
        let output = Command::new("tmux")
            .args([
                "list-sessions",
                "-F",
                "#{session_name}|#{session_id}|#{session_path}|#{session_activity}",
            ])
            .output()
            .await
            .context("Failed to execute tmux list-sessions")?;

        if !output.status.success() {
            // No tmux server running or no sessions
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("no server running") || stderr.contains("no sessions") {
                return Ok(vec![]);
            }
            // Return empty for other errors too (e.g., no sessions)
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut sessions: Vec<Session> = stdout
            .lines()
            .filter_map(|line| self.parse_session_line(line))
            .collect();

        // Enrich sessions with pane PIDs and status
        for session in &mut sessions {
            if let Ok(Some(pid)) = self.get_pane_pid(&session.name).await {
                session.pane_pid = Some(pid);

                // Get resource usage
                let mut cache = self.process_cache.lock().await;
                session.resource_usage = cache.get_resource_usage(pid);

                // Update status based on activity
                session.status = self.determine_status(session, &mut cache).await;
            }
        }

        Ok(sessions)
    }

    /// Parse a session line from tmux list-sessions
    fn parse_session_line(&self, line: &str) -> Option<Session> {
        // Using | as delimiter since paths may contain colons
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 4 {
            return None;
        }

        let name = parts[0].to_string();
        let id = parts[1].to_string();
        let path = PathBuf::from(parts[2]);

        // Parse activity timestamp (Unix timestamp in seconds)
        let activity_timestamp = parts[3].trim().parse::<i64>().ok()?;
        let last_activity = Utc.timestamp_opt(activity_timestamp, 0).single()?;

        Some(Session {
            name,
            id,
            project_root: path,
            status: SessionStatus::default(),
            resource_usage: ResourceUsage::default(),
            last_activity,
            pane_pid: None,
        })
    }

    /// Determine session status based on activity and process state
    async fn determine_status(&self, session: &Session, cache: &mut ProcessCache) -> SessionStatus {
        let now = Utc::now();
        let idle_duration = now.signed_duration_since(session.last_activity);

        // Check if process is still running
        let process_active = session
            .pane_pid
            .map(|pid| cache.process_exists(pid))
            .unwrap_or(false);

        if !process_active && session.pane_pid.is_some() {
            // Process ended - could be completed or error
            // For now, mark as completed
            return SessionStatus::Completed { at: now };
        }

        if idle_duration.num_seconds() > IDLE_THRESHOLD_SECS {
            SessionStatus::Idle {
                since: session.last_activity,
            }
        } else {
            SessionStatus::Running { progress: None }
        }
    }

    /// Capture output from a session's pane
    pub async fn capture_output(&self, session: &str, lines: usize) -> Result<Vec<String>> {
        let output = Command::new("tmux")
            .args([
                "capture-pane",
                "-t",
                session,
                "-p",
                "-S",
                &format!("-{}", lines),
            ])
            .output()
            .await
            .context("Failed to capture pane output")?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(String::from).collect())
    }

    /// Send keys to a session
    pub async fn send_keys(&self, session: &str, keys: &str) -> Result<()> {
        // Escape special characters for tmux
        let escaped_keys = self.escape_tmux_keys(keys);

        Command::new("tmux")
            .args(["send-keys", "-t", session, &escaped_keys, "Enter"])
            .status()
            .await
            .context("Failed to send keys to session")?;
        Ok(())
    }

    /// Send keys without pressing Enter
    pub async fn send_keys_raw(&self, session: &str, keys: &str) -> Result<()> {
        let escaped_keys = self.escape_tmux_keys(keys);

        Command::new("tmux")
            .args(["send-keys", "-t", session, &escaped_keys])
            .status()
            .await
            .context("Failed to send raw keys to session")?;
        Ok(())
    }

    /// Escape special characters for tmux send-keys
    fn escape_tmux_keys(&self, keys: &str) -> String {
        // tmux send-keys treats certain characters specially
        // Escape semicolons and other special chars
        keys.replace(';', "\\;")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
    }

    /// Send interrupt (Ctrl+C) to a session
    pub async fn send_interrupt(&self, session: &str) -> Result<()> {
        Command::new("tmux")
            .args(["send-keys", "-t", session, "C-c"])
            .status()
            .await
            .context("Failed to send interrupt to session")?;
        Ok(())
    }

    /// Send EOF (Ctrl+D) to a session
    pub async fn send_eof(&self, session: &str) -> Result<()> {
        Command::new("tmux")
            .args(["send-keys", "-t", session, "C-d"])
            .status()
            .await
            .context("Failed to send EOF to session")?;
        Ok(())
    }

    /// Get pane PID for a session
    pub async fn get_pane_pid(&self, session: &str) -> Result<Option<u32>> {
        let output = Command::new("tmux")
            .args(["list-panes", "-t", session, "-F", "#{pane_pid}"])
            .output()
            .await
            .context("Failed to get pane PID")?;

        if !output.status.success() {
            return Ok(None);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let pid = stdout.lines().next().and_then(|s| s.parse().ok());
        Ok(pid)
    }

    /// Get resource usage for a session
    pub async fn get_resource_usage(&self, session: &str) -> Result<ResourceUsage> {
        let pid = self.get_pane_pid(session).await?;

        match pid {
            Some(pid) => {
                let mut cache = self.process_cache.lock().await;
                Ok(cache.get_resource_usage(pid))
            }
            None => Ok(ResourceUsage::default()),
        }
    }

    /// Check if a session exists
    pub async fn session_exists(&self, session: &str) -> bool {
        let output = Command::new("tmux")
            .args(["has-session", "-t", session])
            .output()
            .await;

        matches!(output, Ok(o) if o.status.success())
    }

    /// Kill a session
    pub async fn kill_session(&self, session: &str) -> Result<()> {
        Command::new("tmux")
            .args(["kill-session", "-t", session])
            .status()
            .await
            .context("Failed to kill session")?;
        Ok(())
    }

    /// Attach to a session (replaces current process)
    /// Note: This should be called after restoring the terminal
    pub fn attach(&self, session: &str) -> Result<()> {
        use std::os::unix::process::CommandExt;

        // This replaces the current process with tmux attach
        // Uses std::process::Command (sync) because exec() never returns
        let err = std::process::Command::new("tmux")
            .args(["attach", "-t", session])
            .exec();

        // If we get here, the operation failed
        Err(err.into())
    }

    /// Create a new session
    pub async fn new_session(&self, name: &str, start_dir: Option<&str>) -> Result<()> {
        let mut args = vec!["new-session", "-d", "-s", name];

        if let Some(dir) = start_dir {
            args.push("-c");
            args.push(dir);
        }

        Command::new("tmux")
            .args(&args)
            .status()
            .await
            .context("Failed to create new session")?;
        Ok(())
    }
}

impl Default for TmuxClient {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TmuxClient {
    fn clone(&self) -> Self {
        Self {
            process_cache: Arc::clone(&self.process_cache),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_session_line() {
        let client = TmuxClient::new();

        // Test valid line
        let line = "test-session|$0|/home/user/project|1704067200";
        let session = client.parse_session_line(line);
        assert!(session.is_some());

        let session = session.unwrap();
        assert_eq!(session.name, "test-session");
        assert_eq!(session.id, "$0");
        assert_eq!(session.project_root, PathBuf::from("/home/user/project"));
    }

    #[tokio::test]
    async fn test_parse_session_line_with_colon_in_path() {
        let client = TmuxClient::new();

        // Test path with colon (unlikely but possible on some systems)
        let line = "session|$1|/path/to/dir|1704067200";
        let session = client.parse_session_line(line);
        assert!(session.is_some());
    }

    #[tokio::test]
    async fn test_parse_session_line_invalid() {
        let client = TmuxClient::new();

        // Test invalid line (not enough parts)
        let line = "invalid|line";
        let session = client.parse_session_line(line);
        assert!(session.is_none());
    }

    #[test]
    fn test_escape_tmux_keys() {
        let client = TmuxClient::new();

        assert_eq!(client.escape_tmux_keys("echo hello"), "echo hello");
        assert_eq!(
            client.escape_tmux_keys("cmd; echo done"),
            "cmd\\; echo done"
        );
        assert_eq!(
            client.escape_tmux_keys("echo \"test\""),
            "echo \\\"test\\\""
        );
    }
}
