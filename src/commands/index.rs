use chrono::NaiveDate;
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{config::Config, error::AdrscanError, parser::AdrParser};
type Result<T> = std::result::Result<T, AdrscanError>;

#[derive(Args)]
pub struct IndexCommand {
    /// ADR directory to index
    #[arg(short, long)]
    pub adr_dir: Option<PathBuf>,

    /// Index file output path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Sort order (number, date, status, title)
    #[arg(short, long, default_value = "number")]
    pub sort: String,

    /// Include status badges in the index
    #[arg(long)]
    pub badges: bool,

    /// Custom template file for index generation
    #[arg(long)]
    pub template: Option<PathBuf>,

    /// Only include ADRs with specific status
    #[arg(long)]
    pub status_filter: Option<String>,
}

/// Represents an ADR entry in the index
#[derive(Debug, Clone)]
pub struct AdrIndexEntry {
    #[allow(dead_code)] // Used for future file operations
    pub file_path: PathBuf,
    pub relative_path: String,
    pub id: Option<String>,
    pub number: Option<u32>,
    pub title: String,
    pub status: String,
    pub date: Option<NaiveDate>,
    pub deciders: Vec<String>,
    #[allow(dead_code)] // Planned for enhanced templates
    pub tags: Vec<String>,
    #[allow(dead_code)] // Planned for enhanced templates
    pub supersedes: Vec<String>,
    #[allow(dead_code)] // Planned for enhanced templates
    pub relates_to: Vec<String>,
}

impl IndexCommand {
    pub fn execute(&self, config: &Config) -> Result<()> {
        log::info!("Generating ADR index...");

        let adr_dir = self.adr_dir.as_ref().unwrap_or(&config.adr_dir);

        // Validate ADR directory exists
        if !adr_dir.exists() {
            return Err(AdrscanError::DirectoryNotFound(format!(
                "ADR directory not found: {}",
                adr_dir.display()
            )));
        }

        // Scan for ADR files and build index entries
        let mut adr_entries = self.scan_and_parse_adrs(adr_dir)?;
        log::info!("Found {} ADR files", adr_entries.len());

        // Apply status filter if specified
        if let Some(ref status_filter) = self.status_filter {
            adr_entries.retain(|entry| entry.status.eq_ignore_ascii_case(status_filter));
            log::info!(
                "Filtered to {} ADRs with status '{}'",
                adr_entries.len(),
                status_filter
            );
        }

        // Sort entries according to specified criteria
        self.sort_entries(&mut adr_entries);

        // Determine output path
        let output_path = self.get_output_path(adr_dir);

        // Generate index content
        let index_content = if let Some(ref template_path) = self.template {
            self.generate_custom_index(&adr_entries, template_path, adr_dir)?
        } else {
            self.generate_default_index(&adr_entries, adr_dir)?
        };

        // Write index file
        std::fs::write(&output_path, index_content).map_err(AdrscanError::Io)?;

        println!("✅ ADR index generated: {}", output_path.display());
        println!("📋 Indexed {} ADRs", adr_entries.len());

        Ok(())
    }

    /// Scan ADR directory and parse all ADR files
    fn scan_and_parse_adrs(&self, adr_dir: &PathBuf) -> Result<Vec<AdrIndexEntry>> {
        let mut entries = Vec::new();

        for entry in walkdir::WalkDir::new(adr_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" || extension == "markdown" {
                        match self.parse_adr_entry(path, adr_dir) {
                            Ok(adr_entry) => entries.push(adr_entry),
                            Err(e) => {
                                log::warn!("Failed to parse ADR file {}: {}", path.display(), e);
                                // Continue processing other files
                            }
                        }
                    }
                }
            }
        }

        Ok(entries)
    }

    /// Parse a single ADR file into an index entry
    fn parse_adr_entry(
        &self,
        file_path: &std::path::Path,
        adr_dir: &PathBuf,
    ) -> Result<AdrIndexEntry> {
        let adr_doc = AdrParser::parse_file(file_path)?;

        // Calculate relative path from ADR directory
        let relative_path = file_path
            .strip_prefix(adr_dir)
            .map_err(|_| AdrscanError::ParseError("Cannot determine relative path".to_string()))?
            .to_string_lossy()
            .to_string();

        // Try to extract number from ID or filename
        let number = self.extract_number(&adr_doc.metadata.id, file_path);

        Ok(AdrIndexEntry {
            file_path: file_path.to_path_buf(),
            relative_path,
            id: adr_doc.metadata.id,
            number,
            title: adr_doc.metadata.title,
            status: adr_doc.metadata.status,
            date: adr_doc.metadata.date,
            deciders: adr_doc.metadata.deciders,
            tags: adr_doc.metadata.tags,
            supersedes: adr_doc.metadata.supersedes,
            relates_to: adr_doc.metadata.relates_to,
        })
    }

    /// Extract numeric ID from ADR ID or filename
    fn extract_number(&self, id: &Option<String>, file_path: &std::path::Path) -> Option<u32> {
        // Try to extract from ID first
        if let Some(ref id_str) = id {
            if let Ok(num) = id_str.parse::<u32>() {
                return Some(num);
            }
            // Try to extract number from ID string (e.g., "ADR-001" -> 1)
            if let Some(captures) = regex::Regex::new(r"(\d+)").unwrap().captures(id_str) {
                if let Some(num_str) = captures.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        return Some(num);
                    }
                }
            }
        }

        // Try to extract from filename
        if let Some(filename) = file_path.file_name() {
            let filename_str = filename.to_string_lossy();
            if let Some(captures) = regex::Regex::new(r"(\d+)").unwrap().captures(&filename_str) {
                if let Some(num_str) = captures.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        return Some(num);
                    }
                }
            }
        }

        None
    }

    /// Sort entries according to specified criteria
    fn sort_entries(&self, entries: &mut Vec<AdrIndexEntry>) {
        match self.sort.as_str() {
            "number" => {
                entries.sort_by(|a, b| match (&a.number, &b.number) {
                    (Some(a_num), Some(b_num)) => a_num.cmp(b_num),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.title.cmp(&b.title),
                });
            }
            "date" => {
                entries.sort_by(|a, b| match (&a.date, &b.date) {
                    (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.title.cmp(&b.title),
                });
            }
            "status" => {
                entries.sort_by(|a, b| {
                    let status_order = self
                        .get_status_order(&a.status)
                        .cmp(&self.get_status_order(&b.status));
                    if status_order == std::cmp::Ordering::Equal {
                        a.title.cmp(&b.title)
                    } else {
                        status_order
                    }
                });
            }
            "title" => {
                entries.sort_by(|a, b| a.title.cmp(&b.title));
            }
            _ => {
                log::warn!("Unknown sort criteria: {}, using 'number'", self.sort);
                self.sort_entries(entries); // Recursively call with number sorting
            }
        }
    }

    /// Get status ordering for sorting (accepted first, then others)
    fn get_status_order(&self, status: &str) -> u8 {
        match status.to_lowercase().as_str() {
            "accepted" => 1,
            "proposed" => 2,
            "rejected" => 3,
            "deprecated" => 4,
            "superseded" => 5,
            _ => 6,
        }
    }

    /// Determine output path for index file
    fn get_output_path(&self, adr_dir: &PathBuf) -> PathBuf {
        if let Some(ref output) = self.output {
            output.clone()
        } else {
            adr_dir.join("index.md")
        }
    }

    /// Generate default index content
    fn generate_default_index(
        &self,
        entries: &[AdrIndexEntry],
        _adr_dir: &PathBuf,
    ) -> Result<String> {
        let mut content = String::new();

        // Header
        content.push_str("# Architecture Decision Records (ADRs) Index\n\n");
        content.push_str(&format!(
            "This index contains {} Architecture Decision Records.\n\n",
            entries.len()
        ));

        // Generate timestamp
        let now = chrono::Utc::now();
        content.push_str(&format!(
            "*Last updated: {}*\n\n",
            now.format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Status summary
        if !entries.is_empty() {
            let status_counts = self.get_status_counts(entries);
            content.push_str("## Status Summary\n\n");
            for (status, count) in status_counts {
                let badge = if self.badges {
                    format!(" {}", self.get_status_badge(&status))
                } else {
                    String::new()
                };
                content.push_str(&format!("- **{status}**: {count}{badge}\n"));
            }
            content.push('\n');
        }

        // ADR table
        content.push_str("## ADR List\n\n");

        if entries.is_empty() {
            content.push_str("*No ADRs found.*\n");
        } else {
            // Table header
            if self.badges {
                content.push_str("| Number | Title | Status | Date | Deciders |\n");
                content.push_str("|--------|-------|--------|------|----------|\n");
            } else {
                content.push_str("| Number | Title | Status | Date | Deciders |\n");
                content.push_str("|--------|-------|--------|------|----------|\n");
            }

            // Table rows
            for entry in entries {
                let number_display = if let Some(num) = entry.number {
                    format!("{num:04}")
                } else if let Some(ref id) = entry.id {
                    id.clone()
                } else {
                    "N/A".to_string()
                };

                let title_link = format!("[{}]({})", entry.title, entry.relative_path);

                let status_display = if self.badges {
                    format!("{} {}", entry.status, self.get_status_badge(&entry.status))
                } else {
                    entry.status.clone()
                };

                let date_display = entry
                    .date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "N/A".to_string());

                let deciders_display = if entry.deciders.is_empty() {
                    "N/A".to_string()
                } else {
                    entry.deciders.join(", ")
                };

                content.push_str(&format!(
                    "| {number_display} | {title_link} | {status_display} | {date_display} | {deciders_display} |\n"
                ));
            }
        }

        content.push('\n');

        // Footer
        content.push_str("---\n\n");
        content.push_str("*This index was automatically generated by [ADRScan](https://github.com/tbowman01/PhotonDrift).*\n");

        Ok(content)
    }

    /// Generate index using custom template
    fn generate_custom_index(
        &self,
        entries: &[AdrIndexEntry],
        template_path: &PathBuf,
        _adr_dir: &PathBuf,
    ) -> Result<String> {
        let template_content = std::fs::read_to_string(template_path).map_err(AdrscanError::Io)?;

        // Simple template variable replacement
        let mut content = template_content;

        // Replace template variables
        content = content.replace("{{ADR_COUNT}}", &entries.len().to_string());
        content = content.replace(
            "{{TIMESTAMP}}",
            &chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        );

        // Generate ADR list for template
        let mut adr_list = String::new();
        for entry in entries {
            let number_display = if let Some(num) = entry.number {
                format!("{num:04}")
            } else if let Some(ref id) = entry.id {
                id.clone()
            } else {
                "N/A".to_string()
            };

            adr_list.push_str(&format!(
                "- [{}]({}) - {} ({})\n",
                number_display, entry.relative_path, entry.title, entry.status
            ));
        }
        content = content.replace("{{ADR_LIST}}", &adr_list);

        Ok(content)
    }

    /// Get status counts for summary
    fn get_status_counts(&self, entries: &[AdrIndexEntry]) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for entry in entries {
            *counts.entry(entry.status.clone()).or_insert(0) += 1;
        }

        let mut sorted_counts: Vec<_> = counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| {
            self.get_status_order(&a.0)
                .cmp(&self.get_status_order(&b.0))
        });

        sorted_counts
    }

    /// Get status badge for markdown
    fn get_status_badge(&self, status: &str) -> String {
        match status.to_lowercase().as_str() {
            "accepted" => {
                "![Accepted](https://img.shields.io/badge/Status-Accepted-green)".to_string()
            }
            "proposed" => {
                "![Proposed](https://img.shields.io/badge/Status-Proposed-yellow)".to_string()
            }
            "rejected" => {
                "![Rejected](https://img.shields.io/badge/Status-Rejected-red)".to_string()
            }
            "deprecated" => {
                "![Deprecated](https://img.shields.io/badge/Status-Deprecated-orange)".to_string()
            }
            "superseded" => {
                "![Superseded](https://img.shields.io/badge/Status-Superseded-lightgrey)"
                    .to_string()
            }
            _ => format!("![{status}](https://img.shields.io/badge/Status-{status}-blue)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config(adr_dir: &PathBuf) -> Config {
        Config {
            adr_dir: adr_dir.clone(),
            include_patterns: vec!["**/*.md".to_string()],
            exclude_patterns: vec![],
            snapshot_file: adr_dir.join(".adrscan_snapshot.json"),
            template: crate::config::TemplateConfig {
                format: "madr".to_string(),
                custom_path: None,
            },
            drift: crate::config::DriftConfig {
                enabled: true,
                detection_patterns: vec![],
            },
        }
    }

    fn create_test_adr(dir: &PathBuf, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    #[test]
    fn test_index_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());

        // Check that index.md was created
        let index_path = adr_dir.join("index.md");
        assert!(index_path.exists());

        // Check content
        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("Architecture Decision Records (ADRs) Index"));
        assert!(content.contains("*No ADRs found.*"));
    }

    #[test]
    fn test_index_with_adr_files() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        // Create test ADR files
        create_test_adr(
            &adr_dir,
            "0001-use-react.md",
            r#"---
title: "Use React for Frontend"
status: accepted
date: 2023-01-15
deciders: ["Alice", "Bob"]
id: "0001"
---

# Use React for Frontend

This is ADR 001.
"#,
        );

        create_test_adr(
            &adr_dir,
            "0002-use-postgres.md",
            r#"---
title: "Use PostgreSQL Database"
status: proposed
date: 2023-02-01
deciders: ["Charlie"]
id: "0002"
---

# Use PostgreSQL Database

This is ADR 002.
"#,
        );

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());

        // Check that index.md was created
        let index_path = adr_dir.join("index.md");
        assert!(index_path.exists());

        // Check content
        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("Architecture Decision Records (ADRs) Index"));
        assert!(content.contains("Use React for Frontend"));
        assert!(content.contains("Use PostgreSQL Database"));
        assert!(content.contains("This index contains 2 Architecture Decision Records"));
    }

    #[test]
    fn test_index_with_badges() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        create_test_adr(
            &adr_dir,
            "test.md",
            r#"---
title: "Test ADR"
status: accepted
---

# Test ADR
"#,
        );

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: true,
            template: None,
            status_filter: None,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());

        let index_path = adr_dir.join("index.md");
        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("![Accepted](https://img.shields.io/badge/Status-Accepted-green)"));
    }

    #[test]
    fn test_index_with_status_filter() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        create_test_adr(
            &adr_dir,
            "accepted.md",
            r#"---
title: "Accepted ADR"
status: accepted
---

# Accepted ADR
"#,
        );

        create_test_adr(
            &adr_dir,
            "proposed.md",
            r#"---
title: "Proposed ADR"
status: proposed
---

# Proposed ADR
"#,
        );

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: Some("accepted".to_string()),
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());

        let index_path = adr_dir.join("index.md");
        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("Accepted ADR"));
        assert!(!content.contains("Proposed ADR"));
        assert!(content.contains("This index contains 1 Architecture Decision Records"));
    }

    #[test]
    fn test_index_sorting() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();

        // Create test entries
        let entry1 = AdrIndexEntry {
            file_path: PathBuf::from("test1.md"),
            relative_path: "test1.md".to_string(),
            id: Some("001".to_string()),
            number: Some(1),
            title: "Z Title".to_string(),
            status: "proposed".to_string(),
            date: Some(NaiveDate::from_ymd_opt(2023, 12, 1).unwrap()),
            deciders: vec![],
            tags: vec![],
            supersedes: vec![],
            relates_to: vec![],
        };

        let entry2 = AdrIndexEntry {
            file_path: PathBuf::from("test2.md"),
            relative_path: "test2.md".to_string(),
            id: Some("002".to_string()),
            number: Some(2),
            title: "A Title".to_string(),
            status: "accepted".to_string(),
            date: Some(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap()),
            deciders: vec![],
            tags: vec![],
            supersedes: vec![],
            relates_to: vec![],
        };

        let mut entries = vec![entry1, entry2];

        // Test number sorting
        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };
        cmd.sort_entries(&mut entries);
        assert_eq!(entries[0].number, Some(1));
        assert_eq!(entries[1].number, Some(2));

        // Test title sorting
        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "title".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };
        cmd.sort_entries(&mut entries);
        assert_eq!(entries[0].title, "A Title");
        assert_eq!(entries[1].title, "Z Title");
    }

    #[test]
    fn test_extract_number() {
        let cmd = IndexCommand {
            adr_dir: None,
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        // Test extracting from ID
        assert_eq!(
            cmd.extract_number(&Some("001".to_string()), &PathBuf::from("test.md")),
            Some(1)
        );
        assert_eq!(
            cmd.extract_number(&Some("ADR-123".to_string()), &PathBuf::from("test.md")),
            Some(123)
        );

        // Test extracting from filename
        assert_eq!(
            cmd.extract_number(&None, &PathBuf::from("0042-test.md")),
            Some(42)
        );
        assert_eq!(
            cmd.extract_number(&None, &PathBuf::from("adr-007-test.md")),
            Some(7)
        );

        // Test no number found
        assert_eq!(
            cmd.extract_number(&None, &PathBuf::from("no-number.md")),
            None
        );
    }

    #[test]
    fn test_status_badge_generation() {
        let cmd = IndexCommand {
            adr_dir: None,
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        assert!(cmd
            .get_status_badge("accepted")
            .contains("Status-Accepted-green"));
        assert!(cmd
            .get_status_badge("proposed")
            .contains("Status-Proposed-yellow"));
        assert!(cmd
            .get_status_badge("rejected")
            .contains("Status-Rejected-red"));
        assert!(cmd
            .get_status_badge("deprecated")
            .contains("Status-Deprecated-orange"));
        assert!(cmd
            .get_status_badge("superseded")
            .contains("Status-Superseded-lightgrey"));
        assert!(cmd
            .get_status_badge("custom")
            .contains("Status-custom-blue"));
    }

    #[test]
    fn test_custom_template() {
        let temp_dir = TempDir::new().unwrap();

        // Create separate directories for ADRs and templates
        let adr_dir = temp_dir.path().join("adr");
        fs::create_dir(&adr_dir).unwrap();
        let template_dir = temp_dir.path().join("templates");
        fs::create_dir(&template_dir).unwrap();

        let config = create_test_config(&adr_dir);

        // Create custom template
        let template_path = template_dir.join("template.md");
        fs::write(
            &template_path,
            r#"# Custom ADR Index

Total ADRs: {{ADR_COUNT}}
Generated: {{TIMESTAMP}}

## ADRs

{{ADR_LIST}}
"#,
        )
        .unwrap();

        create_test_adr(
            &adr_dir,
            "test.md",
            r#"---
title: "Test ADR"
status: accepted
---

# Test ADR
"#,
        );

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: Some(template_path),
            status_filter: None,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());

        let index_path = adr_dir.join("index.md");
        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("# Custom ADR Index"));
        assert!(content.contains("Total ADRs: 1"));
        assert!(content.contains("Test ADR"));
    }

    #[test]
    fn test_custom_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        let custom_output = temp_dir.path().join("custom-index.md");

        let cmd = IndexCommand {
            adr_dir: Some(adr_dir.clone()),
            output: Some(custom_output.clone()),
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());
        assert!(custom_output.exists());
    }

    #[test]
    fn test_status_ordering() {
        let cmd = IndexCommand {
            adr_dir: None,
            output: None,
            sort: "number".to_string(),
            badges: false,
            template: None,
            status_filter: None,
        };

        assert_eq!(cmd.get_status_order("accepted"), 1);
        assert_eq!(cmd.get_status_order("proposed"), 2);
        assert_eq!(cmd.get_status_order("rejected"), 3);
        assert_eq!(cmd.get_status_order("deprecated"), 4);
        assert_eq!(cmd.get_status_order("superseded"), 5);
        assert_eq!(cmd.get_status_order("unknown"), 6);
    }
}
