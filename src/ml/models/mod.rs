//! Machine learning models for anomaly detection
//!
//! Provides different ML algorithms for detecting architectural drift
//! with support for multiple model types and explainable AI.
//!
//! This module has been refactored into smaller, security-auditable components:
//! - `core`: Core traits and factory pattern
//! - `isolation_forest`: Isolation Forest implementation  
//! - `svm`: One-Class SVM implementation
//! - `statistical`: Statistical anomaly detection
//! - `ensemble`: Ensemble methods

use crate::drift::DriftResult;
// use super::detector::Prediction; // Re-exported from core
// use super::features::DriftFeatures; // Re-exported from core

pub mod core;
pub mod ensemble;
pub mod factory;
pub mod isolation_forest;
pub mod statistical;
pub mod svm;

// Re-export public API
pub use core::{AnomalyModel, ModelType};
pub use ensemble::EnsembleModel;
pub use factory::ModelFactory;
pub use isolation_forest::IsolationForest;
pub use statistical::StatisticalModel;
pub use svm::OneClassSVM;

// Backwards compatibility aliases
pub use isolation_forest::IsolationForest as IsolationForestModel;
pub use svm::OneClassSVM as OneClassSVMModel;

/// Convenience function to create models
pub fn create_model(model_type: ModelType) -> DriftResult<Box<dyn AnomalyModel>> {
    ModelFactory::create(model_type)
}
