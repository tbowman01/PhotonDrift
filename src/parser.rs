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
    #[allow(dead_code)] // Planned for enhanced file tracking
    pub path: std::path::PathBuf,

    /// Parsed metadata
    pub metadata: AdrMetadata,

    /// Raw markdown content (without frontmatter)
    pub content: String,

    /// Raw frontmatter YAML
    #[allow(dead_code)] // Planned for frontmatter analysis
    pub frontmatter: String,
}

/// ADR parser for extracting metadata and content
pub struct AdrParser;

impl AdrParser {
    /// Parse an ADR file
    pub fn parse_file(path: &Path) -> Result<AdrDocument> {
        let content = std::fs::read_to_string(path).map_err(AdrscanError::Io)?;

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

    /// Extract frontmatter from markdown content (supports YAML --- and TOML +++)
    fn extract_frontmatter(content: &str) -> Result<(String, String)> {
        // Check for YAML frontmatter (---)
        if let Some(content_without_first_delimiter) = content.strip_prefix("---\n") {
            // Skip first "---\n"

            if let Some(end_pos) = content_without_first_delimiter.find("\n---\n") {
                let frontmatter = content_without_first_delimiter[..end_pos].to_string();
                let markdown_content = content_without_first_delimiter[end_pos + 5..].to_string(); // Skip "\n---\n"
                Ok((frontmatter, markdown_content))
            } else {
                Err(AdrscanError::ParseError(
                    "YAML frontmatter not properly closed (missing closing ---)".to_string(),
                ))
            }
        }
        // Check for TOML frontmatter (+++)
        else if let Some(content_without_first_delimiter) = content.strip_prefix("+++\n") {
            // Skip first "+++\n"

            if let Some(end_pos) = content_without_first_delimiter.find("\n+++\n") {
                let frontmatter = content_without_first_delimiter[..end_pos].to_string();
                let markdown_content = content_without_first_delimiter[end_pos + 5..].to_string(); // Skip "\n+++\n"
                Ok((frontmatter, markdown_content))
            } else {
                Err(AdrscanError::ParseError(
                    "TOML frontmatter not properly closed (missing closing +++)".to_string(),
                ))
            }
        }
        // No frontmatter found - this is valid, return empty frontmatter
        else {
            Ok((String::new(), content.to_string()))
        }
    }

    /// Parse frontmatter into ADR metadata (supports YAML and TOML)
    fn parse_metadata(frontmatter: &str) -> Result<AdrMetadata> {
        // Handle empty frontmatter gracefully
        if frontmatter.trim().is_empty() {
            return Ok(AdrMetadata {
                id: None,
                title: "Untitled ADR".to_string(),
                status: "proposed".to_string(),
                date: None,
                deciders: Vec::new(),
                tags: Vec::new(),
                supersedes: Vec::new(),
                relates_to: Vec::new(),
                custom_fields: HashMap::new(),
            });
        }

        // Try parsing as YAML first
        if let Ok(yaml_value) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
            if let Some(mapping) = yaml_value.as_mapping() {
                return Self::extract_metadata_from_yaml(mapping);
            }
        }

        // Try parsing as TOML
        if let Ok(toml_value) = toml::from_str::<toml::Value>(frontmatter) {
            if let Some(table) = toml_value.as_table() {
                return Self::extract_metadata_from_toml(table);
            }
        }

        // If both fail, provide detailed error message
        Err(AdrscanError::ParseError(
            "Frontmatter must be valid YAML or TOML format".to_string(),
        ))
    }

    /// Extract metadata from YAML mapping
    fn extract_metadata_from_yaml(mapping: &serde_yaml::Mapping) -> Result<AdrMetadata> {
        // Extract required and optional fields
        let title = Self::extract_yaml_string_field(mapping, "title")
            .unwrap_or_else(|| "Untitled ADR".to_string());

        let status = Self::extract_yaml_string_field(mapping, "status")
            .unwrap_or_else(|| "proposed".to_string());

        let id = Self::extract_yaml_string_field(mapping, "id")
            .or_else(|| Self::extract_yaml_string_field(mapping, "number"));

        let date = Self::extract_yaml_string_field(mapping, "date")
            .and_then(|date_str| chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok());

        let deciders = Self::extract_yaml_string_array(mapping, "deciders")
            .or_else(|| Self::extract_yaml_string_array(mapping, "authors"))
            .unwrap_or_default();

        let tags = Self::extract_yaml_string_array(mapping, "tags").unwrap_or_default();
        let supersedes = Self::extract_yaml_string_array(mapping, "supersedes").unwrap_or_default();
        let relates_to = Self::extract_yaml_string_array(mapping, "relates_to")
            .or_else(|| Self::extract_yaml_string_array(mapping, "related"))
            .unwrap_or_default();

        // Collect custom fields (everything not in standard fields)
        let standard_fields = [
            "title",
            "status",
            "id",
            "number",
            "date",
            "deciders",
            "authors",
            "tags",
            "supersedes",
            "relates_to",
            "related",
        ];
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

    /// Extract metadata from TOML table
    fn extract_metadata_from_toml(
        table: &toml::map::Map<String, toml::Value>,
    ) -> Result<AdrMetadata> {
        // Extract required and optional fields
        let title = Self::extract_toml_string_field(table, "title")
            .unwrap_or_else(|| "Untitled ADR".to_string());

        let status = Self::extract_toml_string_field(table, "status")
            .unwrap_or_else(|| "proposed".to_string());

        let id = Self::extract_toml_string_field(table, "id")
            .or_else(|| Self::extract_toml_string_field(table, "number"));

        let date = Self::extract_toml_string_field(table, "date")
            .and_then(|date_str| chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok());

        let deciders = Self::extract_toml_string_array(table, "deciders")
            .or_else(|| Self::extract_toml_string_array(table, "authors"))
            .unwrap_or_default();

        let tags = Self::extract_toml_string_array(table, "tags").unwrap_or_default();
        let supersedes = Self::extract_toml_string_array(table, "supersedes").unwrap_or_default();
        let relates_to = Self::extract_toml_string_array(table, "relates_to")
            .or_else(|| Self::extract_toml_string_array(table, "related"))
            .unwrap_or_default();

        // Collect custom fields (everything not in standard fields)
        let standard_fields = [
            "title",
            "status",
            "id",
            "number",
            "date",
            "deciders",
            "authors",
            "tags",
            "supersedes",
            "relates_to",
            "related",
        ];
        let custom_fields = table
            .iter()
            .filter_map(|(k, v)| {
                if !standard_fields.contains(&k.as_str()) {
                    // Convert TOML value to YAML value for consistency
                    Some((k.clone(), Self::toml_to_yaml_value(v)))
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

    // YAML helper functions
    fn extract_yaml_string_field(mapping: &serde_yaml::Mapping, field: &str) -> Option<String> {
        mapping
            .get(serde_yaml::Value::String(field.to_string()))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn extract_yaml_string_array(
        mapping: &serde_yaml::Mapping,
        field: &str,
    ) -> Option<Vec<String>> {
        mapping
            .get(serde_yaml::Value::String(field.to_string()))
            .and_then(|v| {
                if let Some(arr) = v.as_sequence() {
                    Some(
                        arr.iter()
                            .filter_map(|item| item.as_str().map(|s| s.to_string()))
                            .collect(),
                    )
                } else if let Some(s) = v.as_str() {
                    // Handle single string as array of one
                    Some(vec![s.to_string()])
                } else {
                    None
                }
            })
    }

    // TOML helper functions
    fn extract_toml_string_field(
        table: &toml::map::Map<String, toml::Value>,
        field: &str,
    ) -> Option<String> {
        table
            .get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn extract_toml_string_array(
        table: &toml::map::Map<String, toml::Value>,
        field: &str,
    ) -> Option<Vec<String>> {
        table.get(field).and_then(|v| {
            if let Some(arr) = v.as_array() {
                Some(
                    arr.iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect(),
                )
            } else if let Some(s) = v.as_str() {
                // Handle single string as array of one
                Some(vec![s.to_string()])
            } else {
                None
            }
        })
    }

    // Convert TOML value to YAML value for consistency in custom fields
    fn toml_to_yaml_value(toml_val: &toml::Value) -> serde_yaml::Value {
        match toml_val {
            toml::Value::String(s) => serde_yaml::Value::String(s.clone()),
            toml::Value::Integer(i) => serde_yaml::Value::Number(serde_yaml::Number::from(*i)),
            toml::Value::Float(f) => serde_yaml::Value::Number(serde_yaml::Number::from(*f)),
            toml::Value::Boolean(b) => serde_yaml::Value::Bool(*b),
            toml::Value::Array(arr) => {
                let yaml_seq: Vec<serde_yaml::Value> =
                    arr.iter().map(Self::toml_to_yaml_value).collect();
                serde_yaml::Value::Sequence(yaml_seq)
            }
            toml::Value::Table(table) => {
                let yaml_map: serde_yaml::Mapping = table
                    .iter()
                    .map(|(k, v)| {
                        (
                            serde_yaml::Value::String(k.clone()),
                            Self::toml_to_yaml_value(v),
                        )
                    })
                    .collect();
                serde_yaml::Value::Mapping(yaml_map)
            }
            toml::Value::Datetime(dt) => serde_yaml::Value::String(dt.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_yaml_frontmatter() {
        let content = r#"---
title: "Test ADR"
status: accepted
date: 2023-12-01
deciders: ["Alice", "Bob"]
tags: ["architecture", "database"]
---

# Test ADR

This is test content.
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.title, "Test ADR");
        assert_eq!(result.metadata.status, "accepted");
        assert_eq!(result.metadata.deciders, vec!["Alice", "Bob"]);
        assert_eq!(result.metadata.tags, vec!["architecture", "database"]);
        assert!(result.content.contains("# Test ADR"));
    }

    #[test]
    fn test_parse_toml_frontmatter() {
        let content = r#"+++
title = "Test ADR"
status = "accepted"
date = "2023-12-01"
deciders = ["Alice", "Bob"]
tags = ["architecture", "database"]
+++

# Test ADR

This is test content.
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.title, "Test ADR");
        assert_eq!(result.metadata.status, "accepted");
        assert_eq!(result.metadata.deciders, vec!["Alice", "Bob"]);
        assert_eq!(result.metadata.tags, vec!["architecture", "database"]);
        assert!(result.content.contains("# Test ADR"));
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = r#"# Test ADR

This is test content without frontmatter.
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.title, "Untitled ADR");
        assert_eq!(result.metadata.status, "proposed");
        assert!(result.metadata.deciders.is_empty());
        assert!(result.content.contains("# Test ADR"));
    }

    #[test]
    fn test_parse_malformed_yaml() {
        let content = r#"---
title: "Test ADR
status: accepted  # Missing quote
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_incomplete_frontmatter() {
        let content = r#"---
title: "Test ADR"
status: accepted

# No closing frontmatter delimiter
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md"));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not properly closed"));
    }

    #[test]
    fn test_parse_custom_fields() {
        let content = r#"---
title: "Test ADR"
status: accepted
custom_field: "custom_value"
complexity: high
priority: 1
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.custom_fields.len(), 3);
        assert!(result.metadata.custom_fields.contains_key("custom_field"));
        assert!(result.metadata.custom_fields.contains_key("complexity"));
        assert!(result.metadata.custom_fields.contains_key("priority"));
    }

    #[test]
    fn test_parse_alternative_field_names() {
        let content = r#"---
title: "Test ADR"
status: accepted
number: "001"
authors: ["Alice"]
related: ["ADR-002"]
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.id, Some("001".to_string()));
        assert_eq!(result.metadata.deciders, vec!["Alice"]);
        assert_eq!(result.metadata.relates_to, vec!["ADR-002"]);
    }

    #[test]
    fn test_single_string_as_array() {
        let content = r#"---
title: "Test ADR"
status: accepted
deciders: "Alice"
tags: "architecture"
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert_eq!(result.metadata.deciders, vec!["Alice"]);
        assert_eq!(result.metadata.tags, vec!["architecture"]);
    }

    #[test]
    fn test_date_parsing() {
        let content = r#"---
title: "Test ADR"
status: accepted
date: "2023-12-01"
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        assert!(result.metadata.date.is_some());
        assert_eq!(
            result.metadata.date.unwrap().format("%Y-%m-%d").to_string(),
            "2023-12-01"
        );
    }

    #[test]
    fn test_invalid_date_format() {
        let content = r#"---
title: "Test ADR"
status: accepted
date: "invalid-date"
---

# Test ADR
"#;

        let result = AdrParser::parse_content(content, PathBuf::from("test.md")).unwrap();
        // Invalid dates should be ignored, not cause errors
        assert!(result.metadata.date.is_none());
    }
}
