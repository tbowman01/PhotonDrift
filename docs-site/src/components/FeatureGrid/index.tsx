/**
 * FeatureGrid Component
 * 
 * Responsive grid layout for showcasing PhotonDrift features
 * with icons, descriptions, and status indicators
 */

import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

interface Feature {
  title: string;
  icon: string;
  description: string;
  status?: 'completed' | 'in-progress' | 'planned';
  link?: string;
}

interface FeatureGridProps {
  features: Feature[];
  columns?: number;
  className?: string;
}

const StatusBadge: React.FC<{ status?: string }> = ({ status }) => {
  if (!status) return null;
  
  const statusConfig = {
    completed: { label: '✅ Complete', className: styles.statusCompleted },
    'in-progress': { label: '🔄 In Progress', className: styles.statusInProgress },
    planned: { label: '📋 Planned', className: styles.statusPlanned }
  };
  
  const config = statusConfig[status as keyof typeof statusConfig];
  if (!config) return null;
  
  return (
    <span className={clsx(styles.statusBadge, config.className)}>
      {config.label}
    </span>
  );
};

const FeatureCard: React.FC<{ feature: Feature }> = ({ feature }) => {
  const CardElement = feature.link ? 'a' : 'div';
  const cardProps = feature.link ? { href: feature.link } : {};
  
  return (
    <CardElement
      className={clsx(styles.featureCard, {
        [styles.featureCardLink]: feature.link
      })}
      {...cardProps}
    >
      <div className={styles.featureIcon}>
        {feature.icon}
      </div>
      
      <div className={styles.featureContent}>
        <h3 className={styles.featureTitle}>
          {feature.title}
          {feature.status && (
            <StatusBadge status={feature.status} />
          )}
        </h3>
        
        <p className={styles.featureDescription}>
          {feature.description}
        </p>
      </div>
    </CardElement>
  );
};

export default function FeatureGrid({
  features,
  columns = 3,
  className
}: FeatureGridProps): JSX.Element {
  return (
    <div 
      className={clsx(
        styles.featureGrid,
        className,
        styles[`columns${columns}`]
      )}
    >
      {features.map((feature, index) => (
        <FeatureCard key={index} feature={feature} />
      ))}
    </div>
  );
}

// Pre-defined feature sets for common use cases
export const CoreFeatures: Feature[] = [
  {
    title: 'AI-Enhanced Detection',
    icon: '🤖',
    description: 'Machine learning models with 5 advanced algorithms for intelligent drift detection',
    status: 'completed',
    link: '/docs/ml-features/neural-training'
  },
  {
    title: 'CLI Tool',
    icon: '💻',
    description: 'Comprehensive command-line interface with 5 core commands for ADR management',
    status: 'completed',
    link: '/docs/getting-started/cli'
  },
  {
    title: 'WebAssembly Support',
    icon: '🌐',
    description: 'Browser and Node.js integration with WebAssembly for universal deployment',
    status: 'completed',
    link: '/docs/deployment/container-versioning-strategy'
  },
  {
    title: 'Multi-Platform',
    icon: '🚀',
    description: 'Support for Linux, Windows, macOS, ARM64, and WebAssembly targets',
    status: 'completed',
    link: '/docs/deployment/docker-build-guide'
  },
  {
    title: 'Smart Configuration',
    icon: '⚙️',
    description: 'YAML/TOML configuration with ML-ready settings and backward compatibility',
    status: 'completed',
    link: '/docs/getting-started/config'
  },
  {
    title: 'Real-time Analysis',
    icon: '⚡',
    description: 'File system watchers with instant ML feedback and live updates',
    status: 'in-progress',
    link: '/docs/ml-features/performance-analysis'
  }
];

export const MLFeatures: Feature[] = [
  {
    title: 'Isolation Forest',
    icon: '🌲',
    description: 'Outlier detection for identifying anomalous code patterns',
    status: 'completed'
  },
  {
    title: 'Support Vector Machine',
    icon: '🎯',
    description: 'Boundary-based anomaly detection with high precision',
    status: 'completed'
  },
  {
    title: 'Local Outlier Factor',
    icon: '📍',
    description: 'Density-based detection for local anomalies',
    status: 'completed'
  },
  {
    title: 'Statistical Methods',
    icon: '📊',
    description: 'Statistical analysis for drift pattern recognition',
    status: 'completed'
  },
  {
    title: 'Ensemble Model',
    icon: '🎼',
    description: 'Combined model approach for maximum accuracy',
    status: 'completed'
  },
  {
    title: 'Online Learning',
    icon: '🧠',
    description: 'Continuous learning from feedback and historical data',
    status: 'in-progress'
  }
];

export const DevelopmentFeatures: Feature[] = [
  {
    title: 'IDE Extensions',
    icon: '🔧',
    description: 'VS Code and IntelliJ plugins with ML insights',
    status: 'planned'
  },
  {
    title: 'Language Server',
    icon: '🗣️',
    description: 'Universal IDE support with intelligent warnings',
    status: 'planned'
  },
  {
    title: 'Visual Dashboard',
    icon: '📈',
    description: 'Web-based analytics with trend analysis',
    status: 'planned'
  },
  {
    title: 'API Integration',
    icon: '🔗',
    description: 'REST API, GraphQL, and webhook support',
    status: 'planned'
  }
];