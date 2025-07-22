import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { WebSocketTestHelper, mockPhotonDriftData } from './utils.js';

describe('WebSocket Communication', () => {
  let wsHelper: WebSocketTestHelper;

  beforeEach(async () => {
    wsHelper = new WebSocketTestHelper();
    await wsHelper.start();
  });

  afterEach(async () => {
    await wsHelper.stop();
  });

  it('should establish WebSocket connection', (done) => {
    const client = wsHelper.getClient();
    
    client.on('connect', () => {
      expect(client.connected).toBe(true);
      done();
    });
  });

  it('should handle real-time data updates', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();
    const testData = mockPhotonDriftData();

    client.on('realtime-update', (data: any) => {
      expect(data).toEqual(testData);
      done();
    });

    client.on('connect', () => {
      // Simulate server sending real-time data
      server.emit('realtime-update', testData);
    });
  });

  it('should handle client subscription requests', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();

    server.on('connection', (socket) => {
      socket.on('subscribe', (channel: string) => {
        expect(channel).toBe('photon-drift-data');
        socket.emit('subscribed', { channel, status: 'success' });
      });
    });

    client.on('subscribed', (response: any) => {
      expect(response.channel).toBe('photon-drift-data');
      expect(response.status).toBe('success');
      done();
    });

    client.on('connect', () => {
      client.emit('subscribe', 'photon-drift-data');
    });
  });

  it('should handle client disconnection gracefully', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();

    server.on('connection', (socket) => {
      socket.on('disconnect', (reason) => {
        expect(reason).toBeDefined();
        done();
      });
    });

    client.on('connect', () => {
      client.disconnect();
    });
  });

  it('should broadcast data to multiple clients', (done) => {
    const server = wsHelper.getServer();
    const client1 = wsHelper.getClient();
    
    // Create second client
    const client2 = wsHelper.getClient();
    const testData = mockPhotonDriftData();
    let receivedCount = 0;

    const handleData = (data: any) => {
      expect(data).toEqual(testData);
      receivedCount++;
      
      if (receivedCount === 2) {
        done();
      }
    };

    client1.on('broadcast', handleData);
    client2.on('broadcast', handleData);

    client1.on('connect', () => {
      client2.on('connect', () => {
        // Broadcast to all clients
        server.emit('broadcast', testData);
      });
    });
  });
});

describe('WebSocket Error Handling', () => {
  let wsHelper: WebSocketTestHelper;

  beforeEach(async () => {
    wsHelper = new WebSocketTestHelper();
    await wsHelper.start();
  });

  afterEach(async () => {
    await wsHelper.stop();
  });

  it('should handle invalid message format', (done) => {
    const server = wsHelper.getServer();
    const client = wsHelper.getClient();

    server.on('connection', (socket) => {
      socket.on('invalid-message', () => {
        socket.emit('error', { message: 'Invalid message format' });
      });
    });

    client.on('error', (error: any) => {
      expect(error.message).toBe('Invalid message format');
      done();
    });

    client.on('connect', () => {
      client.emit('invalid-message', 'not-json');
    });
  });

  it('should handle connection timeout', (done) => {
    const client = wsHelper.getClient();
    
    client.on('connect_error', (error: any) => {
      expect(error).toBeDefined();
      done();
    });

    // Simulate connection to non-existent server
    const invalidClient = require('socket.io-client')('http://localhost:9999');
    invalidClient.on('connect_error', done);
  });
});