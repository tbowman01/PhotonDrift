import * as sqlite3 from 'sqlite3';
import { promises as fs } from 'fs';
import * as path from 'path';
import { logger } from '../utils/logger.js';

// Enable verbose mode in development
if (process.env.NODE_ENV === 'development') {
    sqlite3.verbose();
}

const DB_PATH = process.env.DATABASE_PATH || './data/photondrift.db';
const SCHEMA_PATH = './src/database/schema.sql';

let db: sqlite3.Database | null = null;

/**
 * Initialize the database connection and create tables
 */
export async function initDatabase(): Promise<sqlite3.Database> {
    try {
        // Ensure data directory exists
        const dbDir = path.dirname(DB_PATH);
        await fs.mkdir(dbDir, { recursive: true });
        
        // Create database connection
        db = new sqlite3.Database(DB_PATH, sqlite3.OPEN_READWRITE | sqlite3.OPEN_CREATE, (err) => {
            if (err) {
                logger.error('Failed to connect to database:', err);
                throw err;
            }
            logger.info(`Connected to SQLite database: ${DB_PATH}`);
        });
        
        // Enable foreign keys
        await runQuery('PRAGMA foreign_keys = ON');
        
        // Create tables if they don't exist
        await createTables();
        
        // Run any pending migrations
        await runMigrations();
        
        logger.info('Database initialization completed successfully');
        return db;
        
    } catch (error) {
        logger.error('Database initialization failed:', error);
        throw error;
    }
}

/**
 * Get the current database instance
 */
export function getDatabase(): sqlite3.Database {
    if (!db) {
        throw new Error('Database not initialized. Call initDatabase() first.');
    }
    return db;
}

/**
 * Close the database connection
 */
export async function closeDatabase(): Promise<void> {
    if (db) {
        return new Promise((resolve, reject) => {
            db!.close((err) => {
                if (err) {
                    logger.error('Failed to close database:', err);
                    reject(err);
                } else {
                    logger.info('Database connection closed');
                    db = null;
                    resolve();
                }
            });
        });
    }
}

/**
 * Execute a SQL query that doesn't return results
 */
export function runQuery(sql: string, params: any[] = []): Promise<sqlite3.RunResult> {
    return new Promise((resolve, reject) => {
        if (!db) {
            reject(new Error('Database not initialized'));
            return;
        }
        
        db.run(sql, params, function(err) {
            if (err) {
                logger.error('Query execution failed:', { sql, params, error: err });
                reject(err);
            } else {
                resolve(this);
            }
        });
    });
}

/**
 * Execute a SQL query that returns a single row
 */
export function getQuery<T = any>(sql: string, params: any[] = []): Promise<T | undefined> {
    return new Promise((resolve, reject) => {
        if (!db) {
            reject(new Error('Database not initialized'));
            return;
        }
        
        db.get(sql, params, (err, row) => {
            if (err) {
                logger.error('Query execution failed:', { sql, params, error: err });
                reject(err);
            } else {
                resolve(row as T);
            }
        });
    });
}

/**
 * Execute a SQL query that returns multiple rows
 */
export function getAllQuery<T = any>(sql: string, params: any[] = []): Promise<T[]> {
    return new Promise((resolve, reject) => {
        if (!db) {
            reject(new Error('Database not initialized'));
            return;
        }
        
        db.all(sql, params, (err, rows) => {
            if (err) {
                logger.error('Query execution failed:', { sql, params, error: err });
                reject(err);
            } else {
                resolve(rows as T[]);
            }
        });
    });
}

/**
 * Create database tables
 */
async function createTables(): Promise<void> {
    const tables = [
        // Repositories table
        `
        CREATE TABLE IF NOT EXISTS repositories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            description TEXT,
            last_scanned DATETIME,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            configuration TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        `,
        
        // Drift events table
        `
        CREATE TABLE IF NOT EXISTS drift_events (
            id TEXT PRIMARY KEY,
            repository_id TEXT NOT NULL,
            timestamp DATETIME NOT NULL,
            severity TEXT NOT NULL CHECK(severity IN ('low', 'medium', 'high', 'critical')),
            category TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            file_path TEXT NOT NULL,
            line_number INTEGER,
            column_number INTEGER,
            ml_score REAL,
            confidence REAL,
            resolved BOOLEAN NOT NULL DEFAULT 0,
            assigned_to TEXT,
            tags TEXT NOT NULL DEFAULT '[]',
            suggestion TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (repository_id) REFERENCES repositories (id) ON DELETE CASCADE
        )
        `,
        
        // Scan results table
        `
        CREATE TABLE IF NOT EXISTS scan_results (
            id TEXT PRIMARY KEY,
            repository_id TEXT NOT NULL,
            timestamp DATETIME NOT NULL,
            total_events INTEGER NOT NULL DEFAULT 0,
            critical_events INTEGER NOT NULL DEFAULT 0,
            high_events INTEGER NOT NULL DEFAULT 0,
            medium_events INTEGER NOT NULL DEFAULT 0,
            low_events INTEGER NOT NULL DEFAULT 0,
            scan_duration INTEGER NOT NULL DEFAULT 0,
            success BOOLEAN NOT NULL DEFAULT 1,
            error TEXT,
            options TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (repository_id) REFERENCES repositories (id) ON DELETE CASCADE
        )
        `,
        
        // Users table
        `
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('admin', 'user', 'viewer')),
            preferences TEXT NOT NULL DEFAULT '{}',
            last_active DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        `,
        
        // Alerts table
        `
        CREATE TABLE IF NOT EXISTS alerts (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL CHECK(type IN ('drift_detected', 'health_degraded', 'scan_failed', 'threshold_exceeded')),
            severity TEXT NOT NULL CHECK(severity IN ('info', 'warning', 'error', 'critical')),
            title TEXT NOT NULL,
            message TEXT NOT NULL,
            repository TEXT NOT NULL,
            acknowledged BOOLEAN NOT NULL DEFAULT 0,
            acknowledged_by TEXT,
            acknowledged_at DATETIME,
            metadata TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        `,
        
        // Sessions table for WebSocket tracking
        `
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            client_id TEXT NOT NULL,
            connected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            disconnected_at DATETIME,
            subscriptions TEXT NOT NULL DEFAULT '[]',
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL
        )
        `
    ];
    
    for (const sql of tables) {
        await runQuery(sql);
    }
    
    // Create indexes for better performance
    const indexes = [
        'CREATE INDEX IF NOT EXISTS idx_drift_events_repository_id ON drift_events (repository_id)',
        'CREATE INDEX IF NOT EXISTS idx_drift_events_timestamp ON drift_events (timestamp)',
        'CREATE INDEX IF NOT EXISTS idx_drift_events_severity ON drift_events (severity)',
        'CREATE INDEX IF NOT EXISTS idx_drift_events_resolved ON drift_events (resolved)',
        'CREATE INDEX IF NOT EXISTS idx_scan_results_repository_id ON scan_results (repository_id)',
        'CREATE INDEX IF NOT EXISTS idx_scan_results_timestamp ON scan_results (timestamp)',
        'CREATE INDEX IF NOT EXISTS idx_alerts_type ON alerts (type)',
        'CREATE INDEX IF NOT EXISTS idx_alerts_severity ON alerts (severity)',
        'CREATE INDEX IF NOT EXISTS idx_alerts_acknowledged ON alerts (acknowledged)',
        'CREATE INDEX IF NOT EXISTS idx_repositories_path ON repositories (path)',
        'CREATE INDEX IF NOT EXISTS idx_repositories_is_active ON repositories (is_active)'
    ];
    
    for (const sql of indexes) {
        await runQuery(sql);
    }
    
    logger.info('Database tables and indexes created successfully');
}

/**
 * Run database migrations
 */
async function runMigrations(): Promise<void> {
    try {
        // Create migrations table if it doesn't exist
        await runQuery(`
            CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                executed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        `);
        
        // Get executed migrations
        const executedMigrations = await getAllQuery<{ name: string }>(
            'SELECT name FROM migrations ORDER BY id'
        );
        const executedNames = new Set(executedMigrations.map(m => m.name));
        
        // Define available migrations
        const availableMigrations: { name: string; sql: string }[] = [
            // Add future migrations here
            // { name: '001_add_ml_features', sql: 'ALTER TABLE ...' }
        ];
        
        // Execute pending migrations
        for (const migration of availableMigrations) {
            if (!executedNames.has(migration.name)) {
                logger.info(`Running migration: ${migration.name}`);
                await runQuery(migration.sql);
                await runQuery(
                    'INSERT INTO migrations (name) VALUES (?)',
                    [migration.name]
                );
                logger.info(`Migration completed: ${migration.name}`);
            }
        }
        
        logger.info('Database migrations completed');
        
    } catch (error) {
        logger.error('Migration failed:', error);
        throw error;
    }
}

/**
 * Create a database transaction
 */
export function createTransaction(): Promise<{
    run: (sql: string, params?: any[]) => Promise<sqlite3.RunResult>,
    get: <T = any>(sql: string, params?: any[]) => Promise<T | undefined>,
    all: <T = any>(sql: string, params?: any[]) => Promise<T[]>,
    commit: () => Promise<void>,
    rollback: () => Promise<void>
}> {
    return new Promise((resolve, reject) => {
        if (!db) {
            reject(new Error('Database not initialized'));
            return;
        }
        
        db.serialize(() => {
            db!.run('BEGIN TRANSACTION', (err) => {
                if (err) {
                    reject(err);
                    return;
                }
                
                resolve({
                    run: (sql: string, params: any[] = []) => runQuery(sql, params),
                    get: <T = any>(sql: string, params: any[] = []) => getQuery<T>(sql, params),
                    all: <T = any>(sql: string, params: any[] = []) => getAllQuery<T>(sql, params),
                    commit: () => runQuery('COMMIT').then(() => {}),
                    rollback: () => runQuery('ROLLBACK').then(() => {})
                });
            });
        });
    });
}