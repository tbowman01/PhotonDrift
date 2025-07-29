Feature: AI-Powered Drift Detection
  As a development team
  I want intelligent drift detection
  So that I receive accurate alerts with minimal false positives

  Background:
    Given PhotonDrift has ML features enabled
    And I have a codebase with architectural decisions documented

  Scenario: ML model initialization
    Given I have ML features enabled in configuration
    When I run drift detection for the first time
    Then the system should initialize Isolation Forest model
    And initialize One-Class SVM model
    And initialize Local Outlier Factor model
    And initialize Statistical anomaly detection
    And initialize Ensemble model combining all algorithms
    And extract baseline features from the codebase
    And establish normal behavior patterns
    And configure anomaly detection thresholds based on configuration

  Scenario: Feature extraction from codebase
    Given I have a diverse codebase to analyze
    When the ML engine processes the code
    Then it should extract cyclomatic complexity metrics
    And measure nesting depth and indentation patterns
    And analyze import/dependency relationships
    And identify technology stack components
    And compute code structure diversity metrics
    And analyze naming convention patterns
    And measure temporal change velocity
    And calculate coupling and cohesion metrics
    And generate feature vectors with 50+ dimensions per code segment

  Scenario: Technology detection accuracy
    Given I have code using React, PostgreSQL, and Docker
    When the ML engine analyzes the codebase
    Then it should detect React framework usage with >95% confidence
    And identify PostgreSQL database technology
    And recognize Docker containerization patterns
    And classify technology categories correctly
    And measure technology diversity scores

  Scenario: Anomaly detection with confidence scoring
    Given I have trained ML models on historical data
    And I introduce new architectural changes
    When I run drift detection
    Then the system should score each change for anomalous behavior
    And provide confidence levels between 0.0 and 1.0
    And explain the reasoning behind each detection
    And filter results based on confidence thresholds
    And rank anomalies by severity and confidence

  Scenario: Multi-algorithm ensemble detection
    Given I have configured ensemble detection mode
    When analyzing potential architectural drift
    Then the Isolation Forest should provide anomaly scores
    And One-Class SVM should contribute classification results
    And Local Outlier Factor should identify outlier patterns
    And Statistical methods should detect distributional changes
    And the Ensemble should combine results using weighted voting
    And final confidence should reflect algorithm consensus
    And individual algorithm contributions should be traceable

  Scenario: Confidence threshold configuration
    Given I have set confidence threshold to 0.8
    When processing drift detection results
    Then only results with confidence >= 0.8 should be reported
    And lower confidence results should be logged but not displayed
    And threshold should be configurable per project
    And different thresholds should be supported per drift category

  Scenario: Online learning and adaptation
    Given I have provided feedback on previous detections
    And marked some detections as false positives
    When the system processes new changes
    Then it should incorporate previous feedback into model training
    And adjust detection sensitivity based on feedback
    And update model parameters using online learning algorithms
    And improve accuracy over time through adaptive learning
    And maintain memory efficiency with TTL management for training samples

  Scenario: Model explanation generation
    Given the ML system has detected potential drift
    When requesting explanations for detections
    Then it should identify which features contributed most to the anomaly score
    And explain feature importance in human-readable terms
    And provide specific code patterns that triggered detection
    And suggest potential remediation actions
    And reference relevant ADRs that may be violated

  Scenario: False positive reduction
    Given I have a baseline false positive rate of 20% without ML
    When using ML-enhanced detection
    Then false positive rate should be reduced to <5%
    And true positive rate should remain >90%
    And precision should exceed 95%
    And recall should exceed 90%

  Scenario: Performance requirements for ML inference
    Given I have trained models loaded in memory
    When performing drift detection on individual files
    Then single file analysis should complete within 50ms
    And batch processing should handle 1000 files per minute
    And model loading should complete within 5 seconds
    And memory usage should be bounded and predictable

  Scenario: Feature importance analysis
    Given the ML system has processed a large codebase
    When analyzing feature importance
    Then complexity metrics should be among top 10 important features
    And technology diversity should have high importance
    And temporal change patterns should contribute significantly
    And structural metrics should be relevant for anomaly detection

  Scenario: Cross-validation and model accuracy
    Given I have sufficient training data
    When training ML models with cross-validation
    Then 5-fold cross-validation should be performed
    And each fold should achieve >90% accuracy
    And model variance should be low across folds
    And hyperparameters should be optimized for best performance

  Scenario: Memory management for large codebases
    Given I have a codebase with 100,000+ files
    When training ML models
    Then the system should implement memory-efficient feature extraction
    And use streaming processing for large datasets
    And maintain bounded memory usage regardless of codebase size
    And implement TTL for training samples to prevent memory growth

  Scenario Outline: Model performance validation
    Given I have trained a <model_type> model
    When evaluating model performance on test data
    Then the precision should be at least <min_precision>
    And the recall should be at least <min_recall>
    And the F1-score should be at least <min_f1>
    And training time should be under <max_training_time>
    
    Examples:
      | model_type       | min_precision | min_recall | min_f1 | max_training_time |
      | IsolationForest  | 0.90         | 0.85       | 0.87   | 60s              |
      | OneClassSVM      | 0.88         | 0.82       | 0.85   | 120s             |
      | LocalOutlierFactor| 0.89        | 0.84       | 0.86   | 90s              |
      | Statistical      | 0.85         | 0.80       | 0.82   | 30s              |
      | Ensemble         | 0.95         | 0.90       | 0.92   | 180s             |

  Scenario: Model persistence and loading
    Given I have trained ML models
    When saving models to disk
    Then models should be serialized in efficient format
    And loading should be faster than training
    And model versioning should be supported
    And backward compatibility should be maintained

  Scenario: Hyperparameter optimization
    Given I want to optimize ML model performance
    When running hyperparameter tuning
    Then grid search should be performed for key parameters
    And cross-validation should evaluate parameter combinations
    And best parameters should be selected based on validation scores
    And optimization should complete within reasonable time

  Scenario: Drift severity classification
    Given the ML system has detected anomalies
    When classifying drift severity
    Then high confidence (>0.9) should indicate critical drift
    And medium confidence (0.7-0.9) should indicate moderate drift
    And low confidence (0.5-0.7) should indicate potential drift
    And very low confidence (<0.5) should be filtered out

  Scenario: Integration with rule-based detection
    Given I have both ML and rule-based detection enabled
    When analyzing code for drift
    Then ML results should enhance rule-based findings
    And rule-based detection should provide baseline coverage
    And combined results should have higher accuracy than either alone
    And conflicting results should be handled gracefully

  Scenario: Model retraining triggers
    Given ML models are deployed in production
    When model performance degrades
    Then automatic retraining should be triggered when accuracy drops below threshold
    And new training data should be incorporated regularly
    And retraining schedule should be configurable
    And model rollback should be possible if retraining reduces performance