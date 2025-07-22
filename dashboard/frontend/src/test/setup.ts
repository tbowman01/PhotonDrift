import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock Socket.IO client
vi.mock('socket.io-client', () => ({
  io: vi.fn(() => ({
    on: vi.fn(),
    emit: vi.fn(),
    off: vi.fn(),
    disconnect: vi.fn(),
    connected: true
  }))
}));

// Mock D3 for chart testing
vi.mock('d3', () => ({
  select: vi.fn(() => ({
    append: vi.fn(() => ({
      attr: vi.fn(),
      style: vi.fn(),
      text: vi.fn()
    })),
    selectAll: vi.fn(),
    data: vi.fn(),
    enter: vi.fn(),
    exit: vi.fn()
  })),
  scaleLinear: vi.fn(() => ({
    domain: vi.fn(),
    range: vi.fn()
  })),
  axisBottom: vi.fn(),
  axisLeft: vi.fn()
}));

// Mock window.matchMedia for MUI components
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(), // deprecated
    removeListener: vi.fn(), // deprecated
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

// Mock ResizeObserver for chart components
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

// Setup global test environment
beforeEach(() => {
  // Clear all mocks before each test
  vi.clearAllMocks();
});