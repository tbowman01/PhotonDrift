// Visualization Strategies for PhotonDrift Visual Analytics Dashboard
// Comprehensive guide for implementing effective data visualizations

export interface VisualizationStrategy {
  metric_category: string;
  recommended_charts: ChartRecommendation[];
  color_schemes: ColorSchemeStrategy;
  interaction_patterns: InteractionPattern[];
  performance_considerations: PerformanceStrategy;
  accessibility_requirements: AccessibilityStrategy;
}

export interface ChartRecommendation {
  chart_type: string;
  use_cases: string[];
  data_requirements: DataRequirement[];
  visual_encoding: VisualEncoding;
  best_practices: string[];
  common_mistakes: string[];
  implementation_complexity: 'low' | 'medium' | 'high';
  recommended_libraries: string[];
}

export interface DataRequirement {
  field_type: string;
  cardinality: 'low' | 'medium' | 'high';
  temporal: boolean;
  categorical: boolean;
  numerical: boolean;
  hierarchical: boolean;
}

export interface VisualEncoding {
  position: PositionEncoding;
  color: ColorEncoding;
  size: SizeEncoding;
  shape: ShapeEncoding;
  opacity: OpacityEncoding;
  texture: TextureEncoding;
}

export interface PositionEncoding {
  x_axis: AxisStrategy;
  y_axis: AxisStrategy;
  faceting: FacetingStrategy;
  small_multiples: boolean;
}

export interface AxisStrategy {
  scale_type: 'linear' | 'logarithmic' | 'time' | 'ordinal' | 'band';
  domain_calculation: 'auto' | 'fixed' | 'data_driven' | 'user_defined';
  tick_strategy: TickStrategy;
  label_strategy: LabelStrategy;
}

export interface TickStrategy {
  count_method: 'auto' | 'fixed' | 'data_density';
  target_count: number;
  min_spacing: number; // pixels
  format: string; // d3 format specifier
}

export interface LabelStrategy {
  rotation: number;
  truncation: 'none' | 'ellipsis' | 'word_wrap';
  max_length: number;
  show_all: boolean;
  priority_based: boolean;
}

export interface FacetingStrategy {
  enabled: boolean;
  max_facets: number;
  facet_arrangement: 'grid' | 'horizontal' | 'vertical';
  shared_axes: boolean;
  independent_scales: boolean;
}

export interface ColorEncoding {
  purpose: 'categorical' | 'sequential' | 'diverging' | 'highlight' | 'status';
  palette_name: string;
  custom_colors: string[];
  semantic_mapping: SemanticColorMapping[];
  accessibility_compliant: boolean;
}

export interface SemanticColorMapping {
  value: string | number;
  color: string;
  meaning: string;
  cultural_context: string[];
}

export interface SizeEncoding {
  enabled: boolean;
  size_range: [number, number]; // min, max
  scale_type: 'linear' | 'sqrt' | 'log';
  area_based: boolean; // vs. radius-based for circles
}

export interface ShapeEncoding {
  enabled: boolean;
  shape_set: string[]; // Unicode symbols or SVG paths
  semantic_mapping: { [value: string]: string };
  accessibility_friendly: boolean;
}

export interface OpacityEncoding {
  enabled: boolean;
  opacity_range: [number, number];
  purpose: 'confidence' | 'density' | 'time_decay' | 'focus';
}

export interface TextureEncoding {
  enabled: boolean;
  patterns: string[]; // SVG pattern definitions
  purpose: 'colorblind_support' | 'print_friendly' | 'categorical_distinction';
}

export interface ColorSchemeStrategy {
  primary_schemes: ColorScheme[];
  severity_colors: SeverityColorScheme;
  confidence_colors: ConfidenceColorScheme;
  trend_colors: TrendColorScheme;
  status_colors: StatusColorScheme;
  accessibility: ColorAccessibility;
}

export interface ColorScheme {
  name: string;
  type: 'categorical' | 'sequential' | 'diverging';
  colors: string[];
  use_cases: string[];
  contrast_ratios: number[];
  colorblind_friendly: boolean;
}

export interface SeverityColorScheme {
  critical: string;
  high: string;
  medium: string;
  low: string;
  info: string;
  
  // Additional semantic colors
  resolved: string;
  in_progress: string;
  blocked: string;
  
  // Gradients for continuous scales
  severity_gradient: string[]; // from low to critical
}

export interface ConfidenceColorScheme {
  high_confidence: string;
  medium_confidence: string;
  low_confidence: string;
  unknown_confidence: string;
  
  // Opacity mapping
  confidence_opacity_mapping: { [level: string]: number };
  
  // Pattern overlays for low confidence
  uncertainty_patterns: string[];
}

export interface TrendColorScheme {
  improving: string;
  stable: string;
  declining: string;
  volatile: string;
  
  // Directional indicators
  strong_positive: string;
  weak_positive: string;
  neutral: string;
  weak_negative: string;
  strong_negative: string;
}

export interface StatusColorScheme {
  success: string;
  warning: string;
  error: string;
  info: string;
  
  // Process states
  pending: string;
  active: string;
  completed: string;
  cancelled: string;
  
  // Health states
  healthy: string;
  degraded: string;
  critical: string;
  unknown: string;
}

export interface ColorAccessibility {
  wcag_compliance: 'AA' | 'AAA';
  contrast_ratios: { [color_pair: string]: number };
  colorblind_simulation: ColorblindSimulation;
  alternative_encodings: AlternativeEncoding[];
}

export interface ColorblindSimulation {
  protanopia: ColorMapping[];
  deuteranopia: ColorMapping[];
  tritanopia: ColorMapping[];
  achromatopsia: ColorMapping[];
}

export interface ColorMapping {
  original_color: string;
  simulated_color: string;
  distinguishable: boolean;
}

export interface AlternativeEncoding {
  encoding_type: 'pattern' | 'shape' | 'text' | 'icon';
  mapping: { [color: string]: string };
  implementation: string;
}

export interface InteractionPattern {
  pattern_name: string;
  use_cases: string[];
  implementation: InteractionImplementation;
  user_feedback: UserFeedback;
  performance_impact: 'low' | 'medium' | 'high';
}

export interface InteractionImplementation {
  event_types: string[];
  state_management: StateManagement;
  animation: AnimationStrategy;
  responsive_behavior: ResponsiveBehavior;
}

export interface StateManagement {
  state_variables: StateVariable[];
  update_patterns: UpdatePattern[];
  persistence: PersistenceStrategy;
}

export interface StateVariable {
  name: string;
  type: 'boolean' | 'number' | 'string' | 'array' | 'object';
  default_value: any;
  scope: 'local' | 'global' | 'shared';
  reset_conditions: string[];
}

export interface UpdatePattern {
  trigger: string;
  state_changes: StateChange[];
  side_effects: SideEffect[];
  batching: boolean;
}

export interface StateChange {
  variable: string;
  operation: 'set' | 'toggle' | 'increment' | 'push' | 'merge';
  value: any;
  condition?: string;
}

export interface SideEffect {
  type: 'animation' | 'data_fetch' | 'notification' | 'navigation';
  parameters: { [key: string]: any };
  delay: number; // milliseconds
}

export interface PersistenceStrategy {
  enabled: boolean;
  storage_type: 'localStorage' | 'sessionStorage' | 'indexedDB' | 'url_params';
  keys_to_persist: string[];
  serialization: 'json' | 'compressed' | 'custom';
}

export interface AnimationStrategy {
  enabled: boolean;
  duration: number; // milliseconds
  easing: string; // CSS easing function
  stagger: number; // delay between multiple elements
  
  // Performance optimizations
  use_transforms: boolean;
  use_opacity: boolean;
  avoid_layout_thrash: boolean;
  gpu_acceleration: boolean;
}

export interface ResponsiveBehavior {
  breakpoints: { [breakpoint: string]: number };
  behavior_changes: BehaviorChange[];
  touch_optimizations: TouchOptimization[];
}

export interface BehaviorChange {
  breakpoint: string;
  changes: { [interaction: string]: string };
  disabled_features: string[];
}

export interface TouchOptimization {
  gesture_type: 'tap' | 'pinch' | 'pan' | 'swipe';
  optimization: string;
  fallback_behavior: string;
}

export interface UserFeedback {
  visual_feedback: VisualFeedback[];
  audio_feedback: AudioFeedback[];
  haptic_feedback: HapticFeedback[];
}

export interface VisualFeedback {
  feedback_type: 'highlight' | 'glow' | 'bounce' | 'shake' | 'fade';
  trigger: string;
  duration: number;
  intensity: number;
}

export interface AudioFeedback {
  sound_type: 'click' | 'hover' | 'success' | 'error' | 'notification';
  volume: number;
  frequency: number; // Hz
  respect_system_settings: boolean;
}

export interface HapticFeedback {
  pattern: 'light' | 'medium' | 'heavy' | 'custom';
  duration: number;
  supported_devices: string[];
  fallback: string;
}

export interface PerformanceStrategy {
  optimization_techniques: OptimizationTechnique[];
  rendering_strategy: RenderingStrategy;
  data_handling: DataHandlingStrategy;
  memory_management: MemoryManagement;
  monitoring: PerformanceMonitoring;
}

export interface OptimizationTechnique {
  technique_name: string;
  description: string;
  use_cases: string[];
  implementation_complexity: 'low' | 'medium' | 'high';
  performance_gain: 'low' | 'medium' | 'high';
  trade_offs: string[];
}

export interface RenderingStrategy {
  rendering_mode: 'svg' | 'canvas' | 'webgl' | 'hybrid';
  frame_rate_target: number;
  batch_rendering: boolean;
  
  // Canvas optimizations
  canvas_optimizations: CanvasOptimization[];
  
  // SVG optimizations
  svg_optimizations: SvgOptimization[];
  
  // WebGL optimizations
  webgl_optimizations: WebglOptimization[];
}

export interface CanvasOptimization {
  optimization_type: string;
  description: string;
  applicability: string[];
  performance_impact: string;
}

export interface SvgOptimization {
  optimization_type: string;
  description: string;
  applicability: string[];
  performance_impact: string;
}

export interface WebglOptimization {
  optimization_type: string;
  description: string;
  applicability: string[];
  performance_impact: string;
}

export interface DataHandlingStrategy {
  data_virtualization: DataVirtualization;
  data_streaming: DataStreaming;
  data_aggregation: DataAggregation;
  caching_strategy: CachingStrategy;
}

export interface DataVirtualization {
  enabled: boolean;
  viewport_buffer: number;
  item_height_estimation: 'fixed' | 'dynamic' | 'measured';
  recycling_enabled: boolean;
  
  // Performance parameters
  render_ahead_count: number;
  render_behind_count: number;
  scroll_debounce: number; // milliseconds
}

export interface DataStreaming {
  enabled: boolean;
  chunk_size: number;
  loading_strategy: 'progressive' | 'on_demand' | 'predictive';
  
  // Stream management
  buffer_size: number;
  prefetch_count: number;
  backpressure_handling: string;
}

export interface DataAggregation {
  temporal_aggregation: TemporalAggregation;
  spatial_aggregation: SpatialAggregation;
  categorical_aggregation: CategoricalAggregation;
  
  // Aggregation parameters
  aggregation_threshold: number; // data points
  aggregation_algorithm: string;
  preserve_outliers: boolean;
}

export interface TemporalAggregation {
  intervals: string[]; // '1m', '5m', '1h', '1d', etc.
  aggregation_functions: string[]; // 'avg', 'sum', 'max', 'min', 'count'
  window_sliding: boolean;
  timezone_handling: string;
}

export interface SpatialAggregation {
  clustering_algorithm: 'kmeans' | 'hierarchical' | 'density';
  max_clusters: number;
  distance_threshold: number;
  preserve_boundaries: boolean;
}

export interface CategoricalAggregation {
  grouping_strategy: 'frequency' | 'alphabetical' | 'custom';
  max_categories: number;
  other_category: boolean;
  min_frequency_threshold: number;
}

export interface CachingStrategy {
  cache_levels: CacheLevel[];
  invalidation_strategy: InvalidationStrategy;
  cache_size_limits: CacheSizeLimit[];
}

export interface CacheLevel {
  level_name: string;
  storage_type: 'memory' | 'indexedDB' | 'localStorage';
  max_age: number; // seconds
  max_size: number; // bytes
  eviction_policy: 'LRU' | 'LFU' | 'FIFO';
}

export interface InvalidationStrategy {
  strategy_type: 'time_based' | 'event_based' | 'manual';
  invalidation_triggers: string[];
  cascade_invalidation: boolean;
  partial_invalidation: boolean;
}

export interface CacheSizeLimit {
  resource_type: 'total_memory' | 'per_chart' | 'per_dataset';
  limit_value: number;
  limit_unit: 'bytes' | 'mb' | 'count';
  overflow_behavior: 'evict_lru' | 'reject_new' | 'compress';
}

export interface MemoryManagement {
  garbage_collection: GarbageCollection;
  object_pooling: ObjectPooling;
  memory_monitoring: MemoryMonitoring;
}

export interface GarbageCollection {
  manual_gc_triggers: string[];
  gc_frequency: number; // milliseconds
  gc_threshold: number; // memory usage percentage
  cleanup_strategies: CleanupStrategy[];
}

export interface CleanupStrategy {
  cleanup_type: string;
  trigger_condition: string;
  cleanup_action: string;
  performance_impact: string;
}

export interface ObjectPooling {
  enabled: boolean;
  pooled_types: string[];
  pool_sizes: { [type: string]: number };
  pool_growth_strategy: 'fixed' | 'dynamic' | 'on_demand';
}

export interface MemoryMonitoring {
  enabled: boolean;
  monitoring_interval: number; // milliseconds
  memory_thresholds: MemoryThreshold[];
  reporting_strategy: string;
}

export interface MemoryThreshold {
  threshold_type: 'warning' | 'critical' | 'emergency';
  threshold_value: number; // MB
  response_actions: string[];
}

export interface PerformanceMonitoring {
  metrics: PerformanceMetric[];
  profiling: ProfilingStrategy;
  alerting: AlertingStrategy;
}

export interface PerformanceMetric {
  metric_name: string;
  measurement_method: string;
  target_value: number;
  warning_threshold: number;
  critical_threshold: number;
}

export interface ProfilingStrategy {
  enabled: boolean;
  profiling_tools: string[];
  profiling_frequency: string;
  automatic_optimization: boolean;
}

export interface AlertingStrategy {
  alert_channels: string[];
  alert_thresholds: AlertThreshold[];
  escalation_rules: string[];
}

export interface AlertThreshold {
  metric: string;
  threshold_value: number;
  duration: number; // seconds above threshold
  severity: 'info' | 'warning' | 'error';
}

export interface AccessibilityStrategy {
  wcag_compliance: WcagCompliance;
  assistive_technology: AssistiveTechnology;
  keyboard_navigation: KeyboardNavigation;
  screen_reader_support: ScreenReaderSupport;
  visual_accessibility: VisualAccessibility;
}

export interface WcagCompliance {
  target_level: 'A' | 'AA' | 'AAA';
  guidelines: WcagGuideline[];
  testing_procedures: TestingProcedure[];
  compliance_monitoring: ComplianceMonitoring;
}

export interface WcagGuideline {
  guideline_number: string;
  guideline_title: string;
  success_criteria: SuccessCriterion[];
  implementation_notes: string[];
}

export interface SuccessCriterion {
  criterion_number: string;
  criterion_title: string;
  conformance_level: 'A' | 'AA' | 'AAA';
  implementation_approach: string;
  testing_method: string;
}

export interface TestingProcedure {
  test_type: 'automated' | 'manual' | 'user_testing';
  test_tools: string[];
  test_frequency: string;
  pass_criteria: string;
}

export interface ComplianceMonitoring {
  monitoring_tools: string[];
  monitoring_frequency: string;
  reporting_format: string;
  remediation_process: string;
}

export interface AssistiveTechnology {
  supported_technologies: SupportedTechnology[];
  compatibility_testing: CompatibilityTesting;
  fallback_strategies: FallbackStrategy[];
}

export interface SupportedTechnology {
  technology_name: string;
  technology_type: 'screen_reader' | 'voice_control' | 'switch_navigation' | 'eye_tracking';
  support_level: 'full' | 'partial' | 'basic';
  specific_optimizations: string[];
}

export interface CompatibilityTesting {
  testing_matrix: TestingMatrix[];
  testing_frequency: string;
  user_testing_involvement: boolean;
}

export interface TestingMatrix {
  technology: string;
  browser: string;
  operating_system: string;
  test_scenarios: string[];
  expected_behavior: string[];
}

export interface FallbackStrategy {
  trigger_condition: string;
  fallback_mechanism: string;
  user_notification: boolean;
  recovery_options: string[];
}

export interface KeyboardNavigation {
  navigation_patterns: NavigationPattern[];
  focus_management: FocusManagement;
  keyboard_shortcuts: KeyboardShortcut[];
}

export interface NavigationPattern {
  pattern_name: string;
  key_sequence: string[];
  navigation_flow: string[];
  visual_indicators: string[];
}

export interface FocusManagement {
  focus_order: FocusOrder;
  focus_indicators: FocusIndicator[];
  focus_trapping: FocusTrapping;
}

export interface FocusOrder {
  ordering_strategy: 'dom_order' | 'visual_order' | 'logical_order';
  skip_links: SkipLink[];
  focus_restoration: boolean;
}

export interface SkipLink {
  link_text: string;
  target_element: string;
  visibility: 'always' | 'on_focus' | 'screen_reader_only';
}

export interface FocusIndicator {
  indicator_type: 'outline' | 'background' | 'border' | 'shadow';
  visual_properties: VisualProperties;
  animation: boolean;
}

export interface VisualProperties {
  color: string;
  width: number;
  style: 'solid' | 'dashed' | 'dotted';
  contrast_ratio: number;
}

export interface FocusTrapping {
  trap_boundaries: string[];
  escape_mechanisms: string[];
  cyclic_navigation: boolean;
}

export interface KeyboardShortcut {
  shortcut_combination: string;
  action_description: string;
  context: string[];
  conflict_resolution: string;
}

export interface ScreenReaderSupport {
  aria_implementation: AriaImplementation;
  semantic_markup: SemanticMarkup;
  dynamic_content: DynamicContent;
}

export interface AriaImplementation {
  aria_labels: AriaLabel[];
  aria_descriptions: AriaDescription[];
  aria_roles: AriaRole[];
  aria_states: AriaState[];
}

export interface AriaLabel {
  element_selector: string;
  label_text: string;
  label_source: 'static' | 'dynamic' | 'computed';
  context_dependent: boolean;
}

export interface AriaDescription {
  element_selector: string;
  description_text: string;
  description_detail: 'brief' | 'detailed' | 'comprehensive';
}

export interface AriaRole {
  element_selector: string;
  role_name: string;
  role_justification: string;
  implicit_role_override: boolean;
}

export interface AriaState {
  property_name: string;
  property_values: string[];
  update_triggers: string[];
  announcement_strategy: string;
}

export interface SemanticMarkup {
  heading_structure: HeadingStructure;
  landmark_regions: LandmarkRegion[];
  list_structures: ListStructure[];
}

export interface HeadingStructure {
  heading_levels: HeadingLevel[];
  heading_hierarchy: boolean;
  skip_level_allowed: boolean;
}

export interface HeadingLevel {
  level: number;
  usage_context: string[];
  styling_approach: string;
}

export interface LandmarkRegion {
  region_type: 'banner' | 'navigation' | 'main' | 'complementary' | 'contentinfo';
  region_label: string;
  unique_identification: boolean;
}

export interface ListStructure {
  list_type: 'ordered' | 'unordered' | 'description';
  nesting_allowed: boolean;
  semantic_meaning: string;
}

export interface DynamicContent {
  live_regions: LiveRegion[];
  content_updates: ContentUpdate[];
  progressive_disclosure: ProgressiveDisclosure;
}

export interface LiveRegion {
  region_selector: string;
  politeness_level: 'off' | 'polite' | 'assertive';
  atomic_updates: boolean;
  relevant_changes: string[];
}

export interface ContentUpdate {
  update_type: 'addition' | 'removal' | 'modification';
  announcement_strategy: string;
  timing_considerations: string[];
}

export interface ProgressiveDisclosure {
  disclosure_patterns: DisclosurePattern[];
  state_communication: string[];
  navigation_preservation: boolean;
}

export interface DisclosurePattern {
  trigger_element: string;
  disclosed_content: string;
  disclosure_state: string;
  announcement_text: string;
}

export interface VisualAccessibility {
  color_contrast: ColorContrast;
  text_scaling: TextScaling;
  motion_preferences: MotionPreferences;
  visual_alternatives: VisualAlternative[];
}

export interface ColorContrast {
  contrast_ratios: ContrastRatio[];
  automated_checking: boolean;
  manual_verification: boolean;
}

export interface ContrastRatio {
  element_type: string;
  minimum_ratio: number;
  target_ratio: number;
  measurement_method: string;
}

export interface TextScaling {
  supported_zoom_levels: number[];
  layout_adaptation: boolean;
  content_reflow: boolean;
  functionality_preservation: boolean;
}

export interface MotionPreferences {
  respect_reduced_motion: boolean;
  motion_alternatives: MotionAlternative[];
  user_controls: UserControl[];
}

export interface MotionAlternative {
  motion_type: string;
  static_alternative: string;
  reduced_motion_version: string;
}

export interface UserControl {
  control_type: string;
  control_location: string;
  default_state: string;
  persistence: boolean;
}

export interface VisualAlternative {
  visual_element: string;
  alternative_type: 'text' | 'audio' | 'tactile' | 'simplified_visual';
  alternative_content: string;
  equivalency_level: 'full' | 'partial' | 'summary';
}

// Specific Visualization Strategies for PhotonDrift

export const PHOTON_DRIFT_VISUALIZATIONS: VisualizationStrategy[] = [
  {
    metric_category: 'drift_events',
    recommended_charts: [
      {
        chart_type: 'temporal_scatter_plot',
        use_cases: [
          'Show drift events over time with ML confidence',
          'Identify patterns in drift occurrence',
          'Correlate events with external factors'
        ],
        data_requirements: [
          { field_type: 'datetime', cardinality: 'high', temporal: true, categorical: false, numerical: false, hierarchical: false },
          { field_type: 'severity', cardinality: 'low', temporal: false, categorical: true, numerical: false, hierarchical: false },
          { field_type: 'ml_confidence', cardinality: 'high', temporal: false, categorical: false, numerical: true, hierarchical: false }
        ],
        visual_encoding: {
          position: {
            x_axis: {
              scale_type: 'time',
              domain_calculation: 'data_driven',
              tick_strategy: { count_method: 'auto', target_count: 6, min_spacing: 50, format: '%b %d' },
              label_strategy: { rotation: -45, truncation: 'none', max_length: 20, show_all: false, priority_based: true }
            },
            y_axis: {
              scale_type: 'ordinal',
              domain_calculation: 'fixed',
              tick_strategy: { count_method: 'fixed', target_count: 4, min_spacing: 30, format: '' },
              label_strategy: { rotation: 0, truncation: 'ellipsis', max_length: 15, show_all: true, priority_based: false }
            },
            faceting: { enabled: false, max_facets: 0, facet_arrangement: 'grid', shared_axes: true, independent_scales: false },
            small_multiples: false
          },
          color: {
            purpose: 'categorical',
            palette_name: 'severity_palette',
            custom_colors: ['#28a745', '#ffc107', '#fd7e14', '#dc3545'],
            semantic_mapping: [
              { value: 'low', color: '#28a745', meaning: 'Low severity', cultural_context: ['universal_green_safe'] },
              { value: 'medium', color: '#ffc107', meaning: 'Medium severity', cultural_context: ['universal_yellow_caution'] },
              { value: 'high', color: '#fd7e14', meaning: 'High severity', cultural_context: ['universal_orange_warning'] },
              { value: 'critical', color: '#dc3545', meaning: 'Critical severity', cultural_context: ['universal_red_danger'] }
            ],
            accessibility_compliant: true
          },
          size: {
            enabled: true,
            size_range: [4, 16],
            scale_type: 'sqrt',
            area_based: true
          },
          shape: {
            enabled: true,
            shape_set: ['circle', 'triangle', 'square', 'diamond'],
            semantic_mapping: {
              'code-smell': 'circle',
              'architecture': 'triangle',
              'security': 'square',
              'performance': 'diamond'
            },
            accessibility_friendly: true
          },
          opacity: {
            enabled: true,
            opacity_range: [0.3, 1.0],
            purpose: 'confidence'
          },
          texture: {
            enabled: true,
            patterns: ['solid', 'diagonal-lines', 'dots'],
            purpose: 'colorblind_support'
          }
        },
        best_practices: [
          'Use opacity to encode ML confidence levels',
          'Implement brushing for temporal selection',
          'Add jitter to prevent overplotting',
          'Provide drill-down to event details',
          'Show trend lines for severity over time'
        ],
        common_mistakes: [
          'Overplotting without jitter or aggregation',
          'Using too many visual encodings simultaneously',
          'Not providing clear legends for multiple encodings',
          'Ignoring accessibility in color choices'
        ],
        implementation_complexity: 'medium',
        recommended_libraries: ['D3.js', 'Observable Plot', 'Vega-Lite', 'Chart.js']
      },
      {
        chart_type: 'heatmap_calendar',
        use_cases: [
          'Show daily drift patterns',
          'Identify seasonal trends',
          'Compare activity across time periods'
        ],
        data_requirements: [
          { field_type: 'date', cardinality: 'high', temporal: true, categorical: false, numerical: false, hierarchical: false },
          { field_type: 'event_count', cardinality: 'medium', temporal: false, categorical: false, numerical: true, hierarchical: false }
        ],
        visual_encoding: {
          position: {
            x_axis: {
              scale_type: 'band',
              domain_calculation: 'data_driven',
              tick_strategy: { count_method: 'fixed', target_count: 7, min_spacing: 20, format: '%a' },
              label_strategy: { rotation: 0, truncation: 'none', max_length: 3, show_all: true, priority_based: false }
            },
            y_axis: {
              scale_type: 'band',
              domain_calculation: 'data_driven',
              tick_strategy: { count_method: 'auto', target_count: 12, min_spacing: 15, format: '%b' },
              label_strategy: { rotation: 0, truncation: 'none', max_length: 10, show_all: true, priority_based: false }
            },
            faceting: { enabled: true, max_facets: 4, facet_arrangement: 'horizontal', shared_axes: true, independent_scales: false },
            small_multiples: true
          },
          color: {
            purpose: 'sequential',
            palette_name: 'YlOrRd',
            custom_colors: ['#ffffcc', '#fd8d3c', '#e31a1c'],
            semantic_mapping: [],
            accessibility_compliant: true
          },
          size: { enabled: false, size_range: [0, 0], scale_type: 'linear', area_based: false },
          shape: { enabled: false, shape_set: [], semantic_mapping: {}, accessibility_friendly: false },
          opacity: { enabled: false, opacity_range: [1, 1], purpose: 'confidence' },
          texture: { enabled: false, patterns: [], purpose: 'colorblind_support' }
        },
        best_practices: [
          'Use sequential color scheme for intensity',
          'Add tooltips with exact counts and dates',
          'Implement year/month navigation',
          'Show missing data explicitly',
          'Provide aggregation controls'
        ],
        common_mistakes: [
          'Using diverging colors for sequential data',
          'Not handling missing dates properly',
          'Making cells too small to interact with',
          'Overwhelming users with too much detail'
        ],
        implementation_complexity: 'medium',
        recommended_libraries: ['D3.js', 'Cal-Heatmap', 'Plotly.js']
      }
    ],
    color_schemes: {
      primary_schemes: [
        {
          name: 'severity_categorical',
          type: 'categorical',
          colors: ['#28a745', '#ffc107', '#fd7e14', '#dc3545'],
          use_cases: ['severity levels', 'status indicators', 'categorical data'],
          contrast_ratios: [4.5, 3.2, 4.1, 5.2],
          colorblind_friendly: true
        }
      ],
      severity_colors: {
        critical: '#dc3545',
        high: '#fd7e14',
        medium: '#ffc107',
        low: '#28a745',
        info: '#17a2b8',
        resolved: '#6c757d',
        in_progress: '#007bff',
        blocked: '#6f42c1',
        severity_gradient: ['#28a745', '#ffc107', '#fd7e14', '#dc3545']
      },
      confidence_colors: {
        high_confidence: '#28a745',
        medium_confidence: '#ffc107',
        low_confidence: '#fd7e14',
        unknown_confidence: '#6c757d',
        confidence_opacity_mapping: {
          'high': 1.0,
          'medium': 0.7,
          'low': 0.4,
          'unknown': 0.2
        },
        uncertainty_patterns: ['diagonal-lines', 'dots', 'crosshatch']
      },
      trend_colors: {
        improving: '#28a745',
        stable: '#6c757d',
        declining: '#dc3545',
        volatile: '#6f42c1',
        strong_positive: '#155724',
        weak_positive: '#28a745',
        neutral: '#6c757d',
        weak_negative: '#fd7e14',
        strong_negative: '#dc3545'
      },
      status_colors: {
        success: '#28a745',
        warning: '#ffc107',
        error: '#dc3545',
        info: '#17a2b8',
        pending: '#6c757d',
        active: '#007bff',
        completed: '#28a745',
        cancelled: '#6c757d',
        healthy: '#28a745',
        degraded: '#ffc107',
        critical: '#dc3545',
        unknown: '#6c757d'
      },
      accessibility: {
        wcag_compliance: 'AA',
        contrast_ratios: {
          'critical_white': 5.2,
          'high_white': 4.1,
          'medium_black': 3.2,
          'low_white': 4.5
        },
        colorblind_simulation: {
          protanopia: [
            { original_color: '#dc3545', simulated_color: '#b8860b', distinguishable: true },
            { original_color: '#28a745', simulated_color: '#8b4513', distinguishable: true }
          ],
          deuteranopia: [
            { original_color: '#dc3545', simulated_color: '#daa520', distinguishable: true },
            { original_color: '#28a745', simulated_color: '#8b4513', distinguishable: true }
          ],
          tritanopia: [
            { original_color: '#17a2b8', simulated_color: '#ff69b4', distinguishable: true }
          ],
          achromatopsia: [
            { original_color: '#dc3545', simulated_color: '#696969', distinguishable: false }
          ]
        },
        alternative_encodings: [
          {
            encoding_type: 'pattern',
            mapping: {
              '#dc3545': 'diagonal-lines',
              '#fd7e14': 'dots',
              '#ffc107': 'crosshatch',
              '#28a745': 'solid'
            },
            implementation: 'SVG patterns'
          }
        ]
      }
    },
    interaction_patterns: [
      {
        pattern_name: 'drill_down_exploration',
        use_cases: ['Navigate from overview to detail', 'Explore event hierarchies', 'Progressive disclosure'],
        implementation: {
          event_types: ['click', 'double-click', 'right-click'],
          state_management: {
            state_variables: [
              { name: 'drill_level', type: 'number', default_value: 0, scope: 'local', reset_conditions: ['navigation', 'filter_change'] },
              { name: 'selected_path', type: 'array', default_value: [], scope: 'local', reset_conditions: ['drill_up', 'filter_clear'] }
            ],
            update_patterns: [
              {
                trigger: 'drill_down_click',
                state_changes: [
                  { variable: 'drill_level', operation: 'increment', value: 1 },
                  { variable: 'selected_path', operation: 'push', value: 'clicked_item_id' }
                ],
                side_effects: [
                  { type: 'data_fetch', parameters: { endpoint: '/api/drill-down' }, delay: 0 },
                  { type: 'animation', parameters: { type: 'slide_transition' }, delay: 100 }
                ],
                batching: true
              }
            ],
            persistence: {
              enabled: true,
              storage_type: 'sessionStorage',
              keys_to_persist: ['selected_path', 'drill_level'],
              serialization: 'json'
            }
          },
          animation: {
            enabled: true,
            duration: 300,
            easing: 'ease-out',
            stagger: 50,
            use_transforms: true,
            use_opacity: true,
            avoid_layout_thrash: true,
            gpu_acceleration: true
          },
          responsive_behavior: {
            breakpoints: { 'mobile': 768, 'tablet': 1024, 'desktop': 1200 },
            behavior_changes: [
              { breakpoint: 'mobile', changes: { 'click': 'tap' }, disabled_features: ['hover_preview'] }
            ],
            touch_optimizations: [
              { gesture_type: 'tap', optimization: 'larger_touch_targets', fallback_behavior: 'click_simulation' }
            ]
          }
        },
        user_feedback: {
          visual_feedback: [
            { feedback_type: 'highlight', trigger: 'hover', duration: 200, intensity: 0.8 },
            { feedback_type: 'glow', trigger: 'focus', duration: 0, intensity: 1.0 }
          ],
          audio_feedback: [
            { sound_type: 'click', volume: 0.3, frequency: 800, respect_system_settings: true }
          ],
          haptic_feedback: [
            { pattern: 'light', duration: 50, supported_devices: ['mobile', 'tablet'], fallback: 'none' }
          ]
        },
        performance_impact: 'medium'
      }
    ],
    performance_considerations: {
      optimization_techniques: [
        {
          technique_name: 'canvas_rendering_for_large_datasets',
          description: 'Use HTML5 Canvas for rendering when displaying >1000 drift events',
          use_cases: ['Large scatter plots', 'Heatmaps with high granularity', 'Real-time updates'],
          implementation_complexity: 'high',
          performance_gain: 'high',
          trade_offs: ['Reduced interactivity', 'Accessibility challenges', 'Complexity in event handling']
        },
        {
          technique_name: 'data_aggregation_by_time',
          description: 'Aggregate drift events by time intervals for overview charts',
          use_cases: ['Timeline visualizations', 'Trend analysis', 'Performance optimization'],
          implementation_complexity: 'medium',
          performance_gain: 'high',
          trade_offs: ['Loss of granular detail', 'Aggregation computation overhead']
        }
      ],
      rendering_strategy: {
        rendering_mode: 'hybrid',
        frame_rate_target: 60,
        batch_rendering: true,
        canvas_optimizations: [
          { optimization_type: 'object_pooling', description: 'Reuse canvas drawing objects', applicability: ['scatter_plots', 'animations'], performance_impact: 'medium' },
          { optimization_type: 'dirty_region_rendering', description: 'Only redraw changed areas', applicability: ['updates', 'interactions'], performance_impact: 'high' }
        ],
        svg_optimizations: [
          { optimization_type: 'element_recycling', description: 'Reuse SVG elements for data updates', applicability: ['data_binding', 'transitions'], performance_impact: 'medium' }
        ],
        webgl_optimizations: [
          { optimization_type: 'shader_programs', description: 'Use GPU shaders for complex visualizations', applicability: ['3d_visualizations', 'large_datasets'], performance_impact: 'high' }
        ]
      },
      data_handling: {
        data_virtualization: {
          enabled: true,
          viewport_buffer: 100,
          item_height_estimation: 'dynamic',
          recycling_enabled: true,
          render_ahead_count: 50,
          render_behind_count: 25,
          scroll_debounce: 16
        },
        data_streaming: {
          enabled: true,
          chunk_size: 100,
          loading_strategy: 'progressive',
          buffer_size: 1000,
          prefetch_count: 2,
          backpressure_handling: 'drop_oldest'
        },
        data_aggregation: {
          temporal_aggregation: {
            intervals: ['1m', '5m', '15m', '1h', '1d'],
            aggregation_functions: ['count', 'avg', 'max'],
            window_sliding: true,
            timezone_handling: 'user_local'
          },
          spatial_aggregation: {
            clustering_algorithm: 'kmeans',
            max_clusters: 10,
            distance_threshold: 0.1,
            preserve_boundaries: true
          },
          categorical_aggregation: {
            grouping_strategy: 'frequency',
            max_categories: 20,
            other_category: true,
            min_frequency_threshold: 0.01
          },
          
          // Required aggregation parameters
          aggregation_threshold: 5000,
          aggregation_algorithm: 'adaptive',
          preserve_outliers: true
        },
        caching_strategy: {
          cache_levels: [
            { level_name: 'memory', storage_type: 'memory', max_age: 300, max_size: 50000000, eviction_policy: 'LRU' },
            { level_name: 'browser', storage_type: 'indexedDB', max_age: 3600, max_size: 100000000, eviction_policy: 'LRU' }
          ],
          invalidation_strategy: {
            strategy_type: 'event_based',
            invalidation_triggers: ['data_update', 'filter_change'],
            cascade_invalidation: true,
            partial_invalidation: true
          },
          cache_size_limits: [
            { resource_type: 'total_memory', limit_value: 100, limit_unit: 'mb', overflow_behavior: 'evict_lru' }
          ]
        }
      },
      memory_management: {
        garbage_collection: {
          manual_gc_triggers: ['data_refresh', 'view_change'],
          gc_frequency: 30000,
          gc_threshold: 80,
          cleanup_strategies: [
            { cleanup_type: 'event_listeners', trigger_condition: 'component_unmount', cleanup_action: 'removeEventListener', performance_impact: 'low' }
          ]
        },
        object_pooling: {
          enabled: true,
          pooled_types: ['ChartPoint', 'DataElement', 'InteractionHandler'],
          pool_sizes: { 'ChartPoint': 1000, 'DataElement': 500, 'InteractionHandler': 50 },
          pool_growth_strategy: 'dynamic'
        },
        memory_monitoring: {
          enabled: true,
          monitoring_interval: 5000,
          memory_thresholds: [
            { threshold_type: 'warning', threshold_value: 100, response_actions: ['reduce_cache'] },
            { threshold_type: 'critical', threshold_value: 150, response_actions: ['force_gc', 'reduce_quality'] }
          ],
          reporting_strategy: 'console_and_analytics'
        }
      },
      monitoring: {
        metrics: [
          { metric_name: 'render_time', measurement_method: 'performance.now', target_value: 16, warning_threshold: 20, critical_threshold: 33 },
          { metric_name: 'memory_usage', measurement_method: 'performance.memory', target_value: 50, warning_threshold: 100, critical_threshold: 150 }
        ],
        profiling: {
          enabled: true,
          profiling_tools: ['Chrome DevTools', 'React DevTools Profiler'],
          profiling_frequency: 'on_demand',
          automatic_optimization: false
        },
        alerting: {
          alert_channels: ['console', 'analytics'],
          alert_thresholds: [
            { metric: 'render_time', threshold_value: 33, duration: 5, severity: 'warning' }
          ],
          escalation_rules: ['performance_team_notification']
        }
      }
    },
    accessibility_requirements: {
      wcag_compliance: {
        target_level: 'AA',
        guidelines: [
          {
            guideline_number: '1.4.3',
            guideline_title: 'Contrast (Minimum)',
            success_criteria: [
              {
                criterion_number: '1.4.3',
                criterion_title: 'Color contrast ratio of at least 4.5:1',
                conformance_level: 'AA',
                implementation_approach: 'Use high contrast colors and test with automated tools',
                testing_method: 'Automated contrast checking + manual verification'
              }
            ],
            implementation_notes: ['Test all color combinations', 'Provide alternative encodings for color']
          }
        ],
        testing_procedures: [
          { test_type: 'automated', test_tools: ['axe-core', 'pa11y'], test_frequency: 'every_build', pass_criteria: 'zero_violations' },
          { test_type: 'manual', test_tools: ['keyboard_navigation', 'screen_reader'], test_frequency: 'weekly', pass_criteria: 'user_task_completion' }
        ],
        compliance_monitoring: {
          monitoring_tools: ['axe-core', 'lighthouse'],
          monitoring_frequency: 'continuous',
          reporting_format: 'accessibility_dashboard',
          remediation_process: 'ticket_creation_and_sprint_planning'
        }
      },
      assistive_technology: {
        supported_technologies: [
          { technology_name: 'NVDA', technology_type: 'screen_reader', support_level: 'full', specific_optimizations: ['proper_aria_labels', 'structured_navigation'] },
          { technology_name: 'JAWS', technology_type: 'screen_reader', support_level: 'full', specific_optimizations: ['table_headers', 'form_labels'] },
          { technology_name: 'VoiceOver', technology_type: 'screen_reader', support_level: 'full', specific_optimizations: ['rotor_navigation', 'gesture_support'] }
        ],
        compatibility_testing: {
          testing_matrix: [
            { technology: 'NVDA', browser: 'Firefox', operating_system: 'Windows', test_scenarios: ['chart_navigation', 'data_table_fallback'], expected_behavior: ['announce_chart_type', 'read_table_headers'] }
          ],
          testing_frequency: 'monthly',
          user_testing_involvement: true
        },
        fallback_strategies: [
          { trigger_condition: 'screen_reader_detected', fallback_mechanism: 'data_table_view', user_notification: true, recovery_options: ['toggle_view', 'export_data'] }
        ]
      },
      keyboard_navigation: {
        navigation_patterns: [
          { pattern_name: 'tab_navigation', key_sequence: ['Tab', 'Shift+Tab'], navigation_flow: ['chart_container', 'legend', 'filters', 'data_table'], visual_indicators: ['focus_outline'] }
        ],
        focus_management: {
          focus_order: {
            ordering_strategy: 'logical_order',
            skip_links: [
              { link_text: 'Skip to chart', target_element: '#main-chart', visibility: 'on_focus' },
              { link_text: 'Skip to data table', target_element: '#data-table', visibility: 'on_focus' }
            ],
            focus_restoration: true
          },
          focus_indicators: [
            { indicator_type: 'outline', visual_properties: { color: '#0066cc', width: 2, style: 'solid', contrast_ratio: 4.5 }, animation: false }
          ],
          focus_trapping: {
            trap_boundaries: ['modal_dialogs', 'dropdown_menus'],
            escape_mechanisms: ['Escape_key', 'click_outside'],
            cyclic_navigation: true
          }
        },
        keyboard_shortcuts: [
          { shortcut_combination: 'Alt+D', action_description: 'Open data table view', context: ['chart_focused'], conflict_resolution: 'override_browser' }
        ]
      },
      screen_reader_support: {
        aria_implementation: {
          aria_labels: [
            { element_selector: '.chart-container', label_text: 'Drift events scatter plot', label_source: 'static', context_dependent: false },
            { element_selector: '.data-point', label_text: 'Drift event: {severity} at {timestamp}', label_source: 'dynamic', context_dependent: true }
          ],
          aria_descriptions: [
            { element_selector: '.chart-container', description_text: 'Interactive scatter plot showing drift events over time. Use arrow keys to navigate data points.', description_detail: 'comprehensive' }
          ],
          aria_roles: [
            { element_selector: '.chart-container', role_name: 'img', role_justification: 'Chart represents complex data visualization', implicit_role_override: true }
          ],
          aria_states: [
            { property_name: 'aria-selected', property_values: ['true', 'false'], update_triggers: ['click', 'keyboard_selection'], announcement_strategy: 'immediate' }
          ]
        },
        semantic_markup: {
          heading_structure: {
            heading_levels: [
              { level: 1, usage_context: ['page_title'], styling_approach: 'css_classes' },
              { level: 2, usage_context: ['chart_title'], styling_approach: 'css_classes' },
              { level: 3, usage_context: ['legend_title', 'filter_title'], styling_approach: 'css_classes' }
            ],
            heading_hierarchy: true,
            skip_level_allowed: false
          },
          landmark_regions: [
            { region_type: 'main', region_label: 'Drift Analysis Dashboard', unique_identification: true },
            { region_type: 'navigation', region_label: 'Chart Controls', unique_identification: true }
          ],
          list_structures: [
            { list_type: 'unordered', nesting_allowed: true, semantic_meaning: 'Legend items and filter options' }
          ]
        },
        dynamic_content: {
          live_regions: [
            { region_selector: '.status-updates', politeness_level: 'polite', atomic_updates: true, relevant_changes: ['additions', 'text'] },
            { region_selector: '.error-messages', politeness_level: 'assertive', atomic_updates: true, relevant_changes: ['additions', 'removals'] }
          ],
          content_updates: [
            { update_type: 'addition', announcement_strategy: 'announce_count_and_type', timing_considerations: ['debounce_rapid_updates'] },
            { update_type: 'modification', announcement_strategy: 'announce_change_summary', timing_considerations: ['batch_similar_changes'] }
          ],
          progressive_disclosure: {
            disclosure_patterns: [
              { trigger_element: '.expand-details', disclosed_content: '.event-details', disclosure_state: 'aria-expanded', announcement_text: 'Event details {expanded|collapsed}' }
            ],
            state_communication: ['aria-expanded', 'aria-controls'],
            navigation_preservation: true
          }
        }
      },
      visual_accessibility: {
        color_contrast: {
          contrast_ratios: [
            { element_type: 'chart_text', minimum_ratio: 4.5, target_ratio: 7, measurement_method: 'automated_tool' },
            { element_type: 'chart_background', minimum_ratio: 4.5, target_ratio: 7, measurement_method: 'automated_tool' }
          ],
          automated_checking: true,
          manual_verification: true
        },
        text_scaling: {
          supported_zoom_levels: [100, 125, 150, 200],
          layout_adaptation: true,
          content_reflow: true,
          functionality_preservation: true
        },
        motion_preferences: {
          respect_reduced_motion: true,
          motion_alternatives: [
            { motion_type: 'chart_animations', static_alternative: 'instant_transitions', reduced_motion_version: 'slow_fade' }
          ],
          user_controls: [
            { control_type: 'animation_toggle', control_location: 'accessibility_menu', default_state: 'enabled', persistence: true }
          ]
        },
        visual_alternatives: [
          { visual_element: 'color_coding', alternative_type: 'text', alternative_content: 'severity_labels', equivalency_level: 'full' },
          { visual_element: 'trend_arrows', alternative_type: 'text', alternative_content: 'trend_descriptions', equivalency_level: 'full' }
        ]
      }
    }
  }
];

// Export individual strategies for specific use cases
export const DRIFT_TIMELINE_STRATEGY = PHOTON_DRIFT_VISUALIZATIONS[0];
// TODO: Additional strategies would be defined here
// export const HEALTH_METRICS_STRATEGY = ...;
// export const TEAM_PRODUCTIVITY_STRATEGY = ...;