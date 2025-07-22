import express from 'express';
import request from 'supertest';
import { Server } from 'socket.io';
import { createServer } from 'http';
import { AddressInfo } from 'net';
import Client from 'socket.io-client';

// Test app factory
export const createTestApp = () => {
  const app = express();
  app.use(express.json());
  return app;
};

// Test server with WebSocket factory
export const createTestServer = () => {
  const app = createTestApp();
  const server = createServer(app);
  const io = new Server(server);
  
  return { app, server, io };
};

// WebSocket test helper
export class WebSocketTestHelper {
  private server: any;
  private io: Server;
  private client: any;
  private port: number;

  constructor() {
    const { server, io } = createTestServer();
    this.server = server;
    this.io = io;
    this.port = 0;
  }

  async start(): Promise<void> {
    return new Promise((resolve) => {
      this.server.listen(0, () => {
        this.port = (this.server.address() as AddressInfo).port;
        this.client = Client(`http://localhost:${this.port}`);
        resolve();
      });
    });
  }

  async stop(): Promise<void> {
    return new Promise((resolve) => {
      this.client?.disconnect();
      this.server.close(() => {
        resolve();
      });
    });
  }

  getClient() {
    return this.client;
  }

  getServer() {
    return this.io;
  }

  getApp() {
    return this.server;
  }
}

// Mock data generators
export const mockPhotonDriftData = () => ({
  id: Math.random().toString(36).substr(2, 9),
  timestamp: Date.now(),
  value: Math.random() * 100,
  anomalyScore: Math.random(),
  prediction: Math.random() * 100,
  confidence: Math.random(),
  metadata: {
    sensor: 'test-sensor',
    location: 'test-lab'
  }
});

export const mockHistoricalData = (count: number = 100) => {
  return Array.from({ length: count }, (_, i) => ({
    ...mockPhotonDriftData(),
    timestamp: Date.now() - (count - i) * 60000 // 1 minute intervals
  }));
};

// Database test helpers
export const createMockDatabase = () => ({
  run: vi.fn().mockResolvedValue({ changes: 1, lastID: 1 }),
  get: vi.fn().mockResolvedValue(mockPhotonDriftData()),
  all: vi.fn().mockResolvedValue(mockHistoricalData(10)),
  prepare: vi.fn().mockReturnValue({
    run: vi.fn().mockResolvedValue({ changes: 1, lastID: 1 }),
    get: vi.fn().mockResolvedValue(mockPhotonDriftData()),
    all: vi.fn().mockResolvedValue(mockHistoricalData(10)),
    finalize: vi.fn()
  })
});

// API test helpers
export const makeRequest = (app: express.Application) => request(app);

export const expectSuccessResponse = (response: request.Response, expectedData?: any) => {
  expect(response.status).toBe(200);
  expect(response.body).toBeDefined();
  if (expectedData) {
    expect(response.body).toMatchObject(expectedData);
  }
};

export const expectErrorResponse = (response: request.Response, expectedStatus: number, expectedMessage?: string) => {
  expect(response.status).toBe(expectedStatus);
  expect(response.body.error).toBeDefined();
  if (expectedMessage) {
    expect(response.body.error).toContain(expectedMessage);
  }
};