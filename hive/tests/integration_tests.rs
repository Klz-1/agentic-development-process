//! Integration tests for hive
//!
//! These tests verify that the application components work together correctly.

use std::process::Command;

/// Test that the binary builds and runs with --help
#[test]
fn test_binary_help() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run hive --help");

    assert!(output.status.success(), "hive --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hive"), "Should contain 'hive'");
    assert!(stdout.contains("OPTIONS"), "Should contain OPTIONS section");
    assert!(stdout.contains("KEYBINDINGS"), "Should contain KEYBINDINGS section");
}

/// Test that the binary builds and runs with --version
#[test]
fn test_binary_version() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run hive --version");

    assert!(output.status.success(), "hive --version should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hive"), "Should contain 'hive'");
    assert!(stdout.contains("0.1.0"), "Should contain version number");
}

/// Test that configuration loading works
#[test]
fn test_config_loading() {
    use hive::config;

    let config = config::load_config().expect("Should load default config");
    assert!(config.general.refresh_rate_ms > 0);
    assert!(config.sessions.output_buffer_lines > 0);
}

/// Test ring buffer functionality
#[test]
fn test_ring_buffer_integration() {
    use hive::utils::RingBuffer;

    let mut buffer: RingBuffer<String> = RingBuffer::new(100);

    // Add some lines
    for i in 0..150 {
        buffer.push(format!("Line {}", i));
    }

    // Should only have last 100
    assert_eq!(buffer.len(), 100);

    // First line should be Line 50
    assert_eq!(buffer.get(0), Some(&"Line 50".to_string()));

    // Last line should be Line 149
    assert_eq!(buffer.get(99), Some(&"Line 149".to_string()));
}

/// Test file tree building
#[test]
fn test_file_tree_building() {
    use hive::files::FileTree;
    use std::fs::{self, File};
    use tempfile::TempDir;

    let temp = TempDir::new().unwrap();

    // Create some test files
    File::create(temp.path().join("file1.txt")).unwrap();
    File::create(temp.path().join("file2.rs")).unwrap();
    fs::create_dir(temp.path().join("subdir")).unwrap();
    File::create(temp.path().join("subdir").join("nested.txt")).unwrap();

    let tree = FileTree::new(temp.path().to_path_buf());

    // Should have entries
    assert!(!tree.entries.is_empty());

    // Check that files are present (names may be sorted)
    let names: Vec<String> = tree.entries.iter()
        .filter_map(|e| e.path.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "file1.txt"), "Should contain file1.txt");
    assert!(names.iter().any(|n| n == "file2.rs"), "Should contain file2.rs");
    assert!(names.iter().any(|n| n == "subdir"), "Should contain subdir");
}

/// Test project root detection
#[test]
fn test_project_root_detection() {
    use hive::files::find_project_root;
    use std::fs::{self, File};
    use tempfile::TempDir;

    let temp = TempDir::new().unwrap();

    // Create a git directory marker
    fs::create_dir(temp.path().join(".git")).unwrap();

    // Create nested directory
    let nested = temp.path().join("src").join("lib");
    fs::create_dir_all(&nested).unwrap();
    File::create(nested.join("mod.rs")).unwrap();

    // Find root from nested dir
    let root = find_project_root(&nested);
    assert_eq!(
        root.canonicalize().unwrap(),
        temp.path().canonicalize().unwrap()
    );
}
