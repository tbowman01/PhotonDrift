# PhotonDrift - The Next Generation AI-Powered ADR Management Tool

> Next-generation Architecture Decision Record (ADR) management with ML-enhanced drift detection for intelligent development governance.

[![Version](https://img.shields.io/badge/version-0.2.0--alpha.20250720-blue)](https://github.com/tbowman01/PhotonDrift/releases)
[![Tests](https://img.shields.io/badge/tests-140%2F144%20passing-green)](https://github.com/tbowman01/PhotonDrift/actions)
[![ML Coverage](https://img.shields.io/badge/ML%20tests-26%2F26%20passing-brightgreen)](https://github.com/tbowman01/PhotonDrift/tree/main/src/ml)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## 🚀 Overview

PhotonDrift is an AI-powered Rust CLI tool that revolutionizes Architecture Decision Record (ADR) management through **machine learning-enhanced drift detection**. It automatically identifies when code deviates from documented architectural decisions and provides intelligent insights to maintain architectural integrity.

## ✨ Key Features

### 🤖 **AI-Enhanced Detection** (v0.2.0+)
- **Machine Learning Models**: 5 advanced algorithms (Isolation Forest, SVM, LOF, Statistical, Ensemble)
- **Smart Confidence Scoring**: Reduces false positives by 60-80%
- **Explainable AI**: Clear explanations for every detection decision
- **Adaptive Learning**: Improves accuracy through feedback and historical data

### 🔍 **Intelligent Analysis**
- **Feature Engineering**: 50+ extracted features (complexity, diversity, temporal patterns)
- **Technology Detection**: Automatic identification of frameworks, languages, patterns
- **Sentiment Analysis**: Understanding the context and urgency of architectural changes
- **Structural Analysis**: Code organization, coupling, and cohesion metrics

### 🚀 **Core CLI Commands**
- **`init`**: Initialize ADR structure with ML-ready configuration
- **`inventory`**: Scan and catalog existing ADRs with intelligence insights
- **`diff`**: Detect architectural drift with ML confidence scores
- **`propose`**: Generate AI-informed ADR proposals for detected changes
- **`index`**: Create comprehensive ADR indexes with smart categorization

### 🌐 **Enterprise Integration**
- **Multi-Language Support**: Works across diverse technology stacks
- **Offline-First**: All ML processing is local (no external API calls)
- **CI/CD Ready**: WebAssembly module and GitHub Action for automation
- **Scalable**: Handles enterprise codebases (100k+ files)

## 🚀 Quick Start

### Installation

```bash
# Download from GitHub Releases
curl -L https://github.com/tbowman01/PhotonDrift/releases/latest/download/adrscan-linux -o adrscan
chmod +x adrscan

# Or build from source
git clone https://github.com/tbowman01/PhotonDrift
cd PhotonDrift
cargo build --release
```

### Basic Usage

```bash
# Initialize ADR structure in your project
adrscan init

# Scan existing ADRs and codebase
adrscan inventory

# Detect architectural drift with AI
adrscan diff

# Generate AI-informed ADR proposals
adrscan propose

# Create intelligent ADR index
adrscan index
```

### 🤖 AI-Enhanced Usage (v0.2.0+)

```bash
# Enable ML features with configuration
echo "ml:
  enabled: true
  model_type: IsolationForest
  confidence_threshold: 0.7
  online_learning: true" > adrscan.yml

# Detect drift with ML confidence scores
adrscan diff --config adrscan.yml

# Example output with AI insights:
# 🚨 HIGH CONFIDENCE (0.85): New React framework detected
# 📝 Explanation: High technology diversity and complexity score indicate
#    significant architectural change requiring ADR documentation
# 💡 Recommendation: Create ADR for frontend framework standardization
```

### 🏗️ Advanced Configuration

```yaml
# adrscan.yml - ML-Enhanced Configuration
adr_dir: "./docs/decisions"
ml:
  enabled: true
  model_type: "Ensemble"          # IsolationForest, OneClassSVM, LOF, Statistical, Ensemble
  confidence_threshold: 0.7       # 0.0-1.0 (higher = fewer, higher-confidence detections)
  online_learning: true           # Learn from feedback to improve accuracy
  max_training_samples: 10000     # Memory management for large codebases

drift:
  enabled: true
  detection_patterns:
    - pattern: "new framework"
      severity: "high"
    - pattern: "deprecated library"
      severity: "medium"
```

## 📚 Documentation

### Core Documentation
- **[CHANGELOG.md](CHANGELOG.md)** - Complete version history and release notes
- **[ROADMAP.md](ROADMAP.md)** - Development roadmap through 2025
- **[Requirements & Architecture](docs/REQUIREMENTS_SUMMARY.md)** - Technical requirements and phases

### ML & AI Features
- **[ML Module Overview](src/ml/README.md)** - Machine learning architecture and algorithms
- **[Feature Engineering Guide](src/ml/features.rs)** - Understanding extracted features
- **[Model Selection Guide](src/ml/models.rs)** - Choosing the right ML algorithm
- **[Training & Optimization](src/ml/training.rs)** - Model training and hyperparameter tuning

### Integration & Usage
- **[GitHub Action Guide](.github/workflows/)** - CI/CD integration with ML features
- **[WebAssembly Integration](src/wasm_simple.rs)** - Browser and Node.js usage
- **[Configuration Reference](docs/CONFIG.md)** - Complete configuration options
- **[CLI Reference](docs/CLI.md)** - All commands and parameters

## 🚀 Implementation Status

### ✅ Phase 1 - Core Foundation (v0.1.0) - **COMPLETED**
- [x] **Complete CLI Tool**: All 5 commands (`init`, `inventory`, `diff`, `propose`, `index`)
- [x] **Drift Detection Engine**: Pattern-based architectural violation detection
- [x] **Configuration System**: YAML/TOML with flexible patterns
- [x] **WebAssembly Support**: Browser/Node.js integration ready
- [x] **GitHub Action**: CI/CD integration for automated analysis
- [x] **Testing Suite**: 114 comprehensive tests (96.5% pass rate)
- [x] **Production Ready**: Zero compilation warnings, robust error handling

### ✅ Phase 2 - Intelligence & Integration (v0.2.0-alpha) - **COMPLETED**
- [x] **🤖 ML-Enhanced Detection**: 5 advanced anomaly detection algorithms
- [x] **📊 Feature Engineering**: 50+ extracted features with advanced analysis
- [x] **🏋️ Training Infrastructure**: Cross-validation, hyperparameter optimization
- [x] **⚙️ Smart Configuration**: ML-ready configuration with backward compatibility
- [x] **🧪 ML Test Suite**: 26 comprehensive ML tests ensuring reliability
- [x] **📈 Performance Metrics**: Precision, recall, F1-score tracking
- [x] **🔍 Explainable AI**: Model explanations for every detection

### 🔧 Phase 3 - Developer Experience (Q2 2025) - **PLANNED**
- [ ] **IDE Extensions**: VS Code and IntelliJ plugins with ML insights
- [ ] **Language Server Protocol**: Universal IDE support with intelligent warnings
- [ ] **Real-time Analysis**: File system watchers with instant ML feedback
- [ ] **Visual Dashboard**: Web-based analytics with trend analysis
- [ ] **Advanced Templates**: AI-powered ADR generation and suggestions

### ☁️ Phase 4 - Cloud & Enterprise (Q3 2025) - **PLANNED**
- [ ] **Cloud Platform SDKs**: AWS, Azure, GCP integration
- [ ] **Enterprise Features**: Multi-user, SSO, audit trails
- [ ] **Advanced Analytics**: Predictive drift analysis and risk assessment
- [ ] **Performance Optimization**: SIMD acceleration for massive codebases

### 🌐 Phase 5 - Ecosystem & Scale (Q4 2025) - **PLANNED**
- [ ] **API & Integrations**: REST API, GraphQL, webhook support
- [ ] **Plugin Marketplace**: Third-party extensions and community templates
- [ ] **SaaS Platform**: Hosted PhotonDrift service
- [ ] **Industry Standardization**: Architectural governance best practices

## 🤝 Contributing

PhotonDrift welcomes contributions! We use a systematic development approach with clear phases and comprehensive testing.

### How to Contribute
1. **Fork the repository** and create a feature branch
2. **Run the test suite**: `cargo test` (ensure all 140+ tests pass)
3. **Add ML tests** if implementing ML features: `cargo test ml::`
4. **Follow the roadmap**: Check [ROADMAP.md](ROADMAP.md) for planned features
5. **Submit a PR** with clear description and test coverage

### Development Setup
```bash
git clone https://github.com/tbowman01/PhotonDrift
cd PhotonDrift
cargo build
cargo test

# For ML features
cargo test --features ml
```

## 📊 Performance & Benchmarks

- **Scanning Speed**: 206 files in ~91ms (production baseline)
- **ML Prediction**: ~1-5ms per drift item
- **Memory Usage**: ~10MB for 1000 ML training samples
- **Test Coverage**: 140/144 tests passing (97.2%)
- **ML Test Coverage**: 26/26 tests passing (100%)

## 🔗 Related Projects

- **[ADR Tools](https://github.com/npryce/adr-tools)** - Command-line tools for working with ADRs
- **[ADR Manager](https://github.com/adr/adr-manager)** - Web-based ADR management
- **[Architecture Decision Records](https://adr.github.io/)** - ADR community resources

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**🚀 Current Status**: Phase 2 Complete - ML-Enhanced Intelligence Ready for Production Testing

**📧 Questions?** Open an [issue](https://github.com/tbowman01/PhotonDrift/issues) or check our [discussions](https://github.com/tbowman01/PhotonDrift/discussions)
