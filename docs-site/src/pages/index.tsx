/**
 * PhotonDrift Documentation Homepage
 * 
 * Landing page for PhotonDrift documentation site
 * Features hero section, feature highlights, and quick start links
 */

import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import FeatureGrid, { CoreFeatures, MLFeatures } from '@site/src/components/FeatureGrid';
import CliCommand from '@site/src/components/CliCommand';

import styles from './index.module.css';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        
        <div className={styles.heroSubtext}>
          Revolutionary Architecture Decision Record (ADR) management with AI-powered drift detection 
          and intelligent insights for modern development teams.
        </div>
        
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/getting-started/quick-start"
          >
            🚀 Quick Start
          </Link>
          <Link
            className="button button--primary button--lg"
            to="/docs/getting-started/cli"
          >
            📖 CLI Reference
          </Link>
          <Link
            className="button button--secondary button--lg"
            to="https://github.com/tbowman01/PhotonDrift"
          >
            ⭐ GitHub
          </Link>
        </div>
      </div>
    </header>
  );
}

function QuickStartSection() {
  return (
    <section className={styles.section}>
      <div className="container">
        <div className="row">
          <div className="col col--6">
            <h2>🚀 Get Started in Minutes</h2>
            <p>
              PhotonDrift makes it easy to manage Architecture Decision Records with 
              AI-enhanced drift detection. Install and start detecting architectural 
              changes in your codebase immediately.
            </p>
            
            <Link
              className="button button--primary"
              to="/docs/getting-started/quick-start"
            >
              View Full Guide →
            </Link>
          </div>
          
          <div className="col col--6">
            <CliCommand
              command="# Download and install PhotonDrift
curl -L https://github.com/tbowman01/PhotonDrift/releases/latest/download/adrscan-linux -o adrscan
chmod +x adrscan

# Initialize ADR structure
./adrscan init --adr-dir ./docs/adr

# Detect architectural drift with AI
./adrscan diff --adr-dir ./docs/adr --directory ./src"
              title="Installation & Basic Usage"
            />
          </div>
        </div>
      </div>
    </section>
  );
}

function FeaturesSection() {
  return (
    <section className={styles.section}>
      <div className="container">
        <div className="text--center margin-bottom--lg">
          <h2>🌟 Core Features</h2>
          <p>
            PhotonDrift combines traditional ADR management with cutting-edge AI 
            to provide intelligent insights into your architectural decisions.
          </p>
        </div>
        
        <FeatureGrid features={CoreFeatures} columns={3} />
      </div>
    </section>
  );
}

function MLFeaturesSection() {
  return (
    <section className={clsx(styles.section, styles.sectionAlt)}>
      <div className="container">
        <div className="text--center margin-bottom--lg">
          <h2>🤖 Machine Learning Capabilities</h2>
          <p>
            Advanced AI algorithms provide intelligent drift detection with high 
            confidence scores and explainable results.
          </p>
        </div>
        
        <FeatureGrid features={MLFeatures} columns={3} />
        
        <div className="text--center margin-top--lg">
          <Link
            className="button button--primary button--lg"
            to="/docs/ml-features/neural-training"
          >
            Learn About ML Features →
          </Link>
        </div>
      </div>
    </section>
  );
}

function StatsSection() {
  const stats = [
    { value: '5', label: 'ML Algorithms', icon: '🧠' },
    { value: '97.8%', label: 'Test Coverage', icon: '✅' },
    { value: '50+', label: 'Features Extracted', icon: '📊' },
    { value: '91ms', label: 'Scan 206 Files', icon: '⚡' },
  ];
  
  return (
    <section className={styles.statsSection}>
      <div className="container">
        <div className="row">
          {stats.map((stat, idx) => (
            <div key={idx} className="col col--3">
              <div className={styles.statCard}>
                <div className={styles.statIcon}>{stat.icon}</div>
                <div className={styles.statValue}>{stat.value}</div>
                <div className={styles.statLabel}>{stat.label}</div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function CTASection() {
  return (
    <section className={clsx(styles.section, styles.ctaSection)}>
      <div className="container">
        <div className="row">
          <div className="col col--8 col--offset-2 text--center">
            <h2>Ready to Transform Your ADR Management?</h2>
            <p>
              Join developers who are already using PhotonDrift to maintain 
              architectural integrity with AI-powered insights.
            </p>
            
            <div className={styles.ctaButtons}>
              <Link
                className="button button--primary button--lg"
                to="/docs/getting-started/quick-start"
              >
                🚀 Get Started Now
              </Link>
              <Link
                className="button button--secondary button--lg"
                to="/docs/getting-started/user-guide"
              >
                📖 Read the Guide
              </Link>
            </div>
            
            <div className={styles.ctaNote}>
              <small>
                Open source • MIT License • Active development • 
                <Link to="https://github.com/tbowman01/PhotonDrift"> View on GitHub</Link>
              </small>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

export default function Home(): JSX.Element {
  const { siteConfig } = useDocusaurusContext();
  
  return (
    <Layout
      title={`${siteConfig.title} - AI-Powered ADR Management`}
      description="Revolutionary Architecture Decision Record management with AI-enhanced drift detection and intelligent development governance."
    >
      <HomepageHeader />
      <main>
        <QuickStartSection />
        <FeaturesSection />
        <StatsSection />
        <MLFeaturesSection />
        <CTASection />
      </main>
    </Layout>
  );
}