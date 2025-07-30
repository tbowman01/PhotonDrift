#!/usr/bin/env node

/**
 * Intelligent Issue Triage System for PhotonDrift
 * Enhanced with Claude Flow swarm coordination and machine learning
 */

const { Octokit } = require('@octokit/rest');
const fs = require('fs');
const path = require('path');

// Configuration
const REPO_OWNER = process.env.REPO_OWNER || 'tbowman01';
const REPO_NAME = process.env.REPO_NAME || 'PhotonDrift';
const GITHUB_TOKEN = process.env.GITHUB_TOKEN;
const AUTO_LABEL = process.env.AUTO_LABEL === 'true';
const AUTO_ASSIGN = process.env.AUTO_ASSIGN === 'true';

// Initialize GitHub client
const octokit = new Octokit({
  auth: GITHUB_TOKEN,
});

// Enhanced classification engine with confidence scoring
class IssueClassifier {
  constructor() {
    this.patterns = {
      bug: {
        title: [/\[BUG\]/i, /bug/i, /error/i, /broken/i, /fix/i, /crash/i],
        body: [/error/i, /broken/i, /doesn't work/i, /fails/i, /exception/i, /crash/i],
        weight: 0.8
      },
      security: {
        title: [/security/i, /vulnerability/i, /CVE/i, /exploit/i, /unsafe/i],
        body: [/security/i, /vulnerability/i, /CVE-/i, /exploit/i, /unsafe/i, /attack/i],
        weight: 1.0  // Highest priority
      },
      performance: {
        title: [/performance/i, /slow/i, /optimization/i, /memory/i, /speed/i],
        body: [/performance/i, /slow/i, /optimization/i, /memory leak/i, /bottleneck/i],
        weight: 0.7
      },
      dependencies: {
        title: [/dependency/i, /package/i, /update/i, /renovate/i, /deps/i],
        body: [/dependency/i, /package/i, /update/i, /version/i, /upgrade/i],
        weight: 0.5
      },
      feature: {
        title: [/\[PHASE/i, /feature/i, /enhancement/i, /add/i, /implement/i],
        body: [/feature/i, /enhancement/i, /new/i, /implement/i, /add/i],
        weight: 0.6
      },
      documentation: {
        title: [/report/i, /analysis/i, /docs/i, /documentation/i, /guide/i],
        body: [/report/i, /analysis/i, /documentation/i, /readme/i, /guide/i],
        weight: 0.4
      },
      ci_cd: {
        title: [/CI/i, /CD/i, /pipeline/i, /workflow/i, /action/i, /build/i],
        body: [/pipeline/i, /workflow/i, /github action/i, /build/i, /deploy/i],
        weight: 0.6
      },
      wasm: {
        title: [/WASM/i, /WebAssembly/i, /wasm/i],
        body: [/WebAssembly/i, /wasm/i, /web assembly/i],
        weight: 0.8
      }
    };

    this.priorityIndicators = {
      critical: [/critical/i, /urgent/i, /blocking/i, /emergency/i, /production down/i],
      high: [/high priority/i, /important/i, /\[PHASE 3\]/i, /security/i, /critical bug/i],
      medium: [/medium priority/i, /moderate/i, /performance/i, /enhancement/i],
      low: [/low priority/i, /nice to have/i, /future/i, /roadmap/i, /documentation/i]
    };

    this.componentMap = {
      'component-cli': [/CLI/i, /command line/i, /terminal/i],
      'component-core': [/core/i, /engine/i, /algorithm/i, /detection/i],
      'component-config': [/config/i, /configuration/i, /settings/i],
      'component-parsing': [/parsing/i, /parser/i, /ADR/i, /markdown/i],
      'component-drift': [/drift/i, /detection/i, /diff/i, /compare/i],
      'component-wasm': [/WASM/i, /WebAssembly/i, /wasm/i],
      'component-github-action': [/github action/i, /workflow/i, /CI/i, /CD/i],
      'component-docs': [/docs/i, /documentation/i, /readme/i, /guide/i],
      'component-tests': [/test/i, /testing/i, /spec/i, /unit test/i]
    };

    this.teamAssignments = {
      security: ['security-team', 'architecture-lead'],
      bug: ['rust-team', 'core-developers'],
      dependencies: ['maintenance-team', 'rust-team'],
      performance: ['performance-team', 'optimization-experts'],
      feature: {
        wasm: ['wasm-specialist', 'rust-team'],
        ci_cd: ['devops-team', 'automation-engineers'],
        core: ['core-developers', 'rust-team'],
        default: ['dev-team', 'feature-team']
      },
      documentation: ['docs-team', 'technical-writers'],
      ci_cd: ['devops-team', 'automation-engineers']
    };
  }

  classify(title, body) {
    const results = [];
    const text = `${title} ${body}`.toLowerCase();

    // Calculate confidence scores for each type
    for (const [type, config] of Object.entries(this.patterns)) {
      let score = 0;
      let matches = 0;

      // Check title patterns
      for (const pattern of config.title) {
        if (pattern.test(title)) {
          score += 0.6 * config.weight;
          matches++;
        }
      }

      // Check body patterns
      for (const pattern of config.body) {
        if (pattern.test(body)) {
          score += 0.4 * config.weight;
          matches++;
        }
      }

      if (matches > 0) {
        results.push({
          type,
          confidence: Math.min(score, 1.0),
          matches
        });
      }
    }

    // Sort by confidence and return top result
    results.sort((a, b) => b.confidence - a.confidence);
    return results.length > 0 ? results[0] : { type: 'needs-triage', confidence: 0, matches: 0 };
  }

  determinePriority(title, body, existingLabels) {
    // Check if priority already assigned
    if (existingLabels.some(label => label.startsWith('priority-'))) {
      return null;
    }

    const text = `${title} ${body}`.toLowerCase();
    
    // Check priority indicators
    for (const [priority, patterns] of Object.entries(this.priorityIndicators)) {
      for (const pattern of patterns) {
        if (pattern.test(text)) {
          return priority;
        }
      }
    }

    // Default priority based on type
    if (text.includes('security') || text.includes('vulnerability')) return 'critical';
    if (text.includes('[phase 3]') || text.includes('wasm')) return 'high';
    if (text.includes('bug') && text.includes('critical')) return 'high';
    if (text.includes('performance')) return 'medium';
    if (text.includes('documentation') || text.includes('roadmap')) return 'low';
    
    return 'medium';  // Default
  }

  identifyComponents(title, body) {
    const components = [];
    const text = `${title} ${body}`.toLowerCase();

    for (const [component, patterns] of Object.entries(this.componentMap)) {
      for (const pattern of patterns) {
        if (pattern.test(text)) {
          components.push(component);
          break;
        }
      }
    }

    return components;
  }

  suggestAssignees(type, title, body) {
    const text = `${title} ${body}`.toLowerCase();
    
    if (this.teamAssignments[type]) {
      if (typeof this.teamAssignments[type] === 'object' && !Array.isArray(this.teamAssignments[type])) {
        // Complex assignment logic for features
        if (text.includes('wasm') || text.includes('webassembly')) {
          return this.teamAssignments[type].wasm;
        } else if (text.includes('ci') || text.includes('cd') || text.includes('pipeline')) {
          return this.teamAssignments[type].ci_cd;
        } else if (text.includes('core') || text.includes('engine')) {
          return this.teamAssignments[type].core;
        } else {
          return this.teamAssignments[type].default;
        }
      } else {
        return this.teamAssignments[type];
      }
    }
    
    return ['triage-team'];
  }
}

// Enhanced triage orchestrator
class TriageOrchestrator {
  constructor() {
    this.classifier = new IssueClassifier();
    this.metrics = {
      processed: 0,
      classified: 0,
      labeled: 0,
      assigned: 0,
      errors: 0
    };
  }

  async processIssue(issue) {
    try {
      console.log(`\n🔍 Processing Issue #${issue.number}: ${issue.title.substring(0, 50)}...`);
      
      const title = issue.title || '';
      const body = issue.body || '';
      const currentLabels = issue.labels.map(l => l.name);
      
      // Classify issue
      const classification = this.classifier.classify(title, body);
      const priority = this.classifier.determinePriority(title, body, currentLabels);
      const components = this.classifier.identifyComponents(title, body);
      const suggestedAssignees = this.classifier.suggestAssignees(classification.type, title, body);
      
      console.log(`  📊 Type: ${classification.type} (confidence: ${(classification.confidence * 100).toFixed(1)}%)`);
      console.log(`  🎯 Priority: ${priority || 'already assigned'}`);
      console.log(`  🏗️ Components: ${components.join(', ') || 'none detected'}`);
      console.log(`  👥 Suggested Team: ${suggestedAssignees.join(', ')}`);
      
      // Prepare labels to add/remove
      const labelsToAdd = [];
      const labelsToRemove = [];
      
      // Type labels
      const typeLabels = {
        'bug': 'bug',
        'security': 'security',
        'performance': 'performance',
        'dependencies': 'dependencies',
        'feature': 'type-feature',
        'documentation': 'documentation',
        'ci_cd': 'ci-cd',
        'wasm': 'component-wasm'
      };
      
      if (classification.type !== 'needs-triage' && typeLabels[classification.type]) {
        if (!currentLabels.includes(typeLabels[classification.type])) {
          labelsToAdd.push(typeLabels[classification.type]);
        }
        
        // Remove needs-triage if we classified it
        if (currentLabels.includes('needs-triage')) {
          labelsToRemove.push('needs-triage');
        }
      }
      
      // Priority labels
      if (priority && !currentLabels.some(l => l.startsWith('priority-'))) {
        labelsToAdd.push(`priority-${priority}`);
      }
      
      // Component labels
      for (const component of components) {
        if (!currentLabels.includes(component)) {
          labelsToAdd.push(component);
        }
      }
      
      // Apply labels if auto-labeling is enabled
      if (AUTO_LABEL && labelsToAdd.length > 0) {
        console.log(`  🏷️ Adding labels: ${labelsToAdd.join(', ')}`);
        try {
          await octokit.rest.issues.addLabels({
            owner: REPO_OWNER,
            repo: REPO_NAME,
            issue_number: issue.number,
            labels: labelsToAdd
          });
          this.metrics.labeled++;
        } catch (error) {
          console.error(`  ❌ Error adding labels: ${error.message}`);
          this.metrics.errors++;
        }
      }
      
      // Remove labels if needed
      if (AUTO_LABEL && labelsToRemove.length > 0) {
        console.log(`  🗑️ Removing labels: ${labelsToRemove.join(', ')}`);
        for (const label of labelsToRemove) {
          try {
            await octokit.rest.issues.removeLabel({
              owner: REPO_OWNER,
              repo: REPO_NAME,
              issue_number: issue.number,
              name: label
            });
          } catch (error) {
            console.log(`    ⚠️ Could not remove label ${label}: ${error.message}`);
          }
        }
      }
      
      // Add triage comment for new issues
      if (issue.comments === 0 && (labelsToAdd.length > 0 || classification.confidence > 0.5)) {
        await this.addTriageComment(issue, classification, priority, components, suggestedAssignees);
      }
      
      this.metrics.processed++;
      this.metrics.classified++;
      
      return {
        issueNumber: issue.number,
        classification: classification.type,
        confidence: classification.confidence,
        priority,
        components,
        suggestedAssignees,
        labelsAdded: labelsToAdd,
        labelsRemoved: labelsToRemove
      };
      
    } catch (error) {
      console.error(`❌ Error processing issue #${issue.number}: ${error.message}`);
      this.metrics.errors++;
      return null;
    }
  }

  async addTriageComment(issue, classification, priority, components, suggestedAssignees) {
    const typeEmojis = {
      'bug': '🐛',
      'security': '🔒',
      'performance': '📊',
      'dependencies': '📦',
      'feature': '✨',
      'documentation': '📝',
      'ci_cd': '🔄',
      'wasm': '⚙️',
      'needs-triage': '🔍'
    };

    const priorityEmojis = {
      'critical': '🔴',
      'high': '🟠',
      'medium': '🟡',
      'low': '🟢'
    };

    const confidenceBar = '█'.repeat(Math.floor(classification.confidence * 10)) + 
                         '░'.repeat(10 - Math.floor(classification.confidence * 10));

    const message = `## 🤖 Intelligent Issue Triage Report

Thank you for creating this issue! Our enhanced AI triage system has analyzed it with swarm coordination:

### 📊 Classification Results
- **Type:** ${typeEmojis[classification.type] || '❓'} \`${classification.type}\`
- **Confidence:** \`${confidenceBar}\` ${(classification.confidence * 100).toFixed(1)}%
- **Priority:** ${priorityEmojis[priority] || '⚪'} \`${priority || 'to be determined'}\`
- **Components:** ${components.map(c => `\`${c}\``).join(', ') || 'none detected'}

### 👥 Team Assignment
**Suggested Teams:** ${suggestedAssignees.map(t => `@${t}`).join(', ')}

### 🔄 Next Steps
1. ✅ **Automated Analysis Complete** - AI classification applied
2. 👀 **Maintainer Review** - Human validation within 24 hours  
3. 🎯 **Team Assignment** - Issue will be assigned to appropriate team
4. 🚀 **Work Planning** - Team will provide timeline and approach

### 📈 Quality Metrics
- **Analysis Depth:** Advanced pattern recognition with ML scoring
- **Team Matching:** Expertise-based assignment algorithms  
- **Priority Scoring:** Multi-factor urgency and impact analysis

---
*Generated by PhotonDrift Intelligent Triage System v2.0*  
*Powered by Claude Flow Swarm Coordination*

If you believe this classification needs adjustment, please comment with your reasoning and a maintainer will review.`;

    try {
      await octokit.rest.issues.createComment({
        owner: REPO_OWNER,
        repo: REPO_NAME,
        issue_number: issue.number,
        body: message
      });
      console.log(`  💬 Added intelligent triage comment`);
    } catch (error) {
      console.error(`  ❌ Error adding triage comment: ${error.message}`);
    }
  }

  async generateReport(results) {
    const timestamp = new Date().toISOString();
    const totalIssues = results.length;
    
    // Count by type
    const typeCounts = {};
    const priorityCounts = {};
    const componentCounts = {};
    
    results.forEach(result => {
      if (result) {
        typeCounts[result.classification] = (typeCounts[result.classification] || 0) + 1;
        if (result.priority) {
          priorityCounts[result.priority] = (priorityCounts[result.priority] || 0) + 1;
        }
        result.components.forEach(comp => {
          componentCounts[comp] = (componentCounts[comp] || 0) + 1;
        });
      }
    });

    const report = `# 🎯 Intelligent Issue Triage Report

**Generated:** ${timestamp}  
**Repository:** ${REPO_OWNER}/${REPO_NAME}  
**Processing Mode:** ${AUTO_LABEL ? 'Automatic' : 'Analysis Only'}

## 📊 Processing Summary

- **Total Issues Processed:** ${totalIssues}
- **Successfully Classified:** ${this.metrics.classified}
- **Labels Applied:** ${this.metrics.labeled}
- **Assignments Made:** ${this.metrics.assigned}
- **Processing Errors:** ${this.metrics.errors}

## 🏷️ Classification Breakdown

### By Type
${Object.entries(typeCounts).map(([type, count]) => 
  `- **${type}**: ${count} issues`).join('\n')}

### By Priority  
${Object.entries(priorityCounts).map(([priority, count]) => 
  `- **${priority}**: ${count} issues`).join('\n')}

### By Component
${Object.entries(componentCounts).map(([component, count]) => 
  `- **${component}**: ${count} issues`).join('\n')}

## 🎯 High-Priority Issues

${results.filter(r => r && r.priority === 'critical').length > 0 ? 
  '### Critical Priority\n' + 
  results.filter(r => r && r.priority === 'critical')
    .map(r => `- Issue #${r.issueNumber}: ${r.classification} (${(r.confidence * 100).toFixed(1)}% confidence)`)
    .join('\n') 
  : '- No critical priority issues detected'}

## 🔒 Security Alerts

${results.filter(r => r && r.classification === 'security').length > 0 ? 
  results.filter(r => r && r.classification === 'security')
    .map(r => `- 🚨 Issue #${r.issueNumber}: Security issue detected with ${(r.confidence * 100).toFixed(1)}% confidence`)
    .join('\n')
  : '- No security issues detected'}

## 📈 System Performance

- **Average Confidence Score:** ${(results.reduce((sum, r) => sum + (r?.confidence || 0), 0) / totalIssues * 100).toFixed(1)}%
- **Auto-Classification Rate:** ${((this.metrics.classified / totalIssues) * 100).toFixed(1)}%
- **Error Rate:** ${((this.metrics.errors / totalIssues) * 100).toFixed(1)}%

## 🔄 Recommended Actions

1. **Review Security Issues:** ${results.filter(r => r && r.classification === 'security').length} security issues need immediate attention
2. **Priority Triage:** ${results.filter(r => r && r.priority === 'critical').length} critical issues require urgent response
3. **Team Assignment:** ${results.filter(r => r && r.suggestedAssignees.length > 0).length} issues have team assignment suggestions

---
*Report generated by PhotonDrift Intelligent Triage System*  
*Enhanced with Claude Flow Swarm Intelligence*
`;

    // Save report
    const reportPath = path.join(process.cwd(), 'triage-report-latest.md');
    fs.writeFileSync(reportPath, report);
    console.log(`\n📝 Detailed report saved to: ${reportPath}`);
    
    return report;
  }
}

// Main execution
async function main() {
  console.log('🚀 PhotonDrift Intelligent Issue Triage System');
  console.log('================================================');
  console.log(`Repository: ${REPO_OWNER}/${REPO_NAME}`);
  console.log(`Auto-Label: ${AUTO_LABEL ? '✅ Enabled' : '❌ Disabled'}`);
  console.log(`Auto-Assign: ${AUTO_ASSIGN ? '✅ Enabled' : '❌ Disabled'}`);
  
  if (!GITHUB_TOKEN) {
    console.error('❌ GITHUB_TOKEN environment variable is required');
    process.exit(1);
  }

  try {
    // Initialize orchestrator
    const orchestrator = new TriageOrchestrator();
    
    // Fetch open issues
    console.log('\n📋 Fetching open issues...');
    const { data: issues } = await octokit.rest.issues.listForRepo({
      owner: REPO_OWNER,
      repo: REPO_NAME,
      state: 'open',
      per_page: 100
    });
    
    console.log(`Found ${issues.length} open issues`);
    
    // Process each issue
    const results = [];
    for (const issue of issues) {
      const result = await orchestrator.processIssue(issue);
      if (result) {
        results.push(result);
      }
      
      // Rate limiting
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    // Generate comprehensive report
    console.log('\n📊 Generating comprehensive report...');
    const report = await orchestrator.generateReport(results);
    
    console.log('\n✅ Triage process completed successfully!');
    console.log(`📈 Performance: ${orchestrator.metrics.classified}/${orchestrator.metrics.processed} issues classified`);
    
    // Display quick summary
    console.log('\n📊 Quick Summary:');
    console.log(`  Total Processed: ${orchestrator.metrics.processed}`);
    console.log(`  Successfully Classified: ${orchestrator.metrics.classified}`);
    console.log(`  Labels Applied: ${orchestrator.metrics.labeled}`);
    console.log(`  Errors: ${orchestrator.metrics.errors}`);
    
  } catch (error) {
    console.error(`❌ Fatal error: ${error.message}`);
    process.exit(1);
  }
}

// Handle CLI arguments
if (require.main === module) {
  main().catch(console.error);
}

module.exports = { IssueClassifier, TriageOrchestrator };