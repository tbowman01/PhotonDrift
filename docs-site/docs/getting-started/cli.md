---
id: "cli"
title: "CLI"
sidebar_label: "💻 CLI Reference"
sidebar_position: "3"
description: "Get up and running with PhotonDrift quickly"
slug: "/getting-started/cli"
tags: ["getting-started"]
last_update:
  date: "2025-07-29"
  author: "tbowman01"
---

# CLI Reference

Complete reference for all PhotonDrift (ADRScan) commands and parameters.

## Commands Overview

PhotonDrift provides 5 core commands for Architecture Decision Record management:

### `init` - Initialize ADR Structure
```bash
adrscan init [OPTIONS]
```

Initialize a new ADR directory structure with ML-ready configuration.

**Options:**
- `--adr-dir <PATH>` - ADR directory path (default: ./docs/adr)
- `--config <FILE>` - Configuration file path
- `--template <TYPE>` - Template type (basic, ml-enhanced, enterprise)

**Example:**
```bash
adrscan init --adr-dir ./decisions --template ml-enhanced
```

### `inventory` - Scan and Catalog ADRs
```bash
adrscan inventory [OPTIONS]
```

Scan existing ADRs and create intelligent catalog with insights.

**Options:**
- `--adr-dir <PATH>` - ADR directory to scan
- `--format <FORMAT>` - Output format (json, yaml, table)
- `--watch` - Monitor directory for changes
- `--recursive` - Scan subdirectories recursively

**Example:**
```bash
adrscan inventory --adr-dir ./docs/adr --format json --recursive
```

### `diff` - Detect Architectural Drift
```bash
adrscan diff [OPTIONS]
```

Detect architectural drift with ML confidence scores and explanations.

**Options:**
- `--adr-dir <PATH>` - ADR directory path
- `--directory <PATH>` - Source code directory to analyze
- `--config <FILE>` - Configuration file
- `--confidence <FLOAT>` - Minimum confidence threshold (0.0-1.0)
- `--model <TYPE>` - ML model (IsolationForest, Ensemble, OneClassSVM, LOF)
- `--format <FORMAT>` - Output format (table, json, markdown)

**Example:**
```bash
adrscan diff --adr-dir ./docs/adr --directory ./src --confidence 0.7 --model Ensemble
```

### `propose` - Generate ADR Proposals
```bash
adrscan propose [OPTIONS]
```

Generate AI-informed ADR proposals for detected architectural changes.

**Options:**
- `--adr-dir <PATH>` - ADR directory path
- `--directory <PATH>` - Source code directory
- `--template <TYPE>` - ADR template to use
- `--interactive` - Interactive proposal mode
- `--auto-create` - Automatically create ADR files

**Example:**
```bash
adrscan propose --adr-dir ./docs/adr --directory ./src --interactive
```

### `index` - Create ADR Index
```bash
adrscan index [OPTIONS]
```

Create comprehensive ADR indexes with smart categorization.

**Options:**
- `--adr-dir <PATH>` - ADR directory path
- `--output <FILE>` - Output index file
- `--format <FORMAT>` - Index format (markdown, html, json)
- `--group-by <FIELD>` - Group by field (status, category, date)

**Example:**
```bash
adrscan index --adr-dir ./docs/adr --output index.md --group-by category
```

## Global Options

Available for all commands:

- `--version` - Show version information
- `--help` - Show help message
- `--verbose, -v` - Verbose output
- `--quiet, -q` - Suppress non-error output
- `--no-color` - Disable colored output
- `--config <FILE>` - Global configuration file

## Configuration File

Commands can use configuration files in YAML or TOML format:

```yaml
# adrscan.yml
adr_dir: "./docs/decisions"
ml:
  enabled: true
  model_type: "Ensemble"
  confidence_threshold: 0.7
  online_learning: true
```

## Environment Variables

- `RUST_LOG` - Logging level (debug, info, warn, error)
- `ADR_CONFIG` - Default configuration file path
- `ADR_DIR` - Default ADR directory
- `ML_ENABLED` - Enable ML features (true/false)
- `ML_MODEL` - Default ML model type
- `ML_CONFIDENCE` - Default confidence threshold

## Exit Codes

- `0` - Success
- `1` - General error
- `2` - Configuration error  
- `3` - File system error
- `4` - ML model error
- `5` - Network error (for future features)

## Examples

### Basic Workflow
```bash
# Initialize new ADR structure
adrscan init --adr-dir ./docs/adr

# Scan existing codebase for drift
adrscan diff --adr-dir ./docs/adr --directory ./src

# Generate proposals for changes
adrscan propose --adr-dir ./docs/adr --directory ./src --interactive

# Create index of all ADRs
adrscan index --adr-dir ./docs/adr --output ./docs/adr-index.md
```

### ML-Enhanced Workflow
```bash
# Configure ML settings
echo "ml:
  enabled: true
  model_type: IsolationForest
  confidence_threshold: 0.8" > adrscan.yml

# Run drift detection with ML
adrscan diff --config adrscan.yml --format json > drift-report.json

# Analyze specific directory with high confidence
adrscan diff --directory ./critical-module --confidence 0.9 --model Ensemble
```

### CI/CD Integration
```bash
# Quick validation in CI
adrscan diff --adr-dir ./docs/adr --directory ./src --quiet || exit 1

# Generate reports for review
adrscan diff --format markdown > drift-report.md
adrscan inventory --format json > adr-inventory.json
```

For more detailed usage examples, see the [User Guide](/docs/getting-started/user-guide).