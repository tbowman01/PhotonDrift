// Key Performance Indicators (KPIs) for PhotonDrift Analytics Dashboard
// Comprehensive metrics for measuring architecture health and team productivity

import { KPIDefinition, KPICalculation, KPITarget, KPIThreshold } from './enhanced-types';

export interface PhotonDriftKPI extends KPIDefinition {
  // Additional PhotonDrift specific properties
  ml_enhanced: boolean;
  drift_specific: boolean;
  real_time_updates: boolean;
  alert_thresholds: KPIThreshold[];
}

// Core Architecture Health KPIs
export const ARCHITECTURE_HEALTH_KPIS: PhotonDriftKPI[] = [
  {
    kpi_id: 'overall_health_score',
    name: 'Overall Architecture Health Score',
    description: 'Composite score (0-100) representing overall architecture quality and maintainability',
    category: 'health',
    ml_enhanced: true,
    drift_specific: true,
    real_time_updates: true,
    
    calculation: {
      calculation_type: 'composite',
      sub_kpis: [
        'drift_density_score',
        'resolution_velocity_score', 
        'technical_debt_score',
        'ml_confidence_score',
        'trend_stability_score'
      ],
      aggregation_method: 'weighted_avg',
      weights: {
        'drift_density_score': 0.25,
        'resolution_velocity_score': 0.20,
        'technical_debt_score': 0.25,
        'ml_confidence_score': 0.15,
        'trend_stability_score': 0.15
      },
      time_window: '30d',
      aggregation_level: 'repository',
      source_fields: [],
      formula: '(drift_density * 0.25) + (resolution_velocity * 0.20) + (technical_debt * 0.25) + (ml_confidence * 0.15) + (trend_stability * 0.15)'
    },
    
    targets: [
      {
        target_type: 'absolute',
        target_value: 85,
        target_period: '30d',
        target_description: 'Maintain architecture health above 85 points',
        achievement_probability: 0.75,
        milestone_targets: [
          { milestone_date: new Date('2025-08-01'), target_value: 80, milestone_description: 'Baseline establishment' },
          { milestone_date: new Date('2025-09-01'), target_value: 82, milestone_description: 'Initial improvements' },
          { milestone_date: new Date('2025-10-01'), target_value: 85, milestone_description: 'Target achievement' }
        ]
      }
    ],
    
    thresholds: [
      {
        threshold_type: 'critical',
        threshold_value: 60,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'critical',
          notification_channels: ['slack', 'email', 'dashboard'],
          escalation_rules: [
            {
              escalation_level: 1,
              time_threshold: 15,
              escalation_targets: ['team_lead', 'architect'],
              escalation_message: 'Architecture health critically low - immediate attention required'
            }
          ],
          max_alerts_per_hour: 2,
          suppression_duration: 60
        }
      },
      {
        threshold_type: 'warning',
        threshold_value: 75,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'warning',
          notification_channels: ['dashboard', 'email'],
          escalation_rules: [],
          max_alerts_per_hour: 1,
          suppression_duration: 120
        }
      },
      {
        threshold_type: 'excellent',
        threshold_value: 90,
        threshold_operator: 'greater_than'
      }
    ],
    
    alert_thresholds: [
      {
        threshold_type: 'critical',
        threshold_value: 60,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'critical',
          notification_channels: ['slack', 'email'],
          escalation_rules: [{
            escalation_level: 1,
            time_threshold: 15,
            escalation_targets: ['architecture_team'],
            escalation_message: 'Critical architecture health degradation detected'
          }],
          max_alerts_per_hour: 2,
          suppression_duration: 60
        }
      }
    ],
    
    default_visualization: {
      chart_type: 'gauge',
      chart_config: {
        x_axis: { type: 'linear', title: '', unit: '', format: '', tick_count: 0, show_grid: false, grid_style: { color: '', width: 0, style: 'solid', opacity: 0 } },
        y_axis: { type: 'linear', title: '', unit: '', format: '', tick_count: 0, show_grid: false, grid_style: { color: '', width: 0, style: 'solid', opacity: 0 } },
        series: [{
          name: 'Health Score',
          type: 'gauge',
          data_field: 'overall_score',
          color: '#28a745',
          opacity: 1,
          visible: true,
          selectable: false,
          hoverable: true,
          y_axis: 'primary',
          show_data_labels: true,
          data_label_format: '.0f'
        }],
        layout: {
          width: 300,
          height: 200,
          margin: { top: 20, right: 20, bottom: 20, left: 20 },
          title: 'Architecture Health',
          background_color: '#ffffff',
          responsive: true,
          maintain_aspect_ratio: true,
          legend: { show: false, position: 'bottom', orientation: 'horizontal', alignment: 'center' }
        },
        annotations: [{
          type: 'text',
          content: 'Target: 85',
          position: { x: '50%', y: '80%', anchor: 'middle' },
          styling: { color: '#6c757d', font_size: 12, font_weight: 'normal', opacity: 0.8 },
          clickable: false
        }],
        zoom_config: { enabled: false, type: 'x', min_zoom: 1, max_zoom: 1, show_reset_button: false }
      },
      color_scheme: {
        type: 'sequential',
        name: 'RdYlGn',
        colors: ['#d73027', '#f46d43', '#fdae61', '#fee08b', '#d9ef8b', '#a6d96a', '#66bd63', '#1a9850'],
        mapping: [],
        colorblind_friendly: true,
        contrast_ratio: 4.5
      },
      styling: {
        font_family: 'Inter, system-ui, sans-serif',
        font_size_base: 14,
        primary_color: '#007bff',
        secondary_color: '#6c757d',
        accent_color: '#28a745',
        border_radius: 4,
        border_width: 1,
        padding: 16,
        default_opacity: 1,
        hover_opacity: 0.8,
        animation_duration: 300,
        animation_easing: 'ease-out'
      },
      interactions: {
        hover_effects: {
          enabled: true,
          highlight_style: { opacity_change: 0.2, size_change: 0, border_width_change: 1 },
          tooltip: {
            enabled: true,
            template: '<strong>Health Score:</strong> {{value}}<br><strong>Status:</strong> {{status}}',
            position: 'mouse',
            background_color: '#000000',
            border_color: '#ffffff',
            text_color: '#ffffff',
            font_size: 12,
            show_delay: 0,
            hide_delay: 0,
            follow_mouse: true
          },
          cursor_style: 'pointer'
        },
        click_behavior: {
          enabled: true,
          action: 'drill_down',
          drill_down_target: 'health_details',
          multi_select: false,
          select_modifier_key: 'ctrl'
        },
        selection: { enabled: false, mode: 'single', selection_style: { border_color: '', border_width: 0, background_color: '', opacity: 0 }, persistent_selection: false },
        touch_enabled: true,
        pinch_zoom: true,
        keyboard_shortcuts: [],
        cross_filter: { enabled: false, linked_charts: [], filter_behavior: 'filter', transition_duration: 300, show_reset_button: false }
      },
      performance: {
        max_data_points: 1,
        data_sampling: { enabled: false, method: 'uniform', target_points: 1000, preserve_extremes: true },
        canvas_rendering: false,
        progressive_loading: false,
        virtualization: false,
        cache_config: { enabled: true, cache_size: 10, cache_duration: 300, cache_key_strategy: 'data_hash' },
        lazy_loading: false,
        viewport_buffer: 0
      },
      accessibility: {
        wcag_level: 'AA',
        aria_labels: { 'gauge': 'Architecture health score gauge' },
        data_table_fallback: true,
        keyboard_navigation: true,
        focus_indicators: true,
        high_contrast_mode: false,
        pattern_fills: true,
        text_alternatives: [{ element: 'gauge', alternative_text: 'Current health score: {{value}} out of 100', context: 'health_dashboard' }],
        respect_prefers_reduced_motion: true,
        provide_static_alternative: true
      }
    },
    
    dashboard_priority: 1,
    business_impact: 'High - Directly correlates with maintenance costs, development velocity, and system reliability',
    stakeholders: ['Architecture Team', 'Engineering Managers', 'CTO', 'Development Teams'],
    update_frequency: 'real-time'
  },
  
  {
    kpi_id: 'drift_density_score',
    name: 'Drift Density Score',
    description: 'Score (0-100) based on number and severity of drift events per 1000 lines of code',
    category: 'risk',
    ml_enhanced: true,
    drift_specific: true,
    real_time_updates: true,
    
    calculation: {
      calculation_type: 'simple',
      formula: 'max(0, 100 - ((critical_drift * 10 + high_drift * 5 + medium_drift * 2 + low_drift * 1) / (lines_of_code / 1000)))',
      source_fields: ['critical_drift_count', 'high_drift_count', 'medium_drift_count', 'low_drift_count', 'lines_of_code'],
      time_window: '7d',
      aggregation_level: 'repository'
    },
    
    targets: [
      {
        target_type: 'absolute',
        target_value: 80,
        target_period: '7d',
        target_description: 'Keep drift density score above 80',
        achievement_probability: 0.70
      }
    ],
    
    thresholds: [
      {
        threshold_type: 'critical',
        threshold_value: 50,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'critical',
          notification_channels: ['slack', 'email'],
          escalation_rules: [{
            escalation_level: 1,
            time_threshold: 30,
            escalation_targets: ['tech_lead'],
            escalation_message: 'High drift density detected - code quality intervention needed'
          }],
          max_alerts_per_hour: 1,
          suppression_duration: 120
        }
      }
    ],
    
    alert_thresholds: [
      {
        threshold_type: 'warning',
        threshold_value: 65,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'warning',
          notification_channels: ['dashboard'],
          escalation_rules: [],
          max_alerts_per_hour: 1,
          suppression_duration: 240
        }
      }
    ],
    
    default_visualization: {
      chart_type: 'line',
      chart_config: {
        x_axis: { type: 'time', title: 'Time', unit: '', format: '%b %d', tick_count: 7, show_grid: true, grid_style: { color: '#e9ecef', width: 1, style: 'solid', opacity: 0.5 } },
        y_axis: { type: 'linear', title: 'Drift Density Score', unit: 'points', format: '.0f', tick_count: 5, show_grid: true, grid_style: { color: '#e9ecef', width: 1, style: 'solid', opacity: 0.5 } },
        series: [{
          name: 'Drift Density',
          type: 'line',
          data_field: 'drift_density_score',
          color: '#dc3545',
          opacity: 1,
          line_width: 2,
          visible: true,
          selectable: true,
          hoverable: true,
          y_axis: 'primary',
          trend_line: { type: 'linear', color: '#6c757d', width: 1, style: 'dashed' },
          show_data_labels: false,
          data_label_format: '.1f'
        }],
        layout: {
          width: 600,
          height: 300,
          margin: { top: 20, right: 40, bottom: 40, left: 60 },
          title: 'Drift Density Trend',
          background_color: '#ffffff',
          responsive: true,
          maintain_aspect_ratio: false,
          legend: { show: true, position: 'top', orientation: 'horizontal', alignment: 'center' }
        },
        annotations: [{
          type: 'line',
          content: 'Target: 80',
          position: { x: '0%', y: 80, anchor: 'start' },
          styling: { color: '#28a745', background_color: 'transparent', border_color: '#28a745', font_size: 12, font_weight: 'normal', opacity: 0.8 },
          clickable: false
        }],
        zoom_config: { enabled: true, type: 'x', min_zoom: 0.1, max_zoom: 10, show_reset_button: true }
      },
      color_scheme: {
        type: 'sequential',
        name: 'Reds',
        colors: ['#fee5d9', '#fcbba1', '#fc9272', '#fb6a4a', '#ef3b2c', '#cb181d', '#99000d'],
        mapping: [],
        colorblind_friendly: true,
        contrast_ratio: 4.5
      },
      styling: {
        font_family: 'Inter, system-ui, sans-serif',
        font_size_base: 14,
        primary_color: '#dc3545',
        secondary_color: '#6c757d',
        accent_color: '#28a745',
        border_radius: 4,
        border_width: 1,
        padding: 16,
        default_opacity: 1,
        hover_opacity: 0.8,
        animation_duration: 300,
        animation_easing: 'ease-out'
      },
      interactions: {
        hover_effects: {
          enabled: true,
          highlight_style: { opacity_change: 0.2, size_change: 2, border_width_change: 0 },
          tooltip: {
            enabled: true,
            template: '<strong>Date:</strong> {{x}}<br><strong>Score:</strong> {{y}}<br><strong>Events:</strong> {{event_count}}',
            position: 'mouse',
            background_color: '#000000',
            border_color: '#ffffff',
            text_color: '#ffffff',
            font_size: 12,
            show_delay: 0,
            hide_delay: 100,
            follow_mouse: true
          },
          cursor_style: 'pointer'
        },
        click_behavior: {
          enabled: true,
          action: 'filter',
          filter_field: 'date',
          multi_select: false,
          select_modifier_key: 'ctrl'
        },
        selection: {
          enabled: true,
          mode: 'range',
          selection_style: { border_color: '#007bff', border_width: 2, background_color: '#007bff', opacity: 0.1 },
          persistent_selection: true
        },
        touch_enabled: true,
        pinch_zoom: true,
        keyboard_shortcuts: [
          { key_combination: 'r', action: 'reset_zoom', description: 'Reset chart zoom' }
        ],
        cross_filter: { enabled: true, linked_charts: ['drift_events_chart'], filter_behavior: 'highlight', transition_duration: 300, show_reset_button: true }
      },
      performance: {
        max_data_points: 1000,
        data_sampling: { enabled: true, method: 'lttb', target_points: 500, preserve_extremes: true },
        canvas_rendering: false,
        progressive_loading: true,
        virtualization: false,
        cache_config: { enabled: true, cache_size: 50, cache_duration: 600, cache_key_strategy: 'url' },
        lazy_loading: false,
        viewport_buffer: 0
      },
      accessibility: {
        wcag_level: 'AA',
        aria_labels: { 'chart': 'Drift density score over time line chart' },
        data_table_fallback: true,
        keyboard_navigation: true,
        focus_indicators: true,
        high_contrast_mode: false,
        pattern_fills: true,
        text_alternatives: [{ element: 'line', alternative_text: 'Drift density trending {{trend_direction}} with current score of {{current_value}}', context: 'time_series' }],
        respect_prefers_reduced_motion: true,
        provide_static_alternative: true
      }
    },
    
    dashboard_priority: 2,
    business_impact: 'High - Early indicator of code quality degradation and technical debt accumulation',
    stakeholders: ['Development Teams', 'Tech Leads', 'QA Teams'],
    update_frequency: 'hourly'
  }
];

// Team Productivity KPIs
export const TEAM_PRODUCTIVITY_KPIS: PhotonDriftKPI[] = [
  {
    kpi_id: 'resolution_velocity',
    name: 'Drift Resolution Velocity',
    description: 'Average time to resolve drift events, weighted by severity',
    category: 'productivity',
    ml_enhanced: true,
    drift_specific: true,
    real_time_updates: true,
    
    calculation: {
      calculation_type: 'weighted',
      formula: 'sum(resolution_time * severity_weight) / sum(severity_weight)',
      source_fields: ['resolution_time_hours', 'severity_level'],
      weights: {
        'critical': 4,
        'high': 3,
        'medium': 2,
        'low': 1
      },
      time_window: '14d',
      aggregation_level: 'team'
    },
    
    targets: [
      {
        target_type: 'absolute',
        target_value: 24, // hours
        target_period: '14d',
        target_description: 'Resolve drift events within 24 hours on average',
        achievement_probability: 0.80
      }
    ],
    
    thresholds: [
      {
        threshold_type: 'warning',
        threshold_value: 48,
        threshold_operator: 'greater_than',
        alert_config: {
          alert_severity: 'warning',
          notification_channels: ['dashboard', 'email'],
          escalation_rules: [],
          max_alerts_per_hour: 1,
          suppression_duration: 180
        }
      },
      {
        threshold_type: 'excellent',
        threshold_value: 12,
        threshold_operator: 'less_than'
      }
    ],
    
    alert_thresholds: [
      {
        threshold_type: 'critical',
        threshold_value: 72,
        threshold_operator: 'greater_than',
        alert_config: {
          alert_severity: 'critical',
          notification_channels: ['slack', 'email'],
          escalation_rules: [{
            escalation_level: 1,
            time_threshold: 60,
            escalation_targets: ['team_manager'],
            escalation_message: 'Drift resolution velocity critically slow - process review needed'
          }],
          max_alerts_per_hour: 1,
          suppression_duration: 240
        }
      }
    ],
    
    default_visualization: {
      chart_type: 'bar',
      chart_config: {
        x_axis: { type: 'category', title: 'Team', unit: '', format: '', tick_count: 0, show_grid: false, grid_style: { color: '#e9ecef', width: 1, style: 'solid', opacity: 0.3 } },
        y_axis: { type: 'linear', title: 'Average Resolution Time', unit: 'hours', format: '.1f', tick_count: 5, show_grid: true, grid_style: { color: '#e9ecef', width: 1, style: 'solid', opacity: 0.5 } },
        series: [{
          name: 'Resolution Time',
          type: 'bar',
          data_field: 'avg_resolution_time',
          color: '#007bff',
          opacity: 0.8,
          visible: true,
          selectable: true,
          hoverable: true,
          y_axis: 'primary',
          show_data_labels: true,
          data_label_format: '.1f'
        }],
        layout: {
          width: 500,
          height: 300,
          margin: { top: 20, right: 20, bottom: 60, left: 80 },
          title: 'Team Resolution Velocity',
          background_color: '#ffffff',
          responsive: true,
          maintain_aspect_ratio: false,
          legend: { show: false, position: 'top', orientation: 'horizontal', alignment: 'center' }
        },
        annotations: [{
          type: 'line',
          content: 'Target: 24h',
          position: { x: '0%', y: 24, anchor: 'start' },
          styling: { color: '#28a745', background_color: 'transparent', border_color: '#28a745', font_size: 12, font_weight: 'normal', opacity: 0.8 },
          clickable: false
        }],
        zoom_config: { enabled: false, type: 'y', min_zoom: 1, max_zoom: 1, show_reset_button: false }
      },
      color_scheme: {
        type: 'categorical',
        name: 'team_performance',
        colors: ['#28a745', '#ffc107', '#fd7e14', '#dc3545'],
        mapping: [
          { value: 'excellent', color: '#28a745', description: 'Under 12 hours' },
          { value: 'good', color: '#17a2b8', description: '12-24 hours' },
          { value: 'warning', color: '#ffc107', description: '24-48 hours' },
          { value: 'critical', color: '#dc3545', description: 'Over 48 hours' }
        ],
        colorblind_friendly: true,
        contrast_ratio: 4.5
      },
      styling: {
        font_family: 'Inter, system-ui, sans-serif',
        font_size_base: 14,
        primary_color: '#007bff',
        secondary_color: '#6c757d',
        accent_color: '#28a745',
        border_radius: 4,
        border_width: 1,
        padding: 16,
        default_opacity: 0.8,
        hover_opacity: 1.0,
        animation_duration: 300,
        animation_easing: 'ease-out'
      },
      interactions: {
        hover_effects: {
          enabled: true,
          highlight_style: { opacity_change: 0.2, size_change: 0, border_width_change: 1 },
          tooltip: {
            enabled: true,
            template: '<strong>Team:</strong> {{category}}<br><strong>Avg Time:</strong> {{value}}h<br><strong>Events Resolved:</strong> {{count}}',
            position: 'data_point',
            background_color: '#000000',
            border_color: '#ffffff',
            text_color: '#ffffff',
            font_size: 12,
            show_delay: 0,
            hide_delay: 100,
            follow_mouse: false
          },
          cursor_style: 'pointer'
        },
        click_behavior: {
          enabled: true,
          action: 'drill_down',
          drill_down_target: 'team_details',
          multi_select: false,
          select_modifier_key: 'ctrl'
        },
        selection: {
          enabled: true,
          mode: 'single',
          selection_style: { border_color: '#007bff', border_width: 2, background_color: '#007bff', opacity: 0.2 },
          persistent_selection: true
        },
        touch_enabled: true,
        pinch_zoom: true,
        keyboard_shortcuts: [],
        cross_filter: { enabled: true, linked_charts: ['team_activity_chart'], filter_behavior: 'filter', transition_duration: 300, show_reset_button: true }
      },
      performance: {
        max_data_points: 50,
        data_sampling: { enabled: false, method: 'uniform', target_points: 50, preserve_extremes: false },
        canvas_rendering: false,
        progressive_loading: false,
        virtualization: false,
        cache_config: { enabled: true, cache_size: 20, cache_duration: 300, cache_key_strategy: 'data_hash' },
        lazy_loading: false,
        viewport_buffer: 0
      },
      accessibility: {
        wcag_level: 'AA',
        aria_labels: { 'chart': 'Team resolution velocity bar chart' },
        data_table_fallback: true,
        keyboard_navigation: true,
        focus_indicators: true,
        high_contrast_mode: false,
        pattern_fills: true,
        text_alternatives: [{ element: 'bar', alternative_text: '{{team}} team resolves drift events in {{value}} hours on average', context: 'team_comparison' }],
        respect_prefers_reduced_motion: true,
        provide_static_alternative: true
      }
    },
    
    dashboard_priority: 3,
    business_impact: 'Medium-High - Indicates team efficiency and process effectiveness in maintaining code quality',
    stakeholders: ['Engineering Managers', 'Team Leads', 'Scrum Masters'],
    update_frequency: 'daily'
  }
];

// ML Confidence and Quality KPIs
export const ML_QUALITY_KPIS: PhotonDriftKPI[] = [
  {
    kpi_id: 'ml_prediction_accuracy',
    name: 'ML Prediction Accuracy',
    description: 'Accuracy of ML models in predicting drift severity and impact',
    category: 'quality',
    ml_enhanced: true,
    drift_specific: true,
    real_time_updates: false,
    
    calculation: {
      calculation_type: 'simple',
      formula: '(correct_predictions / total_predictions) * 100',
      source_fields: ['correct_severity_predictions', 'total_severity_predictions'],
      time_window: '7d',
      aggregation_level: 'organization'
    },
    
    targets: [
      {
        target_type: 'absolute',
        target_value: 85,
        target_period: '7d',
        target_description: 'Maintain ML prediction accuracy above 85%',
        achievement_probability: 0.75
      }
    ],
    
    thresholds: [
      {
        threshold_type: 'warning',
        threshold_value: 75,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'warning',
          notification_channels: ['email'],
          escalation_rules: [],
          max_alerts_per_hour: 1,
          suppression_duration: 360
        }
      },
      {
        threshold_type: 'excellent',
        threshold_value: 90,
        threshold_operator: 'greater_than'
      }
    ],
    
    alert_thresholds: [
      {
        threshold_type: 'critical',
        threshold_value: 65,
        threshold_operator: 'less_than',
        alert_config: {
          alert_severity: 'critical',
          notification_channels: ['slack', 'email'],
          escalation_rules: [{
            escalation_level: 1,
            time_threshold: 120,
            escalation_targets: ['ml_team', 'data_scientist'],
            escalation_message: 'ML model accuracy degraded significantly - model retraining required'
          }],
          max_alerts_per_hour: 1,
          suppression_duration: 480
        }
      }
    ],
    
    default_visualization: {
      chart_type: 'gauge',
      chart_config: {
        x_axis: { type: 'linear', title: '', unit: '', format: '', tick_count: 0, show_grid: false, grid_style: { color: '', width: 0, style: 'solid', opacity: 0 } },
        y_axis: { type: 'linear', title: '', unit: '', format: '', tick_count: 0, show_grid: false, grid_style: { color: '', width: 0, style: 'solid', opacity: 0 } },
        series: [{
          name: 'ML Accuracy',
          type: 'gauge',
          data_field: 'accuracy_percentage',
          color: '#17a2b8',
          opacity: 1,
          visible: true,
          selectable: false,
          hoverable: true,
          y_axis: 'primary',
          show_data_labels: true,
          data_label_format: '.1f'
        }],
        layout: {
          width: 250,
          height: 180,
          margin: { top: 10, right: 10, bottom: 10, left: 10 },
          title: 'ML Accuracy',
          background_color: '#ffffff',
          responsive: true,
          maintain_aspect_ratio: true,
          legend: { show: false, position: 'bottom', orientation: 'horizontal', alignment: 'center' }
        },
        annotations: [],
        zoom_config: { enabled: false, type: 'x', min_zoom: 1, max_zoom: 1, show_reset_button: false }
      },
      color_scheme: {
        type: 'sequential',
        name: 'accuracy_gauge',
        colors: ['#dc3545', '#ffc107', '#28a745'],
        mapping: [],
        colorblind_friendly: true,
        contrast_ratio: 4.5
      },
      styling: {
        font_family: 'Inter, system-ui, sans-serif',
        font_size_base: 12,
        primary_color: '#17a2b8',
        secondary_color: '#6c757d',
        accent_color: '#28a745',
        border_radius: 4,
        border_width: 1,
        padding: 12,
        default_opacity: 1,
        hover_opacity: 0.9,
        animation_duration: 500,
        animation_easing: 'ease-in-out'
      },
      interactions: {
        hover_effects: {
          enabled: true,
          highlight_style: { opacity_change: 0.1, size_change: 0, border_width_change: 0 },
          tooltip: {
            enabled: true,
            template: '<strong>Accuracy:</strong> {{value}}%<br><strong>Period:</strong> Last 7 days<br><strong>Predictions:</strong> {{total_count}}',
            position: 'fixed',
            background_color: '#000000',
            border_color: '#ffffff',
            text_color: '#ffffff',
            font_size: 11,
            show_delay: 0,
            hide_delay: 0,
            follow_mouse: false
          },
          cursor_style: 'default'
        },
        click_behavior: { enabled: false, action: 'select', multi_select: false, select_modifier_key: 'ctrl' },
        selection: { enabled: false, mode: 'single', selection_style: { border_color: '', border_width: 0, background_color: '', opacity: 0 }, persistent_selection: false },
        touch_enabled: false,
        pinch_zoom: false,
        keyboard_shortcuts: [],
        cross_filter: { enabled: false, linked_charts: [], filter_behavior: 'filter', transition_duration: 0, show_reset_button: false }
      },
      performance: {
        max_data_points: 1,
        data_sampling: { enabled: false, method: 'uniform', target_points: 1, preserve_extremes: false },
        canvas_rendering: false,
        progressive_loading: false,
        virtualization: false,
        cache_config: { enabled: true, cache_size: 5, cache_duration: 600, cache_key_strategy: 'data_hash' },
        lazy_loading: false,
        viewport_buffer: 0
      },
      accessibility: {
        wcag_level: 'AA',
        aria_labels: { 'gauge': 'ML prediction accuracy gauge showing current accuracy percentage' },
        data_table_fallback: true,
        keyboard_navigation: false,
        focus_indicators: false,
        high_contrast_mode: false,
        pattern_fills: false,
        text_alternatives: [{ element: 'gauge', alternative_text: 'ML model accuracy is {{value}}% over the last 7 days', context: 'ml_performance' }],
        respect_prefers_reduced_motion: true,
        provide_static_alternative: true
      }
    },
    
    dashboard_priority: 4,
    business_impact: 'Medium - Ensures reliability of automated drift detection and prioritization',
    stakeholders: ['ML Team', 'Data Scientists', 'Engineering Leadership'],
    update_frequency: 'daily'
  }
];

// Combined KPI definitions
export const ALL_PHOTON_DRIFT_KPIS: PhotonDriftKPI[] = [
  ...ARCHITECTURE_HEALTH_KPIS,
  ...TEAM_PRODUCTIVITY_KPIS,
  ...ML_QUALITY_KPIS
];

// KPI Categories for dashboard organization
export const KPI_CATEGORIES = {
  HEALTH: 'health',
  PRODUCTIVITY: 'productivity', 
  QUALITY: 'quality',
  RISK: 'risk',
  BUSINESS: 'business'
} as const;

// Default dashboard layout with KPIs
export const DEFAULT_KPI_LAYOUT = {
  primary_kpis: ['overall_health_score', 'drift_density_score'],
  secondary_kpis: ['resolution_velocity', 'ml_prediction_accuracy'],
  detail_kpis: [], // Additional KPIs shown on drill-down
  update_frequencies: {
    'real-time': ['overall_health_score', 'drift_density_score'],
    'hourly': ['drift_density_score'],
    'daily': ['resolution_velocity', 'ml_prediction_accuracy'],
    'weekly': []
  }
};

// KPI calculation utilities and helpers
export const KPI_CALCULATIONS = {
  // Health score calculation weights
  HEALTH_WEIGHTS: {
    drift_density: 0.25,
    resolution_velocity: 0.20,
    technical_debt: 0.25,
    ml_confidence: 0.15,
    trend_stability: 0.15
  },
  
  // Severity weights for weighted averages
  SEVERITY_WEIGHTS: {
    critical: 4,
    high: 3,
    medium: 2,
    low: 1
  },
  
  // Time window mappings
  TIME_WINDOWS: {
    '1d': 86400,
    '7d': 604800,
    '14d': 1209600,
    '30d': 2592000
  }
};