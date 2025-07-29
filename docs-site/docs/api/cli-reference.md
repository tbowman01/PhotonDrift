---
id: cli-reference
title: CLI Reference
sidebar_label: 💻 CLI Reference
sidebar_position: 1
description: Complete command-line interface reference for PhotonDrift (ADRScan)
slug: /api/cli
tags: [cli, reference, commands]
---

# CLI Reference

Complete reference for all PhotonDrift (ADRScan) commands and parameters.

## Overview

PhotonDrift provides a comprehensive command-line interface for Architecture Decision Record management with AI-enhanced drift detection.

**Usage:** `adrscan [OPTIONS] <COMMAND>`

**Description:** Architecture Decision Record (ADR) management and drift detection

## Global Options

- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--verbose`**, **`-v`** - Verbose output
- **`--help`**, **`-h`** - Print help
- **`--version`**, **`-V`** - Print version

## Commands

### `init` - Initialize ADR directory and configuration

**Usage:**
```bash
adrscan init [OPTIONS]
```

Initialize ADR directory and configuration

**Options:**

- **`--adr-dir`**, **`-a`** `<ADR_DIR>` - ADR directory path (default: docs/adr)
- **`--force`**, **`-f`** - Force initialization even if directory exists
- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--verbose`**, **`-v`** - Verbose output
- **`--help`**, **`-h`** - Print help

**Examples:**

**Initialize ADR structure in default location:**
```bash
adrscan init
```
Creates ./docs/adr directory with basic structure

**Initialize with custom directory and template:**
```bash
adrscan init --adr-dir ./decisions --template ml-enhanced
```
Creates ADR structure with ML-enhanced configuration

### `inventory` - Scan and inventory all existing ADRs and project state

**Usage:**
```bash
adrscan inventory [OPTIONS]
```

Scan and inventory all existing ADRs and project state

**Options:**

- **`--adr-dir`**, **`-a`** `<ADR_DIR>` - ADR directory to scan
- **`--format`**, **`-f`** `<FORMAT>` - Output format (console, json, csv) [default: console]
- **`--status`**, **`-s`** `<STATUS>` - Filter by ADR status
- **`--tag`**, **`-t`** `<TAG>` - Filter by tag
- **`--sort-by`** `<SORT_BY>` - Sort by field (date, status, title) [default: date]
- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--stats`** - Include file statistics
- **`--verbose`**, **`-v`** - Verbose output
- **`--help`**, **`-h`** - Print help

**Examples:**

**Scan existing ADRs and create catalog:**
```bash
adrscan inventory --adr-dir ./docs/adr
```
Scans ADR directory and generates intelligent catalog

**Watch directory for changes with JSON output:**
```bash
adrscan inventory --adr-dir ./docs/adr --format json --watch
```
Monitors ADR directory and outputs changes in JSON format

### `diff` - Perform drift detection by diffing current state against baseline

**Usage:**
```bash
adrscan diff [OPTIONS]
```

Perform drift detection by diffing current state against baseline

**Options:**

- **`--baseline`**, **`-b`** `<BASELINE>` - Baseline snapshot file to compare against
- **`--format`**, **`-f`** `<FORMAT>` - Output format (console, json, yaml) [default: console]
- **`--directory`**, **`-d`** `<DIRECTORY>` - Directory to scan (defaults to current directory)
- **`--adr-dir`** `<ADR_DIR>` - ADR directory to analyze (overrides config)
- **`--save-snapshot`** `<SAVE_SNAPSHOT>` - Save current state as snapshot
- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--verbose`**, **`-v`** - Verbose output
- **`--help`**, **`-h`** - Print help

**Examples:**

**Detect architectural drift with ML confidence scores:**
```bash
adrscan diff --adr-dir ./docs/adr --directory ./src
```
Analyzes source code against ADRs with AI-powered detection

**High confidence drift detection with ensemble model:**
```bash
adrscan diff --confidence 0.8 --model Ensemble --format markdown
```
Uses ensemble ML model with 80% confidence threshold

### `propose` - Auto-generate draft ADRs for detected drift

**Usage:**
```bash
adrscan propose [OPTIONS]
```

Auto-generate draft ADRs for detected drift

**Options:**

- **`--drift-file`**, **`-d`** `<DRIFT_FILE>` - Drift report file to generate proposals from (if not provided, runs drift detection)
- **`--template`**, **`-t`** `<TEMPLATE>` - ADR template to use (madr, custom)
- **`--directory`** `<DIRECTORY>` - Directory to scan for drift detection (defaults to current directory)
- **`--adr-dir`** `<ADR_DIR>` - ADR directory where proposals will be created (overrides config)
- **`--severity`** `<SEVERITY>` - Only generate proposals for specific severity levels
- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--category`** `<CATEGORY>` - Only generate proposals for specific categories
- **`--dry-run`** - Dry run - show what would be generated without creating files
- **`--verbose`**, **`-v`** - Verbose output
- **`--force`** - Force overwrite existing ADR files
- **`--help`**, **`-h`** - Print help

**Examples:**

**Generate AI-informed ADR proposals:**
```bash
adrscan propose --adr-dir ./docs/adr --directory ./src
```
Creates ADR proposals based on detected architectural changes

**Interactive proposal mode with auto-creation:**
```bash
adrscan propose --interactive --auto-create
```
Interactive mode that automatically creates ADR files

### `index` - Generate or update an index of ADRs

**Usage:**
```bash
adrscan index [OPTIONS]
```

Generate or update an index of ADRs

**Options:**

- **`--adr-dir`**, **`-a`** `<ADR_DIR>` - ADR directory to index
- **`--output`**, **`-o`** `<OUTPUT>` - Index file output path
- **`--sort`**, **`-s`** `<SORT>` - Sort order (number, date, status, title) [default: number]
- **`--badges`** - Include status badges in the index
- **`--template`** `<TEMPLATE>` - Custom template file for index generation
- **`--config`**, **`-c`** `<CONFIG>` - Configuration file path
- **`--status-filter`** `<STATUS_FILTER>` - Only include ADRs with specific status
- **`--verbose`**, **`-v`** - Verbose output
- **`--help`**, **`-h`** - Print help

**Examples:**

**Create comprehensive ADR index:**
```bash
adrscan index --adr-dir ./docs/adr --output index.md
```
Generates markdown index with smart categorization

**HTML index grouped by category:**
```bash
adrscan index --format html --group-by category
```
Creates HTML index grouped by ADR categories

## Environment Variables

- **`RUST_LOG`** - Logging level (debug, info, warn, error)
- **`ADR_CONFIG`** - Default configuration file path
- **`ADR_DIR`** - Default ADR directory
- **`ML_ENABLED`** - Enable ML features (true/false)
- **`ML_MODEL`** - Default ML model type
- **`ML_CONFIDENCE`** - Default confidence threshold

## Exit Codes

- **`0`** - Success
- **`1`** - General error
- **`2`** - Configuration error
- **`3`** - File system error
- **`4`** - ML model error
- **`5`** - Network error (for future features)

## Configuration File

Commands can use configuration files in YAML or TOML format. See the [Configuration Reference](config.md) for complete details.

## CI/CD Integration

PhotonDrift is designed for seamless CI/CD integration:

```bash
# Quick validation in CI
adrscan diff --adr-dir ./docs/adr --directory ./src --quiet || exit 1

# Generate reports for review
adrscan diff --format markdown > drift-report.md
adrscan inventory --format json > adr-inventory.json
```

## Getting Help

- Run `adrscan --help` for general help
- Run `adrscan <command> --help` for command-specific help
- Visit our [User Guide](../getting-started/user-guide.md) for detailed tutorials
- Check the [Configuration Reference](config.md) for all options

---

*This documentation is automatically generated from the CLI help output. Last updated: 2025-07-29*
