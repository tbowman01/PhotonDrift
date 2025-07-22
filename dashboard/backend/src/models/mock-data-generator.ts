// Mock Data Generator for PhotonDrift Visual Analytics Dashboard
// Generates realistic test data for development and testing

import { 
  EnhancedDriftEvent, 
  EnhancedArchitectureHealth, 
  EnhancedTeamMetrics,
  TimeSeriesData,
  MLAnalysisResults,
  MockDataGenerator,
  GeneratorConfig
} from './enhanced-types';

export class PhotonDriftMockGenerator {
  private config: GeneratorConfig;
  private seededRandom: () => number;

  constructor(config: Partial<GeneratorConfig> = {}) {
    this.config = {
      seed: config.seed || 12345,
      locale: config.locale || 'en-US',
      date_range: config.date_range || {
        start: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000), // 90 days ago
        end: new Date()
      },
      data_volume: config.data_volume || {
        repositories: 5,
        events_per_repo: 200,
        team_members: 15,
        time_period_days: 90,
        scan_frequency_hours: 6
      },
      correlation_strength: config.correlation_strength || 0.7,
      noise_level: config.noise_level || 0.1,
      outlier_frequency: config.outlier_frequency || 0.05,
      batch_size: config.batch_size || 100,
      parallel_generation: config.parallel_generation || false
    };

    // Simple seeded random number generator
    this.seededRandom = this.createSeededRandom(this.config.seed);
  }

  private createSeededRandom(seed: number): () => number {
    let state = seed;
    return () => {
      state = (state * 9301 + 49297) % 233280;
      return state / 233280;
    };
  }

  private randomChoice<T>(array: T[]): T {
    return array[Math.floor(this.seededRandom() * array.length)];
  }

  private randomBetween(min: number, max: number): number {
    return min + this.seededRandom() * (max - min);
  }

  private randomGaussian(mean: number = 0, stdDev: number = 1): number {
    // Box-Muller transformation
    let u = 0, v = 0;
    while(u === 0) u = this.seededRandom(); // Converting [0,1) to (0,1)
    while(v === 0) v = this.seededRandom();
    const z = Math.sqrt(-2 * Math.log(u)) * Math.cos(2 * Math.PI * v);
    return z * stdDev + mean;
  }

  private generateTimeSeries(
    startDate: Date, 
    endDate: Date, 
    baseValue: number,
    trend: number = 0,
    seasonality: number = 0,
    noise: number = 0.1,
    intervalHours: number = 24
  ): TimeSeriesData {
    const data_points = [];
    const totalDuration = endDate.getTime() - startDate.getTime();
    const intervalMs = intervalHours * 60 * 60 * 1000;
    const numPoints = Math.floor(totalDuration / intervalMs);

    for (let i = 0; i < numPoints; i++) {
      const timestamp = new Date(startDate.getTime() + i * intervalMs);
      const progress = i / numPoints;
      
      // Trend component
      const trendValue = baseValue + trend * progress;
      
      // Seasonal component (daily + weekly patterns)
      const dayOfWeek = timestamp.getDay();
      const hourOfDay = timestamp.getHours();
      const seasonalValue = seasonality * (
        Math.sin(2 * Math.PI * dayOfWeek / 7) * 0.3 +  // Weekly pattern
        Math.sin(2 * Math.PI * hourOfDay / 24) * 0.2   // Daily pattern
      );
      
      // Noise component
      const noiseValue = this.randomGaussian(0, noise * baseValue);
      
      // Outliers
      const isOutlier = this.seededRandom() < this.config.outlier_frequency;
      const outlierMultiplier = isOutlier ? (1 + this.randomBetween(-0.5, 2)) : 1;
      
      const finalValue = Math.max(0, (trendValue + seasonalValue + noiseValue) * outlierMultiplier);
      
      data_points.push({
        timestamp,
        value: finalValue,
        confidence: Math.max(0.1, Math.min(1, 1 - Math.abs(noiseValue) / (baseValue * noise))),
        metadata: {
          is_outlier: isOutlier,
          trend_component: trendValue,
          seasonal_component: seasonalValue,
          noise_component: noiseValue
        }
      });
    }

    return {
      data_points,
      sampling_interval: `${intervalHours}h`,
      aggregation_method: 'avg',
      interpolation_method: 'linear'
    };
  }

  generateEnhancedDriftEvent(repositoryId: string, baseTimestamp: Date): EnhancedDriftEvent {
    const severities = ['low', 'medium', 'high', 'critical'];
    const categories = [
      'code-smell', 'architecture', 'security', 'performance', 
      'maintainability', 'documentation', 'testing', 'dependencies'
    ];
    const files = [
      'src/components/UserAuth.tsx',
      'src/services/ApiClient.ts',
      'src/utils/ValidationHelpers.ts',
      'src/models/DataModels.ts',
      'src/hooks/useDataFetching.ts',
      'src/pages/Dashboard.tsx',
      'src/config/AppConfig.ts'
    ];

    const severity = this.randomChoice(severities) as 'low' | 'medium' | 'high' | 'critical';
    const category = this.randomChoice(categories) as string;
    const file = this.randomChoice(files) as string;
    
    // Generate correlated ML confidence (higher for more severe issues)
    const severityConfidenceMap: Record<string, number> = { low: 0.6, medium: 0.75, high: 0.85, critical: 0.9 };
    const baseConfidence = severityConfidenceMap[severity];
    const mlConfidence = Math.max(0.1, Math.min(1, this.randomGaussian(baseConfidence, 0.1)));

    const timestamp = new Date(baseTimestamp.getTime() + this.randomBetween(-86400000, 86400000)); // ±1 day

    return {
      // Base DriftEvent properties
      id: `drift_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp,
      severity,
      category,
      title: this.generateDriftTitle(category as string, severity as string),
      description: this.generateDriftDescription(category as string, severity as string),
      location: {
        file,
        line: Math.floor(this.randomBetween(1, 500)),
        column: Math.floor(this.randomBetween(1, 100))
      },
      mlScore: mlConfidence,
      confidence: mlConfidence,
      resolved: this.seededRandom() < 0.3, // 30% resolved
      assignee: this.seededRandom() < 0.7 ? this.randomChoice(['alice', 'bob', 'charlie', 'diana', 'eve']) : undefined,
      tags: this.generateTags(category as string, severity as string),
      suggestion: this.generateSuggestion(category as string, severity as string),

      // Enhanced properties
      mlAnalysis: this.generateMLAnalysis(severity as string, mlConfidence),
      visualMetadata: this.generateVisualMetadata(severity as string, mlConfidence, category as string),
      historicalContext: this.generateHistoricalContext(category as string, timestamp),
      impact: this.generateImpactAssessment(severity as string, category as string),
      relationships: this.generateEventRelationships()
    };
  }

  private generateDriftTitle(category: string, severity: string): string {
    const titles = {
      'code-smell': [`${severity} code duplication detected`, `Long parameter list found`, `Complex conditional logic`],
      'architecture': [`Layer violation in ${severity} component`, `Circular dependency detected`, `Architecture boundary crossed`],
      'security': [`${severity} security vulnerability`, `Unvalidated input detected`, `Weak encryption usage`],
      'performance': [`${severity} performance bottleneck`, `Memory leak potential`, `Inefficient algorithm usage`],
      'maintainability': [`${severity} maintainability issue`, `High cyclomatic complexity`, `Dead code detected`],
      'documentation': [`Missing ${severity} documentation`, `Outdated API documentation`, `Incomplete code comments`],
      'testing': [`${severity} test coverage gap`, `Missing unit tests`, `Flaky test detected`],
      'dependencies': [`${severity} dependency issue`, `Outdated dependency version`, `Vulnerable package detected`]
    };
    
    return this.randomChoice(titles[category as keyof typeof titles] || titles['code-smell']);
  }

  private generateDriftDescription(category: string, severity: string): string {
    return `${severity.charAt(0).toUpperCase() + severity.slice(1)} ${category} issue detected by ML analysis. ` +
           `This pattern indicates potential technical debt accumulation and should be addressed according to team priorities.`;
  }

  private generateTags(category: string, severity: string): string[] {
    const baseTags = [category, severity];
    const additionalTags = [
      'auto-detected', 'ml-generated', 'needs-review', 'technical-debt',
      'refactor-candidate', 'code-quality', 'maintainability'
    ];
    
    const numAdditional = Math.floor(this.randomBetween(1, 4));
    const selectedTags = [];
    for (let i = 0; i < numAdditional; i++) {
      selectedTags.push(this.randomChoice(additionalTags));
    }
    
    return [...baseTags, ...selectedTags];
  }

  private generateSuggestion(category: string, severity: string): string {
    const suggestions = {
      'code-smell': 'Consider extracting methods and reducing complexity through refactoring.',
      'architecture': 'Review component dependencies and consider architectural refactoring.',
      'security': 'Implement proper input validation and security best practices.',
      'performance': 'Optimize algorithm efficiency and consider caching strategies.',
      'maintainability': 'Simplify logic and improve code readability.',
      'documentation': 'Add comprehensive documentation and code comments.',
      'testing': 'Increase test coverage and implement missing test cases.',
      'dependencies': 'Update dependencies and review security implications.'
    };
    
    return suggestions[category as keyof typeof suggestions] || 'Review and address according to best practices.';
  }

  private generateMLAnalysis(severity: string, confidence: number): MLAnalysisResults {
    const severityScores = { low: 0.25, medium: 0.5, high: 0.75, critical: 1.0 };
    const baseScore = severityScores[severity as keyof typeof severityScores];

    return {
      confidence,
      severity_prediction: Math.max(0, Math.min(1, this.randomGaussian(baseScore, 0.1))),
      priority_score: Math.max(0, Math.min(1, this.randomGaussian(baseScore * confidence, 0.15))),
      
      ensemble_scores: [
        { model_name: 'RandomForest', score: this.randomGaussian(baseScore, 0.08), weight: 0.3 },
        { model_name: 'XGBoost', score: this.randomGaussian(baseScore, 0.06), weight: 0.4 },
        { model_name: 'NeuralNetwork', score: this.randomGaussian(baseScore, 0.1), weight: 0.3 }
      ],
      
      feature_importance: {
        'code_complexity': this.randomBetween(0.15, 0.25),
        'change_frequency': this.randomBetween(0.10, 0.20),
        'team_experience': this.randomBetween(0.05, 0.15),
        'file_size': this.randomBetween(0.08, 0.18),
        'dependency_count': this.randomBetween(0.05, 0.15),
        'test_coverage': this.randomBetween(0.10, 0.20)
      },
      
      uncertainty: {
        epistemic: this.randomBetween(0.05, 0.15), // model uncertainty
        aleatoric: this.randomBetween(0.05, 0.20), // data uncertainty
        total: 0 // calculated below
      },
      
      explanations: {
        shap_values: Array.from({ length: 6 }, () => this.randomGaussian(0, 0.1)),
        feature_names: ['code_complexity', 'change_frequency', 'team_experience', 'file_size', 'dependency_count', 'test_coverage'],
        base_value: 0.5
      },
      
      model_version: '2.1.0',
      prediction_timestamp: new Date(),
      processing_time: this.randomBetween(50, 200)
    };
  }

  private generateVisualMetadata(severity: string, confidence: number, category: string): any {
    const severityColors = {
      low: '#28a745',
      medium: '#ffc107', 
      high: '#fd7e14',
      critical: '#dc3545'
    };
    
    const confidenceOpacity = Math.max(0.3, Math.min(1, confidence));
    
    return {
      severity_color: severityColors[severity as keyof typeof severityColors],
      confidence_color: `rgba(0, 123, 255, ${confidenceOpacity})`,
      trend_color: this.randomChoice(['#28a745', '#ffc107', '#dc3545']),
      
      chart_coordinates: {
        x: this.randomBetween(0, 1),
        y: this.randomBetween(0, 1),
        z: this.randomBetween(0, 1)
      },
      
      visual_weight: confidence * (severity === 'critical' ? 4 : severity === 'high' ? 3 : severity === 'medium' ? 2 : 1),
      opacity: confidenceOpacity,
      
      animation_delay: this.randomBetween(0, 500),
      transition_duration: 300,
      
      clickable: true,
      draggable: false,
      resizable: false,
      
      cluster_id: `cluster_${category}`,
      group_label: category.charAt(0).toUpperCase() + category.slice(1)
    };
  }

  private generateHistoricalContext(category: string, currentTimestamp: Date): any {
    const startDate = new Date(currentTimestamp.getTime() - 30 * 24 * 60 * 60 * 1000); // 30 days ago
    
    return {
      occurrence_frequency: this.generateTimeSeries(startDate, currentTimestamp, 2, 0.1, 0.3, 0.2, 24),
      
      resolution_history: Array.from({ length: Math.floor(this.randomBetween(1, 5)) }, () => ({
        resolved_at: new Date(currentTimestamp.getTime() - this.randomBetween(0, 30 * 24 * 60 * 60 * 1000)),
        resolution_method: this.randomChoice(['refactoring', 'code_review', 'architecture_change', 'documentation']),
        resolver: this.randomChoice(['alice', 'bob', 'charlie', 'diana']),
        time_to_resolve: this.randomBetween(2, 72), // hours
        effectiveness_score: this.randomBetween(0.6, 1.0),
        recurrence_time: this.seededRandom() < 0.3 ? this.randomBetween(24, 168) : undefined // hours
      })),
      
      seasonal_patterns: [
        {
          pattern_type: 'weekly' as const,
          peak_times: ['Monday', 'Tuesday'],
          strength: this.randomBetween(0.3, 0.8),
          confidence: this.randomBetween(0.6, 0.9)
        }
      ],
      
      trend_direction: this.randomChoice(['increasing', 'decreasing', 'stable', 'cyclical'] as const),
      trend_strength: this.randomBetween(0.2, 0.8),
      
      similar_events: Array.from({ length: Math.floor(this.randomBetween(1, 4)) }, () => ({
        event_id: `similar_${Math.random().toString(36).substr(2, 9)}`,
        similarity_score: this.randomBetween(0.6, 0.95),
        similarity_features: ['category', 'file_type', 'complexity'],
        outcome_comparison: {
          resolution_time_ratio: this.randomBetween(0.5, 2.0),
          success_probability: this.randomBetween(0.6, 0.9),
          effort_required_ratio: this.randomBetween(0.7, 1.5)
        }
      })),
      
      lifecycle_stage: this.randomChoice(['emerging', 'peak', 'declining', 'resolved'] as const),
      
      next_occurrence_prediction: {
        probability: this.randomBetween(0.1, 0.8),
        time_window: {
          start: new Date(currentTimestamp.getTime() + 7 * 24 * 60 * 60 * 1000),
          end: new Date(currentTimestamp.getTime() + 30 * 24 * 60 * 60 * 1000)
        },
        confidence_interval: [this.randomBetween(0.1, 0.4), this.randomBetween(0.6, 0.9)] as [number, number]
      }
    };
  }

  private generateImpactAssessment(severity: string, category: string): any {
    const severityMultiplier = { low: 1, medium: 2, high: 3, critical: 4 }[severity] || 1;
    
    return {
      business_impact: {
        revenue_impact: this.randomBetween(0, 10000) * severityMultiplier,
        user_impact: Math.floor(this.randomBetween(10, 1000) * severityMultiplier),
        availability_impact: this.randomBetween(0, 0.1) * severityMultiplier,
        compliance_risk: this.randomBetween(0, 0.3) * (category === 'security' ? 2 : 1),
        reputation_risk: this.randomBetween(0, 0.2) * severityMultiplier
      },
      
      technical_impact: {
        performance_degradation: this.randomBetween(0, 20) * (category === 'performance' ? 2 : 1),
        maintainability_score: Math.max(0, 1 - this.randomBetween(0, 0.3) * severityMultiplier),
        security_risk: this.randomBetween(0, 0.4) * (category === 'security' ? 2 : 0.5),
        scalability_impact: this.randomBetween(0, 0.3) * severityMultiplier,
        code_quality_impact: this.randomBetween(0, 0.4) * severityMultiplier
      },
      
      team_impact: {
        estimated_hours: this.randomBetween(2, 40) * severityMultiplier,
        skill_requirements: this.randomChoice([
          ['javascript', 'react'],
          ['typescript', 'node.js'],
          ['architecture', 'design-patterns'],
          ['security', 'encryption'],
          ['performance', 'optimization']
        ]),
        team_members_affected: Math.floor(this.randomBetween(1, 5)),
        knowledge_transfer_needed: this.seededRandom() < 0.3,
        training_required: this.seededRandom() < 0.2 ? ['security-training', 'architecture-patterns'] : []
      },
      
      risk_score: Math.floor(this.randomBetween(10, 90) * (severityMultiplier / 4)),
      urgency_score: Math.floor(this.randomBetween(20, 95) * (severityMultiplier / 4)),
      complexity_score: Math.floor(this.randomBetween(15, 85)),
      
      dependent_systems: Array.from(
        { length: Math.floor(this.randomBetween(0, 4)) }, 
        () => this.randomChoice(['auth-service', 'api-gateway', 'database', 'ui-components', 'notification-service'])
      ),
      cascading_risk: this.randomBetween(0, 0.6) * severityMultiplier / 4,
      
      estimated_cost: {
        development_hours: this.randomBetween(4, 80) * severityMultiplier,
        development_cost: this.randomBetween(400, 8000) * severityMultiplier,
        opportunity_cost: this.randomBetween(200, 4000) * severityMultiplier,
        risk_mitigation_cost: this.randomBetween(100, 2000) * severityMultiplier,
        total_estimated_cost: 0, // calculated below
        confidence_interval: [
          this.randomBetween(0.7, 0.9), 
          this.randomBetween(1.1, 1.4)
        ] as [number, number]
      }
    };
  }

  private generateEventRelationships(): any {
    return {
      parent_events: Array.from(
        { length: Math.floor(this.randomBetween(0, 3)) },
        () => `parent_${Math.random().toString(36).substr(2, 9)}`
      ),
      child_events: Array.from(
        { length: Math.floor(this.randomBetween(0, 4)) },
        () => `child_${Math.random().toString(36).substr(2, 9)}`
      ),
      
      correlated_events: Array.from({ length: Math.floor(this.randomBetween(1, 5)) }, () => ({
        event_id: `corr_${Math.random().toString(36).substr(2, 9)}`,
        correlation_strength: this.randomBetween(-0.8, 0.8),
        correlation_type: this.randomChoice(['causal', 'temporal', 'spatial', 'categorical'] as const),
        lag_time: this.seededRandom() < 0.5 ? this.randomBetween(1000, 86400000) : undefined
      })),
      
      pattern_membership: Array.from({ length: Math.floor(this.randomBetween(1, 3)) }, () => ({
        pattern_id: `pattern_${Math.random().toString(36).substr(2, 9)}`,
        pattern_name: this.randomChoice(['God Class', 'Feature Envy', 'Long Method', 'Data Clumps']),
        pattern_type: this.randomChoice(['anti-pattern', 'code-smell', 'architectural-issue', 'process-issue'] as const),
        membership_strength: this.randomBetween(0.5, 1.0),
        pattern_frequency: this.randomBetween(0.1, 0.4)
      })),
      
      dependency_graph: Array.from({ length: Math.floor(this.randomBetween(2, 8)) }, () => ({
        node_id: `node_${Math.random().toString(36).substr(2, 9)}`,
        node_type: this.randomChoice(['file', 'module', 'service', 'team', 'process'] as const),
        dependency_strength: this.randomBetween(0.1, 1.0),
        dependency_direction: this.randomChoice(['incoming', 'outgoing', 'bidirectional'] as const),
        risk_propagation: this.randomBetween(0.1, 0.7)
      }))
    };
  }

  generateEnhancedArchitectureHealth(repositoryId: string, timestamp: Date = new Date()): EnhancedArchitectureHealth {
    const baseHealthScore = this.randomBetween(60, 95);
    const startDate = new Date(timestamp.getTime() - 30 * 24 * 60 * 60 * 1000);

    return {
      // Base ArchitectureHealth properties
      id: `health_${repositoryId}_${timestamp.getTime()}`,
      repository: repositoryId,
      timestamp,
      overallScore: Math.floor(baseHealthScore),
      metrics: {
        driftCount: Math.floor(this.randomBetween(5, 50)),
        coverage: this.randomBetween(65, 95),
        compliance: this.randomBetween(70, 98),
        maintainability: this.randomBetween(60, 90),
        technicalDebt: this.randomBetween(10, 40)
      },
      trends: {
        direction: this.randomChoice(['improving', 'stable', 'degrading'] as const),
        velocity: this.randomBetween(-5, 5)
      },

      // Enhanced properties
      detailed_metrics: {
        code_quality: {
          complexity: this.randomBetween(2, 8),
          duplication: this.randomBetween(1, 15),
          test_coverage: this.randomBetween(65, 95),
          documentation_coverage: this.randomBetween(40, 85),
          code_smells: Math.floor(this.randomBetween(10, 100))
        },
        architecture: {
          modularity: this.randomBetween(0.6, 0.95),
          coupling: this.randomBetween(0.1, 0.4),
          cohesion: this.randomBetween(0.7, 0.95),
          dependency_health: this.randomBetween(0.65, 0.9),
          layering_compliance: this.randomBetween(0.8, 0.98)
        },
        process: {
          adr_compliance: this.randomBetween(0.7, 0.95),
          decision_velocity: this.randomBetween(0.6, 0.9),
          review_efficiency: this.randomBetween(0.65, 0.9),
          change_success_rate: this.randomBetween(0.85, 0.98),
          rollback_frequency: this.randomBetween(0.01, 0.1)
        },
        team: {
          knowledge_distribution: this.randomBetween(0.6, 0.9),
          collaboration_index: this.randomBetween(0.7, 0.95),
          onboarding_efficiency: this.randomBetween(0.5, 0.85),
          expert_dependency: this.randomBetween(0.2, 0.6),
          communication_quality: this.randomBetween(0.65, 0.9)
        },
        security: {
          vulnerability_count: Math.floor(this.randomBetween(0, 15)),
          security_debt: this.randomBetween(5, 30),
          compliance_score: this.randomBetween(0.8, 0.98),
          access_control_health: this.randomBetween(0.85, 0.98),
          data_protection_score: this.randomBetween(0.8, 0.95)
        }
      },

      predictions: {
        short_term: this.generateHealthForecast(baseHealthScore, 30),
        medium_term: this.generateHealthForecast(baseHealthScore, 90), 
        long_term: this.generateHealthForecast(baseHealthScore, 365),
        scenarios: Array.from({ length: 3 }, () => ({
          scenario_name: this.randomChoice(['Best Case', 'Likely Case', 'Worst Case']),
          scenario_description: 'Scenario based on current trends and planned improvements',
          probability: this.randomBetween(0.2, 0.4),
          health_impact: this.randomBetween(-15, 15),
          timeline: this.randomChoice(['2-4 weeks', '1-3 months', '3-6 months']),
          mitigation_strategies: ['Regular refactoring', 'Code review improvements', 'Team training']
        }))
      },

      benchmarks: {
        industry_percentile: this.randomBetween(45, 85),
        industry_average: this.randomBetween(70, 80),
        industry_best_practice: this.randomBetween(90, 98),
        peer_ranking: Math.floor(this.randomBetween(1, 20)),
        peer_count: 50,
        peer_average: this.randomBetween(65, 85),
        best_historical_score: Math.max(baseHealthScore, this.randomBetween(80, 98)),
        worst_historical_score: Math.min(baseHealthScore, this.randomBetween(45, 70)),
        average_historical_score: this.randomBetween(65, 85),
        target_score: 85,
        progress_to_target: (baseHealthScore - 65) / (85 - 65),
        estimated_time_to_target: Math.floor(this.randomBetween(30, 180))
      },

      recommendations: Array.from({ length: Math.floor(this.randomBetween(3, 8)) }, (_, i) => ({
        recommendation_id: `rec_${i}_${timestamp.getTime()}`,
        category: this.randomChoice(['quick-win', 'strategic', 'foundational', 'emergency'] as const),
        title: this.randomChoice([
          'Reduce code complexity in core modules',
          'Improve test coverage for critical paths',
          'Implement automated code quality gates',
          'Refactor high-coupling components',
          'Update architectural documentation'
        ]),
        description: 'Detailed recommendation based on ML analysis of architecture patterns',
        expected_impact: this.randomBetween(2, 15),
        effort_required: this.randomBetween(8, 80),
        implementation_time: this.randomChoice(['1-2 weeks', '2-4 weeks', '1-2 months']),
        priority_score: Math.floor(this.randomBetween(20, 95)),
        urgency: this.randomChoice(['low', 'medium', 'high', 'critical'] as const),
        action_items: Array.from({ length: Math.floor(this.randomBetween(2, 6)) }, (_, j) => ({
          id: `action_${i}_${j}`,
          description: `Action item ${j + 1}`,
          estimated_hours: this.randomBetween(2, 16),
          required_skills: ['typescript', 'testing', 'architecture'],
          dependencies: [],
          completed: this.seededRandom() < 0.2
        })),
        prerequisites: [],
        risks: ['Time investment', 'Team capacity'],
        success_criteria: ['Measurable improvement in metrics', 'Team adoption'],
        status: this.randomChoice(['suggested', 'approved', 'in-progress', 'completed', 'rejected'] as const),
        assigned_to: this.seededRandom() < 0.6 ? this.randomChoice(['alice', 'bob', 'charlie']) : undefined,
        estimated_completion: new Date(timestamp.getTime() + this.randomBetween(7, 60) * 24 * 60 * 60 * 1000)
      })),

      health_history: Array.from({ length: 30 }, (_, i) => {
        const historyDate = new Date(timestamp.getTime() - (29 - i) * 24 * 60 * 60 * 1000);
        return {
          timestamp: historyDate,
          overall_score: Math.max(0, baseHealthScore + this.randomGaussian(0, 3)),
          metric_scores: {
            code_quality: this.randomBetween(60, 90),
            architecture: this.randomBetween(70, 95),
            process: this.randomBetween(65, 85),
            team: this.randomBetween(70, 90),
            security: this.randomBetween(80, 95)
          },
          events: Array.from({ length: Math.floor(this.randomBetween(0, 3)) }, () => ({
            event_type: this.randomChoice(['improvement', 'degradation', 'milestone', 'incident'] as const),
            description: 'Health event description',
            impact_score: this.randomBetween(-10, 10),
            related_metrics: ['code_quality', 'architecture']
          })),
          context: {
            team_size: Math.floor(this.randomBetween(5, 15)),
            codebase_size: Math.floor(this.randomBetween(10000, 100000)),
            active_projects: Math.floor(this.randomBetween(1, 5)),
            major_releases: Math.floor(this.randomBetween(0, 2)),
            external_factors: []
          }
        };
      })
    };
  }

  private generateHealthForecast(baseScore: number, daysAhead: number): any {
    const trendStrength = this.randomBetween(-0.1, 0.1) * daysAhead / 30;
    const predictedScore = Math.max(20, Math.min(100, baseScore + trendStrength * 30));
    const uncertainty = Math.min(20, daysAhead / 10);

    return {
      predicted_score: predictedScore,
      confidence_interval: [
        Math.max(0, predictedScore - uncertainty),
        Math.min(100, predictedScore + uncertainty)
      ] as [number, number],
      key_factors: Array.from({ length: 3 }, () => ({
        factor_name: this.randomChoice(['Team velocity', 'Technical debt', 'Code quality', 'Process maturity']),
        impact_weight: this.randomBetween(0.1, 0.4),
        current_trend: this.randomChoice(['positive', 'negative', 'neutral'] as const),
        expected_change: this.randomBetween(-0.2, 0.3)
      })),
      risk_factors: Array.from({ length: 2 }, () => ({
        risk_name: this.randomChoice(['Resource constraints', 'Technical complexity', 'External dependencies']),
        probability: this.randomBetween(0.1, 0.4),
        impact_severity: this.randomBetween(1, 5),
        time_to_impact: this.randomBetween(7, daysAhead),
        mitigation_options: ['Risk mitigation option']
      })),
      improvement_opportunities: Array.from({ length: 2 }, () => ({
        opportunity_name: this.randomChoice(['Process automation', 'Tool adoption', 'Training program']),
        potential_gain: this.randomBetween(3, 12),
        effort_required: this.randomBetween(20, 100),
        timeline: this.randomChoice(['2-4 weeks', '1-2 months', '2-3 months']),
        prerequisites: ['Management buy-in', 'Resource allocation'],
        success_probability: this.randomBetween(0.6, 0.9)
      }))
    };
  }

  generateEnhancedTeamMetrics(teamName: string, period: { start: Date; end: Date }): EnhancedTeamMetrics {
    const teamSize = Math.floor(this.randomBetween(3, 12));
    
    return {
      // Base TeamMetrics properties  
      team: teamName,
      period,
      metrics: {
        adrsCreated: Math.floor(this.randomBetween(2, 20)),
        driftResolved: Math.floor(this.randomBetween(10, 100)),
        reviewTime: this.randomBetween(2, 24),
        collaborationScore: this.randomBetween(0.6, 0.95),
        decisionVelocity: this.randomBetween(0.5, 0.9)
      },
      members: Array.from({ length: teamSize }, (_, i) => ({
        id: `member_${i}`,
        name: this.randomChoice(['Alice', 'Bob', 'Charlie', 'Diana', 'Eve', 'Frank', 'Grace', 'Henry']),
        email: `user${i}@company.com`,
        role: this.randomChoice(['Senior Developer', 'Developer', 'Junior Developer', 'Tech Lead', 'Architect']),
        contributions: {
          adrsAuthored: Math.floor(this.randomBetween(0, 5)),
          driftResolved: Math.floor(this.randomBetween(2, 20)),
          reviewsCompleted: Math.floor(this.randomBetween(5, 30)),
          discussionParticipation: this.randomBetween(0.2, 0.9)
        }
      })),

      // Enhanced properties
      productivity: this.generateProductivityMetrics(period),
      collaboration: this.generateCollaborationMetrics(teamSize),
      knowledge: this.generateKnowledgeMetrics(teamSize),
      performance_trends: this.generatePerformanceTrends(period),
      team_health: this.generateTeamHealthIndicators(teamSize),
      individual_insights: Array.from({ length: teamSize }, (_, i) => ({
        member_id: `member_${i}_anon`,
        role: this.randomChoice(['Senior Developer', 'Developer', 'Junior Developer']),
        tenure: Math.floor(this.randomBetween(6, 48)), // months
        productivity_score: this.randomBetween(0.6, 0.95),
        quality_score: this.randomBetween(0.65, 0.9),
        collaboration_score: this.randomBetween(0.7, 0.95),
        growth_trajectory: this.randomChoice(['accelerating', 'steady', 'plateauing'] as const),
        strengths: this.randomChoice([
          ['Technical expertise', 'Problem solving'],
          ['Code review', 'Mentoring'],
          ['Architecture design', 'Documentation'],
          ['Testing', 'Process improvement']
        ]),
        development_areas: this.randomChoice([
          ['Communication', 'Time management'],
          ['Technical depth', 'Leadership'],
          ['Documentation', 'Testing practices']
        ]),
        career_goals: ['Senior role', 'Technical leadership', 'Architecture'],
        unique_contributions: ['Innovation', 'Knowledge sharing'],
        mentorship_activities: ['Code review', 'Onboarding'],
        innovation_contributions: ['Process improvement', 'Tool evaluation']
      }))
    };
  }

  private generateProductivityMetrics(period: { start: Date; end: Date }): any {
    return {
      velocity: {
        story_points_per_sprint: this.generateTimeSeries(period.start, period.end, 25, 2, 3, 0.15, 168), // weekly
        tasks_completed_per_day: this.generateTimeSeries(period.start, period.end, 3.5, 0.1, 0.5, 0.2, 24),
        cycle_time: this.generateTimeSeries(period.start, period.end, 4.2, -0.2, 0.8, 0.25, 24), // days
        lead_time: this.generateTimeSeries(period.start, period.end, 7.5, -0.1, 1.2, 0.3, 24),
        throughput: this.generateTimeSeries(period.start, period.end, 12, 1, 2, 0.2, 168),
        velocity_variance: this.randomBetween(0.1, 0.3),
        predictability_score: this.randomBetween(0.65, 0.9)
      },
      quality: {
        defect_rate: this.generateTimeSeries(period.start, period.end, 0.05, -0.01, 0.01, 0.02, 24),
        rework_percentage: this.randomBetween(5, 20),
        code_review_effectiveness: this.randomBetween(0.7, 0.95),
        test_coverage_trend: this.generateTimeSeries(period.start, period.end, 75, 2, 5, 2, 168),
        customer_satisfaction: this.generateTimeSeries(period.start, period.end, 4.2, 0.1, 0.2, 0.15, 168),
        first_time_right: this.randomBetween(0.7, 0.9),
        technical_debt_trend: this.generateTimeSeries(period.start, period.end, 20, -1, 3, 2, 168)
      },
      efficiency: {
        coding_time_percentage: this.randomBetween(40, 70),
        meeting_time_percentage: this.randomBetween(15, 35),
        review_time_percentage: this.randomBetween(10, 25),
        planning_time_percentage: this.randomBetween(5, 15),
        waiting_time: this.randomBetween(5, 20),
        rework_time: this.randomBetween(5, 25),
        context_switch_overhead: this.randomBetween(10, 30),
        work_in_progress: this.generateTimeSeries(period.start, period.end, 5, 0, 1, 0.5, 24),
        flow_efficiency: this.randomBetween(0.6, 0.85)
      },
      innovation: {
        experimental_projects: Math.floor(this.randomBetween(1, 5)),
        new_technologies_adopted: Math.floor(this.randomBetween(0, 3)),
        process_improvements_suggested: Math.floor(this.randomBetween(2, 10)),
        patents_or_publications: Math.floor(this.randomBetween(0, 2)),
        knowledge_sharing_sessions: Math.floor(this.randomBetween(1, 8)),
        innovation_time_percentage: this.randomBetween(5, 20),
        idea_implementation_rate: this.randomBetween(0.3, 0.8)
      },
      blockers: {
        total_blockers: Math.floor(this.randomBetween(3, 15)),
        average_blocking_time: this.randomBetween(4, 24), // hours
        blocker_categories: {
          'external_dependency': this.randomBetween(20, 40),
          'technical_issue': this.randomBetween(15, 35),
          'resource_unavailable': this.randomBetween(10, 25),
          'process_bottleneck': this.randomBetween(10, 30)
        },
        blocker_trends: this.generateTimeSeries(period.start, period.end, 0.8, -0.1, 0.2, 0.15, 24),
        resolution_effectiveness: this.randomBetween(0.6, 0.9)
      },
      focus_time: {
        daily_focus_hours: this.generateTimeSeries(period.start, period.end, 4.5, 0.2, 1, 0.5, 24),
        interruption_frequency: this.generateTimeSeries(period.start, period.end, 6, -0.5, 2, 0.8, 24),
        deep_work_sessions: this.generateTimeSeries(period.start, period.end, 2.5, 0.1, 0.5, 0.3, 24),
        optimal_focus_hours: ['09:00-11:00', '14:00-16:00']
      },
      context_switching: {
        switches_per_day: this.generateTimeSeries(period.start, period.end, 8, -0.5, 2, 1, 24),
        switch_cost: this.randomBetween(10, 25), // minutes
        concurrent_projects: this.generateTimeSeries(period.start, period.end, 2.5, 0, 0.5, 0.3, 24),
        multitasking_efficiency: this.randomBetween(0.5, 0.8)
      }
    };
  }

  private generateCollaborationMetrics(teamSize: number): any {
    return {
      communication: {
        meeting_frequency: this.generateTimeSeries(
          new Date(Date.now() - 30 * 24 * 60 * 60 * 1000), 
          new Date(), 
          3, 0, 0.5, 0.2, 24
        ),
        meeting_effectiveness: this.randomBetween(0.6, 0.9),
        response_time: this.generateTimeSeries(
          new Date(Date.now() - 30 * 24 * 60 * 60 * 1000), 
          new Date(), 
          2.5, 0, 0.5, 0.3, 24
        ), // hours
        communication_clarity: this.randomBetween(0.7, 0.95),
        channel_usage: {
          'slack': this.randomBetween(40, 60),
          'email': this.randomBetween(20, 35),
          'face_to_face': this.randomBetween(15, 30),
          'video_call': this.randomBetween(10, 25)
        },
        preferred_communication_methods: ['Slack', 'Video calls', 'Face-to-face']
      },
      knowledge_sharing: {
        documentation_contributions: Math.floor(this.randomBetween(2, 15)),
        mentoring_hours: this.randomBetween(5, 25),
        knowledge_sessions_led: Math.floor(this.randomBetween(1, 8)),
        cross_training_participation: this.randomBetween(0.4, 0.9),
        knowledge_centralization_index: this.randomBetween(0.3, 0.7),
        bus_factor: Math.max(1, Math.floor(teamSize * 0.3)) // minimum 1
      },
      team_dynamics: {
        psychological_safety_score: this.randomBetween(0.6, 0.95),
        trust_index: this.randomBetween(0.7, 0.9),
        conflict_resolution_efficiency: this.randomBetween(0.6, 0.9),
        decision_making_speed: this.randomBetween(0.5, 0.85),
        collaboration_frequency: Object.fromEntries(
          Array.from({ length: Math.min(5, teamSize) }, (_, i) => [
            `pair_${i}`, 
            this.randomBetween(2, 20)
          ])
        ),
        team_cohesion_score: this.randomBetween(0.65, 0.9)
      },
      cross_functional: {
        cross_team_collaborations: Math.floor(this.randomBetween(3, 15)),
        stakeholder_satisfaction: this.randomBetween(0.7, 0.9),
        external_communication_effectiveness: this.randomBetween(0.6, 0.85),
        alignment_with_business_goals: this.randomBetween(0.7, 0.95)
      }
    };
  }

  private generateKnowledgeMetrics(teamSize: number): any {
    return {
      domain_expertise_coverage: {
        'frontend': this.randomBetween(0.6, 0.9),
        'backend': this.randomBetween(0.7, 0.95),
        'database': this.randomBetween(0.5, 0.8),
        'devops': this.randomBetween(0.4, 0.7),
        'testing': this.randomBetween(0.6, 0.85)
      },
      skill_distribution: Array.from({ length: 5 }, (_, i) => ({
        skill_name: this.randomChoice(['JavaScript', 'TypeScript', 'React', 'Node.js', 'Testing']),
        coverage_percentage: this.randomBetween(40, 90),
        expertise_levels: {
          'beginner': this.randomBetween(10, 30),
          'intermediate': this.randomBetween(40, 60),
          'advanced': this.randomBetween(20, 40),
          'expert': this.randomBetween(5, 20)
        },
        growth_trend: this.randomChoice(['improving', 'stable', 'declining'] as const)
      })),
      knowledge_gaps: Array.from({ length: Math.floor(this.randomBetween(1, 4)) }, (_, i) => ({
        gap_name: this.randomChoice(['Cloud Architecture', 'Machine Learning', 'Security Practices', 'Performance Optimization']),
        criticality: this.randomChoice(['low', 'medium', 'high', 'critical'] as const),
        impact_assessment: 'Impact on team capability and project delivery',
        recommended_actions: ['Training program', 'Hire specialist', 'External consultation'],
        timeline_to_fill: this.randomChoice(['1-2 months', '2-4 months', '6+ months'])
      })),
      learning_velocity: {
        new_skills_acquired: this.generateTimeSeries(
          new Date(Date.now() - 90 * 24 * 60 * 60 * 1000), 
          new Date(), 
          1.5, 0.1, 0.3, 0.2, 168
        ), // weekly
        certification_achievements: Math.floor(this.randomBetween(0, teamSize)),
        training_hours: this.generateTimeSeries(
          new Date(Date.now() - 90 * 24 * 60 * 60 * 1000), 
          new Date(), 
          8, 1, 2, 1, 168
        ), // weekly hours
        skill_application_rate: this.randomBetween(0.6, 0.9)
      },
      training_effectiveness: {
        training_satisfaction: this.randomBetween(0.7, 0.95),
        knowledge_retention: this.randomBetween(0.6, 0.85),
        skill_application: this.randomBetween(0.65, 0.9),
        performance_improvement: this.randomBetween(0.5, 0.8),
        roi_on_training: this.randomBetween(1.5, 4.0)
      },
      knowledge_retention_rate: this.randomBetween(0.7, 0.9),
      documentation_quality: this.randomBetween(0.6, 0.85)
    };
  }

  private generatePerformanceTrends(period: { start: Date; end: Date }): any {
    return Array.from({ length: 5 }, (_, i) => ({
      metric_name: this.randomChoice(['Velocity', 'Quality', 'Collaboration', 'Innovation', 'Efficiency']),
      trend_direction: this.randomChoice(['improving', 'stable', 'declining'] as const),
      trend_strength: this.randomBetween(0.2, 0.8),
      trend_duration: Math.floor(this.randomBetween(7, 30)), // days
      correlation_factors: Array.from({ length: 2 }, () => ({
        factor_name: this.randomChoice(['Team size', 'Project complexity', 'Tool adoption', 'Process changes']),
        correlation_coefficient: this.randomBetween(-0.8, 0.8),
        significance: this.randomBetween(0.01, 0.1),
        causal_direction: this.randomChoice(['causes', 'caused_by', 'correlated'] as const)
      })),
      seasonality: [{
        pattern_type: 'weekly' as const,
        peak_times: ['Tuesday', 'Wednesday'],
        strength: this.randomBetween(0.2, 0.6),
        confidence: this.randomBetween(0.6, 0.9)
      }],
      anomalies: Array.from({ length: Math.floor(this.randomBetween(0, 3)) }, () => ({
        timestamp: new Date(period.start.getTime() + this.randomBetween(0, period.end.getTime() - period.start.getTime())),
        severity: this.randomChoice(['minor', 'moderate', 'major'] as const),
        description: 'Performance anomaly detected',
        likely_causes: ['External dependency', 'Process change', 'Team event'],
        impact_duration: Math.floor(this.randomBetween(1, 7)) // days
      }))
    }));
  }

  private generateTeamHealthIndicators(teamSize: number): any {
    return {
      overall_health_score: Math.floor(this.randomBetween(65, 95)),
      workload_balance: this.randomBetween(0.6, 0.9),
      stress_levels: this.randomBetween(0.2, 0.6), // higher is worse
      job_satisfaction: this.randomBetween(0.7, 0.95),
      work_life_balance: this.randomBetween(0.6, 0.9),
      career_growth_satisfaction: this.randomBetween(0.65, 0.9),
      burnout_risk: Array.from({ length: Math.floor(this.randomBetween(0, 2)) }, () => ({
        member_id: `anon_member_${Math.random().toString(36).substr(2, 5)}`,
        risk_level: this.randomChoice(['low', 'medium', 'high', 'critical'] as const),
        contributing_factors: ['High workload', 'Tight deadlines', 'Limited resources'],
        recommended_interventions: ['Workload redistribution', 'Time off', 'Process improvement'],
        timeline: this.randomChoice(['immediate', '1-2 weeks', '2-4 weeks'])
      })),
      turnover_risk: {
        overall_risk: this.randomBetween(0.1, 0.3),
        high_risk_members: Math.floor(this.randomBetween(0, Math.max(1, teamSize * 0.2))),
        contributing_factors: ['Limited growth opportunities', 'Work-life balance', 'Compensation'],
        retention_strategies: ['Career development', 'Flexible work', 'Recognition programs']
      },
      performance_concerns: Array.from({ length: Math.floor(this.randomBetween(0, 2)) }, () => ({
        concern_type: this.randomChoice(['Productivity', 'Quality', 'Collaboration', 'Skills']),
        severity: this.randomChoice(['minor', 'moderate', 'significant'] as const),
        affected_members: Math.floor(this.randomBetween(1, Math.max(1, teamSize * 0.3))),
        recommended_actions: ['Additional training', 'Mentoring', 'Process clarification']
      })),
      engagement_score: this.randomBetween(0.7, 0.95),
      motivation_level: this.randomBetween(0.65, 0.9),
      team_spirit: this.randomBetween(0.7, 0.95)
    };
  }

  // Batch generation methods
  generateDriftEventsBatch(repositoryId: string, count: number, dateRange?: { start: Date; end: Date }): EnhancedDriftEvent[] {
    const range = dateRange || this.config.date_range;
    const events: EnhancedDriftEvent[] = [];
    
    for (let i = 0; i < count; i++) {
      const timestamp = new Date(
        range.start.getTime() + 
        this.seededRandom() * (range.end.getTime() - range.start.getTime())
      );
      events.push(this.generateEnhancedDriftEvent(repositoryId, timestamp));
    }
    
    return events.sort((a, b) => a.timestamp.getTime() - b.timestamp.getTime());
  }

  generateArchitectureHealthHistory(repositoryId: string, days: number = 30): EnhancedArchitectureHealth[] {
    const history: EnhancedArchitectureHealth[] = [];
    const endDate = new Date();
    
    for (let i = days - 1; i >= 0; i--) {
      const date = new Date(endDate.getTime() - i * 24 * 60 * 60 * 1000);
      history.push(this.generateEnhancedArchitectureHealth(repositoryId, date));
    }
    
    return history;
  }

  generateTeamMetricsBatch(teamNames: string[], period: { start: Date; end: Date }): EnhancedTeamMetrics[] {
    return teamNames.map(teamName => this.generateEnhancedTeamMetrics(teamName, period));
  }

  // Export data in various formats
  exportToJSON(data: any): string {
    return JSON.stringify(data, null, 2);
  }

  exportToCSV(data: any[]): string {
    if (!data.length) return '';
    
    const headers = Object.keys(data[0]).join(',');
    const rows = data.map(row => 
      Object.values(row).map(value => 
        typeof value === 'string' ? `"${value}"` : value
      ).join(',')
    );
    
    return [headers, ...rows].join('\n');
  }

  // Complete dataset generation
  generateCompleteDataset(): {
    driftEvents: EnhancedDriftEvent[];
    architectureHealth: EnhancedArchitectureHealth[];
    teamMetrics: EnhancedTeamMetrics[];
    metadata: {
      generationTimestamp: Date;
      config: GeneratorConfig;
      statistics: {
        totalDriftEvents: number;
        totalHealthRecords: number;
        totalTeamRecords: number;
        dateRange: { start: Date; end: Date };
      };
    };
  } {
    const repositories = Array.from({ length: this.config.data_volume.repositories }, (_, i) => `repo_${i + 1}`);
    const teams = ['Frontend Team', 'Backend Team', 'DevOps Team', 'QA Team', 'Architecture Team'];

    // Generate drift events for all repositories
    const driftEvents = repositories.flatMap(repoId => 
      this.generateDriftEventsBatch(repoId, this.config.data_volume.events_per_repo)
    );

    // Generate architecture health history for all repositories
    const architectureHealth = repositories.flatMap(repoId =>
      this.generateArchitectureHealthHistory(repoId, this.config.data_volume.time_period_days)
    );

    // Generate team metrics
    const teamMetrics = this.generateTeamMetricsBatch(teams, this.config.date_range);

    return {
      driftEvents,
      architectureHealth,
      teamMetrics,
      metadata: {
        generationTimestamp: new Date(),
        config: this.config,
        statistics: {
          totalDriftEvents: driftEvents.length,
          totalHealthRecords: architectureHealth.length,
          totalTeamRecords: teamMetrics.length,
          dateRange: this.config.date_range
        }
      }
    };
  }
}

// Helper functions and utilities
export const MOCK_DATA_PRESETS = {
  SMALL_DATASET: {
    repositories: 2,
    events_per_repo: 50,
    team_members: 8,
    time_period_days: 30,
    scan_frequency_hours: 12
  },
  MEDIUM_DATASET: {
    repositories: 5,
    events_per_repo: 200,
    team_members: 15,
    time_period_days: 90,
    scan_frequency_hours: 6
  },
  LARGE_DATASET: {
    repositories: 10,
    events_per_repo: 500,
    team_members: 25,
    time_period_days: 180,
    scan_frequency_hours: 3
  }
};

export function createMockGenerator(preset: keyof typeof MOCK_DATA_PRESETS = 'MEDIUM_DATASET'): PhotonDriftMockGenerator {
  return new PhotonDriftMockGenerator({
    data_volume: MOCK_DATA_PRESETS[preset],
    correlation_strength: 0.7,
    noise_level: 0.15,
    outlier_frequency: 0.05
  });
}

// Usage examples and documentation
export const USAGE_EXAMPLES = {
  basic: `
    const generator = new PhotonDriftMockGenerator();
    const driftEvent = generator.generateEnhancedDriftEvent('repo1', new Date());
    console.log(driftEvent);
  `,
  
  batch: `
    const generator = createMockGenerator('LARGE_DATASET');
    const events = generator.generateDriftEventsBatch('repo1', 100);
    console.log(\`Generated \${events.length} drift events\`);
  `,
  
  complete: `
    const generator = createMockGenerator('MEDIUM_DATASET');
    const dataset = generator.generateCompleteDataset();
    const json = generator.exportToJSON(dataset);
    // Save to file or use in tests
  `
};