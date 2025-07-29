---
id: "temporal-analysis"
title: "TEMPORAL ANALYSIS"
sidebar_label: "TEMPORAL ANALYSIS"
sidebar_position: "1"
description: "Development phases and strategic planning"
slug: "/phase-planning/temporal-analysis"
tags: ["phase-planning"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

# Temporal Analysis in PhotonDrift

## Overview

PhotonDrift's ML-enhanced drift detection includes sophisticated temporal analysis capabilities that analyze time-based patterns in architectural changes. This document describes the temporal analysis features and how they enhance drift detection accuracy.

## Temporal Features

### Core Temporal Features

The temporal analysis system extracts the following time-based features from architectural drift patterns:

#### 1. Days Since Last Similar Drift
- **Description**: Measures the time elapsed since the last similar architectural change
- **Range**: 0.0 to ∞ (infinity for first occurrence)
- **Purpose**: Identifies recurring patterns and change frequency
- **Algorithm**: Uses similarity scoring to find related drifts within historical data

#### 2. Frequency Per Week
- **Description**: Calculates how often similar changes occur per week
- **Range**: 0.0 to ∞
- **Purpose**: Detects high-frequency change patterns that may indicate instability
- **Algorithm**: Counts similar drifts within configurable time window and normalizes to weekly frequency

#### 3. Seasonal Pattern Strength
- **Description**: Measures the strength of seasonal patterns in drift occurrence
- **Range**: 0.0 to 1.0 (0 = no pattern, 1 = strong seasonal pattern)
- **Purpose**: Identifies cyclical architectural changes (e.g., quarterly updates)
- **Algorithm**: Analyzes variance in quarterly distribution of similar drifts

#### 4. Drift Velocity
- **Description**: Rate of architectural changes over time
- **Range**: 0.0 to ∞ (changes per day)
- **Purpose**: Measures the pace of architectural evolution
- **Algorithm**: Counts recent changes within time window and normalizes to daily rate

#### 5. Temporal Clustering Score
- **Description**: Measures how clustered similar changes are in time
- **Range**: 0.0 to 1.0 (0 = evenly distributed, 1 = highly clustered)
- **Purpose**: Identifies bursts of related architectural changes
- **Algorithm**: Calculates coefficient of variation for gaps between similar changes

#### 6. Recency Factor
- **Description**: Weight factor based on how recent similar changes are
- **Range**: 0.0 to 1.0 (0 = old changes, 1 = very recent changes)
- **Purpose**: Gives more weight to recent patterns in decision making
- **Algorithm**: Normalizes the position of most recent similar change in historical data

## Similarity Scoring Algorithm

The temporal analysis relies on a sophisticated similarity scoring system that considers:

### 1. Category Similarity (40% weight)
- Exact category match: full score
- Related categories: partial score
- Different categories: no score

### 2. Severity Similarity (30% weight)
- Exact severity match: full score
- Adjacent severities: partial score
- Distant severities: no score

### 3. Textual Similarity (30% weight)
- Keyword-based matching in titles and descriptions
- Filters keywords by length (>3 characters)
- Uses intersection over union metric

## Configuration

### FeatureConfig Options

```rust
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
```

### Default Configuration

```rust
FeatureConfig {
    enable_temporal: true,
    enable_text_analysis: true,
    enable_structural: true,
    temporal_window_days: 30,
}
```

## Usage Examples

### Basic Temporal Analysis

```rust
use adrscan::ml::features::{FeatureExtractor, FeatureConfig};

// Create extractor with temporal analysis enabled
let config = FeatureConfig {
    enable_temporal: true,
    temporal_window_days: 30,
    ..Default::default()
};

let mut extractor = FeatureExtractor::with_config(config);

// Add historical data for temporal analysis
extractor.add_historical_data(historical_drift_items);

// Extract features including temporal analysis
let features = extractor.extract_features(&current_drift_item)?;

// Access temporal features
let temporal = &features.temporal_features;
println!("Days since last similar: {}", temporal.days_since_last);
println!("Frequency per week: {}", temporal.frequency_per_week);
println!("Seasonal strength: {}", temporal.seasonal_strength);
```

### ML Integration

The temporal features are automatically included when using ML-enhanced drift detection:

```rust
use adrscan::ml::{MLDriftDetector, MLConfig};

let config = MLConfig {
    enabled: true,
    confidence_threshold: 0.7,
    ..Default::default()
};

let mut detector = MLDriftDetector::new(config);

// Train with historical data (automatically includes temporal analysis)
detector.train_model(training_data).await?;

// Enhanced detection with temporal features
let results = detector.enhance_detection(drift_items).await?;
```

## Performance Considerations

### Time Complexity
- **Feature extraction**: O(n) where n is the number of historical items
- **Similarity calculation**: O(k) where k is the number of keywords
- **Temporal clustering**: O(m) where m is the number of similar items

### Memory Usage
- Historical data is stored in memory for temporal analysis
- Configurable through `max_training_samples` in MLConfig
- Default limit: 10,000 samples

### Optimization Tips

1. **Limit historical data**: Use `max_training_samples` to control memory usage
2. **Adjust time window**: Smaller `temporal_window_days` improves performance
3. **Cache similar items**: Reuse similarity calculations when possible

## Algorithm Details

### Similarity Score Calculation

The similarity score between two drift items is calculated as:

```
similarity = category_score + severity_score + text_score
```

Where:
- `category_score`: 0.4 if categories match, 0.0 otherwise
- `severity_score`: 0.3 if exact match, 0.2 for adjacent, 0.15 for near, 0.0 otherwise
- `text_score`: (common_keywords / total_keywords) * 0.3

### Temporal Clustering

The clustering score uses coefficient of variation:

```
cv = standard_deviation / mean
clustering_score = max(0.0, 1.0 - cv)
```

### Seasonal Strength

Seasonal analysis divides the historical data into quarters and measures variance:

```
seasonal_strength = min(1.0, variance / (mean + 1.0))
```

## Testing

The temporal analysis includes comprehensive tests covering:

1. **Feature extraction accuracy**
2. **Similarity score calculation**
3. **Edge cases (empty data, single items)**
4. **Performance with various data sizes**
5. **Integration with ML models**

Run tests with:
```bash
cargo test ml::features::tests::test_temporal_features
cargo test ml::features::tests::test_similarity_score_calculation
```

## Future Enhancements

### Planned Features
1. **Advanced seasonal detection**: Fourier analysis for complex patterns
2. **Anomaly clustering**: Group related temporal anomalies
3. **Predictive patterns**: Forecast future drift likelihood
4. **Cross-repository analysis**: Temporal patterns across multiple projects

### Configuration Expansion
1. **Custom similarity weights**: Adjust category/severity/text weights
2. **Advanced time windows**: Multiple overlapping windows
3. **Pattern templates**: Predefined temporal pattern recognition

## Troubleshooting

### Common Issues

1. **Infinite days_since_last**: No similar items found
   - Solution: Check similarity threshold and historical data
   
2. **Zero frequency**: No similar items in time window
   - Solution: Increase `temporal_window_days` or add more historical data
   
3. **Low clustering scores**: Evenly distributed changes
   - Solution: Normal behavior, indicates regular change patterns

### Debug Information

Enable debug logging to see temporal analysis details:

```rust
env_logger::init();
log::set_max_level(log::LevelFilter::Debug);
```

This will show:
- Similarity calculations
- Historical data usage
- Temporal feature computations
- Performance metrics