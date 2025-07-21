//! Simplified WebAssembly Module for ADRScan
//!
//! This module provides a WASM-compatible version without async/tokio dependencies

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use std::collections::HashMap;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::{
    config::{Config, DriftConfig, TemplateConfig},
    error::AdrscanError,
    parser::AdrParser,
};

// Set up panic hook for better error messages
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// JavaScript-compatible configuration object
#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    /// ADR directory path
    pub adr_dir: String,
    /// Template format
    pub template_format: String,
    /// Enable drift detection
    pub drift_enabled: bool,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmConfig {
        WasmConfig {
            adr_dir: "./docs/adr".to_string(),
            template_format: "madr".to_string(),
            drift_enabled: true,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_adr_dir(&mut self, dir: String) {
        self.adr_dir = dir;
    }

    #[wasm_bindgen(getter)]
    pub fn adr_dir(&self) -> String {
        self.adr_dir.clone()
    }
}

/// JavaScript-compatible drift report (simplified)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmDriftReport {
    /// Timestamp of the report
    pub timestamp: String,
    /// Directory that was scanned
    pub scanned_directory: String,
    /// Total number of drift items
    pub total_items: usize,
    /// Drift summary
    summary: String,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmDriftReport {
    /// Get report as JSON string
    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Get summary
    #[wasm_bindgen(getter)]
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
}

/// Simplified WASM API for ADRScan functionality (without async)
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct AdrscanWasm {
    config: Config,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl AdrscanWasm {
    /// Create a new ADRScan WASM instance
    #[wasm_bindgen(constructor)]
    pub fn new(config: &WasmConfig) -> Result<AdrscanWasm, JsValue> {
        let rust_config = convert_wasm_config(config)?;

        Ok(AdrscanWasm {
            config: rust_config,
        })
    }

    /// Initialize ADR directory structure (returns JSON list of files to create)
    #[wasm_bindgen]
    pub fn init(&self, directory: &str) -> Result<String, JsValue> {
        let init_files = vec![
            format!("{}/README.md", directory),
            format!("{}/.adrscan.yaml", directory),
            format!("{}/0001-record-architecture-decisions.md", directory),
        ];

        let init_content = HashMap::from([
            (format!("{}/README.md", directory), "# Architecture Decision Records\n\nThis directory contains Architecture Decision Records (ADRs) for this project.\n"),
            (format!("{}/.adrscan.yaml", directory), "adr_dir: .\ninclude_patterns:\n  - \"**/*.md\"\nexclude_patterns:\n  - \"**/node_modules/**\"\ntemplate:\n  format: madr\ndrift:\n  enabled: true\n"),
            (format!("{}/0001-record-architecture-decisions.md", directory), include_str!("../templates/madr.md")),
        ]);

        Ok(serde_json::to_string(&init_content)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
    }

    /// Parse ADR files and return inventory (simplified - requires file contents from host)
    #[wasm_bindgen]
    pub fn parse_adr(&self, content: &str, filename: &str) -> Result<String, JsValue> {
        let parser = AdrParser::new();
        match parser.parse_content(content, filename) {
            Ok(adr) => serde_json::to_string(&adr)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
            Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
        }
    }

    /// Simple drift detection (requires file list and contents from host)
    #[wasm_bindgen]
    pub fn detect_drift(&self, files_json: &str) -> Result<WasmDriftReport, JsValue> {
        // Parse the files JSON provided by the host
        let files: HashMap<String, String> = serde_json::from_str(files_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid files JSON: {}", e)))?;

        // Simple drift detection logic
        let mut drift_items = 0;
        let mut summary_items = Vec::new();

        for (path, content) in &files {
            // Basic technology detection
            if content.contains("mongodb") || content.contains("mongoose") {
                drift_items += 1;
                summary_items.push(format!("MongoDB usage detected in {}", path));
            }
            if content.contains("redis") {
                drift_items += 1;
                summary_items.push(format!("Redis usage detected in {}", path));
            }
            if content.contains("docker") && path.ends_with("Dockerfile") {
                drift_items += 1;
                summary_items.push(format!("Docker configuration in {}", path));
            }
            if content.contains("postgresql") || content.contains("pg") {
                drift_items += 1;
                summary_items.push(format!("PostgreSQL usage detected in {}", path));
            }
            if content.contains("kubernetes") || content.contains("kubectl") {
                drift_items += 1;
                summary_items.push(format!("Kubernetes configuration in {}", path));
            }
        }

        let report = WasmDriftReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            scanned_directory: ".".to_string(),
            total_items: drift_items,
            summary: summary_items.join("; "),
        };

        Ok(report)
    }

    /// Full diff functionality with baseline support (WASM compatible)
    #[wasm_bindgen]
    pub fn diff(
        &self,
        current_files_json: &str,
        baseline_json: Option<String>,
    ) -> Result<WasmDriftReport, JsValue> {
        // Parse current files
        let current_files: HashMap<String, String> = serde_json::from_str(current_files_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid current files JSON: {}", e)))?;

        // If baseline provided, compare against it
        if let Some(baseline_str) = baseline_json {
            let baseline_files: HashMap<String, String> = serde_json::from_str(&baseline_str)
                .map_err(|e| JsValue::from_str(&format!("Invalid baseline JSON: {}", e)))?;

            return self.diff_against_baseline(current_files, baseline_files);
        }

        // Otherwise, do standard drift detection
        self.detect_drift(current_files_json)
    }

    /// Compare current state against baseline
    fn diff_against_baseline(
        &self,
        current: HashMap<String, String>,
        baseline: HashMap<String, String>,
    ) -> Result<WasmDriftReport, JsValue> {
        let mut drift_items = 0;
        let mut summary_items = Vec::new();

        // Check for new files
        for (path, content) in &current {
            if !baseline.contains_key(path) {
                drift_items += 1;
                summary_items.push(format!("New file detected: {}", path));

                // Analyze content of new file
                if content.contains("mongodb") {
                    summary_items.push(format!("MongoDB usage in new file: {}", path));
                }
                if content.contains("redis") {
                    summary_items.push(format!("Redis usage in new file: {}", path));
                }
            }
        }

        // Check for modified files
        for (path, current_content) in &current {
            if let Some(baseline_content) = baseline.get(path) {
                if current_content != baseline_content {
                    drift_items += 1;
                    summary_items.push(format!("File modified: {}", path));

                    // Detect technology changes
                    let baseline_has_mongo = baseline_content.contains("mongodb");
                    let current_has_mongo = current_content.contains("mongodb");

                    if !baseline_has_mongo && current_has_mongo {
                        summary_items.push(format!("MongoDB added to: {}", path));
                    } else if baseline_has_mongo && !current_has_mongo {
                        summary_items.push(format!("MongoDB removed from: {}", path));
                    }
                }
            }
        }

        // Check for deleted files
        for path in baseline.keys() {
            if !current.contains_key(path) {
                drift_items += 1;
                summary_items.push(format!("File deleted: {}", path));
            }
        }

        let report = WasmDriftReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            scanned_directory: ".".to_string(),
            total_items: drift_items,
            summary: summary_items.join("; "),
        };

        Ok(report)
    }

    /// Inventory ADR files (requires file list and contents from host)
    #[wasm_bindgen]
    pub fn inventory(&self, files_json: &str) -> Result<String, JsValue> {
        // Parse the files JSON provided by the host
        let files: HashMap<String, String> = serde_json::from_str(files_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid files JSON: {}", e)))?;

        let parser = AdrParser::new();
        let mut adr_summaries = Vec::new();
        let mut total_size = 0u64;
        let mut total_lines = 0;
        let mut status_breakdown = HashMap::new();
        let mut tag_breakdown = HashMap::new();

        // Process each ADR file
        for (path, content) in &files {
            if path.ends_with(".md") {
                let file_size = content.len() as u64;
                let line_count = content.lines().count();
                total_size += file_size;
                total_lines += line_count;

                match parser.parse_content(content, path) {
                    Ok(adr) => {
                        // Extract metadata
                        let status = adr
                            .frontmatter
                            .get("status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let tags: Vec<String> = adr
                            .frontmatter
                            .get("tags")
                            .and_then(|v| v.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str())
                                    .map(|s| s.to_string())
                                    .collect()
                            })
                            .unwrap_or_default();

                        // Update breakdowns
                        *status_breakdown.entry(status.clone()).or_insert(0) += 1;
                        for tag in &tags {
                            *tag_breakdown.entry(tag.clone()).or_insert(0) += 1;
                        }

                        let summary = serde_json::json!({
                            "path": path,
                            "title": adr.frontmatter.get("title")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Untitled"),
                            "status": status,
                            "date": adr.frontmatter.get("date")
                                .and_then(|v| v.as_str()),
                            "tags": tags,
                            "file_size": file_size,
                            "line_count": line_count
                        });

                        adr_summaries.push(summary);
                    }
                    Err(_) => {
                        // Still include basic file info for unparseable files
                        let summary = serde_json::json!({
                            "path": path,
                            "title": "Parse Error",
                            "status": "error",
                            "file_size": file_size,
                            "line_count": line_count,
                            "tags": []
                        });
                        adr_summaries.push(summary);
                    }
                }
            }
        }

        // Build inventory response
        let inventory = serde_json::json!({
            "total_count": adr_summaries.len(),
            "status_breakdown": status_breakdown,
            "tag_breakdown": tag_breakdown,
            "adrs": adr_summaries,
            "statistics": {
                "total_files": adr_summaries.len(),
                "total_size_bytes": total_size,
                "total_lines": total_lines,
                "average_file_size": if adr_summaries.is_empty() { 0.0 } else { total_size as f64 / adr_summaries.len() as f64 },
                "average_lines_per_adr": if adr_summaries.is_empty() { 0.0 } else { total_lines as f64 / adr_summaries.len() as f64 }
            }
        });

        serde_json::to_string_pretty(&inventory)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Generate ADR proposals (simplified)
    #[wasm_bindgen]
    pub fn propose(&self, drift_report: &WasmDriftReport) -> Result<String, JsValue> {
        if drift_report.total_items == 0 {
            return Ok("[]".to_string());
        }

        // Generate simple proposals based on drift
        let mut proposals = Vec::new();
        let summary_parts: Vec<&str> = drift_report.summary.split("; ").collect();

        for (i, item) in summary_parts.iter().enumerate() {
            if !item.is_empty() {
                let adr_number = i + 1;
                let title = if item.contains("MongoDB") {
                    "Document MongoDB Usage Decision"
                } else if item.contains("Redis") {
                    "Document Redis Caching Strategy"
                } else if item.contains("Docker") {
                    "Document Containerization Approach"
                } else {
                    "Document Architectural Decision"
                };

                let proposal = HashMap::from([
                    ("number", format!("{:04}", adr_number)),
                    ("title", title.to_string()),
                    ("status", "proposed".to_string()),
                    ("context", format!("Detected: {}", item)),
                    (
                        "decision",
                        "Document the architectural decision for this technology choice."
                            .to_string(),
                    ),
                ]);

                proposals.push(proposal);
            }
        }

        Ok(serde_json::to_string(&proposals)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?)
    }

    /// Get current configuration
    #[wasm_bindgen]
    pub fn get_config(&self) -> Result<JsValue, JsValue> {
        let config_map = HashMap::from([
            ("adr_dir", self.config.adr_dir.to_string_lossy().to_string()),
            ("template_format", self.config.template.format.clone()),
            ("drift_enabled", self.config.drift.enabled.to_string()),
        ]);

        JsValue::from_serde(&config_map)
            .map_err(|e| JsValue::from_str(&format!("Config serialization error: {}", e)))
    }
}

/// Helper function to convert WASM config to Rust config
#[cfg(feature = "wasm")]
fn convert_wasm_config(wasm_config: &WasmConfig) -> Result<Config, JsValue> {
    Ok(Config {
        adr_dir: wasm_config.adr_dir.clone().into(),
        include_patterns: vec!["**/*.md".to_string()],
        exclude_patterns: vec!["**/node_modules/**".to_string()],
        snapshot_file: "./.adrscan_snapshot.json".into(),
        template: TemplateConfig {
            format: wasm_config.template_format.clone(),
            custom_path: None,
        },
        drift: DriftConfig {
            enabled: wasm_config.drift_enabled,
            detection_patterns: vec![], // Simplified for WASM
        },
    })
}

/// Utility functions for JavaScript interop
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmUtils;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmUtils {
    /// Parse ADR frontmatter from markdown content
    #[wasm_bindgen]
    pub fn parse_frontmatter(content: &str) -> Result<JsValue, JsValue> {
        let parser = AdrParser::new();
        match parser.parse_content(content, "test.md") {
            Ok(adr) => JsValue::from_serde(&adr)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
            Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
        }
    }

    /// Validate ADR template format
    #[wasm_bindgen]
    pub fn validate_template(template: &str) -> bool {
        template.contains("# ")
            && (template.contains("## Status") || template.contains("## Decision"))
    }

    /// Get default MADR template
    #[wasm_bindgen]
    pub fn get_default_template() -> String {
        include_str!("../templates/madr.md").to_string()
    }
}

// Export version information
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Export feature detection
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn features() -> JsValue {
    let features = vec![
        "parse_adr",
        "detect_drift",
        "propose",
        "frontmatter_parsing",
        "template_validation",
    ];

    JsValue::from_serde(&features).unwrap_or(JsValue::NULL)
}
