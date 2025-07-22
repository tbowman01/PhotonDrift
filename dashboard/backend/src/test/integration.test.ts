import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { WebSocketTestHelper, mockPhotonDriftData, mockHistoricalData } from './utils.js';

describe('Integration Tests - Full System', () => {
  let wsHelper: WebSocketTestHelper;

  beforeEach(async () => {
    wsHelper = new WebSocketTestHelper();
    await wsHelper.start();
  });

  afterEach(async () => {
    await wsHelper.stop();
  });

  it('should handle complete data flow from API to WebSocket', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();
    const testData = mockPhotonDriftData();

    // Simulate the complete flow
    server.on('connection', (socket) => {
      // Client subscribes to real-time updates
      socket.on('subscribe', (channel: string) => {
        if (channel === 'realtime-updates') {
          socket.join(channel);
          socket.emit('subscribed', { channel, status: 'success' });
          
          // Simulate new data arriving and being broadcast
          setTimeout(() => {
            server.to(channel).emit('data-update', testData);
          }, 100);
        }
      });
    });

    client.on('data-update', (data: any) => {
      expect(data).toEqual(testData);
      expect(data.timestamp).toBeDefined();
      expect(data.value).toBeDefined();
      expect(data.anomalyScore).toBeDefined();
      done();
    });

    client.on('subscribed', () => {
      // Subscription confirmed, data should arrive soon
    });

    client.on('connect', () => {
      client.emit('subscribe', 'realtime-updates');
    });
  });

  it('should handle multiple clients with different subscriptions', (done) => {
    const server = wsHelper.getServer();
    const client1 = wsHelper.getClient();
    const client2 = wsHelper.getClient();
    
    const realtimeData = mockPhotonDriftData();
    const historicalData = mockHistoricalData(5);
    
    let client1Received = false;
    let client2Received = false;

    const checkCompletion = () => {
      if (client1Received && client2Received) {
        done();
      }
    };

    server.on('connection', (socket) => {
      socket.on('subscribe', (channel: string) => {
        socket.join(channel);
        socket.emit('subscribed', { channel });
        
        if (channel === 'realtime-updates') {
          setTimeout(() => server.to(channel).emit('realtime-data', realtimeData), 50);
        } else if (channel === 'historical-data') {
          setTimeout(() => server.to(channel).emit('historical-data', historicalData), 50);
        }
      });
    });

    client1.on('realtime-data', (data: any) => {
      expect(data).toEqual(realtimeData);
      client1Received = true;
      checkCompletion();
    });

    client2.on('historical-data', (data: any) => {
      expect(data).toEqual(historicalData);
      expect(Array.isArray(data)).toBe(true);
      client2Received = true;
      checkCompletion();
    });

    client1.on('connect', () => {
      client1.emit('subscribe', 'realtime-updates');
    });

    client2.on('connect', () => {
      client2.emit('subscribe', 'historical-data');
    });
  });

  it('should handle error scenarios gracefully', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();

    server.on('connection', (socket) => {
      socket.on('request-data', (params: any) => {
        if (!params || !params.type) {
          socket.emit('error', { 
            message: 'Invalid request parameters',
            code: 'INVALID_PARAMS'
          });
          return;
        }

        if (params.type === 'invalid') {
          socket.emit('error', {
            message: 'Unsupported data type',
            code: 'UNSUPPORTED_TYPE'
          });
          return;
        }

        // Valid request
        socket.emit('data-response', { data: mockPhotonDriftData() });
      });
    });

    let errorCount = 0;
    const expectedErrors = 2;

    client.on('error', (error: any) => {
      errorCount++;
      expect(error.message).toBeDefined();
      expect(error.code).toBeDefined();
      
      if (errorCount === expectedErrors) {
        done();
      }
    });

    client.on('connect', () => {
      // Send invalid requests
      client.emit('request-data'); // No parameters
      client.emit('request-data', { type: 'invalid' }); // Invalid type
    });
  });
});

describe('Integration Tests - Performance', () => {
  let wsHelper: WebSocketTestHelper;

  beforeEach(async () => {
    wsHelper = new WebSocketTestHelper();
    await wsHelper.start();
  });

  afterEach(async () => {
    await wsHelper.stop();
  });

  it('should handle high-frequency data updates', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();
    const updateCount = 50;
    let receivedCount = 0;

    client.on('high-frequency-update', (data: any) => {
      receivedCount++;
      expect(data.sequenceNumber).toBeDefined();
      
      if (receivedCount === updateCount) {
        done();
      }
    });

    server.on('connection', (socket) => {
      socket.on('start-high-frequency', () => {
        for (let i = 0; i < updateCount; i++) {
          setTimeout(() => {
            socket.emit('high-frequency-update', {
              ...mockPhotonDriftData(),
              sequenceNumber: i
            });
          }, i * 10); // 10ms intervals
        }
      });
    });

    client.on('connect', () => {
      client.emit('start-high-frequency');
    });
  }, 10000); // 10 second timeout for performance test

  it('should handle large data payloads', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();
    const largeDataset = mockHistoricalData(1000); // 1000 data points

    client.on('large-dataset', (data: any) => {
      expect(Array.isArray(data)).toBe(true);
      expect(data.length).toBe(1000);
      expect(data[0]).toHaveProperty('timestamp');
      expect(data[0]).toHaveProperty('value');
      done();
    });

    server.on('connection', (socket) => {
      socket.on('request-large-dataset', () => {
        socket.emit('large-dataset', largeDataset);
      });
    });

    client.on('connect', () => {
      client.emit('request-large-dataset');
    });
  }, 5000); // 5 second timeout for large data test
});