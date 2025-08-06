//! Factory for creating ML models

use super::core::{AnomalyModel, ModelType};
use super::{EnsembleModel, IsolationForest, OneClassSVM, StatisticalModel};
use crate::drift::DriftResult;

/// Factory for creating ML models
pub struct ModelFactory;

impl ModelFactory {
    /// Create a new model of the specified type
    pub fn create(model_type: ModelType) -> DriftResult<Box<dyn AnomalyModel>> {
        match model_type {
            ModelType::IsolationForest => Ok(Box::new(IsolationForest::new())),
            ModelType::OneClassSVM => Ok(Box::new(OneClassSVM::new())),
            ModelType::Statistical => Ok(Box::new(StatisticalModel::new())),
            ModelType::Ensemble => Ok(Box::new(EnsembleModel::new())),
            ModelType::LocalOutlierFactor => {
                // TODO: Implement LOF model
                Err(crate::AdrscanError::DriftError(
                    "LocalOutlierFactor not implemented yet".to_string(),
                ))
            }
        }
    }

    /// List available model types
    pub fn available_models() -> Vec<ModelType> {
        vec![
            ModelType::IsolationForest,
            ModelType::OneClassSVM,
            ModelType::Statistical,
            ModelType::Ensemble,
        ]
    }
}
