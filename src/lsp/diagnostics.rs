//! Real-time drift diagnostics for LSP integration
//! 
//! This module provides real-time analysis of ADR documents to detect
//! potential drift issues and surface them as LSP diagnostics.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use regex::Regex;
use chrono::{DateTime, Utc};

use crate::config::Config;
use crate::drift::detector::DriftDetector;
use crate::drift::patterns::DriftPattern;
use crate::parser;

/// Drift diagnostics engine for real-time analysis
pub struct DriftDiagnosticsEngine {
    /// Configuration reference
    config: Arc<RwLock<Option<Config>>>,
    /// Cache of diagnostic results
    diagnostic_cache: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
    /// Pattern matchers for common issues
    pattern_matchers: PatternMatchers,
    /// Performance metrics
    metrics: Arc<RwLock<DiagnosticMetrics>>,
}

impl DriftDiagnosticsEngine {
    /// Create a new diagnostics engine
    pub fn new(config: Arc<RwLock<Option<Config>>>) -> Self {
        Self {
            config,
            diagnostic_cache: Arc::new(RwLock::new(HashMap::new())),
            pattern_matchers: PatternMatchers::new(),
            metrics: Arc::new(RwLock::new(DiagnosticMetrics::default())),
        }
    }

    /// Analyze a document for drift issues and return diagnostics
    pub async fn analyze_document(&self, uri: &Url, text: &str) -> Option<Vec<Diagnostic>> {
        let start_time = std::time::Instant::now();
        let mut diagnostics = Vec::new();

        // Check if this is an ADR document
        if !self.is_adr_document(uri, text) {
            return None;
        }

        // Parse the document
        if let Ok(adr) = parser::parse_adr_content(text, uri.to_file_path().ok()?) {
            // Run various diagnostic checks
            diagnostics.extend(self.check_adr_structure(&adr, text).await);
            diagnostics.extend(self.check_drift_patterns(&adr, text).await);
            diagnostics.extend(self.check_content_quality(&adr, text).await);
            diagnostics.extend(self.check_references(&adr, text).await);
            diagnostics.extend(self.check_metadata(&adr, text).await);
        } else {
            // Add parsing error diagnostic
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: text.lines().next().unwrap_or("").len() as u32 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("parse-error".to_string())),
                source: Some("photon-drift".to_string()),
                message: "Failed to parse ADR document. Please check the format.".to_string(),
                related_information: None,
                tags: None,
                data: None,
                code_description: None,
            });
        }

        // Update metrics
        let elapsed = start_time.elapsed();
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_analyses += 1;
            metrics.total_time += elapsed;
            metrics.average_time = metrics.total_time / metrics.total_analyses;
        }

        // Cache results
        self.diagnostic_cache.write().await.insert(uri.clone(), diagnostics.clone());

        Some(diagnostics)
    }

    /// Check if a document is an ADR based on URI and content
    fn is_adr_document(&self, uri: &Url, text: &str) -> bool {
        // Check file extension
        if let Some(path) = uri.to_file_path().ok() {
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                return false;
            }
        }

        // Check content for ADR markers
        text.contains("# ADR") || 
        text.contains("# Architecture Decision Record") ||
        text.contains("## Status") ||
        text.contains("## Context") ||
        text.contains("## Decision")
    }

    /// Check ADR structural requirements
    async fn check_adr_structure(&self, adr: &parser::Adr, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        // Check required sections
        let required_sections = vec!["Status", "Context", "Decision"];
        
        for section in required_sections {
            if !adr.content.contains(&format!("## {}", section)) &&
               !adr.content.contains(&format!("# {}", section)) {
                
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: lines.get(0).unwrap_or(&"").len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("missing-section".to_string())),
                    source: Some("photon-drift".to_string()),
                    message: format!("Missing required section: '{}'", section),
                    related_information: None,
                    tags: None,
                    data: None,
                    code_description: None,
                });
            }
        }

        // Check for title
        if adr.title.is_empty() {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: lines.get(0).unwrap_or(&"").len() as u32 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing-title".to_string())),
                source: Some("photon-drift".to_string()),
                message: "ADR must have a title".to_string(),
                related_information: None,
                tags: None,
                data: None,
                code_description: None,
            });
        }

        diagnostics
    }

    /// Check for drift patterns in the ADR
    async fn check_drift_patterns(&self, adr: &parser::Adr, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        // Check for outdated technology references
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(tech_issue) = self.pattern_matchers.check_outdated_technology(line) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(NumberOrString::String("outdated-tech".to_string())),
                    source: Some("photon-drift".to_string()),
                    message: tech_issue,
                    related_information: None,
                    tags: Some(vec![DiagnosticTag::DEPRECATED]),
                    data: None,
                    code_description: None,
                });
            }

            // Check for broken links
            if let Some(link_issue) = self.pattern_matchers.check_broken_links(line) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: line_num as u32, character: 0 },
                        end: Position { line: line_num as u32, character: line.len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("broken-link".to_string())),
                    source: Some("photon-drift".to_string()),
                    message: link_issue,
                    related_information: None,
                    tags: None,
                    data: None,
                    code_description: None,
                });
            }
        }

        diagnostics
    }

    /// Check content quality issues
    async fn check_content_quality(&self, adr: &parser::Adr, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        // Check for very short sections
        let sections = self.extract_sections(text);
        for (section_name, content) in sections {
            if content.trim().split_whitespace().count() < 10 {
                // Find the line number for this section
                if let Some(line_num) = self.find_section_line(&lines, &section_name) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num, character: 0 },
                            end: Position { line: line_num, character: lines.get(line_num as usize).unwrap_or(&"").len() as u32 },
                        },
                        severity: Some(DiagnosticSeverity::INFORMATION),
                        code: Some(NumberOrString::String("thin-content".to_string())),
                        source: Some("photon-drift".to_string()),
                        message: format!("Section '{}' appears to have minimal content (consider adding more details)", section_name),
                        related_information: None,
                        tags: None,
                        data: None,
                        code_description: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// Check ADR references and links
    async fn check_references(&self, adr: &parser::Adr, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        // Check for ADR reference format
        let adr_ref_regex = Regex::new(r"ADR-?(\d+)").unwrap();
        
        for (line_num, line) in lines.iter().enumerate() {
            for cap in adr_ref_regex.captures_iter(line) {
                let adr_num = cap.get(1).unwrap().as_str();
                
                // This is a placeholder - in a real implementation, you'd check if the referenced ADR exists
                if adr_num.parse::<i32>().unwrap_or(0) > 100 {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num as u32, character: cap.get(0).unwrap().start() as u32 },
                            end: Position { line: line_num as u32, character: cap.get(0).unwrap().end() as u32 },
                        },
                        severity: Some(DiagnosticSeverity::INFORMATION),
                        code: Some(NumberOrString::String("high-adr-reference".to_string())),
                        source: Some("photon-drift".to_string()),
                        message: format!("High ADR reference number ({}). Verify this ADR exists.", adr_num),
                        related_information: None,
                        tags: None,
                        data: None,
                        code_description: None,
                    });
                }
            }
        }

        diagnostics
    }

    /// Check metadata and frontmatter
    async fn check_metadata(&self, adr: &parser::Adr, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Check if date is in the future
        if let Some(created_date) = &adr.created_date {
            if *created_date > Utc::now() {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 2, character: 0 },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("future-date".to_string())),
                    source: Some("photon-drift".to_string()),
                    message: "ADR creation date is in the future".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                    code_description: None,
                });
            }
        }

        diagnostics
    }

    /// Extract sections from text
    fn extract_sections(&self, text: &str) -> Vec<(String, String)> {
        let mut sections = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut current_section = None;
        let mut current_content = String::new();

        for line in lines {
            if line.starts_with("## ") || line.starts_with("# ") {
                if let Some(section_name) = current_section {
                    sections.push((section_name, current_content.trim().to_string()));
                }
                current_section = Some(line.trim_start_matches("## ").trim_start_matches("# ").to_string());
                current_content.clear();
            } else {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }

        if let Some(section_name) = current_section {
            sections.push((section_name, current_content.trim().to_string()));
        }

        sections
    }

    /// Find the line number for a section header
    fn find_section_line(&self, lines: &[&str], section_name: &str) -> Option<u32> {
        for (i, line) in lines.iter().enumerate() {
            if line.contains(&format!("## {}", section_name)) || line.contains(&format!("# {}", section_name)) {
                return Some(i as u32);
            }
        }
        None
    }
}

/// Pattern matchers for common drift issues
struct PatternMatchers {
    outdated_tech_regex: Regex,
    link_regex: Regex,
}

impl PatternMatchers {
    fn new() -> Self {
        Self {
            outdated_tech_regex: Regex::new(r"\b(jQuery|AngularJS|IE|Internet Explorer|Flash|Java 8|Python 2)\b").unwrap(),
            link_regex: Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap(),
        }
    }

    fn check_outdated_technology(&self, line: &str) -> Option<String> {
        if let Some(cap) = self.outdated_tech_regex.find(line) {
            return Some(format!("Reference to potentially outdated technology: {}", cap.as_str()));
        }
        None
    }

    fn check_broken_links(&self, line: &str) -> Option<String> {
        for cap in self.link_regex.captures_iter(line) {
            let link_url = cap.get(2).unwrap().as_str();
            
            // Simple heuristic for potentially broken links
            if link_url.starts_with("http://") && !link_url.contains("localhost") {
                return Some(format!("Consider using HTTPS for external link: {}", link_url));
            }
            
            // Check for common broken link patterns
            if link_url.contains("example.com") || link_url.contains("TODO") {
                return Some(format!("Placeholder link detected: {}", link_url));
            }
        }
        None
    }
}

/// Diagnostic performance metrics
#[derive(Default)]
struct DiagnosticMetrics {
    total_analyses: u32,
    total_time: std::time::Duration,
    average_time: std::time::Duration,
}