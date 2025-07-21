# Multi-stage build for PhotonDrift ADRScan
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy Cargo files for dependency caching
COPY Cargo.toml ./

# Create a dummy main to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/adrscan*

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m adrscan

# Copy binary from builder stage
COPY --from=builder /usr/src/app/target/release/adrscan /usr/local/bin/adrscan

# Set ownership and permissions
RUN chown adrscan:adrscan /usr/local/bin/adrscan
RUN chmod +x /usr/local/bin/adrscan

# Switch to non-root user
USER adrscan

# Set working directory
WORKDIR /app

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD adrscan --version || exit 1

# Default command
ENTRYPOINT ["adrscan"]
CMD ["--help"]

# Metadata labels
ARG BUILDTIME
ARG VERSION
ARG REVISION

LABEL org.opencontainers.image.created="${BUILDTIME}"
LABEL org.opencontainers.image.description="AI-powered Architecture Decision Record (ADR) management with ML-enhanced drift detection"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.revision="${REVISION}"
LABEL org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift"
LABEL org.opencontainers.image.title="ADRScan"
LABEL org.opencontainers.image.url="https://github.com/tbowman01/PhotonDrift"
LABEL org.opencontainers.image.vendor="PhotonDrift"
LABEL org.opencontainers.image.version="${VERSION}"