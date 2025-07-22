import * as cron from 'node-cron';
import { Server as SocketServer } from 'socket.io';
import { logger } from '../utils/logger.js';
import { PhotonDriftService } from './photonDriftService.js';
import { getAllQuery } from '../database/init.js';
import { DBRepository } from '../models/types.js';

interface ScheduledTask {
    id: string;
    repositoryId: string;
    cronExpression: string;
    task: cron.ScheduledTask;
    isRunning: boolean;
}

const scheduledTasks = new Map<string, ScheduledTask>();

/**
 * Schedule periodic scans for all active repositories
 */
export function schedulePeriodicScans(
    photonDriftService: PhotonDriftService, 
    io: SocketServer
): void {
    logger.info('Setting up periodic scan scheduler');
    
    // Schedule a job to check for repositories that need scanning every 5 minutes
    cron.schedule('*/5 * * * *', async () => {
        try {
            await checkAndScheduleScans(photonDriftService, io);
        } catch (error) {
            logger.error('Failed to check scheduled scans:', error);
        }
    });
    
    logger.info('Periodic scan scheduler initialized');
}

/**
 * Check repositories and schedule scans as needed
 */
async function checkAndScheduleScans(
    photonDriftService: PhotonDriftService,
    io: SocketServer
): Promise<void> {
    try {
        // Get all active repositories from database
        const repositories = await getAllQuery<DBRepository>(`
            SELECT * FROM repositories 
            WHERE is_active = 1 
            AND configuration IS NOT NULL
        `);
        
        for (const repo of repositories) {
            try {
                const config = JSON.parse(repo.configuration || '{}');
                
                if (config.enableAutoScan && config.scanInterval > 0) {
                    await scheduleRepositoryScan(repo, config, photonDriftService, io);
                }
                
            } catch (configError) {
                logger.error(`Invalid configuration for repository ${repo.id}:`, configError);
            }
        }
        
    } catch (error) {
        logger.error('Failed to check repositories for scheduling:', error);
    }
}

/**
 * Schedule a scan for a specific repository
 */
async function scheduleRepositoryScan(
    repository: DBRepository,
    config: any,
    photonDriftService: PhotonDriftService,
    io: SocketServer
): Promise<void> {
    const taskId = `scan_${repository.id}`;
    
    // Check if task is already scheduled
    if (scheduledTasks.has(taskId)) {
        return;
    }
    
    try {
        // Convert interval (minutes) to cron expression
        const cronExpression = minutesToCron(config.scanInterval);
        
        // Create scheduled task
        const task = cron.schedule(cronExpression, async () => {
            const taskId = `${repository.id}_${config.scanInterval}`;
            const scheduledTask = scheduledTasks.get(taskId);
            
            if (scheduledTask) {
                scheduledTask.isRunning = true;
            }
            
            logger.info(`Running scheduled scan for repository: ${repository.path}`, {
                repositoryId: repository.id,
                scanInterval: config.scanInterval
            });
            
            try {
                // Perform the scan
                const scanOptions = {
                    enableML: config.mlThreshold !== undefined,
                    confidenceThreshold: config.mlThreshold,
                    excludeFiles: config.excludePatterns || [],
                    includeFiles: config.includePatterns || [],
                    format: 'json' as const
                };
                
                const result = await photonDriftService.scanForDrift(repository.path, scanOptions);
                
                // Emit scan completion to WebSocket clients
                io.to(`repo:${repository.path}`).emit('scan:completed', result);
                
                // Send individual drift events
                result.driftEvents.forEach(event => {
                    io.to(`repo:${repository.path}`).emit('drift:detected', event);
                });
                
                // Check if we need to send alerts for high-severity drift
                const criticalEvents = result.driftEvents.filter(e => e.severity === 'critical');
                const highEvents = result.driftEvents.filter(e => e.severity === 'high');
                
                if (criticalEvents.length > 0 || highEvents.length >= (config.criticalThreshold || 5)) {
                    await sendDriftAlert(repository, result, config, io);
                }
                
                logger.info(`Scheduled scan completed for repository: ${repository.path}`, {
                    repositoryId: repository.id,
                    totalEvents: result.driftEvents.length,
                    criticalEvents: criticalEvents.length,
                    highEvents: highEvents.length
                });
                
            } catch (scanError) {
                logger.error(`Scheduled scan failed for repository: ${repository.path}`, scanError);
                
                // Emit scan failure
                io.to(`repo:${repository.path}`).emit('scan:failed', {
                    scanId: `scheduled_${Date.now()}`,
                    repo: repository.path,
                    error: scanError instanceof Error ? scanError.message : 'Unknown error'
                });
                
                // Send failure alert if notifications are enabled
                if (config.notifications?.enabled) {
                    await sendScanFailureAlert(repository, scanError, config, io);
                }
            } finally {
                if (scheduledTask) {
                    scheduledTask.isRunning = false;
                }
            }
        }, {
            scheduled: false // Don't start immediately
        });
        
        // Store the scheduled task
        scheduledTasks.set(taskId, {
            id: taskId,
            repositoryId: repository.id,
            cronExpression,
            task,
            isRunning: false
        });
        
        // Start the task
        task.start();
        
        logger.info(`Scheduled periodic scans for repository: ${repository.path}`, {
            repositoryId: repository.id,
            interval: config.scanInterval,
            cronExpression
        });
        
    } catch (error) {
        logger.error(`Failed to schedule scan for repository: ${repository.path}`, error);
    }
}

/**
 * Remove scheduled scan for a repository
 */
export function unscheduleRepositoryScan(repositoryId: string): boolean {
    const taskId = `scan_${repositoryId}`;
    const scheduledTask = scheduledTasks.get(taskId);
    
    if (scheduledTask) {
        scheduledTask.task.stop();
        scheduledTasks.delete(taskId);
        
        logger.info(`Unscheduled scan for repository: ${repositoryId}`);
        return true;
    }
    
    return false;
}

/**
 * Get all currently scheduled scans
 */
export function getScheduledScans(): Array<{
    id: string;
    repositoryId: string;
    cronExpression: string;
    isRunning: boolean;
}> {
    return Array.from(scheduledTasks.values()).map(task => ({
        id: task.id,
        repositoryId: task.repositoryId,
        cronExpression: task.cronExpression,
        isRunning: task.isRunning
    }));
}

/**
 * Stop all scheduled scans
 */
export function stopAllScheduledScans(): void {
    for (const [taskId, task] of scheduledTasks.entries()) {
        task.task.stop();
        scheduledTasks.delete(taskId);
    }
    
    logger.info('All scheduled scans stopped');
}

/**
 * Convert minutes to cron expression
 */
function minutesToCron(minutes: number): string {
    if (minutes < 60) {
        // Less than an hour: run every N minutes
        return `*/${minutes} * * * *`;
    } else if (minutes < 1440) {
        // Less than a day: run every N hours
        const hours = Math.floor(minutes / 60);
        return `0 */${hours} * * *`;
    } else {
        // Days: run daily at a specific time
        const days = Math.floor(minutes / 1440);
        return `0 9 */${days} * *`; // 9 AM every N days
    }
}

/**
 * Send drift detection alert
 */
async function sendDriftAlert(
    repository: DBRepository,
    scanResult: any,
    config: any,
    io: SocketServer
): Promise<void> {
    try {
        const alert = {
            id: `alert_${Date.now()}`,
            type: 'drift_detected' as const,
            severity: scanResult.driftEvents.some((e: any) => e.severity === 'critical') 
                ? 'critical' as const 
                : 'warning' as const,
            title: 'Architecture Drift Detected',
            message: `Found ${scanResult.summary.totalEvents} drift events in ${repository.name}`,
            repository: repository.path,
            timestamp: new Date(),
            acknowledged: false,
            metadata: {
                scanId: scanResult.id,
                totalEvents: scanResult.summary.totalEvents,
                criticalEvents: scanResult.summary.criticalEvents,
                highEvents: scanResult.summary.highEvents
            }
        };
        
        // Broadcast alert to all clients
        io.emit('alert:new', alert);
        
        // TODO: Store alert in database
        // await runQuery('INSERT INTO alerts (...) VALUES (...)', [...]);
        
        // Send email/Slack notification if configured
        if (config.notifications?.emailAlerts) {
            // TODO: Implement email notifications
            logger.info('Email alert would be sent', { repositoryId: repository.id });
        }
        
        if (config.notifications?.slackWebhook) {
            // TODO: Implement Slack notifications
            logger.info('Slack alert would be sent', { repositoryId: repository.id });
        }
        
        logger.info('Drift alert sent', { 
            repositoryId: repository.id, 
            alertId: alert.id 
        });
        
    } catch (error) {
        logger.error('Failed to send drift alert:', error);
    }
}

/**
 * Send scan failure alert
 */
async function sendScanFailureAlert(
    repository: DBRepository,
    error: any,
    config: any,
    io: SocketServer
): Promise<void> {
    try {
        const alert = {
            id: `alert_${Date.now()}`,
            type: 'scan_failed' as const,
            severity: 'error' as const,
            title: 'Scheduled Scan Failed',
            message: `Scheduled scan failed for ${repository.name}: ${error instanceof Error ? error.message : 'Unknown error'}`,
            repository: repository.path,
            timestamp: new Date(),
            acknowledged: false,
            metadata: {
                error: error instanceof Error ? error.message : String(error),
                scanType: 'scheduled'
            }
        };
        
        // Broadcast alert to all clients
        io.emit('alert:new', alert);
        
        // TODO: Store alert in database
        
        logger.info('Scan failure alert sent', { 
            repositoryId: repository.id, 
            alertId: alert.id 
        });
        
    } catch (alertError) {
        logger.error('Failed to send scan failure alert:', alertError);
    }
}

/**
 * Update scheduled scan for a repository when configuration changes
 */
export async function updateRepositorySchedule(
    repositoryId: string,
    newConfig: any,
    photonDriftService: PhotonDriftService,
    io: SocketServer
): Promise<void> {
    // Remove existing schedule
    unscheduleRepositoryScan(repositoryId);
    
    // Add new schedule if auto-scan is enabled
    if (newConfig.enableAutoScan && newConfig.scanInterval > 0) {
        try {
            const repository = await getAllQuery<DBRepository>(
                'SELECT * FROM repositories WHERE id = ? LIMIT 1',
                [repositoryId]
            );
            
            if (repository.length > 0) {
                await scheduleRepositoryScan(repository[0], newConfig, photonDriftService, io);
            }
            
        } catch (error) {
            logger.error(`Failed to update schedule for repository ${repositoryId}:`, error);
        }
    }
}