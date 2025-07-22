# ADRScan Configuration Guide

ADRScan provides flexible configuration options through configuration files, environment variables, and command-line overrides.

## Configuration File Formats

ADRScan supports both YAML and TOML configuration formats:

- `.adrscan.yml` or `.adrscan.yaml` (YAML format)
- `.adrscan.toml` (TOML format)

## Configuration File Discovery

ADRScan searches for configuration files in the following order:

1. **Explicit path**: If specified via `--config` flag
2. **Current directory and parents**: Searches up the directory tree for:
   - `.adrscan.yml`
   - `.adrscan.yaml` 
   - `.adrscan.toml`
3. **Default configuration**: If no file is found, uses built-in defaults

## Configuration Schema

### Core Settings

```yaml
# Directory where ADR files are stored (default: "docs/adr")
adr_dir: docs/adr

# File patterns to include when scanning (glob patterns)
include_patterns:
  - "**/*.md"
  - "**/*.rs"
  - "**/*.py"
  - "**/*.js"
  - "**/*.ts"

# File patterns to exclude from scanning
exclude_patterns:
  - "**/target/**"
  - "**/node_modules/**"
  - "**/.git/**"

# Location of snapshot file for drift detection
snapshot_file: .adrscan_snapshot.json
```

### Template Configuration

```yaml
template:
  # Template format: "madr" or "custom"
  format: madr
  
  # Path to custom template (required if format is "custom")
  # custom_path: templates/custom-adr.md
```

### Drift Detection

```yaml
drift:
  # Enable/disable drift detection
  enabled: true
  
  # Patterns for detecting architectural elements
  detection_patterns:
    - name: "Database Dependencies"
      file_pattern: "**/Cargo.toml"
      content_pattern: "(postgres|mysql|sqlite|mongodb)"
      category: "database"
    
    - name: "Cloud Provider"
      file_pattern: "**/*.tf"
      content_pattern: "(aws|azure|gcp|google)"
      category: "cloud"
```

### LSP Configuration

```yaml
lsp:
  # Enable/disable LSP server features
  enabled: true
  
  # Maximum number of diagnostics to report per document
  max_diagnostics: 100
  
  # Enable/disable specific LSP features
  features:
    diagnostics: true
    completion: true
    hover: true
    document_symbols: true
  
  # LSP server behavior settings
  server:
    # Workspace root directory (auto-detected if not specified)
    # workspace_root: /path/to/workspace
    
    # Response timeout in milliseconds
    response_timeout: 5000
    
    # Enable debug logging for LSP operations
    debug_logging: false
```

## Environment Variables

All configuration options can be overridden using environment variables:

### Core Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `ADRSCAN_ADR_DIR` or `ADR_DIR` | ADR directory path | `docs/decisions` |
| `ADRSCAN_INCLUDE_PATTERNS` | Include patterns (comma-separated) | `*.md,*.txt` |
| `ADRSCAN_EXCLUDE_PATTERNS` | Exclude patterns (comma-separated) | `build/**,tmp/**` |
| `ADRSCAN_SNAPSHOT_FILE` | Snapshot file path | `.snapshot.json` |

### Template Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `ADRSCAN_TEMPLATE_FORMAT` | Template format | `custom` |
| `ADRSCAN_TEMPLATE_CUSTOM_PATH` | Custom template path | `templates/adr.md` |

### Drift Detection Environment Variables

| Variable | Description | Values |
|----------|-------------|--------|
| `ADRSCAN_DRIFT_ENABLED` | Enable drift detection | `true`, `false`, `1`, `0`, `yes`, `no`, `on`, `off` |

### LSP Environment Variables

| Variable | Description | Values |
|----------|-------------|--------|
| `ADRSCAN_LSP_ENABLED` | Enable LSP features | `true`, `false`, `1`, `0`, `yes`, `no`, `on`, `off` |
| `ADRSCAN_LSP_MAX_DIAGNOSTICS` | Maximum diagnostics per document | `50`, `100`, `200` |
| `ADRSCAN_LSP_RESPONSE_TIMEOUT` | Response timeout (milliseconds) | `5000`, `10000` |
| `ADRSCAN_LSP_DEBUG_LOGGING` | Enable LSP debug logging | `true`, `false` |

## Command-Line Overrides

Configuration can be overridden using command-line flags:

```bash
# Override ADR directory
adrscan inventory --adr-dir custom/adr

# Use specific config file
adrscan inventory --config custom-config.yml

# Enable verbose output
adrscan inventory --verbose
```

## Configuration Precedence

Configuration values are applied in the following order (highest to lowest precedence):

1. **Command-line flags** (highest precedence)
2. **Environment variables**
3. **Configuration file**
4. **Default values** (lowest precedence)

## Example Configurations

### Minimal YAML Configuration

```yaml
# .adrscan.yml
adr_dir: docs/decisions
include_patterns:
  - "**/*.md"
```

### Complete TOML Configuration

```toml
# .adrscan.toml
adr_dir = "architecture/decisions"
include_patterns = ["**/*.md", "**/*.adoc"]
exclude_patterns = ["**/build/**", "**/target/**"]
snapshot_file = "architecture/.snapshot.json"

[template]
format = "custom"
custom_path = "templates/decision-template.md"

[drift]
enabled = true

[[drift.detection_patterns]]
name = "Database Usage"
file_pattern = "**/*.sql"
content_pattern = "(CREATE TABLE|ALTER TABLE)"
category = "database"
```

### Docker Environment

```bash
# Docker environment variables
export ADRSCAN_ADR_DIR=/app/docs/adr
export ADRSCAN_INCLUDE_PATTERNS="**/*.md,**/*.adoc"
export ADRSCAN_DRIFT_ENABLED=true

# Run in container
docker run -e ADRSCAN_ADR_DIR -e ADRSCAN_INCLUDE_PATTERNS adrscan inventory
```

### CI/CD Pipeline

```yaml
# GitHub Actions example
env:
  ADRSCAN_ADR_DIR: docs/architecture
  ADRSCAN_DRIFT_ENABLED: true
  ADRSCAN_SNAPSHOT_FILE: .github/adr-snapshot.json

steps:
  - name: Scan ADRs
    run: adrscan inventory --format json > adr-report.json
```

## Configuration Validation

ADRScan validates all configuration values and provides detailed error messages:

- **Path validation**: Ensures directories and files exist when required
- **Pattern validation**: Validates glob patterns and regex expressions
- **Format validation**: Ensures template formats are supported
- **Field validation**: Checks for required fields and valid values

## Generating Sample Configuration

Create a sample configuration file:

```bash
# Generate YAML sample
adrscan config --sample-yaml > .adrscan.yml

# Generate TOML sample  
adrscan config --sample-toml > .adrscan.toml
```

## Troubleshooting

### Common Issues

1. **Configuration not found**: Use `--verbose` to see which config files are being searched
2. **Invalid patterns**: Check glob pattern syntax and regex expressions
3. **Permission errors**: Ensure ADR directory is readable and snapshot file is writable
4. **Environment variable conflicts**: Use `env | grep ADRSCAN` to check current values

### Debugging Configuration

```bash
# Show effective configuration
adrscan config --show

# Validate configuration
adrscan config --validate

# Test with environment variables
ADRSCAN_ADR_DIR=/tmp/test adrscan config --show
```