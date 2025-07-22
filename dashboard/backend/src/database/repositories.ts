import { runQuery, getQuery, getAllQuery, createTransaction } from './init.js';
import { logger } from '../utils/logger.js';
import { 
    Repository, 
    DriftEvent, 
    ScanResult, 
    User, 
    Alert,
    DBRepository,
    DBDriftEvent,
    DBScanResult,
    DBUser
} from '../models/types.js';

/**
 * Repository data access layer
 */
export class RepositoryDAO {
    
    async create(repository: Omit<Repository, 'id'>): Promise<string> {
        const id = `repo_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        
        await runQuery(`
            INSERT INTO repositories (
                id, name, path, description, is_active, configuration, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        `, [
            id,
            repository.name,
            repository.path,
            repository.description || null,
            repository.isActive ? 1 : 0,
            JSON.stringify(repository.configuration)
        ]);
        
        logger.info('Repository created', { id, path: repository.path });
        return id;
    }
    
    async findById(id: string): Promise<Repository | null> {
        const row = await getQuery<DBRepository>(
            'SELECT * FROM repositories WHERE id = ?',
            [id]
        );
        
        return row ? this.mapFromDB(row) : null;
    }
    
    async findByPath(path: string): Promise<Repository | null> {
        const row = await getQuery<DBRepository>(
            'SELECT * FROM repositories WHERE path = ?',
            [path]
        );
        
        return row ? this.mapFromDB(row) : null;
    }
    
    async findAll(activeOnly = true): Promise<Repository[]> {
        const sql = activeOnly 
            ? 'SELECT * FROM repositories WHERE is_active = 1 ORDER BY name'
            : 'SELECT * FROM repositories ORDER BY name';
            
        const rows = await getAllQuery<DBRepository>(sql);
        return rows.map(row => this.mapFromDB(row));
    }
    
    async update(id: string, updates: Partial<Repository>): Promise<boolean> {
        const fields: string[] = [];
        const values: any[] = [];
        
        if (updates.name !== undefined) {
            fields.push('name = ?');
            values.push(updates.name);
        }
        
        if (updates.description !== undefined) {
            fields.push('description = ?');
            values.push(updates.description);
        }
        
        if (updates.isActive !== undefined) {
            fields.push('is_active = ?');
            values.push(updates.isActive ? 1 : 0);
        }
        
        if (updates.configuration !== undefined) {
            fields.push('configuration = ?');
            values.push(JSON.stringify(updates.configuration));
        }
        
        if (updates.lastScanned !== undefined) {
            fields.push('last_scanned = ?');
            values.push(updates.lastScanned?.toISOString());
        }
        
        if (fields.length === 0) {
            return true; // Nothing to update
        }
        
        fields.push('updated_at = CURRENT_TIMESTAMP');
        values.push(id);
        
        const result = await runQuery(`
            UPDATE repositories SET ${fields.join(', ')} WHERE id = ?
        `, values);
        
        logger.info('Repository updated', { id, changes: fields.length });
        return result.changes > 0;
    }
    
    async delete(id: string): Promise<boolean> {
        const result = await runQuery('DELETE FROM repositories WHERE id = ?', [id]);
        logger.info('Repository deleted', { id });
        return result.changes > 0;
    }
    
    private mapFromDB(row: DBRepository): Repository {
        return {
            id: row.id,
            name: row.name,
            path: row.path,
            description: row.description || undefined,
            lastScanned: row.last_scanned ? new Date(row.last_scanned) : undefined,
            isActive: Boolean(row.is_active),
            configuration: JSON.parse(row.configuration || '{}'),
            health: undefined // Will be populated by separate query if needed
        };
    }
}

/**
 * Drift event data access layer
 */
export class DriftEventDAO {
    
    async create(event: DriftEvent, repositoryId: string): Promise<void> {
        await runQuery(`
            INSERT INTO drift_events (
                id, repository_id, timestamp, severity, category, title, description,
                file_path, line_number, column_number, ml_score, confidence,
                resolved, assigned_to, tags, suggestion, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        `, [
            event.id,
            repositoryId,
            event.timestamp.toISOString(),
            event.severity,
            event.category,
            event.title,
            event.description,
            event.location.file,
            event.location.line || null,
            event.location.column || null,
            event.mlScore || null,
            event.confidence || null,
            event.resolved ? 1 : 0,
            event.assignee || null,
            JSON.stringify(event.tags),
            event.suggestion || null
        ]);
        
        logger.debug('Drift event created', { id: event.id, repositoryId });
    }
    
    async findById(id: string): Promise<DriftEvent | null> {
        const row = await getQuery<DBDriftEvent>(
            'SELECT * FROM drift_events WHERE id = ?',
            [id]
        );
        
        return row ? this.mapFromDB(row) : null;
    }
    
    async findByRepository(
        repositoryId: string, 
        options: {
            severity?: string;
            resolved?: boolean;
            limit?: number;
            offset?: number;
            sortBy?: string;
            sortOrder?: 'ASC' | 'DESC';
        } = {}
    ): Promise<{ events: DriftEvent[]; total: number }> {
        const conditions = ['repository_id = ?'];
        const params: any[] = [repositoryId];
        
        if (options.severity) {
            conditions.push('severity = ?');
            params.push(options.severity);
        }
        
        if (options.resolved !== undefined) {
            conditions.push('resolved = ?');
            params.push(options.resolved ? 1 : 0);
        }
        
        const whereClause = conditions.length > 0 ? `WHERE ${conditions.join(' AND ')}` : '';
        const sortBy = options.sortBy || 'timestamp';
        const sortOrder = options.sortOrder || 'DESC';
        const limit = options.limit || 50;
        const offset = options.offset || 0;
        
        // Get total count
        const countResult = await getQuery<{ total: number }>(`
            SELECT COUNT(*) as total FROM drift_events ${whereClause}
        `, params);
        const total = countResult?.total || 0;
        
        // Get events
        const events = await getAllQuery<DBDriftEvent>(`
            SELECT * FROM drift_events ${whereClause}
            ORDER BY ${sortBy} ${sortOrder}
            LIMIT ? OFFSET ?
        `, [...params, limit, offset]);
        
        return {
            events: events.map(row => this.mapFromDB(row)),
            total
        };
    }
    
    async updateResolutionStatus(id: string, resolved: boolean, assignee?: string): Promise<boolean> {
        const result = await runQuery(`
            UPDATE drift_events 
            SET resolved = ?, assigned_to = ?, updated_at = CURRENT_TIMESTAMP 
            WHERE id = ?
        `, [resolved ? 1 : 0, assignee || null, id]);
        
        logger.info('Drift event resolution updated', { id, resolved, assignee });
        return result.changes > 0;
    }
    
    async delete(id: string): Promise<boolean> {
        const result = await runQuery('DELETE FROM drift_events WHERE id = ?', [id]);
        logger.info('Drift event deleted', { id });
        return result.changes > 0;
    }
    
    async getSummaryByRepository(repositoryId: string): Promise<{
        total: number;
        critical: number;
        high: number;
        medium: number;
        low: number;
        resolved: number;
    }> {
        const summary = await getQuery<any>(`
            SELECT 
                COUNT(*) as total,
                SUM(CASE WHEN severity = 'critical' THEN 1 ELSE 0 END) as critical,
                SUM(CASE WHEN severity = 'high' THEN 1 ELSE 0 END) as high,
                SUM(CASE WHEN severity = 'medium' THEN 1 ELSE 0 END) as medium,
                SUM(CASE WHEN severity = 'low' THEN 1 ELSE 0 END) as low,
                SUM(CASE WHEN resolved = 1 THEN 1 ELSE 0 END) as resolved
            FROM drift_events WHERE repository_id = ?
        `, [repositoryId]);
        
        return summary || { total: 0, critical: 0, high: 0, medium: 0, low: 0, resolved: 0 };
    }
    
    private mapFromDB(row: DBDriftEvent): DriftEvent {
        return {
            id: row.id,
            timestamp: new Date(row.timestamp),
            severity: row.severity as 'low' | 'medium' | 'high' | 'critical',
            category: row.category,
            title: row.title,
            description: row.description,
            location: {
                file: row.file_path,
                line: row.line_number || undefined,
                column: row.column_number || undefined
            },
            mlScore: row.ml_score || undefined,
            confidence: row.confidence || undefined,
            resolved: Boolean(row.resolved),
            assignee: row.assigned_to || undefined,
            tags: JSON.parse(row.tags || '[]'),
            suggestion: row.suggestion || undefined
        };
    }
}

/**
 * Scan result data access layer
 */
export class ScanResultDAO {
    
    async create(scanResult: ScanResult, repositoryId: string): Promise<void> {
        await runQuery(`
            INSERT INTO scan_results (
                id, repository_id, timestamp, total_events, critical_events, 
                high_events, medium_events, low_events, scan_duration, 
                success, error, options, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        `, [
            scanResult.id,
            repositoryId,
            scanResult.timestamp.toISOString(),
            scanResult.summary.totalEvents,
            scanResult.summary.criticalEvents,
            scanResult.summary.highEvents,
            scanResult.summary.mediumEvents,
            scanResult.summary.lowEvents,
            scanResult.summary.scanDuration,
            scanResult.success ? 1 : 0,
            scanResult.error || null,
            JSON.stringify(scanResult.options)
        ]);
        
        logger.debug('Scan result created', { id: scanResult.id, repositoryId });
    }
    
    async findByRepository(
        repositoryId: string,
        limit = 50,
        offset = 0
    ): Promise<{ results: DBScanResult[]; total: number }> {
        const total = await getQuery<{ count: number }>(`
            SELECT COUNT(*) as count FROM scan_results WHERE repository_id = ?
        `, [repositoryId]);
        
        const results = await getAllQuery<DBScanResult>(`
            SELECT * FROM scan_results 
            WHERE repository_id = ? 
            ORDER BY timestamp DESC 
            LIMIT ? OFFSET ?
        `, [repositoryId, limit, offset]);
        
        return {
            results,
            total: total?.count || 0
        };
    }
    
    async getLatestByRepository(repositoryId: string): Promise<DBScanResult | null> {
        const result = await getQuery<DBScanResult>(`
            SELECT * FROM scan_results 
            WHERE repository_id = ? AND success = 1
            ORDER BY timestamp DESC 
            LIMIT 1
        `, [repositoryId]);
        return result || null;
    }
}

/**
 * User data access layer
 */
export class UserDAO {
    
    async create(user: Omit<User, 'id' | 'lastActive' | 'createdAt'>, passwordHash: string): Promise<string> {
        const id = `user_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        
        await runQuery(`
            INSERT INTO users (
                id, email, name, password_hash, role, preferences, 
                last_active, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        `, [
            id,
            user.email,
            user.name,
            passwordHash,
            user.role,
            JSON.stringify(user.preferences)
        ]);
        
        logger.info('User created', { id, email: user.email });
        return id;
    }
    
    async findByEmail(email: string): Promise<(User & { passwordHash: string }) | null> {
        const row = await getQuery<DBUser>(
            'SELECT * FROM users WHERE email = ?',
            [email]
        );
        
        if (!row) return null;
        
        return {
            id: row.id,
            email: row.email,
            name: row.name,
            role: row.role as 'admin' | 'user' | 'viewer',
            preferences: JSON.parse(row.preferences || '{}'),
            lastActive: new Date(row.last_active),
            createdAt: new Date(row.created_at),
            passwordHash: row.password_hash
        };
    }
    
    async updateLastActive(id: string): Promise<void> {
        await runQuery(
            'UPDATE users SET last_active = CURRENT_TIMESTAMP WHERE id = ?',
            [id]
        );
    }
}

/**
 * Alert data access layer
 */
export class AlertDAO {
    
    async create(alert: Alert): Promise<void> {
        await runQuery(`
            INSERT INTO alerts (
                id, type, severity, title, message, repository, 
                acknowledged, acknowledged_by, acknowledged_at, metadata, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        `, [
            alert.id,
            alert.type,
            alert.severity,
            alert.title,
            alert.message,
            alert.repository,
            alert.acknowledged ? 1 : 0,
            alert.acknowledgedBy || null,
            alert.acknowledgedAt?.toISOString() || null,
            JSON.stringify(alert.metadata)
        ]);
        
        logger.debug('Alert created', { id: alert.id, type: alert.type });
    }
    
    async findUnacknowledged(limit = 50): Promise<Alert[]> {
        const rows = await getAllQuery<any>(`
            SELECT * FROM alerts 
            WHERE acknowledged = 0 
            ORDER BY created_at DESC 
            LIMIT ?
        `, [limit]);
        
        return rows.map(row => this.mapAlertFromDB(row));
    }
    
    async acknowledge(id: string, acknowledgedBy: string): Promise<boolean> {
        const result = await runQuery(`
            UPDATE alerts 
            SET acknowledged = 1, acknowledged_by = ?, acknowledged_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
        `, [acknowledgedBy, id]);
        
        logger.info('Alert acknowledged', { id, acknowledgedBy });
        return result.changes > 0;
    }
    
    private mapAlertFromDB(row: any): Alert {
        return {
            id: row.id,
            type: row.type,
            severity: row.severity,
            title: row.title,
            message: row.message,
            repository: row.repository,
            timestamp: new Date(row.created_at),
            acknowledged: Boolean(row.acknowledged),
            acknowledgedBy: row.acknowledged_by || undefined,
            acknowledgedAt: row.acknowledged_at ? new Date(row.acknowledged_at) : undefined,
            metadata: JSON.parse(row.metadata || '{}')
        };
    }
}

// Export instances for use throughout the application
export const repositoryDAO = new RepositoryDAO();
export const driftEventDAO = new DriftEventDAO();
export const scanResultDAO = new ScanResultDAO();
export const userDAO = new UserDAO();
export const alertDAO = new AlertDAO();