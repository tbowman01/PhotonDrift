//! Machine Learning Enhanced Drift Detection
//!
//! This module implements ML-based anomaly detection for more intelligent
//! architectural drift identification with reduced false positives.

pub mod detector;
pub mod features;
pub mod models;
pub mod training;

pub use detector::MLDriftDetector;
pub use features::{DriftFeatures, FeatureExtractor};
pub use models::{AnomalyModel, ModelType};
pub use training::{ModelTrainer, TrainingData};

use crate::drift::DriftResult;

/// Configuration for ML-enhanced drift detection
#[derive(Debug, Clone)]
pub struct MLConfig {
    /// Enable ML-enhanced detection
    pub enabled: bool,

    /// Model type to use for detection
    pub model_type: ModelType,

    /// Confidence threshold for anomaly detection (0.0-1.0)
    pub confidence_threshold: f64,

    /// Path to trained model file
    pub model_path: Option<std::path::PathBuf>,

    /// Enable online learning from feedback
    pub online_learning: bool,

    /// Maximum training samples to keep in memory
    pub max_training_samples: usize,
}

impl Default for MLConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for backward compatibility
            model_type: ModelType::IsolationForest,
            confidence_threshold: 0.7,
            model_path: None,
            online_learning: false,
            max_training_samples: 10000,
        }
    }
}

/// ML-enhanced drift detection result
#[derive(Debug, Clone)]
pub struct MLDriftResult {
    /// Original drift detection result
    pub base_result: crate::drift::DriftItem,

    /// ML confidence score (0.0-1.0)
    pub confidence: f64,

    /// Anomaly score from ML model
    pub anomaly_score: f64,

    /// Features used for ML detection
    pub features: DriftFeatures,

    /// Whether this was flagged as anomalous
    pub is_anomaly: bool,

    /// Model explanation (if available)
    pub explanation: Option<String>,
}

impl MLDriftResult {
    /// Create a new ML drift result
    pub fn new(
        base_result: crate::drift::DriftItem,
        confidence: f64,
        anomaly_score: f64,
        features: DriftFeatures,
    ) -> Self {
        let is_anomaly = anomaly_score > 0.5; // Default threshold

        Self {
            base_result,
            confidence,
            anomaly_score,
            features,
            is_anomaly,
            explanation: None,
        }
    }

    /// Add explanation to the result
    pub fn with_explanation(mut self, explanation: String) -> Self {
        self.explanation = Some(explanation);
        self
    }

    /// Check if this result should be reported based on confidence
    pub fn should_report(&self, threshold: f64) -> bool {
        self.confidence >= threshold && self.is_anomaly
    }
}

/// Initialize ML subsystem
pub fn initialize() -> DriftResult<()> {
    log::info!("Initializing ML-enhanced drift detection");

    // Check if ML dependencies are available
    #[cfg(not(feature = "ml"))]
    {
        log::warn!("ML features not compiled in this build");
    }

    #[cfg(feature = "ml")]
    {
        log::info!("ML features available");
        // Initialize ML backend here
    }

    Ok(())
}

/// Check if ML features are available
pub fn is_available() -> bool {
    #[cfg(feature = "ml")]
    return true;

    #[cfg(not(feature = "ml"))]
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_config_default() {
        let config = MLConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.confidence_threshold, 0.7);
        assert!(matches!(config.model_type, ModelType::IsolationForest));
    }

    #[test]
    fn test_ml_drift_result_creation() {
        use crate::drift::{DriftCategory, DriftItem, DriftLocation, DriftSeverity};
        use std::path::PathBuf;

        let base_item = DriftItem::new(
            "test".to_string(),
            DriftSeverity::Medium,
            DriftCategory::NewTechnology,
            "Test".to_string(),
            "Test description".to_string(),
            DriftLocation::new(PathBuf::from("test.rs")),
        );

        let features = DriftFeatures::default();
        let result = MLDriftResult::new(base_item, 0.8, 0.6, features);

        assert_eq!(result.confidence, 0.8);
        assert_eq!(result.anomaly_score, 0.6);
        assert!(result.is_anomaly);
        assert!(result.should_report(0.7));
    }

    #[test]
    fn test_ml_availability() {
        // This will return false in most builds since ml feature is not enabled by default
        let available = is_available();

        #[cfg(feature = "ml")]
        assert!(available);

        #[cfg(not(feature = "ml"))]
        assert!(!available);
    }
}
