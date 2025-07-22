// Core data models for PhotonDrift Dashboard

export interface DriftEvent {
    id: string;
    timestamp: Date;
    severity: 'low' | 'medium' | 'high' | 'critical';
    category: string;
    title: string;
    description: string;
    location: FileLocation;
    mlScore?: number;
    confidence?: number;
    resolved: boolean;
    assignee?: string;
    tags: string[];
    suggestion?: string;
}

export interface FileLocation {
    file: string;
    line?: number;
    column?: number;
}

export interface ScanOptions {
    format?: 'json' | 'yaml' | 'csv';
    enableML?: boolean;
    confidenceThreshold?: number;
    includeFiles?: string[];
    excludeFiles?: string[];
    verbose?: boolean;
}

export interface ScanResult {
    id: string;
    repositoryPath: string;
    timestamp: Date;
    driftEvents: DriftEvent[];
    summary: ScanSummary;
    options: ScanOptions;
    success: boolean;
    error?: string;
}

export interface ScanSummary {
    totalEvents: number;
    criticalEvents: number;
    highEvents: number;
    mediumEvents: number;
    lowEvents: number;
    scanDuration: number;
}

export interface ArchitectureHealth {
    id: string;
    repository: string;
    timestamp: Date;
    overallScore: number; // 0-100
    metrics: HealthMetrics;
    trends: HealthTrends;
}

export interface HealthMetrics {
    driftCount: number;
    coverage: number;
    compliance: number;
    maintainability: number;
    technicalDebt: number;
}

export interface HealthTrends {
    direction: 'improving' | 'stable' | 'degrading';
    velocity: number;
}

export interface TeamMetrics {
    team: string;
    period: DateRange;
    metrics: TeamProductivityMetrics;
    members: TeamMember[];
}

export interface TeamProductivityMetrics {
    adrsCreated: number;
    driftResolved: number;
    reviewTime: number; // hours
    collaborationScore: number;
    decisionVelocity: number;
}

export interface TeamMember {
    id: string;
    name: string;
    email: string;
    role: string;
    contributions: ContributionMetrics;
}

export interface ContributionMetrics {
    adrsAuthored: number;
    driftResolved: number;
    reviewsCompleted: number;
    discussionParticipation: number;
}

export interface DateRange {
    start: Date;
    end: Date;
}

export interface Repository {
    id: string;
    name: string;
    path: string;
    description?: string;
    lastScanned?: Date;
    health?: ArchitectureHealth;
    isActive: boolean;
    configuration: RepositoryConfiguration;
}

export interface RepositoryConfiguration {
    adrDirectory: string;
    enableAutoScan: boolean;
    scanInterval: number; // minutes
    mlThreshold: number;
    excludePatterns: string[];
    includePatterns: string[];
    notifications: NotificationSettings;
}

export interface NotificationSettings {
    enabled: boolean;
    emailAlerts: boolean;
    slackWebhook?: string;
    criticalThreshold: number;
    highThreshold: number;
}

export interface User {
    id: string;
    email: string;
    name: string;
    role: 'admin' | 'user' | 'viewer';
    preferences: UserPreferences;
    lastActive: Date;
    createdAt: Date;
}

export interface UserPreferences {
    theme: 'light' | 'dark' | 'auto';
    notifications: boolean;
    defaultView: string;
    refreshInterval: number;
    chartColors: string[];
}

export interface Dashboard {
    id: string;
    name: string;
    description?: string;
    userId: string;
    isPublic: boolean;
    layout: DashboardLayout;
    widgets: DashboardWidget[];
    createdAt: Date;
    updatedAt: Date;
}

export interface DashboardLayout {
    columns: number;
    rows: number;
    gap: number;
}

export interface DashboardWidget {
    id: string;
    type: 'drift-timeline' | 'health-score' | 'metrics-card' | 'trend-chart' | 'heat-map';
    title: string;
    position: WidgetPosition;
    size: WidgetSize;
    configuration: WidgetConfiguration;
    dataSource: WidgetDataSource;
}

export interface WidgetPosition {
    x: number;
    y: number;
}

export interface WidgetSize {
    width: number;
    height: number;
}

export interface WidgetConfiguration {
    [key: string]: any;
}

export interface WidgetDataSource {
    type: 'repository' | 'team' | 'global';
    filters: DataFilter[];
    aggregation?: DataAggregation;
}

export interface DataFilter {
    field: string;
    operator: 'equals' | 'contains' | 'greater_than' | 'less_than' | 'between';
    value: any;
}

export interface DataAggregation {
    type: 'sum' | 'avg' | 'count' | 'max' | 'min';
    field: string;
    groupBy?: string[];
    timeWindow?: string;
}

export interface Alert {
    id: string;
    type: 'drift_detected' | 'health_degraded' | 'scan_failed' | 'threshold_exceeded';
    severity: 'info' | 'warning' | 'error' | 'critical';
    title: string;
    message: string;
    repository: string;
    timestamp: Date;
    acknowledged: boolean;
    acknowledgedBy?: string;
    acknowledgedAt?: Date;
    metadata: AlertMetadata;
}

export interface AlertMetadata {
    [key: string]: any;
}

export interface SystemStats {
    totalRepositories: number;
    activeScans: number;
    totalDriftEvents: number;
    resolvedDriftEvents: number;
    systemHealth: number;
    uptime: number;
    memoryUsage: number;
    cpuUsage: number;
    lastUpdate: Date;
}

// WebSocket event types
export interface ClientEvents {
    'subscribe:repository': { repo: string };
    'unsubscribe:repository': { repo: string };
    'request:scan': { repo: string; options?: ScanOptions };
    'request:health': { repo: string };
    'acknowledge:alert': { alertId: string };
}

export interface ServerEvents {
    'drift:detected': DriftEvent;
    'drift:resolved': { id: string };
    'health:updated': ArchitectureHealth;
    'scan:progress': { repo: string; progress: number };
    'scan:completed': ScanResult;
    'scan:failed': { repo: string; error: string };
    'alert:new': Alert;
    'system:stats': SystemStats;
}

// API Response types
export interface APIResponse<T = any> {
    success: boolean;
    data?: T;
    error?: string;
    message?: string;
    timestamp: Date;
}

export interface PaginatedResponse<T> extends APIResponse<T[]> {
    pagination: {
        page: number;
        limit: number;
        total: number;
        totalPages: number;
    };
}

// Database models
export interface DBDriftEvent {
    id: string;
    repository_id: string;
    timestamp: string;
    severity: string;
    category: string;
    title: string;
    description: string;
    file_path: string;
    line_number?: number;
    column_number?: number;
    ml_score?: number;
    confidence?: number;
    resolved: boolean;
    assigned_to?: string;
    tags: string; // JSON string
    suggestion?: string;
    created_at: string;
    updated_at: string;
}

export interface DBRepository {
    id: string;
    name: string;
    path: string;
    description?: string;
    last_scanned?: string;
    is_active: boolean;
    configuration: string; // JSON string
    created_at: string;
    updated_at: string;
}

export interface DBScanResult {
    id: string;
    repository_id: string;
    timestamp: string;
    total_events: number;
    critical_events: number;
    high_events: number;
    medium_events: number;
    low_events: number;
    scan_duration: number;
    success: boolean;
    error?: string;
    options: string; // JSON string
    created_at: string;
}

export interface DBUser {
    id: string;
    email: string;
    name: string;
    password_hash: string;
    role: string;
    preferences: string; // JSON string
    last_active: string;
    created_at: string;
    updated_at: string;
}