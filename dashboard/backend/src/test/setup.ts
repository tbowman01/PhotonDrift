import { vi } from 'vitest';
import dotenv from 'dotenv';

// Load test environment variables
dotenv.config({ path: '.env.test' });

// Mock winston logger for testing
vi.mock('../utils/logger.js', () => ({
  logger: {
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
    debug: vi.fn()
  }
}));

// Mock database initialization for tests
vi.mock('../database/init.js', () => ({
  initDatabase: vi.fn().mockResolvedValue(true),
  getDatabase: vi.fn().mockReturnValue({
    run: vi.fn(),
    get: vi.fn(),
    all: vi.fn(),
    prepare: vi.fn().mockReturnValue({
      run: vi.fn(),
      get: vi.fn(),
      all: vi.fn(),
      finalize: vi.fn()
    })
  })
}));

// Mock node-cron for scheduler tests
vi.mock('node-cron', () => ({
  schedule: vi.fn(),
  validate: vi.fn(() => true),
  destroy: vi.fn()
}));

// Set test environment variables
process.env.NODE_ENV = 'test';
process.env.PORT = '3001';
process.env.HOST = 'localhost';
process.env.FRONTEND_URL = 'http://localhost:3000';

// Global test setup
beforeEach(() => {
  vi.clearAllMocks();
});

// Cleanup after tests
afterEach(() => {
  vi.restoreAllMocks();
});