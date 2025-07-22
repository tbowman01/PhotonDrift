# PhotonDrift Dashboard Backend API

Express.js backend API server for the PhotonDrift Visual Analytics Dashboard.

## Features

- **REST API Endpoints**: Complete API for drift detection and repository management
- **WebSocket Support**: Real-time updates for drift events and scan progress
- **PhotonDrift CLI Integration**: Direct integration with `adrscan` command
- **SQLite Database**: Persistent storage for drift events, scan results, and metadata
- **Scheduled Scanning**: Automated periodic repository scanning
- **Error Handling**: Comprehensive error handling and logging

## API Endpoints

### Core Endpoints

- `POST /api/scan` - Trigger a PhotonDrift CLI scan
- `GET /api/drift/events` - Get drift events with filtering
- `GET /api/health/:repo` - Get repository health metrics
- `GET /api/metrics/trends` - Get trend data for analytics
- `GET /api/repositories` - List available repositories
- `POST /api/repositories/:repo/init` - Initialize ADR structure

### Management Endpoints

- `GET /api/scans/active` - Get active scan status
- `DELETE /api/scans/:scanId` - Cancel an active scan
- `GET /api/health` - API health check

## WebSocket Events

### Client Events
- `subscribe:repository` - Subscribe to repository updates
- `unsubscribe:repository` - Unsubscribe from repository
- `request:scan` - Request a scan via WebSocket
- `request:health` - Request health metrics
- `acknowledge:alert` - Acknowledge an alert

### Server Events
- `drift:detected` - New drift event detected
- `drift:resolved` - Drift event resolved
- `health:updated` - Repository health updated
- `scan:progress` - Scan progress update
- `scan:completed` - Scan completed
- `scan:failed` - Scan failed
- `alert:new` - New alert generated
- `system:stats` - System statistics

## Quick Start

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Copy environment file:**
   ```bash
   cp .env.example .env
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

4. **Build for production:**
   ```bash
   npm run build
   npm start
   ```

## Environment Variables

See `.env.example` for all available configuration options.

## Database Schema

The SQLite database includes tables for:
- `repositories` - Repository metadata and configuration
- `drift_events` - Individual drift detection events
- `scan_results` - Scan execution results and summaries
- `users` - User accounts (for future auth implementation)
- `alerts` - System alerts and notifications

## Development

- **TypeScript**: Full TypeScript support with strict mode
- **ESLint + Prettier**: Code formatting and linting
- **Vitest**: Unit testing framework
- **Winston**: Structured logging

## Architecture

- **Service Layer**: PhotonDriftService for CLI integration
- **Data Layer**: Repository pattern with typed database access
- **WebSocket Layer**: Real-time communication with frontend
- **Scheduler**: Automated scanning with configurable intervals

## Dependencies

- Express.js - Web framework
- Socket.io - WebSocket implementation
- SQLite3 - Database
- Winston - Logging
- Node-cron - Scheduled tasks
- Zod - Runtime type validation