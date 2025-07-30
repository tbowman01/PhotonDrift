---
id: "issue-management"
title: "ISSUE MANAGEMENT"
sidebar_label: "ISSUE MANAGEMENT"
sidebar_position: "1"
description: "Development guides and contributing guidelines"
slug: "/development/issue-management"
tags: ["development"]
last_update:
  date: "2025-07-29"
  author: "tbowman01"
---

# Advanced GitHub Issue Management System

PhotonDrift uses a sophisticated issue management system to track development progress across multiple phases and components.

## 🏷️ Label System

### Phase Labels
- `phase-1` - CLI MVP (Foundation)
- `phase-2` - Drift Detection (Core Features) 
- `phase-3` - WASM & GitHub Action (Integration)
- `phase-4` - Documentation & Community

### Priority Labels
- `priority-critical` - Blocking other work
- `priority-high` - Important, work on next
- `priority-medium` - Standard priority
- `priority-low` - Nice to have

### Status Labels
- `status-ready` - Ready to start work
- `status-in-progress` - Currently being worked on
- `status-blocked` - Cannot proceed (waiting for dependency)
- `status-review` - Waiting for review/feedback
- `status-completed` - Work finished

### Component Labels
- `component-cli` - Command-line interface
- `component-core` - Core engine and algorithms
- `component-config` - Configuration system
- `component-parsing` - ADR parsing logic
- `component-drift` - Drift detection
- `component-wasm` - WebAssembly module
- `component-github-action` - GitHub Action
- `component-docs` - Documentation
- `component-tests` - Testing infrastructure

### Estimate Labels
- `estimate-s` - Small (1-2 days)
- `estimate-m` - Medium (3-5 days)
- `estimate-l` - Large (1-2 weeks)
- `estimate-xl` - Extra Large (2+ weeks)

### Type Labels
- `type-feature` - New functionality
- `type-bug` - Bug fixes
- `type-enhancement` - Improvements to existing features
- `type-refactor` - Code improvements without functional changes
- `type-docs` - Documentation updates
- `type-test` - Testing improvements

## 📋 Issue Templates

### Standard Templates
1. **Feature Request** - New functionality proposals
2. **Bug Report** - Issue reporting with reproduction steps
3. **Implementation Task** - Development work tracking
4. **Epic** - Large features spanning multiple issues
5. **Research Spike** - Time-boxed investigation tasks

### Template Usage Guidelines
- **Implementation Tasks**: Use for concrete development work with clear acceptance criteria
- **Epics**: Use for large initiatives that span multiple issues and phases
- **Spikes**: Use for research, proof-of-concepts, or investigations with time limits
- **Feature Requests**: Use for new feature proposals from users/stakeholders
- **Bug Reports**: Use for defects with clear reproduction steps

## 🤖 Automation Features

### Auto-Assignment
- **Phase Labels**: Automatically assigned based on issue title
- **Priority Labels**: Auto-assigned based on content keywords
- **Complexity Estimates**: Based on acceptance criteria count and keywords

### Progress Tracking
- **Completion Tracking**: Automatic labeling and comments when issues close
- **Dependency Checking**: Notifies dependent issues when dependencies complete
- **Weekly Reports**: Automated progress reports every Monday

### Project Board Integration
- **Auto-Movement**: Issues automatically move through board columns
- **Milestone Assignment**: Auto-assign issues to phase milestones
- **PR Integration**: Pull requests linked to issues for tracking

## 📊 Metrics & Reporting

### Weekly Progress Reports
Generated automatically every Monday with:
- Phase completion percentages
- Open vs closed issue counts
- Progress bar visualizations
- Bottleneck identification

### Phase Tracking
Each phase tracked separately:
- **Phase 1**: Foundation and CLI framework
- **Phase 2**: Core drift detection functionality
- **Phase 3**: CI/CD integration and automation
- **Phase 4**: Documentation and community building

## 🔄 Workflow Integration

### Issue → PR → Completion Flow
1. **Issue Creation**: Use appropriate template with labels
2. **Work Assignment**: Assign to developer and move to "In Progress"
3. **Branch Creation**: Create feature branch referencing issue
4. **Development**: Implement with tests and documentation
5. **Pull Request**: Use PR template linking to issue
6. **Review**: Code review with acceptance criteria validation
7. **Merge**: Auto-close issue and trigger dependent notifications
8. **Automation**: Update project boards and generate reports

### Dependency Management
- Issues can declare dependencies on other issues
- Automatic notifications when dependencies complete
- Dependency blocking prevents premature work starts

## 🎯 Best Practices

### Issue Creation
- Use descriptive titles with phase prefixes: `[PHASE 1] Feature Name`
- Include detailed acceptance criteria
- Add appropriate labels (auto-assigned where possible)
- Reference related issues and dependencies
- Provide context and implementation notes

### Progress Tracking
- Update issue status as work progresses
- Comment on blockers and resolutions
- Link pull requests to issues
- Close issues only when all acceptance criteria met

### Quality Gates
- All issues require acceptance criteria
- Pull requests must reference issues
- Code review required before merge
- Tests required for new functionality
- Documentation updates for user-facing changes

## 📈 Advanced Features

### Epic Management
- Large features broken into manageable issues
- Epic progress tracking across multiple milestones
- Cross-phase epic coordination

### Research Spikes
- Time-boxed investigation tasks
- Clear deliverables and outcomes
- Decision documentation and recommendations

### Release Planning
- Milestone-based release planning
- Phase completion gates
- Feature flag coordination for gradual rollouts

---

This advanced issue management system ensures comprehensive tracking, automated workflows, and clear visibility into development progress across all phases of the PhotonDrift project.