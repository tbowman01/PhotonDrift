---
id: "github-labels"
title: "GITHUB LABELS"
sidebar_label: "GITHUB LABELS"
sidebar_position: "1"
description: "Development guides and contributing guidelines"
slug: "/development/github-labels"
tags: ["development"]
last_update:
  date: "2025-07-29"
  author: "tbowman01"
---

# GitHub Labels Management Guide

This document describes the comprehensive label system for PhotonDrift issue and PR management.

## 🏷️ Label Categories

### Phase Labels
Track which development phase an issue belongs to:

| Label | Description | Color |
|-------|-------------|-------|
| `phase-1` | Phase 1 - CLI MVP | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |
| `phase-2` | Phase 2 - Drift Detection | ![#1D76DB](https://via.placeholder.com/15/1D76DB/000000?text=+) |
| `phase-3` | Phase 3 - WASM & GitHub Action | ![#5319E7](https://via.placeholder.com/15/5319E7/000000?text=+) |
| `phase-4` | Phase 4 - Documentation | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |

### Priority Labels
Indicate urgency and importance:

| Label | Description | Color |
|-------|-------------|-------|
| `priority-critical` | Blocking other work | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |
| `priority-high` | Important, work on next | ![#D93F0B](https://via.placeholder.com/15/D93F0B/000000?text=+) |
| `priority-medium` | Standard priority | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |
| `priority-low` | Nice to have | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |

### Status Labels
Track current state of issues:

| Label | Description | Color |
|-------|-------------|-------|
| `status-ready` | Ready to start work | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |
| `status-in-progress` | Currently being worked on | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |
| `status-blocked` | Cannot proceed | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |
| `status-review` | Waiting for review/feedback | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |
| `status-completed` | Work finished | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |
| `status-needs-info` | Needs more information | ![#D876E3](https://via.placeholder.com/15/D876E3/000000?text=+) |
| `status-duplicate` | Duplicate issue | ![#CFD3D7](https://via.placeholder.com/15/CFD3D7/000000?text=+) |
| `status-wontfix` | Will not be fixed | ![#FFFFFF](https://via.placeholder.com/15/FFFFFF/000000?text=+) |

### Component Labels
Identify which part of the system is affected:

| Label | Description | Color |
|-------|-------------|-------|
| `component-cli` | Command-line interface | ![#1D76DB](https://via.placeholder.com/15/1D76DB/000000?text=+) |
| `component-core` | Core engine and algorithms | ![#5319E7](https://via.placeholder.com/15/5319E7/000000?text=+) |
| `component-config` | Configuration system | ![#0366D6](https://via.placeholder.com/15/0366D6/000000?text=+) |
| `component-parsing` | ADR parsing logic | ![#0052CC](https://via.placeholder.com/15/0052CC/000000?text=+) |
| `component-drift` | Drift detection | ![#D4C5F9](https://via.placeholder.com/15/D4C5F9/000000?text=+) |
| `component-wasm` | WebAssembly module | ![#E36209](https://via.placeholder.com/15/E36209/000000?text=+) |
| `component-github-action` | GitHub Action integration | ![#F9C513](https://via.placeholder.com/15/F9C513/000000?text=+) |
| `component-docs` | Documentation | ![#7057FF](https://via.placeholder.com/15/7057FF/000000?text=+) |
| `component-tests` | Testing infrastructure | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |

### Estimate Labels
Rough effort estimation:

| Label | Description | Color |
|-------|-------------|-------|
| `estimate-s` | Small (1-2 days) | ![#C2E0C6](https://via.placeholder.com/15/C2E0C6/000000?text=+) |
| `estimate-m` | Medium (3-5 days) | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |
| `estimate-l` | Large (1-2 weeks) | ![#D93F0B](https://via.placeholder.com/15/D93F0B/000000?text=+) |
| `estimate-xl` | Extra Large (2+ weeks) | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |

### Type Labels
Classify the nature of work:

| Label | Description | Color |
|-------|-------------|-------|
| `type-feature` | New functionality | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |
| `type-bug` | Bug fixes | ![#D73A4A](https://via.placeholder.com/15/D73A4A/000000?text=+) |
| `type-enhancement` | Improvements to existing features | ![#A2EEEF](https://via.placeholder.com/15/A2EEEF/000000?text=+) |
| `type-refactor` | Code improvements without functional changes | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |
| `type-docs` | Documentation updates | ![#0075CA](https://via.placeholder.com/15/0075CA/000000?text=+) |
| `type-test` | Testing improvements | ![#D876E3](https://via.placeholder.com/15/D876E3/000000?text=+) |

### Special Purpose Labels
For specific workflows and issue types:

| Label | Description | Color |
|-------|-------------|-------|
| `epic` | Large feature spanning multiple issues | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |
| `research` | Research or investigation task | ![#0052CC](https://via.placeholder.com/15/0052CC/000000?text=+) |
| `spike` | Time-boxed investigation | ![#1D76DB](https://via.placeholder.com/15/1D76DB/000000?text=+) |
| `task` | Implementation task | ![#5319E7](https://via.placeholder.com/15/5319E7/000000?text=+) |
| `implementation` | Development work | ![#6F42C1](https://via.placeholder.com/15/6F42C1/000000?text=+) |

### Workflow Labels
For process management:

| Label | Description | Color |
|-------|-------------|-------|
| `good-first-issue` | Good for newcomers | ![#7057FF](https://via.placeholder.com/15/7057FF/000000?text=+) |
| `help-wanted` | Extra attention is needed | ![#008672](https://via.placeholder.com/15/008672/000000?text=+) |
| `breaking-change` | Introduces breaking changes | ![#B60205](https://via.placeholder.com/15/B60205/000000?text=+) |
| `dependencies` | Pull requests that update dependencies | ![#0366D6](https://via.placeholder.com/15/0366D6/000000?text=+) |
| `needs-triage` | Needs initial review and categorization | ![#E99695](https://via.placeholder.com/15/E99695/000000?text=+) |
| `planning` | Planning and design work | ![#C2E0C6](https://via.placeholder.com/15/C2E0C6/000000?text=+) |
| `discussion` | Needs team discussion | ![#FBCA04](https://via.placeholder.com/15/FBCA04/000000?text=+) |

### Automation Labels
For automated processes:

| Label | Description | Color |
|-------|-------------|-------|
| `progress-report` | Automated progress tracking | ![#0E8A16](https://via.placeholder.com/15/0E8A16/000000?text=+) |
| `automation` | Automated issue or PR | ![#EDEDED](https://via.placeholder.com/15/EDEDED/000000?text=+) |
| `ci` | Continuous integration | ![#F9C513](https://via.placeholder.com/15/F9C513/000000?text=+) |
| `release` | Release related | ![#D876E3](https://via.placeholder.com/15/D876E3/000000?text=+) |

## 🤖 Automated Label Assignment

The following labels are automatically assigned by GitHub Actions:

### Auto-Assigned by Title
- **Phase labels**: Issues with `[PHASE 1]`, `[PHASE 2]`, etc. in title
- **Component labels**: Based on keywords in title (cli, core, parsing, etc.)

### Auto-Assigned by Content
- **Priority labels**: Based on keywords like "critical", "urgent", "high priority"
- **Estimate labels**: Based on number of acceptance criteria and complexity keywords
- **Type labels**: Based on content analysis and keywords

### Auto-Assigned by Actions
- **Status labels**: `status-completed` when issues are closed
- **Automation labels**: `automation` for bot-created issues
- **Progress labels**: `progress-report` for weekly reports

## 📋 Label Usage Guidelines

### For Issue Creation
1. **Required Labels**: Every issue should have at least:
   - One phase label (`phase-1`, `phase-2`, etc.)
   - One priority label (`priority-high`, `priority-medium`, etc.)
   - One component label (`component-cli`, `component-core`, etc.)

2. **Optional Labels**: Add as appropriate:
   - Estimate label for effort planning
   - Type label for categorization
   - Special labels for epics, spikes, or research

### For Issue Management
1. **Status Updates**: Update status labels as work progresses
2. **Triage Process**: Use `needs-triage` for new issues requiring review
3. **Blocking Issues**: Use `status-blocked` with explanation in comments

### For Pull Requests
1. **Link to Issues**: Always reference related issues
2. **Component Labels**: Match the components being modified
3. **Breaking Changes**: Use `breaking-change` for API changes

## 🔄 Label Lifecycle

### Issue Creation → Resolution
```
needs-triage → status-ready → status-in-progress → status-review → status-completed
```

### Alternative Paths
```
needs-triage → status-needs-info → status-ready
status-in-progress → status-blocked → status-ready
status-review → status-in-progress (if changes needed)
```

## 🛠️ Managing Labels

### Creating New Labels
```bash
gh label create "label-name" --description "Description" --color "HEX-COLOR"
```

### Updating Existing Labels
```bash
gh label edit "label-name" --description "New description" --color "NEW-COLOR"
```

### Bulk Operations
Use the GitHub web interface or GitHub CLI for bulk label management.

---

This comprehensive label system ensures consistent categorization, automated workflows, and clear progress tracking across all PhotonDrift development activities.