//! Ensemble model combining multiple anomaly detection algorithms

use crate::drift::DriftResult;
use super::core::{AnomalyModel, ModelType};
use super::super::detector::Prediction;
use super::super::features::DriftFeatures;
use super::{IsolationForest, OneClassSVM, StatisticalModel};

/// Ensemble model that combines multiple anomaly detection models
pub struct EnsembleModel {
    models: Vec<Box<dyn AnomalyModel>>,
    weights: Vec<f64>,
    voting_strategy: VotingStrategy,
}

#[derive(Debug, Clone, Copy)]
pub enum VotingStrategy {
    /// Simple majority voting
    Majority,
    /// Weighted average of scores
    WeightedAverage,
    /// Conservative (all models must agree)
    Conservative,
}

impl EnsembleModel {
    /// Create a new ensemble model
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            weights: Vec::new(),
            voting_strategy: VotingStrategy::WeightedAverage,
        }
    }

    /// Create ensemble with default models
    pub fn with_default_models() -> Self {
        let mut ensemble = Self::new();
        
        // Add default models with equal weights
        ensemble.add_model(Box::new(IsolationForest::new()), 1.0);
        ensemble.add_model(Box::new(OneClassSVM::new()), 1.0);
        ensemble.add_model(Box::new(StatisticalModel::new()), 1.0);
        
        ensemble
    }

    /// Add a model to the ensemble
    pub fn add_model(&mut self, model: Box<dyn AnomalyModel>, weight: f64) {
        self.models.push(model);
        self.weights.push(weight);
    }

    /// Set voting strategy
    pub fn set_voting_strategy(&mut self, strategy: VotingStrategy) {
        self.voting_strategy = strategy;
    }

    /// Train all models in the ensemble
    pub fn fit(&mut self, _data: &[DriftFeatures]) -> DriftResult<()> {
        // For now, individual models handle their own training
        // In a full implementation, we would train each model here
        Ok(())
    }

    fn ensemble_prediction(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        if self.models.is_empty() {
            return Ok(Prediction {
                is_anomaly: false,
                confidence: 0.0,
                anomaly_score: 0.0,
                explanation: Some("No models in ensemble".to_string()),
            });
        }

        // Get predictions from all models
        let predictions: Result<Vec<Prediction>, _> = self.models
            .iter()
            .map(|model| model.predict(features))
            .collect();
            
        let predictions = predictions?;

        match self.voting_strategy {
            VotingStrategy::Majority => self.majority_vote(&predictions),
            VotingStrategy::WeightedAverage => self.weighted_average(&predictions),
            VotingStrategy::Conservative => self.conservative_vote(&predictions),
        }
    }

    fn majority_vote(&self, predictions: &[Prediction]) -> DriftResult<Prediction> {
        let anomaly_votes = predictions.iter()
            .filter(|p| p.is_anomaly)
            .count();
            
        let is_anomaly = anomaly_votes > predictions.len() / 2;
        let confidence = anomaly_votes as f64 / predictions.len() as f64;
        
        Ok(Prediction {
            is_anomaly,
            confidence,
            anomaly_score: predictions.iter().map(|p| p.anomaly_score).sum::<f64>() / predictions.len() as f64,
            explanation: Some(format!(
                "Ensemble Majority Vote: {}/{} models detected anomaly",
                anomaly_votes, predictions.len()
            )),
        })
    }

    fn weighted_average(&self, predictions: &[Prediction]) -> DriftResult<Prediction> {
        let total_weight: f64 = self.weights.iter().take(predictions.len()).sum();
        
        if total_weight == 0.0 {
            return Ok(Prediction {
                is_anomaly: false,
                confidence: 0.0,
                anomaly_score: 0.0,
                explanation: Some("Zero total weight in ensemble".to_string()),
            });
        }

        let weighted_score: f64 = predictions.iter()
            .zip(self.weights.iter())
            .map(|(pred, weight)| pred.confidence * weight)
            .sum();
            
        let average_score = weighted_score / total_weight;
        let is_anomaly = average_score > 0.5; // Threshold can be tuned
        
        Ok(Prediction {
            is_anomaly,
            confidence: average_score,
            anomaly_score: average_score,
            explanation: Some(format!(
                "Ensemble Weighted Average: score={:.3}",
                average_score
            )),
        })
    }

    fn conservative_vote(&self, predictions: &[Prediction]) -> DriftResult<Prediction> {
        let all_anomaly = predictions.iter().all(|p| p.is_anomaly);
        let min_confidence = predictions.iter()
            .map(|p| p.confidence)
            .fold(f64::INFINITY, f64::min);
            
        Ok(Prediction {
            is_anomaly: all_anomaly,
            anomaly_score: if all_anomaly { min_confidence } else { 1.0 - min_confidence },
            confidence: if all_anomaly { min_confidence } else { 0.0 },
            explanation: Some(format!(
                "Ensemble Conservative: all {} models must agree",
                predictions.len()
            )),
        })
    }
}

impl AnomalyModel for EnsembleModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        self.ensemble_prediction(features)
    }

    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        match self.ensemble_prediction(features) {
            Ok(prediction) => prediction.explanation,
            Err(_) => Some("Ensemble prediction failed".to_string()),
        }
    }

    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(format!("EnsembleModel:{}", self.models.len()).into_bytes())
    }

    fn model_type(&self) -> ModelType {
        ModelType::Ensemble
    }
}

impl Default for EnsembleModel {
    fn default() -> Self {
        Self::with_default_models()
    }
}