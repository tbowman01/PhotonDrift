---
id: "container-versioning-implementation"
title: "ContAIner Versioning Implementation"
sidebar_label: "ContAIner Versioning Implementation"
sidebar_position: "1"
description: "Miscellaneous documentation and guides"
slug: "/misc/container-versioning-implementation"
tags: ["misc"]
last_update:
  date: "2025-07-28"
  author: "tbowman01"
---

---
title: "Container Versioning Implementation Guide"
sidebar_label: "Implementation"
sidebar_position: 2
description: "Implementation guide for container versioning strategy in PhotonDrift"
tags: ["containers", "versioning", "implementation", "docker"]
---

# Container Versioning Implementation Guide

## Updated GitHub Actions Workflow

This guide provides the enhanced container-build.yml workflow that implements the comprehensive versioning strategy.

### Complete Enhanced Workflow

```yaml
name: Container Build and Publish

on:
  workflow_dispatch:
    inputs:
      platforms:
        description: 'Target platforms'
        required: false
        default: 'linux/amd64,linux/arm64'
        type: string
      push_to_registry:
        description: 'Push to container registry'
        required: false
        default: 'true'
        type: boolean
      canary:
        description: 'Mark as canary release'
        required: false
        default: 'false'
        type: boolean
  push:
    branches: [ main, develop ]
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'Cargo.lock'
      - 'Dockerfile'
      - '.dockerignore'
      - '.github/workflows/container-build.yml'
  pull_request:
    branches: [ main, develop ]
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'Dockerfile'
      - '.dockerignore'
  release:
    types: [ published ]
  schedule:
    # Nightly builds at 2 AM UTC
    - cron: '0 2 * * *'

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}
  PLATFORMS: ${{ github.event.inputs.platforms || 'linux/amd64,linux/arm64' }}
  SHOULD_PUSH: ${{ github.event_name == 'release' || github.event_name == 'push' || github.event_name == 'schedule' || (github.event_name == 'workflow_dispatch' && github.event.inputs.push_to_registry == 'true') }}

jobs:
  build:
    name: Build Multi-Arch Container
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write
      security-events: write
      attestations: write
      id-token: write
    
    outputs:
      image-tag: ${{ steps.meta.outputs.tags }}
      image-digest: ${{ steps.build.outputs.digest }}
      version: ${{ steps.meta.outputs.version }}

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Full history for version detection

      - name: Setup build environment
        id: setup
        run: |
          # Environment setup
          echo "IMAGE_NAME_LOWER=${IMAGE_NAME,,}" >> $GITHUB_ENV
          echo "BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> $GITHUB_ENV
          echo "GIT_SHA=${GITHUB_SHA::8}" >> $GITHUB_ENV
          echo "GIT_SHA_FULL=${GITHUB_SHA}" >> $GITHUB_ENV
          
          # Branch name sanitization for tags
          BRANCH_NAME="${GITHUB_REF_NAME//\//-}"
          echo "BRANCH_NAME_CLEAN=${BRANCH_NAME//[^a-zA-Z0-9-]/-}" >> $GITHUB_ENV
          
          # Determine build type
          if [[ "${{ github.event_name }}" == "pull_request" ]]; then
            echo "BUILD_TYPE=pr" >> $GITHUB_ENV
            echo "PR_NUMBER=${{ github.event.pull_request.number }}" >> $GITHUB_ENV
          elif [[ "${{ github.event_name }}" == "release" ]]; then
            echo "BUILD_TYPE=release" >> $GITHUB_ENV
          elif [[ "${{ github.event_name }}" == "schedule" ]]; then
            echo "BUILD_TYPE=nightly" >> $GITHUB_ENV
          elif [[ "${{ github.ref }}" == "refs/heads/develop" ]]; then
            echo "BUILD_TYPE=edge" >> $GITHUB_ENV
          elif [[ "${{ github.ref }}" == "refs/heads/main" ]]; then
            echo "BUILD_TYPE=stable" >> $GITHUB_ENV
          else
            echo "BUILD_TYPE=dev" >> $GITHUB_ENV
          fi

      - name: Extract version information
        id: version
        run: |
          # Get latest tag
          LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
          echo "latest_tag=${LATEST_TAG}" >> $GITHUB_OUTPUT
          
          # Extract semver
          if [[ "${LATEST_TAG}" =~ ^v?([0-9]+)\.([0-9]+)\.([0-9]+)(-.*)?$ ]]; then
            echo "semver=${BASH_REMATCH[1]}.${BASH_REMATCH[2]}.${BASH_REMATCH[3]}" >> $GITHUB_OUTPUT
            echo "major=${BASH_REMATCH[1]}" >> $GITHUB_OUTPUT
            echo "minor=${BASH_REMATCH[2]}" >> $GITHUB_OUTPUT
            echo "patch=${BASH_REMATCH[3]}" >> $GITHUB_OUTPUT
            echo "prerelease=${BASH_REMATCH[4]}" >> $GITHUB_OUTPUT
          fi
          
          # Check if current commit is tagged
          CURRENT_TAG=$(git tag --points-at HEAD | grep "^v" | head -1)
          echo "current_tag=${CURRENT_TAG}" >> $GITHUB_OUTPUT
          echo "is_tagged=$([[ -n "${CURRENT_TAG}" ]] && echo "true" || echo "false")" >> $GITHUB_OUTPUT

      - name: Set up QEMU
        uses: docker/setup-qemu-action@v3

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
        with:
          platforms: ${{ env.PLATFORMS }}

      - name: Log in to Container Registry
        if: env.SHOULD_PUSH == 'true'
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Generate metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}
          flavor: |
            latest=false
          tags: |
            # Branch-based tags
            type=ref,event=branch
            type=ref,event=pr,prefix=pr-
            
            # Semantic versioning (from git tags)
            type=semver,pattern={{version}},enable=${{ steps.version.outputs.is_tagged == 'true' }}
            type=semver,pattern={{major}}.{{minor}},enable=${{ steps.version.outputs.is_tagged == 'true' }}
            type=semver,pattern={{major}},enable=${{ steps.version.outputs.is_tagged == 'true' && steps.version.outputs.major != '0' }}
            
            # Latest tags
            type=raw,value=latest,enable=${{ github.ref == 'refs/heads/main' && !contains(steps.version.outputs.prerelease, '-') }}
            type=raw,value=stable,enable=${{ github.event_name == 'release' && !contains(github.ref, '-') }}
            type=raw,value=edge,enable=${{ github.ref == 'refs/heads/develop' }}
            
            # SHA-based tags
            type=sha,prefix=sha-,format=short
            type=raw,value={{date 'YYYYMMDD'}}-${{ env.GIT_SHA }}
            
            # Branch with SHA
            type=raw,value=${{ env.BRANCH_NAME_CLEAN }}-${{ env.GIT_SHA }}
            
            # Feature branch tags
            type=raw,value=feature-${{ env.BRANCH_NAME_CLEAN }},enable=${{ startsWith(github.ref, 'refs/heads/feature/') }}
            
            # Special tags
            type=raw,value=canary,enable=${{ github.event.inputs.canary == 'true' }}
            type=raw,value=nightly,enable=${{ github.event_name == 'schedule' }}
            type=raw,value=nightly-{{date 'YYYYMMDD'}},enable=${{ github.event_name == 'schedule' }}
          
          labels: |
            org.opencontainers.image.title=ADRScan
            org.opencontainers.image.description=AI-powered Architecture Decision Record (ADR) management with ML-enhanced drift detection
            org.opencontainers.image.licenses=MIT
            org.opencontainers.image.vendor=PhotonDrift
            org.opencontainers.image.authors=tbowman01
            org.opencontainers.image.source=https://github.com/tbowman01/PhotonDrift
            org.opencontainers.image.documentation=https://github.com/tbowman01/PhotonDrift/blob/main/README.md
            
            # Build metadata
            build.timestamp=${{ env.BUILD_DATE }}
            build.version=${{ steps.version.outputs.semver || '0.0.0' }}
            build.commit=${{ env.GIT_SHA_FULL }}
            build.commit.short=${{ env.GIT_SHA }}
            build.branch=${{ github.ref_name }}
            build.type=${{ env.BUILD_TYPE }}
            
            # Version info
            version.major=${{ steps.version.outputs.major || '0' }}
            version.minor=${{ steps.version.outputs.minor || '0' }}
            version.patch=${{ steps.version.outputs.patch || '0' }}
            version.prerelease=${{ steps.version.outputs.prerelease || '' }}
            
            # Security
            security.scan=enabled
            security.nonroot=true

      # Build container with enhanced build args
      - name: Build container image
        id: build
        uses: docker/build-push-action@v6
        with:
          context: .
          file: ./Dockerfile
          platforms: ${{ github.event_name == 'pull_request' && 'linux/amd64' || env.PLATFORMS }}
          push: ${{ env.SHOULD_PUSH }}
          load: ${{ github.event_name == 'pull_request' }}
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: |
            type=gha
            type=registry,ref=${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}:buildcache
          cache-to: |
            type=gha,mode=max
            ${{ env.SHOULD_PUSH == 'true' && format('type=registry,ref={0}/{1}:buildcache,mode=max', env.REGISTRY, env.IMAGE_NAME_LOWER) || '' }}
          build-args: |
            BUILD_DATE=${{ env.BUILD_DATE }}
            GIT_SHA=${{ env.GIT_SHA_FULL }}
            GIT_SHA_SHORT=${{ env.GIT_SHA }}
            GIT_REF=${{ github.ref }}
            VERSION=${{ steps.version.outputs.semver || '0.0.0-dev' }}
            SEMVER=${{ steps.version.outputs.semver || '0.0.0' }}
            BRANCH=${{ github.ref_name }}
            BUILD_TYPE=${{ env.BUILD_TYPE }}
            TARGETPLATFORM=$TARGETPLATFORM
            TARGETARCH=$TARGETARCH
          provenance: ${{ env.SHOULD_PUSH }}
          sbom: ${{ env.SHOULD_PUSH }}
          annotations: |
            org.opencontainers.image.title=ADRScan
            org.opencontainers.image.description=AI-powered Architecture Decision Record management

      # Platform-specific tags (optional, for debugging)
      - name: Generate platform-specific tags
        if: env.SHOULD_PUSH == 'true' && github.event_name == 'release'
        run: |
          # This would require separate builds per platform
          # Implement only if platform-specific debugging is needed
          echo "Platform-specific tags can be implemented if needed"

      # Enhanced container testing
      - name: Test container
        if: github.event_name == 'pull_request' || env.BUILD_TYPE == 'nightly'
        run: |
          set -e
          IMAGE="${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}:${{ env.BUILD_TYPE }}-${{ env.GIT_SHA }}"
          
          echo "Testing image: ${IMAGE}"
          
          # Basic functionality tests
          docker run --rm "$IMAGE" --version
          docker run --rm "$IMAGE" --help
          
          # Verify non-root execution
          USER_ID=$(docker run --rm "$IMAGE" id -u)
          if [[ "${USER_ID}" == "0" ]]; then
            echo "❌ Container runs as root!"
            exit 1
          fi
          echo "✅ Container runs as non-root user (UID: ${USER_ID})"
          
          # Verify build metadata
          docker inspect "$IMAGE" | jq -r '.[] | .Config.Labels' > labels.json
          
          # Check required labels
          for label in "org.opencontainers.image.version" "build.commit" "build.timestamp"; do
            if ! jq -e ".[\"${label}\"]" labels.json > /dev/null; then
              echo "❌ Missing required label: ${label}"
              exit 1
            fi
          done
          echo "✅ All required labels present"
          
          # Test actual functionality
          docker run --rm -v "${PWD}:/workspace:ro" "$IMAGE" scan /workspace || true
          
          echo "✅ Container tests passed"

      # Enhanced security scanning
      - name: Security scan with Trivy
        if: github.event_name != 'schedule'  # Skip on nightly to save time
        uses: aquasecurity/trivy-action@master
        with:
          image-ref: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}@${{ steps.build.outputs.digest }}
          format: 'sarif'
          output: 'trivy-results.sarif'
          severity: 'CRITICAL,HIGH,MEDIUM'
          exit-code: ${{ github.event_name == 'pull_request' && '1' || '0' }}
          vuln-type: 'os,library'
          scanners: 'vuln,secret,config'

      - name: Upload scan results
        if: always()
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: 'trivy-results.sarif'

      # Sign container images
      - name: Install cosign
        if: env.SHOULD_PUSH == 'true'
        uses: sigstore/cosign-installer@v3

      - name: Sign container image
        if: env.SHOULD_PUSH == 'true' && steps.build.outputs.digest != ''
        env:
          COSIGN_EXPERIMENTAL: 1
        run: |
          cosign sign --yes \
            ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}@${{ steps.build.outputs.digest }}

      # Generate attestations
      - name: Generate SLSA provenance
        if: env.SHOULD_PUSH == 'true' && steps.build.outputs.digest != ''
        uses: actions/attest-build-provenance@v1
        with:
          subject-name: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}
          subject-digest: ${{ steps.build.outputs.digest }}
          push-to-registry: true

      # Enhanced build summary
      - name: Generate build summary
        if: always()
        run: |
          cat >> $GITHUB_STEP_SUMMARY << EOF
          # 🐳 Container Build Summary
          
          ## Build Information
          - **Type**: ${{ env.BUILD_TYPE }}
          - **Platforms**: ${{ github.event_name == 'pull_request' && 'linux/amd64' || env.PLATFORMS }}
          - **Registry**: \`${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}\`
          - **Commit**: \`${{ env.GIT_SHA_FULL }}\`
          - **Branch**: \`${{ github.ref_name }}\`
          - **Timestamp**: \`${{ env.BUILD_DATE }}\`
          
          ## Version Information
          - **Latest Tag**: \`${{ steps.version.outputs.latest_tag }}\`
          - **SemVer**: \`${{ steps.version.outputs.semver || 'dev' }}\`
          - **Is Tagged**: \`${{ steps.version.outputs.is_tagged }}\`
          
          EOF
          
          if [[ "${{ env.SHOULD_PUSH }}" == "true" ]]; then
            cat >> $GITHUB_STEP_SUMMARY << EOF
          ## Published Tags
          \`\`\`
          ${{ steps.meta.outputs.tags }}
          \`\`\`
          
          ## Image Digest
          \`\`\`
          ${{ steps.build.outputs.digest }}
          \`\`\`
          
          ## Pull Commands
          \`\`\`bash
          # By tag
          docker pull ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}:${{ env.BUILD_TYPE }}
          
          # By digest (immutable)
          docker pull ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}@${{ steps.build.outputs.digest }}
          
          # Specific architecture
          docker pull --platform linux/arm64 ${{ env.REGISTRY }}/${{ env.IMAGE_NAME_LOWER }}:${{ env.BUILD_TYPE }}
          \`\`\`
          EOF
          else
            echo "## Status: Built Successfully (Not Published)" >> $GITHUB_STEP_SUMMARY
          fi

  # Tag cleanup job (runs on schedule)
  cleanup:
    name: Cleanup Old Tags
    runs-on: ubuntu-latest
    if: github.event_name == 'schedule'
    needs: build
    permissions:
      packages: write
      
    steps:
      - name: Cleanup old PR tags
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # Clean up PR tags older than 7 days
          echo "Cleaning up old PR tags..."
          
          # Get all PR tags
          gh api \
            -H "Accept: application/vnd.github+json" \
            -H "X-GitHub-Api-Version: 2022-11-28" \
            "/user/packages/container/${{ env.IMAGE_NAME_LOWER }}/versions" \
            --paginate \
            --jq '.[] | select(.metadata.container.tags[] | startswith("pr-")) | .id' | \
          while read -r version_id; do
            echo "Checking version $version_id..."
            # Delete if older than 7 days (implement date check)
          done
          
      - name: Prune SHA tags
        run: |
          # Keep only last 50 SHA-based tags
          echo "Pruning old SHA tags..."
          # Implementation depends on registry API
```

## Dockerfile Updates

Update the Dockerfile to use build arguments effectively:

```dockerfile
# Accept build arguments
ARG BUILD_DATE
ARG GIT_SHA
ARG VERSION="dev"
ARG BRANCH="unknown"

# Multi-stage build...
FROM rust:1.75-alpine AS builder

# ... existing build steps ...

# Runtime stage
FROM alpine:3.22 AS runtime

# ... existing setup ...

# Add version information
ENV ADRSCAN_VERSION=${VERSION} \
    ADRSCAN_BUILD_DATE=${BUILD_DATE} \
    ADRSCAN_COMMIT=${GIT_SHA} \
    ADRSCAN_BRANCH=${BRANCH}

# Update labels with build args
LABEL org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${GIT_SHA}" \
      org.opencontainers.image.ref.name="${BRANCH}"
```

## Testing the Implementation

### Local Testing

```bash
# Test metadata generation locally
docker buildx build \
  --build-arg VERSION=0.2.0-test \
  --build-arg GIT_SHA=$(git rev-parse HEAD) \
  --build-arg BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ) \
  --tag photondrift:test \
  .

# Inspect labels
docker inspect photondrift:test | jq '.[0].Config.Labels'
```

### CI Testing

```bash
# Trigger workflow with custom parameters
gh workflow run container-build.yml \
  -f platforms="linux/amd64,linux/arm64" \
  -f push_to_registry=false \
  -f canary=false

# Test PR build
git checkout -b test/container-versioning
git push origin test/container-versioning
# Create PR to trigger PR build
```

## Rollout Plan

1. **Week 1**: Deploy to develop branch, monitor builds
2. **Week 2**: Test with canary releases
3. **Week 3**: Deploy to main branch
4. **Week 4**: Full production rollout
5. **Week 5**: Implement tag cleanup automation

## Monitoring

Set up monitoring for:

1. Build success rates per tag type
2. Image pull statistics
3. Platform distribution (amd64 vs arm64)
4. Security scan results trending
5. Registry storage usage