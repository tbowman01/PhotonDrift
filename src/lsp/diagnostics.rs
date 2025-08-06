//! Real-time drift diagnostics for ADR files

#![cfg(feature = "lsp")]

use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{config::Config, drift::DriftDetector, Result};

#[cfg(feature = "ml")]
use crate::ml::{MLConfig, MLDriftDetector};

/// Engine for creating LSP diagnostics from drift detection
pub struct DiagnosticEngine {
    detector: DriftDetector,
    #[cfg(feature = "ml")]
    ml_detector: Option<Arc<Mutex<MLDriftDetector>>>,
    config: Config,
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            detector: DriftDetector::new(),
            #[cfg(feature = "ml")]
            ml_detector: None,
            config: Config::default(),
        };

        // Initialize ML detector if available
        #[cfg(feature = "ml")]
        {
            let ml_config = MLConfig {
                enabled: true,
                model_type: crate::ml::ModelType::Ensemble,
                confidence_threshold: 0.7,
                model_path: None,
                cache_ttl_seconds: 3600,
                max_features: 50,
            };

            match MLDriftDetector::new(ml_config) {
                Ok(ml_detector) => {
                    engine.ml_detector = Some(Arc::new(Mutex::new(ml_detector)));
                    log::info!("ML-enhanced diagnostics enabled");
                }
                Err(e) => {
                    log::warn!(
                        "Failed to initialize ML detector, using basic detection: {}",
                        e
                    );
                }
            }
        }

        engine
    }

    /// Analyze content and return LSP diagnostics
    pub async fn analyze_content(&self, content: &str, uri: &Url) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        // Basic structural validation (always run - fast)
        diagnostics.extend(self.validate_adr_structure(content));
        diagnostics.extend(self.check_adr_content_quality(content));

        // Advanced drift detection with ML enhancement
        if let Ok(path) = uri.to_file_path() {
            if let Ok(drift_items) = self.detector.analyze_single_file(&path) {
                // Use ML enhancement if available
                #[cfg(feature = "ml")]
                {
                    if let Some(ref ml_detector) = self.ml_detector {
                        match self
                            .enhance_with_ml(drift_items.items, content, ml_detector)
                            .await
                        {
                            Ok(ml_diagnostics) => {
                                diagnostics.extend(ml_diagnostics);
                            }
                            Err(e) => {
                                log::warn!("ML enhancement failed, using basic detection: {}", e);
                                // Fallback to basic drift detection
                                for item in drift_items.items {
                                    diagnostics.push(create_drift_diagnostic(&item, content));
                                }
                            }
                        }
                    } else {
                        // No ML available, use basic detection
                        for item in drift_items.items {
                            diagnostics.push(create_drift_diagnostic(&item, content));
                        }
                    }
                }

                #[cfg(not(feature = "ml"))]
                {
                    // ML feature not enabled, use basic detection
                    for item in drift_items.items {
                        diagnostics.push(create_drift_diagnostic(&item, content));
                    }
                }
            }
        }

        Ok(diagnostics)
    }

    /// Enhance drift detection with ML analysis (async, non-blocking)
    #[cfg(feature = "ml")]
    async fn enhance_with_ml(
        &self,
        drift_items: Vec<crate::drift::DriftItem>,
        content: &str,
        ml_detector: &Arc<Mutex<MLDriftDetector>>,
    ) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        // Create a timeout for ML processing to prevent LSP blocking
        let ml_future = tokio::time::timeout(
            tokio::time::Duration::from_millis(500), // 500ms max for real-time responsiveness
            async {
                let mut detector = ml_detector.lock().await;

                for item in drift_items {
                    // Extract features for ML analysis
                    let features = self.extract_lsp_features(content, &item).await?;

                    // Get ML prediction with confidence scoring
                    match detector.predict_anomaly(&features).await {
                        Ok(ml_result) => {
                            // Only report if confidence is above threshold
                            if ml_result.confidence >= 0.7 {
                                diagnostics.push(create_ml_enhanced_diagnostic(
                                    &item, &ml_result, content,
                                ));
                            }
                        }
                        Err(_) => {
                            // ML failed for this item, use basic detection
                            diagnostics.push(create_drift_diagnostic(&item, content));
                        }
                    }
                }

                Ok::<Vec<Diagnostic>, crate::error::AdrscanError>(diagnostics)
            },
        );

        match ml_future.await {
            Ok(Ok(ml_diagnostics)) => Ok(ml_diagnostics),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                log::warn!("ML analysis timed out, falling back to basic detection");
                // Timeout occurred, return basic diagnostics
                let mut basic_diagnostics = Vec::new();
                for item in drift_items {
                    basic_diagnostics.push(create_drift_diagnostic(&item, content));
                }
                Ok(basic_diagnostics)
            }
        }
    }

    /// Extract ML features specifically for LSP context
    #[cfg(feature = "ml")]
    async fn extract_lsp_features(
        &self,
        content: &str,
        drift_item: &crate::drift::DriftItem,
    ) -> Result<crate::ml::DriftFeatures> {
        // This would use the existing feature extractor but focus on LSP-relevant features
        let extractor = crate::ml::FeatureExtractor::new();

        // Create a minimal file representation for feature extraction
        let temp_file_data = crate::drift::FileData {
            path: std::path::PathBuf::from(&drift_item.file_path),
            content: content.to_string(),
            size: content.len() as u64,
            modified: std::time::SystemTime::now(),
        };

        extractor.extract_features(&[temp_file_data]).await
    }

    /// Validate basic ADR structure
    fn validate_adr_structure(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Check for title
        if !lines.iter().any(|line| line.starts_with("# ")) {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(lsp_types::NumberOrString::String(
                    "missing-title".to_string(),
                )),
                source: Some("photondrift".to_string()),
                message: "ADR should have a title starting with '# ADR-XXX:'".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }

        // Check for status section
        if !content.to_lowercase().contains("## status") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(lsp_types::NumberOrString::String(
                    "missing-status".to_string(),
                )),
                source: Some("photondrift".to_string()),
                message: "ADR should include a '## Status' section".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }

        // Check for context section
        if !content.to_lowercase().contains("## context") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
                severity: Some(DiagnosticSeverity::INFO),
                code: Some(lsp_types::NumberOrString::String(
                    "missing-context".to_string(),
                )),
                source: Some("photondrift".to_string()),
                message: "Consider adding a '## Context' section".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }

        // Check for decision section
        if !content.to_lowercase().contains("## decision") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(lsp_types::NumberOrString::String(
                    "missing-decision".to_string(),
                )),
                source: Some("photondrift".to_string()),
                message: "ADR should include a '## Decision' section".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }

        diagnostics
    }

    /// Check for content quality issues
    fn check_adr_content_quality(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Check for empty sections
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("## ") {
                // Check if next non-empty line is another section
                let mut next_line_idx = i + 1;
                while next_line_idx < lines.len() && lines[next_line_idx].trim().is_empty() {
                    next_line_idx += 1;
                }

                if next_line_idx < lines.len() && lines[next_line_idx].starts_with("## ") {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: i as u32,
                                character: 0,
                            },
                            end: Position {
                                line: i as u32,
                                character: line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(lsp_types::NumberOrString::String(
                            "empty-section".to_string(),
                        )),
                        source: Some("photondrift".to_string()),
                        message: format!("Section '{}' appears to be empty", line.trim()),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
        }

        // Check for broken links
        for (i, line) in lines.iter().enumerate() {
            if line.contains("](") {
                // Simple check for markdown links - could be enhanced
                if line.contains("](http") && (line.contains("404") || line.contains("broken")) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: i as u32,
                                character: 0,
                            },
                            end: Position {
                                line: i as u32,
                                character: line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(lsp_types::NumberOrString::String("broken-link".to_string())),
                        source: Some("photondrift".to_string()),
                        message: "Potentially broken link detected".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
        }

        // Check for outdated technology references
        let outdated_tech = ["jQuery", "AngularJS", "Internet Explorer", "Flash"];
        for (i, line) in lines.iter().enumerate() {
            for tech in &outdated_tech {
                if line.to_lowercase().contains(&tech.to_lowercase()) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: i as u32,
                                character: 0,
                            },
                            end: Position {
                                line: i as u32,
                                character: line.len() as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::HINT),
                        code: Some(lsp_types::NumberOrString::String(
                            "outdated-tech".to_string(),
                        )),
                        source: Some("photondrift".to_string()),
                        message: format!("Reference to potentially outdated technology: {}", tech),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
        }

        diagnostics
    }
}

/// Create a diagnostic from a drift detection item
pub fn create_drift_diagnostic(drift_item: &crate::drift::DriftItem, content: &str) -> Diagnostic {
    // Find the line number for this drift item (simple heuristic)
    let line_number = content
        .lines()
        .position(|line| line.contains(&drift_item.pattern))
        .unwrap_or(0) as u32;

    let severity = match drift_item.severity {
        crate::drift::DriftSeverity::High => DiagnosticSeverity::ERROR,
        crate::drift::DriftSeverity::Medium => DiagnosticSeverity::WARNING,
        crate::drift::DriftSeverity::Low => DiagnosticSeverity::HINT,
    };

    Diagnostic {
        range: Range {
            start: Position {
                line: line_number,
                character: 0,
            },
            end: Position {
                line: line_number,
                character: content
                    .lines()
                    .nth(line_number as usize)
                    .map(|l| l.len())
                    .unwrap_or(0) as u32,
            },
        },
        severity: Some(severity),
        code: Some(lsp_types::NumberOrString::String(
            "drift-detected".to_string(),
        )),
        source: Some("photondrift".to_string()),
        message: format!("{}: {}", drift_item.summary, drift_item.description),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// Create ML-enhanced diagnostic with confidence scoring
#[cfg(feature = "ml")]
pub fn create_ml_enhanced_diagnostic(
    drift_item: &crate::drift::DriftItem,
    ml_result: &crate::ml::detector::Prediction,
    content: &str,
) -> Diagnostic {
    // Find the line number for this drift item
    let line_number = content
        .lines()
        .position(|line| line.contains(&drift_item.pattern))
        .unwrap_or(0) as u32;

    // Use ML confidence to determine severity
    let severity = if ml_result.confidence >= 0.9 {
        DiagnosticSeverity::ERROR
    } else if ml_result.confidence >= 0.7 {
        DiagnosticSeverity::WARNING
    } else {
        DiagnosticSeverity::HINT
    };

    // Enhanced message with ML confidence
    let message = format!(
        "{}: {} (ML Confidence: {:.1}%)",
        drift_item.summary,
        drift_item.description,
        ml_result.confidence * 100.0
    );

    Diagnostic {
        range: Range {
            start: Position {
                line: line_number,
                character: 0,
            },
            end: Position {
                line: line_number,
                character: content
                    .lines()
                    .nth(line_number as usize)
                    .map(|l| l.len())
                    .unwrap_or(0) as u32,
            },
        },
        severity: Some(severity),
        code: Some(lsp_types::NumberOrString::String(
            "ml-drift-detected".to_string(),
        )),
        source: Some("photondrift-ml".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adr_structure_validation() {
        let engine = DiagnosticEngine::new();

        // Test content missing title
        let content = "This is not a proper ADR";
        let uri = Url::parse("file:///test.md").unwrap();
        let diagnostics = engine.analyze_content(content, &uri).await.unwrap();

        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| d.code
            == Some(lsp_types::NumberOrString::String(
                "missing-title".to_string()
            ))));
    }

    #[tokio::test]
    async fn test_proper_adr_structure() {
        let engine = DiagnosticEngine::new();

        let content = r#"# ADR-001: Use Rust for Backend

## Status
Accepted

## Context
We need a performant backend.

## Decision
We will use Rust.

## Consequences
Better performance.
"#;

        let uri = Url::parse("file:///test.md").unwrap();
        let diagnostics = engine.analyze_content(content, &uri).await.unwrap();

        // Should have fewer warnings for proper ADR
        let warnings = diagnostics
            .iter()
            .filter(|d| matches!(d.severity, Some(DiagnosticSeverity::WARNING)))
            .count();

        assert!(warnings <= 1); // May have some minor suggestions
    }

    #[test]
    fn test_empty_section_detection() {
        let engine = DiagnosticEngine::new();
        let content = r#"# ADR-001: Test

## Status

## Context
Some context here
"#;

        let diagnostics = engine.check_adr_content_quality(content);
        assert!(diagnostics.iter().any(|d| {
            d.code
                == Some(lsp_types::NumberOrString::String(
                    "empty-section".to_string(),
                ))
        }));
    }
}
