//! Feature extraction for ML-enhanced drift detection
//!
//! Converts drift detection results into numerical features suitable
//! for machine learning algorithms.

use crate::drift::{DriftItem, DriftResult};
use std::collections::HashMap;

/// Features extracted from drift items for ML analysis
#[derive(Debug, Clone, Default)]
pub struct DriftFeatures {
    /// Number of files affected by this drift
    pub file_count: usize,

    /// Complexity score (0.0-1.0)
    pub complexity_score: f64,

    /// Number of lines changed
    pub lines_changed: usize,

    /// Technology diversity (number of different technologies)
    pub tech_diversity: usize,

    /// Pattern frequency (how often this pattern appears)
    pub pattern_frequency: f64,

    /// Temporal features (time-based patterns)
    pub temporal_features: TemporalFeatures,

    /// Text features (from descriptions and titles)
    pub text_features: TextFeatures,

    /// Structural features (code organization patterns)
    pub structural_features: StructuralFeatures,
}

/// Time-based features
#[derive(Debug, Clone, Default)]
pub struct TemporalFeatures {
    /// Days since last similar drift
    pub days_since_last: f64,

    /// Frequency of this type of drift (per week)
    pub frequency_per_week: f64,

    /// Seasonal pattern strength (0.0-1.0)
    pub seasonal_strength: f64,

    /// Time-based drift velocity (changes per day)
    pub drift_velocity: f64,

    /// Temporal clustering score (0.0-1.0)
    pub clustering_score: f64,

    /// Recency bias factor (0.0-1.0)
    pub recency_factor: f64,
}

/// Text-based features from drift descriptions
#[derive(Debug, Clone, Default)]
pub struct TextFeatures {
    /// Sentiment score of description (-1.0 to 1.0)
    pub sentiment_score: f64,

    /// Number of technical terms
    pub tech_term_count: usize,

    /// Readability score (0.0-1.0)
    pub readability_score: f64,

    /// Description length
    pub description_length: usize,
}

/// Code structure features
#[derive(Debug, Clone, Default)]
pub struct StructuralFeatures {
    /// Directory depth
    pub directory_depth: usize,

    /// File extension diversity
    pub extension_diversity: usize,

    /// Coupling strength (0.0-1.0)
    pub coupling_strength: f64,

    /// Cohesion score (0.0-1.0)
    pub cohesion_score: f64,
}

/// Feature extractor for converting drift items to ML features
pub struct FeatureExtractor {
    /// Historical data for temporal features
    historical_data: Vec<DriftItem>,

    /// Technology pattern cache
    tech_patterns: HashMap<String, f64>,

    /// Configuration for feature extraction
    config: FeatureConfig,
}

/// Configuration for feature extraction
#[derive(Debug, Clone)]
pub struct FeatureConfig {
    /// Enable temporal features
    pub enable_temporal: bool,

    /// Enable text analysis features
    pub enable_text_analysis: bool,

    /// Enable structural analysis
    pub enable_structural: bool,

    /// Window size for temporal analysis (days)
    pub temporal_window_days: usize,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            enable_temporal: true,
            enable_text_analysis: true,
            enable_structural: true,
            temporal_window_days: 30,
        }
    }
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureExtractor {
    /// Create a new feature extractor
    pub fn new() -> Self {
        Self::with_config(FeatureConfig::default())
    }

    /// Create feature extractor with custom configuration
    pub fn with_config(config: FeatureConfig) -> Self {
        Self {
            historical_data: Vec::new(),
            tech_patterns: HashMap::new(),
            config,
        }
    }

    /// Extract features from a drift item
    pub fn extract_features(&self, drift_item: &DriftItem) -> DriftResult<DriftFeatures> {
        let mut features = DriftFeatures::default();

        // Basic features
        features.file_count = 1; // Single item for now
        features.lines_changed = self.estimate_lines_changed(drift_item);
        features.tech_diversity = self.calculate_tech_diversity(drift_item);
        features.complexity_score = self.calculate_complexity_score(drift_item);
        features.pattern_frequency = self.calculate_pattern_frequency(drift_item);

        // Temporal features
        if self.config.enable_temporal {
            features.temporal_features = self.extract_temporal_features(drift_item);
        }

        // Text features
        if self.config.enable_text_analysis {
            features.text_features = self.extract_text_features(drift_item);
        }

        // Structural features
        if self.config.enable_structural {
            features.structural_features = self.extract_structural_features(drift_item);
        }

        Ok(features)
    }

    /// Add historical data for temporal analysis
    pub fn add_historical_data(&mut self, drift_items: Vec<DriftItem>) {
        self.historical_data.extend(drift_items);

        // Update technology pattern cache
        self.update_tech_patterns();
    }

    /// Estimate lines changed based on drift severity and category
    fn estimate_lines_changed(&self, drift_item: &DriftItem) -> usize {
        use crate::drift::{DriftCategory, DriftSeverity};

        let base_lines = match drift_item.category {
            DriftCategory::NewTechnology => 50,
            DriftCategory::PatternViolation => 30,
            DriftCategory::Configuration => 10,
            DriftCategory::Other => 5,
            DriftCategory::ConflictingTechnology => 40,
            DriftCategory::DeprecatedTechnology => 35,
            DriftCategory::MissingComponent => 25,
            DriftCategory::Security => 60,
            DriftCategory::Performance => 45,
            DriftCategory::Database => 30,
            DriftCategory::Infrastructure => 40,
            DriftCategory::Framework => 35,
        };

        let severity_multiplier = match drift_item.severity {
            DriftSeverity::Low => 0.5,
            DriftSeverity::Medium => 1.0,
            DriftSeverity::High => 2.0,
            DriftSeverity::Critical => 3.0,
            DriftSeverity::Info => 0.1,
        };

        (base_lines as f64 * severity_multiplier) as usize
    }

    /// Calculate technology diversity from drift item
    fn calculate_tech_diversity(&self, drift_item: &DriftItem) -> usize {
        // Simple heuristic: count unique technology keywords
        let tech_keywords = [
            "react",
            "vue",
            "angular",
            "typescript",
            "javascript",
            "rust",
            "python",
            "java",
            "go",
            "kotlin",
            "docker",
            "kubernetes",
            "aws",
            "gcp",
            "azure",
            "postgres",
            "mysql",
            "mongodb",
            "redis",
            "elasticsearch",
        ];

        let text = format!("{} {}", drift_item.title, drift_item.description).to_lowercase();

        tech_keywords
            .iter()
            .filter(|&keyword| text.contains(keyword))
            .count()
    }

    /// Calculate complexity score based on multiple factors
    fn calculate_complexity_score(&self, drift_item: &DriftItem) -> f64 {
        let mut score = 0.0;

        // Severity contribution
        score += match drift_item.severity {
            crate::drift::DriftSeverity::Low => 0.2,
            crate::drift::DriftSeverity::Medium => 0.4,
            crate::drift::DriftSeverity::High => 0.7,
            crate::drift::DriftSeverity::Critical => 1.0,
            crate::drift::DriftSeverity::Info => 0.1,
        };

        // Category contribution
        score += match drift_item.category {
            crate::drift::DriftCategory::NewTechnology => 0.8,
            crate::drift::DriftCategory::PatternViolation => 0.6,
            crate::drift::DriftCategory::Configuration => 0.3,
            crate::drift::DriftCategory::Other => 0.2,
            crate::drift::DriftCategory::ConflictingTechnology => 0.9,
            crate::drift::DriftCategory::DeprecatedTechnology => 0.7,
            crate::drift::DriftCategory::MissingComponent => 0.8,
            crate::drift::DriftCategory::Security => 1.0,
            crate::drift::DriftCategory::Performance => 0.7,
            crate::drift::DriftCategory::Database => 0.6,
            crate::drift::DriftCategory::Infrastructure => 0.8,
            crate::drift::DriftCategory::Framework => 0.7,
        };

        // Description complexity (length and technical terms)
        let desc_complexity = (drift_item.description.len() as f64 / 500.0).min(0.5);
        score += desc_complexity;

        // Normalize to 0.0-1.0 range
        (score / 2.5).min(1.0)
    }

    /// Calculate pattern frequency
    fn calculate_pattern_frequency(&self, drift_item: &DriftItem) -> f64 {
        // Look for similar patterns in historical data
        let similar_count = self
            .historical_data
            .iter()
            .filter(|item| {
                item.category == drift_item.category && item.severity == drift_item.severity
            })
            .count();

        if self.historical_data.is_empty() {
            0.5 // Default frequency for new patterns
        } else {
            similar_count as f64 / self.historical_data.len() as f64
        }
    }

    /// Extract temporal features
    fn extract_temporal_features(&self, drift_item: &DriftItem) -> TemporalFeatures {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let days_since_last = self.calculate_days_since_similar(drift_item, current_time);
        let frequency_per_week = self.calculate_frequency_per_week(drift_item, current_time);
        let seasonal_strength = self.calculate_seasonal_strength(drift_item, current_time);
        let drift_velocity = self.calculate_drift_velocity(drift_item, current_time);
        let clustering_score = self.calculate_temporal_clustering(drift_item, current_time);
        let recency_factor = self.calculate_recency_factor(drift_item, current_time);

        TemporalFeatures {
            days_since_last,
            frequency_per_week,
            seasonal_strength,
            drift_velocity,
            clustering_score,
            recency_factor,
        }
    }

    /// Extract text features from drift descriptions
    fn extract_text_features(&self, drift_item: &DriftItem) -> TextFeatures {
        let description = &drift_item.description;

        TextFeatures {
            sentiment_score: self.calculate_sentiment(description),
            tech_term_count: self.count_tech_terms(description),
            readability_score: self.calculate_readability(description),
            description_length: description.len(),
        }
    }

    /// Extract structural features
    fn extract_structural_features(&self, drift_item: &DriftItem) -> StructuralFeatures {
        let file_path = &drift_item.location.file_path;

        StructuralFeatures {
            directory_depth: file_path.components().count(),
            extension_diversity: 1, // Single file for now
            coupling_strength: 0.5, // Default value
            cohesion_score: 0.7,    // Default value
        }
    }

    /// Simple sentiment analysis (placeholder)
    fn calculate_sentiment(&self, text: &str) -> f64 {
        let positive_words = ["good", "great", "excellent", "improve", "better", "optimal"];
        let negative_words = [
            "bad", "poor", "terrible", "problem", "issue", "bug", "error",
        ];

        let text_lower = text.to_lowercase();
        let positive_count = positive_words
            .iter()
            .filter(|&word| text_lower.contains(word))
            .count();
        let negative_count = negative_words
            .iter()
            .filter(|&word| text_lower.contains(word))
            .count();

        if positive_count + negative_count == 0 {
            0.0 // Neutral
        } else {
            (positive_count as f64 - negative_count as f64)
                / (positive_count + negative_count) as f64
        }
    }

    /// Count technical terms in text
    fn count_tech_terms(&self, text: &str) -> usize {
        let tech_terms = [
            "api",
            "database",
            "service",
            "component",
            "module",
            "library",
            "framework",
            "architecture",
            "pattern",
            "interface",
            "protocol",
            "algorithm",
            "optimization",
            "performance",
            "scalability",
            "security",
        ];

        let text_lower = text.to_lowercase();
        tech_terms
            .iter()
            .filter(|&term| text_lower.contains(term))
            .count()
    }

    /// Calculate readability score (simplified)
    fn calculate_readability(&self, text: &str) -> f64 {
        if text.is_empty() {
            return 0.0;
        }

        let word_count = text.split_whitespace().count();
        let sentence_count = text.matches('.').count().max(1);
        let avg_words_per_sentence = word_count as f64 / sentence_count as f64;

        // Simple readability heuristic (lower is more readable)
        // Normalize to 0.0-1.0 where 1.0 is most readable
        (20.0 - avg_words_per_sentence.min(20.0)) / 20.0
    }

    /// Update technology pattern cache
    fn update_tech_patterns(&mut self) {
        self.tech_patterns.clear();

        for item in &self.historical_data {
            let tech_diversity = self.calculate_tech_diversity(item);
            *self.tech_patterns.entry(item.title.clone()).or_insert(0.0) += tech_diversity as f64;
        }

        // Normalize patterns
        let max_count = self
            .tech_patterns
            .values()
            .fold(0.0_f64, |max, &val| max.max(val));
        if max_count > 0.0 {
            for value in self.tech_patterns.values_mut() {
                *value /= max_count;
            }
        }
    }

    /// Calculate days since last similar drift
    fn calculate_days_since_similar(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        let similar_items: Vec<_> = self
            .historical_data
            .iter()
            .filter(|item| {
                item.category == drift_item.category
                    || self.calculate_similarity_score(item, drift_item) > 0.7
            })
            .collect();

        if similar_items.is_empty() {
            return f64::INFINITY; // No similar items found
        }

        // Find the most recent similar item
        // For now, use a mock timestamp based on item order
        let days_per_item = 7.0; // Assume items are spaced 7 days apart
        let most_recent_index = self.historical_data.len() - 1;
        
        for (index, _item) in similar_items.iter().enumerate() {
            if index == most_recent_index {
                return days_per_item * (self.historical_data.len() - index) as f64;
            }
        }

        30.0 // Default to 30 days if no recent similar items
    }

    /// Calculate frequency of similar drifts per week
    fn calculate_frequency_per_week(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        let window_days = self.config.temporal_window_days as f64;
        let similar_count = self
            .historical_data
            .iter()
            .filter(|item| {
                item.category == drift_item.category
                    || self.calculate_similarity_score(item, drift_item) > 0.6
            })
            .count();

        if window_days <= 0.0 {
            return 0.0;
        }

        let weeks_in_window = window_days / 7.0;
        similar_count as f64 / weeks_in_window
    }

    /// Calculate seasonal pattern strength
    fn calculate_seasonal_strength(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        // Group historical items by approximate time periods
        let similar_items: Vec<_> = self
            .historical_data
            .iter()
            .filter(|item| item.category == drift_item.category)
            .collect();

        if similar_items.len() < 4 {
            return 0.0; // Need at least 4 data points for seasonal analysis
        }

        // Simple seasonal detection: look for patterns in item distribution
        let total_items = similar_items.len();
        let items_per_quarter = total_items / 4;
        
        // Calculate variance in quarterly distribution
        let quarters = [items_per_quarter; 4];
        let mean = items_per_quarter as f64;
        let variance: f64 = quarters
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
            / 4.0;

        // Convert variance to strength score (0.0-1.0)
        (variance / (mean + 1.0)).min(1.0)
    }

    /// Calculate drift velocity (rate of change)
    fn calculate_drift_velocity(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        let window_days = self.config.temporal_window_days as f64;
        let recent_items: Vec<_> = self
            .historical_data
            .iter()
            .rev() // Most recent first
            .take((window_days / 7.0) as usize) // Approximate items in window
            .filter(|item| item.category == drift_item.category)
            .collect();

        if recent_items.len() < 2 {
            return 0.0;
        }

        // Calculate velocity as items per day
        recent_items.len() as f64 / window_days
    }

    /// Calculate temporal clustering score
    fn calculate_temporal_clustering(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        let similar_items: Vec<_> = self
            .historical_data
            .iter()
            .enumerate()
            .filter(|(_, item)| item.category == drift_item.category)
            .collect();

        if similar_items.len() < 3 {
            return 0.0;
        }

        // Calculate clustering by looking at gaps between similar items
        let mut gaps = Vec::new();
        for window in similar_items.windows(2) {
            let gap = window[1].0 - window[0].0;
            gaps.push(gap);
        }

        // Calculate coefficient of variation for gaps
        let mean_gap = gaps.iter().sum::<usize>() as f64 / gaps.len() as f64;
        let variance = gaps
            .iter()
            .map(|&gap| (gap as f64 - mean_gap).powi(2))
            .sum::<f64>()
            / gaps.len() as f64;
        
        let std_dev = variance.sqrt();
        
        // Higher clustering = lower coefficient of variation
        if mean_gap > 0.0 {
            (1.0 - (std_dev / mean_gap)).max(0.0)
        } else {
            0.0
        }
    }

    /// Calculate recency factor (weight for recent items)
    fn calculate_recency_factor(&self, drift_item: &DriftItem, _current_time: u64) -> f64 {
        // Find the most recent similar item
        let similar_items: Vec<_> = self
            .historical_data
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.category == drift_item.category
                    || self.calculate_similarity_score(item, drift_item) > 0.8
            })
            .collect();

        if similar_items.is_empty() {
            return 0.0;
        }

        // Get the index of the most recent similar item
        let most_recent_index = similar_items
            .iter()
            .max_by_key(|(index, _)| *index)
            .map(|(index, _)| *index)
            .unwrap_or(0);

        // Calculate recency factor (1.0 = most recent, 0.0 = oldest)
        let total_items = self.historical_data.len();
        if total_items > 0 {
            most_recent_index as f64 / total_items as f64
        } else {
            0.0
        }
    }

    /// Calculate similarity score between two drift items
    fn calculate_similarity_score(&self, item1: &DriftItem, item2: &DriftItem) -> f64 {
        let mut score = 0.0;

        // Category similarity (40% weight)
        if item1.category == item2.category {
            score += 0.4;
        }

        // Severity similarity (30% weight)
        let severity_score = match (&item1.severity, &item2.severity) {
            (s1, s2) if s1 == s2 => 0.3,
            (crate::drift::DriftSeverity::High, crate::drift::DriftSeverity::Critical)
            | (crate::drift::DriftSeverity::Critical, crate::drift::DriftSeverity::High) => 0.2,
            (crate::drift::DriftSeverity::Medium, crate::drift::DriftSeverity::High)
            | (crate::drift::DriftSeverity::High, crate::drift::DriftSeverity::Medium) => 0.15,
            _ => 0.0,
        };
        score += severity_score;

        // Title/description similarity (30% weight)
        let title1_lower = item1.title.to_lowercase();
        let title2_lower = item2.title.to_lowercase();
        let keywords1: std::collections::HashSet<_> = title1_lower
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();
        let keywords2: std::collections::HashSet<_> = title2_lower
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        let common_keywords = keywords1.intersection(&keywords2).count();
        let total_keywords = keywords1.union(&keywords2).count();
        
        if total_keywords > 0 {
            let text_similarity = common_keywords as f64 / total_keywords as f64;
            score += text_similarity * 0.3;
        }

        score.min(1.0) // Cap at 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drift::{DriftCategory, DriftLocation, DriftSeverity};
    use std::path::PathBuf;

    fn create_test_drift_item() -> DriftItem {
        DriftItem::new(
            "test".to_string(),
            DriftSeverity::Medium,
            DriftCategory::NewTechnology,
            "Use React for frontend".to_string(),
            "We need to implement a modern frontend using React framework for better user experience".to_string(),
            DriftLocation::new(PathBuf::from("src/frontend/components/App.tsx")),
        )
    }

    #[test]
    fn test_feature_extraction() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();

        let features = extractor.extract_features(&drift_item).unwrap();

        assert_eq!(features.file_count, 1);
        assert!(features.complexity_score > 0.0);
        assert!(features.tech_diversity > 0);
        assert!(features.lines_changed > 0);
    }

    #[test]
    fn test_tech_diversity_calculation() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();

        let diversity = extractor.calculate_tech_diversity(&drift_item);
        assert!(diversity > 0); // Should detect "react" keyword
    }

    #[test]
    fn test_complexity_score() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();

        let score = extractor.calculate_complexity_score(&drift_item);
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn test_sentiment_analysis() {
        let extractor = FeatureExtractor::new();

        let positive_score = extractor.calculate_sentiment("This is a great improvement");
        assert!(positive_score > 0.0);

        let negative_score = extractor.calculate_sentiment("This is a terrible problem");
        assert!(negative_score < 0.0);

        let neutral_score = extractor.calculate_sentiment("This is a change");
        assert_eq!(neutral_score, 0.0);
    }

    #[test]
    fn test_tech_term_counting() {
        let extractor = FeatureExtractor::new();

        let count =
            extractor.count_tech_terms("This API uses a database service with security patterns");
        assert!(count >= 4); // Should find api, database, service, security
    }

    #[test]
    fn test_readability_calculation() {
        let extractor = FeatureExtractor::new();

        let simple_text = "This is simple. Easy to read.";
        let complex_text = "This is an extremely complex sentence with many subordinate clauses and technical jargon that makes it difficult to understand.";

        let simple_score = extractor.calculate_readability(simple_text);
        let complex_score = extractor.calculate_readability(complex_text);

        assert!(simple_score > complex_score);
    }

    #[test]
    fn test_historical_data_integration() {
        let mut extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();

        // Add historical data
        extractor.add_historical_data(vec![drift_item.clone()]);

        let frequency = extractor.calculate_pattern_frequency(&drift_item);
        assert!(frequency > 0.0);
    }

    #[test]
    fn test_temporal_features() {
        let mut extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();

        // Add some historical data for temporal analysis
        let historical_items = vec![
            create_test_drift_item(),
            DriftItem::new(
                "historical1".to_string(),
                DriftSeverity::High,
                DriftCategory::NewTechnology,
                "Previous React implementation".to_string(),
                "Earlier React change".to_string(),
                DriftLocation::new(PathBuf::from("src/old/App.js")),
            ),
        ];
        extractor.add_historical_data(historical_items);

        let features = extractor.extract_features(&drift_item).unwrap();
        let temporal = &features.temporal_features;

        // Verify temporal features are calculated
        assert!(temporal.days_since_last >= 0.0);
        assert!(temporal.frequency_per_week >= 0.0);
        assert!(temporal.seasonal_strength >= 0.0);
        assert!(temporal.drift_velocity >= 0.0);
        assert!(temporal.clustering_score >= 0.0);
        assert!(temporal.recency_factor >= 0.0);
        
        // Verify ranges
        assert!(temporal.seasonal_strength <= 1.0);
        assert!(temporal.clustering_score <= 1.0);
        assert!(temporal.recency_factor <= 1.0);
    }

    #[test]
    fn test_similarity_score_calculation() {
        let extractor = FeatureExtractor::new();
        
        let item1 = create_test_drift_item();
        let item2 = DriftItem::new(
            "test2".to_string(),
            DriftSeverity::Medium,
            DriftCategory::NewTechnology,
            "Use React for frontend application".to_string(), // Very similar title
            "Different description".to_string(),
            DriftLocation::new(PathBuf::from("src/components/Other.tsx")),
        );
        
        let similarity = extractor.calculate_similarity_score(&item1, &item2);
        println!("Similarity score: {}", similarity);
        
        // Both items have same category (0.4) and severity (0.3) = 0.7/2 = 0.35 base
        assert!(similarity > 0.3); // Should be at least category + severity match
        
        let item3 = DriftItem::new(
            "test3".to_string(),
            DriftSeverity::Low,
            DriftCategory::Configuration,
            "Database config".to_string(),
            "Completely different".to_string(),
            DriftLocation::new(PathBuf::from("config/db.yml")),
        );
        
        let dissimilarity = extractor.calculate_similarity_score(&item1, &item3);
        println!("Dissimilarity score: {}", dissimilarity);
        assert!(dissimilarity < similarity); // Should be less similar
    }

    #[test]
    fn test_days_since_similar_calculation() {
        let mut extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        // Test with no historical data
        let days = extractor.calculate_days_since_similar(&drift_item, 0);
        assert!(days.is_infinite());
        
        // Add historical data
        extractor.add_historical_data(vec![drift_item.clone()]);
        let days = extractor.calculate_days_since_similar(&drift_item, 0);
        assert!(days.is_finite() && days > 0.0);
    }

    #[test]
    fn test_frequency_per_week_calculation() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        let frequency = extractor.calculate_frequency_per_week(&drift_item, 0);
        assert!(frequency >= 0.0);
    }

    #[test]
    fn test_seasonal_strength_calculation() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        let strength = extractor.calculate_seasonal_strength(&drift_item, 0);
        assert!(strength >= 0.0 && strength <= 1.0);
    }

    #[test]
    fn test_drift_velocity_calculation() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        let velocity = extractor.calculate_drift_velocity(&drift_item, 0);
        assert!(velocity >= 0.0);
    }

    #[test]
    fn test_temporal_clustering_calculation() {
        let extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        let clustering = extractor.calculate_temporal_clustering(&drift_item, 0);
        assert!(clustering >= 0.0 && clustering <= 1.0);
    }

    #[test]
    fn test_recency_factor_calculation() {
        let mut extractor = FeatureExtractor::new();
        let drift_item = create_test_drift_item();
        
        // Test with no historical data
        let recency = extractor.calculate_recency_factor(&drift_item, 0);
        assert_eq!(recency, 0.0);
        
        // Add historical data
        extractor.add_historical_data(vec![drift_item.clone()]);
        let recency = extractor.calculate_recency_factor(&drift_item, 0);
        assert!(recency >= 0.0 && recency <= 1.0);
    }
}
