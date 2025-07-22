import { Server as SocketServer, Socket } from 'socket.io';
import { logger } from '../utils/logger.js';
import { PhotonDriftService } from '../services/photonDriftService.js';
import { ClientEvents, ServerEvents, ScanOptions } from '../models/types.js';

interface ServiceDependencies {
    photonDriftService: PhotonDriftService;
}

// Track connected clients and their subscriptions
const clientSubscriptions = new Map<string, Set<string>>();

export function setupWebSocket(io: SocketServer, deps: ServiceDependencies) {
    const { photonDriftService } = deps;
    
    // Set up service event listeners
    setupServiceListeners(photonDriftService, io);
    
    io.on('connection', (socket: Socket) => {
        const clientId = socket.id;
        logger.info(`Client connected: ${clientId}`);
        
        // Initialize client subscriptions
        clientSubscriptions.set(clientId, new Set());
        
        // Send welcome message with system status
        socket.emit('system:welcome', {
            clientId,
            timestamp: new Date(),
            activeScans: photonDriftService.getActiveScanStatus().length
        });
        
        // Handle repository subscription
        socket.on('subscribe:repository', (data: { repo: string }) => {
            try {
                const { repo } = data;
                if (!repo) {
                    socket.emit('error', { message: 'Repository path required' });
                    return;
                }
                
                const subscriptions = clientSubscriptions.get(clientId);
                if (subscriptions) {
                    subscriptions.add(repo);
                    socket.join(`repo:${repo}`);
                    
                    logger.info(`Client ${clientId} subscribed to repository: ${repo}`);
                    socket.emit('subscription:confirmed', { repo, type: 'repository' });
                }
                
            } catch (error) {
                logger.error(`Subscription error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to subscribe to repository' });
            }
        });
        
        // Handle repository unsubscription
        socket.on('unsubscribe:repository', (data: { repo: string }) => {
            try {
                const { repo } = data;
                if (!repo) {
                    socket.emit('error', { message: 'Repository path required' });
                    return;
                }
                
                const subscriptions = clientSubscriptions.get(clientId);
                if (subscriptions) {
                    subscriptions.delete(repo);
                    socket.leave(`repo:${repo}`);
                    
                    logger.info(`Client ${clientId} unsubscribed from repository: ${repo}`);
                    socket.emit('subscription:removed', { repo, type: 'repository' });
                }
                
            } catch (error) {
                logger.error(`Unsubscription error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to unsubscribe from repository' });
            }
        });
        
        // Handle scan requests
        socket.on('request:scan', async (data: { repo: string; options?: ScanOptions }) => {
            try {
                const { repo, options } = data;
                if (!repo) {
                    socket.emit('error', { message: 'Repository path required' });
                    return;
                }
                
                logger.info(`Scan requested by client ${clientId}`, { repo, options });
                
                // Start scan and send immediate response
                const scanId = `ws_scan_${Date.now()}_${clientId}`;
                socket.emit('scan:started', { scanId, repo });
                
                // Perform scan asynchronously
                try {
                    const result = await photonDriftService.scanForDrift(repo, options || {});
                    
                    // Send results to requesting client and subscribers
                    const room = `repo:${repo}`;
                    io.to(room).emit('scan:completed', result);
                    
                    // Send individual drift events
                    result.driftEvents.forEach((event: any) => {
                        io.to(room).emit('drift:detected', event);
                    });
                    
                } catch (scanError) {
                    logger.error(`Scan failed for client ${clientId}:`, scanError);
                    socket.emit('scan:failed', {
                        scanId,
                        repo,
                        error: scanError instanceof Error ? scanError.message : 'Unknown error'
                    });
                }
                
            } catch (error) {
                logger.error(`Scan request error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to process scan request' });
            }
        });
        
        // Handle health check requests
        socket.on('request:health', async (data: { repo: string }) => {
            try {
                const { repo } = data;
                if (!repo) {
                    socket.emit('error', { message: 'Repository path required' });
                    return;
                }
                
                logger.info(`Health check requested by client ${clientId}`, { repo });
                
                const health = await photonDriftService.getArchitectureHealth(repo);
                
                // Send to requesting client and repository subscribers
                const room = `repo:${repo}`;
                io.to(room).emit('health:updated', health);
                
            } catch (error) {
                logger.error(`Health check error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to get health metrics' });
            }
        });
        
        // Handle alert acknowledgment
        socket.on('acknowledge:alert', (data: { alertId: string }) => {
            try {
                const { alertId } = data;
                if (!alertId) {
                    socket.emit('error', { message: 'Alert ID required' });
                    return;
                }
                
                logger.info(`Alert acknowledged by client ${clientId}`, { alertId });
                
                // TODO: Update alert status in database
                // For now, just broadcast the acknowledgment
                io.emit('alert:acknowledged', { 
                    alertId, 
                    acknowledgedBy: clientId, 
                    acknowledgedAt: new Date() 
                });
                
            } catch (error) {
                logger.error(`Alert acknowledgment error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to acknowledge alert' });
            }
        });
        
        // Handle client status requests
        socket.on('request:status', () => {
            try {
                const subscriptions = clientSubscriptions.get(clientId) || new Set();
                const activeScans = photonDriftService.getActiveScanStatus();
                
                socket.emit('status:update', {
                    clientId,
                    connectedAt: new Date(), // TODO: Track actual connection time
                    subscriptions: Array.from(subscriptions),
                    activeScans: activeScans.length,
                    serverUptime: process.uptime()
                });
                
            } catch (error) {
                logger.error(`Status request error for client ${clientId}:`, error);
                socket.emit('error', { message: 'Failed to get status' });
            }
        });
        
        // Handle disconnection
        socket.on('disconnect', (reason) => {
            logger.info(`Client disconnected: ${clientId}`, { reason });
            
            // Clean up client subscriptions
            const subscriptions = clientSubscriptions.get(clientId);
            if (subscriptions) {
                subscriptions.forEach(repo => {
                    socket.leave(`repo:${repo}`);
                });
                clientSubscriptions.delete(clientId);
            }
        });
        
        // Handle connection errors
        socket.on('error', (error) => {
            logger.error(`Socket error for client ${clientId}:`, error);
        });
    });
    
    // Set up periodic system stats broadcast
    setInterval(() => {
        const stats = {
            timestamp: new Date(),
            connectedClients: io.engine.clientsCount,
            activeScans: photonDriftService.getActiveScanStatus().length,
            uptime: process.uptime(),
            memoryUsage: process.memoryUsage(),
            // TODO: Add more system metrics
        };
        
        io.emit('system:stats', stats);
    }, 30000); // Every 30 seconds
    
    logger.info('WebSocket handlers configured successfully');
}

// Set up listeners for PhotonDrift service events
function setupServiceListeners(service: PhotonDriftService, io: SocketServer) {
    // Listen for scan completion events
    service.on('scan:completed', (result) => {
        logger.info('Broadcasting scan completion', { scanId: result.id });
        
        // Broadcast to all clients subscribed to this repository
        const room = `repo:${result.repositoryPath}`;
        io.to(room).emit('scan:completed', result);
        
        // Also broadcast individual drift events
        result.driftEvents.forEach((event: any) => {
            io.to(room).emit('drift:detected', event);
        });
    });
    
    // Listen for scan failure events
    service.on('scan:failed', (result) => {
        logger.info('Broadcasting scan failure', { scanId: result.id });
        
        const room = `repo:${result.repositoryPath}`;
        io.to(room).emit('scan:failed', {
            scanId: result.id,
            repo: result.repositoryPath,
            error: result.error
        });
    });
    
    // Listen for any additional service events
    service.on('health:updated', (health) => {
        const room = `repo:${health.repository}`;
        io.to(room).emit('health:updated', health);
    });
    
    logger.info('Service event listeners configured');
}

// Utility function to broadcast to all clients
export function broadcastToAll(io: SocketServer, event: string, data: any) {
    io.emit(event, data);
    logger.debug(`Broadcasted event: ${event}`, { clientCount: io.engine.clientsCount });
}

// Utility function to broadcast to repository subscribers
export function broadcastToRepository(io: SocketServer, repo: string, event: string, data: any) {
    const room = `repo:${repo}`;
    io.to(room).emit(event, data);
    logger.debug(`Broadcasted event to repository: ${event}`, { repo });
}

// Get current client statistics
export function getClientStats(io: SocketServer) {
    return {
        connectedClients: io.engine.clientsCount,
        totalSubscriptions: Array.from(clientSubscriptions.values())
            .reduce((total, subs) => total + subs.size, 0),
        repositorySubscriptions: Array.from(clientSubscriptions.values())
            .flatMap(subs => Array.from(subs))
            .reduce((acc, repo) => {
                acc[repo] = (acc[repo] || 0) + 1;
                return acc;
            }, {} as Record<string, number>)
    };
}