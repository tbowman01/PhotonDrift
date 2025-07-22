// Enhanced data models for PhotonDrift Visual Analytics Dashboard
// Designed for advanced ML integration and comprehensive visualizations

export interface EnhancedDriftEvent extends DriftEvent {
  // Enhanced ML scoring
  mlAnalysis: MLAnalysisResults;
  
  // Visualization-specific properties
  visualMetadata: VisualMetadata;
  
  // Trend and temporal data
  historicalContext: HistoricalContext;
  
  // Impact assessment
  impact: ImpactAssessment;
  
  // Related events and dependencies
  relationships: EventRelationships;
}

export interface MLAnalysisResults {
  // Core ML scores
  confidence: number; // 0-1
  severity_prediction: number; // 0-1
  priority_score: number; // 0-1
  
  // Multiple model predictions
  ensemble_scores: {
    model_name: string;
    score: number;
    weight: number;
  }[];
  
  // Feature importance
  feature_importance: {
    [feature_name: string]: number;
  };
  
  // Uncertainty quantification
  uncertainty: {
    epistemic: number; // model uncertainty
    aleatoric: number; // data uncertainty
    total: number;
  };
  
  // Explanation data for visualizations
  explanations: {
    shap_values: number[];
    feature_names: string[];
    base_value: number;
  };
  
  // Model metadata
  model_version: string;
  prediction_timestamp: Date;
  processing_time: number; // milliseconds
}

export interface VisualMetadata {
  // Color coding
  severity_color: string;
  confidence_color: string;
  trend_color: string;
  
  // Chart positioning
  chart_coordinates: {
    x: number;
    y: number;
    z?: number; // for 3D visualizations
  };
  
  // Size and scaling
  visual_weight: number; // for bubble charts, heatmaps
  opacity: number; // based on confidence
  
  // Animation properties
  animation_delay: number;
  transition_duration: number;
  
  // Interactive properties
  clickable: boolean;
  draggable: boolean;
  resizable: boolean;
  
  // Grouping and clustering
  cluster_id?: string;
  group_label?: string;
}

export interface HistoricalContext {
  // Time series data
  occurrence_frequency: TimeSeriesData;
  resolution_history: ResolutionHistory[];
  
  // Temporal patterns
  seasonal_patterns: SeasonalPattern[];
  trend_direction: 'increasing' | 'decreasing' | 'stable' | 'cyclical';
  trend_strength: number; // 0-1
  
  // Comparison with similar events
  similar_events: SimilarEvent[];
  
  // Lifecycle stage
  lifecycle_stage: 'emerging' | 'peak' | 'declining' | 'resolved';
  
  // Prediction windows
  next_occurrence_prediction: {
    probability: number;
    time_window: DateRange;
    confidence_interval: [number, number];
  };
}

export interface TimeSeriesData {
  data_points: TimePoint[];
  sampling_interval: string; // '1h', '1d', '1w', etc.
  aggregation_method: 'sum' | 'avg' | 'count' | 'max' | 'min';
  interpolation_method: 'linear' | 'cubic' | 'step';
}

export interface TimePoint {
  timestamp: Date;
  value: number;
  confidence?: number;
  metadata?: { [key: string]: any };
}

export interface ResolutionHistory {
  resolved_at: Date;
  resolution_method: string;
  resolver: string;
  time_to_resolve: number; // hours
  effectiveness_score: number; // 0-1
  recurrence_time?: number; // hours until next occurrence
}

export interface SeasonalPattern {
  pattern_type: 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly';
  peak_times: string[]; // time expressions like "14:00", "Monday", "January"
  strength: number; // 0-1
  confidence: number; // 0-1
}

export interface SimilarEvent {
  event_id: string;
  similarity_score: number; // 0-1
  similarity_features: string[];
  outcome_comparison: OutcomeComparison;
}

export interface OutcomeComparison {
  resolution_time_ratio: number;
  success_probability: number;
  effort_required_ratio: number;
}

export interface ImpactAssessment {
  // Quantified impact metrics
  business_impact: BusinessImpact;
  technical_impact: TechnicalImpact;
  team_impact: TeamImpact;
  
  // Overall risk scores
  risk_score: number; // 0-100
  urgency_score: number; // 0-100
  complexity_score: number; // 0-100
  
  // Dependencies and cascading effects
  dependent_systems: string[];
  cascading_risk: number; // 0-1
  
  // Cost implications
  estimated_cost: CostEstimate;
}

export interface BusinessImpact {
  revenue_impact: number; // monetary value
  user_impact: number; // number of affected users
  availability_impact: number; // percentage downtime risk
  compliance_risk: number; // 0-1
  reputation_risk: number; // 0-1
}

export interface TechnicalImpact {
  performance_degradation: number; // percentage
  maintainability_score: number; // 0-1
  security_risk: number; // 0-1
  scalability_impact: number; // 0-1
  code_quality_impact: number; // 0-1
}

export interface TeamImpact {
  estimated_hours: number;
  skill_requirements: string[];
  team_members_affected: number;
  knowledge_transfer_needed: boolean;
  training_required: string[];
}

export interface CostEstimate {
  development_hours: number;
  development_cost: number;
  opportunity_cost: number;
  risk_mitigation_cost: number;
  total_estimated_cost: number;
  confidence_interval: [number, number];
}

export interface EventRelationships {
  // Direct relationships
  parent_events: string[]; // events that caused this one
  child_events: string[]; // events caused by this one
  
  // Correlation relationships
  correlated_events: CorrelatedEvent[];
  
  // Anti-patterns and pattern clusters
  pattern_membership: PatternMembership[];
  
  // Dependency graph
  dependency_graph: DependencyNode[];
}

export interface CorrelatedEvent {
  event_id: string;
  correlation_strength: number; // -1 to 1
  correlation_type: 'causal' | 'temporal' | 'spatial' | 'categorical';
  lag_time?: number; // milliseconds between events
}

export interface PatternMembership {
  pattern_id: string;
  pattern_name: string;
  pattern_type: 'anti-pattern' | 'code-smell' | 'architectural-issue' | 'process-issue';
  membership_strength: number; // 0-1
  pattern_frequency: number; // how often this pattern occurs
}

export interface DependencyNode {
  node_id: string;
  node_type: 'file' | 'module' | 'service' | 'team' | 'process';
  dependency_strength: number; // 0-1
  dependency_direction: 'incoming' | 'outgoing' | 'bidirectional';
  risk_propagation: number; // 0-1
}

// Enhanced Architecture Health with ML predictions and trends
export interface EnhancedArchitectureHealth extends ArchitectureHealth {
  // Detailed metric breakdown
  detailed_metrics: DetailedHealthMetrics;
  
  // Predictive analysis
  predictions: HealthPredictions;
  
  // Benchmarking
  benchmarks: HealthBenchmarks;
  
  // Recommendations
  recommendations: HealthRecommendation[];
  
  // Historical analysis
  health_history: HealthHistoryPoint[];
}

export interface DetailedHealthMetrics {
  // Code quality metrics
  code_quality: {
    complexity: number;
    duplication: number;
    test_coverage: number;
    documentation_coverage: number;
    code_smells: number;
  };
  
  // Architecture metrics
  architecture: {
    modularity: number;
    coupling: number;
    cohesion: number;
    dependency_health: number;
    layering_compliance: number;
  };
  
  // Process metrics
  process: {
    adr_compliance: number;
    decision_velocity: number;
    review_efficiency: number;
    change_success_rate: number;
    rollback_frequency: number;
  };
  
  // Team metrics
  team: {
    knowledge_distribution: number;
    collaboration_index: number;
    onboarding_efficiency: number;
    expert_dependency: number;
    communication_quality: number;
  };
  
  // Security metrics
  security: {
    vulnerability_count: number;
    security_debt: number;
    compliance_score: number;
    access_control_health: number;
    data_protection_score: number;
  };
}

export interface HealthPredictions {
  // Short-term predictions (1-4 weeks)
  short_term: HealthForecast;
  
  // Medium-term predictions (1-3 months)
  medium_term: HealthForecast;
  
  // Long-term predictions (3-12 months)
  long_term: HealthForecast;
  
  // Scenario analysis
  scenarios: ScenarioAnalysis[];
}

export interface HealthForecast {
  predicted_score: number;
  confidence_interval: [number, number];
  key_factors: ForecastFactor[];
  risk_factors: RiskFactor[];
  improvement_opportunities: ImprovementOpportunity[];
}

export interface ForecastFactor {
  factor_name: string;
  impact_weight: number;
  current_trend: 'positive' | 'negative' | 'neutral';
  expected_change: number;
}

export interface RiskFactor {
  risk_name: string;
  probability: number;
  impact_severity: number;
  time_to_impact: number; // days
  mitigation_options: string[];
}

export interface ImprovementOpportunity {
  opportunity_name: string;
  potential_gain: number;
  effort_required: number;
  timeline: string;
  prerequisites: string[];
  success_probability: number;
}

export interface ScenarioAnalysis {
  scenario_name: string;
  scenario_description: string;
  probability: number;
  health_impact: number;
  timeline: string;
  mitigation_strategies: string[];
}

export interface HealthBenchmarks {
  // Industry benchmarks
  industry_percentile: number;
  industry_average: number;
  industry_best_practice: number;
  
  // Peer comparisons
  peer_ranking: number;
  peer_count: number;
  peer_average: number;
  
  // Historical benchmarks
  best_historical_score: number;
  worst_historical_score: number;
  average_historical_score: number;
  
  // Goal tracking
  target_score: number;
  progress_to_target: number;
  estimated_time_to_target: number; // days
}

export interface HealthRecommendation {
  recommendation_id: string;
  category: 'quick-win' | 'strategic' | 'foundational' | 'emergency';
  title: string;
  description: string;
  
  // Impact and effort
  expected_impact: number; // health score improvement
  effort_required: number; // person-hours
  implementation_time: string; // timeline
  
  // Prioritization
  priority_score: number; // 0-100
  urgency: 'low' | 'medium' | 'high' | 'critical';
  
  // Implementation details
  action_items: ActionItem[];
  prerequisites: string[];
  risks: string[];
  success_criteria: string[];
  
  // Tracking
  status: 'suggested' | 'approved' | 'in-progress' | 'completed' | 'rejected';
  assigned_to?: string;
  estimated_completion?: Date;
}

export interface ActionItem {
  id: string;
  description: string;
  estimated_hours: number;
  required_skills: string[];
  dependencies: string[];
  completed: boolean;
}

export interface HealthHistoryPoint {
  timestamp: Date;
  overall_score: number;
  metric_scores: { [metric_name: string]: number };
  events: HealthEvent[];
  context: HealthContextData;
}

export interface HealthEvent {
  event_type: 'improvement' | 'degradation' | 'milestone' | 'incident';
  description: string;
  impact_score: number;
  related_metrics: string[];
}

export interface HealthContextData {
  team_size: number;
  codebase_size: number;
  active_projects: number;
  major_releases: number;
  external_factors: string[];
}

// Enhanced Team Metrics with detailed productivity and collaboration insights
export interface EnhancedTeamMetrics extends TeamMetrics {
  // Advanced productivity metrics
  productivity: ProductivityMetrics;
  
  // Collaboration analysis
  collaboration: CollaborationMetrics;
  
  // Knowledge management
  knowledge: KnowledgeMetrics;
  
  // Performance trends
  performance_trends: PerformanceTrend[];
  
  // Team health indicators
  team_health: TeamHealthIndicators;
  
  // Individual contributions (anonymized)
  individual_insights: IndividualInsights[];
}

export interface ProductivityMetrics {
  // Core productivity measures
  velocity: VelocityMetrics;
  quality: QualityMetrics;
  efficiency: EfficiencyMetrics;
  innovation: InnovationMetrics;
  
  // Productivity factors
  blockers: BlockerAnalysis;
  focus_time: FocusTimeAnalysis;
  context_switching: ContextSwitchingAnalysis;
}

export interface VelocityMetrics {
  story_points_per_sprint: TimeSeriesData;
  tasks_completed_per_day: TimeSeriesData;
  cycle_time: TimeSeriesData;
  lead_time: TimeSeriesData;
  throughput: TimeSeriesData;
  
  // Velocity consistency
  velocity_variance: number;
  predictability_score: number;
}

export interface QualityMetrics {
  defect_rate: TimeSeriesData;
  rework_percentage: number;
  code_review_effectiveness: number;
  test_coverage_trend: TimeSeriesData;
  customer_satisfaction: TimeSeriesData;
  
  // Quality indicators
  first_time_right: number;
  technical_debt_trend: TimeSeriesData;
}

export interface EfficiencyMetrics {
  // Time utilization
  coding_time_percentage: number;
  meeting_time_percentage: number;
  review_time_percentage: number;
  planning_time_percentage: number;
  
  // Waste identification
  waiting_time: number;
  rework_time: number;
  context_switch_overhead: number;
  
  // Flow metrics
  work_in_progress: TimeSeriesData;
  flow_efficiency: number;
}

export interface InnovationMetrics {
  experimental_projects: number;
  new_technologies_adopted: number;
  process_improvements_suggested: number;
  patents_or_publications: number;
  knowledge_sharing_sessions: number;
  
  // Innovation indicators
  innovation_time_percentage: number;
  idea_implementation_rate: number;
}

export interface BlockerAnalysis {
  total_blockers: number;
  average_blocking_time: number;
  blocker_categories: { [category: string]: number };
  blocker_trends: TimeSeriesData;
  resolution_effectiveness: number;
}

export interface FocusTimeAnalysis {
  daily_focus_hours: TimeSeriesData;
  interruption_frequency: TimeSeriesData;
  deep_work_sessions: TimeSeriesData;
  optimal_focus_hours: string[];
}

export interface ContextSwitchingAnalysis {
  switches_per_day: TimeSeriesData;
  switch_cost: number; // minutes lost per switch
  concurrent_projects: TimeSeriesData;
  multitasking_efficiency: number;
}

export interface CollaborationMetrics {
  // Communication patterns
  communication: CommunicationMetrics;
  
  // Knowledge sharing
  knowledge_sharing: KnowledgeSharingMetrics;
  
  // Team dynamics
  team_dynamics: TeamDynamicsMetrics;
  
  // Cross-functional collaboration
  cross_functional: CrossFunctionalMetrics;
}

export interface CommunicationMetrics {
  meeting_frequency: TimeSeriesData;
  meeting_effectiveness: number;
  response_time: TimeSeriesData;
  communication_clarity: number;
  
  // Communication channels
  channel_usage: { [channel: string]: number };
  preferred_communication_methods: string[];
}

export interface KnowledgeSharingMetrics {
  documentation_contributions: number;
  mentoring_hours: number;
  knowledge_sessions_led: number;
  cross_training_participation: number;
  
  // Knowledge distribution
  knowledge_centralization_index: number;
  bus_factor: number; // risk of knowledge loss
}

export interface TeamDynamicsMetrics {
  psychological_safety_score: number;
  trust_index: number;
  conflict_resolution_efficiency: number;
  decision_making_speed: number;
  
  // Team cohesion
  collaboration_frequency: { [member_pair: string]: number };
  team_cohesion_score: number;
}

export interface CrossFunctionalMetrics {
  cross_team_collaborations: number;
  stakeholder_satisfaction: number;
  external_communication_effectiveness: number;
  alignment_with_business_goals: number;
}

export interface KnowledgeMetrics {
  // Knowledge coverage
  domain_expertise_coverage: { [domain: string]: number };
  skill_distribution: SkillDistribution[];
  knowledge_gaps: KnowledgeGap[];
  
  // Learning and development
  learning_velocity: LearningVelocity;
  training_effectiveness: TrainingEffectiveness;
  
  // Knowledge retention
  knowledge_retention_rate: number;
  documentation_quality: number;
}

export interface SkillDistribution {
  skill_name: string;
  coverage_percentage: number;
  expertise_levels: { [level: string]: number };
  growth_trend: 'improving' | 'stable' | 'declining';
}

export interface KnowledgeGap {
  gap_name: string;
  criticality: 'low' | 'medium' | 'high' | 'critical';
  impact_assessment: string;
  recommended_actions: string[];
  timeline_to_fill: string;
}

export interface LearningVelocity {
  new_skills_acquired: TimeSeriesData;
  certification_achievements: number;
  training_hours: TimeSeriesData;
  skill_application_rate: number;
}

export interface TrainingEffectiveness {
  training_satisfaction: number;
  knowledge_retention: number;
  skill_application: number;
  performance_improvement: number;
  roi_on_training: number;
}

export interface PerformanceTrend {
  metric_name: string;
  trend_direction: 'improving' | 'stable' | 'declining';
  trend_strength: number; // 0-1
  trend_duration: number; // days
  
  // Statistical analysis
  correlation_factors: CorrelationFactor[];
  seasonality: SeasonalPattern[];
  anomalies: AnomalyPoint[];
}

export interface CorrelationFactor {
  factor_name: string;
  correlation_coefficient: number; // -1 to 1
  significance: number; // p-value
  causal_direction: 'causes' | 'caused_by' | 'correlated';
}

export interface AnomalyPoint {
  timestamp: Date;
  severity: 'minor' | 'moderate' | 'major';
  description: string;
  likely_causes: string[];
  impact_duration: number; // days
}

export interface TeamHealthIndicators {
  overall_health_score: number; // 0-100
  
  // Health dimensions
  workload_balance: number;
  stress_levels: number;
  job_satisfaction: number;
  work_life_balance: number;
  career_growth_satisfaction: number;
  
  // Risk indicators
  burnout_risk: BurnoutRisk[];
  turnover_risk: TurnoverRisk;
  performance_concerns: PerformanceConcern[];
  
  // Positive indicators
  engagement_score: number;
  motivation_level: number;
  team_spirit: number;
}

export interface BurnoutRisk {
  member_id: string; // anonymized
  risk_level: 'low' | 'medium' | 'high' | 'critical';
  contributing_factors: string[];
  recommended_interventions: string[];
  timeline: string;
}

export interface TurnoverRisk {
  overall_risk: number; // 0-1
  high_risk_members: number;
  contributing_factors: string[];
  retention_strategies: string[];
}

export interface PerformanceConcern {
  concern_type: string;
  severity: 'minor' | 'moderate' | 'significant';
  affected_members: number;
  recommended_actions: string[];
}

export interface IndividualInsights {
  member_id: string; // anonymized
  role: string;
  tenure: number; // months
  
  // Performance indicators
  productivity_score: number;
  quality_score: number;
  collaboration_score: number;
  growth_trajectory: 'accelerating' | 'steady' | 'plateauing';
  
  // Development areas
  strengths: string[];
  development_areas: string[];
  career_goals: string[];
  
  // Contributions
  unique_contributions: string[];
  mentorship_activities: string[];
  innovation_contributions: string[];
}

// Enhanced Trend Data Structures
export interface TrendData {
  // Core trend information
  metric_name: string;
  time_series: TimeSeriesData;
  
  // Statistical analysis
  statistics: TrendStatistics;
  
  // Forecasting
  forecasts: TrendForecast[];
  
  // Change detection
  change_points: ChangePoint[];
  
  // Comparative analysis
  comparisons: TrendComparison[];
  
  // Visualization metadata
  visualization_config: VisualizationConfig;
}

export interface TrendStatistics {
  // Basic statistics
  mean: number;
  median: number;
  standard_deviation: number;
  variance: number;
  
  // Trend characteristics
  trend_slope: number;
  trend_intercept: number;
  trend_r_squared: number;
  trend_p_value: number;
  
  // Seasonality
  seasonal_strength: number;
  seasonal_periods: number[];
  
  // Volatility
  volatility: number;
  max_drawdown: number;
  
  // Distribution properties
  skewness: number;
  kurtosis: number;
  normality_test_p_value: number;
}

export interface TrendForecast {
  forecast_horizon: number; // days
  forecast_method: 'linear' | 'exponential' | 'arima' | 'prophet' | 'ml_ensemble';
  
  // Forecast values
  point_forecast: number[];
  confidence_intervals: ConfidenceInterval[];
  prediction_intervals: PredictionInterval[];
  
  // Forecast quality
  forecast_accuracy: ForecastAccuracy;
  model_diagnostics: ModelDiagnostics;
}

export interface ConfidenceInterval {
  timestamp: Date;
  lower_bound: number;
  upper_bound: number;
  confidence_level: number; // e.g., 0.95 for 95%
}

export interface PredictionInterval {
  timestamp: Date;
  lower_bound: number;
  upper_bound: number;
  probability: number;
}

export interface ForecastAccuracy {
  mae: number; // mean absolute error
  mape: number; // mean absolute percentage error
  rmse: number; // root mean square error
  mase: number; // mean absolute scaled error
  accuracy_score: number; // 0-1
}

export interface ModelDiagnostics {
  model_fit_statistics: { [statistic: string]: number };
  residual_analysis: ResidualAnalysis;
  validation_results: ValidationResults;
}

export interface ResidualAnalysis {
  residual_autocorrelation: number[];
  residual_normality_p_value: number;
  heteroscedasticity_p_value: number;
  outliers: OutlierPoint[];
}

export interface ValidationResults {
  cross_validation_score: number;
  holdout_validation_score: number;
  walk_forward_validation: WalkForwardResult[];
}

export interface WalkForwardResult {
  training_end: Date;
  forecast_start: Date;
  forecast_accuracy: ForecastAccuracy;
}

export interface OutlierPoint {
  timestamp: Date;
  value: number;
  outlier_score: number;
  outlier_type: 'mild' | 'moderate' | 'extreme';
}

export interface ChangePoint {
  timestamp: Date;
  change_type: 'level' | 'trend' | 'variance';
  change_magnitude: number;
  change_significance: number; // p-value
  change_description: string;
  
  // Before and after characteristics
  before_statistics: TrendStatistics;
  after_statistics: TrendStatistics;
  
  // Potential causes
  potential_causes: PotentialCause[];
}

export interface PotentialCause {
  cause_description: string;
  likelihood: number; // 0-1
  evidence: string[];
  impact_assessment: string;
}

export interface TrendComparison {
  comparison_type: 'period_over_period' | 'benchmark' | 'peer_group' | 'target';
  comparison_target: string;
  
  // Comparison metrics
  absolute_difference: number;
  percentage_difference: number;
  statistical_significance: number; // p-value
  
  // Comparative analysis
  performance_ranking: number;
  percentile_rank: number;
  z_score: number;
  
  // Insights
  comparison_insights: string[];
  improvement_opportunities: string[];
}

// Visualization Configuration Types
export interface VisualizationConfig {
  // Chart type and configuration
  chart_type: ChartType;
  chart_config: ChartConfiguration;
  
  // Color and styling
  color_scheme: ColorScheme;
  styling: ChartStyling;
  
  // Interactivity
  interactions: InteractionConfig;
  
  // Performance optimization
  performance: PerformanceConfig;
  
  // Accessibility
  accessibility: AccessibilityConfig;
}

export type ChartType = 
  | 'line' | 'area' | 'bar' | 'column' | 'scatter' | 'bubble'
  | 'heatmap' | 'treemap' | 'sankey' | 'network' | 'radar'
  | 'candlestick' | 'box_plot' | 'violin_plot' | 'histogram'
  | 'gauge' | 'funnel' | 'waterfall' | 'bullet' | 'sparkline'
  | 'timeline' | 'gantt' | 'calendar' | 'geographic';

export interface ChartConfiguration {
  // Axes configuration
  x_axis: AxisConfig;
  y_axis: AxisConfig;
  y2_axis?: AxisConfig; // secondary y-axis
  
  // Series configuration
  series: SeriesConfig[];
  
  // Layout
  layout: LayoutConfig;
  
  // Annotations
  annotations: AnnotationConfig[];
  
  // Zoom and pan
  zoom_config: ZoomConfig;
}

export interface AxisConfig {
  type: 'linear' | 'logarithmic' | 'time' | 'category';
  title: string;
  unit: string;
  
  // Scale configuration
  min?: number;
  max?: number;
  step?: number;
  
  // Formatting
  format: string; // d3 format string
  tick_count: number;
  
  // Styling
  show_grid: boolean;
  grid_style: GridStyle;
  label_rotation?: number;
}

export interface GridStyle {
  color: string;
  width: number;
  style: 'solid' | 'dashed' | 'dotted';
  opacity: number;
}

export interface SeriesConfig {
  name: string;
  type: ChartType;
  data_field: string;
  
  // Styling
  color: string;
  opacity: number;
  line_width?: number;
  marker_size?: number;
  
  // Behavior
  visible: boolean;
  selectable: boolean;
  hoverable: boolean;
  
  // Axis binding
  y_axis: 'primary' | 'secondary';
  
  // Trend lines
  trend_line?: TrendLineConfig;
  
  // Data labels
  show_data_labels: boolean;
  data_label_format: string;
}

export interface TrendLineConfig {
  type: 'linear' | 'polynomial' | 'exponential' | 'moving_average';
  color: string;
  width: number;
  style: 'solid' | 'dashed' | 'dotted';
  
  // Moving average specific
  window_size?: number;
  
  // Polynomial specific
  degree?: number;
}

export interface LayoutConfig {
  width: number;
  height: number;
  margin: MarginConfig;
  
  // Title and labels
  title: string;
  subtitle?: string;
  
  // Legend
  legend: LegendConfig;
  
  // Background
  background_color: string;
  
  // Responsive behavior
  responsive: boolean;
  maintain_aspect_ratio: boolean;
}

export interface MarginConfig {
  top: number;
  right: number;
  bottom: number;
  left: number;
}

export interface LegendConfig {
  show: boolean;
  position: 'top' | 'bottom' | 'left' | 'right' | 'floating';
  floating_position?: { x: number; y: number };
  orientation: 'horizontal' | 'vertical';
  alignment: 'start' | 'center' | 'end';
}

export interface AnnotationConfig {
  type: 'text' | 'line' | 'rectangle' | 'circle' | 'arrow';
  content: string;
  position: AnnotationPosition;
  styling: AnnotationStyling;
  
  // Conditional display
  show_condition?: string; // expression to evaluate
  
  // Interactivity
  clickable: boolean;
  tooltip?: string;
}

export interface AnnotationPosition {
  x: number | string; // absolute pixel or data value
  y: number | string;
  anchor: 'start' | 'middle' | 'end';
}

export interface AnnotationStyling {
  color: string;
  background_color?: string;
  border_color?: string;
  font_size: number;
  font_weight: 'normal' | 'bold';
  opacity: number;
}

export interface ZoomConfig {
  enabled: boolean;
  type: 'x' | 'y' | 'xy';
  
  // Zoom limits
  min_zoom: number;
  max_zoom: number;
  
  // Pan limits
  pan_limits?: PanLimits;
  
  // Reset button
  show_reset_button: boolean;
}

export interface PanLimits {
  x_min?: number;
  x_max?: number;
  y_min?: number;
  y_max?: number;
}

export interface ColorScheme {
  type: 'categorical' | 'sequential' | 'diverging' | 'custom';
  name: string; // e.g., 'viridis', 'category10', 'rdylbu'
  
  // Custom colors
  colors?: string[];
  
  // Color mapping
  mapping: ColorMapping[];
  
  // Accessibility considerations
  colorblind_friendly: boolean;
  contrast_ratio: number;
}

export interface ColorMapping {
  value: string | number;
  color: string;
  description: string;
}

export interface ChartStyling {
  // Font configuration
  font_family: string;
  font_size_base: number;
  
  // Color palette
  primary_color: string;
  secondary_color: string;
  accent_color: string;
  
  // Border and spacing
  border_radius: number;
  border_width: number;
  padding: number;
  
  // Transparency
  default_opacity: number;
  hover_opacity: number;
  
  // Animation
  animation_duration: number;
  animation_easing: string;
}

export interface InteractionConfig {
  // Mouse interactions
  hover_effects: HoverEffects;
  click_behavior: ClickBehavior;
  selection: SelectionConfig;
  
  // Touch interactions
  touch_enabled: boolean;
  pinch_zoom: boolean;
  
  // Keyboard shortcuts
  keyboard_shortcuts: KeyboardShortcut[];
  
  // Cross-filtering
  cross_filter: CrossFilterConfig;
}

export interface HoverEffects {
  enabled: boolean;
  highlight_style: HighlightStyle;
  tooltip: TooltipConfig;
  cursor_style: string;
}

export interface HighlightStyle {
  opacity_change: number;
  size_change: number;
  color_change?: string;
  border_width_change: number;
}

export interface TooltipConfig {
  enabled: boolean;
  template: string; // HTML template with placeholders
  position: 'mouse' | 'fixed' | 'data_point';
  
  // Styling
  background_color: string;
  border_color: string;
  text_color: string;
  font_size: number;
  
  // Behavior
  show_delay: number; // milliseconds
  hide_delay: number;
  follow_mouse: boolean;
}

export interface ClickBehavior {
  enabled: boolean;
  action: 'select' | 'drill_down' | 'filter' | 'navigate' | 'custom';
  
  // Action parameters
  drill_down_target?: string;
  filter_field?: string;
  navigation_url?: string;
  custom_function?: string;
  
  // Multi-selection
  multi_select: boolean;
  select_modifier_key: 'ctrl' | 'shift' | 'alt' | 'meta';
}

export interface SelectionConfig {
  enabled: boolean;
  mode: 'single' | 'multiple' | 'range';
  
  // Visual feedback
  selection_style: SelectionStyle;
  
  // Programmatic selection
  initial_selection?: SelectionCriteria;
  persistent_selection: boolean;
}

export interface SelectionStyle {
  border_color: string;
  border_width: number;
  background_color: string;
  opacity: number;
}

export interface SelectionCriteria {
  field: string;
  operator: 'equals' | 'contains' | 'range';
  value: any;
}

export interface KeyboardShortcut {
  key_combination: string; // e.g., 'ctrl+z', 'shift+click'
  action: string;
  description: string;
}

export interface CrossFilterConfig {
  enabled: boolean;
  linked_charts: string[]; // chart IDs
  filter_behavior: 'highlight' | 'filter' | 'both';
  
  // Animation
  transition_duration: number;
  
  // Reset capability
  show_reset_button: boolean;
}

export interface PerformanceConfig {
  // Data handling
  max_data_points: number;
  data_sampling: SamplingConfig;
  
  // Rendering optimization
  canvas_rendering: boolean;
  progressive_loading: boolean;
  virtualization: boolean;
  
  // Caching
  cache_config: CacheConfig;
  
  // Lazy loading
  lazy_loading: boolean;
  viewport_buffer: number; // pixels
}

export interface SamplingConfig {
  enabled: boolean;
  method: 'uniform' | 'adaptive' | 'lttb'; // Largest Triangle Three Buckets
  target_points: number;
  
  // Adaptive sampling
  importance_threshold?: number;
  preserve_extremes: boolean;
}

export interface CacheConfig {
  enabled: boolean;
  cache_size: number; // MB
  cache_duration: number; // minutes
  cache_key_strategy: 'url' | 'data_hash' | 'custom';
}

export interface AccessibilityConfig {
  // WCAG compliance
  wcag_level: 'AA' | 'AAA';
  
  // Screen reader support
  aria_labels: { [element: string]: string };
  data_table_fallback: boolean;
  
  // Keyboard navigation
  keyboard_navigation: boolean;
  focus_indicators: boolean;
  
  // Visual accessibility
  high_contrast_mode: boolean;
  pattern_fills: boolean; // for colorblind users
  text_alternatives: TextAlternative[];
  
  // Motion and animation
  respect_prefers_reduced_motion: boolean;
  provide_static_alternative: boolean;
}

export interface TextAlternative {
  element: string;
  alternative_text: string;
  context: string;
}

// Mock Data Generation Types
export interface MockDataGenerator {
  // Generator configuration
  config: GeneratorConfig;
  
  // Data schemas
  schemas: DataSchema[];
  
  // Generation methods
  generators: DataGenerator[];
  
  // Output options
  output: OutputConfig;
}

export interface GeneratorConfig {
  seed: number; // for reproducible random data
  locale: string; // for locale-specific data
  date_range: DateRange;
  data_volume: DataVolume;
  
  // Realism settings
  correlation_strength: number; // how realistic correlations should be
  noise_level: number; // amount of random variation
  outlier_frequency: number; // frequency of outliers
  
  // Performance settings
  batch_size: number;
  parallel_generation: boolean;
}

export interface DataVolume {
  repositories: number;
  events_per_repo: number;
  team_members: number;
  time_period_days: number;
  scan_frequency_hours: number;
}

export interface DataSchema {
  schema_name: string;
  entity_type: string; // 'drift_event', 'health_metric', etc.
  
  // Field definitions
  fields: FieldDefinition[];
  
  // Constraints
  constraints: DataConstraint[];
  
  // Relationships
  relationships: SchemaRelationship[];
}

export interface FieldDefinition {
  field_name: string;
  field_type: 'string' | 'number' | 'boolean' | 'date' | 'array' | 'object';
  
  // Generation parameters
  generator_type: string; // 'faker', 'random', 'sequence', 'distribution'
  generator_params: { [key: string]: any };
  
  // Constraints
  nullable: boolean;
  unique: boolean;
  min_value?: number;
  max_value?: number;
  enum_values?: any[];
  
  // Patterns
  pattern?: string; // regex for string fields
  format?: string; // for dates, numbers
}

export interface DataConstraint {
  constraint_type: 'foreign_key' | 'check' | 'unique_composite' | 'conditional';
  fields: string[];
  constraint_expression: string;
  error_message: string;
}

export interface SchemaRelationship {
  related_schema: string;
  relationship_type: 'one_to_one' | 'one_to_many' | 'many_to_many';
  foreign_key_fields: string[];
  cascade_delete: boolean;
}

export interface DataGenerator {
  generator_name: string;
  generator_type: string;
  
  // Generation logic
  generation_function: string; // function name or code
  dependencies: string[]; // other generators this depends on
  
  // Caching
  cache_results: boolean;
  cache_duration: number;
}

export interface OutputConfig {
  // File formats
  formats: OutputFormat[];
  
  // File organization
  output_directory: string;
  file_naming_pattern: string;
  
  // Compression
  compression: 'none' | 'gzip' | 'brotli';
  
  // Streaming
  streaming_output: boolean;
  chunk_size: number;
}

export interface OutputFormat {
  format_type: 'json' | 'csv' | 'parquet' | 'sql' | 'typescript';
  file_extension: string;
  
  // Format-specific options
  json_pretty_print?: boolean;
  csv_delimiter?: string;
  sql_dialect?: 'postgresql' | 'mysql' | 'sqlite';
  typescript_interfaces_only?: boolean;
}

// KPI and Analytics Types
export interface KPIDefinition {
  kpi_id: string;
  name: string;
  description: string;
  category: 'health' | 'productivity' | 'quality' | 'risk' | 'business';
  
  // Calculation
  calculation: KPICalculation;
  
  // Targets and thresholds
  targets: KPITarget[];
  thresholds: KPIThreshold[];
  
  // Visualization
  default_visualization: VisualizationConfig;
  dashboard_priority: number;
  
  // Business context
  business_impact: string;
  stakeholders: string[];
  update_frequency: string;
}

export interface KPICalculation {
  calculation_type: 'simple' | 'weighted' | 'composite' | 'ml_model';
  
  // Simple calculation
  formula?: string; // mathematical expression
  source_fields: string[];
  
  // Weighted calculation
  weights?: { [field: string]: number };
  
  // Composite calculation
  sub_kpis?: string[]; // other KPI IDs
  aggregation_method?: 'sum' | 'avg' | 'weighted_avg' | 'max' | 'min';
  
  // ML model calculation
  model_id?: string;
  feature_fields?: string[];
  
  // Common parameters
  time_window: string; // '1d', '1w', '1m', etc.
  aggregation_level: 'repository' | 'team' | 'organization';
}

export interface KPITarget {
  target_type: 'absolute' | 'relative' | 'percentile';
  target_value: number;
  target_period: string;
  
  // Context
  target_description: string;
  achievement_probability: number;
  
  // Time-based targets
  milestone_targets?: MilestoneTarget[];
}

export interface MilestoneTarget {
  milestone_date: Date;
  target_value: number;
  milestone_description: string;
}

export interface KPIThreshold {
  threshold_type: 'warning' | 'critical' | 'excellent' | 'good';
  threshold_value: number;
  threshold_operator: 'greater_than' | 'less_than' | 'equals' | 'between';
  
  // Actions
  alert_config?: AlertConfig;
  automated_actions?: AutomatedAction[];
}

export interface AlertConfig {
  alert_severity: 'info' | 'warning' | 'error' | 'critical';
  notification_channels: string[];
  escalation_rules: EscalationRule[];
  
  // Throttling
  max_alerts_per_hour: number;
  suppression_duration: number; // minutes
}

export interface EscalationRule {
  escalation_level: number;
  time_threshold: number; // minutes before escalating
  escalation_targets: string[];
  escalation_message: string;
}

export interface AutomatedAction {
  action_type: string;
  action_parameters: { [key: string]: any };
  execution_delay: number; // minutes
  
  // Conditions
  additional_conditions?: string[];
  max_executions_per_day?: number;
}

// Advanced Analytics Types
export interface HealthScore {
  // Overall health score (0-100)
  overall_score: number;
  score_timestamp: Date;
  
  // Component scores
  component_scores: ComponentScore[];
  
  // Score calculation details
  calculation_metadata: ScoreCalculationMetadata;
  
  // Historical context
  historical_percentile: number;
  trend_analysis: ScoreTrendAnalysis;
  
  // Benchmarking
  benchmark_comparisons: BenchmarkComparison[];
  
  // Recommendations
  improvement_recommendations: ScoreImprovement[];
}

export interface ComponentScore {
  component_name: string;
  weight: number; // contribution to overall score
  score: number; // 0-100
  
  // Sub-components
  sub_components: ComponentScore[];
  
  // Contributing factors
  positive_factors: ScoreFactor[];
  negative_factors: ScoreFactor[];
  
  // Trend
  trend_direction: 'improving' | 'stable' | 'declining';
  trend_velocity: number;
}

export interface ScoreFactor {
  factor_name: string;
  impact: number; // positive or negative points
  confidence: number; // 0-1
  evidence: string[];
  
  // Actionability
  actionable: boolean;
  recommended_actions?: string[];
}

export interface ScoreCalculationMetadata {
  calculation_version: string;
  calculation_timestamp: Date;
  data_sources: DataSource[];
  calculation_duration: number; // milliseconds
  
  // Quality indicators
  data_completeness: number; // 0-1
  data_freshness: number; // hours since last update
  calculation_confidence: number; // 0-1
  
  // Model information
  model_versions: { [component: string]: string };
  feature_importance: { [feature: string]: number };
}

export interface DataSource {
  source_name: string;
  source_type: 'database' | 'api' | 'file' | 'calculation';
  last_updated: Date;
  record_count: number;
  quality_score: number; // 0-1
}

export interface ScoreTrendAnalysis {
  // Trend characteristics
  trend_slope: number; // points per day
  trend_acceleration: number; // change in slope
  trend_consistency: number; // 0-1, how consistent the trend is
  
  // Statistical measures
  volatility: number;
  mean_reversion_tendency: number;
  
  // Predictions
  short_term_forecast: ScoreForecast;
  medium_term_forecast: ScoreForecast;
  long_term_forecast: ScoreForecast;
}

export interface ScoreForecast {
  time_horizon: number; // days
  predicted_score: number;
  confidence_interval: [number, number];
  
  // Scenario analysis
  optimistic_scenario: number;
  pessimistic_scenario: number;
  most_likely_scenario: number;
  
  // Key assumptions
  assumptions: string[];
  risk_factors: string[];
}

export interface BenchmarkComparison {
  benchmark_type: 'industry' | 'peer_group' | 'best_practice' | 'historical';
  benchmark_name: string;
  
  // Comparison results
  relative_performance: number; // percentile
  score_gap: number; // points difference
  
  // Context
  benchmark_sample_size: number;
  benchmark_date: Date;
  benchmark_methodology: string;
  
  // Insights
  performance_insights: string[];
  improvement_opportunities: string[];
}

export interface ScoreImprovement {
  improvement_id: string;
  category: 'quick_win' | 'strategic' | 'transformational';
  
  // Impact
  potential_score_improvement: number; // points
  confidence_in_improvement: number; // 0-1
  
  // Effort
  estimated_effort: EffortEstimate;
  implementation_complexity: 'low' | 'medium' | 'high';
  
  // Implementation
  action_plan: ActionPlan;
  prerequisites: string[];
  risks: Risk[];
  
  // Monitoring
  success_metrics: SuccessMetric[];
  monitoring_plan: MonitoringPlan;
}

export interface EffortEstimate {
  person_hours: number;
  person_months?: number;
  cost_estimate?: number;
  resource_requirements: ResourceRequirement[];
}

export interface ResourceRequirement {
  resource_type: 'human' | 'technical' | 'financial';
  requirement_description: string;
  quantity_needed: number;
  duration_needed: string;
  criticality: 'essential' | 'important' | 'nice_to_have';
}

export interface ActionPlan {
  phases: ActionPhase[];
  dependencies: PhaseDependency[];
  critical_path: string[]; // phase IDs
  
  // Timeline
  estimated_duration: number; // days
  start_date?: Date;
  target_completion?: Date;
  
  // Resources
  required_roles: string[];
  budget_requirements?: number;
}

export interface ActionPhase {
  phase_id: string;
  phase_name: string;
  description: string;
  
  // Timeline
  estimated_duration: number; // days
  earliest_start?: Date;
  latest_finish?: Date;
  
  // Tasks
  tasks: PhaseTask[];
  
  // Resources
  required_resources: ResourceRequirement[];
  
  // Deliverables
  deliverables: string[];
  success_criteria: string[];
}

export interface PhaseTask {
  task_id: string;
  task_name: string;
  description: string;
  estimated_hours: number;
  
  // Assignment
  required_skills: string[];
  assigned_to?: string;
  
  // Dependencies
  depends_on: string[]; // task IDs
  
  // Status
  status: 'not_started' | 'in_progress' | 'completed' | 'blocked';
  completion_percentage: number;
}

export interface PhaseDependency {
  predecessor_phase: string;
  successor_phase: string;
  dependency_type: 'finish_to_start' | 'start_to_start' | 'finish_to_finish' | 'start_to_finish';
  lag_time?: number; // days
}

export interface Risk {
  risk_id: string;
  risk_description: string;
  risk_category: 'technical' | 'resource' | 'timeline' | 'stakeholder' | 'external';
  
  // Assessment
  probability: number; // 0-1
  impact_severity: number; // 0-10
  risk_score: number; // probability * impact
  
  // Mitigation
  mitigation_strategies: MitigationStrategy[];
  contingency_plans: ContingencyPlan[];
  
  // Monitoring
  early_warning_indicators: string[];
  review_frequency: string;
}

export interface MitigationStrategy {
  strategy_description: string;
  implementation_cost: number;
  effectiveness: number; // 0-1
  implementation_timeline: string;
}

export interface ContingencyPlan {
  trigger_conditions: string[];
  response_actions: string[];
  resource_requirements: ResourceRequirement[];
  activation_timeline: string;
}

export interface SuccessMetric {
  metric_name: string;
  metric_description: string;
  target_value: number;
  measurement_method: string;
  measurement_frequency: string;
  
  // Thresholds
  minimum_acceptable: number;
  stretch_target: number;
  
  // Tracking
  baseline_value?: number;
  current_value?: number;
  trend: 'on_track' | 'at_risk' | 'off_track';
}

export interface MonitoringPlan {
  monitoring_frequency: string;
  key_checkpoints: MonitoringCheckpoint[];
  reporting_schedule: ReportingSchedule[];
  escalation_triggers: EscalationTrigger[];
  
  // Reviews
  review_meetings: ReviewMeeting[];
  stakeholder_communication: CommunicationPlan[];
}

export interface MonitoringCheckpoint {
  checkpoint_date: Date;
  checkpoint_name: string;
  success_criteria: string[];
  deliverables_due: string[];
  review_participants: string[];
}

export interface ReportingSchedule {
  report_type: string;
  frequency: string;
  recipients: string[];
  format: 'dashboard' | 'email' | 'presentation' | 'document';
  automated: boolean;
}

export interface EscalationTrigger {
  trigger_condition: string;
  escalation_level: number;
  escalation_timeline: string; // how quickly to escalate
  escalation_recipients: string[];
  required_actions: string[];
}

export interface ReviewMeeting {
  meeting_type: string;
  frequency: string;
  participants: string[];
  agenda_template: string[];
  decision_making_authority: string;
}

export interface CommunicationPlan {
  audience: string;
  message_type: string;
  frequency: string;
  communication_channel: string;
  responsible_party: string;
  
  // Content
  key_messages: string[];
  success_stories: boolean;
  challenges_discussion: boolean;
  next_steps_focus: boolean;
}

// Import base types for extension
import { 
  DriftEvent, ArchitectureHealth, TeamMetrics, DateRange,
  // Add other imported base types as needed
} from './types';