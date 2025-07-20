use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::AdrscanError;
type Result<T> = std::result::Result<T, AdrscanError>;

/// ADR metadata parsed from frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrMetadata {
    /// ADR number/identifier
    pub id: Option<String>,
    
    /// ADR title
    pub title: String,
    
    /// Status (proposed, accepted, rejected, deprecated, superseded)
    pub status: String,
    
    /// Date of decision
    pub date: Option<chrono::NaiveDate>,
    
    /// Decision makers/authors
    pub deciders: Vec<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// ADRs this one supersedes
    pub supersedes: Vec<String>,
    
    /// Related ADRs
    pub relates_to: Vec<String>,
    
    /// Additional custom fields
    pub custom_fields: HashMap<String, serde_yaml::Value>,
}

/// Parsed ADR document
#[derive(Debug, Clone)]
pub struct AdrDocument {
    /// File path
    pub path: std::path::PathBuf,
    
    /// Parsed metadata
    pub metadata: AdrMetadata,
    
    /// Raw markdown content (without frontmatter)
    pub content: String,
    
    /// Raw frontmatter YAML
    pub frontmatter: String,
}

/// ADR parser for extracting metadata and content
pub struct AdrParser;

impl AdrParser {
    /// Parse an ADR file
    pub fn parse_file(path: &Path) -> Result<AdrDocument> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AdrscanError::Io(e))?;

        Self::parse_content(&content, path.to_path_buf())
    }

    /// Parse ADR content string
    pub fn parse_content(content: &str, path: std::path::PathBuf) -> Result<AdrDocument> {
        let (frontmatter, markdown_content) = Self::extract_frontmatter(content)?;
        let metadata = Self::parse_metadata(&frontmatter)?;

        Ok(AdrDocument {
            path,
            metadata,
            content: markdown_content,
            frontmatter: frontmatter.to_string(),
        })
    }

    /// Extract YAML frontmatter from markdown content
    fn extract_frontmatter(content: &str) -> Result<(String, String)> {
        if !content.starts_with("---\n") {
            return Err(AdrscanError::ParseError(
                "No YAML frontmatter found (must start with ---)".to_string(),
            ));
        }

        let content_without_first_delimiter = &content[4..]; // Skip first "---\n"
        
        if let Some(end_pos) = content_without_first_delimiter.find("\n---\n") {
            let frontmatter = content_without_first_delimiter[..end_pos].to_string();
            let markdown_content = content_without_first_delimiter[end_pos + 5..].to_string(); // Skip "\n---\n"
            Ok((frontmatter, markdown_content))
        } else {
            Err(AdrscanError::ParseError(
                "Frontmatter not properly closed (missing closing ---)".to_string(),
            ))
        }
    }

    /// Parse YAML frontmatter into ADR metadata
    fn parse_metadata(frontmatter: &str) -> Result<AdrMetadata> {
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(frontmatter)
            .map_err(|e| AdrscanError::ParseError(format!("Invalid YAML frontmatter: {}", e)))?;

        let mapping = yaml_value.as_mapping()
            .ok_or_else(|| AdrscanError::ParseError("Frontmatter must be a YAML mapping".to_string()))?;

        // Extract required and optional fields
        let title = Self::extract_string_field(mapping, "title")
            .unwrap_or_else(|| "Untitled ADR".to_string());

        let status = Self::extract_string_field(mapping, "status")
            .unwrap_or_else(|| "proposed".to_string());

        let id = Self::extract_string_field(mapping, "id")
            .or_else(|| Self::extract_string_field(mapping, "number"));

        let date = Self::extract_string_field(mapping, "date")
            .and_then(|date_str| chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok());

        let deciders = Self::extract_string_array(mapping, "deciders")
            .or_else(|| Self::extract_string_array(mapping, "authors"))
            .unwrap_or_default();

        let tags = Self::extract_string_array(mapping, "tags").unwrap_or_default();
        let supersedes = Self::extract_string_array(mapping, "supersedes").unwrap_or_default();
        let relates_to = Self::extract_string_array(mapping, "relates_to")
            .or_else(|| Self::extract_string_array(mapping, "related"))
            .unwrap_or_default();

        // Collect custom fields (everything not in standard fields)
        let standard_fields = ["title", "status", "id", "number", "date", "deciders", "authors", "tags", "supersedes", "relates_to", "related"];
        let custom_fields = mapping
            .iter()
            .filter_map(|(k, v)| {
                if let Some(key_str) = k.as_str() {
                    if !standard_fields.contains(&key_str) {
                        Some((key_str.to_string(), v.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(AdrMetadata {
            id,
            title,
            status,
            date,
            deciders,
            tags,
            supersedes,
            relates_to,
            custom_fields,
        })
    }

    fn extract_string_field(mapping: &serde_yaml::Mapping, field: &str) -> Option<String> {
        mapping.get(&serde_yaml::Value::String(field.to_string()))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn extract_string_array(mapping: &serde_yaml::Mapping, field: &str) -> Option<Vec<String>> {
        mapping.get(&serde_yaml::Value::String(field.to_string()))
            .and_then(|v| {
                if let Some(arr) = v.as_sequence() {
                    Some(arr.iter().filter_map(|item| item.as_str().map(|s| s.to_string())).collect())
                } else if let Some(s) = v.as_str() {
                    // Handle single string as array of one
                    Some(vec![s.to_string()])
                } else {
                    None
                }
            })
    }
}