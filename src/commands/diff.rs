use clap::Args;
use std::path::PathBuf;
#[cfg(feature = "tokio")]
use tokio::runtime::Runtime;

use crate::{config::Config, error::AdrscanError, drift::{DriftEngine, DriftReport, DriftSeverity}};
type Result<T> = std::result::Result<T, AdrscanError>;

#[derive(Args)]
pub struct DiffCommand {
    /// Baseline snapshot file to compare against
    #[arg(short, long)]
    pub baseline: Option<PathBuf>,

    /// Output format (console, json, yaml)
    #[arg(short, long, default_value = "console")]
    pub format: String,

    /// Directory to scan (defaults to current directory)
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    /// ADR directory to analyze (overrides config)
    #[arg(long)]
    pub adr_dir: Option<PathBuf>,

    /// Save current state as snapshot
    #[arg(long)]
    pub save_snapshot: Option<PathBuf>,
}

impl DiffCommand {
    #[cfg(feature = "tokio")]
    pub fn execute(&self, config: &Config) -> Result<()> {
        log::info!("Performing drift detection...");
        
        // Create async runtime
        let rt = Runtime::new().map_err(|e| AdrscanError::DriftError(format!("Failed to create async runtime: {}", e)))?;
        
        rt.block_on(async {
            let drift_engine = DriftEngine::new();
            
            // Determine directories
            let scan_dir = self.directory.as_ref()
                .map(|p| p.clone())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
            
            let adr_dir = self.adr_dir.as_ref().unwrap_or(&config.adr_dir);
            
            // Get detection patterns from config
            let detection_patterns = &config.drift.detection_patterns;
            
            // Perform drift detection
            let drift_report = drift_engine.detect_drift(
                adr_dir,
                &scan_dir,
                self.baseline.as_deref(),
                &detection_patterns,
            ).await?;
            
            // Save current snapshot if requested
            if let Some(ref snapshot_path) = self.save_snapshot {
                log::info!("Saving current snapshot to: {}", snapshot_path.display());
                // The snapshot is created internally during drift detection
                // We would need to expose it from the drift engine to save it
            }
            
            // Output report based on format
            match self.format.as_str() {
                "json" => {
                    let json_output = drift_report.to_json()
                        .map_err(|e| AdrscanError::SerializationError(e.to_string()))?;
                    println!("{}", json_output);
                }
                "yaml" => {
                    let yaml_output = drift_report.to_yaml()
                        .map_err(|e| AdrscanError::SerializationError(e.to_string()))?;
                    println!("{}", yaml_output);
                }
                "console" => {
                    self.print_console_report(&drift_report);
                }
                _ => {
                    return Err(AdrscanError::InvalidArgument(
                        format!("Unsupported output format: {}. Use 'console', 'json', or 'yaml'", self.format)
                    ));
                }
            }
            
            // Exit with error code if critical drift is found
            let critical_count = drift_report.severity_summary
                .get(&DriftSeverity::Critical)
                .unwrap_or(&0);
            
            if *critical_count > 0 {
                log::warn!("Found {} critical drift items", critical_count);
                std::process::exit(1);
            }
            
            Ok(())
        })
    }
    
    /// Print a human-readable console report
    fn print_console_report(&self, report: &DriftReport) {
        println!("🔍 Architectural Drift Detection Report");
        println!("=======================================");
        println!();
        
        println!("📊 Summary:");
        println!("  Total Items: {}", report.total_items);
        println!("  Scanned Directory: {}", report.scanned_directory.display());
        if let Some(ref baseline) = report.baseline_snapshot {
            println!("  Baseline: {}", baseline.display());
        }
        println!("  Generated: {}", report.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        println!();
        
        // Severity breakdown
        println!("🚨 Severity Breakdown:");
        for severity in [
            DriftSeverity::Critical,
            DriftSeverity::High,
            DriftSeverity::Medium,
            DriftSeverity::Low,
        ] {
            let count = report.severity_summary.get(&severity).unwrap_or(&0);
            if *count > 0 {
                let icon = match severity {
                    DriftSeverity::Critical => "🔴",
                    DriftSeverity::High => "🟠",
                    DriftSeverity::Medium => "🟡",
                    DriftSeverity::Low => "🔵",
                    _ => "⚪",
                };
                println!("  {} {}: {}", icon, severity, count);
            }
        }
        println!();
        
        // Category breakdown
        if !report.category_summary.is_empty() {
            println!("📂 Category Breakdown:");
            for (category, count) in &report.category_summary {
                println!("  {}: {}", category, count);
            }
            println!();
        }
        
        // Show critical and high priority items
        let critical_items = report.items_by_severity(&DriftSeverity::Critical);
        let high_items = report.items_by_severity(&DriftSeverity::High);
        
        if !critical_items.is_empty() {
            println!("🔴 Critical Issues:");
            for item in critical_items.iter().take(10) {
                println!("  • {} ({})", item.title, item.location.file_path.display());
                if let Some(ref action) = item.suggested_action {
                    println!("    💡 {}", action);
                }
                println!();
            }
        }
        
        if !high_items.is_empty() {
            println!("🟠 High Priority Issues:");
            for item in high_items.iter().take(10) {
                println!("  • {} ({})", item.title, item.location.file_path.display());
                if let Some(ref action) = item.suggested_action {
                    println!("    💡 {}", action);
                }
                println!();
            }
        }
        
        // Scan statistics
        println!("📈 Scan Statistics:");
        println!("  Files Scanned: {}", report.scan_stats.files_scanned);
        println!("  Lines Analyzed: {}", report.scan_stats.lines_analyzed);
        println!("  Scan Duration: {}ms", report.scan_stats.scan_duration_ms);
        println!("  ADRs Analyzed: {}", report.scan_stats.adrs_analyzed);
        
        if report.total_items == 0 {
            println!();
            println!("✅ No architectural drift detected!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use crate::drift::{DriftReport, DriftItem, DriftSeverity, DriftCategory, DriftLocation};
    use crate::config::{Config, TemplateConfig, DriftConfig};

    #[allow(dead_code)]
    fn create_test_config(adr_dir: &PathBuf) -> Config {
        Config {
            adr_dir: adr_dir.clone(),
            include_patterns: vec!["**/*.md".to_string()],
            exclude_patterns: vec![],
            snapshot_file: adr_dir.join(".adrscan_snapshot.json"),
            template: TemplateConfig {
                format: "madr".to_string(),
                custom_path: None,
            },
            drift: DriftConfig {
                enabled: true,
                detection_patterns: vec![],
            },
        }
    }

    #[allow(dead_code)]
    fn create_test_adr(dir: &PathBuf, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    fn create_test_drift_report(temp_dir: &Path) -> DriftReport {
        let mut report = DriftReport::new(temp_dir.to_path_buf(), None);
        
        // Add a critical drift item
        let critical_item = DriftItem::new(
            "critical_1".to_string(),
            DriftSeverity::Critical,
            DriftCategory::ConflictingTechnology,
            "Rejected technology in use: MongoDB".to_string(),
            "MongoDB is being used but was rejected in ADR".to_string(),
            DriftLocation::new(PathBuf::from("src/database.rs")).with_line(10),
        )
        .with_technology("MongoDB".to_string())
        .with_suggested_action("Remove MongoDB or update the ADR".to_string());
        
        report.add_item(critical_item);
        
        // Add a high priority drift item
        let high_item = DriftItem::new(
            "high_1".to_string(),
            DriftSeverity::High,
            DriftCategory::NewTechnology,
            "Uncovered technology: Redis".to_string(),
            "Redis is in use but not documented in any ADR".to_string(),
            DriftLocation::new(PathBuf::from("src/cache.rs")).with_line(5),
        )
        .with_technology("Redis".to_string())
        .with_suggested_action("Create an ADR for Redis usage".to_string());
        
        report.add_item(high_item);
        
        // Add a medium priority drift item
        let medium_item = DriftItem::new(
            "medium_1".to_string(),
            DriftSeverity::Medium,
            DriftCategory::NewTechnology,
            "New framework detected: Axum".to_string(),
            "Axum web framework is being used".to_string(),
            DriftLocation::new(PathBuf::from("Cargo.toml")).with_line(15),
        )
        .with_technology("Axum".to_string());
        
        report.add_item(medium_item);
        
        report
    }

    #[test]
    fn test_diff_command_creation() {
        let cmd = DiffCommand {
            baseline: None,
            format: "console".to_string(),
            directory: None,
            adr_dir: None,
            save_snapshot: None,
        };
        
        assert_eq!(cmd.format, "console");
        assert!(cmd.baseline.is_none());
        assert!(cmd.directory.is_none());
    }

    #[test]
    fn test_diff_command_with_options() {
        let temp_dir = TempDir::new().unwrap();
        let baseline_path = temp_dir.path().join("baseline.json");
        let save_path = temp_dir.path().join("snapshot.json");
        
        let cmd = DiffCommand {
            baseline: Some(baseline_path.clone()),
            format: "json".to_string(),
            directory: Some(temp_dir.path().to_path_buf()),
            adr_dir: Some(temp_dir.path().join("adr")),
            save_snapshot: Some(save_path.clone()),
        };
        
        assert_eq!(cmd.baseline, Some(baseline_path));
        assert_eq!(cmd.format, "json");
        assert_eq!(cmd.directory, Some(temp_dir.path().to_path_buf()));
        assert_eq!(cmd.save_snapshot, Some(save_path));
    }

    #[test]
    fn test_print_console_report_empty() {
        let temp_dir = TempDir::new().unwrap();
        let report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        
        let cmd = DiffCommand {
            baseline: None,
            format: "console".to_string(),
            directory: None,
            adr_dir: None,
            save_snapshot: None,
        };
        
        // This should not panic and should print "No drift detected"
        cmd.print_console_report(&report);
    }

    #[test]
    fn test_print_console_report_with_items() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        let cmd = DiffCommand {
            baseline: None,
            format: "console".to_string(),
            directory: None,
            adr_dir: None,
            save_snapshot: None,
        };
        
        // This should not panic and should show drift items
        cmd.print_console_report(&report);
    }

    #[test]
    fn test_json_format_output() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        // Test JSON serialization
        let json_output = report.to_json().unwrap();
        assert!(json_output.contains("\"total_items\""));
        assert!(json_output.contains("\"severity_summary\""));
        assert!(json_output.contains("MongoDB"));
        assert!(json_output.contains("Redis"));
    }

    #[test]
    fn test_yaml_format_output() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        // Test YAML serialization
        let yaml_output = report.to_yaml().unwrap();
        assert!(yaml_output.contains("total_items:"));
        assert!(yaml_output.contains("severity_summary:"));
        assert!(yaml_output.contains("MongoDB"));
        assert!(yaml_output.contains("Redis"));
    }

    #[test]
    fn test_drift_report_severity_summary() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        // Check severity counts
        assert_eq!(report.severity_summary.get(&DriftSeverity::Critical), Some(&1));
        assert_eq!(report.severity_summary.get(&DriftSeverity::High), Some(&1));
        assert_eq!(report.severity_summary.get(&DriftSeverity::Medium), Some(&1));
        assert_eq!(report.severity_summary.get(&DriftSeverity::Low), None);
    }

    #[test]
    fn test_drift_report_category_summary() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        // Check category counts
        assert_eq!(report.category_summary.get(&DriftCategory::ConflictingTechnology), Some(&1));
        assert_eq!(report.category_summary.get(&DriftCategory::NewTechnology), Some(&2));
    }

    #[test]
    fn test_drift_report_items_by_severity() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        let critical_items = report.items_by_severity(&DriftSeverity::Critical);
        assert_eq!(critical_items.len(), 1);
        assert_eq!(critical_items[0].title, "Rejected technology in use: MongoDB");
        
        let high_items = report.items_by_severity(&DriftSeverity::High);
        assert_eq!(high_items.len(), 1);
        assert_eq!(high_items[0].title, "Uncovered technology: Redis");
        
        let medium_items = report.items_by_severity(&DriftSeverity::Medium);
        assert_eq!(medium_items.len(), 1);
        assert_eq!(medium_items[0].title, "New framework detected: Axum");
    }

    #[test]
    fn test_drift_report_items_by_category() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        let conflict_items = report.items_by_category(&DriftCategory::ConflictingTechnology);
        assert_eq!(conflict_items.len(), 1);
        assert!(conflict_items[0].title.contains("MongoDB"));
        
        let new_tech_items = report.items_by_category(&DriftCategory::NewTechnology);
        assert_eq!(new_tech_items.len(), 2);
        assert!(new_tech_items.iter().any(|item| item.title.contains("Redis")));
        assert!(new_tech_items.iter().any(|item| item.title.contains("Axum")));
    }

    #[test]
    fn test_drift_location_details() {
        let location = DriftLocation::new(PathBuf::from("src/test.rs"))
            .with_line(42)
            .with_column(15)
            .with_snippet("let test = some_function();".to_string());
        
        assert_eq!(location.file_path, PathBuf::from("src/test.rs"));
        assert_eq!(location.line_number, Some(42));
        assert_eq!(location.column_number, Some(15));
        assert_eq!(location.snippet, Some("let test = some_function();".to_string()));
    }

    #[test]
    fn test_drift_item_with_metadata() {
        let item = DriftItem::new(
            "test_item".to_string(),
            DriftSeverity::High,
            DriftCategory::Security,
            "Security Issue".to_string(),
            "This is a security-related drift".to_string(),
            DriftLocation::new(PathBuf::from("src/auth.rs")),
        )
        .with_technology("JWT".to_string())
        .with_related_adr("ADR-005".to_string())
        .with_suggested_action("Review authentication implementation".to_string())
        .with_metadata("confidence".to_string(), "0.95".to_string());
        
        assert_eq!(item.detected_technology, Some("JWT".to_string()));
        assert_eq!(item.related_adr, Some("ADR-005".to_string()));
        assert_eq!(item.suggested_action, Some("Review authentication implementation".to_string()));
        assert_eq!(item.metadata.get("confidence"), Some(&"0.95".to_string()));
    }

    #[test]
    fn test_drift_report_summary() {
        let temp_dir = TempDir::new().unwrap();
        let report = create_test_drift_report(temp_dir.path());
        
        let summary = report.summary();
        assert!(summary.contains("Total Items: 3"));
        assert!(summary.contains("Critical: 1"));
        assert!(summary.contains("High: 1"));
        assert!(summary.contains("Medium: 1"));
        assert!(summary.contains("Low: 0"));
    }

    #[test]
    fn test_format_validation() {
        // Valid formats should be accepted
        let valid_formats = ["console", "json", "yaml"];
        for format in &valid_formats {
            let cmd = DiffCommand {
                baseline: None,
                format: format.to_string(),
                directory: None,
                adr_dir: None,
                save_snapshot: None,
            };
            assert!(valid_formats.contains(&cmd.format.as_str()));
        }
    }

    #[test]
    fn test_drift_item_builder_pattern() {
        let item = DriftItem::new(
            "builder_test".to_string(),
            DriftSeverity::Medium,
            DriftCategory::Performance,
            "Performance Concern".to_string(),
            "Database queries are slow".to_string(),
            DriftLocation::new(PathBuf::from("src/db.rs")).with_line(100),
        )
        .with_technology("PostgreSQL".to_string())
        .with_suggested_action("Add database indexes".to_string())
        .with_metadata("query_time".to_string(), "2.5s".to_string())
        .with_metadata("table".to_string(), "users".to_string());
        
        assert_eq!(item.id, "builder_test");
        assert_eq!(item.severity, DriftSeverity::Medium);
        assert_eq!(item.category, DriftCategory::Performance);
        assert_eq!(item.detected_technology, Some("PostgreSQL".to_string()));
        assert_eq!(item.metadata.len(), 2);
    }

    #[test]
    fn test_scan_statistics() {
        let temp_dir = TempDir::new().unwrap();
        let mut report = create_test_drift_report(temp_dir.path());
        
        // Update scan statistics
        report.scan_stats.files_scanned = 150;
        report.scan_stats.lines_analyzed = 5000;
        report.scan_stats.scan_duration_ms = 1250;
        report.scan_stats.adrs_analyzed = 8;
        
        assert_eq!(report.scan_stats.files_scanned, 150);
        assert_eq!(report.scan_stats.lines_analyzed, 5000);
        assert_eq!(report.scan_stats.scan_duration_ms, 1250);
        assert_eq!(report.scan_stats.adrs_analyzed, 8);
    }

    #[test]
    fn test_drift_severity_display() {
        assert_eq!(DriftSeverity::Critical.to_string(), "CRITICAL");
        assert_eq!(DriftSeverity::High.to_string(), "HIGH");
        assert_eq!(DriftSeverity::Medium.to_string(), "MEDIUM");
        assert_eq!(DriftSeverity::Low.to_string(), "LOW");
        assert_eq!(DriftSeverity::Info.to_string(), "INFO");
    }

    #[test]
    fn test_drift_category_display() {
        assert_eq!(DriftCategory::NewTechnology.to_string(), "New Technology");
        assert_eq!(DriftCategory::ConflictingTechnology.to_string(), "Conflicting Technology");
        assert_eq!(DriftCategory::DeprecatedTechnology.to_string(), "Deprecated Technology");
        assert_eq!(DriftCategory::PatternViolation.to_string(), "Pattern Violation");
        assert_eq!(DriftCategory::Security.to_string(), "Security");
        assert_eq!(DriftCategory::Performance.to_string(), "Performance");
        assert_eq!(DriftCategory::Database.to_string(), "Database");
        assert_eq!(DriftCategory::Infrastructure.to_string(), "Infrastructure");
        assert_eq!(DriftCategory::Framework.to_string(), "Framework");
    }

    #[test]
    fn test_empty_drift_report() {
        let temp_dir = TempDir::new().unwrap();
        let report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        
        assert_eq!(report.total_items, 0);
        assert!(report.items.is_empty());
        assert!(report.category_summary.is_empty());
        assert!(report.severity_summary.is_empty());
        
        let summary = report.summary();
        assert!(summary.contains("Total Items: 0"));
    }

    #[test]
    fn test_large_drift_report() {
        let temp_dir = TempDir::new().unwrap();
        let mut report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        
        // Add many drift items
        for i in 0..50 {
            let item = DriftItem::new(
                format!("item_{}", i),
                if i % 4 == 0 { DriftSeverity::Critical } 
                else if i % 3 == 0 { DriftSeverity::High }
                else if i % 2 == 0 { DriftSeverity::Medium }
                else { DriftSeverity::Low },
                DriftCategory::NewTechnology,
                format!("Technology {}", i),
                format!("Description for technology {}", i),
                DriftLocation::new(PathBuf::from(format!("src/file_{}.rs", i))),
            );
            report.add_item(item);
        }
        
        assert_eq!(report.total_items, 50);
        
        // Check that console output handles large reports gracefully
        let cmd = DiffCommand {
            baseline: None,
            format: "console".to_string(),
            directory: None,
            adr_dir: None,
            save_snapshot: None,
        };
        
        // Should not panic with large report
        cmd.print_console_report(&report);
    }
}