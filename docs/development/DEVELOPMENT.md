# ADRScan Development Guide

This document outlines the development approach and coordination for PhotonDrift ADRScan.

## Project Structure

```
PhotonDrift/
├── docs/                          # Documentation
│   ├── REQUIREMENTS_SUMMARY.md    # Technical requirements
│   └── DEVELOPMENT.md             # This file
├── plans/                         # Research and planning
│   └── research/                  # Original PDF requirements
├── .github/                       # GitHub workflows and templates
│   └── ISSUE_TEMPLATE/            # Issue templates
├── src/                          # Rust source code (Phase 1)
├── Cargo.toml                    # Rust project manifest (Phase 1)
└── README.md                     # Project overview
```

## Development Phases

### Phase 1: CLI MVP (4-6 weeks)
**Status**: 🏗️ In Progress  
**Focus**: Core Rust CLI with basic ADR management

**Critical Path Issues**:
1. **[#1 Rust Project Structure](https://github.com/tbowman01/PhotonDrift/issues/1)** - Foundation 
2. **[#3 Frontmatter Parsing](https://github.com/tbowman01/PhotonDrift/issues/3)** - Core parsing logic
3. **[#6 Configuration System](https://github.com/tbowman01/PhotonDrift/issues/6)** - Flexibility framework

**Parallel Development**:
- **[#2 Init Command](https://github.com/tbowman01/PhotonDrift/issues/2)** - Bootstrap functionality
- **[#4 Inventory Command](https://github.com/tbowman01/PhotonDrift/issues/4)** - Core scanning
- **[#5 Index Command](https://github.com/tbowman01/PhotonDrift/issues/5)** - Documentation generation

### Phase 2: Drift Detection (3-4 weeks)
**Status**: 📋 Planned  
**Focus**: Core drift detection and ADR proposal generation

**Dependencies**: Phase 1 completion  
**Key Issues**:
- **[#7 Drift Detection Engine](https://github.com/tbowman01/PhotonDrift/issues/7)** - Core algorithms
- **[#8 Diff Command](https://github.com/tbowman01/PhotonDrift/issues/8)** - User interface
- **[#9 Propose Command](https://github.com/tbowman01/PhotonDrift/issues/9)** - Auto-generation

### Phase 3: WASM & CI Integration (2-3 weeks)
**Status**: 🔧 Planned  
**Focus**: WebAssembly compilation and GitHub Action

**Dependencies**: Phase 2 completion  
**Key Issues**:
- **[#10 WebAssembly Module](https://github.com/tbowman01/PhotonDrift/issues/10)** - CI/CD integration
- **[#11 GitHub Action](https://github.com/tbowman01/PhotonDrift/issues/11)** - Automation

## Getting Started

### Prerequisites
- Rust 1.70+ with Cargo
- Git and GitHub CLI (`gh`)
- Basic familiarity with ADR concepts

### Development Workflow

1. **Pick an Issue**: Start with [Issue #1](https://github.com/tbowman01/PhotonDrift/issues/1) for foundation work
2. **Create Branch**: `git checkout -b feature/issue-N-description`
3. **Implement**: Follow acceptance criteria in the issue
4. **Test**: Include unit/integration tests
5. **Document**: Update relevant documentation
6. **Pull Request**: Link to the issue and describe changes

### Code Standards
- Follow Rust conventions and use `cargo fmt`
- Include comprehensive error handling
- Add unit tests for all public functions
- Document public APIs with rustdoc
- Use Clap for CLI argument parsing
- Support cross-platform operation (Linux, Windows, macOS)

## Testing Strategy

### Phase 1 Testing
- **Unit Tests**: Frontmatter parsing, file operations, command logic
- **Integration Tests**: End-to-end CLI workflows
- **Platform Tests**: Linux (primary), Windows, macOS (smoke tests)

### Phase 2 Testing  
- **Drift Detection**: Algorithm accuracy and performance
- **Large Repository**: Scalability testing
- **Edge Cases**: Malformed ADRs, missing files, permission issues

### Phase 3 Testing
- **WASM Compatibility**: Node.js and browser environments  
- **GitHub Action**: Real repository integration
- **API Parity**: CLI vs WASM output consistency

## Architecture Decisions

Key architectural decisions will be documented as ADRs in `docs/adr/` once the `init` command is implemented. This follows our own tool's conventions.

## Contributing

All development is coordinated through GitHub Issues. Each issue includes:
- Detailed acceptance criteria
- Priority and dependency information  
- Estimated effort and phase assignment
- Component and label categorization

See our [issue tracker](https://github.com/tbowman01/PhotonDrift/issues) for current tasks and priorities.