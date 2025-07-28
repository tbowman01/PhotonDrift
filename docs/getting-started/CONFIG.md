# Configuration Reference

Complete configuration options for PhotonDrift (ADRScan).

## Configuration File Formats

PhotonDrift supports both YAML and TOML configuration formats:

### YAML Configuration (`adrscan.yml`)
```yaml
# Core Settings
adr_dir: "./docs/decisions"
source_dirs: 
  - "./src"
  - "./lib" 
  - "./components"

# ML Configuration
ml:
  enabled: true
  model_type: "Ensemble"          # IsolationForest, OneClassSVM, LOF, Statistical, Ensemble
  confidence_threshold: 0.7       # 0.0-1.0 (higher = fewer, higher-confidence detections)
  online_learning: true           # Learn from feedback to improve accuracy
  max_training_samples: 10000     # Memory management for large codebases
  feature_extraction:
    enable_complexity_metrics: true
    enable_semantic_analysis: true
    enable_temporal_patterns: true

# Drift Detection
drift:
  enabled: true
  detection_patterns:
    - pattern: "new framework"
      severity: "high"
      description: "Introduction of new framework without ADR"
    - pattern: "deprecated library"
      severity: "medium"
      description: "Usage of deprecated libraries"
    - pattern: "api breaking change"
      severity: "high"
      description: "Breaking API changes without documentation"

# Output Configuration
output:
  format: "table"                 # table, json, yaml, markdown
  show_confidence: true
  show_explanations: true
  group_by_severity: true
  max_results: 100

# File Processing
processing:
  excluded_patterns:
    - "*.test.js"
    - "*.spec.ts"
    - "node_modules/**"
    - "target/**"
    - ".git/**"
  included_extensions:
    - ".rs"
    - ".js"
    - ".ts"
    - ".py"
    - ".java"
    - ".go"
  max_file_size_mb: 10
  parallel_processing: true

# ADR Template Configuration
templates:
  default_template: "standard"
  custom_templates:
    - name: "architecture"
      path: "./templates/architecture-adr.md"
    - name: "security"
      path: "./templates/security-adr.md"

# Logging
logging:
  level: "info"                   # debug, info, warn, error
  file: "./logs/adrscan.log"
  max_file_size_mb: 50
  max_files: 5
```

### TOML Configuration (`adrscan.toml`)
```toml
# Core Settings
adr_dir = "./docs/decisions"
source_dirs = ["./src", "./lib", "./components"]

[ml]
enabled = true
model_type = "Ensemble"
confidence_threshold = 0.7
online_learning = true
max_training_samples = 10000

[ml.feature_extraction]
enable_complexity_metrics = true
enable_semantic_analysis = true
enable_temporal_patterns = true

[drift]
enabled = true

[[drift.detection_patterns]]
pattern = "new framework"
severity = "high"
description = "Introduction of new framework without ADR"

[[drift.detection_patterns]]
pattern = "deprecated library"
severity = "medium"
description = "Usage of deprecated libraries"

[output]
format = "table"
show_confidence = true
show_explanations = true
group_by_severity = true
max_results = 100

[processing]
excluded_patterns = ["*.test.js", "*.spec.ts", "node_modules/**"]
included_extensions = [".rs", ".js", ".ts", ".py", ".java", ".go"]
max_file_size_mb = 10
parallel_processing = true

[logging]
level = "info"
file = "./logs/adrscan.log"
max_file_size_mb = 50
max_files = 5
```

## Configuration Options Reference

### Core Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `adr_dir` | String | `"./docs/adr"` | Directory containing ADR files |
| `source_dirs` | Array | `["./src"]` | Source code directories to analyze |
| `config_file` | String | `"adrscan.yml"` | Configuration file path |

### ML Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `ml.enabled` | Boolean | `false` | Enable ML-enhanced drift detection |
| `ml.model_type` | String | `"IsolationForest"` | ML model type |
| `ml.confidence_threshold` | Float | `0.7` | Minimum confidence for detections (0.0-1.0) |
| `ml.online_learning` | Boolean | `false` | Enable continuous learning |
| `ml.max_training_samples` | Integer | `10000` | Maximum samples for training |

#### ML Model Types

- **`IsolationForest`** - Best for detecting outliers in code patterns
- **`OneClassSVM`** - Effective for boundary-based anomaly detection  
- **`LOF`** - Local Outlier Factor for density-based detection
- **`Statistical`** - Statistical methods for drift detection
- **`Ensemble`** - Combines multiple models for best accuracy

#### ML Feature Extraction

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `ml.feature_extraction.enable_complexity_metrics` | Boolean | `true` | Extract code complexity features |
| `ml.feature_extraction.enable_semantic_analysis` | Boolean | `true` | Analyze semantic patterns |
| `ml.feature_extraction.enable_temporal_patterns` | Boolean | `true` | Track temporal change patterns |

### Drift Detection

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `drift.enabled` | Boolean | `true` | Enable drift detection |
| `drift.detection_patterns` | Array | `[]` | Custom detection patterns |

#### Detection Pattern Structure

```yaml
detection_patterns:
  - pattern: "regex_or_string"    # Pattern to match
    severity: "high|medium|low"   # Severity level
    description: "explanation"    # Human-readable description
    confidence_boost: 0.1         # Additional confidence (optional)
    category: "architecture"      # Category for grouping (optional)
```

### Output Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `output.format` | String | `"table"` | Output format (table, json, yaml, markdown) |
| `output.show_confidence` | Boolean | `true` | Show ML confidence scores |
| `output.show_explanations` | Boolean | `true` | Show AI explanations |
| `output.group_by_severity` | Boolean | `false` | Group results by severity |
| `output.max_results` | Integer | `100` | Maximum results to display |

### File Processing

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `processing.excluded_patterns` | Array | `["node_modules/**"]` | Glob patterns to exclude |
| `processing.included_extensions` | Array | `[".rs", ".js", ".ts"]` | File extensions to include |
| `processing.max_file_size_mb` | Integer | `10` | Maximum file size to process |
| `processing.parallel_processing` | Boolean | `true` | Enable parallel file processing |

### Template Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `templates.default_template` | String | `"standard"` | Default ADR template |
| `templates.custom_templates` | Array | `[]` | Custom template definitions |

### Logging Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `logging.level` | String | `"info"` | Log level (debug, info, warn, error) |
| `logging.file` | String | `null` | Log file path (optional) |
| `logging.max_file_size_mb` | Integer | `50` | Maximum log file size |
| `logging.max_files` | Integer | `5` | Maximum number of log files |

## Environment Variables

Environment variables override configuration file settings:

| Variable | Description | Example |
|----------|-------------|---------|
| `ADR_DIR` | ADR directory path | `export ADR_DIR="./decisions"` |
| `ADR_CONFIG` | Configuration file path | `export ADR_CONFIG="./config/adrscan.yml"` |
| `ML_ENABLED` | Enable ML features | `export ML_ENABLED=true` |
| `ML_MODEL` | ML model type | `export ML_MODEL="Ensemble"` |
| `ML_CONFIDENCE` | Confidence threshold | `export ML_CONFIDENCE=0.8` |
| `RUST_LOG` | Rust logging level | `export RUST_LOG=debug` |

## Configuration Precedence

Settings are applied in this order (highest to lowest precedence):

1. **Command-line arguments** (e.g., `--confidence 0.8`)
2. **Environment variables** (e.g., `ML_CONFIDENCE=0.8`)
3. **Configuration file** (e.g., `adrscan.yml`)
4. **Default values**

## Configuration Examples

### Minimal Configuration
```yaml
adr_dir: "./docs/adr"
ml:
  enabled: true
```

### Development Configuration
```yaml
adr_dir: "./docs/decisions"
source_dirs: ["./src", "./lib"]
ml:
  enabled: true
  model_type: "IsolationForest"
  confidence_threshold: 0.6
logging:
  level: "debug"
```

### Production Configuration
```yaml
adr_dir: "./docs/adr"
source_dirs: ["./src"]
ml:
  enabled: true
  model_type: "Ensemble"
  confidence_threshold: 0.8
  online_learning: true
drift:
  enabled: true
  detection_patterns:
    - pattern: "TODO|FIXME|HACK"
      severity: "medium"
      description: "Code debt indicators"
output:
  format: "json"
  show_confidence: true
processing:
  excluded_patterns:
    - "*.test.*"
    - "node_modules/**"
    - "target/**"
    - ".git/**"
logging:
  level: "warn"
  file: "./logs/adrscan.log"
```

### Enterprise Configuration
```yaml
adr_dir: "./architecture/decisions"
source_dirs: 
  - "./services"
  - "./shared"
  - "./infrastructure"
ml:
  enabled: true
  model_type: "Ensemble"
  confidence_threshold: 0.9
  max_training_samples: 50000
  feature_extraction:
    enable_complexity_metrics: true
    enable_semantic_analysis: true
    enable_temporal_patterns: true
drift:
  enabled: true
  detection_patterns:
    - pattern: "spring-boot"
      severity: "high"
      description: "Framework changes require architecture review"
    - pattern: "database migration"
      severity: "high"
      description: "Schema changes need ADR documentation"
    - pattern: "microservice"
      severity: "medium"
      description: "Service boundary changes"
output:
  format: "json"
  show_confidence: true
  show_explanations: true
  max_results: 1000
processing:
  excluded_patterns:
    - "*.test.*"
    - "*.spec.*"
    - "**/test/**"
    - "**/tests/**"
    - "node_modules/**"
    - "target/**"
    - "build/**"
    - ".git/**"
  parallel_processing: true
  max_file_size_mb: 50
templates:
  default_template: "enterprise"
  custom_templates:
    - name: "api-change"
      path: "./templates/api-change-adr.md"
    - name: "security-review"
      path: "./templates/security-adr.md"
logging:
  level: "info"
  file: "./logs/adrscan.log"
  max_file_size_mb: 100
  max_files: 10
```

## Validation

PhotonDrift validates configuration on startup and will report errors for:

- Invalid file paths
- Out-of-range numeric values
- Unknown configuration keys
- Malformed patterns
- Missing required fields

Use `adrscan --help` or `adrscan <command> --help` for command-specific options.

For usage examples, see the [CLI Reference](CLI.md) and [User Guide](USER_GUIDE.md).