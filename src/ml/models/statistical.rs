//! Statistical anomaly detection model

use crate::drift::DriftResult;
use super::core::{AnomalyModel, ModelType};
use super::super::detector::Prediction;
use super::super::features::DriftFeatures;

/// Simple statistical model using mean and standard deviation
pub struct StatisticalModel {
    feature_stats: Vec<FeatureStats>,
    threshold_multiplier: f64,
}

#[derive(Debug, Clone)]
struct FeatureStats {
    mean: f64,
    std_dev: f64,
    min: f64,
    max: f64,
}

impl StatisticalModel {
    /// Create a new statistical model
    pub fn new() -> Self {
        Self {
            feature_stats: Vec::new(),
            threshold_multiplier: 2.0, // 2 standard deviations
        }
    }

    /// Train the statistical model
    pub fn fit(&mut self, data: &[DriftFeatures]) -> DriftResult<()> {
        if data.is_empty() {
            return Ok(());
        }

        // Extract feature vectors
        let feature_vectors: Vec<Vec<f64>> = data.iter()
            .map(|f| self.extract_feature_vector(f))
            .collect();

        let feature_count = feature_vectors[0].len();
        self.feature_stats.clear();

        // Compute statistics for each feature
        for i in 0..feature_count {
            let values: Vec<f64> = feature_vectors.iter()
                .map(|vec| vec[i])
                .collect();

            let stats = self.compute_stats(&values);
            self.feature_stats.push(stats);
        }

        Ok(())
    }

    fn extract_feature_vector(&self, features: &DriftFeatures) -> Vec<f64> {
        vec![
            features.line_count as f64,
            features.decision_count as f64,
            features.complexity_score,
            features.change_frequency,
            features.coupling_score,
            features.cohesion_score,
        ]
    }

    fn compute_stats(&self, values: &[f64]) -> FeatureStats {
        if values.is_empty() {
            return FeatureStats {
                mean: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
            };
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        FeatureStats {
            mean,
            std_dev,
            min,
            max,
        }
    }

    fn anomaly_score(&self, features: &DriftFeatures) -> f64 {
        if self.feature_stats.is_empty() {
            return 0.5; // Default score if not trained
        }

        let feature_vector = self.extract_feature_vector(features);
        let mut total_deviation = 0.0;
        let mut max_deviation = 0.0;

        for (i, &value) in feature_vector.iter().enumerate() {
            if i < self.feature_stats.len() {
                let stats = &self.feature_stats[i];
                let deviation = if stats.std_dev > 0.0 {
                    ((value - stats.mean) / stats.std_dev).abs()
                } else {
                    0.0
                };
                
                total_deviation += deviation;
                max_deviation = max_deviation.max(deviation);
            }
        }

        // Combine average and maximum deviation
        let avg_deviation = total_deviation / feature_vector.len() as f64;
        (avg_deviation + max_deviation) / 2.0
    }
}

impl AnomalyModel for StatisticalModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let score = self.anomaly_score(features);
        let is_anomaly = score > self.threshold_multiplier;
        
        // Normalize confidence to 0-1 range
        let confidence = (score / (self.threshold_multiplier * 2.0)).min(1.0);
        
        Ok(Prediction {
            is_anomaly,
            confidence,
            explanation: self.explain(features),
        })
    }

    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        let score = self.anomaly_score(features);
        Some(format!(
            "Statistical Model: Deviation score {:.2} (threshold: {:.2})",
            score, self.threshold_multiplier
        ))
    }

    fn serialize(&self) -> DriftResult<Vec<u8>> {
        // Simplified serialization
        Ok(format!("StatisticalModel:{}", self.feature_stats.len()).into_bytes())
    }

    fn model_type(&self) -> ModelType {
        ModelType::Statistical
    }
}

impl Default for StatisticalModel {
    fn default() -> Self {
        Self::new()
    }
}