# Multi-stage Docker build for ADRScan
# Security-hardened container following best practices

# Build stage - Use Alpine-based Rust image for better multi-platform support
FROM rust:1.75-alpine AS builder

# Build arguments for comprehensive versioning and metadata
ARG VERSION="unknown"
ARG BUILD_DATE="unknown"
ARG GIT_SHA="unknown"
ARG GIT_SHA_SHORT="unknown"
ARG GIT_REF="unknown"
ARG BRANCH="unknown"
ARG BUILD_TYPE="unknown"
ARG SEMVER="unknown"
ARG TARGETPLATFORM
ARG TARGETARCH

# Install build dependencies
RUN apk add --no-cache \
        musl-dev \
        pkgconfig \
        openssl-dev \
        openssl-libs-static

# Create non-root user for building
RUN addgroup -g 1001 -S builder && \
    adduser -u 1001 -S builder -G builder

# Set working directory
WORKDIR /usr/src/adrscan

# Copy dependency manifests first for better layer caching
COPY Cargo.toml ./

# Create dummy src files to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src/ src/

# Build the actual application with version metadata
RUN RUST_BACKTRACE=1 cargo build --release

# Embed version information into the binary
RUN echo "${VERSION}" > /tmp/version.txt && \
    echo "${BUILD_DATE}" > /tmp/build_date.txt && \
    echo "${GIT_SHA}" > /tmp/git_sha.txt

# Strip debug symbols for smaller binary
RUN strip target/release/adrscan

# Verify binary works
RUN ./target/release/adrscan --version

# Runtime stage - Use Alpine for minimal attack surface and musl compatibility
FROM alpine:3.22 AS runtime

# Install CA certificates and create non-root user
RUN apk add --no-cache ca-certificates && \
    addgroup -g 65532 -S nonroot && \
    adduser -u 65532 -S nonroot -G nonroot

# Copy the binary from builder stage and verify it exists
COPY --from=builder /usr/src/adrscan/target/release/adrscan /usr/local/bin/adrscan

# Create metadata directory first
RUN mkdir -p /etc/adrscan

# Copy version metadata files
COPY --from=builder /tmp/version.txt /etc/adrscan/version
COPY --from=builder /tmp/build_date.txt /etc/adrscan/build_date
COPY --from=builder /tmp/git_sha.txt /etc/adrscan/git_sha

# Set permissions after files are copied
RUN chmod 755 /etc/adrscan && \
    chmod 644 /etc/adrscan/* && \
    chmod +x /usr/local/bin/adrscan

# Create directory for ADRs with proper permissions
USER 65532:65532
WORKDIR /workspace

# Health check to ensure container is working
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/adrscan", "--version"]

# Set entrypoint and default command
ENTRYPOINT ["/usr/local/bin/adrscan"]
CMD ["--help"]

# Comprehensive metadata labels following OCI standards
ARG VERSION
ARG BUILD_DATE
ARG GIT_SHA
ARG GIT_SHA_SHORT
ARG GIT_REF
ARG BRANCH
ARG BUILD_TYPE
ARG SEMVER
ARG TARGETPLATFORM
ARG TARGETARCH

# Set environment variables for runtime access
ENV ADRSCAN_VERSION="${VERSION}" \
    ADRSCAN_BUILD_DATE="${BUILD_DATE}" \
    ADRSCAN_COMMIT="${GIT_SHA}" \
    ADRSCAN_BRANCH="${BRANCH}" \
    ADRSCAN_BUILD_TYPE="${BUILD_TYPE}" \
    ADRSCAN_PLATFORM="${TARGETPLATFORM}"

# OCI Standard Labels
LABEL org.opencontainers.image.title="ADRScan" \
      org.opencontainers.image.description="AI-powered Architecture Decision Record (ADR) management with ML-enhanced drift detection" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.vendor="PhotonDrift" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift" \
      org.opencontainers.image.documentation="https://github.com/tbowman01/PhotonDrift/blob/main/README.md" \
      org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.revision="${GIT_SHA}" \
      org.opencontainers.image.ref.name="${BRANCH}" \
      org.opencontainers.image.authors="tbowman01"

# Build Metadata Labels
LABEL build.timestamp="${BUILD_DATE}" \
      build.version="${SEMVER}" \
      build.commit="${GIT_SHA}" \
      build.commit.short="${GIT_SHA_SHORT}" \
      build.branch="${BRANCH}" \
      build.type="${BUILD_TYPE}" \
      build.ref="${GIT_REF}" \
      build.platform="${TARGETPLATFORM}" \
      build.arch="${TARGETARCH}"

# Security Labels
LABEL security.scan="enabled" \
      security.distroless="false" \
      security.nonroot="true" \
      security.readonly.rootfs="false"

# Application Labels
LABEL app.name="adrscan" \
      app.component="cli" \
      app.part-of="photondrift" \
      app.managed-by="github-actions"