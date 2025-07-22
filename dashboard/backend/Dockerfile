# Dashboard Backend Dockerfile
# Multi-stage build for optimized production image (~50MB target)

FROM node:22-alpine AS base
WORKDIR /app

# Install pnpm for faster dependency management
RUN corepack enable

# Dependencies stage
FROM base AS dependencies
COPY package*.json pnpm-lock.yaml* ./
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile --prod

# Development dependencies stage
FROM dependencies AS dev-dependencies
RUN --mount=type=cache,target=/root/.local/share/pnpm/store \
    pnpm install --frozen-lockfile

# Build stage
FROM dev-dependencies AS build
COPY . .
RUN pnpm build

# Production stage - optimized for size and security
FROM node:22-alpine AS production

# Security: Create non-root user
RUN addgroup -g 1001 -S nodejs && \
    adduser -S photondrift -u 1001 -G nodejs

# Install production dependencies only
WORKDIR /app

# Copy package files
COPY --from=dependencies --chown=photondrift:nodejs /app/node_modules ./node_modules
COPY --from=build --chown=photondrift:nodejs /app/dist ./dist
COPY --from=build --chown=photondrift:nodejs /app/package.json ./package.json

# Security hardening
RUN apk --no-cache add dumb-init && \
    rm -rf /var/cache/apk/* && \
    chmod -R 755 /app && \
    chown -R photondrift:nodejs /app

# Switch to non-root user
USER photondrift

# Health check for container orchestration
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node -e "const http = require('http'); \
        const options = { host: 'localhost', port: 3001, path: '/health', timeout: 2000 }; \
        const req = http.request(options, (res) => { \
            if (res.statusCode === 200) process.exit(0); else process.exit(1); \
        }); \
        req.on('error', () => process.exit(1)); \
        req.end();"

# Environment configuration
ENV NODE_ENV=production \
    PORT=3001 \
    HOST=0.0.0.0

# Expose ports for API and WebSocket
EXPOSE 3001

# Use dumb-init for proper signal handling
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "dist/server.js"]

# Metadata labels
LABEL org.opencontainers.image.title="PhotonDrift Dashboard Backend" \
      org.opencontainers.image.description="Express API server with WebSocket support for PhotonDrift Visual Analytics Dashboard" \
      org.opencontainers.image.vendor="PhotonDrift" \
      org.opencontainers.image.version="0.2.0-alpha" \
      org.opencontainers.image.created="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
      org.opencontainers.image.source="https://github.com/tbowman01/PhotonDrift" \
      org.opencontainers.image.documentation="https://github.com/tbowman01/PhotonDrift/tree/main/dashboard/backend"