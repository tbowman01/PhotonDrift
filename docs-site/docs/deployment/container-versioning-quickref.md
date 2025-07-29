---
id: "container-versioning-quickref"
title: "ContAIner Versioning Quickref"
sidebar_label: "ContAIner Versioning Quickref"
sidebar_position: "1"
description: "Deploy and operate PhotonDrift in production"
slug: "/deployment/container-versioning-quickref"
tags: ["deployment"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

# Container Versioning Quick Reference

## Tag Patterns Overview

| Tag Pattern | Example | When to Use | Mutable |
|------------|---------|-------------|---------|
| `latest` | `photondrift:latest` | Latest stable from main | Yes |
| `stable` | `photondrift:stable` | Latest stable release | Yes |
| `edge` | `photondrift:edge` | Latest from develop | Yes |
| `X.Y.Z` | `photondrift:0.2.0` | Specific version | No |
| `X.Y` | `photondrift:0.2` | Latest patch of minor | Yes |
| `X` | `photondrift:1` | Latest minor of major | Yes |
| `main` | `photondrift:main` | Latest from main branch | Yes |
| `develop` | `photondrift:develop` | Latest from develop | Yes |
| `sha-XXXXXXXX` | `photondrift:sha-a1b2c3d4` | Specific commit | No |
| `YYYYMMDD-XXXXXXXX` | `photondrift:20250728-a1b2c3d4` | Date + commit | No |
| `pr-NNN` | `photondrift:pr-123` | Pull request build | Yes |
| `feature-XXX` | `photondrift:feature-auth` | Feature branch | Yes |
| `nightly` | `photondrift:nightly` | Latest nightly | Yes |
| `nightly-YYYYMMDD` | `photondrift:nightly-20250728` | Specific nightly | No |

## Common Use Cases

### Production Deployment
```bash
# Most stable
docker pull ghcr.io/tbowman01/photondrift:stable
docker pull ghcr.io/tbowman01/photondrift:0.2.0

# Latest from main (rolling)
docker pull ghcr.io/tbowman01/photondrift:latest
```

### Development/Testing
```bash
# Bleeding edge
docker pull ghcr.io/tbowman01/photondrift:edge
docker pull ghcr.io/tbowman01/photondrift:develop

# Specific PR
docker pull ghcr.io/tbowman01/photondrift:pr-123

# Specific commit
docker pull ghcr.io/tbowman01/photondrift:sha-a1b2c3d4
```

### CI/CD Integration
```yaml
# In docker-compose.yml
services:
  adrscan:
    image: ghcr.io/tbowman01/photondrift:${VERSION:-latest}
    
# In Kubernetes
spec:
  containers:
  - name: adrscan
    image: ghcr.io/tbowman01/photondrift:0.2.0  # Pinned version
```

## Architecture Selection

### Automatic (Recommended)
```bash
# Docker automatically selects correct architecture
docker pull ghcr.io/tbowman01/photondrift:latest
```

### Manual Selection
```bash
# Force specific architecture
docker pull --platform linux/amd64 ghcr.io/tbowman01/photondrift:latest
docker pull --platform linux/arm64 ghcr.io/tbowman01/photondrift:latest
```

## Image Verification

### Check Image Details
```bash
# View all tags
docker images ghcr.io/tbowman01/photondrift

# Inspect labels
docker inspect ghcr.io/tbowman01/photondrift:latest | jq '.[0].Config.Labels'

# Check digest
docker inspect ghcr.io/tbowman01/photondrift:latest | jq -r '.[0].RepoDigests[]'
```

### Verify Signatures
```bash
# Using cosign
cosign verify ghcr.io/tbowman01/photondrift:latest \
  --certificate-identity-regexp=https://github.com/tbowman01/PhotonDrift/.* \
  --certificate-oidc-issuer=https://token.actions.githubusercontent.com
```

## Version Lifecycle

```
Feature Development:
feature/auth → feature-auth → pr-123 → develop → edge

Release Process:
develop → edge → main → latest → tag v0.2.0 → 0.2.0, 0.2, stable

Hotfix Process:
hotfix/critical → main → latest → tag v0.2.1 → 0.2.1, 0.2, stable
```

## Troubleshooting

### Wrong Architecture
```bash
# Check current architecture
docker version --format '{{.Server.Arch}}'

# Force correct platform
docker run --platform linux/amd64 ghcr.io/tbowman01/photondrift:latest
```

### Old Version Cached
```bash
# Force fresh pull
docker pull ghcr.io/tbowman01/photondrift:latest --no-cache

# Remove local image
docker rmi ghcr.io/tbowman01/photondrift:latest
docker pull ghcr.io/tbowman01/photondrift:latest
```

### Find Specific Build
```bash
# By date
docker pull ghcr.io/tbowman01/photondrift:20250728-a1b2c3d4

# By commit
docker pull ghcr.io/tbowman01/photondrift:sha-a1b2c3d4

# Immutable reference
docker pull ghcr.io/tbowman01/photondrift@sha256:abc123...
```

## Best Practices

1. **Production**: Always use immutable tags (version numbers or digests)
2. **Staging**: Use `latest` or `main` for automatic updates
3. **Development**: Use `edge` or `develop` for latest features
4. **CI/CD**: Pin to specific versions, update deliberately
5. **Testing**: Use SHA-based tags for reproducibility

## Environment Variables

When running the container, these variables are available:

```bash
docker run --rm ghcr.io/tbowman01/photondrift:latest env | grep ADRSCAN
# ADRSCAN_VERSION=0.2.0
# ADRSCAN_BUILD_DATE=2025-07-28T10:30:00Z
# ADRSCAN_COMMIT=a1b2c3d4
# ADRSCAN_BRANCH=main
```