import { Express, Request, Response } from 'express';
import { Server as SocketServer } from 'socket.io';
import { logger } from '../utils/logger.js';
import { PhotonDriftService } from '../services/photonDriftService.js';
import { ScanOptions, APIResponse, DriftEvent, ArchitectureHealth } from '../models/types.js';
import { z } from 'zod';

// Validation schemas
const ScanRequestSchema = z.object({
    repositoryPath: z.string().min(1),
    options: z.object({
        format: z.enum(['json', 'yaml', 'csv']).optional(),
        enableML: z.boolean().optional(),
        confidenceThreshold: z.number().min(0).max(1).optional(),
        includeFiles: z.array(z.string()).optional(),
        excludeFiles: z.array(z.string()).optional(),
        verbose: z.boolean().optional()
    }).optional()
});

const HealthRequestSchema = z.object({
    repositoryPath: z.string().min(1)
});

interface ServiceDependencies {
    photonDriftService: PhotonDriftService;
    io: SocketServer;
}

export function setupRoutes(app: Express, deps: ServiceDependencies) {
    const { photonDriftService, io } = deps;

    // GET /api/health - API health check
    app.get('/api/health', (req: Request, res: Response) => {
        res.json({
            success: true,
            data: {
                status: 'healthy',
                timestamp: new Date(),
                version: process.env.npm_package_version || '1.0.0',
                uptime: process.uptime()
            }
        } as APIResponse);
    });

    // POST /api/scan - Trigger PhotonDrift CLI scan
    app.post('/api/scan', async (req: Request, res: Response) => {
        try {
            const { repositoryPath, options } = ScanRequestSchema.parse(req.body);
            
            logger.info('Scan request received', { repositoryPath, options });
            
            // Start scan asynchronously
            const scanPromise = photonDriftService.scanForDrift(repositoryPath, options || {});
            
            // Set up progress tracking
            const scanId = `scan_${Date.now()}`;
            
            // Emit scan started event
            io.emit('scan:started', { scanId, repositoryPath });
            
            // Handle scan completion
            scanPromise.then(result => {
                io.emit('scan:completed', result);
                
                // Emit drift events individually
                result.driftEvents.forEach(event => {
                    io.emit('drift:detected', event);
                });
            }).catch(error => {
                logger.error('Scan failed:', error);
                io.emit('scan:failed', { 
                    scanId, 
                    repositoryPath, 
                    error: error.message 
                });
            });
            
            res.json({
                success: true,
                data: { 
                    scanId, 
                    message: 'Scan started',
                    repositoryPath 
                },
                timestamp: new Date()
            } as APIResponse);
            
        } catch (error) {
            logger.error('Scan request validation failed:', error);
            
            res.status(400).json({
                success: false,
                error: 'Invalid request',
                message: error instanceof z.ZodError 
                    ? error.errors.map(e => e.message).join(', ')
                    : 'Invalid scan request parameters',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // GET /api/drift/events - Get drift events
    app.get('/api/drift/events', async (req: Request, res: Response) => {
        try {
            const { repo, severity, resolved, limit = '50', offset = '0' } = req.query;
            
            // TODO: Implement database queries when DB layer is ready
            // For now, return mock data structure
            const mockEvents: DriftEvent[] = [];
            
            res.json({
                success: true,
                data: mockEvents,
                pagination: {
                    limit: parseInt(limit as string),
                    offset: parseInt(offset as string),
                    total: mockEvents.length
                },
                timestamp: new Date()
            } as APIResponse);
            
        } catch (error) {
            logger.error('Failed to fetch drift events:', error);
            
            res.status(500).json({
                success: false,
                error: 'Failed to fetch drift events',
                message: 'Internal server error',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // GET /api/health/:repo - Get repository health metrics
    app.get('/api/health/:repo(*)', async (req: Request, res: Response) => {
        try {
            const repositoryPath = req.params.repo;
            
            if (!repositoryPath) {
                res.status(400).json({
                    success: false,
                    error: 'Repository path required',
                    timestamp: new Date()
                } as APIResponse);
                return;
            }
            
            logger.info('Health check requested', { repositoryPath });
            
            const health = await photonDriftService.getArchitectureHealth(repositoryPath);
            
            // Emit health update via WebSocket
            io.emit('health:updated', health);
            
            res.json({
                success: true,
                data: health,
                timestamp: new Date()
            } as APIResponse<ArchitectureHealth>);
            
        } catch (error) {
            logger.error('Health check failed:', error);
            
            res.status(500).json({
                success: false,
                error: 'Health check failed',
                message: error instanceof Error ? error.message : 'Unknown error',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // GET /api/metrics/trends - Get trend data
    app.get('/api/metrics/trends', async (req: Request, res: Response) => {
        try {
            const { 
                repo, 
                metric = 'drift_count', 
                period = '30d',
                granularity = 'day'
            } = req.query;
            
            // TODO: Implement actual trend calculation from database
            // For now, return mock trend data
            const mockTrends = {
                metric: metric as string,
                period: period as string,
                granularity: granularity as string,
                repository: repo as string || 'all',
                dataPoints: [
                    { timestamp: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000), value: 10 },
                    { timestamp: new Date(Date.now() - 6 * 24 * 60 * 60 * 1000), value: 12 },
                    { timestamp: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000), value: 8 },
                    { timestamp: new Date(Date.now() - 4 * 24 * 60 * 60 * 1000), value: 15 },
                    { timestamp: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000), value: 11 },
                    { timestamp: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000), value: 9 },
                    { timestamp: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000), value: 13 },
                    { timestamp: new Date(), value: 7 }
                ],
                summary: {
                    current: 7,
                    previous: 10,
                    change: -3,
                    changePercent: -30,
                    trend: 'improving'
                }
            };
            
            res.json({
                success: true,
                data: mockTrends,
                timestamp: new Date()
            } as APIResponse);
            
        } catch (error) {
            logger.error('Failed to fetch trend data:', error);
            
            res.status(500).json({
                success: false,
                error: 'Failed to fetch trends',
                message: 'Internal server error',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // GET /api/repositories - List available repositories
    app.get('/api/repositories', async (req: Request, res: Response) => {
        try {
            const repositories = await photonDriftService.listRepositories();
            
            res.json({
                success: true,
                data: repositories.map(path => ({
                    id: path.replace(/[^a-zA-Z0-9]/g, '_'),
                    name: path.split('/').pop() || path,
                    path: path,
                    isActive: true
                })),
                timestamp: new Date()
            } as APIResponse);
            
        } catch (error) {
            logger.error('Failed to list repositories:', error);
            
            res.status(500).json({
                success: false,
                error: 'Failed to list repositories',
                message: error instanceof Error ? error.message : 'Unknown error',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // POST /api/repositories/:repo/init - Initialize ADR structure
    app.post('/api/repositories/:repo(*)/init', async (req: Request, res: Response) => {
        try {
            const repositoryPath = req.params.repo;
            
            if (!repositoryPath) {
                res.status(400).json({
                    success: false,
                    error: 'Repository path required',
                    timestamp: new Date()
                } as APIResponse);
                return;
            }
            
            logger.info('Repository initialization requested', { repositoryPath });
            
            const success = await photonDriftService.initializeRepository(repositoryPath);
            
            if (success) {
                res.json({
                    success: true,
                    data: { 
                        message: 'ADR structure initialized successfully',
                        repositoryPath 
                    },
                    timestamp: new Date()
                } as APIResponse);
            } else {
                res.status(500).json({
                    success: false,
                    error: 'Initialization failed',
                    message: 'Failed to initialize ADR structure',
                    timestamp: new Date()
                } as APIResponse);
            }
            
        } catch (error) {
            logger.error('Repository initialization failed:', error);
            
            res.status(500).json({
                success: false,
                error: 'Initialization failed',
                message: error instanceof Error ? error.message : 'Unknown error',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // GET /api/scans/active - Get active scan status
    app.get('/api/scans/active', (req: Request, res: Response) => {
        try {
            const activeScans = photonDriftService.getActiveScanStatus();
            
            res.json({
                success: true,
                data: {
                    count: activeScans.length,
                    scans: activeScans
                },
                timestamp: new Date()
            } as APIResponse);
            
        } catch (error) {
            logger.error('Failed to get active scans:', error);
            
            res.status(500).json({
                success: false,
                error: 'Failed to get active scans',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    // DELETE /api/scans/:scanId - Cancel active scan
    app.delete('/api/scans/:scanId', (req: Request, res: Response) => {
        try {
            const { scanId } = req.params;
            
            if (!scanId) {
                res.status(400).json({
                    success: false,
                    error: 'Scan ID required',
                    timestamp: new Date()
                } as APIResponse);
                return;
            }
            
            const cancelled = photonDriftService.cancelScan(scanId);
            
            if (cancelled) {
                io.emit('scan:cancelled', { scanId });
                
                res.json({
                    success: true,
                    data: { 
                        message: 'Scan cancelled successfully',
                        scanId 
                    },
                    timestamp: new Date()
                } as APIResponse);
            } else {
                res.status(404).json({
                    success: false,
                    error: 'Scan not found',
                    message: 'No active scan found with the provided ID',
                    timestamp: new Date()
                } as APIResponse);
            }
            
        } catch (error) {
            logger.error('Failed to cancel scan:', error);
            
            res.status(500).json({
                success: false,
                error: 'Failed to cancel scan',
                timestamp: new Date()
            } as APIResponse);
        }
    });

    logger.info('API routes configured successfully');
}