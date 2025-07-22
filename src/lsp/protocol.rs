//! LSP protocol utilities and helpers for PhotonDrift
//! 
//! This module provides utility functions and helpers for working with
//! the Language Server Protocol, including message handling and data conversion.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use serde_json::Value;
use url::Url;

/// Protocol helper utilities for LSP operations
pub struct LspProtocolHelper {
    /// Cache for URI to file path conversions
    uri_cache: std::sync::RwLock<HashMap<Url, std::path::PathBuf>>,
}

impl LspProtocolHelper {
    /// Create a new protocol helper instance
    pub fn new() -> Self {
        Self {
            uri_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Convert URI to file path with caching
    pub fn uri_to_path(&self, uri: &Url) -> Option<std::path::PathBuf> {
        // Check cache first
        if let Ok(cache) = self.uri_cache.read() {
            if let Some(path) = cache.get(uri) {
                return Some(path.clone());
            }
        }

        // Convert and cache
        if let Ok(path) = uri.to_file_path() {
            if let Ok(mut cache) = self.uri_cache.write() {
                cache.insert(uri.clone(), path.clone());
            }
            Some(path)
        } else {
            None
        }
    }

    /// Convert file path to URI
    pub fn path_to_uri(&self, path: &std::path::Path) -> Option<Url> {
        Url::from_file_path(path).ok()
    }

    /// Create a diagnostic with standard formatting
    pub fn create_diagnostic(
        &self,
        range: Range,
        severity: DiagnosticSeverity,
        code: &str,
        message: &str,
    ) -> Diagnostic {
        Diagnostic {
            range,
            severity: Some(severity),
            code: Some(NumberOrString::String(code.to_string())),
            source: Some("photon-drift".to_string()),
            message: message.to_string(),
            related_information: None,
            tags: None,
            data: None,
            code_description: None,
        }
    }

    /// Create a diagnostic with related information
    pub fn create_diagnostic_with_related(
        &self,
        range: Range,
        severity: DiagnosticSeverity,
        code: &str,
        message: &str,
        related_info: Vec<DiagnosticRelatedInformation>,
    ) -> Diagnostic {
        Diagnostic {
            range,
            severity: Some(severity),
            code: Some(NumberOrString::String(code.to_string())),
            source: Some("photon-drift".to_string()),
            message: message.to_string(),
            related_information: Some(related_info),
            tags: None,
            data: None,
            code_description: None,
        }
    }

    /// Create a completion item with snippet support
    pub fn create_completion_item(
        &self,
        label: &str,
        kind: CompletionItemKind,
        detail: Option<&str>,
        documentation: Option<&str>,
        insert_text: Option<&str>,
        is_snippet: bool,
    ) -> CompletionItem {
        CompletionItem {
            label: label.to_string(),
            kind: Some(kind),
            detail: detail.map(|s| s.to_string()),
            documentation: documentation.map(|s| Documentation::String(s.to_string())),
            insert_text: insert_text.map(|s| s.to_string()),
            insert_text_format: if is_snippet {
                Some(InsertTextFormat::SNIPPET)
            } else {
                Some(InsertTextFormat::PLAIN_TEXT)
            },
            ..Default::default()
        }
    }

    /// Create a hover response with markdown content
    pub fn create_hover(&self, markdown_content: &str, range: Option<Range>) -> Hover {
        Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown_content.to_string(),
            }),
            range,
        }
    }

    /// Extract position information from text
    pub fn position_to_offset(&self, text: &str, position: Position) -> Option<usize> {
        let mut offset = 0;
        let mut current_line = 0;
        let mut current_char = 0;

        for ch in text.chars() {
            if current_line == position.line as usize && current_char == position.character as usize {
                return Some(offset);
            }

            if ch == '\n' {
                current_line += 1;
                current_char = 0;
            } else {
                current_char += 1;
            }

            offset += ch.len_utf8();
        }

        // Handle end of file position
        if current_line == position.line as usize && current_char == position.character as usize {
            Some(offset)
        } else {
            None
        }
    }

    /// Convert offset back to position
    pub fn offset_to_position(&self, text: &str, offset: usize) -> Position {
        let mut current_offset = 0;
        let mut line = 0;
        let mut character = 0;

        for ch in text.chars() {
            if current_offset >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                character = 0;
            } else {
                character += 1;
            }

            current_offset += ch.len_utf8();
        }

        Position {
            line: line as u32,
            character: character as u32,
        }
    }

    /// Create a range from line and character bounds
    pub fn create_range(&self, start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Range {
        Range {
            start: Position {
                line: start_line,
                character: start_char,
            },
            end: Position {
                line: end_line,
                character: end_char,
            },
        }
    }

    /// Get line range for a given line number
    pub fn get_line_range(&self, text: &str, line_number: u32) -> Option<Range> {
        let lines: Vec<&str> = text.lines().collect();
        
        if let Some(line) = lines.get(line_number as usize) {
            Some(Range {
                start: Position {
                    line: line_number,
                    character: 0,
                },
                end: Position {
                    line: line_number,
                    character: line.chars().count() as u32,
                },
            })
        } else {
            None
        }
    }

    /// Extract word at position
    pub fn get_word_at_position(&self, text: &str, position: Position) -> Option<(String, Range)> {
        let lines: Vec<&str> = text.lines().collect();
        let line = lines.get(position.line as usize)?;
        let chars: Vec<char> = line.chars().collect();
        
        if position.character as usize >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = position.character as usize;
        let mut end = position.character as usize;

        // Move start backwards to beginning of word
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_' || chars[start - 1] == '-') {
            start -= 1;
        }

        // Move end forwards to end of word
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_' || chars[end] == '-') {
            end += 1;
        }

        if start < end {
            let word: String = chars[start..end].iter().collect();
            let range = Range {
                start: Position {
                    line: position.line,
                    character: start as u32,
                },
                end: Position {
                    line: position.line,
                    character: end as u32,
                },
            };
            Some((word, range))
        } else {
            None
        }
    }

    /// Validate LSP message structure
    pub fn validate_message(&self, message: &Value) -> Result<(), String> {
        // Basic LSP message validation
        if !message.is_object() {
            return Err("Message must be a JSON object".to_string());
        }

        let obj = message.as_object().unwrap();

        // Check for required fields
        if !obj.contains_key("jsonrpc") {
            return Err("Missing 'jsonrpc' field".to_string());
        }

        if obj.get("jsonrpc").unwrap().as_str() != Some("2.0") {
            return Err("Invalid jsonrpc version".to_string());
        }

        // Check message type
        let has_id = obj.contains_key("id");
        let has_method = obj.contains_key("method");
        let has_result = obj.contains_key("result");
        let has_error = obj.contains_key("error");

        match (has_id, has_method, has_result, has_error) {
            // Request
            (true, true, false, false) => Ok(()),
            // Notification
            (false, true, false, false) => Ok(()),
            // Response with result
            (true, false, true, false) => Ok(()),
            // Response with error
            (true, false, false, true) => Ok(()),
            _ => Err("Invalid message structure".to_string()),
        }
    }

    /// Create error response
    pub fn create_error_response(&self, id: Option<Value>, code: i64, message: &str) -> Value {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        })
    }

    /// Performance measurement utilities
    pub fn measure_performance<F, R>(&self, operation_name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        
        if elapsed.as_millis() > 100 {
            eprintln!("LSP operation '{}' took {}ms (>100ms threshold)", operation_name, elapsed.as_millis());
        }
        
        result
    }
}

impl Default for LspProtocolHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// LSP message types for type-safe handling
#[derive(Debug, Clone)]
pub enum LspMessage {
    Request {
        id: Value,
        method: String,
        params: Option<Value>,
    },
    Notification {
        method: String,
        params: Option<Value>,
    },
    Response {
        id: Value,
        result: Option<Value>,
        error: Option<LspError>,
    },
}

/// LSP error structure
#[derive(Debug, Clone)]
pub struct LspError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

/// Common LSP error codes
pub mod error_codes {
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;
    
    // LSP specific error codes
    pub const SERVER_NOT_INITIALIZED: i64 = -32002;
    pub const UNKNOWN_ERROR_CODE: i64 = -32001;
    pub const REQUEST_CANCELLED: i64 = -32800;
    pub const CONTENT_MODIFIED: i64 = -32801;
}

/// File system utilities for LSP operations
pub struct FileSystemHelper;

impl FileSystemHelper {
    /// Check if a file is an ADR based on path and content
    pub fn is_adr_file(path: &std::path::Path) -> bool {
        // Check file extension
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            return false;
        }

        // Check for ADR directory patterns
        let path_str = path.to_string_lossy().to_lowercase();
        path_str.contains("adr") || 
        path_str.contains("decision") || 
        path_str.contains("architecture")
    }

    /// Find all ADR files in a directory
    pub fn find_adr_files(root: &std::path::Path) -> std::io::Result<Vec<std::path::PathBuf>> {
        let mut adr_files = Vec::new();
        
        for entry in walkdir::WalkDir::new(root).follow_links(true) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                if Self::is_adr_file(path) {
                    adr_files.push(path.to_path_buf());
                }
            }
        }
        
        Ok(adr_files)
    }
}