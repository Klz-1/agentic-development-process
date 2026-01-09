# Task: Wire Up Real File Explorer

## Context

- Project: /Users/klz/Desktop/Prototypes/agentic-development-process/hive
- Current state: UI works with mock file tree, need real filesystem
- Files: src/files/\*.rs, src/ui/files.rs, src/app.rs

## Tasks

### 1. Connect FileTree to real filesystem

In src/files/tree.rs:

- FileTree::new(path) should read actual directory
- Lazy load: only read children when directory is expanded
- Sort: directories first, then alphabetical
- Handle permission errors gracefully

### 2. Wire git status with git2

In src/files/git.rs:

- Find repo root from any path (walk up to .git)
- Use git2::Repository::open()
- Get status with repo.statuses()
- Map git2::Status to our GitStatus enum
- Handle non-git directories (return empty HashMap)

### 3. Implement file preview

In src/files/preview.rs:

- Read first 20 lines of text files
- Detect binary files (check for null bytes)
- Return placeholder for binary files
- Handle read errors gracefully

### 4. Wire to App state

- When session changes, update file tree root to session's cwd
- Store FileTree in App struct
- Update files panel to use real FileTree instead of mock data

### 5. Connect UI to real data

In src/ui/files.rs:

- Replace MockFileEntry with real FileEntry from tree
- Use tree.display_name() for rendering
- Use tree.entries for list items

## Verification

```bash
cargo check
cargo clippy -- -D warnings
cargo build --release
./target/release/hive  # Files panel should show real files
```

## Completion

Output: FILES_REAL_COMPLETE
