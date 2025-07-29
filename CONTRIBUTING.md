# Contributing to PhotonDrift

Thank you for your interest in contributing to PhotonDrift! This guide will help you get started with contributing to our AI-powered ADR management tool.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Security Guidelines](#security-guidelines)
- [Making Contributions](#making-contributions)
- [Code Style](#code-style)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/PhotonDrift.git`
3. Add upstream remote: `git remote add upstream https://github.com/tbowman01/PhotonDrift.git`
4. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git
- GitHub CLI (optional, for some scripts)

### Building the Project

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with all features
cargo test --all-features

# Run benchmarks
cargo bench
```

## Security Guidelines

### 🔒 Dependency Security

We take security seriously. All contributions must pass our automated security checks:

#### Automated Security Scanning

Our CI/CD pipeline automatically runs security audits on every PR and daily on the main branch:

- **cargo-audit**: Checks for known vulnerabilities in dependencies
- **cargo-deny**: Validates licenses and security advisories
- **SARIF reporting**: Results are visible in GitHub's Security tab

#### Running Security Checks Locally

Before submitting a PR, run security checks locally:

```bash
# Install security tools
cargo install cargo-audit cargo-deny

# Run security audit
cargo audit

# Check licenses and advisories
cargo deny check

# Run with our configuration
cargo audit --config .cargo/audit.toml
```

#### Handling Security Vulnerabilities

1. **If you find a vulnerability**: 
   - DO NOT open a public issue
   - Email security@photondrift.io or use GitHub's security reporting
   - Include: description, impact, reproduction steps, suggested fix

2. **If cargo-audit fails**:
   - Check the error message for the vulnerability ID
   - Look for existing issues tracking the vulnerability
   - Update the affected dependency if possible
   - If update isn't possible, document in `.cargo/audit.toml` with justification

3. **Known Issues**:
   - We track `RUSTSEC-2024-0436` (paste crate) in issue #45
   - Check `.cargo/audit.toml` for other acknowledged issues

### 🛡️ Secure Coding Practices

1. **Never use `unsafe` code** without thorough review and justification
2. **Validate all inputs**, especially for ML features
3. **Use secure deserialization** with validation
4. **Implement rate limiting** for resource-intensive operations
5. **Follow the principle of least privilege**
6. **Never commit secrets** - we use pre-commit hooks to prevent this

### 🔐 Pre-commit Security Hooks

We use `detect-secrets` to prevent accidental secret commits:

```bash
# Install pre-commit hooks
./scripts/setup-hooks.sh

# Run secret detection manually
detect-secrets scan --baseline .secrets.baseline
```

## Making Contributions

### Types of Contributions

- **Bug fixes**: Fix issues labeled `bug`
- **Features**: Implement features labeled `enhancement`
- **Documentation**: Improve docs, add examples
- **Performance**: Optimize code, reduce memory usage
- **Security**: Fix vulnerabilities, improve security
- **Tests**: Add missing tests, improve coverage

### Contribution Process

1. **Check existing issues** before starting work
2. **Discuss major changes** in an issue first
3. **Write tests** for new functionality
4. **Update documentation** as needed
5. **Follow code style** guidelines
6. **Pass all checks** before submitting

## Code Style

We use standard Rust formatting:

```bash
# Format your code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Fix clippy suggestions
cargo clippy --fix
```

### Style Guidelines

- Use meaningful variable names
- Add comments for complex logic
- Keep functions focused and small
- Use appropriate error handling
- Document public APIs

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run ML tests
cargo test --features ml

# Run with coverage
cargo tarpaulin --out Html
```

### Writing Tests

- Test edge cases
- Use property-based testing where appropriate
- Mock external dependencies
- Ensure tests are deterministic
- Include both positive and negative test cases

## 📚 Documentation

PhotonDrift uses a modern, automated documentation system built with Docusaurus v3. We welcome and encourage documentation contributions!

### 🛠️ Documentation Setup

```bash
# Navigate to documentation directory
cd docs-site

# Install Node.js dependencies
npm install

# Sync content from source docs
npm run sync-docs

# Generate CLI documentation
npm run generate-cli-docs

# Start development server
npm start
# Visit http://localhost:3000
```

### 📝 Documentation Workflow

#### Quick Contributing Guide

1. **Edit source files** in the `docs/` directory (not `docs-site/docs/`)
2. **Use standard Markdown** with frontmatter for metadata
3. **Test locally** by running sync and development server
4. **Submit pull request** - auto-deployment to GitHub Pages

#### Documentation Structure

```
docs/                           # 📝 Edit files here
├── getting-started/           # User guides and setup
├── development/              # Contributing and development
├── architecture/            # Technical architecture
├── deployment/             # Deployment guides
├── ml-features/           # AI/ML capabilities
├── phase-planning/       # Project roadmaps
└── adr/                 # Architecture Decision Records

docs-site/                     # 🚀 Generated site (don't edit)
├── src/components/           # Custom React components
├── static/                  # Static assets
├── docs/                   # Auto-generated from docs/
└── scripts/               # Build automation
```

#### Content Guidelines

**Frontmatter Example:**
```yaml
---
title: "Page Title"
sidebar_label: "Short Label"
sidebar_position: 1
description: "Brief description for SEO and navigation"
tags: ["relevant", "tags", "here"]
---
```

**Writing Standards:**
- Use **bold** for emphasis and important terms
- Use `code blocks` for commands, file names, and code
- Include practical, testable examples
- Link to related documentation
- Keep paragraphs concise and focused

### 🔧 Advanced Documentation Features

- **Automated Content Sync**: Changes in `docs/` auto-sync to site
- **CLI Documentation Generation**: Extracts help from Rust binary
- **Link Validation**: Automatic checking of internal/external links
- **Progressive Web App**: Offline support and mobile optimization
- **Search Ready**: Configured for Algolia DocSearch integration
- **Performance Optimized**: Code splitting, image optimization, caching

### Code Documentation

- Document all public APIs
- Include examples in doc comments
- Explain complex algorithms
- Document security considerations

### Project Documentation

- Update README.md for user-facing changes
- Add technical docs to `docs/`
- Include architectural decisions in `adr/`
- Update CHANGELOG.md

## Submitting Changes

### Pull Request Process

1. **Update your branch**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run all checks**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   cargo audit
   ```

3. **Create PR**:
   - Use descriptive title
   - Reference related issues
   - Describe changes clearly
   - Add screenshots if applicable

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Security audit passes (`cargo audit`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Commits are logical and well-described

### Review Process

1. Automated checks must pass
2. At least one maintainer review required
3. Security-related changes need security team review
4. Address all review comments
5. Squash commits if requested

## Questions?

- Open an issue for questions
- Join our discussions
- Check existing documentation
- Email: contribute@photondrift.io

Thank you for contributing to PhotonDrift! 🚀