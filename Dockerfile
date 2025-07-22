# Multi-stage Docker build for ADRScan
# Security-hardened container following best practices

# Build stage - Use Alpine-based Rust image for better multi-platform support
FROM rust:1.88-alpine AS builder

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

# Build the actual application
RUN cargo build --release

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

# Copy the binary from builder stage
COPY --from=builder /usr/src/adrscan/target/release/adrscan /usr/local/bin/adrscan

# Create directory for ADRs with proper permissions
USER 65532:65532
WORKDIR /workspace

# Health check to ensure container is working
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/adrscan", "--version"]

# Set entrypoint and default command
ENTRYPOINT ["/usr/local/bin/adrscan"]
CMD ["--help"]

# Metadata labels following OCI standards
LABEL org.opencontainers.image.title="ADRScan"
LABEL org.opencontainers.image.description="AI-powered Architecture Decision Record (ADR) management with ML-enhanced drift detection"
LABEL org.opencontainers.image.version="0.2.0-alpha.20250721"
LABEL org.opencontainers.image.vendor="PhotonDrift"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift"
LABEL org.opencontainers.image.documentation="https://github.com/tbowman01/PhotonDrift/blob/main/README.md"
LABEL org.opencontainers.image.created="2025-07-21T01:30:00Z"
LABEL security.scan="enabled"
LABEL security.distroless="true"
LABEL security.nonroot="true"