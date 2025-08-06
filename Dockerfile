# ==============================================================================
# OPTIMIZED MULTI-STAGE DOCKERFILE FOR ADRScan
# Security-hardened container with performance optimizations
# ==============================================================================

# Build arguments for comprehensive versioning and metadata
ARG RUST_VERSION=1.76
ARG ALPINE_VERSION=3.22
ARG VERSION="unknown"
ARG BUILD_DATE="unknown"
ARG GIT_SHA="unknown"
ARG GIT_SHA_SHORT="unknown"
ARG GIT_REF="unknown"
ARG BRANCH="unknown"
ARG BUILD_TYPE="unknown"
ARG SEMVER="unknown"
ARG GITHUB_RUN_ID="unknown"
ARG TARGETPLATFORM
ARG TARGETARCH

# ==============================================================================
# STAGE 1: DEPENDENCY CACHE
# Optimized for maximum layer caching efficiency
# ==============================================================================
FROM rust:${RUST_VERSION}-alpine AS dependencies

# Install build dependencies with explicit versioning for reproducibility
RUN apk add --no-cache \
        musl-dev=1.2.5-r0 \
        pkgconfig=2.2.0-r0 \
        openssl-dev=3.3.2-r0 \
        openssl-libs-static=3.3.2-r0

WORKDIR /build

# Copy dependency manifests for aggressive caching
COPY Cargo.toml Cargo.lock* ./

# Pre-compile dependencies only (cached layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    cargo build --release --locked && \
    rm -rf src target/release/deps/adrscan* target/release/adrscan*

# ==============================================================================
# STAGE 2: APPLICATION BUILD
# Leverages dependency cache for fast rebuilds
# ==============================================================================
FROM dependencies AS builder

# Copy source code
COPY src/ src/

# Build with optimizations and security features
ENV CARGO_BUILD_TARGET_DIR=/build/target
RUN RUST_BACKTRACE=full \
    RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort" \
    cargo build --release --locked

# Strip debug symbols and verify binary
RUN strip target/release/adrscan && \
    ./target/release/adrscan --version

# Create version metadata files with comprehensive build info
RUN echo "${VERSION}" > /tmp/version.txt && \
    echo "${BUILD_DATE}" > /tmp/build_date.txt && \
    echo "${GIT_SHA}" > /tmp/git_sha.txt && \
    echo "${BRANCH}" > /tmp/branch.txt && \
    echo "${BUILD_TYPE}" > /tmp/build_type.txt && \
    echo "${GITHUB_RUN_ID}" > /tmp/run_id.txt && \
    echo "${TARGETPLATFORM}" > /tmp/platform.txt

# ==============================================================================
# STAGE 3: SECURITY-HARDENED RUNTIME
# Minimal attack surface with comprehensive security controls
# ==============================================================================
FROM alpine:${ALPINE_VERSION} AS runtime

# Re-declare build args for runtime stage
ARG VERSION
ARG BUILD_DATE  
ARG GIT_SHA
ARG GIT_SHA_SHORT
ARG GIT_REF
ARG BRANCH
ARG BUILD_TYPE
ARG SEMVER
ARG GITHUB_RUN_ID
ARG TARGETPLATFORM
ARG TARGETARCH

# Install minimal runtime dependencies and security updates
RUN apk update && \
    apk add --no-cache \
        ca-certificates=20240705-r0 \
        tzdata=2024b-r0 && \
    apk upgrade --no-cache && \
    rm -rf /var/cache/apk/*

# Create secure non-root user with minimal privileges
RUN addgroup -g 65532 -S nonroot && \
    adduser -u 65532 -S nonroot -G nonroot -h /home/nonroot -s /sbin/nologin

# Copy zscaler certificate for corporate environments
COPY assets/zscaler.crt /usr/local/share/ca-certificates/zscaler.crt
RUN update-ca-certificates

# Copy optimized binary from builder
COPY --from=builder --chown=nonroot:nonroot /build/target/release/adrscan /usr/local/bin/adrscan

# Create secure metadata directory structure
RUN mkdir -p /etc/adrscan /home/nonroot/.config/adrscan && \
    chown -R nonroot:nonroot /etc/adrscan /home/nonroot

# Copy version metadata with secure permissions
COPY --from=builder --chown=nonroot:nonroot /tmp/version.txt /etc/adrscan/version
COPY --from=builder --chown=nonroot:nonroot /tmp/build_date.txt /etc/adrscan/build_date  
COPY --from=builder --chown=nonroot:nonroot /tmp/git_sha.txt /etc/adrscan/git_sha
COPY --from=builder --chown=nonroot:nonroot /tmp/branch.txt /etc/adrscan/branch
COPY --from=builder --chown=nonroot:nonroot /tmp/build_type.txt /etc/adrscan/build_type
COPY --from=builder --chown=nonroot:nonroot /tmp/run_id.txt /etc/adrscan/run_id
COPY --from=builder --chown=nonroot:nonroot /tmp/platform.txt /etc/adrscan/platform

# Set secure permissions
RUN chmod 755 /etc/adrscan && \
    chmod 644 /etc/adrscan/* && \
    chmod +x /usr/local/bin/adrscan && \
    chmod 755 /home/nonroot/.config/adrscan

# Create secure workspace with proper permissions
USER nonroot:nonroot
WORKDIR /workspace

# Set comprehensive security environment variables
ENV ADRSCAN_VERSION="${VERSION}" \
    ADRSCAN_BUILD_DATE="${BUILD_DATE}" \
    ADRSCAN_COMMIT="${GIT_SHA}" \
    ADRSCAN_BRANCH="${BRANCH}" \
    ADRSCAN_BUILD_TYPE="${BUILD_TYPE}" \
    ADRSCAN_GITHUB_RUN_ID="${GITHUB_RUN_ID}" \
    ADRSCAN_PLATFORM="${TARGETPLATFORM}" \
    RUST_LOG=info \
    RUST_BACKTRACE=0 \
    HOME=/home/nonroot \
    USER=nonroot

# Advanced health check with timeout and retry logic
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/adrscan", "--version"]

# Set secure entrypoint and default command
ENTRYPOINT ["/usr/local/bin/adrscan"]
CMD ["--help"]

# ==============================================================================
# COMPREHENSIVE METADATA LABELS (OCI Compliant)
# ==============================================================================

# Core OCI Image Specification Labels
LABEL org.opencontainers.image.title="ADRScan" \
      org.opencontainers.image.description="AI-powered Architecture Decision Record (ADR) management with ML-enhanced drift detection and automated analysis" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.vendor="PhotonDrift" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift" \
      org.opencontainers.image.documentation="https://github.com/tbowman01/PhotonDrift/blob/main/README.md" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${GIT_SHA}" \
      org.opencontainers.image.ref.name="${BRANCH}" \
      org.opencontainers.image.authors="tbowman01" \
      org.opencontainers.image.url="https://github.com/tbowman01/PhotonDrift"

# Build and CI/CD Metadata Labels  
LABEL build.timestamp="${BUILD_DATE}" \
      build.version="${SEMVER}" \
      build.commit="${GIT_SHA}" \
      build.commit.short="${GIT_SHA_SHORT}" \
      build.branch="${BRANCH}" \
      build.type="${BUILD_TYPE}" \
      build.ref="${GIT_REF}" \
      build.github_run_id="${GITHUB_RUN_ID}" \
      build.platform="${TARGETPLATFORM}" \
      build.arch="${TARGETARCH}" \
      build.rust_version="${RUST_VERSION}" \
      build.alpine_version="${ALPINE_VERSION}"

# Security and Compliance Labels
LABEL security.scan="enabled" \
      security.distroless="false" \
      security.nonroot="true" \
      security.readonly.rootfs="recommended" \
      security.user.uid="65532" \
      security.user.gid="65532" \
      security.capabilities.drop="ALL" \
      security.selinux="compatible" \
      security.apparmor="compatible"

# Application and Service Labels
LABEL app.name="adrscan" \
      app.component="cli" \
      app.part-of="photondrift" \
      app.managed-by="github-actions" \
      app.version="${VERSION}" \
      app.tier="application" \
      app.environment="${BUILD_TYPE}"

# Operational and Monitoring Labels
LABEL monitoring.health-check="enabled" \
      monitoring.metrics="available" \
      monitoring.logs="structured" \
      monitoring.tracing="enabled" \
      performance.optimized="true" \
      performance.multi-stage="true" \
      performance.binary-stripped="true"