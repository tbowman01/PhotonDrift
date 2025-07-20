use clap::Args;
use std::path::PathBuf;
use tokio::runtime::Runtime;

use crate::{config::Config, error::AdrscanError, drift::DriftEngine};
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
    pub fn execute(&self, config: &Config) -> Result<()> {
        log::info!("Performing drift detection...");
        
        // Create async runtime
        let rt = Runtime::new().map_err(|e| AdrscanError::DriftError(format!("Failed to create async runtime: {}", e)))?;
        
        rt.block_on(async {
            let drift_engine = DriftEngine::new();
            
            // Determine directories
            let scan_dir = self.directory.as_ref()
                .or_else(|| Some(&std::env::current_dir().unwrap_or_default()))
                .unwrap();
            
            let adr_dir = self.adr_dir.as_ref().unwrap_or(&config.adr_dir);
            
            // Get detection patterns from config
            let detection_patterns = &config.drift.detection_patterns;
            
            // Perform drift detection
            let drift_report = drift_engine.detect_drift(
                adr_dir,
                scan_dir,
                self.baseline.as_deref(),
                detection_patterns,
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
                .get(&crate::drift::DriftSeverity::Critical)
                .unwrap_or(&0);
            
            if *critical_count > 0 {
                log::warn!("Found {} critical drift items", critical_count);
                std::process::exit(1);
            }
            
            Ok(())
        })
    }
    
    /// Print a human-readable console report
    fn print_console_report(&self, report: &crate::drift::DriftReport) {
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
            crate::drift::DriftSeverity::Critical,
            crate::drift::DriftSeverity::High,
            crate::drift::DriftSeverity::Medium,
            crate::drift::DriftSeverity::Low,
        ] {
            let count = report.severity_summary.get(&severity).unwrap_or(&0);
            if *count > 0 {
                let icon = match severity {
                    crate::drift::DriftSeverity::Critical => "🔴",
                    crate::drift::DriftSeverity::High => "🟠",
                    crate::drift::DriftSeverity::Medium => "🟡",
                    crate::drift::DriftSeverity::Low => "🔵",
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
        let critical_items = report.items_by_severity(&crate::drift::DriftSeverity::Critical);
        let high_items = report.items_by_severity(&crate::drift::DriftSeverity::High);
        
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