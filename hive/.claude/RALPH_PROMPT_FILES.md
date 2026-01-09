# Task: Hive TUI - Phase 3: File Explorer

## Context

- Project: /Users/klz/Desktop/Prototypes/agentic-development-process/hive
- Stack: Rust + git2 crate + std::fs
- Primary files: src/files/\*.rs

## Overview

Complete the file explorer module for Hive. The skeleton exists but needs full implementation for tree building, git status, and file preview.

## Tasks (Complete ALL)

### Task 1: Fix Module Compilation

- Ensure src/files/mod.rs and all submodules compile
- Fix any type errors or missing imports
- Run `cargo check` to verify

### Task 2: File Tree Implementation

- Complete FileTree in src/files/tree.rs
- Implement lazy loading (only read directories when expanded)
- Add proper sorting: directories first, then alphabetical
- Handle hidden files (configurable show/hide)
- Implement expand/collapse toggle

### Task 3: Git Status Integration

- Complete get_git_status() in src/files/git.rs
- Use git2 crate to read repository status
- Map git2::Status to our GitStatus enum
- Handle non-git directories gracefully
- Find repo root from any subdirectory

### Task 4: File Preview

- Complete FilePreview in src/files/preview.rs
- Read first N lines of text files
- Detect binary files and show placeholder
- Handle read errors gracefully
- Truncate very long lines

### Task 5: Project Root Detection

- Implement find_project_root() function
- Walk up directory tree to find .git
- Fall back to configured project_root
- Cache results to avoid repeated lookups

### Task 6: External Editor Integration

- Add open_in_editor() function
- Read $EDITOR or fall back to sensible default
- Properly suspend TUI, run editor, resume TUI
- Handle editor exit codes

## Technical Requirements

- Use git2 crate for git operations (already in Cargo.toml)
- No blocking operations in async context
- Proper error handling with anyhow
- Memory efficient for large directories

## Testing Guidelines

1. Run `cargo check` after each change
2. Run `cargo clippy` for linting
3. Manual testing:
   ```bash
   # Test on the hive project itself
   # It has .git and various files
   ```
4. Verify git status shows [M], [+], [?] correctly
5. Test file preview on various file types

## Verification Commands

```bash
cd /Users/klz/Desktop/Prototypes/agentic-development-process/hive
cargo check
cargo clippy -- -D warnings
cargo test --lib

# Test git commands manually
git status --porcelain
```

## Permissions

### Allowed:

- Modify any file in src/files/
- Add helper modules in src/files/
- Run cargo commands
- Read any files for testing

### NOT Allowed:

- Modify UI code (src/ui/)
- Modify tmux code (src/tmux/)
- Add new crate dependencies
- Write/delete files outside the project
- Git operations (commit, push, etc.)

## Completion Criteria

When ALL of the following are true:

- [ ] `cargo check` passes with no errors
- [ ] `cargo clippy -- -D warnings` passes
- [ ] FileTree builds correctly for any directory
- [ ] Git status shows correct indicators
- [ ] File preview works for text files
- [ ] Binary files are detected and handled

Output: <promise>HIVE_FILES_COMPLETE</promise>
