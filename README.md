# PhotonDrift - AI-Powered ADR Management

> Next-generation Architecture Decision Record (ADR) management with ML-enhanced drift detection for intelligent development governance.

[![CI](https://github.com/tbowman01/PhotonDrift/actions/workflows/ci.yml/badge.svg)](https://github.com/tbowman01/PhotonDrift/actions/workflows/ci.yml)
[![Container Build](https://github.com/tbowman01/PhotonDrift/actions/workflows/container-build.yml/badge.svg)](https://github.com/tbowman01/PhotonDrift/actions/workflows/container-build.yml)
[![Security Audit](https://github.com/tbowman01/PhotonDrift/actions/workflows/security-audit.yml/badge.svg)](https://github.com/tbowman01/PhotonDrift/actions/workflows/security-audit.yml)
[![Version](https://img.shields.io/badge/version-0.2.0--alpha.20250721-blue)](https://github.com/tbowman01/PhotonDrift/releases)
[![Tests](https://img.shields.io/badge/tests-178%2F182%20passing-green)](https://github.com/tbowman01/PhotonDrift/actions)
[![ML Coverage](https://img.shields.io/badge/ML%20tests-26%2F26%20passing-brightgreen)](https://github.com/tbowman01/PhotonDrift/tree/main/src/ml)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://rust-lang.org)

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

### 🐳 **Next-Gen Container System** (v2.0)
- **Simplified Build Pipeline**: 60-80% faster builds with intelligent caching
- **Multi-platform Support**: AMD64/ARM64 with optimized cross-compilation
- **Security-First**: Vulnerability scanning, SBOM generation, and attestation
- **Developer-Friendly**: One-command builds with environment automation
- **Production-Ready**: Health checks, monitoring, and enterprise deployment

## 🐳 Quick Start - Container Usage

### Using the Simplified Build System (Recommended)

```bash
# Quick development setup
make dev

# Build for staging with multi-platform support
./scripts/build-automation.sh -e staging all

# Full production pipeline
make prod-all
```

### Using Pre-built Images
### Quick Start with Containers

```bash
# Pull the latest image
docker pull ghcr.io/tbowman01/photondrift:latest

# Run ADR analysis on your project
docker run --rm -v "$(pwd)":/workspace \
  ghcr.io/tbowman01/photondrift:latest \
  diff --adr-dir /workspace/docs/adr

# Interactive container shell
docker run -it --rm -v "$(pwd)":/workspace \
  ghcr.io/tbowman01/photondrift:latest bash
```

### Multi-Service Deployment

```bash
# Using Docker Compose for multi-service setup
version: '3.8'
services:
  photondrift-analyzer:
    image: ghcr.io/tbowman01/photondrift:latest
    volumes:
      - ./docs/adr:/workspace/adr:ro
      - ./src:/workspace/src:ro
    command: diff --adr-dir /workspace/adr --directory /workspace/src
    environment:
      - RUST_LOG=info
      - ADR_CONFIG=/workspace/adr/config.yml

  photondrift-monitor:
    image: ghcr.io/tbowman01/photondrift:latest
    volumes:
      - ./:/workspace:ro
    command: inventory --adr-dir /workspace/docs/adr --watch
    restart: unless-stopped
```

### Available Container Tags

- `latest` - Latest stable release (recommended for production)
- `main` - Latest from main branch (stable development)
- `develop` - Latest development features (testing only)
- `v0.2.0-alpha.20250721` - Specific version tags (reproducible builds)
- `main-<sha>` - Commit-specific builds (debugging)

### Registry Information

**Primary Registry**: `ghcr.io/tbowman01/photondrift`
- **Security**: Images scanned with Trivy, SBOM/Provenance included
- **Platforms**: `linux/amd64`, `linux/arm64` (multi-arch support)
- **Base**: Distroless (security-hardened, minimal attack surface)
- **User**: Non-root (`65532:65532` for security compliance)

### Environment Variables

```bash
# Configuration
RUST_LOG=debug              # Logging level (debug, info, warn, error)
ADR_CONFIG=/workspace/config.yml  # Custom config file location
ADR_DIR=/workspace/adr      # Default ADR directory

# ML Features
ML_ENABLED=true             # Enable machine learning features
ML_MODEL=Ensemble           # ML model type (IsolationForest, Ensemble)
ML_CONFIDENCE=0.7           # Confidence threshold (0.0-1.0)

# Security
TRUST_DNS=1                 # Trust container DNS resolution
NO_PROXY=localhost,127.0.0.1  # Proxy bypass patterns
```

### Volume Mounting Examples

```bash
# Read-only project analysis
docker run --rm \
  -v "$(pwd)":/workspace:ro \
  ghcr.io/tbowman01/photondrift:latest \
  inventory --adr-dir /workspace/docs/adr

# Read-write for generating ADRs
docker run --rm \
  -v "$(pwd)/docs/adr":/workspace/adr \
  -v "$(pwd)/src":/workspace/src:ro \
  ghcr.io/tbowman01/photondrift:latest \
  propose --adr-dir /workspace/adr --directory /workspace/src

# Configuration file mounting
docker run --rm \
  -v "$(pwd)":/workspace:ro \
  -v "$(pwd)/photondrift-config.yml":/config.yml:ro \
  ghcr.io/tbowman01/photondrift:latest \
  diff --config /config.yml
```

### Build Locally

```bash
# Clone and build
git clone https://github.com/tbowman01/PhotonDrift.git
cd PhotonDrift
docker build -t photondrift:local .

# Multi-platform build
docker buildx build --platform linux/amd64,linux/arm64 \
  -t photondrift:multi-arch .

# See comprehensive build guide
# docs/DOCKER_BUILD_GUIDE.md
```

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
- **[User Guide](docs/USER_GUIDE.md)** - Comprehensive usage guide
- **[Quick Start](docs/QUICK_START.md)** - Get started quickly
- **[Development Guide](docs/DEVELOPMENT.md)** - Contributing guidelines
- **[Configuration](docs/CONFIGURATION.md)** - Configuration options

### ML & Security Features ✨
- **[ML Security Guide](docs/ML_SECURITY_GUIDE.md)** - AI-powered security analysis and secret detection
- **[Neural Training](docs/NEURAL_TRAINING.md)** - Train models from operations and improve accuracy
- **[Performance Analysis](docs/PERFORMANCE_ANALYSIS.md)** - Monitor performance and optimize bottlenecks

### Integration & Automation
- **[GitHub Integration](docs/GITHUB_LABELS.md)** - GitHub automation features
- **[Development Hooks](docs/DEVELOPMENT_HOOKS.md)** - Pre-commit hooks and automation setup
- **[Docker Build Guide](docs/DOCKER_BUILD_GUIDE.md)** - Comprehensive Docker build instructions

### Technical Reference
- **[CHANGELOG.md](CHANGELOG.md)** - Complete version history and release notes
- **[ROADMAP.md](ROADMAP.md)** - Development roadmap through 2025
- **[Requirements & Architecture](docs/REQUIREMENTS_SUMMARY.md)** - Technical requirements and phases
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
- [x] **🐳 Docker Support**: Production-ready containerization
- [x] **🛠️ DevOps Pipeline**: Enhanced CI/CD with comprehensive automation

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
2. **Setup development environment**: Run `./setup-hooks.sh` for automated setup
3. **Run the test suite**: `cargo test` (ensure all 178+ tests pass)
4. **Add ML tests** if implementing ML features: `cargo test ml::`
5. **Follow pre-commit hooks**: Code formatting, linting, and testing are automated
6. **Follow the roadmap**: Check [ROADMAP.md](ROADMAP.md) for planned features
7. **Submit a PR** with clear description and test coverage

### Development Setup

```bash
git clone https://github.com/tbowman01/PhotonDrift
cd PhotonDrift

# Setup development environment with pre-commit hooks
./setup-hooks.sh

cargo build
cargo test

# For ML features
cargo test --features ml
```

### Code Quality

The project uses comprehensive pre-commit hooks to ensure code quality:

- **Rust formatting** with `rustfmt`
- **Linting** with `clippy` (warnings as errors)
- **Compilation checks** with `cargo check`
- **Test suite** execution
- **Security scanning** for secrets and patterns
- **File quality** checks (trailing whitespace, line endings, etc.)

See [Development Hooks Documentation](docs/DEVELOPMENT_HOOKS.md) for detailed information.

## 📊 Performance & Benchmarks

- **Scanning Speed**: 206 files in ~91ms (production baseline)
- **ML Prediction**: ~1-5ms per drift item
- **Memory Usage**: ~10MB for 1000 ML training samples
- **Test Coverage**: 178/182 tests passing (97.8%)
- **ML Test Coverage**: 26/26 tests passing (100%)

## 🔗 Related Projects

- **[ADR Tools](https://github.com/npryce/adr-tools)** - Command-line tools for working with ADRs
- **[ADR Manager](https://github.com/adr/adr-manager)** - Web-based ADR management
- **[Architecture Decision Records](https://adr.github.io/)** - ADR community resources

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**🚀 Current Status**: Phase 2 Complete - Production-Ready Alpha with Enhanced DevOps Pipeline

**📧 Questions?** Open an [issue](https://github.com/tbowman01/PhotonDrift/issues) or check our [discussions](https://github.com/tbowman01/PhotonDrift/discussions)