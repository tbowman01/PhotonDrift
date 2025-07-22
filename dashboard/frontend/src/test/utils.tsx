import React, { ReactElement } from 'react';
import { render, RenderOptions } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { vi } from 'vitest';
import { ThemeProvider } from '@mui/material/styles';
import { createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';

const theme = createTheme({
  palette: {
    mode: 'light',
  },
});

// Custom render function that includes providers
const AllTheProviders = ({ children }: { children: React.ReactNode }) => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {children}
    </ThemeProvider>
  );
};

const customRender = (
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>,
) => {
  return {
    user: userEvent.setup(),
    ...render(ui, { wrapper: AllTheProviders, ...options })
  };
};

export * from '@testing-library/react';
export { customRender as render };

// Mock data generators
export const mockPhotonDriftData = () => ({
  timestamp: Date.now(),
  value: Math.random() * 100,
  anomalyScore: Math.random(),
  prediction: Math.random() * 100,
  confidence: Math.random()
});

export const mockHistoricalData = (count: number = 100) => {
  return Array.from({ length: count }, (_, i) => ({
    timestamp: Date.now() - (count - i) * 60000, // 1 minute intervals
    value: Math.sin(i * 0.1) * 50 + 50 + Math.random() * 10,
    anomalyScore: Math.random(),
    prediction: Math.sin(i * 0.1) * 50 + 50 + Math.random() * 5,
    confidence: 0.8 + Math.random() * 0.2
  }));
};

export const mockWebSocketData = () => ({
  type: 'realtime-update',
  data: mockPhotonDriftData()
});

// Test helpers for async operations
export const waitForNextUpdate = () => new Promise(resolve => setTimeout(resolve, 0));

export const createMockSocket = () => ({
  on: vi.fn(),
  emit: vi.fn(),
  off: vi.fn(),
  disconnect: vi.fn(),
  connected: true
});