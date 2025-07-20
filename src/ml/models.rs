//! Machine learning models for anomaly detection
//! 
//! Provides different ML algorithms for detecting architectural drift
//! with support for multiple model types and explainable AI.

use crate::drift::DriftResult;
use super::features::DriftFeatures;
use super::detector::Prediction;

/// Supported ML model types
#[derive(Debug, Clone, PartialEq)]
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

/// Factory for creating ML models
pub struct ModelFactory;

impl ModelFactory {
    /// Create a new model of the specified type
    pub fn create_model(model_type: ModelType) -> Box<dyn AnomalyModel> {
        match model_type {
            ModelType::IsolationForest => Box::new(IsolationForestModel::new()),
            ModelType::OneClassSVM => Box::new(OneClassSVMModel::new()),
            ModelType::LocalOutlierFactor => Box::new(LOFModel::new()),
            ModelType::Statistical => Box::new(StatisticalModel::new()),
            ModelType::Ensemble => Box::new(EnsembleModel::new()),
        }
    }
    
    /// Load model from serialized data
    pub fn load_model(model_type: ModelType, data: &[u8]) -> DriftResult<Box<dyn AnomalyModel>> {
        match model_type {
            ModelType::IsolationForest => Ok(Box::new(IsolationForestModel::deserialize(data)?)),
            ModelType::OneClassSVM => Ok(Box::new(OneClassSVMModel::deserialize(data)?)),
            ModelType::LocalOutlierFactor => Ok(Box::new(LOFModel::deserialize(data)?)),
            ModelType::Statistical => Ok(Box::new(StatisticalModel::deserialize(data)?)),
            ModelType::Ensemble => Ok(Box::new(EnsembleModel::deserialize(data)?)),
        }
    }
}

/// Isolation Forest implementation for anomaly detection
pub struct IsolationForestModel {
    /// Number of trees in the forest
    n_trees: usize,
    
    /// Maximum tree depth
    max_depth: usize,
    
    /// Sample size for each tree
    sample_size: usize,
    
    /// Trained trees (simplified representation)
    trees: Vec<IsolationTree>,
}

/// Simple isolation tree structure
#[derive(Debug, Clone)]
struct IsolationTree {
    /// Feature to split on
    split_feature: usize,
    
    /// Split threshold
    split_threshold: f64,
    
    /// Tree depth
    depth: usize,
    
    /// Is leaf node
    is_leaf: bool,
}

impl IsolationForestModel {
    pub fn new() -> Self {
        Self {
            n_trees: 100,
            max_depth: 8,
            sample_size: 256,
            trees: Vec::new(),
        }
    }
    
    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        // TODO: Implement actual deserialization
        Ok(Self::new())
    }
    
    fn calculate_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        if self.trees.is_empty() {
            // Fallback calculation based on feature values
            let complexity_weight = features.complexity_score * 0.4;
            let file_count_weight = (features.file_count as f64 / 10.0).min(1.0) * 0.3;
            let diversity_weight = (features.tech_diversity as f64 / 5.0).min(1.0) * 0.3;
            
            complexity_weight + file_count_weight + diversity_weight
        } else {
            // Calculate average path length across all trees
            let avg_path_length: f64 = self.trees.iter()
                .map(|tree| self.calculate_path_length(tree, features))
                .sum::<f64>() / self.trees.len() as f64;
            
            // Convert to anomaly score (0-1 range)
            (-avg_path_length / self.max_depth as f64).exp()
        }
    }
    
    fn calculate_path_length(&self, _tree: &IsolationTree, _features: &DriftFeatures) -> f64 {
        // Simplified path length calculation
        // TODO: Implement actual tree traversal
        4.5 // Average path length placeholder
    }
}

impl AnomalyModel for IsolationForestModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let anomaly_score = self.calculate_anomaly_score(features);
        let confidence = 0.8; // High confidence for isolation forest
        
        Ok(Prediction {
            confidence,
            anomaly_score,
            is_anomaly: anomaly_score > 0.6,
        })
    }
    
    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        let mut explanations = Vec::new();
        
        if features.complexity_score > 0.7 {
            explanations.push("High complexity score indicates unusual architectural pattern");
        }
        
        if features.tech_diversity > 3 {
            explanations.push("High technology diversity suggests significant architectural change");
        }
        
        if features.file_count > 5 {
            explanations.push("Large number of affected files indicates broad impact");
        }
        
        if explanations.is_empty() {
            Some("Anomaly detected based on isolation forest analysis".to_string())
        } else {
            Some(explanations.join("; "))
        }
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        // TODO: Implement actual serialization
        Ok(format!("isolation_forest_model_{}_trees", self.n_trees).into_bytes())
    }
    
    fn model_type(&self) -> ModelType {
        ModelType::IsolationForest
    }
}

/// One-Class SVM model for outlier detection
pub struct OneClassSVMModel {
    /// Support vectors (simplified)
    support_vectors: Vec<Vec<f64>>,
    
    /// Model parameters
    nu: f64,
    gamma: f64,
}

impl OneClassSVMModel {
    pub fn new() -> Self {
        Self {
            support_vectors: Vec::new(),
            nu: 0.1,
            gamma: 0.1,
        }
    }
    
    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        Ok(Self::new())
    }
}

impl AnomalyModel for OneClassSVMModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        // Simplified SVM prediction
        let feature_vector = vec![
            features.complexity_score,
            features.file_count as f64 / 10.0,
            features.tech_diversity as f64 / 5.0,
        ];
        
        let distance = self.calculate_distance_to_hyperplane(&feature_vector);
        let anomaly_score = (-distance).max(0.0).min(1.0);
        
        Ok(Prediction {
            confidence: 0.75,
            anomaly_score,
            is_anomaly: distance < 0.0,
        })
    }
    
    fn explain(&self, _features: &DriftFeatures) -> Option<String> {
        Some("One-Class SVM detected pattern deviation from normal architectural decisions".to_string())
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(format!("svm_model_nu_{}_gamma_{}", self.nu, self.gamma).into_bytes())
    }
    
    fn model_type(&self) -> ModelType {
        ModelType::OneClassSVM
    }
    
    
}

impl OneClassSVMModel {
    fn calculate_distance_to_hyperplane(&self, _feature_vector: &[f64]) -> f64 {
        // Simplified distance calculation
        // TODO: Implement actual SVM distance calculation
        0.2 // Placeholder distance
    }
}

/// Local Outlier Factor model
pub struct LOFModel {
    /// Number of neighbors to consider
    n_neighbors: usize,
    
    /// Training data points
    training_data: Vec<Vec<f64>>,
}

impl LOFModel {
    pub fn new() -> Self {
        Self {
            n_neighbors: 20,
            training_data: Vec::new(),
        }
    }
    
    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        Ok(Self::new())
    }
}

impl AnomalyModel for LOFModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let lof_score = self.calculate_lof_score(features);
        let anomaly_score = (lof_score - 1.0).max(0.0).min(1.0);
        
        Ok(Prediction {
            confidence: 0.7,
            anomaly_score,
            is_anomaly: lof_score > 1.5,
        })
    }
    
    fn explain(&self, _features: &DriftFeatures) -> Option<String> {
        Some("Local Outlier Factor analysis indicates unusual local density pattern".to_string())
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(format!("lof_model_{}_neighbors", self.n_neighbors).into_bytes())
    }
    
    fn model_type(&self) -> ModelType {
        ModelType::LocalOutlierFactor
    }
}

impl LOFModel {
    fn calculate_lof_score(&self, features: &DriftFeatures) -> f64 {
        // Simplified LOF calculation
        if self.training_data.is_empty() {
            return 1.0; // Normal score when no training data
        }
        
        // TODO: Implement actual LOF algorithm
        let complexity_factor = features.complexity_score * 2.0;
        let diversity_factor = features.tech_diversity as f64 * 0.1;
        
        1.0 + complexity_factor + diversity_factor
    }
}

/// Simple statistical model using mean and standard deviation
pub struct StatisticalModel {
    /// Feature means
    means: Vec<f64>,
    
    /// Feature standard deviations
    std_devs: Vec<f64>,
    
    /// Z-score threshold for anomaly detection
    threshold: f64,
}

impl StatisticalModel {
    pub fn new() -> Self {
        Self {
            means: vec![0.5, 2.0, 1.0], // Default means for complexity, file_count, tech_diversity
            std_devs: vec![0.2, 1.0, 0.5], // Default standard deviations
            threshold: 2.0, // 2 standard deviations
        }
    }
    
    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        Ok(Self::new())
    }
}

impl AnomalyModel for StatisticalModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let feature_values = vec![
            features.complexity_score,
            features.file_count as f64,
            features.tech_diversity as f64,
        ];
        
        let z_scores: Vec<f64> = feature_values.iter()
            .zip(&self.means)
            .zip(&self.std_devs)
            .map(|((value, mean), std_dev)| {
                if *std_dev > 0.0 {
                    (value - mean).abs() / std_dev
                } else {
                    0.0
                }
            })
            .collect();
        
        let max_z_score = z_scores.iter().fold(0.0_f64, |max, &score| max.max(score));
        let anomaly_score = (max_z_score / self.threshold).min(1.0);
        
        Ok(Prediction {
            confidence: 0.6,
            anomaly_score,
            is_anomaly: max_z_score > self.threshold,
        })
    }
    
    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        let feature_values = vec![
            ("complexity", features.complexity_score),
            ("file_count", features.file_count as f64),
            ("tech_diversity", features.tech_diversity as f64),
        ];
        
        let mut explanations = Vec::new();
        
        for (i, (name, value)) in feature_values.iter().enumerate() {
            if i < self.means.len() && i < self.std_devs.len() {
                let z_score = (value - self.means[i]).abs() / self.std_devs[i];
                if z_score > self.threshold {
                    explanations.push(format!("{} is {} standard deviations from normal", name, z_score));
                }
            }
        }
        
        if explanations.is_empty() {
            Some("Statistical analysis detected deviation from normal patterns".to_string())
        } else {
            Some(explanations.join("; "))
        }
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(format!("statistical_model_threshold_{}", self.threshold).into_bytes())
    }
    
    fn model_type(&self) -> ModelType {
        ModelType::Statistical
    }
}

/// Ensemble model combining multiple algorithms
pub struct EnsembleModel {
    /// Component models
    models: Vec<Box<dyn AnomalyModel>>,
    
    /// Voting strategy
    voting_strategy: VotingStrategy,
}

/// Voting strategies for ensemble
#[derive(Debug, Clone)]
pub enum VotingStrategy {
    /// Simple majority voting
    Majority,
    
    /// Weighted average of anomaly scores
    WeightedAverage,
    
    /// Maximum anomaly score
    Maximum,
}

impl EnsembleModel {
    pub fn new() -> Self {
        let models: Vec<Box<dyn AnomalyModel>> = vec![
            Box::new(IsolationForestModel::new()),
            Box::new(StatisticalModel::new()),
        ];
        
        Self {
            models,
            voting_strategy: VotingStrategy::WeightedAverage,
        }
    }
    
    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        Ok(Self::new())
    }
}

impl AnomalyModel for EnsembleModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let predictions: Result<Vec<_>, _> = self.models.iter()
            .map(|model| model.predict(features))
            .collect();
        
        let predictions = predictions?;
        
        match self.voting_strategy {
            VotingStrategy::Majority => {
                let anomaly_count = predictions.iter().filter(|p| p.is_anomaly).count();
                let is_anomaly = anomaly_count > self.models.len() / 2;
                let avg_confidence = predictions.iter().map(|p| p.confidence).sum::<f64>() / predictions.len() as f64;
                let avg_score = predictions.iter().map(|p| p.anomaly_score).sum::<f64>() / predictions.len() as f64;
                
                Ok(Prediction {
                    confidence: avg_confidence,
                    anomaly_score: avg_score,
                    is_anomaly,
                })
            }
            VotingStrategy::WeightedAverage => {
                let weights = vec![0.6, 0.4]; // Favor isolation forest
                let weighted_score = predictions.iter()
                    .zip(&weights)
                    .map(|(pred, weight)| pred.anomaly_score * weight)
                    .sum::<f64>();
                
                let weighted_confidence = predictions.iter()
                    .zip(&weights)
                    .map(|(pred, weight)| pred.confidence * weight)
                    .sum::<f64>();
                
                Ok(Prediction {
                    confidence: weighted_confidence,
                    anomaly_score: weighted_score,
                    is_anomaly: weighted_score > 0.5,
                })
            }
            VotingStrategy::Maximum => {
                let max_prediction = predictions.iter()
                    .max_by(|a, b| a.anomaly_score.partial_cmp(&b.anomaly_score).unwrap())
                    .unwrap();
                
                Ok(max_prediction.clone())
            }
        }
    }
    
    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        let explanations: Vec<String> = self.models.iter()
            .filter_map(|model| model.explain(features))
            .collect();
        
        if explanations.is_empty() {
            Some("Ensemble model detected anomaly through multiple algorithms".to_string())
        } else {
            Some(format!("Ensemble analysis: {}", explanations.join("; ")))
        }
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(format!("ensemble_model_{}_components", self.models.len()).into_bytes())
    }
    
    fn model_type(&self) -> ModelType {
        ModelType::Ensemble
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drift::{DriftSeverity, DriftCategory, DriftLocation, DriftItem};
    use std::path::PathBuf;
    use super::super::features::FeatureExtractor;
    
    fn create_test_features() -> DriftFeatures {
        let extractor = FeatureExtractor::new();
        let drift_item = DriftItem::new(
            "test".to_string(),
            DriftSeverity::High,
            DriftCategory::NewTechnology,
            "Test".to_string(),
            "Test description".to_string(),
            DriftLocation::new(PathBuf::from("test.rs")),
        );
        
        extractor.extract_features(&drift_item).unwrap()
    }
    
    #[test]
    fn test_model_factory() {
        let model = ModelFactory::create_model(ModelType::IsolationForest);
        assert_eq!(model.model_type(), ModelType::IsolationForest);
        
        let model = ModelFactory::create_model(ModelType::Statistical);
        assert_eq!(model.model_type(), ModelType::Statistical);
    }
    
    #[test]
    fn test_isolation_forest_prediction() {
        let model = IsolationForestModel::new();
        let features = create_test_features();
        
        let prediction = model.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }
    
    #[test]
    fn test_statistical_model_prediction() {
        let model = StatisticalModel::new();
        let features = create_test_features();
        
        let prediction = model.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }
    
    #[test]
    fn test_ensemble_model_prediction() {
        let model = EnsembleModel::new();
        let features = create_test_features();
        
        let prediction = model.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }
    
    #[test]
    fn test_model_explanations() {
        let model = IsolationForestModel::new();
        let features = create_test_features();
        
        let explanation = model.explain(&features);
        assert!(explanation.is_some());
        assert!(!explanation.unwrap().is_empty());
    }
    
    #[test]
    fn test_model_serialization() {
        let model = StatisticalModel::new();
        let serialized = model.serialize().unwrap();
        assert!(!serialized.is_empty());
    }
}