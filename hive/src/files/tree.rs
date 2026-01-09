//! File tree building and management

use super::git::GitStatus;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the file tree for a project
#[derive(Debug, Default)]
pub struct FileTree {
    /// Root directory
    pub root: PathBuf,
    /// Flat list of visible entries
    pub entries: Vec<FileEntry>,
    /// Set of expanded directories
    pub expanded: HashSet<PathBuf>,
    /// Whether to show hidden files (starting with .)
    pub show_hidden: bool,
}

/// A single entry in the file tree
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Full path to the file/directory
    pub path: PathBuf,
    /// File or directory
    pub kind: FileKind,
    /// Depth in the tree (for indentation)
    pub depth: usize,
    /// Git status if applicable
    pub git_status: Option<GitStatus>,
}

/// Type of file entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Directory,
    File,
}

impl FileTree {
    /// Create a new file tree rooted at the given path
    pub fn new(root: PathBuf) -> Self {
        Self::with_options(root, false)
    }

    /// Create a new file tree with configuration options
    pub fn with_options(root: PathBuf, show_hidden: bool) -> Self {
        let mut tree = Self {
            root: root.clone(),
            entries: Vec::new(),
            expanded: HashSet::new(),
            show_hidden,
        };

        // Expand root by default
        tree.expanded.insert(root);
        tree.rebuild();
        tree
    }

    /// Toggle showing hidden files
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.rebuild();
    }

    /// Check if a directory is expanded
    pub fn is_expanded(&self, path: &Path) -> bool {
        self.expanded.contains(path)
    }

    /// Toggle expansion of a directory
    pub fn toggle_expand(&mut self, path: &Path) {
        if self.expanded.contains(path) {
            self.expanded.remove(path);
        } else {
            self.expanded.insert(path.to_path_buf());
        }
        self.rebuild();
    }

    /// Rebuild the flat entry list from the tree structure
    pub fn rebuild(&mut self) {
        self.entries.clear();
        self.build_entries(&self.root.clone(), 0);
    }

    fn build_entries(&mut self, dir: &Path, depth: usize) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };

        let mut entries: Vec<_> = entries
            .filter_map(|e| e.ok())
            .collect();

        // Sort: directories first, then alphabetically
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in entries {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            // Skip hidden files if not configured to show them
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }

            let kind = if path.is_dir() {
                FileKind::Directory
            } else {
                FileKind::File
            };

            self.entries.push(FileEntry {
                path: path.clone(),
                kind,
                depth,
                git_status: None, // Will be filled in later
            });

            // Recursively add children if expanded
            if kind == FileKind::Directory && self.expanded.contains(&path) {
                self.build_entries(&path, depth + 1);
            }
        }
    }

    /// Update git status for all entries
    pub fn update_git_status(&mut self, status: &std::collections::HashMap<PathBuf, GitStatus>) {
        for entry in &mut self.entries {
            entry.git_status = status.get(&entry.path).copied();
        }
    }

    /// Get display name for an entry (with tree characters)
    pub fn display_name(&self, index: usize) -> String {
        let Some(entry) = self.entries.get(index) else {
            return String::new();
        };

        let name = entry.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let indent = "│   ".repeat(entry.depth.saturating_sub(1));
        let prefix = if entry.depth == 0 {
            ""
        } else {
            "├── "
        };

        let suffix = match entry.kind {
            FileKind::Directory => "/",
            FileKind::File => "",
        };

        let git_indicator = match entry.git_status {
            Some(GitStatus::Modified) => " [M]",
            Some(GitStatus::Staged) => " [S]",
            Some(GitStatus::Untracked) => " [+]",
            Some(GitStatus::Conflicted) => " [!]",
            None => "",
        };

        format!("{}{}{}{}{}", indent, prefix, name, suffix, git_indicator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    fn create_test_structure() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create directories
        fs::create_dir(root.join("src")).unwrap();
        fs::create_dir(root.join("tests")).unwrap();
        fs::create_dir(root.join(".hidden")).unwrap();

        // Create files
        File::create(root.join("Cargo.toml")).unwrap();
        File::create(root.join("README.md")).unwrap();
        File::create(root.join("src/main.rs")).unwrap();
        File::create(root.join("src/lib.rs")).unwrap();
        File::create(root.join(".gitignore")).unwrap();

        temp
    }

    #[test]
    fn test_new_tree_expands_root() {
        let temp = create_test_structure();
        let tree = FileTree::new(temp.path().to_path_buf());

        assert!(tree.is_expanded(temp.path()));
        assert!(!tree.entries.is_empty());
    }

    #[test]
    fn test_hidden_files_hidden_by_default() {
        let temp = create_test_structure();
        let tree = FileTree::new(temp.path().to_path_buf());

        // .gitignore and .hidden should not appear
        for entry in &tree.entries {
            let name = entry.path.file_name().unwrap().to_string_lossy();
            assert!(!name.starts_with('.'), "Hidden file found: {}", name);
        }
    }

    #[test]
    fn test_show_hidden_files() {
        let temp = create_test_structure();
        let tree = FileTree::with_options(temp.path().to_path_buf(), true);

        // Now hidden files should appear
        let has_hidden = tree.entries.iter().any(|e| {
            e.path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with('.')
        });
        assert!(has_hidden, "No hidden files found when show_hidden=true");
    }

    #[test]
    fn test_toggle_hidden() {
        let temp = create_test_structure();
        let mut tree = FileTree::new(temp.path().to_path_buf());

        // Initially hidden files are not shown
        assert!(!tree.show_hidden);

        tree.toggle_hidden();
        assert!(tree.show_hidden);

        // Now hidden files should appear
        let has_hidden = tree.entries.iter().any(|e| {
            e.path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with('.')
        });
        assert!(has_hidden);
    }

    #[test]
    fn test_directories_sorted_first() {
        let temp = create_test_structure();
        let tree = FileTree::new(temp.path().to_path_buf());

        // Find the first file (non-directory) at depth 0
        let first_file_idx = tree
            .entries
            .iter()
            .position(|e| e.depth == 0 && e.kind == FileKind::File);

        let last_dir_idx = tree
            .entries
            .iter()
            .rposition(|e| e.depth == 0 && e.kind == FileKind::Directory);

        if let (Some(first_file), Some(last_dir)) = (first_file_idx, last_dir_idx) {
            assert!(
                last_dir < first_file,
                "Directories should come before files"
            );
        }
    }

    #[test]
    fn test_toggle_expand() {
        let temp = create_test_structure();
        let mut tree = FileTree::new(temp.path().to_path_buf());

        let src_path = temp.path().join("src");

        // Initially src is not expanded
        assert!(!tree.is_expanded(&src_path));

        // Expand it
        tree.toggle_expand(&src_path);
        assert!(tree.is_expanded(&src_path));

        // Check that children are now visible
        let has_main_rs = tree.entries.iter().any(|e| {
            e.path.file_name().unwrap().to_string_lossy() == "main.rs"
        });
        assert!(has_main_rs, "main.rs should be visible after expanding src");

        // Collapse it
        tree.toggle_expand(&src_path);
        assert!(!tree.is_expanded(&src_path));
    }

    #[test]
    fn test_display_name() {
        let temp = create_test_structure();
        let mut tree = FileTree::new(temp.path().to_path_buf());

        // Find src directory and expand it
        let src_idx = tree
            .entries
            .iter()
            .position(|e| e.path.file_name().unwrap().to_string_lossy() == "src")
            .expect("src directory not found");

        let src_path = tree.entries[src_idx].path.clone();
        tree.toggle_expand(&src_path);

        // Check display names
        let name = tree.display_name(src_idx);
        assert!(name.contains("src/"), "Directory should have trailing slash");

        // Find main.rs
        let main_rs_idx = tree
            .entries
            .iter()
            .position(|e| e.path.file_name().unwrap().to_string_lossy() == "main.rs")
            .expect("main.rs not found");

        let main_name = tree.display_name(main_rs_idx);
        assert!(main_name.contains("main.rs"));
        assert!(main_name.contains("├──"), "Nested files should have tree prefix");
    }

    #[test]
    fn test_update_git_status() {
        let temp = create_test_structure();
        let mut tree = FileTree::new(temp.path().to_path_buf());

        let mut status = std::collections::HashMap::new();
        let cargo_path = temp.path().join("Cargo.toml");
        status.insert(cargo_path.clone(), GitStatus::Modified);

        tree.update_git_status(&status);

        let cargo_entry = tree
            .entries
            .iter()
            .find(|e| e.path == cargo_path)
            .expect("Cargo.toml not found");

        assert_eq!(cargo_entry.git_status, Some(GitStatus::Modified));
    }
}
