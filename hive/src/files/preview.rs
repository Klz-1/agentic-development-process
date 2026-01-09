//! File preview functionality

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// File preview data
pub struct FilePreview {
    /// Lines of the file
    pub lines: Vec<String>,
    /// Whether the file was truncated
    pub truncated: bool,
    /// Total number of lines in the file
    pub total_lines: usize,
}

impl FilePreview {
    /// Read a file preview with the given number of lines
    pub fn read(path: &Path, max_lines: usize) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut lines = Vec::new();
        let mut total_lines = 0;

        for line_result in reader.lines() {
            total_lines += 1;
            if lines.len() < max_lines {
                let line = line_result?;
                // Truncate very long lines
                if line.len() > 200 {
                    lines.push(format!("{}...", &line[..197]));
                } else {
                    lines.push(line);
                }
            }
        }

        Ok(Self {
            truncated: total_lines > max_lines,
            lines,
            total_lines,
        })
    }

    /// Check if a file is binary (contains null bytes in first KB)
    pub fn is_binary(path: &Path) -> bool {
        use std::io::Read;

        let Ok(mut file) = File::open(path) else {
            return false;
        };

        let mut buffer = [0u8; 1024];
        let Ok(bytes_read) = file.read(&mut buffer) else {
            return false;
        };

        buffer[..bytes_read].contains(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_read_text_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Line 1").unwrap();
            writeln!(file, "Line 2").unwrap();
            writeln!(file, "Line 3").unwrap();
        }

        let preview = FilePreview::read(&file_path, 10).unwrap();
        assert_eq!(preview.lines.len(), 3);
        assert_eq!(preview.total_lines, 3);
        assert!(!preview.truncated);
    }

    #[test]
    fn test_read_truncated_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        {
            let mut file = File::create(&file_path).unwrap();
            for i in 1..=20 {
                writeln!(file, "Line {}", i).unwrap();
            }
        }

        let preview = FilePreview::read(&file_path, 5).unwrap();
        assert_eq!(preview.lines.len(), 5);
        assert_eq!(preview.total_lines, 20);
        assert!(preview.truncated);
    }

    #[test]
    fn test_long_lines_truncated() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        {
            let mut file = File::create(&file_path).unwrap();
            let long_line = "a".repeat(300);
            writeln!(file, "{}", long_line).unwrap();
        }

        let preview = FilePreview::read(&file_path, 10).unwrap();
        assert_eq!(preview.lines.len(), 1);
        assert!(preview.lines[0].len() <= 200);
        assert!(preview.lines[0].ends_with("..."));
    }

    #[test]
    fn test_is_binary_text_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");

        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }

        assert!(!FilePreview::is_binary(&file_path));
    }

    #[test]
    fn test_is_binary_binary_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.bin");

        {
            let mut file = File::create(&file_path).unwrap();
            // Write some bytes including null byte
            file.write_all(&[0x48, 0x65, 0x00, 0x6c, 0x6f]).unwrap();
        }

        assert!(FilePreview::is_binary(&file_path));
    }

    #[test]
    fn test_is_binary_nonexistent_file() {
        let path = Path::new("/nonexistent/file.txt");
        // Should return false for non-existent files (not binary)
        assert!(!FilePreview::is_binary(path));
    }

    #[test]
    fn test_read_empty_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("empty.txt");

        File::create(&file_path).unwrap();

        let preview = FilePreview::read(&file_path, 10).unwrap();
        assert!(preview.lines.is_empty());
        assert_eq!(preview.total_lines, 0);
        assert!(!preview.truncated);
    }
}
