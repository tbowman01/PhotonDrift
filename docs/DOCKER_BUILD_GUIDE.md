# Docker Build Guide

This guide provides comprehensive instructions for manually building PhotonDrift Docker images.

## Prerequisites

- Docker 20.10+ installed
- Docker Buildx for multi-platform builds (optional)
- Git for cloning the repository
- At least 2GB free disk space

## Quick Build

### Basic Build (AMD64 only)

```bash
# Clone the repository
git clone https://github.com/tbowman01/PhotonDrift.git
cd PhotonDrift

# Build the Docker image
docker build -t photondrift:local .

# Verify the build
docker run --rm photondrift:local --version
```

## Advanced Build Options

### Multi-stage Build Details

The Dockerfile uses a multi-stage build for security and optimization:

1. **Builder Stage** - Compiles Rust binary
2. **Runtime Stage** - Minimal distroless image

### Build Arguments

```bash
# Build with custom Rust version
docker build --build-arg RUST_VERSION=1.75 -t photondrift:custom .

# Build with specific target
docker build --target builder -t photondrift:builder .
```

### Platform-Specific Builds

```bash
# Build for AMD64 (default)
docker build --platform linux/amd64 -t photondrift:amd64 .

# Build for ARM64 (if supported)
docker build --platform linux/arm64 -t photondrift:arm64 .
```

### Build with Cache

```bash
# Use BuildKit for better caching
DOCKER_BUILDKIT=1 docker build -t photondrift:cached .

# Build with external cache
docker build --cache-from ghcr.io/tbowman01/photondrift:latest -t photondrift:local .
```

## Development Builds

### Debug Build

```bash
# Build with debug symbols (larger image)
docker build --build-arg CARGO_BUILD_FLAGS="" -f Dockerfile.dev -t photondrift:debug .
```

### Local Development

```bash
# Mount source code for live development
docker run -it --rm \
  -v "$(pwd)":/usr/src/adrscan \
  -w /usr/src/adrscan \
  rust:1.75-slim-bullseye \
  cargo build --release
```

## Build Verification

### Basic Tests

```bash
# Test the binary works
docker run --rm photondrift:local --help

# Test with sample ADR directory
docker run --rm -v "$(pwd)/docs/adr":/workspace/adr photondrift:local inventory --adr-dir /workspace/adr
```

### Security Verification

```bash
# Verify non-root user
docker run --rm photondrift:local whoami
# Expected output: nonroot

# Check for vulnerabilities
docker scan photondrift:local

# Inspect image layers
docker history photondrift:local
```

## Optimization Tips

### Reduce Image Size

```bash
# Check image size
docker images photondrift:local

# Remove build cache
docker builder prune

# Use slim base images
docker build --build-arg BASE_IMAGE=gcr.io/distroless/cc-debian11:nonroot -t photondrift:slim .
```

### Build Performance

```bash
# Parallel builds
docker build --jobs 4 -t photondrift:fast .

# Use build cache mount
docker build \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/src/adrscan/target \
  -t photondrift:cached .
```

## Tagging and Registry

### Local Tagging

```bash
# Tag with version
docker tag photondrift:local photondrift:v0.2.0-alpha

# Tag for registry
docker tag photondrift:local ghcr.io/yourusername/photondrift:latest
```

### Push to Registry

```bash
# Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Push image
docker push ghcr.io/yourusername/photondrift:latest
```

## Troubleshooting

### Common Issues

#### Build Fails with "Cargo.lock not found"
The Dockerfile doesn't require Cargo.lock. This error indicates an outdated Dockerfile.

#### Out of Memory
```bash
# Increase Docker memory limit
docker build --memory 4g -t photondrift:local .
```

#### Network Issues
```bash
# Build with proxy
docker build \
  --build-arg HTTP_PROXY=http://proxy:8080 \
  --build-arg HTTPS_PROXY=http://proxy:8080 \
  -t photondrift:proxy .
```

### Build Logs

```bash
# Verbose build output
docker build --progress=plain -t photondrift:debug .

# Save build log
docker build -t photondrift:local . 2>&1 | tee build.log
```

## CI/CD Integration

### GitHub Actions

The repository includes automated Docker builds. See `.github/workflows/container-build.yml` for the CI configuration.

### Manual CI Build

```bash
# Simulate CI build
docker build \
  --label "org.opencontainers.image.source=https://github.com/tbowman01/PhotonDrift" \
  --label "org.opencontainers.image.revision=$(git rev-parse HEAD)" \
  --label "org.opencontainers.image.created=$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  -t photondrift:ci .
```

## Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  photondrift:
    build:
      context: .
      dockerfile: Dockerfile
      cache_from:
        - ghcr.io/tbowman01/photondrift:latest
    image: photondrift:local
    volumes:
      - ./docs/adr:/workspace/adr
      - ./src:/workspace/src
    command: diff --adr-dir /workspace/adr --directory /workspace/src
```

Run with:
```bash
docker-compose build
docker-compose run --rm photondrift
```

## Best Practices

1. **Always use specific tags** in production, not `latest`
2. **Scan images** for vulnerabilities before deployment
3. **Use multi-stage builds** to reduce image size
4. **Pin base image versions** for reproducibility
5. **Label images** with metadata for traceability
6. **Test images** before pushing to registry
7. **Use BuildKit** for improved performance
8. **Clean up** unused images regularly

## Additional Resources

- [Dockerfile Reference](https://docs.docker.com/engine/reference/builder/)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
- [BuildKit Documentation](https://docs.docker.com/develop/develop-images/build_enhancements/)
- [Container Security](https://docs.docker.com/engine/security/)

---

*Last updated: 2025-07-21*