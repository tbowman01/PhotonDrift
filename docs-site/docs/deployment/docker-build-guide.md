---
id: "docker-build-guide"
title: "DOCKER BUILD GUIDE"
sidebar_label: "DOCKER BUILD GUIDE"
sidebar_position: "1"
description: "Deploy and operate PhotonDrift in production"
slug: "/deployment/docker-build-guide"
tags: ["deployment"]
last_update:
  date: "2025-07-29"
  author: "tbowman01"
---

# Docker Build Guide

This guide covers the simplified PhotonDrift container build system with automated workflows and optimized performance.

## 🚀 Quick Start

The build system is now fully automated with smart defaults:

### Using the Build Script (Recommended)

```bash
# Quick development build
./scripts/build-automation.sh build

# Build for staging with multi-platform
./scripts/build-automation.sh -e staging -p linux/amd64,linux/arm64 build

# Full production pipeline (build + test + scan + push)
./scripts/build-automation.sh -e prod all
```

### Manual Build (Advanced)

```bash
# Basic build with optimized Dockerfile
docker build -f Dockerfile.optimized -t photondrift:local .

# Multi-platform build
docker buildx build --platform linux/amd64,linux/arm64 -t photondrift:latest .
```

## Prerequisites

- Docker 20.10+ installed
- Docker Buildx for multi-platform builds
- Git for cloning the repository
- At least 2GB free disk space

## Build System Architecture

### Simplified Workflow Structure

1. **Reusable GitHub Action** (`.github/actions/docker-build/`)
   - Standardized build, cache, and security scanning
   - Configurable for any service or environment
   
2. **Matrix Build Pipeline** (`.github/workflows/matrix-build.yml`)
   - Parallel builds across services and environments
   - Smart dependency caching and optimization
   
3. **Optimized Dockerfile** (`Dockerfile.optimized`)
   - Enhanced dependency caching with separate stages
   - Reduced rebuild times by 60-80%
   
4. **Build Automation Script** (`scripts/build-automation.sh`)
   - Local development and CI/CD automation
   - Environment-aware builds with smart defaults

### Build Configurations

```bash
# Service-specific builds
./scripts/build-automation.sh -s cli build                    # CLI only
./scripts/build-automation.sh -s dashboard-backend build      # Backend API
./scripts/build-automation.sh -s all build                    # All services

# Environment-specific builds
./scripts/build-automation.sh -e dev build        # Development (AMD64, local)
./scripts/build-automation.sh -e staging build    # Staging (multi-platform, push)
./scripts/build-automation.sh -e prod build       # Production (optimized, secured)

# Custom configurations
./scripts/build-automation.sh -p linux/arm64 -t my-tag build
```

### Platform and Caching Optimizations

```bash
# Enhanced caching for faster rebuilds
./scripts/build-automation.sh --cache build

# Skip cache for clean builds
./scripts/build-automation.sh --no-cache build

# Verbose output for debugging
./scripts/build-automation.sh --verbose build
```

## Automated Testing and Security

### Integrated Test Suite

```bash
# Run all automated tests
./scripts/build-automation.sh test

# Full pipeline with security scanning
./scripts/build-automation.sh scan
```

The automated test suite includes:
- **Functionality Tests**: Version check, help command, ADR processing
- **Security Tests**: Non-root execution, vulnerability scanning
- **Performance Tests**: Image size, startup time validation
- **Integration Tests**: Multi-service connectivity (for dashboard components)

### Security Scanning

Built-in Trivy security scanning:
- **Critical/High vulnerabilities** - Fail builds in production
- **SARIF report generation** - Integrated with GitHub Security tab
- **Supply chain validation** - SBOM and provenance attestation

## CI/CD Integration

### Simplified GitHub Actions

The new build system provides multiple automation levels:

1. **Primary Build Pipeline** (`.github/workflows/container-build.yml`)
   - Single unified job with conditional logic
   - Environment-aware builds (dev/staging/prod)
   - Automatic security scanning and attestation

2. **Matrix Build Pipeline** (`.github/workflows/matrix-build.yml`)
   - Parallel builds across multiple services
   - Customizable service and environment selection
   - Build result aggregation and reporting

3. **Reusable Workflows** (`.github/workflows/build-configs.yml`)
   - Service-specific build configurations
   - Environment-specific optimizations
   - Standardized across all components

### Local CI Simulation

```bash
# Simulate CI environment locally
GITHUB_REPOSITORY=tbowman01/photondrift ./scripts/build-automation.sh -e staging all

# Test specific GitHub Actions workflow
act -j build-test-publish
```

## Performance Optimizations

### Automatic Optimizations

The new build system includes several performance improvements:

- **60-80% faster rebuilds** through enhanced dependency caching
- **Multi-stage optimization** with separate dependency and source layers
- **Registry cache integration** for cross-environment builds
- **Parallel matrix builds** for multiple services/platforms
- **Smart cache invalidation** based on file changes

### Manual Optimizations

```bash
# Use optimized Dockerfile
docker build -f Dockerfile.optimized -t photondrift:optimized .

# Enable experimental features
DOCKER_BUILDKIT=1 docker build --progress=plain -t photondrift:fast .

# Custom cache configuration
./scripts/build-automation.sh --cache build
```

## Migration Guide

### From Previous Build System

If you were using the previous build system:

```bash
# Old way
docker build -t photondrift .

# New way (equivalent)
./scripts/build-automation.sh build

# Old way (CI)
# Manual workflow triggers

# New way (CI)
gh workflow run matrix-build.yml -f services=cli -f environments=staging
```

### Environment Migration

- **Development**: Builds are local-only by default
- **Staging**: Multi-platform builds with registry push
- **Production**: Full security scanning and attestation

## Best Practices

### Automated Compliance

The new system enforces best practices automatically:

1. ✅ **Immutable tags** - Environment-based tagging strategy
2. ✅ **Security scanning** - Integrated Trivy vulnerability assessment
3. ✅ **Multi-stage builds** - Optimized dependency caching
4. ✅ **Pinned base images** - Controlled through build arguments
5. ✅ **Rich metadata** - OCI-compliant labels and attestation
6. ✅ **Automated testing** - Built-in functionality and security tests
7. ✅ **BuildKit optimization** - Default in all workflows
8. ✅ **Cache management** - Automatic cleanup and optimization

### Additional Recommendations

- Use environment-specific builds (`-e dev/staging/prod`)
- Enable verbose output for debugging (`--verbose`)
- Monitor build performance with GitHub Actions insights
- Review security scan results in GitHub Security tab
- Use matrix builds for comprehensive testing across platforms

## Troubleshooting

### Common Issues

#### Build Script Permission Error
```bash
chmod +x scripts/build-automation.sh
```

#### Docker Buildx Not Found
```bash
docker buildx install
# or
./scripts/build-automation.sh build  # Auto-installs buildx
```

#### Cache Issues
```bash
# Clear all caches
./scripts/build-automation.sh --no-cache build

# Reset buildx
docker buildx rm photondrift-builder
```

## Docker Compose Integration

Create `docker-compose.yml` for simplified development:

```yaml
version: '3.8'

services:
  photondrift:
    image: ghcr.io/tbowman01/photondrift:dev
    volumes:
      - ./docs/adr:/workspace/adr
      - ./output:/workspace/output
    command: inventory --adr-dir /workspace/adr --output /workspace/output
    
  dashboard-backend:
    image: ghcr.io/tbowman01/photondrift-dashboard-backend:dev
    ports:
      - "3000:3000"
    environment:
      - DATABASE_URL=postgresql://localhost/photondrift
      
  dashboard-frontend:
    image: ghcr.io/tbowman01/photondrift-dashboard-frontend:dev
    ports:
      - "8080:80"
    depends_on:
      - dashboard-backend
```

Run with:
```bash
# Build all services
./scripts/build-automation.sh -s all -e dev build

# Start stack
docker-compose up
```

## Additional Resources

- [Build Automation Script Documentation](../scripts/build-automation.sh)
- [GitHub Actions Workflows](../.github/workflows/)
- [Reusable Docker Action](../.github/actions/docker-build/)
- [Dockerfile Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [Container Security Guide](https://docs.docker.com/engine/security/)

---

*Build system simplified and optimized - Last updated: 2025-07-22*