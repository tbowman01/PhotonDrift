-- PhotonDrift Dashboard Database Schema
-- Supports both SQLite (development) and PostgreSQL (production)

-- Enable foreign key constraints (SQLite)
PRAGMA foreign_keys = ON;

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    email TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT CHECK (role IN ('admin', 'user', 'viewer')) DEFAULT 'user',
    preferences TEXT DEFAULT '{}', -- JSON
    last_active DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Teams table
CREATE TABLE IF NOT EXISTS teams (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Team memberships
CREATE TABLE IF NOT EXISTS team_members (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    team_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role TEXT CHECK (role IN ('admin', 'member', 'viewer')) DEFAULT 'member',
    joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(team_id, user_id)
);

-- Repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT UNIQUE NOT NULL,
    path TEXT NOT NULL,
    description TEXT,
    last_scanned DATETIME,
    is_active BOOLEAN DEFAULT 1,
    configuration TEXT DEFAULT '{}', -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Repository access control
CREATE TABLE IF NOT EXISTS repository_access (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    repository_id TEXT NOT NULL,
    user_id TEXT,
    team_id TEXT,
    permission TEXT CHECK (permission IN ('read', 'write', 'admin')) DEFAULT 'read',
    granted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    granted_by TEXT,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (granted_by) REFERENCES users(id),
    CHECK ((user_id IS NOT NULL AND team_id IS NULL) OR (user_id IS NULL AND team_id IS NOT NULL))
);

-- Scan results table
CREATE TABLE IF NOT EXISTS scan_results (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    repository_id TEXT NOT NULL,
    scan_id TEXT UNIQUE NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    total_events INTEGER DEFAULT 0,
    critical_events INTEGER DEFAULT 0,
    high_events INTEGER DEFAULT 0,
    medium_events INTEGER DEFAULT 0,
    low_events INTEGER DEFAULT 0,
    scan_duration REAL DEFAULT 0, -- seconds
    success BOOLEAN DEFAULT 1,
    error TEXT,
    options TEXT DEFAULT '{}', -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
);

-- Drift events table
CREATE TABLE IF NOT EXISTS drift_events (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    repository_id TEXT NOT NULL,
    scan_result_id TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    severity TEXT CHECK (severity IN ('low', 'medium', 'high', 'critical')) DEFAULT 'medium',
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    file_path TEXT NOT NULL,
    line_number INTEGER,
    column_number INTEGER,
    ml_score REAL,
    confidence REAL,
    resolved BOOLEAN DEFAULT 0,
    assigned_to TEXT,
    tags TEXT DEFAULT '[]', -- JSON array
    suggestion TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (scan_result_id) REFERENCES scan_results(id) ON DELETE SET NULL,
    FOREIGN KEY (assigned_to) REFERENCES users(id) ON DELETE SET NULL
);

-- Drift event resolutions
CREATE TABLE IF NOT EXISTS drift_resolutions (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    drift_event_id TEXT UNIQUE NOT NULL,
    resolution_type TEXT CHECK (resolution_type IN ('fixed', 'accepted', 'false_positive', 'deferred')) DEFAULT 'fixed',
    description TEXT,
    commit_hash TEXT,
    resolved_by TEXT NOT NULL,
    verified_by TEXT,
    resolved_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    verified_at DATETIME,
    FOREIGN KEY (drift_event_id) REFERENCES drift_events(id) ON DELETE CASCADE,
    FOREIGN KEY (resolved_by) REFERENCES users(id),
    FOREIGN KEY (verified_by) REFERENCES users(id)
);

-- Architecture health snapshots
CREATE TABLE IF NOT EXISTS architecture_health (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    repository_id TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    overall_score INTEGER CHECK (overall_score >= 0 AND overall_score <= 100),
    drift_count INTEGER DEFAULT 0,
    coverage_score INTEGER DEFAULT 0,
    compliance_score INTEGER DEFAULT 0,
    maintainability_score INTEGER DEFAULT 0,
    technical_debt_score INTEGER DEFAULT 0,
    trend_direction TEXT CHECK (trend_direction IN ('improving', 'stable', 'degrading')) DEFAULT 'stable',
    trend_velocity REAL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE
);

-- System alerts
CREATE TABLE IF NOT EXISTS alerts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    type TEXT CHECK (type IN ('drift_detected', 'health_degraded', 'scan_failed', 'threshold_exceeded', 'system_error')) NOT NULL,
    severity TEXT CHECK (severity IN ('info', 'warning', 'error', 'critical')) DEFAULT 'info',
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    repository_id TEXT,
    user_id TEXT,
    metadata TEXT DEFAULT '{}', -- JSON
    acknowledged BOOLEAN DEFAULT 0,
    acknowledged_by TEXT,
    acknowledged_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (acknowledged_by) REFERENCES users(id)
);

-- Dashboards (custom user dashboards)
CREATE TABLE IF NOT EXISTS dashboards (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    name TEXT NOT NULL,
    description TEXT,
    user_id TEXT NOT NULL,
    is_public BOOLEAN DEFAULT 0,
    layout TEXT DEFAULT '{"columns":12,"rows":8,"gap":16}', -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Dashboard widgets
CREATE TABLE IF NOT EXISTS dashboard_widgets (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    dashboard_id TEXT NOT NULL,
    widget_type TEXT CHECK (widget_type IN ('drift-timeline', 'health-score', 'metrics-card', 'trend-chart', 'heat-map', 'team-metrics')) NOT NULL,
    title TEXT NOT NULL,
    position_x INTEGER NOT NULL,
    position_y INTEGER NOT NULL,
    size_width INTEGER NOT NULL,
    size_height INTEGER NOT NULL,
    configuration TEXT DEFAULT '{}', -- JSON
    data_source TEXT DEFAULT '{}', -- JSON
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (dashboard_id) REFERENCES dashboards(id) ON DELETE CASCADE
);

-- Audit log for tracking changes
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    user_id TEXT,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    old_values TEXT, -- JSON
    new_values TEXT, -- JSON
    ip_address TEXT,
    user_agent TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- System configuration
CREATE TABLE IF NOT EXISTS system_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT,
    FOREIGN KEY (updated_by) REFERENCES users(id)
);

-- WebSocket sessions (for connection management)
CREATE TABLE IF NOT EXISTS websocket_sessions (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    socket_id TEXT UNIQUE NOT NULL,
    user_id TEXT,
    ip_address TEXT,
    user_agent TEXT,
    subscriptions TEXT DEFAULT '[]', -- JSON array of repository IDs
    connected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_activity DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Active scans tracking
CREATE TABLE IF NOT EXISTS active_scans (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    scan_id TEXT UNIQUE NOT NULL,
    repository_id TEXT NOT NULL,
    user_id TEXT,
    status TEXT CHECK (status IN ('queued', 'running', 'completed', 'failed', 'cancelled')) DEFAULT 'queued',
    progress INTEGER DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
    stage TEXT DEFAULT 'initializing',
    process_id INTEGER,
    options TEXT DEFAULT '{}', -- JSON
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    error TEXT,
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_drift_events_repository ON drift_events(repository_id);
CREATE INDEX IF NOT EXISTS idx_drift_events_timestamp ON drift_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_drift_events_severity ON drift_events(severity);
CREATE INDEX IF NOT EXISTS idx_drift_events_resolved ON drift_events(resolved);
CREATE INDEX IF NOT EXISTS idx_drift_events_category ON drift_events(category);

CREATE INDEX IF NOT EXISTS idx_scan_results_repository ON scan_results(repository_id);
CREATE INDEX IF NOT EXISTS idx_scan_results_timestamp ON scan_results(timestamp);

CREATE INDEX IF NOT EXISTS idx_architecture_health_repository ON architecture_health(repository_id);
CREATE INDEX IF NOT EXISTS idx_architecture_health_timestamp ON architecture_health(timestamp);

CREATE INDEX IF NOT EXISTS idx_alerts_repository ON alerts(repository_id);
CREATE INDEX IF NOT EXISTS idx_alerts_severity ON alerts(severity);
CREATE INDEX IF NOT EXISTS idx_alerts_acknowledged ON alerts(acknowledged);
CREATE INDEX IF NOT EXISTS idx_alerts_created_at ON alerts(created_at);

CREATE INDEX IF NOT EXISTS idx_audit_log_user ON audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp ON audit_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_audit_log_resource ON audit_log(resource_type, resource_id);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_last_active ON users(last_active);

CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name);
CREATE INDEX IF NOT EXISTS idx_repositories_active ON repositories(is_active);

-- Triggers for updating timestamps (SQLite version)
CREATE TRIGGER IF NOT EXISTS trigger_users_updated_at
    AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trigger_repositories_updated_at
    AFTER UPDATE ON repositories
BEGIN
    UPDATE repositories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trigger_drift_events_updated_at
    AFTER UPDATE ON drift_events
BEGIN
    UPDATE drift_events SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trigger_dashboards_updated_at
    AFTER UPDATE ON dashboards
BEGIN
    UPDATE dashboards SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trigger_dashboard_widgets_updated_at
    AFTER UPDATE ON dashboard_widgets
BEGIN
    UPDATE dashboard_widgets SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- Trigger to update drift_events.resolved when resolution is added
CREATE TRIGGER IF NOT EXISTS trigger_drift_resolution_created
    AFTER INSERT ON drift_resolutions
BEGIN
    UPDATE drift_events 
    SET resolved = 1, updated_at = CURRENT_TIMESTAMP 
    WHERE id = NEW.drift_event_id;
END;

-- Trigger to update drift_events.resolved when resolution is deleted
CREATE TRIGGER IF NOT EXISTS trigger_drift_resolution_deleted
    AFTER DELETE ON drift_resolutions
BEGIN
    UPDATE drift_events 
    SET resolved = 0, updated_at = CURRENT_TIMESTAMP 
    WHERE id = OLD.drift_event_id;
END;

-- Views for common queries
CREATE VIEW IF NOT EXISTS v_drift_events_with_resolution AS
SELECT 
    de.*,
    dr.resolution_type,
    dr.description as resolution_description,
    dr.commit_hash,
    dr.resolved_by,
    dr.verified_by,
    dr.resolved_at,
    dr.verified_at,
    ru.name as resolved_by_name,
    vu.name as verified_by_name
FROM drift_events de
LEFT JOIN drift_resolutions dr ON de.id = dr.drift_event_id
LEFT JOIN users ru ON dr.resolved_by = ru.id
LEFT JOIN users vu ON dr.verified_by = vu.id;

CREATE VIEW IF NOT EXISTS v_repository_health_latest AS
SELECT 
    r.id as repository_id,
    r.name as repository_name,
    r.path as repository_path,
    r.is_active,
    ah.*
FROM repositories r
LEFT JOIN architecture_health ah ON r.id = ah.repository_id
WHERE ah.id = (
    SELECT id 
    FROM architecture_health ah2 
    WHERE ah2.repository_id = r.id 
    ORDER BY timestamp DESC 
    LIMIT 1
);

CREATE VIEW IF NOT EXISTS v_drift_summary_by_repository AS
SELECT 
    r.id as repository_id,
    r.name as repository_name,
    COUNT(de.id) as total_drift_events,
    COUNT(CASE WHEN de.severity = 'critical' THEN 1 END) as critical_events,
    COUNT(CASE WHEN de.severity = 'high' THEN 1 END) as high_events,
    COUNT(CASE WHEN de.severity = 'medium' THEN 1 END) as medium_events,
    COUNT(CASE WHEN de.severity = 'low' THEN 1 END) as low_events,
    COUNT(CASE WHEN de.resolved = 1 THEN 1 END) as resolved_events,
    COUNT(CASE WHEN de.resolved = 0 THEN 1 END) as unresolved_events
FROM repositories r
LEFT JOIN drift_events de ON r.id = de.repository_id
GROUP BY r.id, r.name;

-- Sample data insertion (for development)
INSERT OR IGNORE INTO system_config (key, value, description) VALUES 
('app_version', '1.0.0', 'Application version'),
('max_scan_duration', '300', 'Maximum scan duration in seconds'),
('default_scan_interval', '3600', 'Default scan interval in seconds'),
('ml_confidence_threshold', '0.7', 'Default ML confidence threshold'),
('alert_retention_days', '90', 'Days to retain alert records'),
('audit_retention_days', '365', 'Days to retain audit logs');

-- Create default admin user (password: 'admin123' - change in production!)
INSERT OR IGNORE INTO users (id, email, name, password_hash, role) VALUES 
('admin_001', 'admin@photondrift.com', 'System Administrator', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LDMcjkKCz9hKvhFfe', 'admin');

-- PostgreSQL-specific optimizations (uncomment for production PostgreSQL)
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- CREATE EXTENSION IF NOT EXISTS "pg_trgm";
-- 
-- -- Use UUIDs for PostgreSQL
-- ALTER TABLE users ALTER COLUMN id SET DEFAULT gen_random_uuid()::text;
-- ALTER TABLE teams ALTER COLUMN id SET DEFAULT gen_random_uuid()::text;
-- ALTER TABLE repositories ALTER COLUMN id SET DEFAULT gen_random_uuid()::text;
-- 
-- -- Full-text search indexes
-- CREATE INDEX IF NOT EXISTS idx_drift_events_fts ON drift_events USING gin(to_tsvector('english', title || ' ' || description));
-- CREATE INDEX IF NOT EXISTS idx_repositories_fts ON repositories USING gin(to_tsvector('english', name || ' ' || description));
-- 
-- -- Partial indexes for common queries
-- CREATE INDEX IF NOT EXISTS idx_drift_events_unresolved ON drift_events(repository_id, timestamp) WHERE resolved = false;
-- CREATE INDEX IF NOT EXISTS idx_alerts_unacknowledged ON alerts(repository_id, created_at) WHERE acknowledged = false;

PRAGMA analyze;