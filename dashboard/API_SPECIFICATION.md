# PhotonDrift Dashboard API Specification

## Overview

This document provides detailed specifications for the PhotonDrift Visual Analytics Dashboard API. The API follows RESTful principles with GraphQL support for complex queries and WebSocket connections for real-time updates.

## Base Configuration

- **Base URL**: `http://localhost:3001/api` (development) | `https://api.photondrift.com/api` (production)
- **Authentication**: JWT Bearer tokens
- **Content Type**: `application/json`
- **API Version**: `v1`

## Authentication

### JWT Token Structure
```typescript
interface JWTPayload {
  userId: string;
  email: string;
  role: 'admin' | 'user' | 'viewer';
  permissions: string[];
  iat: number;
  exp: number;
}
```

### Authentication Endpoints

#### POST /api/auth/login
**Description**: Authenticate user and return JWT token

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refreshToken": "refresh_token_here",
    "user": {
      "id": "user_123",
      "email": "user@example.com",
      "name": "John Doe",
      "role": "user"
    },
    "expiresIn": 86400
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

#### POST /api/auth/refresh
**Description**: Refresh expired JWT token

**Request Body**:
```json
{
  "refreshToken": "refresh_token_here"
}
```

**Response**: Same as login response with new tokens

## Repository Management

### GET /api/repositories
**Description**: List all repositories with pagination and filtering

**Query Parameters**:
- `page` (number, default: 1): Page number
- `limit` (number, default: 20, max: 100): Items per page
- `search` (string): Search repositories by name
- `status` (string): Filter by active/inactive
- `sortBy` (string): Sort field (name, lastScanned, health)
- `order` (string): Sort order (asc, desc)

**Response**:
```json
{
  "success": true,
  "data": [
    {
      "id": "repo_123",
      "name": "my-project",
      "path": "/path/to/repository",
      "description": "Main application repository",
      "lastScanned": "2025-01-21T17:30:00.000Z",
      "isActive": true,
      "health": {
        "overallScore": 85,
        "driftCount": 3,
        "lastUpdated": "2025-01-21T17:30:00.000Z"
      },
      "configuration": {
        "adrDirectory": "docs/adr",
        "enableAutoScan": true,
        "scanInterval": 60
      },
      "createdAt": "2025-01-15T10:00:00.000Z",
      "updatedAt": "2025-01-21T17:30:00.000Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 45,
    "totalPages": 3
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### POST /api/repositories
**Description**: Add new repository to monitoring

**Request Body**:
```json
{
  "name": "new-project",
  "path": "/path/to/new/repository",
  "description": "New project repository",
  "configuration": {
    "adrDirectory": "docs/decisions",
    "enableAutoScan": true,
    "scanInterval": 30,
    "mlThreshold": 0.7,
    "excludePatterns": ["node_modules", "*.log"],
    "includePatterns": ["src/**", "docs/**"],
    "notifications": {
      "enabled": true,
      "emailAlerts": true,
      "criticalThreshold": 90,
      "highThreshold": 70
    }
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "repo_124",
    "name": "new-project",
    "message": "Repository added successfully"
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### GET /api/repositories/{id}
**Description**: Get detailed repository information

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "repo_123",
    "name": "my-project",
    "path": "/path/to/repository",
    "description": "Main application repository",
    "lastScanned": "2025-01-21T17:30:00.000Z",
    "isActive": true,
    "statistics": {
      "totalScans": 150,
      "totalDriftEvents": 45,
      "resolvedDriftEvents": 42,
      "avgScanDuration": 25.5,
      "lastScanDuration": 28.2
    },
    "health": {
      "overallScore": 85,
      "metrics": {
        "driftCount": 3,
        "coverage": 92,
        "compliance": 88,
        "maintainability": 85,
        "technicalDebt": 15
      },
      "trends": {
        "direction": "improving",
        "velocity": 2.3
      },
      "lastUpdated": "2025-01-21T17:30:00.000Z"
    },
    "configuration": {
      "adrDirectory": "docs/adr",
      "enableAutoScan": true,
      "scanInterval": 60,
      "mlThreshold": 0.8,
      "excludePatterns": ["node_modules", "dist", "*.log"],
      "includePatterns": ["src/**", "docs/**", "tests/**"],
      "notifications": {
        "enabled": true,
        "emailAlerts": true,
        "slackWebhook": "https://hooks.slack.com/...",
        "criticalThreshold": 90,
        "highThreshold": 70
      }
    },
    "recentActivity": [
      {
        "timestamp": "2025-01-21T17:30:00.000Z",
        "type": "scan_completed",
        "message": "Scan completed with 2 new drift events"
      }
    ]
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### POST /api/repositories/{id}/scan
**Description**: Trigger manual drift detection scan

**Request Body**:
```json
{
  "options": {
    "enableML": true,
    "confidenceThreshold": 0.7,
    "includeFiles": ["src/**/*.js", "docs/**/*.md"],
    "excludeFiles": ["*.test.js"],
    "verbose": true
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "scanId": "scan_1737739200_abc123",
    "status": "started",
    "estimatedDuration": 30,
    "message": "Scan initiated successfully"
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

## Drift Event Management

### GET /api/drift-events
**Description**: List drift events with advanced filtering

**Query Parameters**:
- `page` (number): Page number
- `limit` (number): Items per page
- `repository` (string): Filter by repository ID
- `severity` (string): Filter by severity (low, medium, high, critical)
- `category` (string): Filter by drift category
- `resolved` (boolean): Filter by resolution status
- `assignee` (string): Filter by assigned user
- `dateFrom` (string): Start date filter (ISO 8601)
- `dateTo` (string): End date filter (ISO 8601)
- `tags` (string): Comma-separated tags
- `sortBy` (string): Sort field
- `order` (string): Sort order

**Response**:
```json
{
  "success": true,
  "data": [
    {
      "id": "drift_123",
      "timestamp": "2025-01-21T17:15:00.000Z",
      "severity": "high",
      "category": "architecture_violation",
      "title": "Unauthorized database access pattern detected",
      "description": "Direct database access found in presentation layer components, violating architecture principles",
      "location": {
        "file": "src/components/UserProfile.tsx",
        "line": 45,
        "column": 12
      },
      "repository": {
        "id": "repo_123",
        "name": "my-project"
      },
      "mlScore": 0.89,
      "confidence": 0.92,
      "resolved": false,
      "assignee": {
        "id": "user_456",
        "name": "Jane Smith",
        "email": "jane.smith@company.com"
      },
      "tags": ["architecture", "database", "violation"],
      "suggestion": "Move database operations to a service layer component",
      "resolution": null,
      "createdAt": "2025-01-21T17:15:00.000Z",
      "updatedAt": "2025-01-21T17:15:00.000Z"
    }
  ],
  "aggregations": {
    "totalEvents": 156,
    "criticalEvents": 5,
    "highEvents": 23,
    "mediumEvents": 89,
    "lowEvents": 39,
    "resolvedEvents": 134,
    "avgResolutionTime": 3.5
  },
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 156,
    "totalPages": 8
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### POST /api/drift-events/{id}/resolve
**Description**: Mark drift event as resolved

**Request Body**:
```json
{
  "resolution": {
    "type": "fixed",
    "description": "Moved database operations to UserService",
    "commitHash": "abc123def456",
    "resolvedBy": "user_456",
    "verifiedBy": "user_789"
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "drift_123",
    "resolved": true,
    "resolvedAt": "2025-01-21T18:00:00.000Z",
    "resolution": {
      "type": "fixed",
      "description": "Moved database operations to UserService",
      "commitHash": "abc123def456",
      "resolvedBy": "user_456",
      "verifiedBy": "user_789"
    }
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

## Analytics & Metrics

### GET /api/analytics/overview
**Description**: Get dashboard overview metrics

**Query Parameters**:
- `period` (string): Time period (24h, 7d, 30d, 90d)
- `repositories` (string): Comma-separated repository IDs

**Response**:
```json
{
  "success": true,
  "data": {
    "summary": {
      "totalRepositories": 12,
      "activeRepositories": 10,
      "totalDriftEvents": 245,
      "criticalEvents": 8,
      "resolvedEvents": 198,
      "avgHealthScore": 82.5,
      "trendDirection": "improving"
    },
    "metrics": {
      "driftDetectionRate": {
        "value": 15.2,
        "unit": "events/week",
        "change": -8.5,
        "trend": "decreasing"
      },
      "resolutionTime": {
        "average": 2.8,
        "median": 1.5,
        "unit": "days",
        "change": -12.3,
        "trend": "improving"
      },
      "healthScore": {
        "current": 82.5,
        "previous": 79.8,
        "change": 2.7,
        "trend": "improving"
      },
      "complianceRate": {
        "current": 94.2,
        "target": 95.0,
        "change": 1.8,
        "trend": "improving"
      }
    },
    "topRepositories": [
      {
        "id": "repo_123",
        "name": "my-project",
        "healthScore": 95,
        "driftCount": 2,
        "trend": "stable"
      }
    ],
    "recentActivity": [
      {
        "timestamp": "2025-01-21T17:45:00.000Z",
        "type": "drift_resolved",
        "repository": "my-project",
        "description": "Security vulnerability resolved in authentication module"
      }
    ]
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### GET /api/analytics/trends
**Description**: Get historical trend data for charting

**Query Parameters**:
- `metric` (string): Metric type (drift_count, health_score, resolution_time)
- `period` (string): Time period
- `granularity` (string): Data granularity (hour, day, week, month)
- `repositories` (string): Repository filter

**Response**:
```json
{
  "success": true,
  "data": {
    "metric": "drift_count",
    "period": "30d",
    "granularity": "day",
    "dataPoints": [
      {
        "timestamp": "2025-01-01T00:00:00.000Z",
        "value": 23,
        "metadata": {
          "critical": 1,
          "high": 5,
          "medium": 12,
          "low": 5
        }
      }
    ],
    "aggregations": {
      "total": 456,
      "average": 15.2,
      "peak": 28,
      "lowest": 8
    },
    "forecasting": {
      "nextWeekPrediction": 12,
      "confidence": 0.85,
      "trend": "decreasing"
    }
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### GET /api/analytics/teams
**Description**: Get team productivity and collaboration metrics

**Response**:
```json
{
  "success": true,
  "data": [
    {
      "team": "Backend Engineering",
      "period": {
        "start": "2025-01-01T00:00:00.000Z",
        "end": "2025-01-21T23:59:59.000Z"
      },
      "metrics": {
        "adrsCreated": 12,
        "driftResolved": 34,
        "reviewTime": 4.2,
        "collaborationScore": 87,
        "decisionVelocity": 2.3
      },
      "members": [
        {
          "id": "user_123",
          "name": "John Doe",
          "email": "john.doe@company.com",
          "role": "Senior Engineer",
          "contributions": {
            "adrsAuthored": 5,
            "driftResolved": 12,
            "reviewsCompleted": 8,
            "discussionParticipation": 92
          }
        }
      ],
      "trends": {
        "productivityChange": 15.2,
        "qualityImprovement": 8.7,
        "collaborationTrend": "increasing"
      }
    }
  ],
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

## System Management

### GET /api/system/status
**Description**: Get system health and operational status

**Response**:
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "uptime": 345600,
    "stats": {
      "totalRepositories": 45,
      "activeScans": 2,
      "totalDriftEvents": 1234,
      "resolvedDriftEvents": 1156,
      "systemHealth": 94,
      "lastUpdate": "2025-01-21T18:00:00.000Z"
    },
    "performance": {
      "memoryUsage": {
        "used": 256,
        "total": 512,
        "percentage": 50
      },
      "cpuUsage": {
        "current": 25.5,
        "average": 18.2
      },
      "diskUsage": {
        "used": 15.6,
        "total": 100,
        "percentage": 15.6
      }
    },
    "services": {
      "database": {
        "status": "connected",
        "responseTime": 12,
        "connections": 8
      },
      "websocket": {
        "status": "active",
        "connections": 24,
        "messageRate": 45.2
      },
      "cli": {
        "status": "available",
        "version": "2.1.0",
        "activeProcesses": 2
      }
    }
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### GET /api/system/scans/active
**Description**: Get status of currently active scans

**Response**:
```json
{
  "success": true,
  "data": [
    {
      "scanId": "scan_1737739200_abc123",
      "repository": {
        "id": "repo_123",
        "name": "my-project"
      },
      "status": "running",
      "progress": 65,
      "stage": "analyzing_files",
      "startedAt": "2025-01-21T17:55:00.000Z",
      "estimatedCompletion": "2025-01-21T18:02:00.000Z",
      "processId": 12345,
      "options": {
        "enableML": true,
        "confidenceThreshold": 0.7
      }
    }
  ],
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

## WebSocket API

### Connection
```javascript
// Client connection
const socket = io('ws://localhost:3001', {
  auth: {
    token: 'jwt_token_here'
  },
  transports: ['websocket']
});
```

### Client Events

#### subscribe:repository
Subscribe to real-time updates for a specific repository
```javascript
socket.emit('subscribe:repository', { repo: 'repo_123' });
```

#### request:scan
Request a new drift detection scan
```javascript
socket.emit('request:scan', {
  repo: 'repo_123',
  options: {
    enableML: true,
    confidenceThreshold: 0.8
  }
});
```

### Server Events

#### drift:detected
Real-time drift event notification
```javascript
socket.on('drift:detected', (event) => {
  // Handle new drift event
  console.log('New drift detected:', event);
});
```

#### scan:progress
Real-time scan progress updates
```javascript
socket.on('scan:progress', (data) => {
  // Update scan progress UI
  console.log(`Scan ${data.repo}: ${data.progress}%`);
});
```

#### health:updated
Repository health score updates
```javascript
socket.on('health:updated', (health) => {
  // Update health dashboard
  console.log('Health updated:', health);
});
```

## Error Handling

### Standard Error Response
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid repository path provided",
    "details": {
      "field": "path",
      "reason": "Path does not exist or is not accessible"
    }
  },
  "timestamp": "2025-01-21T18:00:00.000Z"
}
```

### Common Error Codes
- `AUTHENTICATION_FAILED`: Invalid or expired token
- `AUTHORIZATION_DENIED`: Insufficient permissions
- `VALIDATION_ERROR`: Request validation failed
- `RESOURCE_NOT_FOUND`: Requested resource doesn't exist
- `RATE_LIMIT_EXCEEDED`: Too many requests
- `CLI_EXECUTION_FAILED`: PhotonDrift CLI error
- `DATABASE_ERROR`: Database operation failed
- `INTERNAL_ERROR`: Unexpected server error

### HTTP Status Codes
- `200`: Success
- `201`: Created
- `400`: Bad Request
- `401`: Unauthorized
- `403`: Forbidden
- `404`: Not Found
- `409`: Conflict
- `422`: Validation Error
- `429`: Rate Limited
- `500`: Internal Server Error
- `503`: Service Unavailable

## Rate Limiting

### Default Limits
- **Authentication**: 5 requests/minute per IP
- **API Endpoints**: 100 requests/minute per user
- **WebSocket Messages**: 50 messages/minute per connection
- **Scan Requests**: 10 scans/hour per repository

### Rate Limit Headers
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1737739260
X-RateLimit-RetryAfter: 60
```

## API Versioning

### Version Header
```
Accept: application/vnd.photondrift.v1+json
```

### Backward Compatibility
- V1 endpoints will remain supported for 12 months after V2 release
- Deprecation notices provided 6 months before removal
- Migration guides provided for major version changes

---

**Version**: 1.0.0  
**Last Updated**: 2025-01-21  
**Status**: Implementation Ready