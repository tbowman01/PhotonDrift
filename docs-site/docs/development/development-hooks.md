---
id: "development-hooks"
title: "DEVELOPMENT HOOKS"
sidebar_label: "DEVELOPMENT HOOKS"
sidebar_position: "1"
description: "Development guides and contributing guidelines"
slug: "/development/development-hooks"
tags: ["development"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

# Development Hooks and Code Quality

This document describes the pre-commit hooks and code quality checks for ADRScan development.

## Overview

ADRScan uses a comprehensive set of pre-commit hooks to ensure code quality, security, and consistency. These hooks run automatically before each commit to catch issues early in the development process.

## Quick Setup

```bash
# Run the setup script to configure everything
./setup-hooks.sh
```

This will:
- Install pre-commit if not available
- Configure git hooks directory
- Install all pre-commit hooks
- Set up Rust toolchain components
- Run initial code formatting and checks

## Manual Setup

If you prefer to set up manually:

```bash
# Install pre-commit
pip install pre-commit

# Configure git hooks
git config core.hooksPath .githooks

# Install pre-commit hooks
pre-commit install

# Install Rust components
rustup component add rustfmt clippy

# Make hooks executable
chmod +x .githooks/pre-commit
```

## Pre-commit Hooks

### Rust-Specific Checks

1. **Code Formatting (`rust-fmt`)**
   - Runs: `cargo fmt --all -- --check`
   - Purpose: Ensures consistent code formatting
   - Fix: `cargo fmt --all`

2. **Linting (`rust-clippy`)**
   - Runs: `cargo clippy --all-targets --all-features -- -D warnings`
   - Purpose: Catches common mistakes and enforces best practices
   - Fix: Address clippy warnings or use `#[allow(clippy::lint_name)]`

3. **Compilation Check (`rust-check`)**
   - Runs: `cargo check --all-targets --all-features`
   - Purpose: Ensures code compiles without errors
   - Fix: Fix compilation errors

4. **Test Suite (`rust-test`)**
   - Runs: `cargo test --all-features`
   - Purpose: Ensures all tests pass
   - Skip: Set `SKIP_TESTS=1` environment variable

### General Checks

1. **File Quality**
   - Trailing whitespace removal
   - End-of-file fixing
   - Large file detection (>1MB)
   - Mixed line ending fixes

2. **Format Validation**
   - YAML syntax checking
   - TOML syntax checking  
   - JSON syntax checking
   - Markdown linting

3. **Security**
   - Secret detection with `detect-secrets`
   - Basic security pattern checking
   - Merge conflict detection

## Custom Git Hook

The `.githooks/pre-commit` script provides additional Rust-specific checks:

### Features

- **Smart File Detection**: Only runs Rust checks when `.rs` files are staged
- **Colored Output**: Clear, colored status messages
- **Interactive Prompts**: Asks for confirmation on warnings
- **Security Scanning**: Basic checks for sensitive data patterns
- **TODO/FIXME Detection**: Warns about new TODO comments

### Environment Variables

Control hook behavior with these environment variables:

```bash
# Skip test suite (useful for quick commits)
SKIP_TESTS=1 git commit -m "quick fix"

# Allow TODO/FIXME comments without prompts
ALLOW_TODOS=1 git commit -m "add feature with todos"

# Skip security checks
SKIP_SECURITY_CHECK=1 git commit -m "test commit"
```

## Running Hooks Manually

```bash
# Run all pre-commit hooks on all files
pre-commit run --all-files

# Run specific hook
pre-commit run rust-fmt
pre-commit run rust-clippy

# Run only on staged files
pre-commit run

# Bypass hooks for emergency commits
git commit --no-verify -m "emergency fix"
```

## Common Issues and Solutions

### Hook Failures

**Formatting Failure:**
```bash
# Fix formatting
cargo fmt --all
git add .
git commit -m "your message"
```

**Clippy Warnings:**
```bash
# See detailed warnings
cargo clippy --all-targets --all-features

# Fix or allow specific warnings
# In code: #[allow(clippy::warning_name)]
```

**Test Failures:**
```bash
# Run tests to see failures
cargo test

# Skip tests if needed
SKIP_TESTS=1 git commit -m "wip: work in progress"
```

### Installation Issues

**Pre-commit not found:**
```bash
# Install with pip
pip install pre-commit

# Or with homebrew (macOS)
brew install pre-commit

# Or with conda
conda install -c conda-forge pre-commit
```

**Rust components missing:**
```bash
# Install required components
rustup component add rustfmt clippy
```

## Hook Configuration

### Pre-commit Configuration (`.pre-commit-config.yaml`)

The configuration includes:
- Rust-specific hooks for formatting, linting, and testing
- General file quality hooks
- Security scanning with `detect-secrets`
- Markdown linting
- YAML formatting

### Custom Hook Configuration (`.githooks/pre-commit`)

The custom hook provides:
- Intelligent Rust file detection
- Interactive prompts for warnings
- Environment variable controls
- Comprehensive status reporting

## Best Practices

1. **Run hooks before committing:**
   ```bash
   pre-commit run --all-files
   ```

2. **Keep commits focused:**
   - Fix formatting issues in separate commits
   - Address clippy warnings before feature commits

3. **Use environment variables for special cases:**
   - `SKIP_TESTS=1` for work-in-progress commits
   - `ALLOW_TODOS=1` when adding planned work

4. **Regular maintenance:**
   ```bash
   # Update hook repositories
   pre-commit autoupdate
   
   # Clean hook cache
   pre-commit clean
   ```

## Troubleshooting

### Slow Hook Performance

```bash
# Skip heavy checks for quick commits
SKIP_TESTS=1 git commit -m "quick fix"

# Or use --no-verify for emergencies
git commit --no-verify -m "emergency hotfix"
```

### Hook Conflicts

```bash
# Reset hooks if corrupted
pre-commit uninstall
pre-commit install

# Or reconfigure git hooks
git config core.hooksPath .githooks
```

### False Positives

For security scanning false positives, update `.secrets.baseline`:

```bash
# Regenerate baseline
detect-secrets scan --baseline .secrets.baseline

# Or update manually in .secrets.baseline
```

## Integration with IDE

### VS Code

Add to `.vscode/settings.json`:
```json
{
  "rust-analyzer.rustfmt.rangeFormatting.enable": true,
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  }
}
```

### CLI Aliases

Add to your shell profile:
```bash
alias fmt="cargo fmt --all"
alias check="cargo clippy --all-targets --all-features"
alias test="cargo test --all-features"
alias hooks="pre-commit run --all-files"
```

## Contributing

When contributing to ADRScan:

1. Run `./setup-hooks.sh` after cloning
2. Ensure all hooks pass before submitting PR
3. Include hook output in PR description if relevant
4. Update hook configuration for new requirements

For questions about the hook configuration, please open an issue or discussion on the repository.