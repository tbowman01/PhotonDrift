import { io, Socket } from 'socket.io-client';
import { 
  ClientEvents, 
  ServerEvents, 
  DriftEvent, 
  ArchitectureHealth, 
  ScanResult, 
  Alert, 
  SystemStats 
} from '../types';
import { useStore } from '../stores/useStore';

export class WebSocketService {
  private socket: Socket<ServerEvents, ClientEvents> | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private reconnectDelay = 1000;
  private isManuallyDisconnected = false;

  constructor(private serverUrl: string = 'http://localhost:3001') {
    this.setupEventListeners();
  }

  connect(): void {
    if (this.socket?.connected) {
      console.log('WebSocket already connected');
      return;
    }

    console.log('Connecting to WebSocket server...');
    this.isManuallyDisconnected = false;

    this.socket = io(this.serverUrl, {
      transports: ['websocket', 'polling'],
      timeout: 10000,
      reconnection: true,
      reconnectionAttempts: this.maxReconnectAttempts,
      reconnectionDelay: this.reconnectDelay,
    });

    this.setupSocketListeners();
  }

  disconnect(): void {
    console.log('Disconnecting from WebSocket server...');
    this.isManuallyDisconnected = true;
    this.socket?.disconnect();
    this.updateConnectionStatus(false);
  }

  isConnected(): boolean {
    return this.socket?.connected ?? false;
  }

  // Repository subscription methods
  subscribeToRepository(repositoryId: string): void {
    if (!this.socket?.connected) {
      console.warn('Cannot subscribe - WebSocket not connected');
      return;
    }

    console.log(`Subscribing to repository: ${repositoryId}`);
    this.socket.emit('subscribe:repository', { repo: repositoryId });
  }

  unsubscribeFromRepository(repositoryId: string): void {
    if (!this.socket?.connected) {
      console.warn('Cannot unsubscribe - WebSocket not connected');
      return;
    }

    console.log(`Unsubscribing from repository: ${repositoryId}`);
    this.socket.emit('unsubscribe:repository', { repo: repositoryId });
  }

  // Request methods
  requestScan(repositoryId: string, options?: any): void {
    if (!this.socket?.connected) {
      console.warn('Cannot request scan - WebSocket not connected');
      return;
    }

    console.log(`Requesting scan for repository: ${repositoryId}`);
    this.socket.emit('request:scan', { repo: repositoryId, options });
  }

  requestHealth(repositoryId: string): void {
    if (!this.socket?.connected) {
      console.warn('Cannot request health - WebSocket not connected');
      return;
    }

    console.log(`Requesting health for repository: ${repositoryId}`);
    this.socket.emit('request:health', { repo: repositoryId });
  }

  acknowledgeAlert(alertId: string): void {
    if (!this.socket?.connected) {
      console.warn('Cannot acknowledge alert - WebSocket not connected');
      return;
    }

    console.log(`Acknowledging alert: ${alertId}`);
    this.socket.emit('acknowledge:alert', { alertId });
  }

  private setupEventListeners(): void {
    // Listen for store changes to handle repository subscriptions
    useStore.subscribe((state, prevState) => {
      if (state.selectedRepository !== prevState.selectedRepository) {
        if (prevState.selectedRepository) {
          this.unsubscribeFromRepository(prevState.selectedRepository.id);
        }
        if (state.selectedRepository) {
          this.subscribeToRepository(state.selectedRepository.id);
        }
      }
    });
  }

  private setupSocketListeners(): void {
    if (!this.socket) return;

    // Connection events
    this.socket.on('connect', () => {
      console.log('✅ Connected to WebSocket server');
      this.reconnectAttempts = 0;
      this.updateConnectionStatus(true);

      // Subscribe to current repository if one is selected
      const selectedRepo = useStore.getState().selectedRepository;
      if (selectedRepo) {
        this.subscribeToRepository(selectedRepo.id);
      }
    });

    this.socket.on('disconnect', (reason) => {
      console.log(`❌ Disconnected from WebSocket server: ${reason}`);
      this.updateConnectionStatus(false);

      if (!this.isManuallyDisconnected && reason === 'io server disconnect') {
        // Server disconnected, try to reconnect
        this.attemptReconnect();
      }
    });

    this.socket.on('connect_error', (error) => {
      console.error('❌ WebSocket connection error:', error);
      this.updateConnectionStatus(false);
      this.attemptReconnect();
    });

    // Data events
    this.socket.on('drift:detected', (event: DriftEvent) => {
      console.log('🔍 Drift detected:', event);
      useStore.getState().addDriftEvent(event);
      useStore.getState().updateLastUpdated();
      
      // Create alert for high/critical severity
      if (event.severity === 'high' || event.severity === 'critical') {
        const alert: Alert = {
          id: `drift-${event.id}`,
          type: 'drift_detected',
          severity: event.severity === 'critical' ? 'critical' : 'error',
          title: `${event.severity.toUpperCase()} drift detected`,
          message: event.title,
          repository: event.location.file.split('/')[0] || 'Unknown',
          timestamp: new Date(),
          acknowledged: false,
          metadata: { driftEventId: event.id }
        };
        useStore.getState().addAlert(alert);
      }
    });

    this.socket.on('drift:resolved', ({ id }) => {
      console.log('✅ Drift resolved:', id);
      useStore.getState().updateDriftEvent(id, { resolved: true });
      useStore.getState().updateLastUpdated();
    });

    this.socket.on('health:updated', (health: ArchitectureHealth) => {
      console.log('📊 Health updated:', health);
      useStore.getState().setArchitectureHealth(health);
      useStore.getState().updateLastUpdated();
    });

    this.socket.on('scan:progress', ({ repo, progress }) => {
      console.log(`🔄 Scan progress for ${repo}: ${progress}%`);
      // You could add a progress indicator to the store if needed
    });

    this.socket.on('scan:completed', (result: ScanResult) => {
      console.log('✅ Scan completed:', result);
      useStore.getState().setDriftEvents(result.driftEvents);
      useStore.getState().updateLastUpdated();
    });

    this.socket.on('scan:failed', ({ repo, error }) => {
      console.error(`❌ Scan failed for ${repo}:`, error);
      useStore.getState().setError(`Scan failed for ${repo}: ${error}`);
    });

    this.socket.on('alert:new', (alert: Alert) => {
      console.log('🚨 New alert:', alert);
      useStore.getState().addAlert(alert);
    });

    this.socket.on('system:stats', (stats: SystemStats) => {
      console.log('📈 System stats updated:', stats);
      useStore.getState().setSystemStats(stats);
      useStore.getState().updateLastUpdated();
    });
  }

  private updateConnectionStatus(isConnected: boolean): void {
    useStore.getState().setConnectionStatus({ 
      isConnected, 
      lastConnected: isConnected ? new Date() : null,
      reconnectAttempts: this.reconnectAttempts,
      error: null 
    });
  }

  private attemptReconnect(): void {
    if (this.isManuallyDisconnected || this.reconnectAttempts >= this.maxReconnectAttempts) {
      return;
    }

    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1); // Exponential backoff

    console.log(`🔄 Attempting reconnection ${this.reconnectAttempts}/${this.maxReconnectAttempts} in ${delay}ms`);

    setTimeout(() => {
      if (!this.isManuallyDisconnected && !this.socket?.connected) {
        this.connect();
      }
    }, delay);
  }

  // Cleanup method
  destroy(): void {
    console.log('🧹 Cleaning up WebSocket service');
    this.isManuallyDisconnected = true;
    this.socket?.disconnect();
    this.socket?.removeAllListeners();
    this.socket = null;
  }
}

// Singleton instance
export const websocketService = new WebSocketService();

// React hook for WebSocket connection management
export const useWebSocket = () => {
  const isConnected = useStore(state => state.isConnected);
  
  const connect = () => websocketService.connect();
  const disconnect = () => websocketService.disconnect();
  const subscribeToRepository = (repoId: string) => websocketService.subscribeToRepository(repoId);
  const unsubscribeFromRepository = (repoId: string) => websocketService.unsubscribeFromRepository(repoId);
  const requestScan = (repoId: string, options?: any) => websocketService.requestScan(repoId, options);
  const requestHealth = (repoId: string) => websocketService.requestHealth(repoId);
  const acknowledgeAlert = (alertId: string) => websocketService.acknowledgeAlert(alertId);

  return {
    isConnected,
    connect,
    disconnect,
    subscribeToRepository,
    unsubscribeFromRepository,
    requestScan,
    requestHealth,
    acknowledgeAlert
  };
};