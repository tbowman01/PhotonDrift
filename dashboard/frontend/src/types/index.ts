// Frontend-specific types for PhotonDrift Dashboard
import { 
  DriftEvent, 
  ScanResult, 
  ArchitectureHealth, 
  SystemStats,
  Alert,
  Repository
} from '../../../backend/src/models/types';

// Re-export backend types
export * from '../../../backend/src/models/types';

// Frontend-specific state types
export interface AppState {
  user: User | null;
  selectedRepository: Repository | null;
  isLoading: boolean;
  error: string | null;
  theme: 'light' | 'dark';
}

export interface DashboardState {
  driftEvents: DriftEvent[];
  architectureHealth: ArchitectureHealth | null;
  systemStats: SystemStats | null;
  alerts: Alert[];
  isConnected: boolean;
  lastUpdated: Date | null;
  filters: DriftEventFilters;
  selectedTimeRange: TimeRange;
}

export interface DriftEventFilters {
  severity: string[];
  category: string[];
  resolved: boolean | null;
  dateRange: DateRange | null;
  search: string;
}

export interface TimeRange {
  label: string;
  hours?: number;
  days?: number;
  months?: number;
}

export interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'user' | 'viewer';
  preferences: UserPreferences;
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'auto';
  notifications: boolean;
  defaultView: string;
  refreshInterval: number;
  chartColors: string[];
}

export interface DateRange {
  start: Date;
  end: Date;
}

// Chart data types
export interface TimeSeriesData {
  timestamp: Date;
  value: number;
  category?: string;
  severity?: string;
}

export interface ChartData {
  labels: string[];
  datasets: ChartDataset[];
}

export interface ChartDataset {
  label: string;
  data: number[];
  backgroundColor?: string;
  borderColor?: string;
  fill?: boolean;
}

// Component prop types
export interface DriftTimelineProps {
  events: DriftEvent[];
  height?: number;
  onEventClick?: (event: DriftEvent) => void;
  selectedTimeRange: TimeRange;
  isLoading?: boolean;
}

export interface ArchitectureHealthProps {
  health: ArchitectureHealth;
  isLoading?: boolean;
  showTrends?: boolean;
}

export interface MetricsCardsProps {
  stats: SystemStats;
  isLoading?: boolean;
}

export interface DriftEventListProps {
  events: DriftEvent[];
  onEventClick?: (event: DriftEvent) => void;
  onResolve?: (eventId: string) => void;
  filters: DriftEventFilters;
  onFiltersChange: (filters: DriftEventFilters) => void;
  isLoading?: boolean;
}

// WebSocket types
export interface WebSocketMessage {
  type: string;
  data: any;
  timestamp: Date;
}

export interface ConnectionStatus {
  isConnected: boolean;
  lastConnected: Date | null;
  reconnectAttempts: number;
  error: string | null;
}

// API types
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
  timestamp: Date;
}

export interface PaginatedResponse<T> extends ApiResponse<T[]> {
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

// UI Component types
export interface LoadingState {
  isLoading: boolean;
  message?: string;
}

export interface ErrorState {
  hasError: boolean;
  message: string;
  code?: string;
}

export interface NotificationMessage {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  timestamp: Date;
  autoHide?: boolean;
  duration?: number;
}

// Table types for DriftEventList
export interface TableColumn {
  id: keyof DriftEvent;
  label: string;
  width?: number;
  sortable?: boolean;
  filterable?: boolean;
  render?: (value: any, row: DriftEvent) => React.ReactNode;
}

export interface SortConfig {
  field: keyof DriftEvent;
  direction: 'asc' | 'desc';
}

// Theme types
export interface ThemeColors {
  primary: string;
  secondary: string;
  success: string;
  warning: string;
  error: string;
  info: string;
  background: {
    default: string;
    paper: string;
  };
  text: {
    primary: string;
    secondary: string;
  };
}

// Dashboard layout types
export interface WidgetConfig {
  id: string;
  type: 'drift-timeline' | 'health-score' | 'metrics-cards' | 'event-list';
  title: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  props: any;
}

export interface DashboardLayout {
  widgets: WidgetConfig[];
  columns: number;
  rowHeight: number;
  margin: [number, number];
  containerPadding: [number, number];
}

// Form types
export interface FormField {
  name: string;
  label: string;
  type: 'text' | 'email' | 'password' | 'select' | 'multiselect' | 'date' | 'number';
  required?: boolean;
  validation?: (value: any) => string | null;
  options?: { label: string; value: any }[];
}

export interface FormData {
  [key: string]: any;
}

// Real-time update types
export interface LiveUpdate {
  type: 'drift_detected' | 'health_updated' | 'scan_completed' | 'alert_created';
  data: any;
  timestamp: Date;
  repository?: string;
}

// Export utility types
export type Severity = 'low' | 'medium' | 'high' | 'critical';
export type AlertType = 'drift_detected' | 'health_degraded' | 'scan_failed' | 'threshold_exceeded';
export type UserRole = 'admin' | 'user' | 'viewer';
export type Theme = 'light' | 'dark' | 'auto';