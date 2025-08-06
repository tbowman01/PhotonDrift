//! One-Class SVM implementation for outlier detection

use super::super::detector::Prediction;
use super::super::features::DriftFeatures;
use super::core::{AnomalyModel, ModelType};
use crate::drift::DriftResult;

/// One-Class SVM model for outlier detection
pub struct OneClassSVM {
    support_vectors: Vec<DriftFeatures>,
    decision_function_offset: f64,
    gamma: f64,
    nu: f64,
}

impl OneClassSVM {
    /// Create a new One-Class SVM
    pub fn new() -> Self {
        Self {
            support_vectors: Vec::new(),
            decision_function_offset: 0.0,
            gamma: 0.1,
            nu: 0.1,
        }
    }

    /// Train the SVM model
    pub fn fit(&mut self, data: &[DriftFeatures]) -> DriftResult<()> {
        // Simplified SVM training - in production, use proper SVM implementation
        self.support_vectors = data.to_vec();
        self.decision_function_offset = self.compute_offset(data);
        Ok(())
    }

    /// Train method alias for compatibility
    pub fn train(&mut self, data: &[DriftFeatures]) -> DriftResult<()> {
        self.fit(data)
    }

    fn compute_offset(&self, data: &[DriftFeatures]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        // Simplified offset computation
        let scores: Vec<f64> = data
            .iter()
            .map(|features| self.rbf_kernel_sum(features, data))
            .collect();

        let mean_score = scores.iter().sum::<f64>() / scores.len() as f64;
        mean_score * self.nu
    }

    fn rbf_kernel_sum(&self, x: &DriftFeatures, data: &[DriftFeatures]) -> f64 {
        data.iter().map(|y| self.rbf_kernel(x, y)).sum()
    }

    fn rbf_kernel(&self, x: &DriftFeatures, y: &DriftFeatures) -> f64 {
        // Simplified RBF kernel computation
        let distance_sq = self.euclidean_distance_squared(x, y);
        (-self.gamma * distance_sq).exp()
    }

    fn euclidean_distance_squared(&self, x: &DriftFeatures, y: &DriftFeatures) -> f64 {
        // Simplified distance computation using basic features
        let x_vec = vec![
            x.line_count as f64,
            x.decision_count as f64,
            x.complexity_score,
            x.change_frequency,
        ];

        let y_vec = vec![
            y.line_count as f64,
            y.decision_count as f64,
            y.complexity_score,
            y.change_frequency,
        ];

        x_vec
            .iter()
            .zip(y_vec.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }

    fn decision_function(&self, features: &DriftFeatures) -> f64 {
        if self.support_vectors.is_empty() {
            return 0.0;
        }

        let kernel_sum = self.rbf_kernel_sum(features, &self.support_vectors);
        kernel_sum - self.decision_function_offset
    }
}

impl AnomalyModel for OneClassSVM {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let score = self.decision_function(features);
        let is_anomaly = score < 0.0; // Negative scores indicate outliers
        let confidence = score.abs(); // Use absolute value as confidence

        Ok(Prediction {
            is_anomaly,
            confidence,
            anomaly_score: if is_anomaly {
                score.abs()
            } else {
                1.0 - score.abs()
            },
            explanation: self.explain(features),
        })
    }

    fn explain(&self, _features: &DriftFeatures) -> Option<String> {
        Some(format!(
            "One-Class SVM: Outlier detection using RBF kernel (gamma={}, nu={})",
            self.gamma, self.nu
        ))
    }

    fn serialize(&self) -> DriftResult<Vec<u8>> {
        // Simplified serialization
        Ok(format!(
            "OneClassSVM:{}:{}:{}",
            self.gamma,
            self.nu,
            self.support_vectors.len()
        )
        .into_bytes())
    }

    fn model_type(&self) -> ModelType {
        ModelType::OneClassSVM
    }
}

impl Default for OneClassSVM {
    fn default() -> Self {
        Self::new()
    }
}
