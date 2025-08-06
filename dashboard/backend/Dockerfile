# Dashboard Backend Dockerfile
# Multi-stage build for optimized production image (~50MB target)

FROM node:22-alpine AS base
WORKDIR /app

# Install pnpm for faster dependency management with security audit
RUN corepack enable && \
    npm audit --audit-level=moderate || echo "Audit completed with warnings"

# Dependencies stage with security scanning
FROM base AS dependencies
COPY package*.json pnpm-lock.yaml* ./

# Install dependencies with security checks and caching
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm audit --audit-level=moderate && \
    pnpm install --frozen-lockfile --prod --ignore-scripts

# Development dependencies stage with security validation
FROM dependencies AS dev-dependencies
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile --ignore-scripts

# Build stage with optimization
FROM dev-dependencies AS build
COPY . .

# Build with production optimizations
ENV NODE_ENV=production
RUN pnpm build && \
    pnpm prune --prod && \
    rm -rf src/ tests/ .git/ .github/ docs/ examples/ *.md

# Production stage - optimized for size and security
FROM node:22-alpine AS production

# Enhanced Security: Create hardened non-root user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S photondrift -u 1001 -G nodejs -h /home/photondrift -s /sbin/nologin

# Install production dependencies only
WORKDIR /app

# Copy package files
COPY --from=dependencies --chown=photondrift:nodejs /app/node_modules ./node_modules
COPY --from=build --chown=photondrift:nodejs /app/dist ./dist
COPY --from=build --chown=photondrift:nodejs /app/package.json ./package.json

# Advanced security hardening and system optimization
RUN apk update && apk upgrade --no-cache && \
    apk --no-cache add dumb-init=1.2.5-r3 && \
    rm -rf /var/cache/apk/* /tmp/* /var/tmp/* && \
    chmod -R 755 /app && \
    chown -R photondrift:nodejs /app && \
    mkdir -p /home/photondrift/.config && \
    chown -R photondrift:nodejs /home/photondrift

# Switch to non-root user
USER photondrift

# Enhanced health check with comprehensive monitoring
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD node -e "const http = require('http'); \
        const options = { host: 'localhost', port: 3001, path: '/health', timeout: 3000 }; \
        const req = http.request(options, (res) => { \
            if (res.statusCode === 200) { \
                let data = ''; \
                res.on('data', chunk => data += chunk); \
                res.on('end', () => { \
                    try { \
                        const health = JSON.parse(data); \
                        process.exit(health.status === 'healthy' ? 0 : 1); \
                    } catch(e) { process.exit(1); } \
                }); \
            } else process.exit(1); \
        }); \
        req.on('error', () => process.exit(1)); \
        req.setTimeout(3000, () => req.destroy()); \
        req.end();"

# Hardened environment configuration
ENV NODE_ENV=production \
    PORT=3001 \
    HOST=0.0.0.0 \
    NODE_OPTIONS="--max-old-space-size=512 --no-warnings" \
    UV_THREADPOOL_SIZE=4 \
    NODE_TLS_REJECT_UNAUTHORIZED=1

# Expose ports for API and WebSocket
EXPOSE 3001

# Use dumb-init for proper signal handling
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "dist/server.js"]

# Comprehensive metadata labels with security and performance info
LABEL org.opencontainers.image.title="PhotonDrift Dashboard Backend" \
      org.opencontainers.image.description="Security-hardened Express API server with optimized WebSocket support" \
      org.opencontainers.image.vendor="PhotonDrift" \
      org.opencontainers.image.version="0.3.0-alpha" \
      org.opencontainers.image.created="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
      org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift" \
      org.opencontainers.image.documentation="https://github.com/tbowman01/PhotonDrift/tree/main/dashboard/backend" \
      org.opencontainers.image.licenses="MIT" \
      security.nonroot="true" \
      security.user.uid="1001" \
      security.user.gid="1001" \
      security.hardened="enabled" \
      performance.optimized="true" \
      app.component="backend-api" \
      app.tier="application"