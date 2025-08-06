---
id: "container-versioning-strategy"
title: "ContAIner Versioning Strategy"
sidebar_label: "ContAIner Versioning Strategy"
sidebar_position: "1"
description: "Miscellaneous documentation and guides"
slug: "/misc/container-versioning-strategy"
tags: ["misc"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

---
title: "Container Versioning Strategy for PhotonDrift"
sidebar_label: "Versioning Strategy"
sidebar_position: 4
description: "Comprehensive container versioning strategy for PhotonDrift project"
tags: ["containers", "versioning", "strategy", "deployment"]
---

# Container Versioning Strategy for PhotonDrift

## Overview

This document outlines a comprehensive container versioning strategy for the PhotonDrift project that provides clear traceability, supports multiple deployment scenarios, and enables efficient multi-architecture deployments.

## Versioning Components

### 1. Semantic Versioning (Primary Tags)

Based on git tags following [Semantic Versioning 2.0.0](https://semver.org/):

- **Format**: `vMAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]`
- **Examples**:
  - `v0.2.0` - Stable release
  - `v0.2.0-alpha.20250721` - Alpha pre-release
  - `v0.2.0-beta.1` - Beta pre-release
  - `v0.2.0-rc.1` - Release candidate

**Container Tags Generated**:
```
ghcr.io/tbowman01/photondrift:0.2.0
ghcr.io/tbowman01/photondrift:0.2
ghcr.io/tbowman01/photondrift:0
ghcr.io/tbowman01/photondrift:0.2.0-alpha.20250721  # For pre-releases
```

### 2. Branch-Based Tags

Tags generated from branch names for continuous deployment:

- **main branch**: 
  - `latest` - Always points to the latest main branch build
  - `stable` - Points to the latest stable release (no pre-release)
  - `main` - Explicit main branch reference
  - `main-{SHA}` - Main branch with commit SHA

- **develop branch**:
  - `develop` - Latest develop branch build
  - `edge` - Alias for develop (bleeding edge)
  - `develop-{SHA}` - Develop branch with commit SHA

- **feature branches**:
  - `feature-{branch-name}` - Sanitized branch name
  - `feature-{branch-name}-{SHA}` - With commit SHA

- **pull requests**:
  - `pr-{number}` - Pull request builds
  - `pr-{number}-{SHA}` - PR with commit SHA

### 3. Commit SHA Tags

For complete traceability:

- **Short SHA**: `sha-{8-char-sha}` - First 8 characters of commit
- **Full SHA**: `sha-{full-sha}` - Complete commit SHA (on demand)
- **Date-SHA**: `{YYYYMMDD}-{8-char-sha}` - Date prefix for chronological sorting

### 4. Multi-Architecture Support

Each tag supports multiple architectures with manifest lists:

- **Architectures**:
  - `linux/amd64` - x86_64 architecture
  - `linux/arm64` - ARM 64-bit (Apple Silicon, AWS Graviton)
  - `linux/arm/v7` - ARM 32-bit (optional, for embedded devices)

- **Architecture-Specific Tags** (for debugging):
  - `{tag}-amd64` - AMD64-only image
  - `{tag}-arm64` - ARM64-only image

### 5. Special Purpose Tags

- **latest**: Always points to the most recent stable release from main
- **edge**: Always points to the most recent develop build
- **canary**: Special tag for canary deployments
- **nightly**: Automated nightly builds from develop

## Implementation in GitHub Actions

### Enhanced Metadata Generation

```yaml
- name: Generate metadata
  id: meta
  uses: docker/metadata-action@v5
  with:
    images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}
    tags: |
      # Branch-based tags
      type=ref,event=branch
      type=ref,event=pr
      
      # Semantic versioning (from git tags)
      type=semver,pattern={{version}}
      type=semver,pattern={{major}}.{{minor}}
      type=semver,pattern={{major}},enable=${{ !startsWith(github.ref, 'refs/tags/v0.') }}
      
      # Latest tags
      type=raw,value=latest,enable={{is_default_branch}}
      type=raw,value=stable,enable=${{ github.event_name == 'release' && !contains(github.ref, '-') }}
      type=raw,value=edge,enable=${{ github.ref == 'refs/heads/develop' }}
      
      # SHA-based tags
      type=sha,prefix=sha-,format=short
      type=raw,value={{date 'YYYYMMDD'}}-{{sha}}
      
      # Branch with SHA
      type=raw,value=${{ github.ref_name }}-{{sha}}
      
      # Special tags
      type=raw,value=canary,enable=${{ github.event_name == 'workflow_dispatch' && github.event.inputs.canary == 'true' }}
      type=schedule,pattern=nightly,enable=${{ github.event_name == 'schedule' }}
    
    labels: |
      org.opencontainers.image.title=ADRScan
      org.opencontainers.image.description=AI-powered Architecture Decision Record (ADR) management
      org.opencontainers.image.licenses=MIT
      org.opencontainers.image.vendor=PhotonDrift
      maintainer=tbowman01
      
      # Build metadata
      build.timestamp={{date 'YYYY-MM-DDTHH:mm:ssZ'}}
      build.version={{version}}
      build.commit={{sha}}
      build.branch={{branch}}
```

### Build Arguments for Versioning

```yaml
build-args: |
  BUILD_DATE=${{ env.BUILD_DATE }}
  GIT_SHA=${{ github.sha }}
  GIT_REF=${{ github.ref }}
  VERSION=${{ steps.meta.outputs.version }}
  SEMVER=${{ steps.version.outputs.semver }}
  BRANCH=${{ github.ref_name }}
```

## Tag Lifecycle Management

### 1. Development Cycle

```
develop branch → edge, develop, develop-{sha}
    ↓ (merge)
main branch → latest, main, main-{sha}
    ↓ (tag)
release → v0.2.0, 0.2.0, 0.2, stable
```

### 2. Feature Development

```
feature/xyz branch → feature-xyz, feature-xyz-{sha}
    ↓ (PR)
pull request → pr-123, pr-123-{sha}
    ↓ (merge)
develop branch → edge, develop-{sha}
```

### 3. Hotfix Process

```
hotfix/critical branch → hotfix-critical, hotfix-critical-{sha}
    ↓ (merge to main)
main branch → latest, main-{sha}
    ↓ (tag)
release → v0.2.1, 0.2.1, stable
```

## Version Resolution Examples

### For Different Deployment Scenarios

1. **Production Deployment**:
   ```bash
   # Stable release
   docker pull ghcr.io/tbowman01/photondrift:stable
   # Or specific version
   docker pull ghcr.io/tbowman01/photondrift:0.2.0
   ```

2. **Staging Deployment**:
   ```bash
   # Latest from main
   docker pull ghcr.io/tbowman01/photondrift:latest
   # Or main branch explicitly
   docker pull ghcr.io/tbowman01/photondrift:main
   ```

3. **Development Deployment**:
   ```bash
   # Edge/develop version
   docker pull ghcr.io/tbowman01/photondrift:edge
   # Or specific develop build
   docker pull ghcr.io/tbowman01/photondrift:develop-a1b2c3d4
   ```

4. **Testing Specific Commit**:
   ```bash
   # By commit SHA
   docker pull ghcr.io/tbowman01/photondrift:sha-a1b2c3d4
   # Or by date-SHA
   docker pull ghcr.io/tbowman01/photondrift:20250728-a1b2c3d4
   ```

5. **Pull Request Testing**:
   ```bash
   # PR number
   docker pull ghcr.io/tbowman01/photondrift:pr-123
   ```

## Multi-Architecture Considerations

### Build Strategy

1. **Default Multi-arch Build**:
   - Production releases: `linux/amd64,linux/arm64`
   - Development builds: `linux/amd64,linux/arm64`
   - PR builds: `linux/amd64` only (faster CI)

2. **Platform Detection**:
   ```dockerfile
   # In Dockerfile
   ARG TARGETPLATFORM
   ARG TARGETARCH
   ARG TARGETVARIANT
   
   # Use for conditional compilation
   RUN echo "Building for ${TARGETPLATFORM}"
   ```

3. **Manifest Lists**:
   - All tags automatically create manifest lists
   - Users pull appropriate architecture automatically
   - Can force architecture: `--platform linux/arm64`

## Security Considerations

1. **Immutable Tags**:
   - Version tags (e.g., `0.2.0`) are immutable
   - SHA-based tags are inherently immutable
   - Rolling tags (`latest`, `edge`) are mutable by design

2. **Signing and Attestation**:
   - All images signed with cosign
   - SBOM (Software Bill of Materials) included
   - Build provenance attestation attached

3. **Vulnerability Scanning**:
   - Automated scanning on all builds
   - Security status in image labels
   - CVE tracking in release notes

## Migration Strategy

To implement this versioning strategy:

1. **Phase 1**: Update GitHub Actions workflow with new tagging rules
2. **Phase 2**: Document new tags in README and release notes  
3. **Phase 3**: Notify users of new tagging scheme
4. **Phase 4**: Maintain backward compatibility for 3 releases
5. **Phase 5**: Deprecate old tags after transition period

## Monitoring and Maintenance

1. **Tag Cleanup**:
   - Remove PR tags after merge
   - Prune SHA tags older than 90 days
   - Keep all version tags permanently

2. **Usage Analytics**:
   - Monitor pull statistics per tag
   - Track architecture distribution
   - Identify unused tags for cleanup

3. **Documentation**:
   - Update README with current tags
   - Maintain CHANGELOG with tag history
   - Document breaking changes clearly