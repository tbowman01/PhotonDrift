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
use super::detector::Prediction;
use super::features::DriftFeatures;

pub mod core;
pub mod isolation_forest;
pub mod svm;
pub mod statistical;
pub mod ensemble;
pub mod factory;

// Re-export public API
pub use core::{AnomalyModel, ModelType};
pub use factory::ModelFactory;
pub use isolation_forest::IsolationForest;
pub use svm::OneClassSVM;
pub use statistical::StatisticalModel;
pub use ensemble::EnsembleModel;

/// Convenience function to create models
pub fn create_model(model_type: ModelType) -> DriftResult<Box<dyn AnomalyModel>> {
    ModelFactory::create(model_type)
}