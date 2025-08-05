---
id: "issue-triage-guide"
title: "ISSUE TRIAGE GUIDE"
sidebar_label: "ISSUE TRIAGE GUIDE"
sidebar_position: "1"
description: "Development guides and contributing guidelines"
slug: "/development/issue-triage-guide"
tags: ["development"]
last_update:
  date: "2025-07-29"
  author: "tbowman01"
---

# 📋 Issue Triage Guide - PhotonDrift

## Overview

This guide documents our intelligent issue triage system that automatically classifies, labels, and routes issues to the appropriate team members.

## 🤖 Automated Triage System

### How It Works

1. **Automatic Classification**: When an issue is created or edited, our system analyzes:
   - Issue title patterns
   - Body content keywords
   - Existing labels
   - Context clues

2. **Intelligent Labeling**: Based on classification, the system applies:
   - Type labels (bug, feature, security, etc.)
   - Priority labels (high, medium, low)
   - Component labels (wasm, ci-cd, dependencies)

3. **Smart Assignment**: Suggests appropriate team members based on:
   - Issue type
   - Required expertise
   - Component affected

## 📊 Classification Categories

### Issue Types

| Type | Trigger Keywords | Label Applied | Team Assignment |
|------|-----------------|---------------|-----------------|
| 🐛 Bug | `[BUG]`, error, broken, fix | `bug` | Rust Team |
| 🔒 Security | security, vulnerability, CVE | `security` | Security Team |
| 📊 Performance | performance, slow, optimization | `performance` | Performance Team |
| 📦 Dependencies | dependency, package, update | `dependencies` | Maintenance Team |
| ✨ Feature | `[PHASE]`, feature, enhancement | `type-feature` | Dev Team |
| 📝 Documentation | report, analysis, docs | `documentation` | Doc Team |

### Priority Levels

| Priority | Criteria | Label | Response Time |
|----------|----------|-------|---------------|
| 🔴 High | Security issues, Phase 3 features, Critical bugs | `priority-high` | < 24 hours |
| 🟡 Medium | Performance issues, Standard features | `priority-medium` | < 3 days |
| 🟢 Low | Enhancements, Future roadmap items | `priority-low` | < 1 week |

## 🏷️ Label Structure

### Core Labels

```yaml
Type Labels:
  - bug
  - security
  - performance
  - dependencies
  - type-feature
  - documentation

Priority Labels:
  - priority-high
  - priority-medium
  - priority-low

Status Labels:
  - needs-triage
  - in-progress
  - blocked
  - ready-for-review

Component Labels:
  - component-wasm
  - component-ml
  - ci-cd
  - platform-windows
  - platform-linux
  - platform-macos
```

## 🔄 Triage Workflow

### For New Issues

1. **Automatic Analysis** (0-5 minutes)
   - Bot analyzes issue content
   - Applies initial labels
   - Posts triage comment

2. **Human Review** (< 24 hours)
   - Maintainer validates classification
   - Adjusts labels if needed
   - Assigns to team member

3. **Team Response** (per SLA)
   - Assigned team member responds
   - Creates action plan
   - Updates issue status

### Manual Triage

Run the triage script locally:

```bash
# Analyze without making changes
./scripts/issue-triage.sh

# Apply labels automatically
AUTO_LABEL=true ./scripts/issue-triage.sh

# Apply labels and suggest assignments
AUTO_LABEL=true AUTO_ASSIGN=true ./scripts/issue-triage.sh
```

## 👥 Team Assignments

### Team Structure

| Team | Responsibilities | GitHub Teams |
|------|-----------------|--------------|
| Security Team | Security vulnerabilities, audits | `@security` |
| Rust Team | Core functionality, bugs | `@rust-devs` |
| DevOps Team | CI/CD, infrastructure | `@devops` |
| WASM Specialist | WebAssembly module | `@wasm-dev` |
| Performance Team | Optimization, benchmarks | `@perf-team` |
| Doc Team | Documentation, reports | `@docs` |

### Assignment Rules

```javascript
if (issue.type === 'security') {
  assign('@security-team-lead');
  notify('security-channel');
} else if (issue.type === 'bug' && issue.priority === 'high') {
  assign('@rust-team-lead');
  escalate();
}
```

## 📈 Metrics & Reporting

### Key Metrics

- **Response Time**: Time to first maintainer response
- **Resolution Time**: Time from creation to closure
- **Classification Accuracy**: % of correctly auto-labeled issues
- **Assignment Efficiency**: % of issues assigned to correct team first time

### Weekly Reports

Every Monday, the system generates:
- Issue volume by type
- Average response times
- Backlog analysis
- Team performance metrics

## 🛠️ Configuration

### GitHub Actions Setup

The automated triage runs via `.github/workflows/issue-triage.yml`:

```yaml
on:
  issues:
    types: [opened, edited, labeled, unlabeled]
```

### Environment Variables

```bash
# Required
GITHUB_TOKEN=<token>

# Optional
AUTO_LABEL=true
AUTO_ASSIGN=false
TRIAGE_TEAM=@triage-team
```

## 🚨 Special Handling

### Security Issues

1. Automatically marked as high priority
2. Security team notified immediately
3. Consider private security advisory
4. Follow security disclosure policy

### Phase 3 Features

1. High priority by default
2. Assigned to specialized teams
3. Tracked in project board
4. Regular status updates required

## 📝 Best Practices

### For Issue Creators

1. Use issue templates when available
2. Include clear reproduction steps for bugs
3. Add relevant context and environment info
4. Use descriptive titles with prefixes (`[BUG]`, `[FEATURE]`)

### For Maintainers

1. Review auto-triage results daily
2. Adjust labels for accuracy
3. Provide feedback on classification errors
4. Update assignment mappings as team changes

### For Contributors

1. Check existing issues before creating new ones
2. Add comments if classification seems wrong
3. Update issue when more info is available
4. Link related issues and PRs

## 🔧 Troubleshooting

### Common Issues

**Issue: Labels not being applied**
- Check GitHub Actions logs
- Verify label exists in repository
- Ensure bot has write permissions

**Issue: Wrong classification**
- Update keyword patterns in workflow
- Retrain classification algorithm
- Manual override with correct labels

**Issue: Assignment not working**
- Verify team mappings are current
- Check user permissions
- Update assignment rules

## 📚 Additional Resources

- [GitHub Issues Best Practices](https://docs.github.com/en/issues)
- [Label Management Guide](/development/labels)
- [Team Responsibilities](/development/teams)
- [Security Policy](/security)

---

*Last updated: 2025-07-21*  
*Maintained by: PhotonDrift Team*