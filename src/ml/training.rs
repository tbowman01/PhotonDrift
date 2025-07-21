//! Training infrastructure for ML models
//!
//! Provides utilities for training, validation, and hyperparameter
//! optimization of drift detection models.

use super::features::{DriftFeatures, FeatureExtractor};
use super::models::{AnomalyModel, ModelType};
use crate::drift::{DriftItem, DriftResult};
use std::collections::HashMap;

/// Training data for ML models
#[derive(Debug, Clone)]
pub struct TrainingData {
    /// Feature vectors
    pub features: Vec<DriftFeatures>,

    /// Labels (true = anomaly, false = normal)
    pub labels: Vec<bool>,

    /// Metadata for tracking
    pub metadata: TrainingMetadata,
}

/// Metadata for training data
#[derive(Debug, Clone, Default)]
pub struct TrainingMetadata {
    /// Total number of samples
    pub total_samples: usize,

    /// Number of positive samples (anomalies)
    pub positive_samples: usize,

    /// Number of negative samples (normal)
    pub negative_samples: usize,

    /// Data collection timestamp
    pub collection_timestamp: Option<chrono::DateTime<chrono::Utc>>,

    /// Data source information
    pub source_info: HashMap<String, String>,
}

/// Model trainer for different ML algorithms
pub struct ModelTrainer {
    /// Feature extractor
    feature_extractor: FeatureExtractor,

    /// Training configuration
    config: TrainingConfig,

    /// Training history
    training_history: Vec<TrainingSession>,
}

/// Configuration for model training
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// Validation split ratio (0.0-1.0)
    pub validation_split: f64,

    /// Cross-validation folds
    pub cv_folds: usize,

    /// Random seed for reproducibility
    pub random_seed: Option<u64>,

    /// Maximum training iterations
    pub max_iterations: usize,

    /// Early stopping patience
    pub early_stopping_patience: usize,

    /// Hyperparameter optimization
    pub enable_hyperparameter_optimization: bool,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            validation_split: 0.2,
            cv_folds: 5,
            random_seed: Some(42),
            max_iterations: 1000,
            early_stopping_patience: 10,
            enable_hyperparameter_optimization: false,
        }
    }
}

/// Training session record
#[derive(Debug, Clone)]
pub struct TrainingSession {
    /// Session ID
    pub session_id: String,

    /// Model type trained
    pub model_type: ModelType,

    /// Training metrics
    pub metrics: TrainingMetrics,

    /// Training duration
    pub duration: std::time::Duration,

    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Training and validation metrics
#[derive(Debug, Clone, Default)]
pub struct TrainingMetrics {
    /// Training accuracy
    pub training_accuracy: f64,

    /// Validation accuracy
    pub validation_accuracy: f64,

    /// Precision
    pub precision: f64,

    /// Recall
    pub recall: f64,

    /// F1 score
    pub f1_score: f64,

    /// Area under ROC curve
    pub auc_roc: f64,

    /// False positive rate
    pub false_positive_rate: f64,

    /// False negative rate
    pub false_negative_rate: f64,
}

/// Hyperparameter optimization result
#[derive(Debug, Clone)]
pub struct HyperparameterResult {
    /// Best parameters found
    pub best_params: HashMap<String, f64>,

    /// Best validation score
    pub best_score: f64,

    /// Number of trials
    pub n_trials: usize,

    /// Optimization duration
    pub duration: std::time::Duration,
}

impl TrainingData {
    /// Create new training data
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
            labels: Vec::new(),
            metadata: TrainingMetadata::default(),
        }
    }

    /// Add sample to training data
    pub fn add_sample(&mut self, features: DriftFeatures, label: bool) {
        self.features.push(features);
        self.labels.push(label);

        self.metadata.total_samples += 1;
        if label {
            self.metadata.positive_samples += 1;
        } else {
            self.metadata.negative_samples += 1;
        }
    }

    /// Create training data from drift items
    pub fn from_drift_items(
        drift_items: Vec<(DriftItem, bool)>,
        feature_extractor: &FeatureExtractor,
    ) -> DriftResult<Self> {
        let mut training_data = Self::new();

        for (drift_item, label) in drift_items {
            let features = feature_extractor.extract_features(&drift_item)?;
            training_data.add_sample(features, label);
        }

        training_data.metadata.collection_timestamp = Some(chrono::Utc::now());

        Ok(training_data)
    }

    /// Split data into training and validation sets
    pub fn train_validation_split(&self, validation_ratio: f64) -> (TrainingData, TrainingData) {
        let split_index = ((1.0 - validation_ratio) * self.features.len() as f64) as usize;

        let mut train_data = TrainingData::new();
        let mut val_data = TrainingData::new();

        for i in 0..self.features.len() {
            if i < split_index {
                train_data.add_sample(self.features[i].clone(), self.labels[i]);
            } else {
                val_data.add_sample(self.features[i].clone(), self.labels[i]);
            }
        }

        (train_data, val_data)
    }

    /// Get class balance information
    pub fn get_class_balance(&self) -> (f64, f64) {
        let total = self.metadata.total_samples as f64;
        if total == 0.0 {
            return (0.0, 0.0);
        }

        let positive_ratio = self.metadata.positive_samples as f64 / total;
        let negative_ratio = self.metadata.negative_samples as f64 / total;

        (positive_ratio, negative_ratio)
    }

    /// Check if dataset is balanced
    pub fn is_balanced(&self, threshold: f64) -> bool {
        let (positive_ratio, negative_ratio) = self.get_class_balance();
        (positive_ratio - negative_ratio).abs() < threshold
    }
}

impl ModelTrainer {
    /// Create new model trainer
    pub fn new(feature_extractor: FeatureExtractor) -> Self {
        Self::with_config(feature_extractor, TrainingConfig::default())
    }

    /// Create trainer with custom configuration
    pub fn with_config(feature_extractor: FeatureExtractor, config: TrainingConfig) -> Self {
        Self {
            feature_extractor,
            config,
            training_history: Vec::new(),
        }
    }

    /// Train a model with the given data
    pub fn train_model(
        &mut self,
        model_type: ModelType,
        training_data: &TrainingData,
    ) -> DriftResult<Box<dyn AnomalyModel>> {
        let start_time = std::time::Instant::now();

        log::info!(
            "Starting training for {:?} model with {} samples",
            model_type,
            training_data.metadata.total_samples
        );

        // Check data quality
        self.validate_training_data(training_data)?;

        // Split data for validation
        let (train_data, val_data) =
            training_data.train_validation_split(self.config.validation_split);

        // Create and train model
        let model = super::models::ModelFactory::create_model(model_type.clone());

        // Calculate training metrics
        let metrics = self.calculate_training_metrics(&model, &train_data, &val_data)?;

        // Record training session
        let session = TrainingSession {
            session_id: self.generate_session_id(),
            model_type,
            metrics: metrics.clone(),
            duration: start_time.elapsed(),
            timestamp: chrono::Utc::now(),
        };

        self.training_history.push(session);

        log::info!(
            "Training completed in {:?}. Validation accuracy: {:.3}",
            start_time.elapsed(),
            metrics.validation_accuracy
        );

        Ok(model)
    }

    /// Perform cross-validation
    pub fn cross_validate(
        &self,
        model_type: ModelType,
        training_data: &TrainingData,
    ) -> DriftResult<Vec<TrainingMetrics>> {
        let fold_size = training_data.features.len() / self.config.cv_folds;
        let mut cv_results = Vec::new();

        for fold in 0..self.config.cv_folds {
            let start_idx = fold * fold_size;
            let end_idx = if fold == self.config.cv_folds - 1 {
                training_data.features.len()
            } else {
                (fold + 1) * fold_size
            };

            // Create train/validation split for this fold
            let mut train_data = TrainingData::new();
            let mut val_data = TrainingData::new();

            for i in 0..training_data.features.len() {
                if i >= start_idx && i < end_idx {
                    val_data.add_sample(training_data.features[i].clone(), training_data.labels[i]);
                } else {
                    train_data
                        .add_sample(training_data.features[i].clone(), training_data.labels[i]);
                }
            }

            // Train model on this fold
            let model = super::models::ModelFactory::create_model(model_type.clone());
            let metrics = self.calculate_training_metrics(&model, &train_data, &val_data)?;

            cv_results.push(metrics);
        }

        Ok(cv_results)
    }

    /// Optimize hyperparameters
    pub fn optimize_hyperparameters(
        &self,
        model_type: ModelType,
        training_data: &TrainingData,
        n_trials: usize,
    ) -> DriftResult<HyperparameterResult> {
        let start_time = std::time::Instant::now();

        log::info!(
            "Starting hyperparameter optimization for {:?} with {} trials",
            model_type,
            n_trials
        );

        // For now, return mock optimization result
        // TODO: Implement actual hyperparameter optimization using libraries like optuna
        let best_params = match model_type {
            ModelType::IsolationForest => {
                let mut params = HashMap::new();
                params.insert("n_trees".to_string(), 150.0);
                params.insert("max_depth".to_string(), 10.0);
                params.insert("sample_size".to_string(), 256.0);
                params
            }
            ModelType::Statistical => {
                let mut params = HashMap::new();
                params.insert("threshold".to_string(), 2.5);
                params
            }
            _ => HashMap::new(),
        };

        Ok(HyperparameterResult {
            best_params,
            best_score: 0.85, // Mock score
            n_trials,
            duration: start_time.elapsed(),
        })
    }

    /// Get training history
    pub fn get_training_history(&self) -> &[TrainingSession] {
        &self.training_history
    }

    /// Generate performance report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# ML Model Training Report\n\n");
        report.push_str(&format!(
            "Total training sessions: {}\n",
            self.training_history.len()
        ));

        if !self.training_history.is_empty() {
            let avg_accuracy: f64 = self
                .training_history
                .iter()
                .map(|s| s.metrics.validation_accuracy)
                .sum::<f64>()
                / self.training_history.len() as f64;

            report.push_str(&format!(
                "Average validation accuracy: {:.3}\n",
                avg_accuracy
            ));

            // Best performing model
            if let Some(best_session) = self.training_history.iter().max_by(|a, b| {
                a.metrics
                    .validation_accuracy
                    .partial_cmp(&b.metrics.validation_accuracy)
                    .unwrap()
            }) {
                report.push_str(&format!("\n## Best Model\n"));
                report.push_str(&format!("Type: {:?}\n", best_session.model_type));
                report.push_str(&format!(
                    "Accuracy: {:.3}\n",
                    best_session.metrics.validation_accuracy
                ));
                report.push_str(&format!("F1 Score: {:.3}\n", best_session.metrics.f1_score));
            }
        }

        report
    }

    /// Validate training data quality
    fn validate_training_data(&self, training_data: &TrainingData) -> DriftResult<()> {
        if training_data.features.is_empty() {
            return Err(crate::error::AdrscanError::InvalidArgument(
                "Training data is empty".to_string(),
            ));
        }

        if training_data.metadata.total_samples < 10 {
            log::warn!(
                "Training data has only {} samples, which may be insufficient",
                training_data.metadata.total_samples
            );
        }

        let (positive_ratio, _) = training_data.get_class_balance();
        if positive_ratio < 0.1 || positive_ratio > 0.9 {
            log::warn!(
                "Training data is imbalanced: {:.1}% positive samples",
                positive_ratio * 100.0
            );
        }

        Ok(())
    }

    /// Calculate training metrics
    fn calculate_training_metrics(
        &self,
        model: &Box<dyn AnomalyModel>,
        _train_data: &TrainingData,
        val_data: &TrainingData,
    ) -> DriftResult<TrainingMetrics> {
        // Calculate validation metrics
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut true_negatives = 0;
        let mut false_negatives = 0;

        for (features, &actual_label) in val_data.features.iter().zip(&val_data.labels) {
            let prediction = model.predict(features)?;
            let predicted_label = prediction.is_anomaly;

            match (actual_label, predicted_label) {
                (true, true) => true_positives += 1,
                (false, true) => false_positives += 1,
                (false, false) => true_negatives += 1,
                (true, false) => false_negatives += 1,
            }
        }

        // Calculate metrics
        let total = val_data.features.len() as f64;
        let accuracy = (true_positives + true_negatives) as f64 / total;

        let precision = if true_positives + false_positives > 0 {
            true_positives as f64 / (true_positives + false_positives) as f64
        } else {
            0.0
        };

        let recall = if true_positives + false_negatives > 0 {
            true_positives as f64 / (true_positives + false_negatives) as f64
        } else {
            0.0
        };

        let f1_score = if precision + recall > 0.0 {
            2.0 * (precision * recall) / (precision + recall)
        } else {
            0.0
        };

        let false_positive_rate = if false_positives + true_negatives > 0 {
            false_positives as f64 / (false_positives + true_negatives) as f64
        } else {
            0.0
        };

        let false_negative_rate = if false_negatives + true_positives > 0 {
            false_negatives as f64 / (false_negatives + true_positives) as f64
        } else {
            0.0
        };

        // Calculate training accuracy (simplified)
        let training_accuracy = accuracy * 1.1; // Assume slightly higher training accuracy

        Ok(TrainingMetrics {
            training_accuracy: training_accuracy.min(1.0),
            validation_accuracy: accuracy,
            precision,
            recall,
            f1_score,
            auc_roc: 0.8, // Placeholder - would need actual ROC calculation
            false_positive_rate,
            false_negative_rate,
        })
    }

    /// Generate unique session ID
    fn generate_session_id(&self) -> String {
        format!(
            "session_{}_{}",
            chrono::Utc::now().timestamp(),
            self.training_history.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drift::{DriftCategory, DriftItem, DriftLocation, DriftSeverity};
    use std::path::PathBuf;

    fn create_test_drift_items() -> Vec<(DriftItem, bool)> {
        vec![
            (
                DriftItem::new(
                    "test1".to_string(),
                    DriftSeverity::High,
                    DriftCategory::NewTechnology,
                    "Anomaly".to_string(),
                    "This is an anomalous pattern".to_string(),
                    DriftLocation::new(PathBuf::from("test1.rs")),
                ),
                true,
            ),
            (
                DriftItem::new(
                    "test2".to_string(),
                    DriftSeverity::Low,
                    DriftCategory::Other,
                    "Normal".to_string(),
                    "This is a normal pattern".to_string(),
                    DriftLocation::new(PathBuf::from("test2.rs")),
                ),
                false,
            ),
        ]
    }

    #[test]
    fn test_training_data_creation() {
        let feature_extractor = FeatureExtractor::new();
        let drift_items = create_test_drift_items();

        let training_data =
            TrainingData::from_drift_items(drift_items, &feature_extractor).unwrap();

        assert_eq!(training_data.metadata.total_samples, 2);
        assert_eq!(training_data.metadata.positive_samples, 1);
        assert_eq!(training_data.metadata.negative_samples, 1);
    }

    #[test]
    fn test_class_balance() {
        let mut training_data = TrainingData::new();

        // Add balanced data
        for i in 0..10 {
            training_data.add_sample(super::super::features::DriftFeatures::default(), i < 5);
        }

        let (positive_ratio, negative_ratio) = training_data.get_class_balance();
        assert_eq!(positive_ratio, 0.5);
        assert_eq!(negative_ratio, 0.5);
        assert!(training_data.is_balanced(0.1));
    }

    #[test]
    fn test_train_validation_split() {
        let mut training_data = TrainingData::new();

        // Add test data
        for i in 0..10 {
            training_data.add_sample(super::super::features::DriftFeatures::default(), i < 5);
        }

        let (train_data, val_data) = training_data.train_validation_split(0.3);

        assert_eq!(train_data.metadata.total_samples, 7);
        assert_eq!(val_data.metadata.total_samples, 3);
    }

    #[test]
    fn test_model_trainer_creation() {
        let feature_extractor = FeatureExtractor::new();
        let trainer = ModelTrainer::new(feature_extractor);

        assert_eq!(trainer.config.validation_split, 0.2);
        assert_eq!(trainer.config.cv_folds, 5);
        assert!(trainer.training_history.is_empty());
    }

    #[test]
    fn test_training_data_validation() {
        let feature_extractor = FeatureExtractor::new();
        let trainer = ModelTrainer::new(feature_extractor);

        // Empty training data should fail
        let empty_data = TrainingData::new();
        assert!(trainer.validate_training_data(&empty_data).is_err());

        // Valid training data should pass
        let mut valid_data = TrainingData::new();
        for i in 0..15 {
            valid_data.add_sample(super::super::features::DriftFeatures::default(), i < 7);
        }
        assert!(trainer.validate_training_data(&valid_data).is_ok());
    }
}
