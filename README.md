# PhotonDrift - ADRScan CLI Tool

> Architecture Decision Record (ADR) management and drift detection for modern development teams.

## Overview

PhotonDrift is a Rust-based CLI tool that helps teams manage Architecture Decision Records (ADRs) and detect architectural drift in their codebases. It provides automated scanning, drift detection, and proposal generation to keep architectural documentation synchronized with actual implementation.

## Key Features

- **🚀 CLI Commands**: `init`, `inventory`, `diff`, `propose`, `index`
- **🔍 Drift Detection**: Automatically detect when code deviates from documented decisions
- **📝 Auto-Proposal**: Generate draft ADRs for detected architectural changes  
- **🌐 Multi-Language**: Support for diverse programming languages and project structures
- **⚡ Offline-First**: No network calls required for core functionality
- **🔗 CI/CD Integration**: WebAssembly module and GitHub Action for automation

## Quick Start

```bash
# Initialize ADR structure in your project
adrscan init

# Scan existing ADRs and codebase
adrscan inventory

# Detect architectural drift
adrscan diff

# Generate ADR proposals for detected changes
adrscan propose

# Update ADR index
adrscan index
```

## Documentation

- **[Requirements & Architecture](docs/REQUIREMENTS_SUMMARY.md)** - Detailed technical requirements and implementation phases
- **[Development Roadmap](https://github.com/tbowman01/PhotonDrift/issues)** - GitHub issues tracking implementation progress
- **Installation Guide** _(Coming in Phase 1)_
- **Configuration Reference** _(Coming in Phase 1)_
- **API Documentation** _(Coming in Phase 3)_

## Implementation Status

### Phase 1 - CLI MVP 🏗️ *In Progress*
- [ ] **[Issue #1](https://github.com/tbowman01/PhotonDrift/issues/1)**: Rust project structure
- [ ] **[Issue #2](https://github.com/tbowman01/PhotonDrift/issues/2)**: `init` command
- [ ] **[Issue #3](https://github.com/tbowman01/PhotonDrift/issues/3)**: Frontmatter parsing
- [ ] **[Issue #4](https://github.com/tbowman01/PhotonDrift/issues/4)**: `inventory` command
- [ ] **[Issue #5](https://github.com/tbowman01/PhotonDrift/issues/5)**: `index` command
- [ ] **[Issue #6](https://github.com/tbowman01/PhotonDrift/issues/6)**: Configuration system

### Phase 2 - Drift Detection ✅ *Completed*
- [x] **[Issue #7](https://github.com/tbowman01/PhotonDrift/issues/7)**: Drift detection engine
- [x] **[Issue #8](https://github.com/tbowman01/PhotonDrift/issues/8)**: `diff` command
- [x] **[Issue #9](https://github.com/tbowman01/PhotonDrift/issues/9)**: `propose` command

### Phase 3 - WASM & GitHub Action 🔧 *Planned*
- [ ] **[Issue #10](https://github.com/tbowman01/PhotonDrift/issues/10)**: WebAssembly module
- [ ] **[Issue #11](https://github.com/tbowman01/PhotonDrift/issues/11)**: GitHub Action

## Contributing

PhotonDrift uses a coordinated development approach with GitHub Issues for task management. See our [active issues](https://github.com/tbowman01/PhotonDrift/issues) to get involved.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Next Priority**: [Initialize Rust Project Structure](https://github.com/tbowman01/PhotonDrift/issues/1) - Ready to begin Phase 1 development.