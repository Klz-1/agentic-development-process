//! tmux integration - session discovery, output capture, and interaction
//!
//! This module provides comprehensive tmux integration for the Hive TUI:
//! - Session discovery and enumeration
//! - Output streaming with efficient diff detection
//! - Resource monitoring (CPU, memory) via sysinfo
//! - Session interaction (send keys, interrupt, attach)

#![allow(dead_code)]
#![allow(unused_imports)]

mod client;
mod output;
mod session;

pub use client::TmuxClient;
pub use output::{CaptureHandle, OutputCapture};
pub use session::{Progress, ResourceUsage, Session, SessionStatus};
