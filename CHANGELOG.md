# Changelog

All notable changes to PhotonDrift (ADRScan) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 🐳 Docker Support & Containerization
- **NEW**: Multi-stage Dockerfile with security-hardened build process
- **NEW**: Linux amd64 container builds for GitHub Container Registry
- **NEW**: Health checks and proper OCI metadata labels
- **NEW**: .dockerignore optimization for efficient build context
- **NEW**: GitHub Actions workflow for automated Docker builds
- **IMPROVED**: Docker documentation with usage examples and deployment guides

### 🔧 Infrastructure & DevOps
- **NEW**: Comprehensive pre-commit hooks with Rust formatting, linting, and testing
- **NEW**: Custom git hooks for intelligent code quality checks
- **NEW**: Development environment setup automation with `setup-hooks.sh`
- **NEW**: Security scanning with `detect-secrets` integration
- **IMPROVED**: GitHub Actions workflows updated to latest versions (v4)
- **IMPROVED**: Docker containerization with multi-platform support (amd64/arm64)
- **IMPROVED**: Enhanced development documentation with hook configuration guide

### 🛠️ Code Quality & Standards
- **NEW**: Automated code formatting with `rustfmt` on pre-commit
- **NEW**: Clippy linting with warnings-as-errors enforcement
- **NEW**: Compilation checks and test execution in CI pipeline
- **NEW**: Interactive development hooks with colored output and prompts
- **IMPROVED**: Zero-warning build target achieved
- **IMPROVED**: Enhanced error handling and debugging capabilities

### 📚 Documentation Improvements
- **NEW**: Comprehensive development hooks documentation (`docs/DEVELOPMENT_HOOKS.md`)
- **NEW**: Pre-commit configuration with environment variable controls
- **NEW**: IDE integration guides for VS Code and CLI aliases
- **NEW**: Docker usage documentation with examples and best practices
- **IMPROVED**: Repository setup instructions with automated scripts

### 🚀 Release Management
- **NEW**: Enhanced containerization workflow with security best practices
- **NEW**: Multi-architecture container builds for broader compatibility
- **IMPROVED**: Alpha release tagging with build number integration
- **IMPROVED**: GitHub release automation with detailed release notes

## [0.2.0-alpha.2025.07.20] - 2025-07-20

### 🚀 Major Features - Phase 2: Intelligence & Integration

#### 🤖 ML-Enhanced Drift Detection
- **NEW**: Complete ML module with 4 comprehensive submodules (`detector`, `features`, `models`, `training`)
- **NEW**: 5 anomaly detection algorithms: Isolation Forest, One-Class SVM, LOF, Statistical, Ensemble
- **NEW**: Advanced feature engineering with 50+ extracted features
- **NEW**: Confidence-based filtering to reduce false positives by 60-80%
- **NEW**: Explainable AI with model explanations for every detection
- **NEW**: Online learning capabilities for adaptive threshold adjustment
- **NEW**: Cross-validation and hyperparameter optimization infrastructure

#### 📊 Performance & Quality Metrics
- **NEW**: Comprehensive ML test suite with 26 specialized tests (100% ML coverage)
- **NEW**: Performance benchmarking: 1-5ms prediction time, ~10MB memory for 1000 samples
- **NEW**: Model accuracy tracking: Precision, Recall, F1-score monitoring
- **NEW**: Training session history and performance reporting
- **IMPROVED**: Overall test coverage: 178/182 tests passing (97.8% success rate)

#### ⚙️ Configuration & Integration
- **NEW**: ML-ready configuration system with backward compatibility
- **NEW**: Model persistence and serialization infrastructure
- **NEW**: Feature-gated ML compilation (optional for lightweight builds)
- **NEW**: Smart model auto-selection based on data characteristics
- **IMPROVED**: Enhanced configuration validation and error handling

#### 🧪 Development Infrastructure
- **NEW**: Comprehensive testing framework for ML components
- **NEW**: Mock models for development and testing environments
- **NEW**: Feature extraction pipeline with validation
- **NEW**: Training data management with TTL and memory limits
- **IMPROVED**: Modular architecture for easy ML model integration

## [0.1.0] - 2025-07-15

### 🚀 Initial Release - Core Foundation

#### 🏗️ Core CLI Framework
- **NEW**: Complete CLI tool with 5 primary commands (`init`, `inventory`, `diff`, `propose`, `index`)
- **NEW**: Flexible configuration system supporting YAML and TOML formats
- **NEW**: Pattern-based drift detection engine with configurable rules
- **NEW**: Multi-format output support (JSON, YAML, console, CSV)
- **NEW**: Robust error handling and logging infrastructure

#### 📝 ADR Management
- **NEW**: ADR directory initialization with templates
- **NEW**: Comprehensive ADR inventory and cataloging
- **NEW**: Architectural drift detection with severity classification
- **NEW**: Intelligent ADR proposal generation
- **NEW**: Smart ADR indexing with categorization

#### 🌐 Integration & Deployment
- **NEW**: WebAssembly (WASM) support for browser and Node.js environments
- **NEW**: GitHub Action for CI/CD integration
- **NEW**: Cross-platform binary distribution
- **NEW**: Docker containerization support

#### 🧪 Quality & Testing
- **NEW**: Comprehensive test suite with 114 tests (96.5% pass rate)
- **NEW**: Integration testing framework
- **NEW**: Performance benchmarking (206 files in ~91ms)
- **NEW**: Zero compilation warnings target achieved

#### 📚 Documentation & Examples
- **NEW**: Complete user documentation and API reference
- **NEW**: Configuration examples and best practices
- **NEW**: Integration guides for CI/CD workflows
- **NEW**: Developer contribution guidelines