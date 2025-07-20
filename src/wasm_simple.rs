//! Simplified WebAssembly Module for ADRScan
//! 
//! This module provides a WASM-compatible version without async/tokio dependencies

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "wasm")]
use std::collections::HashMap;

#[cfg(feature = "wasm")]
use crate::{
    config::{Config, TemplateConfig, DriftConfig},
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
            Ok(adr) => {
                serde_json::to_string(&adr)
                    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
            }
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
        }
        
        let report = WasmDriftReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            scanned_directory: ".".to_string(),
            total_items: drift_items,
            summary: summary_items.join("; "),
        };
        
        Ok(report)
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
                    ("decision", "Document the architectural decision for this technology choice.".to_string()),
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
        template.contains("# ") && 
        (template.contains("## Status") || template.contains("## Decision"))
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