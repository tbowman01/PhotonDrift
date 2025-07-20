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
        use crate::drift::{DriftSeverity, DriftCategory};
        
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
            "react", "vue", "angular", "typescript", "javascript",
            "rust", "python", "java", "go", "kotlin",
            "docker", "kubernetes", "aws", "gcp", "azure",
            "postgres", "mysql", "mongodb", "redis", "elasticsearch"
        ];
        
        let text = format!("{} {}", drift_item.title, drift_item.description).to_lowercase();
        
        tech_keywords.iter()
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
        let similar_count = self.historical_data.iter()
            .filter(|item| {
                item.category == drift_item.category && 
                item.severity == drift_item.severity
            })
            .count();
        
        if self.historical_data.is_empty() {
            0.5 // Default frequency for new patterns
        } else {
            similar_count as f64 / self.historical_data.len() as f64
        }
    }
    
    /// Extract temporal features
    fn extract_temporal_features(&self, _drift_item: &DriftItem) -> TemporalFeatures {
        // TODO: Implement actual temporal analysis
        // For now, return default values
        TemporalFeatures {
            days_since_last: 7.0,
            frequency_per_week: 1.2,
            seasonal_strength: 0.3,
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
        let negative_words = ["bad", "poor", "terrible", "problem", "issue", "bug", "error"];
        
        let text_lower = text.to_lowercase();
        let positive_count = positive_words.iter().filter(|&word| text_lower.contains(word)).count();
        let negative_count = negative_words.iter().filter(|&word| text_lower.contains(word)).count();
        
        if positive_count + negative_count == 0 {
            0.0 // Neutral
        } else {
            (positive_count as f64 - negative_count as f64) / (positive_count + negative_count) as f64
        }
    }
    
    /// Count technical terms in text
    fn count_tech_terms(&self, text: &str) -> usize {
        let tech_terms = [
            "api", "database", "service", "component", "module", "library",
            "framework", "architecture", "pattern", "interface", "protocol",
            "algorithm", "optimization", "performance", "scalability", "security"
        ];
        
        let text_lower = text.to_lowercase();
        tech_terms.iter()
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
        let max_count = self.tech_patterns.values().fold(0.0_f64, |max, &val| max.max(val));
        if max_count > 0.0 {
            for value in self.tech_patterns.values_mut() {
                *value /= max_count;
            }
        }
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
        assert!(score >= 0.0 && score <= 1.0);
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
        
        let count = extractor.count_tech_terms("This API uses a database service with security patterns");
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
}