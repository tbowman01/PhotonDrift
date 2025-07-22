import express from 'express';
import { createServer } from 'http';
import { Server as SocketServer } from 'socket.io';
import cors from 'cors';
import helmet from 'helmet';
import dotenv from 'dotenv';
import { logger } from './utils/logger.js';
import { initDatabase } from './database/init.js';
import { setupRoutes } from './api/routes.js';
import { setupWebSocket } from './websocket/handlers.js';
import { PhotonDriftService } from './services/photonDriftService.js';
import { schedulePeriodicScans } from './services/schedulerService.js';

// Load environment variables
dotenv.config();

const app = express();
const server = createServer(app);
const io = new SocketServer(server, {
    cors: {
        origin: process.env.FRONTEND_URL || "http://localhost:3000",
        methods: ["GET", "POST"]
    }
});

// Configuration
const PORT = process.env.PORT || 3001;
const HOST = process.env.HOST || 'localhost';

async function startServer() {
    try {
        // Initialize database
        await initDatabase();
        logger.info('Database initialized successfully');

        // Middleware
        app.use(helmet());
        app.use(cors({
            origin: process.env.FRONTEND_URL || "http://localhost:3000",
            credentials: true
        }));
        app.use(express.json({ limit: '10mb' }));
        app.use(express.urlencoded({ extended: true }));

        // Request logging
        app.use((req, res, next) => {
            logger.info(`${req.method} ${req.path}`, { 
                ip: req.ip,
                userAgent: req.get('User-Agent')
            });
            next();
        });

        // Health check endpoint
        app.get('/health', (req, res) => {
            res.json({ 
                status: 'healthy', 
                timestamp: new Date().toISOString(),
                version: process.env.npm_package_version || '1.0.0'
            });
        });

        // Initialize services
        const photonDriftService = new PhotonDriftService();
        
        // Setup API routes
        setupRoutes(app, { photonDriftService, io });

        // Setup WebSocket handlers
        setupWebSocket(io, { photonDriftService });

        // Setup periodic scans (if enabled)
        if (process.env.ENABLE_PERIODIC_SCANS === 'true') {
            schedulePeriodicScans(photonDriftService, io);
            logger.info('Periodic scans scheduled');
        }

        // Error handling middleware
        app.use((err: any, req: express.Request, res: express.Response, next: express.NextFunction) => {
            logger.error('Unhandled error:', err);
            res.status(500).json({ 
                error: 'Internal server error',
                message: process.env.NODE_ENV === 'development' ? err.message : 'Something went wrong'
            });
        });

        // 404 handler
        app.use('*', (req, res) => {
            res.status(404).json({ 
                error: 'Not found',
                path: req.originalUrl
            });
        });

        // Start server
        server.listen(Number(PORT), HOST, () => {
            logger.info(`PhotonDrift Dashboard API server running on http://${HOST}:${PORT}`);
            logger.info(`WebSocket server ready for connections`);
            logger.info(`Environment: ${process.env.NODE_ENV || 'development'}`);
        });

        // Graceful shutdown
        process.on('SIGTERM', () => {
            logger.info('SIGTERM received, shutting down gracefully...');
            server.close(() => {
                logger.info('Server closed');
                process.exit(0);
            });
        });

        process.on('SIGINT', () => {
            logger.info('SIGINT received, shutting down gracefully...');
            server.close(() => {
                logger.info('Server closed');
                process.exit(0);
            });
        });

    } catch (error) {
        logger.error('Failed to start server:', error);
        process.exit(1);
    }
}

startServer();