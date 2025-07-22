import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { 
  AppState, 
  DashboardState, 
  DriftEvent, 
  ArchitectureHealth, 
  SystemStats, 
  Alert,
  Repository,
  DriftEventFilters,
  TimeRange,
  User,
  ConnectionStatus
} from '../types';

interface Store extends AppState, DashboardState {
  // App actions
  setUser: (user: User | null) => void;
  setSelectedRepository: (repo: Repository | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setTheme: (theme: 'light' | 'dark') => void;
  
  // Dashboard actions
  setDriftEvents: (events: DriftEvent[]) => void;
  addDriftEvent: (event: DriftEvent) => void;
  updateDriftEvent: (id: string, updates: Partial<DriftEvent>) => void;
  setArchitectureHealth: (health: ArchitectureHealth | null) => void;
  setSystemStats: (stats: SystemStats | null) => void;
  setAlerts: (alerts: Alert[]) => void;
  addAlert: (alert: Alert) => void;
  removeAlert: (alertId: string) => void;
  setConnectionStatus: (status: ConnectionStatus) => void;
  setFilters: (filters: DriftEventFilters) => void;
  setSelectedTimeRange: (range: TimeRange) => void;
  updateLastUpdated: () => void;
  
  // Computed values
  getFilteredDriftEvents: () => DriftEvent[];
  getCriticalAlertsCount: () => number;
  getUnresolvedEventsCount: () => number;
}

const defaultTimeRange: TimeRange = {
  label: 'Last 24 hours',
  hours: 24
};

const defaultFilters: DriftEventFilters = {
  severity: [],
  category: [],
  resolved: null,
  dateRange: null,
  search: ''
};

export const useStore = create<Store>()(
  persist(
    (set, get) => ({
      // App state
      user: null,
      selectedRepository: null,
      isLoading: false,
      error: null,
      theme: 'light',
      
      // Dashboard state
      driftEvents: [],
      architectureHealth: null,
      systemStats: null,
      alerts: [],
      isConnected: false,
      lastUpdated: null,
      filters: defaultFilters,
      selectedTimeRange: defaultTimeRange,
      
      // App actions
      setUser: (user) => set({ user }),
      setSelectedRepository: (repo) => set({ selectedRepository: repo }),
      setLoading: (loading) => set({ isLoading: loading }),
      setError: (error) => set({ error }),
      setTheme: (theme) => set({ theme }),
      
      // Dashboard actions
      setDriftEvents: (events) => set({ driftEvents: events }),
      addDriftEvent: (event) => set((state) => ({
        driftEvents: [event, ...state.driftEvents]
      })),
      updateDriftEvent: (id, updates) => set((state) => ({
        driftEvents: state.driftEvents.map(event =>
          event.id === id ? { ...event, ...updates } : event
        )
      })),
      setArchitectureHealth: (health) => set({ architectureHealth: health }),
      setSystemStats: (stats) => set({ systemStats: stats }),
      setAlerts: (alerts) => set({ alerts }),
      addAlert: (alert) => set((state) => ({
        alerts: [alert, ...state.alerts]
      })),
      removeAlert: (alertId) => set((state) => ({
        alerts: state.alerts.filter(alert => alert.id !== alertId)
      })),
      setConnectionStatus: (status) => set({ isConnected: status.isConnected }),
      setFilters: (filters) => set({ filters }),
      setSelectedTimeRange: (range) => set({ selectedTimeRange: range }),
      updateLastUpdated: () => set({ lastUpdated: new Date() }),
      
      // Computed values
      getFilteredDriftEvents: () => {
        const { driftEvents, filters, selectedTimeRange } = get();
        let filtered = [...driftEvents];
        
        // Filter by severity
        if (filters.severity.length > 0) {
          filtered = filtered.filter(event => 
            filters.severity.includes(event.severity)
          );
        }
        
        // Filter by category
        if (filters.category.length > 0) {
          filtered = filtered.filter(event =>
            filters.category.includes(event.category)
          );
        }
        
        // Filter by resolved status
        if (filters.resolved !== null) {
          filtered = filtered.filter(event => event.resolved === filters.resolved);
        }
        
        // Filter by search
        if (filters.search) {
          const search = filters.search.toLowerCase();
          filtered = filtered.filter(event =>
            event.title.toLowerCase().includes(search) ||
            event.description.toLowerCase().includes(search) ||
            event.category.toLowerCase().includes(search)
          );
        }
        
        // Filter by date range
        const now = new Date();
        let startDate: Date | null = null;
        
        if (filters.dateRange) {
          startDate = filters.dateRange.start;
        } else if (selectedTimeRange.hours) {
          startDate = new Date(now.getTime() - selectedTimeRange.hours * 60 * 60 * 1000);
        } else if (selectedTimeRange.days) {
          startDate = new Date(now.getTime() - selectedTimeRange.days * 24 * 60 * 60 * 1000);
        } else if (selectedTimeRange.months) {
          startDate = new Date(now.getTime() - selectedTimeRange.months * 30 * 24 * 60 * 60 * 1000);
        }
        
        if (startDate) {
          filtered = filtered.filter(event => 
            new Date(event.timestamp) >= startDate!
          );
        }
        
        return filtered.sort((a, b) => 
          new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()
        );
      },
      
      getCriticalAlertsCount: () => {
        const { alerts } = get();
        return alerts.filter(alert => 
          alert.severity === 'critical' && !alert.acknowledged
        ).length;
      },
      
      getUnresolvedEventsCount: () => {
        const { driftEvents } = get();
        return driftEvents.filter(event => !event.resolved).length;
      }
    }),
    {
      name: 'photon-drift-store',
      partialize: (state) => ({
        user: state.user,
        theme: state.theme,
        selectedRepository: state.selectedRepository,
        filters: state.filters,
        selectedTimeRange: state.selectedTimeRange
      })
    }
  )
);

// Utility hooks for commonly used computed values
export const useFilteredDriftEvents = () => useStore(state => state.getFilteredDriftEvents());
export const useCriticalAlertsCount = () => useStore(state => state.getCriticalAlertsCount());
export const useUnresolvedEventsCount = () => useStore(state => state.getUnresolvedEventsCount());

// Selector hooks for better performance
export const useUser = () => useStore(state => state.user);
export const useSelectedRepository = () => useStore(state => state.selectedRepository);
export const useTheme = () => useStore(state => state.theme);
export const useIsLoading = () => useStore(state => state.isLoading);
export const useError = () => useStore(state => state.error);
export const useIsConnected = () => useStore(state => state.isConnected);
export const useArchitectureHealth = () => useStore(state => state.architectureHealth);
export const useSystemStats = () => useStore(state => state.systemStats);
export const useAlerts = () => useStore(state => state.alerts);
export const useFilters = () => useStore(state => state.filters);
export const useSelectedTimeRange = () => useStore(state => state.selectedTimeRange);