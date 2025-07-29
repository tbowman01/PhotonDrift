//! Machine learning models for anomaly detection
//!
//! Provides different ML algorithms for detecting architectural drift
//! with support for multiple model types and explainable AI.
//!
//! This module has been refactored into smaller, security-auditable components.
//! See the `models/` subdirectory for individual implementations.

// Re-export the new modular structure
pub use models::*;

mod models;

// ML framework imports (currently using custom implementations)
// Future versions will integrate with smartcore or other ML libraries

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
    #[allow(dead_code)]
    max_depth: usize,

    /// Sample size for each tree
    #[allow(dead_code)]
    sample_size: usize,

    /// Real isolation forest model (when ML feature enabled)
    #[cfg(feature = "ml")]
    #[allow(dead_code)]
    model: Option<String>, // Placeholder for now

    /// Fallback trees for non-ML builds
    #[cfg(not(feature = "ml"))]
    trees: Vec<IsolationTree>,

    /// Training data for model fitting
    training_data: Vec<Vec<f64>>,

    /// Is the model trained
    is_trained: bool,
}

/// Simple isolation tree structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
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

impl Default for IsolationForestModel {
    fn default() -> Self {
        Self::new()
    }
}

impl IsolationForestModel {
    pub fn new() -> Self {
        Self {
            n_trees: 100,
            max_depth: 8,
            sample_size: 256,
            #[cfg(feature = "ml")]
            model: None,
            #[cfg(not(feature = "ml"))]
            trees: Vec::new(),
            training_data: Vec::new(),
            is_trained: false,
        }
    }

    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        // TODO: Implement actual deserialization with serde
        Ok(Self::new())
    }

    /// Train the isolation forest with given data
    pub fn train(&mut self, training_features: &[DriftFeatures]) -> DriftResult<()> {
        if training_features.is_empty() {
            return Err(crate::AdrscanError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No training data provided",
            )));
        }

        // Convert features to training matrix
        let feature_matrix = self.features_to_matrix(training_features);
        self.training_data = feature_matrix.clone();

        #[cfg(feature = "ml")]
        {
            // For now, use a custom isolation forest implementation since SmartCore's may be different
            // Create basic model with feature analysis
            self.is_trained = true;
            log::info!(
                "Custom Isolation Forest trained with {} samples",
                feature_matrix.len()
            );
            Ok(())
        }

        #[cfg(not(feature = "ml"))]
        {
            // Fallback: create mock trees
            self.trees = (0..self.n_trees)
                .map(|i| IsolationTree {
                    split_feature: i % feature_matrix[0].len(),
                    split_threshold: 0.5,
                    depth: self.max_depth / 2,
                    is_leaf: false,
                })
                .collect();
            self.is_trained = true;
            log::info!(
                "Mock Isolation Forest initialized with {} trees",
                self.n_trees
            );
            Ok(())
        }
    }

    /// Convert DriftFeatures to feature matrix for ML training
    fn features_to_matrix(&self, features: &[DriftFeatures]) -> Vec<Vec<f64>> {
        features
            .iter()
            .map(|f| self.features_to_vector(f))
            .collect()
    }

    /// Convert single DriftFeatures to feature vector
    fn features_to_vector(&self, features: &DriftFeatures) -> Vec<f64> {
        vec![
            features.file_count as f64,
            features.lines_changed as f64,
            features.complexity_score,
            features.tech_diversity as f64,
            features.pattern_frequency,
            features.temporal_features.days_since_last,
            features.temporal_features.frequency_per_week,
            features.temporal_features.seasonal_strength,
            features.text_features.sentiment_score,
            features.text_features.tech_term_count as f64,
            features.text_features.readability_score,
            features.text_features.description_length as f64,
            features.structural_features.directory_depth as f64,
            features.structural_features.extension_diversity as f64,
            features.structural_features.coupling_strength,
            features.structural_features.cohesion_score,
        ]
    }

    fn calculate_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        if !self.is_trained {
            log::warn!("Isolation Forest not trained, using fallback scoring");
            return self.fallback_anomaly_score(features);
        }

        #[cfg(feature = "ml")]
        {
            // Custom isolation forest algorithm implementation
            let feature_vec = self.features_to_vector(features);
            self.custom_isolation_score(&feature_vec)
        }

        #[cfg(not(feature = "ml"))]
        {
            self.fallback_anomaly_score(features)
        }
    }

    fn fallback_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        // Enhanced fallback calculation based on multiple factors
        let complexity_weight = features.complexity_score * 0.25;
        let file_count_weight = (features.file_count as f64 / 10.0).min(1.0) * 0.20;
        let diversity_weight = (features.tech_diversity as f64 / 5.0).min(1.0) * 0.15;
        let pattern_weight = features.pattern_frequency * 0.15;
        let sentiment_weight = features.text_features.sentiment_score.abs() * 0.10;
        let complexity_terms_weight =
            (features.text_features.tech_term_count as f64 / 20.0).min(1.0) * 0.15;

        (complexity_weight
            + file_count_weight
            + diversity_weight
            + pattern_weight
            + sentiment_weight
            + complexity_terms_weight)
            .min(1.0)
    }

    #[allow(dead_code)]
    fn calculate_path_length(&self, _tree: &IsolationTree, _features: &DriftFeatures) -> f64 {
        // Simplified path length calculation
        // TODO: Implement actual tree traversal
        4.5 // Average path length placeholder
    }

    #[cfg(feature = "ml")]
    fn custom_isolation_score(&self, feature_vec: &[f64]) -> f64 {
        // Custom isolation forest implementation using distance-based anomaly detection
        if self.training_data.is_empty() {
            return self.fallback_anomaly_score_from_vec(feature_vec);
        }

        // Calculate average distance to training samples
        let mut distances: Vec<f64> = self
            .training_data
            .iter()
            .map(|training_sample| self.euclidean_distance(feature_vec, training_sample))
            .collect();

        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Use k-nearest neighbors approach for anomaly detection
        let k = (self.training_data.len() / 10).max(1).min(10);
        let avg_distance: f64 = distances.iter().take(k).sum::<f64>() / k as f64;

        // Convert distance to anomaly score (0-1 range)
        // Higher distance = higher anomaly score
        (avg_distance / 10.0).min(1.0)
    }

    #[cfg(feature = "ml")]
    fn euclidean_distance(&self, vec1: &[f64], vec2: &[f64]) -> f64 {
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    fn fallback_anomaly_score_from_vec(&self, feature_vec: &[f64]) -> f64 {
        // Simple anomaly scoring based on feature vector values
        let mean = feature_vec.iter().sum::<f64>() / feature_vec.len() as f64;
        let variance =
            feature_vec.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / feature_vec.len() as f64;

        // Higher variance indicates more unusual patterns
        (variance / 2.0).min(1.0)
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
            explanations
                .push("High technology diversity suggests significant architectural change");
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
///
/// Implements a simplified but functional One-Class SVM algorithm using:
/// - RBF (Radial Basis Function) kernel
/// - Support vector identification through distance-based selection
/// - Hyperplane boundary calculation using support vectors
/// - Proper anomaly scoring based on distance to decision boundary
pub struct OneClassSVMModel {
    /// Support vectors identified during training
    support_vectors: Vec<Vec<f64>>,

    /// Weights for support vectors (alpha values)
    support_weights: Vec<f64>,

    /// Model hyperparameters
    nu: f64, // Fraction of outliers in training data (0 < nu <= 1)
    gamma: f64, // RBF kernel parameter (higher = more complex boundary)

    /// Training data for distance calculations
    training_data: Vec<Vec<f64>>,

    /// Decision boundary threshold (rho parameter)
    decision_threshold: f64,

    /// Is the model trained
    is_trained: bool,

    /// Statistics for normalization
    feature_means: Vec<f64>,
    feature_stds: Vec<f64>,
}

impl Default for OneClassSVMModel {
    fn default() -> Self {
        Self::new()
    }
}

impl OneClassSVMModel {
    pub fn new() -> Self {
        Self {
            support_vectors: Vec::new(),
            support_weights: Vec::new(),
            nu: 0.1,    // Expect 10% outliers
            gamma: 0.5, // Moderate kernel complexity
            training_data: Vec::new(),
            decision_threshold: 0.0,
            is_trained: false,
            feature_means: Vec::new(),
            feature_stds: Vec::new(),
        }
    }

    /// Create SVM with custom parameters
    pub fn with_params(nu: f64, gamma: f64) -> Self {
        Self {
            support_vectors: Vec::new(),
            support_weights: Vec::new(),
            nu: nu.clamp(0.001, 1.0),
            gamma: gamma.max(0.001),
            training_data: Vec::new(),
            decision_threshold: 0.0,
            is_trained: false,
            feature_means: Vec::new(),
            feature_stds: Vec::new(),
        }
    }

    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        // TODO: Implement actual deserialization with serde
        Ok(Self::new())
    }

    /// Train the One-Class SVM with given data
    pub fn train(&mut self, training_features: &[DriftFeatures]) -> DriftResult<()> {
        if training_features.is_empty() {
            return Err(crate::AdrscanError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No training data provided for SVM",
            )));
        }

        // Convert features to training matrix
        let feature_matrix = self.features_to_matrix(training_features);
        self.training_data = feature_matrix.clone();

        // Normalize features
        self.calculate_normalization_stats(&feature_matrix);
        let normalized_data = self.normalize_features(&feature_matrix);

        #[cfg(feature = "ml")]
        {
            // Implement actual One-Class SVM training
            self.train_svm(&normalized_data)?;
            self.is_trained = true;
            log::info!(
                "One-Class SVM trained with {} samples, {} support vectors",
                feature_matrix.len(),
                self.support_vectors.len()
            );
        }

        #[cfg(not(feature = "ml"))]
        {
            // Fallback: use centroid-based approach
            self.train_fallback(&normalized_data)?;
            self.is_trained = true;
            log::info!("Fallback SVM trained with {} samples", feature_matrix.len());
        }

        Ok(())
    }

    /// Convert DriftFeatures to feature matrix for ML training
    fn features_to_matrix(&self, features: &[DriftFeatures]) -> Vec<Vec<f64>> {
        features
            .iter()
            .map(|f| self.features_to_vector(f))
            .collect()
    }

    /// Convert single DriftFeatures to feature vector (same as Isolation Forest)
    fn features_to_vector(&self, features: &DriftFeatures) -> Vec<f64> {
        vec![
            features.file_count as f64,
            features.lines_changed as f64,
            features.complexity_score,
            features.tech_diversity as f64,
            features.pattern_frequency,
            features.temporal_features.days_since_last,
            features.temporal_features.frequency_per_week,
            features.temporal_features.seasonal_strength,
            features.text_features.sentiment_score,
            features.text_features.tech_term_count as f64,
            features.text_features.readability_score,
            features.text_features.description_length as f64,
            features.structural_features.directory_depth as f64,
            features.structural_features.extension_diversity as f64,
            features.structural_features.coupling_strength,
            features.structural_features.cohesion_score,
        ]
    }

    /// Calculate normalization statistics
    fn calculate_normalization_stats(&mut self, data: &[Vec<f64>]) {
        if data.is_empty() {
            return;
        }

        let feature_count = data[0].len();
        self.feature_means = vec![0.0; feature_count];
        self.feature_stds = vec![1.0; feature_count];

        // Calculate means
        for sample in data {
            for (i, &value) in sample.iter().enumerate() {
                self.feature_means[i] += value;
            }
        }
        for mean in &mut self.feature_means {
            *mean /= data.len() as f64;
        }

        // Calculate standard deviations
        for sample in data {
            for (i, &value) in sample.iter().enumerate() {
                let diff = value - self.feature_means[i];
                self.feature_stds[i] += diff * diff;
            }
        }
        for std in self.feature_stds.iter_mut() {
            *std = (*std / data.len() as f64).sqrt().max(1e-8); // Avoid division by zero
        }
    }

    /// Normalize features using calculated statistics
    fn normalize_features(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        data.iter()
            .map(|sample| {
                sample
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| {
                        if i < self.feature_means.len() && i < self.feature_stds.len() {
                            (value - self.feature_means[i]) / self.feature_stds[i]
                        } else {
                            value
                        }
                    })
                    .collect()
            })
            .collect()
    }

    /// Normalize a single feature vector
    fn normalize_single(&self, features: &[f64]) -> Vec<f64> {
        features
            .iter()
            .enumerate()
            .map(|(i, &value)| {
                if i < self.feature_means.len() && i < self.feature_stds.len() {
                    (value - self.feature_means[i]) / self.feature_stds[i]
                } else {
                    value
                }
            })
            .collect()
    }

    #[cfg(feature = "ml")]
    /// Train the SVM using a simplified SMO-like algorithm
    fn train_svm(&mut self, normalized_data: &[Vec<f64>]) -> DriftResult<()> {
        let n = normalized_data.len();
        if n == 0 {
            return Ok(());
        }

        // Initialize: all points as potential support vectors with equal weights
        let mut alpha = vec![1.0 / n as f64; n];

        // Calculate kernel matrix (RBF kernel)
        let kernel_matrix = self.calculate_kernel_matrix(normalized_data);

        // Simplified optimization: iterate to find support vectors
        for iteration in 0..50 {
            // Limited iterations for performance
            let mut changed = false;

            // Calculate decision values for all points
            let mut decision_values = Vec::new();
            for i in 0..n {
                let mut decision_value = 0.0;
                for j in 0..n {
                    decision_value += alpha[j] * kernel_matrix[i][j];
                }
                decision_values.push(decision_value);
            }

            // Update alpha values based on One-Class SVM objective
            // Points with high decision values should have lower alpha (normal points)
            // Points with low decision values should have higher alpha (potential outliers)
            let median_decision = {
                let mut sorted_decisions = decision_values.clone();
                sorted_decisions.sort_by(|a, b| a.partial_cmp(b).unwrap());
                sorted_decisions[n / 2]
            };

            for i in 0..n {
                let old_alpha = alpha[i];
                // Update alpha based on distance from median
                let deviation = (decision_values[i] - median_decision).abs();
                alpha[i] = (alpha[i] * (1.0 + deviation * 0.1)).min(2.0 / n as f64);

                if (old_alpha - alpha[i]).abs() > 1e-6 {
                    changed = true;
                }
            }

            if !changed {
                log::debug!("SVM training converged after {} iterations", iteration + 1);
                break;
            }
        }

        // Identify support vectors (points with significant alpha values)
        let alpha_threshold = 1e-6;
        self.support_vectors.clear();
        self.support_weights.clear();

        for i in 0..n {
            if alpha[i] > alpha_threshold {
                self.support_vectors.push(normalized_data[i].clone());
                self.support_weights.push(alpha[i]);
            }
        }

        // Calculate decision threshold (rho parameter)
        // Use the nu parameter to set the fraction of outliers
        let mut all_decision_values = Vec::new();
        for sample in normalized_data {
            let decision_value = self.calculate_decision_function(sample);
            all_decision_values.push(decision_value);
        }
        all_decision_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Set threshold such that nu fraction of training data are outliers
        let outlier_index = ((1.0 - self.nu) * n as f64) as usize;
        self.decision_threshold = all_decision_values
            .get(outlier_index.min(n - 1))
            .copied()
            .unwrap_or(0.0);

        Ok(())
    }

    #[cfg(not(feature = "ml"))]
    /// Fallback training using centroid-based approach
    fn train_fallback(&mut self, normalized_data: &[Vec<f64>]) -> DriftResult<()> {
        if normalized_data.is_empty() {
            return Ok(());
        }

        // Calculate centroid of training data
        let feature_count = normalized_data[0].len();
        let mut centroid = vec![0.0; feature_count];

        for sample in normalized_data {
            for (i, &value) in sample.iter().enumerate() {
                centroid[i] += value;
            }
        }
        for value in &mut centroid {
            *value /= normalized_data.len() as f64;
        }

        // Use centroid as the single "support vector"
        self.support_vectors = vec![centroid];
        self.support_weights = vec![1.0];

        // Calculate distances from centroid to set threshold
        let mut distances: Vec<f64> = normalized_data
            .iter()
            .map(|sample| self.rbf_kernel(sample, &self.support_vectors[0]))
            .collect();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Set threshold using nu parameter
        let outlier_index = ((1.0 - self.nu) * distances.len() as f64) as usize;
        self.decision_threshold = distances
            .get(outlier_index.min(distances.len() - 1))
            .copied()
            .unwrap_or(0.5);

        Ok(())
    }

    /// Calculate kernel matrix for all training samples
    fn calculate_kernel_matrix(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = data.len();
        let mut kernel_matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                kernel_matrix[i][j] = self.rbf_kernel(&data[i], &data[j]);
            }
        }

        kernel_matrix
    }

    /// RBF (Radial Basis Function) kernel
    fn rbf_kernel(&self, x1: &[f64], x2: &[f64]) -> f64 {
        let distance_squared = x1
            .iter()
            .zip(x2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>();

        (-self.gamma * distance_squared).exp()
    }

    /// Calculate decision function value
    fn calculate_decision_function(&self, sample: &[f64]) -> f64 {
        let mut decision_value = 0.0;

        for (sv, &weight) in self.support_vectors.iter().zip(self.support_weights.iter()) {
            decision_value += weight * self.rbf_kernel(sample, sv);
        }

        decision_value
    }

    /// Calculate anomaly score for given features
    fn calculate_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        if !self.is_trained {
            log::warn!("One-Class SVM not trained, using fallback scoring");
            return self.fallback_anomaly_score(features);
        }

        let feature_vec = self.features_to_vector(features);
        let normalized_vec = self.normalize_single(&feature_vec);

        let decision_value = self.calculate_decision_function(&normalized_vec);

        // Convert decision value to anomaly score (0-1 range)
        // Points below threshold are anomalies
        if decision_value < self.decision_threshold {
            // Anomaly - map to score between 0.5 and 1.0
            let distance_below = self.decision_threshold - decision_value;
            (0.5 + 0.5 * (distance_below / self.decision_threshold.abs().max(1.0))).min(1.0)
        } else {
            // Normal - map to score between 0.0 and 0.5
            let distance_above = decision_value - self.decision_threshold;
            (0.5 - 0.5 * (distance_above / (1.0 + self.decision_threshold.abs()))).max(0.0)
        }
    }

    /// Fallback anomaly scoring when model is not trained
    fn fallback_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        // Enhanced fallback calculation similar to Isolation Forest
        let complexity_weight = features.complexity_score * 0.25;
        let file_count_weight = (features.file_count as f64 / 10.0).min(1.0) * 0.20;
        let diversity_weight = (features.tech_diversity as f64 / 5.0).min(1.0) * 0.15;
        let pattern_weight = features.pattern_frequency * 0.15;
        let sentiment_weight = features.text_features.sentiment_score.abs() * 0.10;
        let tech_terms_weight =
            (features.text_features.tech_term_count as f64 / 20.0).min(1.0) * 0.15;

        (complexity_weight
            + file_count_weight
            + diversity_weight
            + pattern_weight
            + sentiment_weight
            + tech_terms_weight)
            .min(1.0)
    }
}

impl AnomalyModel for OneClassSVMModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let anomaly_score = self.calculate_anomaly_score(features);
        let confidence = if self.is_trained { 0.85 } else { 0.6 }; // Higher confidence when trained

        Ok(Prediction {
            confidence,
            anomaly_score,
            is_anomaly: anomaly_score > 0.6, // Threshold for anomaly classification
        })
    }

    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        if !self.is_trained {
            return Some(
                "One-Class SVM analysis using fallback scoring (model not trained)".to_string(),
            );
        }

        let mut explanations = Vec::new();

        // Analyze which features contribute most to the anomaly score
        if features.complexity_score > 0.7 {
            explanations.push("High complexity score deviates from normal architectural patterns");
        }

        if features.tech_diversity > 3 {
            explanations.push("Technology diversity exceeds typical project patterns");
        }

        if features.file_count > 5 {
            explanations.push("Large number of affected files indicates unusual change scope");
        }

        if features.pattern_frequency < 0.2 {
            explanations.push("Rare pattern detected by SVM boundary analysis");
        }

        if features.text_features.sentiment_score.abs() > 0.5 {
            explanations.push("Significant sentiment deviation in change description");
        }

        if explanations.is_empty() {
            Some(format!(
                "One-Class SVM detected deviation using {} support vectors (nu={:.2}, gamma={:.2})",
                self.support_vectors.len(),
                self.nu,
                self.gamma
            ))
        } else {
            Some(format!(
                "SVM boundary analysis ({}): {}",
                if self.support_vectors.is_empty() {
                    "fallback"
                } else {
                    "trained"
                },
                explanations.join("; ")
            ))
        }
    }

    fn serialize(&self) -> DriftResult<Vec<u8>> {
        // TODO: Implement actual serialization with serde
        Ok(format!(
            "svm_model_nu_{}_gamma_{}_sv_{}_trained_{}",
            self.nu,
            self.gamma,
            self.support_vectors.len(),
            self.is_trained
        )
        .into_bytes())
    }

    fn model_type(&self) -> ModelType {
        ModelType::OneClassSVM
    }
}

/// Local Outlier Factor model for anomaly detection
///
/// Implements the real LOF algorithm using:
/// - K-nearest neighbors identification
/// - Reachability distance computation
/// - Local reachability density (LRD) calculation
/// - LOF score as ratio of neighbor LRDs to point's LRD
/// - Proper anomaly scoring based on local density comparison
pub struct LOFModel {
    /// Number of neighbors to consider (k parameter)
    n_neighbors: usize,

    /// Training data points (feature vectors)
    training_data: Vec<Vec<f64>>,

    /// Precomputed k-distances for training data
    k_distances: Vec<f64>,

    /// Precomputed local reachability densities for training data
    lrd_values: Vec<f64>,

    /// Is the model trained
    is_trained: bool,

    /// Statistics for normalization
    feature_means: Vec<f64>,
    feature_stds: Vec<f64>,
}

impl Default for LOFModel {
    fn default() -> Self {
        Self::new()
    }
}

impl LOFModel {
    /// Create a new LOF model with default k=20
    pub fn new() -> Self {
        Self::with_neighbors(20)
    }

    /// Create LOF model with custom k value
    pub fn with_neighbors(k: usize) -> Self {
        Self {
            n_neighbors: k.max(1), // Ensure k >= 1
            training_data: Vec::new(),
            k_distances: Vec::new(),
            lrd_values: Vec::new(),
            is_trained: false,
            feature_means: Vec::new(),
            feature_stds: Vec::new(),
        }
    }

    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        // TODO: Implement actual deserialization with serde
        Ok(Self::new())
    }

    /// Train the LOF model with given data
    pub fn train(&mut self, training_features: &[DriftFeatures]) -> DriftResult<()> {
        if training_features.is_empty() {
            return Err(crate::AdrscanError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No training data provided for LOF",
            )));
        }

        // Convert features to training matrix
        let feature_matrix = self.features_to_matrix(training_features);

        // Adjust k if necessary (k should be less than number of training samples)
        if self.n_neighbors >= feature_matrix.len() {
            self.n_neighbors = (feature_matrix.len() - 1).max(1);
            log::warn!(
                "Adjusted k to {} due to limited training data",
                self.n_neighbors
            );
        }

        // Normalize features
        self.calculate_normalization_stats(&feature_matrix);
        let normalized_data = self.normalize_features(&feature_matrix);
        self.training_data = normalized_data.clone();

        // Precompute k-distances and LRD values for all training points
        self.precompute_k_distances(&normalized_data)?;
        self.precompute_lrd_values(&normalized_data)?;

        self.is_trained = true;
        log::info!(
            "LOF model trained with {} samples, k={}",
            feature_matrix.len(),
            self.n_neighbors
        );

        Ok(())
    }

    /// Convert DriftFeatures to feature matrix for ML training
    fn features_to_matrix(&self, features: &[DriftFeatures]) -> Vec<Vec<f64>> {
        features
            .iter()
            .map(|f| self.features_to_vector(f))
            .collect()
    }

    /// Convert single DriftFeatures to feature vector (same as other models)
    fn features_to_vector(&self, features: &DriftFeatures) -> Vec<f64> {
        vec![
            features.file_count as f64,
            features.lines_changed as f64,
            features.complexity_score,
            features.tech_diversity as f64,
            features.pattern_frequency,
            features.temporal_features.days_since_last,
            features.temporal_features.frequency_per_week,
            features.temporal_features.seasonal_strength,
            features.text_features.sentiment_score,
            features.text_features.tech_term_count as f64,
            features.text_features.readability_score,
            features.text_features.description_length as f64,
            features.structural_features.directory_depth as f64,
            features.structural_features.extension_diversity as f64,
            features.structural_features.coupling_strength,
            features.structural_features.cohesion_score,
        ]
    }

    /// Calculate normalization statistics
    fn calculate_normalization_stats(&mut self, data: &[Vec<f64>]) {
        if data.is_empty() {
            return;
        }

        let feature_count = data[0].len();
        self.feature_means = vec![0.0; feature_count];
        self.feature_stds = vec![1.0; feature_count];

        // Calculate means
        for sample in data {
            for (i, &value) in sample.iter().enumerate() {
                self.feature_means[i] += value;
            }
        }
        for mean in &mut self.feature_means {
            *mean /= data.len() as f64;
        }

        // Calculate standard deviations
        for sample in data {
            for (i, &value) in sample.iter().enumerate() {
                let diff = value - self.feature_means[i];
                self.feature_stds[i] += diff * diff;
            }
        }
        for std in &mut self.feature_stds {
            *std = (*std / data.len() as f64).sqrt().max(1e-8); // Avoid division by zero
        }
    }

    /// Normalize features using calculated statistics
    fn normalize_features(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        data.iter()
            .map(|sample| {
                sample
                    .iter()
                    .enumerate()
                    .map(|(i, &value)| {
                        if i < self.feature_means.len() && i < self.feature_stds.len() {
                            (value - self.feature_means[i]) / self.feature_stds[i]
                        } else {
                            value
                        }
                    })
                    .collect()
            })
            .collect()
    }

    /// Normalize a single feature vector
    fn normalize_single(&self, features: &[f64]) -> Vec<f64> {
        features
            .iter()
            .enumerate()
            .map(|(i, &value)| {
                if i < self.feature_means.len() && i < self.feature_stds.len() {
                    (value - self.feature_means[i]) / self.feature_stds[i]
                } else {
                    value
                }
            })
            .collect()
    }

    /// Precompute k-distances for all training points
    fn precompute_k_distances(&mut self, normalized_data: &[Vec<f64>]) -> DriftResult<()> {
        self.k_distances.clear();

        for i in 0..normalized_data.len() {
            let k_distance = self.calculate_k_distance(&normalized_data[i], normalized_data, i)?;
            self.k_distances.push(k_distance);
        }

        Ok(())
    }

    /// Precompute local reachability densities for all training points
    fn precompute_lrd_values(&mut self, normalized_data: &[Vec<f64>]) -> DriftResult<()> {
        self.lrd_values.clear();

        for i in 0..normalized_data.len() {
            let lrd = self.calculate_lrd(&normalized_data[i], normalized_data, i)?;
            self.lrd_values.push(lrd);
        }

        Ok(())
    }

    /// Calculate k-distance for a point (distance to its k-th nearest neighbor)
    fn calculate_k_distance(
        &self,
        point: &[f64],
        dataset: &[Vec<f64>],
        point_index: usize,
    ) -> DriftResult<f64> {
        // Find distances to all other points
        let mut distances: Vec<(f64, usize)> = Vec::new();

        for (i, other_point) in dataset.iter().enumerate() {
            if i != point_index {
                let distance = self.euclidean_distance(point, other_point);
                distances.push((distance, i));
            }
        }

        // Sort by distance
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Return k-th nearest neighbor distance
        if distances.len() >= self.n_neighbors {
            Ok(distances[self.n_neighbors - 1].0)
        } else if !distances.is_empty() {
            // If we have fewer than k neighbors, use the farthest available
            Ok(distances.last().unwrap().0)
        } else {
            Ok(1.0) // Fallback for single point
        }
    }

    /// Calculate Local Reachability Density (LRD) for a point
    fn calculate_lrd(
        &self,
        point: &[f64],
        dataset: &[Vec<f64>],
        point_index: usize,
    ) -> DriftResult<f64> {
        // Find k-nearest neighbors
        let neighbors = self.find_k_nearest_neighbors(point, dataset, point_index)?;

        if neighbors.is_empty() {
            return Ok(1.0); // Fallback LRD
        }

        // Calculate sum of reachability distances
        let mut sum_reachability_dist = 0.0;

        for &neighbor_idx in &neighbors {
            let neighbor_point = &dataset[neighbor_idx];
            let direct_distance = self.euclidean_distance(point, neighbor_point);

            // Reachability distance = max(k-distance(neighbor), direct_distance)
            let k_distance_neighbor = if neighbor_idx < self.k_distances.len() {
                self.k_distances[neighbor_idx]
            } else {
                // Fallback: calculate k-distance on the fly
                self.calculate_k_distance(neighbor_point, dataset, neighbor_idx)?
            };

            let reachability_distance = direct_distance.max(k_distance_neighbor);
            sum_reachability_dist += reachability_distance;
        }

        // LRD = k / sum(reachability_distances)
        if sum_reachability_dist > 0.0 {
            Ok(neighbors.len() as f64 / sum_reachability_dist)
        } else {
            // All neighbors at same location - use very high but finite density
            Ok(1e6) // High density for identical points
        }
    }

    /// Find k-nearest neighbors of a point
    fn find_k_nearest_neighbors(
        &self,
        point: &[f64],
        dataset: &[Vec<f64>],
        point_index: usize,
    ) -> DriftResult<Vec<usize>> {
        // Find distances to all other points
        let mut distances: Vec<(f64, usize)> = Vec::new();

        for (i, other_point) in dataset.iter().enumerate() {
            if i != point_index {
                let distance = self.euclidean_distance(point, other_point);
                distances.push((distance, i));
            }
        }

        // Sort by distance and take k nearest
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let k = self.n_neighbors.min(distances.len());
        Ok(distances.into_iter().take(k).map(|(_, idx)| idx).collect())
    }

    /// Calculate Euclidean distance between two points
    fn euclidean_distance(&self, point1: &[f64], point2: &[f64]) -> f64 {
        point1
            .iter()
            .zip(point2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Calculate LOF score for given features
    fn calculate_lof_score(&self, features: &DriftFeatures) -> f64 {
        if !self.is_trained || self.training_data.is_empty() {
            log::warn!("LOF model not trained, using fallback scoring");
            return self.fallback_lof_score(features);
        }

        let feature_vec = self.features_to_vector(features);
        let normalized_vec = self.normalize_single(&feature_vec);

        // Find k-nearest neighbors in training data
        let neighbors_result =
            self.find_k_nearest_neighbors(&normalized_vec, &self.training_data, usize::MAX);
        let neighbors = match neighbors_result {
            Ok(neighbors) => neighbors,
            Err(_) => return self.fallback_lof_score(features),
        };

        if neighbors.is_empty() {
            return self.fallback_lof_score(features);
        }

        // Calculate LRD for the query point
        let query_lrd = match self.calculate_lrd(&normalized_vec, &self.training_data, usize::MAX) {
            Ok(lrd) => lrd,
            Err(_) => return self.fallback_lof_score(features),
        };

        // Calculate LOF score as ratio of neighbor LRDs to query LRD
        let mut lrd_ratio_sum = 0.0;

        for &neighbor_idx in &neighbors {
            if neighbor_idx < self.lrd_values.len() {
                let neighbor_lrd = self.lrd_values[neighbor_idx];
                if query_lrd > 0.0 && query_lrd.is_finite() && neighbor_lrd.is_finite() {
                    lrd_ratio_sum += neighbor_lrd / query_lrd;
                } else if query_lrd > 0.0 && query_lrd.is_finite() {
                    // Neighbor has high density, query has normal density
                    lrd_ratio_sum += 1e6 / query_lrd;
                } else {
                    // Both have high density - normal case
                    lrd_ratio_sum += 1.0;
                }
            }
        }

        // LOF score = average ratio of neighbor LRDs to query LRD
        if neighbors.is_empty() {
            1.0 // Normal score
        } else {
            lrd_ratio_sum / neighbors.len() as f64
        }
    }

    /// Fallback LOF scoring when model is not trained
    fn fallback_lof_score(&self, features: &DriftFeatures) -> f64 {
        // Enhanced fallback based on feature analysis
        let mut anomaly_factors = 0.0;

        // Complexity factor
        if features.complexity_score > 0.7 {
            anomaly_factors += features.complexity_score;
        }

        // Technology diversity factor
        if features.tech_diversity > 3 {
            anomaly_factors += features.tech_diversity as f64 / 10.0;
        }

        // Pattern frequency factor (rare patterns = higher anomaly)
        if features.pattern_frequency < 0.2 {
            anomaly_factors += (0.2 - features.pattern_frequency) * 2.0;
        }

        // File count factor
        if features.file_count > 5 {
            anomaly_factors += features.file_count as f64 / 20.0;
        }

        // Text features factor
        if features.text_features.tech_term_count > 10 {
            anomaly_factors += features.text_features.tech_term_count as f64 / 50.0;
        }

        // Structural complexity factor
        if features.structural_features.coupling_strength > 0.7 {
            anomaly_factors += features.structural_features.coupling_strength * 0.5;
        }

        // LOF score: 1.0 = normal, >1.0 = anomaly
        1.0 + anomaly_factors.min(2.0) // Cap at 3.0 total
    }

    /// Calculate anomaly score for given features (0.0-1.0 range)
    fn calculate_anomaly_score(&self, features: &DriftFeatures) -> f64 {
        let lof_score = self.calculate_lof_score(features);

        // Convert LOF score to anomaly score (0.0-1.0)
        // LOF = 1.0 means normal (anomaly_score = 0.0)
        // LOF > 1.0 means anomaly (higher LOF = higher anomaly_score)
        if lof_score <= 1.0 {
            0.0 // Normal point
        } else {
            // Map LOF score to anomaly score
            // LOF 1.5 -> 0.5, LOF 2.0 -> 0.75, LOF 3.0+ -> 1.0
            ((lof_score - 1.0) / 2.0).min(1.0)
        }
    }
}

impl AnomalyModel for LOFModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let lof_score = self.calculate_lof_score(features);
        let anomaly_score = self.calculate_anomaly_score(features);
        let confidence = if self.is_trained { 0.8 } else { 0.6 };

        Ok(Prediction {
            confidence,
            anomaly_score,
            is_anomaly: lof_score > 1.5, // LOF threshold for anomaly classification
        })
    }

    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        if !self.is_trained {
            return Some("LOF analysis using fallback scoring (model not trained)".to_string());
        }

        let lof_score = self.calculate_lof_score(features);
        let mut explanations = Vec::new();

        // Analyze which features contribute most to the LOF score
        if features.complexity_score > 0.7 {
            explanations.push("High complexity score deviates from local neighborhood density");
        }

        if features.tech_diversity > 3 {
            explanations.push("Technology diversity exceeds local pattern expectations");
        }

        if features.pattern_frequency < 0.2 {
            explanations.push("Rare pattern with low local density compared to neighbors");
        }

        if features.file_count > 5 {
            explanations.push("Large change scope differs from typical local patterns");
        }

        if features.text_features.tech_term_count > 10 {
            explanations.push("High technical complexity deviates from neighborhood");
        }

        if features.structural_features.coupling_strength > 0.7 {
            explanations.push("High coupling strength unusual in local context");
        }

        if explanations.is_empty() {
            Some(format!(
                "LOF detected density deviation (score={:.2}, k={}, trained_samples={})",
                lof_score,
                self.n_neighbors,
                self.training_data.len()
            ))
        } else {
            Some(format!(
                "LOF analysis (score={:.2}): {}",
                lof_score,
                explanations.join("; ")
            ))
        }
    }

    fn serialize(&self) -> DriftResult<Vec<u8>> {
        // TODO: Implement actual serialization with serde
        Ok(format!(
            "lof_model_k_{}_samples_{}_trained_{}",
            self.n_neighbors,
            self.training_data.len(),
            self.is_trained
        )
        .into_bytes())
    }

    fn model_type(&self) -> ModelType {
        ModelType::LocalOutlierFactor
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

impl Default for StatisticalModel {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticalModel {
    pub fn new() -> Self {
        Self {
            means: vec![0.5, 2.0, 1.0], // Default means for complexity, file_count, tech_diversity
            std_devs: vec![0.2, 1.0, 0.5], // Default standard deviations
            threshold: 2.0,             // 2 standard deviations
        }
    }

    pub fn deserialize(_data: &[u8]) -> DriftResult<Self> {
        Ok(Self::new())
    }
}

impl AnomalyModel for StatisticalModel {
    fn predict(&self, features: &DriftFeatures) -> DriftResult<Prediction> {
        let feature_values = [
            features.complexity_score,
            features.file_count as f64,
            features.tech_diversity as f64,
        ];

        let z_scores: Vec<f64> = feature_values
            .iter()
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
        let feature_values = [
            ("complexity", features.complexity_score),
            ("file_count", features.file_count as f64),
            ("tech_diversity", features.tech_diversity as f64),
        ];

        let mut explanations = Vec::new();

        for (i, (name, value)) in feature_values.iter().enumerate() {
            if i < self.means.len() && i < self.std_devs.len() {
                let z_score = (value - self.means[i]).abs() / self.std_devs[i];
                if z_score > self.threshold {
                    explanations.push(format!(
                        "{name} is {z_score} standard deviations from normal"
                    ));
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

impl Default for EnsembleModel {
    fn default() -> Self {
        Self::new()
    }
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
        let predictions: Result<Vec<_>, _> = self
            .models
            .iter()
            .map(|model| model.predict(features))
            .collect();

        let predictions = predictions?;

        match self.voting_strategy {
            VotingStrategy::Majority => {
                let anomaly_count = predictions.iter().filter(|p| p.is_anomaly).count();
                let is_anomaly = anomaly_count > self.models.len() / 2;
                let avg_confidence = predictions.iter().map(|p| p.confidence).sum::<f64>()
                    / predictions.len() as f64;
                let avg_score = predictions.iter().map(|p| p.anomaly_score).sum::<f64>()
                    / predictions.len() as f64;

                Ok(Prediction {
                    confidence: avg_confidence,
                    anomaly_score: avg_score,
                    is_anomaly,
                })
            }
            VotingStrategy::WeightedAverage => {
                let weights = vec![0.6, 0.4]; // Favor isolation forest
                let weighted_score = predictions
                    .iter()
                    .zip(&weights)
                    .map(|(pred, weight)| pred.anomaly_score * weight)
                    .sum::<f64>();

                let weighted_confidence = predictions
                    .iter()
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
                let max_prediction = predictions
                    .iter()
                    .max_by(|a, b| a.anomaly_score.partial_cmp(&b.anomaly_score).unwrap())
                    .unwrap();

                Ok(max_prediction.clone())
            }
        }
    }

    fn explain(&self, features: &DriftFeatures) -> Option<String> {
        let explanations: Vec<String> = self
            .models
            .iter()
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
    use super::super::features::FeatureExtractor;
    use super::*;
    use crate::drift::{DriftCategory, DriftItem, DriftLocation, DriftSeverity};
    use std::path::PathBuf;

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

    #[test]
    fn test_one_class_svm_creation() {
        let svm = OneClassSVMModel::new();
        assert_eq!(svm.model_type(), ModelType::OneClassSVM);
        assert!(!svm.is_trained);
        assert_eq!(svm.nu, 0.1);
        assert_eq!(svm.gamma, 0.5);
    }

    #[test]
    fn test_one_class_svm_with_params() {
        let svm = OneClassSVMModel::with_params(0.2, 1.0);
        assert_eq!(svm.nu, 0.2);
        assert_eq!(svm.gamma, 1.0);

        // Test parameter clamping
        let svm_clamped = OneClassSVMModel::with_params(-0.1, 0.0);
        assert!(svm_clamped.nu > 0.0);
        assert!(svm_clamped.gamma > 0.0);
    }

    #[test]
    fn test_one_class_svm_feature_conversion() {
        let svm = OneClassSVMModel::new();
        let features = create_test_features();

        let feature_vec = svm.features_to_vector(&features);
        assert_eq!(feature_vec.len(), 16); // Expected number of features

        // Verify all features are finite numbers
        for &value in &feature_vec {
            assert!(value.is_finite());
        }
    }

    #[test]
    fn test_one_class_svm_rbf_kernel() {
        let svm = OneClassSVMModel::new();
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let vec3 = vec![4.0, 5.0, 6.0];

        // Identical vectors should have kernel value of 1.0
        let kernel_same = svm.rbf_kernel(&vec1, &vec2);
        assert!((kernel_same - 1.0).abs() < 1e-10);

        // Different vectors should have kernel value between 0 and 1
        let kernel_diff = svm.rbf_kernel(&vec1, &vec3);
        assert!(kernel_diff > 0.0 && kernel_diff < 1.0);
    }

    #[test]
    fn test_one_class_svm_normalization() {
        let mut svm = OneClassSVMModel::new();
        let test_data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        svm.calculate_normalization_stats(&test_data);
        let normalized = svm.normalize_features(&test_data);

        // Check that normalized data has correct dimensions
        assert_eq!(normalized.len(), test_data.len());
        assert_eq!(normalized[0].len(), test_data[0].len());

        // Verify normalization statistics
        assert_eq!(svm.feature_means.len(), 3);
        assert_eq!(svm.feature_stds.len(), 3);

        // Check that means are reasonable
        assert!((svm.feature_means[0] - 4.0).abs() < 1e-10); // (1+4+7)/3 = 4
        assert!((svm.feature_means[1] - 5.0).abs() < 1e-10); // (2+5+8)/3 = 5
    }

    #[test]
    fn test_one_class_svm_training() {
        let mut svm = OneClassSVMModel::new();
        let features = vec![create_test_features(); 5]; // Create 5 training samples

        let result = svm.train(&features);
        assert!(result.is_ok());
        assert!(svm.is_trained);
        assert!(!svm.training_data.is_empty());
        assert!(!svm.feature_means.is_empty());
        assert!(!svm.feature_stds.is_empty());
    }

    #[test]
    fn test_one_class_svm_training_empty_data() {
        let mut svm = OneClassSVMModel::new();
        let features = vec![];

        let result = svm.train(&features);
        assert!(result.is_err());
        assert!(!svm.is_trained);
    }

    #[test]
    fn test_one_class_svm_prediction_untrained() {
        let svm = OneClassSVMModel::new();
        let features = create_test_features();

        let prediction = svm.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);

        // Untrained model should have lower confidence
        assert!(prediction.confidence < 0.85);
    }

    #[test]
    fn test_one_class_svm_prediction_trained() {
        let mut svm = OneClassSVMModel::new();
        let training_features = vec![create_test_features(); 10];

        svm.train(&training_features).unwrap();
        let test_features = create_test_features();
        let prediction = svm.predict(&test_features).unwrap();

        assert!(prediction.confidence >= 0.8); // Higher confidence when trained
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }

    #[test]
    fn test_one_class_svm_explanation() {
        let svm = OneClassSVMModel::new();
        let features = create_test_features();

        let explanation = svm.explain(&features);
        assert!(explanation.is_some());
        let exp_text = explanation.unwrap();
        assert!(!exp_text.is_empty());
        assert!(exp_text.contains("SVM") || exp_text.contains("fallback"));
    }

    #[test]
    fn test_one_class_svm_trained_explanation() {
        let mut svm = OneClassSVMModel::new();
        let training_features = vec![create_test_features(); 5];
        svm.train(&training_features).unwrap();

        let test_features = create_test_features();
        let explanation = svm.explain(&test_features);
        assert!(explanation.is_some());

        let exp_text = explanation.unwrap();
        assert!(exp_text.contains("SVM") || exp_text.contains("boundary"));
    }

    #[test]
    fn test_one_class_svm_decision_function() {
        let mut svm = OneClassSVMModel::new();
        let training_features = vec![create_test_features(); 3];
        svm.train(&training_features).unwrap();

        // Test decision function with normalized data
        let test_vec = vec![0.0, 0.0, 0.0];
        let decision = svm.calculate_decision_function(&test_vec);
        assert!(decision.is_finite());
    }

    #[test]
    fn test_one_class_svm_anomaly_scoring() {
        let mut svm = OneClassSVMModel::new();
        let features = create_test_features();

        // Test untrained scoring (fallback)
        let score_untrained = svm.calculate_anomaly_score(&features);
        assert!((0.0..=1.0).contains(&score_untrained));

        // Train and test trained scoring
        let training_features = vec![create_test_features(); 5];
        svm.train(&training_features).unwrap();

        let score_trained = svm.calculate_anomaly_score(&features);
        assert!((0.0..=1.0).contains(&score_trained));
    }

    #[test]
    fn test_one_class_svm_serialization() {
        let svm = OneClassSVMModel::new();
        let serialized = svm.serialize().unwrap();
        assert!(!serialized.is_empty());

        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("svm_model"));
        assert!(serialized_str.contains("nu_"));
        assert!(serialized_str.contains("gamma_"));
    }

    #[test]
    fn test_one_class_svm_kernel_matrix() {
        let svm = OneClassSVMModel::new();
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];

        let kernel_matrix = svm.calculate_kernel_matrix(&data);
        assert_eq!(kernel_matrix.len(), 3);
        assert_eq!(kernel_matrix[0].len(), 3);

        // Diagonal elements should be 1.0 (RBF kernel of identical vectors)
        for i in 0..3 {
            assert!((kernel_matrix[i][i] - 1.0).abs() < 1e-10);
        }

        // Matrix should be symmetric
        for i in 0..3 {
            for j in 0..3 {
                assert!((kernel_matrix[i][j] - kernel_matrix[j][i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_one_class_svm_ensemble_integration() {
        // Test that SVM works in ensemble
        let ensemble = EnsembleModel::new();
        let features = create_test_features();

        let prediction = ensemble.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }

    // LOF Model Tests

    #[test]
    fn test_lof_model_creation() {
        let lof = LOFModel::new();
        assert_eq!(lof.model_type(), ModelType::LocalOutlierFactor);
        assert!(!lof.is_trained);
        assert_eq!(lof.n_neighbors, 20);

        let lof_custom = LOFModel::with_neighbors(10);
        assert_eq!(lof_custom.n_neighbors, 10);

        // Test k >= 1 constraint
        let lof_zero = LOFModel::with_neighbors(0);
        assert_eq!(lof_zero.n_neighbors, 1);
    }

    #[test]
    fn test_lof_feature_conversion() {
        let lof = LOFModel::new();
        let features = create_test_features();

        let feature_vec = lof.features_to_vector(&features);
        assert_eq!(feature_vec.len(), 16); // Expected number of features

        // Verify all features are finite numbers
        for &value in &feature_vec {
            assert!(value.is_finite());
        }
    }

    #[test]
    fn test_lof_euclidean_distance() {
        let lof = LOFModel::new();
        let vec1 = vec![1.0, 2.0, 3.0];
        let vec2 = vec![1.0, 2.0, 3.0];
        let vec3 = vec![4.0, 5.0, 6.0];

        // Identical vectors should have distance 0
        let distance_same = lof.euclidean_distance(&vec1, &vec2);
        assert!((distance_same - 0.0).abs() < 1e-10);

        // Different vectors should have positive distance
        let distance_diff = lof.euclidean_distance(&vec1, &vec3);
        assert!(distance_diff > 0.0);

        // Distance should be symmetric
        let distance_reverse = lof.euclidean_distance(&vec3, &vec1);
        assert!((distance_diff - distance_reverse).abs() < 1e-10);
    }

    #[test]
    fn test_lof_normalization() {
        let mut lof = LOFModel::new();
        let test_data = vec![
            vec![1.0, 10.0, 100.0],
            vec![2.0, 20.0, 200.0],
            vec![3.0, 30.0, 300.0],
        ];

        lof.calculate_normalization_stats(&test_data);
        let normalized = lof.normalize_features(&test_data);

        // Check dimensions preserved
        assert_eq!(normalized.len(), test_data.len());
        assert_eq!(normalized[0].len(), test_data[0].len());

        // Verify normalization statistics
        assert_eq!(lof.feature_means.len(), 3);
        assert_eq!(lof.feature_stds.len(), 3);

        // Check means are reasonable (should be 2.0, 20.0, 200.0)
        assert!((lof.feature_means[0] - 2.0).abs() < 1e-10);
        assert!((lof.feature_means[1] - 20.0).abs() < 1e-10);
        assert!((lof.feature_means[2] - 200.0).abs() < 1e-10);

        // Test single vector normalization
        let single_vec = vec![2.0, 20.0, 200.0]; // Should normalize to ~0
        let normalized_single = lof.normalize_single(&single_vec);
        for &value in &normalized_single {
            assert!(value.abs() < 1e-6); // Should be close to 0 (mean values)
        }
    }

    #[test]
    fn test_lof_k_distance_calculation() {
        let lof = LOFModel::with_neighbors(2);
        let dataset = vec![
            vec![0.0, 0.0], // Point 0
            vec![1.0, 0.0], // Point 1 (distance 1 from point 0)
            vec![2.0, 0.0], // Point 2 (distance 2 from point 0)
            vec![3.0, 0.0], // Point 3 (distance 3 from point 0)
        ];

        // k-distance for point 0 with k=2 should be distance to 2nd nearest neighbor
        let k_dist = lof.calculate_k_distance(&dataset[0], &dataset, 0).unwrap();
        assert!((k_dist - 2.0).abs() < 1e-10); // 2nd nearest is at distance 2

        // Test with insufficient neighbors
        let small_dataset = vec![vec![0.0, 0.0], vec![1.0, 0.0]];
        let k_dist_small = lof
            .calculate_k_distance(&small_dataset[0], &small_dataset, 0)
            .unwrap();
        assert!((k_dist_small - 1.0).abs() < 1e-10); // Only one neighbor
    }

    #[test]
    fn test_lof_find_neighbors() {
        let lof = LOFModel::with_neighbors(2);
        let dataset = vec![
            vec![0.0, 0.0],  // Point 0
            vec![1.0, 0.0],  // Point 1
            vec![2.0, 0.0],  // Point 2
            vec![10.0, 0.0], // Point 3 (far away)
        ];

        let neighbors = lof
            .find_k_nearest_neighbors(&dataset[0], &dataset, 0)
            .unwrap();
        assert_eq!(neighbors.len(), 2); // Should find 2 neighbors
        assert!(neighbors.contains(&1)); // Point 1 should be closest
        assert!(neighbors.contains(&2)); // Point 2 should be second closest
        assert!(!neighbors.contains(&3)); // Point 3 should not be in k=2 neighbors
    }

    #[test]
    fn test_lof_lrd_calculation() {
        let mut lof = LOFModel::with_neighbors(2);
        let dataset = vec![
            vec![0.0, 0.0], // Dense cluster
            vec![0.1, 0.0],
            vec![0.0, 0.1],
            vec![10.0, 10.0], // Isolated point
        ];

        // Precompute k-distances for LRD calculation
        lof.precompute_k_distances(&dataset).unwrap();

        // LRD for point in dense cluster should be high
        let lrd_dense = lof.calculate_lrd(&dataset[0], &dataset, 0).unwrap();
        assert!(lrd_dense > 0.0);
        assert!(lrd_dense.is_finite());

        // LRD for isolated point should be lower
        let lrd_isolated = lof.calculate_lrd(&dataset[3], &dataset, 3).unwrap();
        assert!(lrd_isolated > 0.0);
        assert!(lrd_isolated.is_finite());
        assert!(lrd_isolated < lrd_dense); // Isolated point should have lower density
    }

    #[test]
    fn test_lof_training() {
        let mut lof = LOFModel::with_neighbors(3);
        let features = vec![create_test_features(); 10]; // Create 10 training samples

        let result = lof.train(&features);
        assert!(result.is_ok());
        assert!(lof.is_trained);
        assert_eq!(lof.training_data.len(), 10);
        assert_eq!(lof.k_distances.len(), 10);
        assert_eq!(lof.lrd_values.len(), 10);
        assert!(!lof.feature_means.is_empty());
        assert!(!lof.feature_stds.is_empty());

        // Verify precomputed values are reasonable
        for &k_dist in &lof.k_distances {
            assert!(k_dist >= 0.0);
            assert!(k_dist.is_finite());
        }

        for &lrd in &lof.lrd_values {
            assert!(lrd > 0.0);
            assert!(lrd.is_finite());
        }
    }

    #[test]
    fn test_lof_training_k_adjustment() {
        let mut lof = LOFModel::with_neighbors(10); // k larger than training data
        let features = vec![create_test_features(); 5]; // Only 5 training samples

        let result = lof.train(&features);
        assert!(result.is_ok());
        assert!(lof.n_neighbors < 5); // k should be adjusted to < 5
        assert!(lof.is_trained);
    }

    #[test]
    fn test_lof_training_empty_data() {
        let mut lof = LOFModel::new();
        let features = vec![];

        let result = lof.train(&features);
        assert!(result.is_err());
        assert!(!lof.is_trained);
    }

    #[test]
    fn test_lof_prediction_untrained() {
        let lof = LOFModel::new();
        let features = create_test_features();

        let prediction = lof.predict(&features).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);

        // Untrained model should have lower confidence
        assert!(prediction.confidence < 0.8);
    }

    #[test]
    fn test_lof_prediction_trained() {
        let mut lof = LOFModel::with_neighbors(3);
        let training_features = vec![create_test_features(); 10];

        lof.train(&training_features).unwrap();
        let test_features = create_test_features();
        let prediction = lof.predict(&test_features).unwrap();

        assert!(prediction.confidence >= 0.8); // Higher confidence when trained
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);
    }

    #[test]
    fn test_lof_score_calculation() {
        let mut lof = LOFModel::with_neighbors(2);

        // Create training data with clear normal and anomalous patterns
        let mut training_features = Vec::new();

        // Normal cluster (low complexity, low diversity)
        for _ in 0..8 {
            let mut features = create_test_features();
            features.complexity_score = 0.3;
            features.tech_diversity = 1;
            features.pattern_frequency = 0.8;
            training_features.push(features);
        }

        // Train the model
        lof.train(&training_features).unwrap();

        // Test normal point (should have LOF ~1.0)
        let mut normal_features = create_test_features();
        normal_features.complexity_score = 0.3;
        normal_features.tech_diversity = 1;
        normal_features.pattern_frequency = 0.8;

        let normal_score = lof.calculate_lof_score(&normal_features);
        assert!((0.1..=5.0).contains(&normal_score)); // Should be reasonable (broader range for real LOF)

        // Test anomalous point (should have higher LOF score)
        let mut anomaly_features = create_test_features();
        anomaly_features.complexity_score = 0.9;
        anomaly_features.tech_diversity = 5;
        anomaly_features.pattern_frequency = 0.1;

        let anomaly_score = lof.calculate_lof_score(&anomaly_features);
        assert!(anomaly_score >= 0.5); // Should detect as different
    }

    #[test]
    fn test_lof_anomaly_score_conversion() {
        let lof = LOFModel::new();

        // Test LOF to anomaly score conversion
        let mut features = create_test_features();

        // Normal case (LOF ~1.0) should give low anomaly score
        features.complexity_score = 0.2;
        features.tech_diversity = 1;
        let normal_anomaly = lof.calculate_anomaly_score(&features);
        assert!((0.0..=1.0).contains(&normal_anomaly));

        // High complexity case should give higher anomaly score
        features.complexity_score = 0.9;
        features.tech_diversity = 5;
        let high_anomaly = lof.calculate_anomaly_score(&features);
        assert!((0.0..=1.0).contains(&high_anomaly));
        assert!(high_anomaly >= normal_anomaly); // Should be higher
    }

    #[test]
    fn test_lof_fallback_scoring() {
        let lof = LOFModel::new();

        // Test fallback scoring with various feature combinations
        let mut features = create_test_features();

        // High complexity features should increase LOF score
        features.complexity_score = 0.9;
        features.tech_diversity = 5;
        features.pattern_frequency = 0.1;
        features.file_count = 10;
        features.text_features.tech_term_count = 15;
        features.structural_features.coupling_strength = 0.8;

        let fallback_score = lof.fallback_lof_score(&features);
        assert!(fallback_score > 1.0); // Should detect as anomaly
        assert!(fallback_score <= 3.0); // Should be capped

        // Normal features should give score ~1.0
        features.complexity_score = 0.3;
        features.tech_diversity = 1;
        features.pattern_frequency = 0.8;
        features.file_count = 2;
        features.text_features.tech_term_count = 3;
        features.structural_features.coupling_strength = 0.3;

        let normal_score = lof.fallback_lof_score(&features);
        assert!((1.0..=1.5).contains(&normal_score)); // Should be near normal
    }

    #[test]
    fn test_lof_explanation() {
        let lof = LOFModel::new();
        let features = create_test_features();

        // Test untrained explanation
        let explanation = lof.explain(&features);
        assert!(explanation.is_some());
        let exp_text = explanation.unwrap();
        assert!(!exp_text.is_empty());
        assert!(exp_text.contains("fallback"));

        // Test trained explanation
        let mut trained_lof = LOFModel::with_neighbors(3);
        let training_features = vec![create_test_features(); 5];
        trained_lof.train(&training_features).unwrap();

        let trained_explanation = trained_lof.explain(&features);
        assert!(trained_explanation.is_some());
        let trained_exp_text = trained_explanation.unwrap();
        assert!(trained_exp_text.contains("LOF") || trained_exp_text.contains("score"));
    }

    #[test]
    fn test_lof_serialization() {
        let lof = LOFModel::with_neighbors(15);
        let serialized = lof.serialize().unwrap();
        assert!(!serialized.is_empty());

        let serialized_str = String::from_utf8(serialized).unwrap();
        assert!(serialized_str.contains("lof_model"));
        assert!(serialized_str.contains("k_15"));
        assert!(serialized_str.contains("trained_false"));

        // Test serialization after training
        let mut trained_lof = LOFModel::with_neighbors(5);
        let training_features = vec![create_test_features(); 8];
        trained_lof.train(&training_features).unwrap();

        let trained_serialized = trained_lof.serialize().unwrap();
        let trained_str = String::from_utf8(trained_serialized).unwrap();
        assert!(trained_str.contains("samples_8"));
        assert!(trained_str.contains("trained_true"));
    }

    #[test]
    fn test_lof_model_type() {
        let lof = LOFModel::new();
        assert_eq!(lof.model_type(), ModelType::LocalOutlierFactor);
    }

    #[test]
    fn test_lof_edge_cases() {
        let mut lof = LOFModel::with_neighbors(1);

        // Single training sample
        let single_feature = vec![create_test_features()];
        let result = lof.train(&single_feature);
        assert!(result.is_ok());
        assert!(lof.is_trained);

        // Prediction with single training sample should work
        let prediction = lof.predict(&create_test_features()).unwrap();
        assert!(prediction.confidence > 0.0);
        assert!(prediction.anomaly_score >= 0.0 && prediction.anomaly_score <= 1.0);

        // Test with identical training samples
        let mut identical_lof = LOFModel::with_neighbors(2);
        let identical_features = vec![create_test_features(); 5]; // All identical
        let identical_result = identical_lof.train(&identical_features);
        assert!(identical_result.is_ok());

        // Should handle identical samples gracefully
        let identical_prediction = identical_lof.predict(&create_test_features()).unwrap();
        assert!(identical_prediction.confidence > 0.0);
    }

    #[test]
    fn test_lof_performance_characteristics() {
        let mut lof = LOFModel::with_neighbors(5);

        // Test with larger dataset
        let large_dataset = vec![create_test_features(); 50];
        let start_time = std::time::Instant::now();
        let result = lof.train(&large_dataset);
        let training_time = start_time.elapsed();

        assert!(result.is_ok());
        assert!(training_time.as_secs() < 5); // Should complete in reasonable time

        // Test prediction performance
        let start_pred = std::time::Instant::now();
        let prediction = lof.predict(&create_test_features()).unwrap();
        let pred_time = start_pred.elapsed();

        assert!(prediction.confidence > 0.0);
        assert!(pred_time.as_millis() < 100); // Should be fast
    }
}
