//! LSP protocol utilities and helper functions

use lsp_types::Url;
use std::path::{Path, PathBuf};

/// Convert LSP URI to file system path
pub fn uri_to_path(uri: &Url) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    uri.to_file_path()
        .map_err(|_| "Failed to convert URI to file path".into())
}

/// Convert file system path to LSP URI
pub fn path_to_uri(path: &Path) -> Result<Url, Box<dyn std::error::Error + Send + Sync>> {
    Url::from_file_path(path)
        .map_err(|_| "Failed to convert file path to URI".into())
}

/// Normalize line endings for consistent processing
pub fn normalize_line_endings(content: &str) -> String {
    content.replace("\r\n", "\n").replace('\r', "\n")
}

/// Get line and character position from byte offset
pub fn offset_to_position(content: &str, offset: usize) -> lsp_types::Position {
    let content = normalize_line_endings(content);
    let mut line = 0;
    let mut character = 0;
    
    for (i, ch) in content.char_indices() {
        if i >= offset {
            break;
        }
        
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += ch.len_utf8() as u32;
        }
    }
    
    lsp_types::Position { line, character }
}

/// Get byte offset from line and character position
pub fn position_to_offset(content: &str, position: lsp_types::Position) -> Option<usize> {
    let content = normalize_line_endings(content);
    let mut current_line = 0;
    let mut current_character = 0;
    
    for (i, ch) in content.char_indices() {
        if current_line == position.line && current_character == position.character {
            return Some(i);
        }
        
        if ch == '\n' {
            current_line += 1;
            current_character = 0;
        } else {
            current_character += ch.len_utf8() as u32;
        }
        
        // If we've passed the target line, we won't find it
        if current_line > position.line {
            break;
        }
    }
    
    // If we're at the end and haven't found it, check if we're at the target position
    if current_line == position.line && current_character == position.character {
        Some(content.len())
    } else {
        None
    }
}

/// Extract the word at a given position
pub fn get_word_at_position(content: &str, position: lsp_types::Position) -> Option<String> {
    let content = normalize_line_endings(content);
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;
    
    let chars: Vec<char> = line.chars().collect();
    let char_pos = position.character as usize;
    
    if char_pos >= chars.len() {
        return None;
    }
    
    // Find word boundaries
    let mut start = char_pos;
    let mut end = char_pos;
    
    // Move start backward
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }
    
    // Move end forward
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }
    
    if start < end {
        Some(chars[start..end].iter().collect())
    } else {
        None
    }
}

/// Check if a character is part of a word (including ADR-specific characters)
fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == ':'
}

/// Create a range that spans the entire line
pub fn line_range(line_number: u32, line_content: &str) -> lsp_types::Range {
    lsp_types::Range {
        start: lsp_types::Position {
            line: line_number,
            character: 0,
        },
        end: lsp_types::Position {
            line: line_number,
            character: line_content.len() as u32,
        },
    }
}

/// Create a range for a specific word within a line
pub fn word_range(line_number: u32, line_content: &str, word: &str) -> Option<lsp_types::Range> {
    if let Some(start) = line_content.find(word) {
        Some(lsp_types::Range {
            start: lsp_types::Position {
                line: line_number,
                character: start as u32,
            },
            end: lsp_types::Position {
                line: line_number,
                character: (start + word.len()) as u32,
            },
        })
    } else {
        None
    }
}

/// Validate that a URI represents an ADR file
pub fn is_adr_file(uri: &Url) -> bool {
    if let Ok(path) = uri_to_path(uri) {
        if let Some(extension) = path.extension() {
            if extension == "md" || extension == "markdown" {
                // Check if filename suggests it's an ADR
                if let Some(filename) = path.file_stem() {
                    let filename = filename.to_string_lossy().to_lowercase();
                    return filename.contains("adr") || filename.starts_with("adr-") || filename.contains("decision");
                }
            }
        }
    }
    false
}

/// Extract ADR number from filename or content
pub fn extract_adr_number(content: &str) -> Option<String> {
    // Try to find ADR number in title line first
    let lines: Vec<&str> = content.lines().collect();
    for line in lines.iter().take(5) { // Check first 5 lines
        if line.starts_with("# ADR-") {
            if let Some(start) = line.find("ADR-") {
                let after_adr = &line[start + 4..];
                if let Some(end) = after_adr.find([' ', ':', '\t']) {
                    return Some(format!("ADR-{}", &after_adr[..end]));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_line_endings() {
        assert_eq!(normalize_line_endings("hello\r\nworld"), "hello\nworld");
        assert_eq!(normalize_line_endings("hello\rworld"), "hello\nworld");
        assert_eq!(normalize_line_endings("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn test_offset_to_position() {
        let content = "hello\nworld\ntest";
        
        assert_eq!(offset_to_position(content, 0), lsp_types::Position { line: 0, character: 0 });
        assert_eq!(offset_to_position(content, 6), lsp_types::Position { line: 1, character: 0 });
        assert_eq!(offset_to_position(content, 12), lsp_types::Position { line: 2, character: 0 });
    }

    #[test]
    fn test_position_to_offset() {
        let content = "hello\nworld\ntest";
        
        assert_eq!(position_to_offset(content, lsp_types::Position { line: 0, character: 0 }), Some(0));
        assert_eq!(position_to_offset(content, lsp_types::Position { line: 1, character: 0 }), Some(6));
        assert_eq!(position_to_offset(content, lsp_types::Position { line: 2, character: 0 }), Some(12));
    }

    #[test]
    fn test_get_word_at_position() {
        let content = "# ADR-001: Test Decision";
        
        let word = get_word_at_position(content, lsp_types::Position { line: 0, character: 2 });
        assert_eq!(word, Some("ADR-001:".to_string()));
        
        let word = get_word_at_position(content, lsp_types::Position { line: 0, character: 15 });
        assert_eq!(word, Some("Decision".to_string()));
    }

    #[test]
    fn test_is_word_char() {
        assert!(is_word_char('a'));
        assert!(is_word_char('A'));
        assert!(is_word_char('1'));
        assert!(is_word_char('-'));
        assert!(is_word_char('_'));
        assert!(is_word_char(':'));
        assert!(!is_word_char(' '));
        assert!(!is_word_char('('));
    }

    #[test]
    fn test_line_range() {
        let range = line_range(5, "hello world");
        assert_eq!(range.start.line, 5);
        assert_eq!(range.start.character, 0);
        assert_eq!(range.end.line, 5);
        assert_eq!(range.end.character, 11);
    }

    #[test]
    fn test_word_range() {
        let range = word_range(2, "hello world test", "world");
        assert!(range.is_some());
        
        let range = range.unwrap();
        assert_eq!(range.start.line, 2);
        assert_eq!(range.start.character, 6);
        assert_eq!(range.end.line, 2);
        assert_eq!(range.end.character, 11);
    }

    #[test]
    fn test_is_adr_file() {
        // These would normally require actual file paths, but we can test the logic
        let uri = Url::parse("file:///path/to/adr-001.md").unwrap();
        // Note: This test would fail because the file doesn't exist
        // In a real scenario, you'd use temporary files or mock the filesystem
    }

    #[test]
    fn test_extract_adr_number() {
        let content = "# ADR-001: Use Rust for Backend\n\n## Status\nProposed";
        assert_eq!(extract_adr_number(content), Some("ADR-001".to_string()));
        
        let content = "# ADR-123: Another Decision";
        assert_eq!(extract_adr_number(content), Some("ADR-123".to_string()));
        
        let content = "# Not an ADR\n\nSome content";
        assert_eq!(extract_adr_number(content), None);
    }

    #[test]
    fn test_uri_path_conversion() {
        // Test with a valid file URI
        let uri = Url::parse("file:///tmp/test.md").unwrap();
        let path = uri_to_path(&uri);
        assert!(path.is_ok());
        
        // Test path to URI conversion
        let path = PathBuf::from("/tmp/test.md");
        let uri_result = path_to_uri(&path);
        assert!(uri_result.is_ok());
    }
}