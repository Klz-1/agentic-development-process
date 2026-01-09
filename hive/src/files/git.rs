//! Git status integration

use anyhow::Result;
use git2::{Repository, StatusOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Git status for a file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    /// File has been modified
    Modified,
    /// File has been staged
    Staged,
    /// File is untracked
    Untracked,
    /// File has conflicts
    Conflicted,
}

/// Get git status for all files in a repository
pub fn get_git_status(repo_path: &Path) -> Result<HashMap<PathBuf, GitStatus>> {
    let repo = Repository::discover(repo_path)?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow::anyhow!("Repository has no working directory"))?;

    let mut status_map = HashMap::new();

    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false);

    let statuses = repo.statuses(Some(&mut opts))?;

    for entry in statuses.iter() {
        let Some(path_str) = entry.path() else {
            continue;
        };

        // Use the actual workdir for consistent paths
        let path = workdir.join(path_str);
        let status = entry.status();

        let git_status = if status.is_conflicted() {
            GitStatus::Conflicted
        } else if status.is_index_new() || status.is_index_modified() || status.is_index_deleted() {
            GitStatus::Staged
        } else if status.is_wt_new() {
            GitStatus::Untracked
        } else if status.is_wt_modified() || status.is_wt_deleted() {
            GitStatus::Modified
        } else {
            continue;
        };

        status_map.insert(path, git_status);
    }

    Ok(status_map)
}

/// Find the git repository root from a given path
pub fn find_repo_root(path: &Path) -> Option<PathBuf> {
    Repository::discover(path)
        .ok()
        .and_then(|repo| repo.workdir().map(|p| p.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    fn init_test_repo() -> TempDir {
        let temp = TempDir::new().unwrap();
        let repo = Repository::init(temp.path()).unwrap();

        // Configure user for commits
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();

        // Create initial commit to set up HEAD
        {
            let mut index = repo.index().unwrap();
            let tree_id = index.write_tree().unwrap();
            let tree = repo.find_tree(tree_id).unwrap();
            let sig = repo.signature().unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
                .unwrap();
        }

        temp
    }

    #[test]
    fn test_find_repo_root() {
        let temp = init_test_repo();

        // Create a subdirectory
        let sub_dir = temp.path().join("src");
        fs::create_dir(&sub_dir).unwrap();

        // find_repo_root should work from subdirectory
        let root = find_repo_root(&sub_dir);
        assert!(root.is_some());
        assert_eq!(root.unwrap().canonicalize().unwrap(), temp.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_repo_root_not_a_repo() {
        let temp = TempDir::new().unwrap();
        // This is just a regular directory, not a git repo

        let root = find_repo_root(temp.path());
        assert!(root.is_none());
    }

    #[test]
    fn test_get_git_status_untracked() {
        let temp = init_test_repo();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let root = temp.path().canonicalize().unwrap();

        // Create an untracked file
        let file_path = root.join("untracked.txt");
        File::create(&file_path).unwrap();

        let status = get_git_status(&root).unwrap();
        assert_eq!(status.get(&file_path), Some(&GitStatus::Untracked));
    }

    #[test]
    fn test_get_git_status_staged() {
        let temp = init_test_repo();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let root = temp.path().canonicalize().unwrap();
        let repo = Repository::open(&root).unwrap();

        // Create and stage a file
        let file_path = root.join("staged.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "content").unwrap();
        }

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("staged.txt")).unwrap();
        index.write().unwrap();

        let status = get_git_status(&root).unwrap();
        assert_eq!(status.get(&file_path), Some(&GitStatus::Staged));
    }

    #[test]
    fn test_get_git_status_modified() {
        let temp = init_test_repo();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let root = temp.path().canonicalize().unwrap();
        let repo = Repository::open(&root).unwrap();

        // Create, commit, then modify a file
        let file_path = root.join("modified.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "original content").unwrap();
        }

        // Stage and commit
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("modified.txt")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = repo.signature().unwrap();
        let parent = repo.head().unwrap().peel_to_commit().unwrap();
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Add modified.txt",
            &tree,
            &[&parent],
        )
        .unwrap();

        // Now modify the file
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "modified content").unwrap();
        }

        let status = get_git_status(&root).unwrap();
        assert_eq!(status.get(&file_path), Some(&GitStatus::Modified));
    }

    #[test]
    fn test_get_git_status_not_a_repo() {
        let temp = TempDir::new().unwrap();
        // This is just a regular directory, not a git repo

        let result = get_git_status(temp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_git_status_from_subdirectory() {
        let temp = init_test_repo();
        // Canonicalize to handle macOS /var -> /private/var symlinks
        let root = temp.path().canonicalize().unwrap();

        // Create a subdirectory and file
        let sub_dir = root.join("src");
        fs::create_dir(&sub_dir).unwrap();
        let file_path = sub_dir.join("lib.rs");
        File::create(&file_path).unwrap();

        // Get status from subdirectory
        let status = get_git_status(&sub_dir).unwrap();
        assert_eq!(status.get(&file_path), Some(&GitStatus::Untracked));
    }
}
