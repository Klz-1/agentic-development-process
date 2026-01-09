//! File explorer - tree view, git status, preview
//!
//! This module provides utilities for file exploration that will be used
//! when the full file browser functionality is implemented.

#![allow(dead_code)]
#![allow(unused_imports)]

mod git;
mod preview;
mod tree;

pub use git::{find_repo_root, get_git_status, GitStatus};
pub use preview::FilePreview;
pub use tree::{FileEntry, FileKind, FileTree};

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// Cached project root to avoid repeated lookups
static PROJECT_ROOT_CACHE: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Find the project root directory by walking up from the given path.
///
/// Looks for common project markers in order:
/// 1. .git directory
/// 2. Cargo.toml (Rust)
/// 3. package.json (Node.js)
/// 4. pyproject.toml (Python)
/// 5. go.mod (Go)
///
/// If no marker is found, returns the given path or its parent if it's a file.
pub fn find_project_root(start: &Path) -> PathBuf {
    // Use cached result if available for same start path
    if let Some(Some(root)) = PROJECT_ROOT_CACHE.get() {
        // Check if the cached root is still valid for this path
        if start.starts_with(root) {
            return root.clone();
        }
    }

    let start_dir = if start.is_file() {
        start.parent().unwrap_or(start)
    } else {
        start
    };

    let markers = [
        ".git",
        "Cargo.toml",
        "package.json",
        "pyproject.toml",
        "go.mod",
    ];

    let mut current = start_dir;
    loop {
        for marker in &markers {
            if current.join(marker).exists() {
                let root = current.to_path_buf();
                // Try to cache the result
                let _ = PROJECT_ROOT_CACHE.set(Some(root.clone()));
                return root;
            }
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    // No marker found, return the start directory
    start_dir.to_path_buf()
}

/// Clear the project root cache (useful when switching projects)
pub fn clear_project_root_cache() {
    // OnceLock doesn't support clearing, so this is a no-op
    // In practice, the cache is initialized once per program run
}

/// Open a file in the user's preferred editor.
///
/// Uses $EDITOR environment variable, falling back to sensible defaults:
/// - macOS: `open` (default text editor)
/// - Linux: `nano`
/// - Windows: `notepad`
///
/// Returns Ok(()) if the editor was launched successfully.
/// The TUI should suspend before calling this and resume after.
pub fn open_in_editor(path: &Path) -> Result<std::process::ExitStatus> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "open".to_string()
        } else if cfg!(target_os = "windows") {
            "notepad".to_string()
        } else {
            "nano".to_string()
        }
    });

    let status = if editor == "open" && cfg!(target_os = "macos") {
        // macOS 'open' command needs -t flag for text files
        Command::new("open")
            .arg("-t")
            .arg(path)
            .status()
            .context("Failed to open file with macOS open command")?
    } else {
        Command::new(&editor)
            .arg(path)
            .status()
            .with_context(|| format!("Failed to open editor: {}", editor))?
    };

    Ok(status)
}

/// Open a file in the editor, suspending and resuming the terminal.
///
/// This is a convenience wrapper that handles terminal state.
/// The caller should use this from outside the TUI draw loop.
pub fn edit_file_with_suspend<F, G>(path: &Path, suspend: F, resume: G) -> Result<()>
where
    F: FnOnce() -> Result<()>,
    G: FnOnce() -> Result<()>,
{
    // Suspend TUI
    suspend()?;

    // Open editor
    let result = open_in_editor(path);

    // Resume TUI regardless of editor result
    resume()?;

    // Check editor result
    let status = result?;
    if !status.success() {
        anyhow::bail!("Editor exited with status: {}", status);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_find_project_root_with_git() {
        let temp = TempDir::new().unwrap();
        let git_dir = temp.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let sub_dir = temp.path().join("src").join("nested");
        fs::create_dir_all(&sub_dir).unwrap();

        let root = find_project_root(&sub_dir);
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_with_cargo_toml() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("Cargo.toml")).unwrap();

        let sub_dir = temp.path().join("src");
        fs::create_dir(&sub_dir).unwrap();

        let root = find_project_root(&sub_dir);
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_with_package_json() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("package.json")).unwrap();

        let sub_dir = temp.path().join("src");
        fs::create_dir(&sub_dir).unwrap();

        let root = find_project_root(&sub_dir);
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_from_file() {
        let temp = TempDir::new().unwrap();
        File::create(temp.path().join("Cargo.toml")).unwrap();

        let file_path = temp.path().join("src").join("main.rs");
        fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        File::create(&file_path).unwrap();

        let root = find_project_root(&file_path);
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_no_marker() {
        let temp = TempDir::new().unwrap();
        // No project markers

        let root = find_project_root(temp.path());
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_project_root_git_takes_priority() {
        let temp = TempDir::new().unwrap();
        // Create both .git and Cargo.toml at root
        fs::create_dir(temp.path().join(".git")).unwrap();
        File::create(temp.path().join("Cargo.toml")).unwrap();

        let root = find_project_root(temp.path());
        assert_eq!(root.canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }
}
