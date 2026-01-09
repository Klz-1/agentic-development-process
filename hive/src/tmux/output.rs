//! Output capture and streaming
//!
//! This module provides efficient output capture from tmux sessions using
//! a ring buffer to store the last N lines and diff detection to only
//! process new content.

use super::client::TmuxClient;
use crate::utils::RingBuffer;
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use tokio::time::{interval, Duration};

/// Handle to control a running capture task
pub struct CaptureHandle {
    /// Sender to signal the capture task to stop
    stop_tx: watch::Sender<bool>,
}

impl CaptureHandle {
    /// Stop the capture task
    pub fn stop(&self) {
        let _ = self.stop_tx.send(true);
    }
}

/// State for tracking output changes
struct CaptureState {
    /// Hash of the last captured content for quick comparison
    last_hash: u64,
    /// Last captured lines for diff computation
    last_lines: Vec<String>,
    /// Number of consecutive empty captures (for detecting disconnection)
    empty_count: usize,
}

impl CaptureState {
    fn new() -> Self {
        Self {
            last_hash: 0,
            last_lines: Vec::new(),
            empty_count: 0,
        }
    }

    /// Compute hash of lines for quick comparison
    fn hash_lines(lines: &[String]) -> u64 {
        let mut hasher = DefaultHasher::new();
        lines.hash(&mut hasher);
        hasher.finish()
    }
}

/// Manages output capture for sessions
pub struct OutputCapture {
    client: TmuxClient,
    buffer_size: usize,
}

impl OutputCapture {
    /// Create a new output capture manager
    pub fn new(buffer_size: usize) -> Self {
        Self {
            client: TmuxClient::new(),
            buffer_size,
        }
    }

    /// Start capturing output for a session
    /// Returns a handle that can be used to stop the capture
    pub fn start_capture(
        &self,
        session: String,
        buffer: Arc<Mutex<RingBuffer<String>>>,
        poll_interval_ms: u64,
    ) -> CaptureHandle {
        let client = self.client.clone();
        let buffer_size = self.buffer_size;
        let (stop_tx, mut stop_rx) = watch::channel(false);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(poll_interval_ms));
            let mut state = CaptureState::new();
            const MAX_EMPTY_COUNT: usize = 10;

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        match client.capture_output(&session, buffer_size).await {
                            Ok(lines) => {
                                // Quick hash check to avoid expensive diff on unchanged content
                                let new_hash = CaptureState::hash_lines(&lines);

                                if new_hash != state.last_hash {
                                    // Content changed, compute diff
                                    let new_lines = Self::diff_lines(&state.last_lines, &lines);

                                    if !new_lines.is_empty() {
                                        let mut buf = buffer.lock().await;
                                        for line in new_lines {
                                            buf.push(line);
                                        }
                                    }

                                    state.last_hash = new_hash;
                                    state.last_lines = lines;
                                    state.empty_count = 0;
                                } else if lines.is_empty() {
                                    state.empty_count += 1;
                                    if state.empty_count >= MAX_EMPTY_COUNT {
                                        // Session likely disconnected
                                        tracing::debug!(
                                            "Session {} appears disconnected after {} empty captures",
                                            session,
                                            state.empty_count
                                        );
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to capture output for {}: {}", session, e);
                                // Session might have been closed
                                break;
                            }
                        }
                    }
                    _ = stop_rx.changed() => {
                        if *stop_rx.borrow() {
                            tracing::debug!("Capture stopped for session {}", session);
                            break;
                        }
                    }
                }
            }
        });

        CaptureHandle { stop_tx }
    }

    /// Capture output once (non-streaming)
    pub async fn capture_once(&self, session: &str) -> Result<Vec<String>> {
        self.client.capture_output(session, self.buffer_size).await
    }

    /// Find lines that are new compared to the previous capture
    fn diff_lines(old: &[String], new: &[String]) -> Vec<String> {
        if old.is_empty() {
            return new.to_vec();
        }

        if new.is_empty() {
            return Vec::new();
        }

        // Try to find the overlap point using multiple lines for better accuracy
        // This helps when the last line of old appears multiple times in new
        let overlap_size = 3.min(old.len());
        let old_suffix: Vec<&String> = old.iter().rev().take(overlap_size).collect();

        // Search for the suffix pattern in new
        for (i, _) in new.iter().enumerate() {
            if i + overlap_size > new.len() {
                break;
            }

            // Check if this position matches our old suffix
            let mut matches = true;
            for (j, old_line) in old_suffix.iter().rev().enumerate() {
                if &new[i + j] != *old_line {
                    matches = false;
                    break;
                }
            }

            if matches {
                // Found the overlap, return everything after
                let start = i + overlap_size;
                if start < new.len() {
                    return new[start..].to_vec();
                } else {
                    return Vec::new();
                }
            }
        }

        // Fallback: simple last-line comparison
        if let Some(last_old) = old.last() {
            if let Some(pos) = new.iter().rposition(|l| l == last_old) {
                if pos + 1 < new.len() {
                    return new[pos + 1..].to_vec();
                }
                return Vec::new();
            }
        }

        // If we can't find the overlap, this might be a complete refresh
        // Return all new lines
        new.to_vec()
    }
}

impl Default for OutputCapture {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl Clone for OutputCapture {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            buffer_size: self.buffer_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_lines_empty_old() {
        let old: Vec<String> = vec![];
        let new = vec!["line1".to_string(), "line2".to_string()];
        let diff = OutputCapture::diff_lines(&old, &new);
        assert_eq!(diff, new);
    }

    #[test]
    fn test_diff_lines_empty_new() {
        let old = vec!["line1".to_string()];
        let new: Vec<String> = vec![];
        let diff = OutputCapture::diff_lines(&old, &new);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_diff_lines_no_change() {
        let old = vec!["line1".to_string(), "line2".to_string()];
        let new = vec!["line1".to_string(), "line2".to_string()];
        let diff = OutputCapture::diff_lines(&old, &new);
        assert!(diff.is_empty());
    }

    #[test]
    fn test_diff_lines_new_content() {
        let old = vec!["line1".to_string(), "line2".to_string()];
        let new = vec![
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
        ];
        let diff = OutputCapture::diff_lines(&old, &new);
        assert_eq!(diff, vec!["line3".to_string()]);
    }

    #[test]
    fn test_diff_lines_scrolled() {
        let old = vec!["line1".to_string(), "line2".to_string()];
        let new = vec![
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];
        let diff = OutputCapture::diff_lines(&old, &new);
        assert_eq!(diff, vec!["line3".to_string(), "line4".to_string()]);
    }

    #[test]
    fn test_diff_lines_complete_refresh() {
        let old = vec!["old1".to_string(), "old2".to_string()];
        let new = vec!["new1".to_string(), "new2".to_string()];
        let diff = OutputCapture::diff_lines(&old, &new);
        // When no overlap is found, return all new lines
        assert_eq!(diff, new);
    }

    #[test]
    fn test_hash_consistency() {
        let lines1 = vec!["line1".to_string(), "line2".to_string()];
        let lines2 = vec!["line1".to_string(), "line2".to_string()];
        let lines3 = vec!["line1".to_string(), "line3".to_string()];

        assert_eq!(
            CaptureState::hash_lines(&lines1),
            CaptureState::hash_lines(&lines2)
        );
        assert_ne!(
            CaptureState::hash_lines(&lines1),
            CaptureState::hash_lines(&lines3)
        );
    }
}
