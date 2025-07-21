use clap::Args;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::AdrscanError, parser::AdrParser};
type Result<T> = std::result::Result<T, AdrscanError>;

#[derive(Args)]
pub struct InventoryCommand {
    /// ADR directory to scan
    #[arg(short, long)]
    pub adr_dir: Option<PathBuf>,

    /// Output format (console, json, csv)
    #[arg(short, long, default_value = "console")]
    pub format: String,

    /// Filter by ADR status
    #[arg(short, long)]
    pub status: Option<String>,

    /// Filter by tag
    #[arg(short, long)]
    pub tag: Option<String>,

    /// Sort by field (date, status, title)
    #[arg(long, default_value = "date")]
    pub sort_by: String,

    /// Include file statistics
    #[arg(long)]
    pub stats: bool,
}

/// ADR inventory summary
#[derive(Debug, Serialize, Deserialize)]
pub struct AdrInventory {
    pub total_count: usize,
    pub status_breakdown: HashMap<String, usize>,
    pub tag_breakdown: HashMap<String, usize>,
    pub adrs: Vec<AdrSummary>,
    pub statistics: Option<InventoryStats>,
}

/// Summary of a single ADR
#[derive(Debug, Serialize, Deserialize)]
pub struct AdrSummary {
    pub path: String,
    pub id: Option<String>,
    pub title: String,
    pub status: String,
    pub date: Option<String>,
    pub deciders: Vec<String>,
    pub tags: Vec<String>,
    pub file_size: u64,
    pub line_count: usize,
}

/// Inventory statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryStats {
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub total_lines: usize,
    pub average_file_size: f64,
    pub average_lines_per_adr: f64,
}

impl InventoryCommand {
    pub fn execute(&self, config: &Config) -> Result<()> {
        log::info!("Scanning ADR inventory...");
        
        let adr_dir = self.adr_dir.as_ref().unwrap_or(&config.adr_dir);
        
        // Validate ADR directory exists
        if !adr_dir.exists() {
            return Err(AdrscanError::DirectoryNotFound(
                format!("ADR directory not found: {}", adr_dir.display())
            ));
        }

        // Scan for ADR files
        let adr_files = self.scan_adr_files(adr_dir)?;
        log::info!("Found {} ADR files", adr_files.len());

        // Parse ADR files and build inventory
        let mut adrs = Vec::new();
        for file_path in adr_files {
            match self.parse_adr_file(&file_path) {
                Ok(adr_summary) => {
                    // Apply filters
                    if self.should_include_adr(&adr_summary) {
                        adrs.push(adr_summary);
                    }
                }
                Err(e) => {
                    log::warn!("Failed to parse ADR file {}: {}", file_path.display(), e);
                    // Continue processing other files
                }
            }
        }

        // Sort ADRs
        self.sort_adrs(&mut adrs);

        // Build inventory summary
        let inventory = self.build_inventory(adrs)?;

        // Output results
        self.output_inventory(&inventory)?;

        Ok(())
    }

    /// Scan directory for ADR files (markdown files)
    fn scan_adr_files(&self, adr_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut adr_files = Vec::new();

        for entry in walkdir::WalkDir::new(adr_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" || extension == "markdown" {
                        adr_files.push(path.to_path_buf());
                    }
                }
            }
        }

        adr_files.sort();
        Ok(adr_files)
    }

    /// Parse a single ADR file into a summary
    fn parse_adr_file(&self, file_path: &PathBuf) -> Result<AdrSummary> {
        let adr_doc = AdrParser::parse_file(file_path)?;
        
        // Get file metadata
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| AdrscanError::Io(e))?;
        let file_size = metadata.len();

        // Count lines
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| AdrscanError::Io(e))?;
        let line_count = content.lines().count();

        // Format date
        let date_str = adr_doc.metadata.date.map(|d| d.format("%Y-%m-%d").to_string());

        Ok(AdrSummary {
            path: file_path.to_string_lossy().to_string(),
            id: adr_doc.metadata.id,
            title: adr_doc.metadata.title,
            status: adr_doc.metadata.status,
            date: date_str,
            deciders: adr_doc.metadata.deciders,
            tags: adr_doc.metadata.tags,
            file_size,
            line_count,
        })
    }

    /// Check if ADR should be included based on filters
    fn should_include_adr(&self, adr: &AdrSummary) -> bool {
        // Filter by status
        if let Some(ref status_filter) = self.status {
            if !adr.status.eq_ignore_ascii_case(status_filter) {
                return false;
            }
        }

        // Filter by tag
        if let Some(ref tag_filter) = self.tag {
            if !adr.tags.iter().any(|tag| tag.eq_ignore_ascii_case(tag_filter)) {
                return false;
            }
        }

        true
    }

    /// Sort ADRs based on sort criteria
    fn sort_adrs(&self, adrs: &mut Vec<AdrSummary>) {
        match self.sort_by.as_str() {
            "date" => {
                adrs.sort_by(|a, b| {
                    // Sort by date, putting None dates last
                    match (&a.date, &b.date) {
                        (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            }
            "status" => {
                adrs.sort_by(|a, b| a.status.cmp(&b.status));
            }
            "title" => {
                adrs.sort_by(|a, b| a.title.cmp(&b.title));
            }
            _ => {
                log::warn!("Unknown sort field: {}, using date", self.sort_by);
                adrs.sort_by(|a, b| {
                    match (&a.date, &b.date) {
                        (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            }
        }
    }

    /// Build comprehensive inventory from ADR summaries
    fn build_inventory(&self, adrs: Vec<AdrSummary>) -> Result<AdrInventory> {
        let total_count = adrs.len();

        // Build status breakdown
        let mut status_breakdown: HashMap<String, usize> = HashMap::new();
        for adr in &adrs {
            *status_breakdown.entry(adr.status.clone()).or_insert(0) += 1;
        }

        // Build tag breakdown
        let mut tag_breakdown: HashMap<String, usize> = HashMap::new();
        for adr in &adrs {
            for tag in &adr.tags {
                *tag_breakdown.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        // Calculate statistics if requested
        let statistics = if self.stats {
            let total_files = adrs.len();
            let total_size_bytes: u64 = adrs.iter().map(|adr| adr.file_size).sum();
            let total_lines: usize = adrs.iter().map(|adr| adr.line_count).sum();
            
            let average_file_size = if total_files > 0 {
                total_size_bytes as f64 / total_files as f64
            } else {
                0.0
            };
            
            let average_lines_per_adr = if total_files > 0 {
                total_lines as f64 / total_files as f64
            } else {
                0.0
            };

            Some(InventoryStats {
                total_files,
                total_size_bytes,
                total_lines,
                average_file_size,
                average_lines_per_adr,
            })
        } else {
            None
        };

        Ok(AdrInventory {
            total_count,
            status_breakdown,
            tag_breakdown,
            adrs,
            statistics,
        })
    }

    /// Output inventory in the requested format
    fn output_inventory(&self, inventory: &AdrInventory) -> Result<()> {
        match self.format.as_str() {
            "json" => {
                let json_output = serde_json::to_string_pretty(inventory)
                    .map_err(|e| AdrscanError::SerializationError(e.to_string()))?;
                println!("{}", json_output);
            }
            "csv" => {
                self.output_csv(inventory)?;
            }
            "console" | _ => {
                self.output_console(inventory)?;
            }
        }
        Ok(())
    }

    /// Output inventory as CSV
    fn output_csv(&self, inventory: &AdrInventory) -> Result<()> {
        println!("Path,ID,Title,Status,Date,Deciders,Tags,FileSize,LineCount");
        for adr in &inventory.adrs {
            let deciders = adr.deciders.join(";");
            let tags = adr.tags.join(";");
            let date = adr.date.as_deref().unwrap_or("");
            let id = adr.id.as_deref().unwrap_or("");
            
            println!("{},{},{},{},{},{},{},{},{}",
                adr.path, id, adr.title, adr.status, date, deciders, tags, adr.file_size, adr.line_count);
        }
        Ok(())
    }

    /// Output inventory to console in table format
    fn output_console(&self, inventory: &AdrInventory) -> Result<()> {
        println!("🔍 ADR Inventory Report");
        println!("═══════════════════════");
        println!();

        // Summary statistics
        println!("📊 Summary:");
        println!("  Total ADRs: {}", inventory.total_count);
        println!();

        // Status breakdown
        if !inventory.status_breakdown.is_empty() {
            println!("📈 Status Breakdown:");
            let mut status_pairs: Vec<_> = inventory.status_breakdown.iter().collect();
            status_pairs.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
            for (status, count) in status_pairs {
                println!("  {}: {}", status, count);
            }
            println!();
        }

        // Tag breakdown
        if !inventory.tag_breakdown.is_empty() {
            println!("🏷️  Tag Breakdown:");
            let mut tag_pairs: Vec<_> = inventory.tag_breakdown.iter().collect();
            tag_pairs.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
            for (tag, count) in tag_pairs.iter().take(10) { // Show top 10
                println!("  {}: {}", tag, count);
            }
            if tag_pairs.len() > 10 {
                println!("  ... and {} more", tag_pairs.len() - 10);
            }
            println!();
        }

        // File statistics
        if let Some(ref stats) = inventory.statistics {
            println!("📋 File Statistics:");
            println!("  Total files: {}", stats.total_files);
            println!("  Total size: {} bytes", stats.total_size_bytes);
            println!("  Total lines: {}", stats.total_lines);
            println!("  Average file size: {:.1} bytes", stats.average_file_size);
            println!("  Average lines per ADR: {:.1}", stats.average_lines_per_adr);
            println!();
        }

        // ADR details
        if !inventory.adrs.is_empty() {
            println!("📋 ADR Details:");
            println!("  {:<4} {:<30} {:<12} {:<12} {}", "ID", "Title", "Status", "Date", "Tags");
            println!("  {}", "─".repeat(80));
            
            for adr in &inventory.adrs {
                let id = adr.id.as_deref().unwrap_or("N/A");
                let title = if adr.title.len() > 28 {
                    format!("{}...", &adr.title[..25])
                } else {
                    adr.title.clone()
                };
                let date = adr.date.as_deref().unwrap_or("N/A");
                let tags = if adr.tags.is_empty() {
                    "".to_string()
                } else {
                    adr.tags.join(",")
                };
                
                println!("  {:<4} {:<30} {:<12} {:<12} {}", 
                    id, title, adr.status, date, tags);
            }
        }

        Ok(())
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
    fn test_inventory_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        let cmd = InventoryCommand {
            adr_dir: Some(adr_dir),
            format: "json".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_inventory_with_adr_files() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        // Create test ADR files
        create_test_adr(&adr_dir, "adr-001.md", r#"---
title: "Use React for Frontend"
status: accepted
date: 2023-01-15
deciders: ["Alice", "Bob"]
tags: ["frontend", "react"]
---

# Use React for Frontend

This is the content of ADR 001.
"#);

        create_test_adr(&adr_dir, "adr-002.md", r#"---
title: "Use PostgreSQL Database"
status: proposed
date: 2023-02-01
deciders: ["Charlie"]
tags: ["database", "postgresql"]
---

# Use PostgreSQL Database

This is the content of ADR 002.
"#);

        let cmd = InventoryCommand {
            adr_dir: Some(adr_dir),
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: true,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scan_adr_files() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();

        // Create test files
        create_test_adr(&adr_dir, "adr-001.md", "# ADR 1");
        create_test_adr(&adr_dir, "adr-002.markdown", "# ADR 2");
        create_test_adr(&adr_dir, "readme.txt", "Not an ADR");

        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };

        let files = cmd.scan_adr_files(&adr_dir).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.file_name().unwrap() == "adr-001.md"));
        assert!(files.iter().any(|f| f.file_name().unwrap() == "adr-002.markdown"));
    }

    #[test]
    fn test_parse_adr_file() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();

        let content = r#"---
title: "Test ADR"
status: accepted
date: 2023-12-01
deciders: ["Alice", "Bob"]
tags: ["test", "unit"]
id: "001"
---

# Test ADR

This is test content.
Line 2
Line 3
"#;

        let file_path = create_test_adr(&adr_dir, "test.md", content);

        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };

        let summary = cmd.parse_adr_file(&file_path).unwrap();
        assert_eq!(summary.title, "Test ADR");
        assert_eq!(summary.status, "accepted");
        assert_eq!(summary.id, Some("001".to_string()));
        assert_eq!(summary.date, Some("2023-12-01".to_string()));
        assert_eq!(summary.deciders, vec!["Alice", "Bob"]);
        assert_eq!(summary.tags, vec!["test", "unit"]);
        assert!(summary.line_count > 0);
        assert!(summary.file_size > 0);
    }

    #[test]
    fn test_status_filtering() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        create_test_adr(&adr_dir, "accepted.md", r#"---
title: "Accepted ADR"
status: accepted
---
# Accepted
"#);

        create_test_adr(&adr_dir, "proposed.md", r#"---
title: "Proposed ADR"
status: proposed
---
# Proposed
"#);

        let cmd = InventoryCommand {
            adr_dir: Some(adr_dir),
            format: "json".to_string(),
            status: Some("accepted".to_string()),
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tag_filtering() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().to_path_buf();
        let config = create_test_config(&adr_dir);

        create_test_adr(&adr_dir, "frontend.md", r#"---
title: "Frontend ADR"
status: accepted
tags: ["frontend", "react"]
---
# Frontend
"#);

        create_test_adr(&adr_dir, "backend.md", r#"---
title: "Backend ADR"
status: accepted
tags: ["backend", "database"]
---
# Backend
"#);

        let cmd = InventoryCommand {
            adr_dir: Some(adr_dir),
            format: "json".to_string(),
            status: None,
            tag: Some("frontend".to_string()),
            sort_by: "date".to_string(),
            stats: false,
        };

        let result = cmd.execute(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_should_include_adr() {
        let adr = AdrSummary {
            path: "test.md".to_string(),
            id: Some("001".to_string()),
            title: "Test ADR".to_string(),
            status: "accepted".to_string(),
            date: Some("2023-12-01".to_string()),
            deciders: vec!["Alice".to_string()],
            tags: vec!["frontend".to_string(), "react".to_string()],
            file_size: 100,
            line_count: 10,
        };

        // Test no filters
        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };
        assert!(cmd.should_include_adr(&adr));

        // Test status filter - match
        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: Some("accepted".to_string()),
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };
        assert!(cmd.should_include_adr(&adr));

        // Test status filter - no match
        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: Some("proposed".to_string()),
            tag: None,
            sort_by: "date".to_string(),
            stats: false,
        };
        assert!(!cmd.should_include_adr(&adr));

        // Test tag filter - match
        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: Some("frontend".to_string()),
            sort_by: "date".to_string(),
            stats: false,
        };
        assert!(cmd.should_include_adr(&adr));

        // Test tag filter - no match
        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: Some("backend".to_string()),
            sort_by: "date".to_string(),
            stats: false,
        };
        assert!(!cmd.should_include_adr(&adr));
    }

    #[test]
    fn test_sort_adrs() {
        let mut adrs = vec![
            AdrSummary {
                path: "adr1.md".to_string(),
                id: Some("001".to_string()),
                title: "Z Title".to_string(),
                status: "proposed".to_string(),
                date: Some("2023-12-01".to_string()),
                deciders: vec![],
                tags: vec![],
                file_size: 100,
                line_count: 10,
            },
            AdrSummary {
                path: "adr2.md".to_string(),
                id: Some("002".to_string()),
                title: "A Title".to_string(),
                status: "accepted".to_string(),
                date: Some("2023-11-01".to_string()),
                deciders: vec![],
                tags: vec![],
                file_size: 200,
                line_count: 20,
            },
        ];

        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "title".to_string(),
            stats: false,
        };

        cmd.sort_adrs(&mut adrs);
        assert_eq!(adrs[0].title, "A Title");
        assert_eq!(adrs[1].title, "Z Title");
    }

    #[test]
    fn test_build_inventory() {
        let adrs = vec![
            AdrSummary {
                path: "adr1.md".to_string(),
                id: Some("001".to_string()),
                title: "ADR 1".to_string(),
                status: "accepted".to_string(),
                date: Some("2023-12-01".to_string()),
                deciders: vec!["Alice".to_string()],
                tags: vec!["frontend".to_string()],
                file_size: 100,
                line_count: 10,
            },
            AdrSummary {
                path: "adr2.md".to_string(),
                id: Some("002".to_string()),
                title: "ADR 2".to_string(),
                status: "proposed".to_string(),
                date: Some("2023-11-01".to_string()),
                deciders: vec!["Bob".to_string()],
                tags: vec!["backend".to_string(), "frontend".to_string()],
                file_size: 200,
                line_count: 20,
            },
        ];

        let cmd = InventoryCommand {
            adr_dir: None,
            format: "console".to_string(),
            status: None,
            tag: None,
            sort_by: "date".to_string(),
            stats: true,
        };

        let inventory = cmd.build_inventory(adrs).unwrap();
        assert_eq!(inventory.total_count, 2);
        assert_eq!(inventory.status_breakdown.get("accepted"), Some(&1));
        assert_eq!(inventory.status_breakdown.get("proposed"), Some(&1));
        assert_eq!(inventory.tag_breakdown.get("frontend"), Some(&2));
        assert_eq!(inventory.tag_breakdown.get("backend"), Some(&1));
        
        let stats = inventory.statistics.unwrap();
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_size_bytes, 300);
        assert_eq!(stats.total_lines, 30);
        assert_eq!(stats.average_file_size, 150.0);
        assert_eq!(stats.average_lines_per_adr, 15.0);
    }
}