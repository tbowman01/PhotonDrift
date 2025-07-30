//! Common traits and types for ML models

use super::super::detector::Prediction;
use super::super::features::DriftFeatures;
use crate::drift::DriftResult;

/// Supported ML model types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModelType {
    /// Isolation Forest for anomaly detection
    IsolationForest,

    /// One-Class SVM for outlier detection
    OneClassSVM,

    /// Local Outlier Factor
    LocalOutlierFactor,

    /// Simple statistical model (mean + std dev)
    Statistical,

    /// Ensemble of multiple models
    Ensemble,
}

/// Trait for anomaly detection models
pub trait AnomalyModel: Send + Sync {
    /// Predict if features represent an anomaly
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction>;

    /// Provide explanation for the prediction (if supported)
    fn explain(&self, features: &DriftFeatures) -> Option<String>;

    /// Serialize model for saving
    fn serialize(&self) -> DriftResult<Vec<u8>>;

    /// Get model type
    fn model_type(&self) -> ModelType;
}