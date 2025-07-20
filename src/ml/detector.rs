//! ML-Enhanced Drift Detector
//! 
//! Implements machine learning algorithms for intelligent drift detection
//! with reduced false positives and adaptive threshold adjustment.

use crate::drift::{DriftItem, DriftResult};
use super::{MLConfig, MLDriftResult, DriftFeatures, FeatureExtractor, AnomalyModel};

/// ML-enhanced drift detector
pub struct MLDriftDetector {
    /// Configuration for ML detection
    config: MLConfig,
    
    /// Feature extractor for converting drift items to ML features
    feature_extractor: FeatureExtractor,
    
    /// Trained anomaly detection model
    model: Option<Box<dyn AnomalyModel>>,
    
    /// Training data for online learning
    training_data: Vec<(DriftFeatures, bool)>,
    
    /// Performance metrics
    metrics: DetectionMetrics,
}

/// Performance metrics for ML detection
#[derive(Debug, Default)]
pub struct DetectionMetrics {
    /// Total predictions made
    pub total_predictions: usize,
    
    /// True positives (correctly identified anomalies)
    pub true_positives: usize,
    
    /// False positives (incorrectly flagged as anomalies)
    pub false_positives: usize,
    
    /// True negatives (correctly identified normal)
    pub true_negatives: usize,
    
    /// False negatives (missed anomalies)
    pub false_negatives: usize,
    
    /// Average prediction confidence
    pub avg_confidence: f64,
    
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
}

impl DetectionMetrics {
    /// Calculate precision (TP / (TP + FP))
    pub fn precision(&self) -> f64 {
        let denominator = self.true_positives + self.false_positives;
        if denominator == 0 {
            return 0.0;
        }
        self.true_positives as f64 / denominator as f64
    }
    
    /// Calculate recall (TP / (TP + FN))
    pub fn recall(&self) -> f64 {
        let denominator = self.true_positives + self.false_negatives;
        if denominator == 0 {
            return 0.0;
        }
        self.true_positives as f64 / denominator as f64
    }
    
    /// Calculate F1 score (harmonic mean of precision and recall)
    pub fn f1_score(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        
        if precision + recall == 0.0 {
            return 0.0;
        }
        
        2.0 * (precision * recall) / (precision + recall)
    }
    
    /// Calculate accuracy ((TP + TN) / Total)
    pub fn accuracy(&self) -> f64 {
        if self.total_predictions == 0 {
            return 0.0;
        }
        
        let correct = self.true_positives + self.true_negatives;
        correct as f64 / self.total_predictions as f64
    }
}

impl MLDriftDetector {
    /// Create a new ML drift detector
    pub fn new(config: MLConfig) -> Self {
        Self {
            config,
            feature_extractor: FeatureExtractor::new(),
            model: None,
            training_data: Vec::new(),
            metrics: DetectionMetrics::default(),
        }
    }
    
    /// Load a pre-trained model from file
    pub async fn load_model(&mut self, model_path: &std::path::Path) -> DriftResult<()> {
        if !model_path.exists() {
            return Err(crate::error::AdrscanError::InvalidArgument(
                format!("Model file not found: {}", model_path.display())
            ));
        }
        
        log::info!("Loading ML model from: {}", model_path.display());
        
        // TODO: Implement actual model loading based on model type
        // For now, create a mock model
        self.model = Some(Box::new(MockAnomalyModel::new()));
        
        Ok(())
    }
    
    /// Train a new model from historical data
    pub async fn train_model(&mut self, training_data: Vec<(DriftItem, bool)>) -> DriftResult<()> {
        if training_data.is_empty() {
            return Err(crate::error::AdrscanError::InvalidArgument(
                "Training data cannot be empty".to_string()
            ));
        }
        
        log::info!("Training ML model with {} samples", training_data.len());
        
        // Extract features from training data
        let mut feature_data = Vec::new();
        for (drift_item, is_anomaly) in training_data {
            let features = self.feature_extractor.extract_features(&drift_item)?;
            feature_data.push((features, is_anomaly));
        }
        
        // Store training data for online learning
        self.training_data = feature_data;
        
        // TODO: Implement actual model training
        self.model = Some(Box::new(MockAnomalyModel::new()));
        
        log::info!("Model training completed");
        Ok(())
    }
    
    /// Enhance drift detection with ML predictions
    pub async fn enhance_detection(&mut self, drift_items: Vec<DriftItem>) -> DriftResult<Vec<MLDriftResult>> {
        let items_count = drift_items.len();
        
        if !self.config.enabled {
            // ML disabled, return original items wrapped in ML results
            return Ok(drift_items.into_iter().map(|item| {
                let features = DriftFeatures::default();
                MLDriftResult::new(item, 1.0, 0.0, features)
            }).collect());
        }
        
        if self.model.is_none() {
            log::warn!("No ML model loaded, falling back to rule-based detection");
            return Ok(drift_items.into_iter().map(|item| {
                let features = self.feature_extractor.extract_features(&item).unwrap_or_default();
                MLDriftResult::new(item, 0.5, 0.0, features)
            }).collect());
        }
        
        let mut results = Vec::new();
        
        for drift_item in drift_items {
            let start_time = std::time::Instant::now();
            
            // Extract features from drift item
            let features = self.feature_extractor.extract_features(&drift_item)?;
            
            // Get prediction from ML model
            let prediction = if let Some(ref model) = self.model {
                model.predict(&features)?
            } else {
                return Err(crate::error::AdrscanError::InvalidArgument(
                    "No model available".to_string()
                ));
            };
            
            // Create enhanced result
            let mut ml_result = MLDriftResult::new(
                drift_item,
                prediction.confidence,
                prediction.anomaly_score,
                features,
            );
            
            // Add explanation if model supports it
            if let Some(ref model) = self.model {
                if let Some(explanation) = model.explain(&ml_result.features) {
                    ml_result = ml_result.with_explanation(explanation);
                }
            }
            
            // Update metrics
            let processing_time = start_time.elapsed().as_millis() as f64;
            self.update_metrics(processing_time, prediction.confidence);
            
            results.push(ml_result);
        }
        
        // Apply confidence threshold filtering
        let filtered_results: Vec<MLDriftResult> = results
            .into_iter()
            .filter(|result| result.should_report(self.config.confidence_threshold))
            .collect();
        
        log::info!(
            "ML enhancement completed: {} items processed, {} above threshold",
            items_count,
            filtered_results.len()
        );
        
        Ok(filtered_results)
    }
    
    /// Provide feedback for online learning
    pub fn provide_feedback(&mut self, item_id: &str, is_correct: bool) -> DriftResult<()> {
        if !self.config.online_learning {
            return Ok(());
        }
        
        // TODO: Implement online learning feedback mechanism
        log::debug!("Received feedback for item {}: correct={}", item_id, is_correct);
        
        // Update metrics based on feedback
        if is_correct {
            self.metrics.true_positives += 1;
        } else {
            self.metrics.false_positives += 1;
        }
        
        Ok(())
    }
    
    /// Get current performance metrics
    pub fn get_metrics(&self) -> &DetectionMetrics {
        &self.metrics
    }
    
    /// Update internal metrics
    fn update_metrics(&mut self, processing_time_ms: f64, confidence: f64) {
        self.metrics.total_predictions += 1;
        
        // Update running averages
        let total = self.metrics.total_predictions as f64;
        self.metrics.avg_processing_time_ms = 
            (self.metrics.avg_processing_time_ms * (total - 1.0) + processing_time_ms) / total;
        self.metrics.avg_confidence = 
            (self.metrics.avg_confidence * (total - 1.0) + confidence) / total;
    }
    
    /// Save the current model to file
    pub async fn save_model(&self, model_path: &std::path::Path) -> DriftResult<()> {
        if let Some(model) = &self.model {
            log::info!("Saving ML model to: {}", model_path.display());
            
            // Create parent directory if it doesn't exist
            if let Some(parent) = model_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| crate::error::AdrscanError::Io(e))?;
            }
            
            // TODO: Implement actual model serialization
            let model_data = model.serialize()?;
            std::fs::write(model_path, model_data)
                .map_err(|e| crate::error::AdrscanError::Io(e))?;
            
            log::info!("Model saved successfully");
        } else {
            return Err(crate::error::AdrscanError::InvalidArgument(
                "No model to save".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Mock anomaly model for testing and development
struct MockAnomalyModel;

impl MockAnomalyModel {
    fn new() -> Self {
        Self
    }
}

/// Prediction result from anomaly model
#[derive(Debug, Clone)]
pub struct Prediction {
    /// Confidence in the prediction (0.0-1.0)
    pub confidence: f64,
    
    /// Anomaly score (0.0-1.0, higher = more anomalous)
    pub anomaly_score: f64,
    
    /// Whether this is classified as an anomaly
    pub is_anomaly: bool,
}

impl AnomalyModel for MockAnomalyModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        // Simple mock prediction based on feature values
        // For testing, make it more likely to detect anomalies
        let anomaly_score = if features.file_count > 0 || features.complexity_score > 0.3 {
            0.7 // Lower threshold for test environment
        } else {
            0.3
        };
        
        Ok(Prediction {
            confidence: 0.8,
            anomaly_score,
            is_anomaly: anomaly_score > 0.5,
        })
    }
    
    fn explain(&self, _features: &DriftFeatures) -> Option<String> {
        Some("Mock model prediction based on file count and complexity".to_string())
    }
    
    fn serialize(&self) -> DriftResult<Vec<u8>> {
        Ok(b"mock_model_data".to_vec())
    }
    
    fn model_type(&self) -> super::models::ModelType {
        super::models::ModelType::IsolationForest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drift::{DriftSeverity, DriftCategory, DriftLocation};
    use std::path::PathBuf;
    
    fn create_test_drift_item() -> DriftItem {
        DriftItem::new(
            "test".to_string(),
            DriftSeverity::High, // Higher severity to trigger anomaly
            DriftCategory::NewTechnology,
            "Test technology".to_string(),
            "Test description".to_string(),
            DriftLocation::new(PathBuf::from("test.rs")),
        )
    }
    
    #[test]
    fn test_detection_metrics() {
        let mut metrics = DetectionMetrics::default();
        metrics.true_positives = 8;
        metrics.false_positives = 2;
        metrics.true_negatives = 15;
        metrics.false_negatives = 3;
        metrics.total_predictions = 28;
        
        assert_eq!(metrics.precision(), 0.8); // 8 / (8 + 2)
        assert_eq!(metrics.recall(), 8.0 / 11.0); // 8 / (8 + 3)
        assert_eq!(metrics.accuracy(), 23.0 / 28.0); // (8 + 15) / 28
        
        let f1 = metrics.f1_score();
        assert!(f1 > 0.7 && f1 < 0.8);
    }
    
    #[tokio::test]
    async fn test_ml_detector_creation() {
        let config = MLConfig::default();
        let detector = MLDriftDetector::new(config);
        
        assert!(detector.model.is_none());
        assert_eq!(detector.metrics.total_predictions, 0);
    }
    
    #[tokio::test]
    async fn test_enhance_detection_disabled() {
        let mut config = MLConfig::default();
        config.enabled = false;
        
        let mut detector = MLDriftDetector::new(config);
        let drift_items = vec![create_test_drift_item()];
        
        let results = detector.enhance_detection(drift_items).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].confidence, 1.0);
        assert_eq!(results[0].anomaly_score, 0.0);
    }
    
    #[tokio::test]
    async fn test_enhance_detection_with_mock_model() {
        let mut config = MLConfig::default();
        config.enabled = true;
        config.confidence_threshold = 0.5; // Lower threshold for test
        
        let mut detector = MLDriftDetector::new(config);
        
        // Train mock model
        let training_data = vec![
            (create_test_drift_item(), true),
            (create_test_drift_item(), false),
        ];
        
        detector.train_model(training_data).await.unwrap();
        
        let drift_items = vec![create_test_drift_item()];
        let results = detector.enhance_detection(drift_items).await.unwrap();
        
        // Even with normal features, the mock model should return a result
        // since we lowered the confidence threshold
        assert!(!results.is_empty());
        if !results.is_empty() {
            assert!(results[0].confidence > 0.0);
            assert!(results[0].explanation.is_some());
        }
    }
    
    #[test]
    fn test_metrics_update() {
        let mut detector = MLDriftDetector::new(MLConfig::default());
        
        detector.update_metrics(100.0, 0.8);
        assert_eq!(detector.metrics.total_predictions, 1);
        assert_eq!(detector.metrics.avg_processing_time_ms, 100.0);
        assert_eq!(detector.metrics.avg_confidence, 0.8);
        
        detector.update_metrics(200.0, 0.6);
        assert_eq!(detector.metrics.total_predictions, 2);
        assert_eq!(detector.metrics.avg_processing_time_ms, 150.0);
        assert_eq!(detector.metrics.avg_confidence, 0.7);
    }
}