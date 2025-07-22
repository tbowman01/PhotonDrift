import * as winston from 'winston';

const { combine, timestamp, colorize, printf, json, errors } = winston.format;

// Custom log format for console output
const consoleFormat = printf(({ level, message, timestamp, ...meta }) => {
    const metaStr = Object.keys(meta).length ? JSON.stringify(meta, null, 2) : '';
    return `${timestamp} [${level}]: ${message} ${metaStr}`;
});

// Create logger instance
export const logger = winston.createLogger({
    level: process.env.LOG_LEVEL || 'info',
    format: combine(
        timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
        errors({ stack: true }),
        json()
    ),
    defaultMeta: { service: 'photondrift-dashboard-api' },
    transports: [
        // Console transport for development
        new winston.transports.Console({
            format: combine(
                colorize(),
                timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
                consoleFormat
            )
        }),
        
        // File transport for errors
        new winston.transports.File({
            filename: 'logs/error.log',
            level: 'error',
            format: combine(
                timestamp(),
                json()
            )
        }),
        
        // File transport for all logs
        new winston.transports.File({
            filename: 'logs/combined.log',
            format: combine(
                timestamp(),
                json()
            )
        })
    ]
});

// Create logs directory if it doesn't exist
import * as fs from 'fs';
if (!fs.existsSync('logs')) {
    fs.mkdirSync('logs', { recursive: true });
}

// If we're not in production, log to the console with colorized output
if (process.env.NODE_ENV !== 'production') {
    logger.add(new winston.transports.Console({
        format: combine(
            colorize({ all: true }),
            timestamp({ format: 'HH:mm:ss' }),
            printf(({ level, message, timestamp, ...meta }) => {
                const metaStr = Object.keys(meta).length ? 
                    '\n' + JSON.stringify(meta, null, 2) : '';
                return `${timestamp} ${level}: ${message}${metaStr}`;
            })
        )
    }));
}