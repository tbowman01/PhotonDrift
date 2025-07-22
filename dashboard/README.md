# PhotonDrift Visual Analytics Dashboard

## 🎯 Overview

The PhotonDrift Visual Analytics Dashboard provides real-time insights into architectural drift patterns, team collaboration metrics, and system health indicators. Built with modern web technologies for scalable, interactive data visualization.

## 🏗️ Architecture

### Technology Stack
- **Frontend**: React 18 + TypeScript + Vite
- **UI Framework**: Material-UI (MUI) with custom theme
- **Data Visualization**: D3.js + Recharts for interactive charts
- **Real-time Updates**: WebSocket connections + React Query
- **State Management**: Zustand for global state
- **Styling**: Styled Components + CSS-in-JS
- **Testing**: Vitest + React Testing Library

### Backend API
- **Runtime**: Node.js + Express + TypeScript
- **WebSocket**: Socket.IO for real-time updates
- **Database**: SQLite for development, PostgreSQL for production
- **CLI Integration**: Spawn PhotonDrift CLI processes
- **Authentication**: JWT tokens with optional OAuth
- **API Design**: RESTful endpoints + GraphQL for complex queries

### Data Flow
```
PhotonDrift CLI → Backend API → WebSocket → React Components → D3 Visualizations
```

## 🎨 Features

### Core Dashboard
- **Overview Cards**: Key metrics at a glance
- **Drift Timeline**: Interactive time-series visualization
- **Architecture Map**: Visual representation of system components
- **Health Score**: Real-time system health indicators
- **Alert Center**: Notifications for critical drift events

### Analytics Views
- **Trend Analysis**: Historical drift patterns and predictions
- **Team Insights**: Productivity metrics and collaboration data
- **Impact Assessment**: Change impact visualization
- **Compliance Dashboard**: ADR governance and compliance tracking
- **Performance Metrics**: System performance and optimization insights

### Interactive Features
- **Real-time Updates**: Live data streaming via WebSockets
- **Drill-down Analysis**: Click-through detailed views
- **Custom Filters**: Date ranges, teams, repositories
- **Export Capabilities**: PDF reports, CSV data, JSON exports
- **Collaborative Annotations**: Team comments and insights

## 📊 Data Models

### Drift Event
```typescript
interface DriftEvent {
  id: string;
  timestamp: Date;
  severity: 'low' | 'medium' | 'high' | 'critical';
  category: DriftCategory;
  title: string;
  description: string;
  location: FileLocation;
  mlScore?: number;
  confidence?: number;
  resolved: boolean;
  assignee?: string;
  tags: string[];
}
```

### Architecture Health
```typescript
interface ArchitectureHealth {
  id: string;
  repository: string;
  timestamp: Date;
  overallScore: number; // 0-100
  metrics: {
    driftCount: number;
    coverage: number;
    compliance: number;
    maintainability: number;
    technical_debt: number;
  };
  trends: {
    direction: 'improving' | 'stable' | 'degrading';
    velocity: number;
  };
}
```

### Team Productivity
```typescript
interface TeamMetrics {
  team: string;
  period: DateRange;
  metrics: {
    adrs_created: number;
    drift_resolved: number;
    review_time: number; // hours
    collaboration_score: number;
    decision_velocity: number;
  };
  members: TeamMember[];
}
```

## 🚀 Development Setup

### Prerequisites
- Node.js 18+
- npm or yarn
- PhotonDrift CLI installed
- Git

### Quick Start
```bash
# Clone and setup
git clone https://github.com/tbowman01/PhotonDrift
cd PhotonDrift/dashboard

# Install dependencies
npm install

# Setup environment
cp .env.example .env.local

# Start development servers
npm run dev        # Frontend + Backend
npm run dev:frontend  # Frontend only
npm run dev:backend   # Backend only

# Open browser
open http://localhost:3000
```

### Environment Variables
```env
# Backend Configuration
PORT=3001
DATABASE_URL=sqlite:./dev.db
PHOTONDRIFT_CLI_PATH=adrscan

# WebSocket Configuration  
WEBSOCKET_PORT=3002

# Authentication (Optional)
JWT_SECRET=your-jwt-secret
OAUTH_CLIENT_ID=optional-oauth-id
OAUTH_CLIENT_SECRET=optional-oauth-secret

# Frontend Configuration
REACT_APP_API_URL=http://localhost:3001
REACT_APP_WS_URL=ws://localhost:3002
```

## 🎯 Component Architecture

### Layout Components
- `DashboardLayout` - Main application shell
- `Sidebar` - Navigation and filters
- `Header` - User controls and notifications
- `MainContent` - Dynamic content area

### Visualization Components
- `DriftTimeline` - Interactive time-series chart
- `ArchitectureMap` - System component visualization
- `MetricsCards` - Key indicator cards
- `TrendChart` - Historical trend analysis
- `HeatMap` - Repository health visualization

### Data Components  
- `DataProvider` - React Query setup
- `WebSocketProvider` - Real-time updates
- `StateProvider` - Global state management
- `ErrorBoundary` - Error handling

### Feature Components
- `ADRList` - Architecture decisions list
- `DriftDetails` - Individual drift analysis
- `TeamDashboard` - Team-specific metrics
- `ReportBuilder` - Custom report generation
- `SettingsPanel` - User preferences

## 📈 Visualization Library

### Chart Types
- **Time Series**: Drift events over time
- **Bar Charts**: Categorical comparisons
- **Pie Charts**: Distribution analysis
- **Scatter Plots**: Correlation analysis
- **Heat Maps**: Multi-dimensional data
- **Network Graphs**: Architecture relationships
- **Sankey Diagrams**: Flow visualization

### Interactive Features
- **Zoom/Pan**: Detailed time range exploration
- **Brushing**: Multi-chart linked selection
- **Tooltips**: Contextual information display
- **Crossfilter**: Dynamic filtering across views
- **Annotations**: User-added insights
- **Export**: Save visualizations as images

## 🔄 Real-time Updates

### WebSocket Events
```typescript
// Client → Server
interface ClientEvents {
  'subscribe:repository': { repo: string };
  'unsubscribe:repository': { repo: string };
  'request:scan': { repo: string, options?: ScanOptions };
}

// Server → Client  
interface ServerEvents {
  'drift:detected': DriftEvent;
  'drift:resolved': { id: string };
  'health:updated': ArchitectureHealth;
  'scan:progress': { repo: string, progress: number };
  'scan:completed': { repo: string, results: DriftEvent[] };
}
```

### Update Strategies
- **Incremental Updates**: Only send changed data
- **Batch Processing**: Group related updates
- **Conflict Resolution**: Handle concurrent modifications
- **Offline Support**: Queue updates for reconnection
- **Rate Limiting**: Prevent server overload

## 🧪 Testing Strategy

### Unit Tests
- Component rendering and behavior
- Data transformation utilities
- WebSocket event handlers
- State management logic
- Chart interaction functions

### Integration Tests
- API endpoint functionality
- Database operations
- CLI integration
- WebSocket communication
- Authentication flows

### E2E Tests
- Complete user workflows
- Real-time update scenarios
- Cross-browser compatibility
- Performance benchmarks
- Accessibility compliance

### Performance Tests
- Large dataset rendering
- Real-time update performance
- Memory usage optimization
- Bundle size analysis
- Loading time benchmarks

## 🔐 Security Considerations

### Frontend Security
- **XSS Prevention**: Input sanitization
- **CSRF Protection**: Token validation
- **Content Security Policy**: Strict CSP headers
- **Secure Communication**: HTTPS/WSS only
- **Input Validation**: Client-side validation

### Backend Security
- **Authentication**: JWT token validation
- **Authorization**: Role-based access control
- **Rate Limiting**: API abuse prevention
- **Input Sanitization**: SQL injection prevention
- **Audit Logging**: Security event tracking

### Data Privacy
- **Data Minimization**: Collect only necessary data
- **Encryption**: Sensitive data encryption
- **Access Logs**: Track data access
- **Retention Policies**: Automatic data cleanup
- **GDPR Compliance**: Privacy regulation adherence

## 📱 Responsive Design

### Breakpoints
- **Mobile**: < 768px (Stack layout)
- **Tablet**: 768px - 1024px (Simplified views)
- **Desktop**: > 1024px (Full feature set)
- **Large Screen**: > 1440px (Enhanced data density)

### Mobile Optimizations
- **Touch Interactions**: Gesture-friendly controls
- **Simplified Navigation**: Collapsible menus
- **Optimized Charts**: Mobile-friendly visualizations
- **Performance**: Reduced data loading
- **Offline Capability**: Core features without connection

## 🚀 Deployment Options

### Development
```bash
npm run dev
# Frontend: http://localhost:3000
# Backend: http://localhost:3001
# WebSocket: ws://localhost:3002
```

### Production Build
```bash
npm run build
npm run start
```

### Docker Deployment
```bash
docker build -t photondrift-dashboard .
docker run -p 3000:3000 photondrift-dashboard
```

### Cloud Deployment
- **Vercel**: Frontend static deployment
- **Railway**: Full-stack deployment
- **AWS**: ECS/Lambda deployment
- **Google Cloud**: Cloud Run deployment
- **Self-hosted**: Docker Compose setup

## 🎛️ Configuration

### Dashboard Settings
```json
{
  "refresh_interval": 30000,
  "max_events": 1000,
  "chart_theme": "dark",
  "notification_level": "medium",
  "auto_export": false,
  "collaboration": true
}
```

### Visualization Options
```json
{
  "charts": {
    "drift_timeline": {
      "height": 400,
      "show_confidence": true,
      "aggregation": "daily"
    },
    "health_score": {
      "threshold_warning": 70,
      "threshold_critical": 50,
      "trend_days": 30
    }
  }
}
```

## 📊 Performance Metrics

### Target Performance
- **First Contentful Paint**: < 1.2s
- **Largest Contentful Paint**: < 2.5s
- **Time to Interactive**: < 3s
- **Cumulative Layout Shift**: < 0.1
- **Bundle Size**: < 500KB gzipped

### Optimization Techniques
- **Code Splitting**: Route-based chunks
- **Lazy Loading**: Component-level loading
- **Virtual Scrolling**: Large dataset handling
- **Memoization**: Expensive calculation caching
- **Worker Threads**: Heavy computation offloading

## 🔧 Development Tools

### Code Quality
- **ESLint**: JavaScript/TypeScript linting
- **Prettier**: Code formatting
- **Husky**: Git hooks
- **TypeScript**: Static type checking
- **Storybook**: Component development

### Build Tools
- **Vite**: Fast build tool and dev server
- **Rollup**: Production bundling
- **PostCSS**: CSS processing
- **Workbox**: Service worker generation
- **Bundle Analyzer**: Size analysis

### Development Experience
- **Hot Reload**: Instant development feedback
- **TypeScript**: Enhanced developer experience
- **DevTools**: Redux/Zustand debugging
- **Error Boundaries**: Graceful error handling
- **Logging**: Structured development logging

---

**Version**: 1.0.0  
**Last Updated**: 2025-01-21  
**Status**: Development Ready