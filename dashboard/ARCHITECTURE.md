# PhotonDrift Visual Analytics Dashboard - Integration Architecture

## Executive Summary

This document outlines the comprehensive integration architecture for the PhotonDrift Visual Analytics Dashboard, designed to provide real-time insights into architectural drift patterns, team collaboration metrics, and system health indicators. The architecture emphasizes scalability, real-time capabilities, and seamless integration with the PhotonDrift CLI.

## Architecture Overview

### System Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend API   │    │ PhotonDrift CLI │
│   React + TS    │◄──►│  Node.js + TS   │◄──►│   Rust Binary   │
│                 │    │                 │    │                 │
│ • React 18      │    │ • Express       │    │ • adrscan       │
│ • TypeScript    │    │ • Socket.IO     │    │ • drift detect. │
│ • Material-UI   │    │ • SQLite/PG     │    │ • ML analysis   │
│ • D3.js/Charts  │    │ • JWT Auth      │    │ • ADR parsing   │
│ • WebSocket     │    │ • REST + GQL    │    │ • Health checks │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
                    ┌─────────────────┐
                    │   WebSocket     │
                    │   Real-time     │
                    │   Updates       │
                    └─────────────────┘
```

## Backend API Architecture

### Core Service Layer

#### PhotonDriftService (Enhanced)
- **CLI Integration**: Spawn and manage PhotonDrift CLI processes
- **Process Management**: Track active scans with proper cleanup
- **Result Parsing**: Handle JSON and text output from CLI
- **Event Emission**: Real-time notifications via EventEmitter
- **Health Monitoring**: Repository health score calculations
- **Scan Scheduling**: Automated periodic scans

#### Database Service
```typescript
interface DatabaseService {
  // Repository Management
  repositories: RepositoryRepository;
  driftEvents: DriftEventRepository;
  scanResults: ScanResultRepository;
  
  // User & Team Management
  users: UserRepository;
  teams: TeamRepository;
  
  // Analytics
  metrics: MetricsRepository;
  alerts: AlertRepository;
}
```

#### WebSocket Service
```typescript
interface WebSocketService {
  // Connection Management
  connections: Map<string, SocketConnection>;
  subscriptions: Map<string, Set<string>>; // repo -> socketIds
  
  // Event Broadcasting
  broadcastDriftEvent(event: DriftEvent): void;
  broadcastHealthUpdate(health: ArchitectureHealth): void;
  broadcastScanProgress(repo: string, progress: number): void;
}
```

### API Endpoints Design

#### Repository Management
```
GET    /api/repositories              - List all repositories
POST   /api/repositories              - Add new repository
GET    /api/repositories/{id}         - Get repository details
PUT    /api/repositories/{id}         - Update repository config
DELETE /api/repositories/{id}         - Remove repository
POST   /api/repositories/{id}/scan    - Trigger manual scan
GET    /api/repositories/{id}/health  - Get health metrics
```

#### Drift Events
```
GET    /api/drift-events                    - List drift events (paginated)
GET    /api/drift-events/{id}              - Get specific drift event
PUT    /api/drift-events/{id}              - Update drift event
POST   /api/drift-events/{id}/resolve      - Mark as resolved
GET    /api/repositories/{id}/drift-events - Repository drift events
```

#### Analytics & Metrics
```
GET    /api/analytics/overview         - Dashboard overview metrics
GET    /api/analytics/trends          - Historical trend data
GET    /api/analytics/teams           - Team productivity metrics
GET    /api/analytics/health-scores   - Architecture health trends
POST   /api/analytics/reports         - Generate custom reports
```

#### System Management
```
GET    /api/system/status             - System health & stats
GET    /api/system/scans/active       - Active scan status
POST   /api/system/scans/{id}/cancel  - Cancel active scan
GET    /api/system/alerts             - System alerts
POST   /api/system/alerts/{id}/ack    - Acknowledge alert
```

#### Authentication & Users
```
POST   /api/auth/login                - User authentication
POST   /api/auth/refresh              - Refresh JWT token
GET    /api/users/me                  - Current user profile
PUT    /api/users/me                  - Update user preferences
GET    /api/teams                     - List user teams
```

### WebSocket Event Structure

#### Client → Server Events
```typescript
interface ClientEvents {
  'subscribe:repository': { repo: string };
  'unsubscribe:repository': { repo: string };
  'subscribe:global': {};
  'request:scan': { 
    repo: string; 
    options?: ScanOptions 
  };
  'request:health': { repo: string };
  'acknowledge:alert': { alertId: string };
  'ping': {};
}
```

#### Server → Client Events
```typescript
interface ServerEvents {
  // Real-time Data Updates
  'drift:detected': DriftEvent;
  'drift:resolved': { id: string; resolvedBy: string };
  'drift:updated': { id: string; changes: Partial<DriftEvent> };
  
  // Health & Performance
  'health:updated': ArchitectureHealth;
  'metrics:updated': SystemMetrics;
  
  // Scan Operations
  'scan:started': { repo: string; scanId: string };
  'scan:progress': { repo: string; progress: number; stage: string };
  'scan:completed': ScanResult;
  'scan:failed': { repo: string; error: string };
  
  // Alerts & Notifications
  'alert:new': Alert;
  'alert:updated': Alert;
  'system:notification': SystemNotification;
  
  // Connection Management
  'pong': {};
  'error': { code: string; message: string };
}
```

## Frontend Architecture

### State Management Strategy

#### Zustand Store Structure
```typescript
interface AppState {
  // Authentication
  auth: AuthState;
  
  // Data Management
  repositories: RepositoryState;
  driftEvents: DriftEventState;
  metrics: MetricsState;
  
  // UI State
  ui: UIState;
  notifications: NotificationState;
  filters: FilterState;
  
  // Real-time Connections
  websocket: WebSocketState;
}

// Actions organized by domain
interface AppActions {
  auth: AuthActions;
  repositories: RepositoryActions;
  driftEvents: DriftEventActions;
  metrics: MetricsActions;
  ui: UIActions;
  websocket: WebSocketActions;
}
```

#### React Query Integration
```typescript
// Custom hooks for server state
const useRepositories = () => useQuery(['repositories'], fetchRepositories);
const useDriftEvents = (filters) => useQuery(['drift-events', filters], () => fetchDriftEvents(filters));
const useHealthMetrics = (repoId) => useQuery(['health', repoId], () => fetchHealthMetrics(repoId));

// Mutations for server updates
const useCreateRepository = () => useMutation(createRepository);
const useResolveDriftEvent = () => useMutation(resolveDriftEvent);
const useTriggerScan = () => useMutation(triggerScan);
```

### Component Architecture

#### Layout Components
```
DashboardLayout/
├── Header/
│   ├── UserMenu/
│   ├── NotificationCenter/
│   └── GlobalSearch/
├── Sidebar/
│   ├── Navigation/
│   ├── RepositorySelector/
│   └── FilterPanel/
└── MainContent/
    ├── DashboardView/
    ├── AnalyticsView/
    ├── RepositoryView/
    └── SettingsView/
```

#### Data Visualization Components
```
Visualizations/
├── DriftTimeline/           # Time-series drift events
├── ArchitectureMap/         # System component relationships
├── HealthScoreGauge/        # Real-time health indicators
├── TrendAnalysis/           # Historical trends & predictions
├── TeamProductivity/        # Team collaboration metrics
├── ComplianceMatrix/        # ADR governance tracking
└── ImpactAssessment/        # Change impact visualization
```

#### Real-time Components
```typescript
// WebSocket integration component
const RealtimeProvider: React.FC = ({ children }) => {
  const { socket, connect, disconnect } = useWebSocket();
  const { addNotification } = useNotifications();
  const { updateDriftEvent, addDriftEvent } = useDriftEvents();
  
  useEffect(() => {
    socket?.on('drift:detected', addDriftEvent);
    socket?.on('drift:resolved', updateDriftEvent);
    socket?.on('alert:new', addNotification);
    
    return () => {
      socket?.off('drift:detected');
      socket?.off('drift:resolved');
      socket?.off('alert:new');
    };
  }, [socket]);
  
  return children;
};
```

## Data Flow Architecture

### Scan Workflow
```
1. User triggers scan → Frontend sends WebSocket request
2. Backend spawns PhotonDrift CLI process
3. CLI performs drift analysis with ML models
4. Backend parses CLI output → stores to database
5. WebSocket broadcasts real-time progress updates
6. Frontend updates UI with live scan progress
7. Scan completion → results displayed in dashboard
```

### Real-time Update Flow
```
1. CLI detects drift → Backend receives event
2. Backend processes and stores drift event
3. WebSocket broadcasts to subscribed clients
4. Frontend React Query cache invalidated
5. UI components re-render with new data
6. Notifications displayed to relevant users
```

### Authentication Flow
```
1. User login → JWT token issued
2. Token stored in secure httpOnly cookie
3. WebSocket connection authenticated via token
4. API requests include Authorization header
5. Token refresh handled automatically
6. Logout → token invalidated & WebSocket disconnected
```

## Security Architecture

### Authentication & Authorization
- **JWT Tokens**: Secure token-based authentication
- **Role-based Access**: Admin, user, viewer roles
- **API Rate Limiting**: Prevent abuse and DoS
- **CORS Configuration**: Restrict cross-origin requests
- **Input Validation**: Sanitize all user inputs

### Data Protection
- **Database Encryption**: Sensitive data encrypted at rest
- **HTTPS/WSS Only**: All communications encrypted
- **SQL Injection Prevention**: Parameterized queries
- **XSS Protection**: Content Security Policy headers
- **CSRF Tokens**: Cross-site request forgery prevention

### CLI Integration Security
- **Process Sandboxing**: Limit CLI process permissions
- **Path Validation**: Prevent directory traversal
- **Resource Limits**: CPU and memory constraints
- **Timeout Controls**: Prevent hanging processes
- **Audit Logging**: Track all CLI executions

## Performance Architecture

### Backend Performance
- **Connection Pooling**: Database connection management
- **Query Optimization**: Indexed database queries
- **Caching Strategy**: Redis for frequent data
- **Process Management**: Efficient CLI process handling
- **Memory Management**: Garbage collection optimization

### Frontend Performance
- **Code Splitting**: Route-based chunk loading
- **Lazy Loading**: Component-level loading
- **Virtual Scrolling**: Large dataset handling
- **Memoization**: Expensive calculation caching
- **Bundle Optimization**: Tree shaking and minification

### Real-time Performance
- **Connection Pooling**: WebSocket connection reuse
- **Message Batching**: Group related updates
- **Selective Broadcasting**: Only send relevant updates
- **Backpressure Handling**: Prevent message queuing
- **Heartbeat Monitoring**: Detect stale connections

## Scalability Architecture

### Horizontal Scaling
- **Stateless Backend**: Support multiple API instances
- **Load Balancer**: Distribute traffic across instances
- **Database Clustering**: PostgreSQL read replicas
- **WebSocket Clustering**: Socket.IO cluster adapter
- **Microservice Ready**: Modular service architecture

### Vertical Scaling
- **Database Optimization**: Efficient indexing strategy
- **Memory Management**: Optimized data structures
- **CPU Utilization**: Efficient algorithms
- **Disk I/O**: SSD storage and caching
- **Network Optimization**: Compression and CDN

### Multi-tenant Support
- **Repository Isolation**: Secure data separation
- **Resource Quotas**: Per-tenant limits
- **Custom Configurations**: Tenant-specific settings
- **Billing Integration**: Usage tracking
- **API Versioning**: Backward compatibility

## Monitoring & Observability

### Application Metrics
- **Response Times**: API endpoint performance
- **Error Rates**: Success/failure tracking
- **Resource Usage**: CPU, memory, disk metrics
- **WebSocket Metrics**: Connection counts and latency
- **CLI Performance**: Scan duration and success rates

### Business Metrics
- **Drift Detection Rate**: Events per repository
- **Resolution Time**: Time to resolve drift
- **User Engagement**: Dashboard usage patterns
- **System Health**: Overall architecture scores
- **Team Productivity**: Collaboration metrics

### Alerting Strategy
- **Critical Alerts**: System failures and security issues
- **Performance Alerts**: High latency or error rates
- **Business Alerts**: Drift thresholds exceeded
- **Capacity Alerts**: Resource utilization warnings
- **User Notifications**: In-app and email alerts

## Deployment Architecture

### Development Environment
```yaml
# docker-compose.dev.yml
version: '3.8'
services:
  frontend:
    build: ./frontend
    ports: ["3000:3000"]
    volumes: ["./frontend:/app"]
  
  backend:
    build: ./backend
    ports: ["3001:3001"]
    environment:
      - DATABASE_URL=sqlite:./dev.db
    volumes: ["./backend:/app"]
  
  database:
    image: postgres:15
    environment:
      - POSTGRES_DB=photondrift
    ports: ["5432:5432"]
```

### Production Environment
```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  frontend:
    image: photondrift-frontend:latest
    environment:
      - REACT_APP_API_URL=https://api.photondrift.com
    
  backend:
    image: photondrift-backend:latest
    environment:
      - DATABASE_URL=postgresql://...
      - REDIS_URL=redis://...
    replicas: 3
    
  nginx:
    image: nginx:alpine
    ports: ["80:80", "443:443"]
    volumes: ["./nginx.conf:/etc/nginx/nginx.conf"]
    
  database:
    image: postgres:15
    volumes: ["postgres_data:/var/lib/postgresql/data"]
    
  redis:
    image: redis:7-alpine
    volumes: ["redis_data:/data"]
```

### CI/CD Pipeline
```yaml
# .github/workflows/deploy.yml
name: Deploy PhotonDrift Dashboard
on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: |
          cd backend && npm test
          cd frontend && npm test
          
  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Build Docker images
        run: |
          docker build -t photondrift-frontend ./frontend
          docker build -t photondrift-backend ./backend
          
  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to production
        run: docker-compose -f docker-compose.prod.yml up -d
```

## Implementation Phases

### Phase 1: Core Integration (Weeks 1-2)
- [ ] Complete backend API implementation
- [ ] WebSocket handlers for real-time updates
- [ ] Database schema and migrations
- [ ] CLI integration service enhancement
- [ ] Basic authentication system

### Phase 2: Frontend Foundation (Weeks 3-4)
- [ ] React component architecture
- [ ] State management with Zustand
- [ ] WebSocket integration
- [ ] Basic visualization components
- [ ] Repository management UI

### Phase 3: Advanced Features (Weeks 5-6)
- [ ] Advanced analytics dashboards
- [ ] Team collaboration features
- [ ] Alert and notification system
- [ ] Export and reporting capabilities
- [ ] Performance optimizations

### Phase 4: Production Readiness (Weeks 7-8)
- [ ] Security hardening
- [ ] Performance testing and optimization
- [ ] Deployment automation
- [ ] Monitoring and observability
- [ ] Documentation and training

## Risk Mitigation

### Technical Risks
- **CLI Process Management**: Implement robust process cleanup and timeout handling
- **WebSocket Scalability**: Use Socket.IO cluster adapter for multi-instance support
- **Database Performance**: Implement proper indexing and query optimization
- **Memory Leaks**: Regular monitoring and profiling of Node.js processes
- **Security Vulnerabilities**: Regular dependency updates and security audits

### Operational Risks
- **Deployment Failures**: Blue-green deployment strategy with rollback capability
- **Data Loss**: Automated backups and disaster recovery procedures
- **Performance Degradation**: Real-time monitoring with automated scaling
- **User Adoption**: Comprehensive documentation and training materials
- **Maintenance Overhead**: Automated testing and deployment pipelines

## Success Metrics

### Technical Metrics
- **API Response Time**: < 200ms for 95th percentile
- **WebSocket Latency**: < 50ms for real-time updates
- **Uptime**: > 99.9% availability
- **Error Rate**: < 0.1% for API endpoints
- **Scan Performance**: < 30 seconds for large repositories

### Business Metrics
- **User Adoption**: 80% of development teams actively using dashboard
- **Drift Resolution Time**: 50% reduction in average resolution time
- **Architecture Compliance**: 90% of repositories meeting health thresholds
- **Team Productivity**: 25% improvement in decision velocity
- **System ROI**: Positive return on investment within 6 months

---

**Version**: 1.0.0  
**Author**: Dashboard Architect Agent  
**Date**: 2025-01-21  
**Status**: Architecture Ready for Implementation