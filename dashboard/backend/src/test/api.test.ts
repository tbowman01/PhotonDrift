import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import express from 'express';
import { createTestApp, makeRequest, expectSuccessResponse, expectErrorResponse } from './utils.js';

describe('API Health Check', () => {
  let app: express.Application;

  beforeEach(() => {
    app = createTestApp();
    
    // Add health check endpoint
    app.get('/health', (req, res) => {
      res.json({ 
        status: 'healthy', 
        timestamp: new Date().toISOString(),
        version: '1.0.0'
      });
    });
  });

  it('should return health status', async () => {
    const response = await makeRequest(app)
      .get('/health')
      .expect(200);

    expectSuccessResponse(response, {
      status: 'healthy',
      version: '1.0.0'
    });
    expect(response.body.timestamp).toBeDefined();
  });

  it('should handle 404 for unknown routes', async () => {
    const response = await makeRequest(app)
      .get('/unknown-route')
      .expect(404);

    expect(response.status).toBe(404);
  });
});

describe('API Data Endpoints', () => {
  let app: express.Application;

  beforeEach(() => {
    app = createTestApp();
    
    // Mock data endpoint
    app.get('/api/data/current', (req, res) => {
      res.json({
        timestamp: Date.now(),
        value: 42.5,
        anomalyScore: 0.1,
        prediction: 43.2,
        confidence: 0.85
      });
    });

    app.get('/api/data/historical', (req, res) => {
      const count = parseInt(req.query.limit as string) || 100;
      const data = Array.from({ length: count }, (_, i) => ({
        timestamp: Date.now() - (count - i) * 60000,
        value: Math.sin(i * 0.1) * 50 + 50,
        anomalyScore: Math.random() * 0.2,
        prediction: Math.sin(i * 0.1) * 50 + 50 + Math.random() * 5,
        confidence: 0.8 + Math.random() * 0.2
      }));
      res.json(data);
    });
  });

  it('should return current photon drift data', async () => {
    const response = await makeRequest(app)
      .get('/api/data/current')
      .expect(200);

    expectSuccessResponse(response);
    expect(response.body).toHaveProperty('timestamp');
    expect(response.body).toHaveProperty('value');
    expect(response.body).toHaveProperty('anomalyScore');
    expect(response.body).toHaveProperty('prediction');
    expect(response.body).toHaveProperty('confidence');
  });

  it('should return historical data with default limit', async () => {
    const response = await makeRequest(app)
      .get('/api/data/historical')
      .expect(200);

    expectSuccessResponse(response);
    expect(Array.isArray(response.body)).toBe(true);
    expect(response.body.length).toBe(100);
    expect(response.body[0]).toHaveProperty('timestamp');
    expect(response.body[0]).toHaveProperty('value');
  });

  it('should return historical data with custom limit', async () => {
    const limit = 50;
    const response = await makeRequest(app)
      .get(`/api/data/historical?limit=${limit}`)
      .expect(200);

    expectSuccessResponse(response);
    expect(Array.isArray(response.body)).toBe(true);
    expect(response.body.length).toBe(limit);
  });
});

describe('API Error Handling', () => {
  let app: express.Application;

  beforeEach(() => {
    app = createTestApp();
    
    // Endpoint that throws an error
    app.get('/api/error', (req, res) => {
      throw new Error('Test error');
    });

    // Error handling middleware
    app.use((err: any, req: express.Request, res: express.Response, next: express.NextFunction) => {
      res.status(500).json({ 
        error: 'Internal server error',
        message: err.message
      });
    });
  });

  it('should handle server errors gracefully', async () => {
    const response = await makeRequest(app)
      .get('/api/error')
      .expect(500);

    expectErrorResponse(response, 500, 'Internal server error');
    expect(response.body.message).toBe('Test error');
  });
});