# PhotonDrift ADR Manager

> AI-powered Architecture Decision Record management with intelligent drift detection for VS Code

[![Visual Studio Marketplace](https://img.shields.io/badge/VS%20Code-Extension-blue)](https://marketplace.visualstudio.com/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

PhotonDrift ADR Manager brings intelligent Architecture Decision Record (ADR) management directly into VS Code with AI-powered drift detection, real-time diagnostics, and comprehensive IDE integration.

### ✨ Key Features

- **🤖 AI-Enhanced Drift Detection**: Machine learning algorithms detect when code deviates from documented architectural decisions
- **⚡ Real-time LSP Integration**: Instant diagnostics, completion, and hover information for ADR files
- **🎨 Rich Syntax Highlighting**: Custom themes and syntax highlighting for ADR documents
- **📊 Analytics Dashboard**: Visual insights into architectural health and drift trends
- **🚀 Smart Templates**: Quick ADR creation with MADR and custom templates
- **🔄 Live File Watching**: Automatic drift detection as you code

## Installation

### From VS Code Marketplace
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "PhotonDrift ADR Manager"
4. Click Install

### Prerequisites
- [PhotonDrift CLI](https://github.com/tbowman01/PhotonDrift) installed and accessible in PATH
- VS Code 1.74.0 or higher

## Quick Start

1. **Initialize ADR structure**: Use Command Palette (`Ctrl+Shift+P`) → "PhotonDrift: Initialize ADR Structure"
2. **Create your first ADR**: `Ctrl+Shift+A` or use Command Palette → "PhotonDrift: Create New ADR"
3. **Start drift detection**: The extension automatically monitors your codebase for architectural drift
4. **View results**: Check the ADRs and Drift Detection panels in the Explorer

## Features

### 🏗️ ADR Management

- **Tree View Explorer**: Browse all ADRs with status indicators and metadata
- **Quick Creation**: Templates for MADR, basic, and custom ADR formats
- **Status Tracking**: Visual indicators for Proposed, Accepted, Rejected, Superseded states
- **Cross-references**: Automatic linking and validation of ADR references

### 🔍 Intelligent Drift Detection

- **Real-time Monitoring**: Continuous scanning for architectural violations
- **ML-powered Analysis**: 5 advanced algorithms (Isolation Forest, SVM, LOF, Statistical, Ensemble)
- **Categorized Results**: Organized drift detection by technology, structure, and configuration changes
- **Severity Classification**: High, medium, and low priority drift items with explanations

### 🎯 Language Server Integration

- **Instant Diagnostics**: Real-time validation of ADR structure and content
- **Smart Completion**: Context-aware suggestions for ADR sections, status values, and templates
- **Rich Hover Info**: Detailed explanations for ADR elements and references
- **Document Lifecycle**: Full support for opening, editing, and closing ADR documents

### 📊 Analytics Dashboard

- **Architecture Health Metrics**: Total ADRs, drift items, coverage percentage, and health scores
- **Visual Insights**: Charts and graphs for drift trends (coming soon)
- **Team Productivity**: Track ADR creation and maintenance patterns
- **Export Capabilities**: Generate reports for stakeholders

## Configuration

### Basic Settings

```json
{
  "photondrift.lsp.enabled": true,
  "photondrift.lsp.serverPath": "adrscan-lsp",
  "photondrift.adr.directory": "docs/adr",
  "photondrift.adr.template": "madr",
  "photondrift.drift.enabled": true,
  "photondrift.drift.watchMode": true,
  "photondrift.ml.enabled": true,
  "photondrift.ml.model": "Ensemble"
}
```

### Advanced Configuration

```json
{
  "photondrift.lsp.maxDiagnostics": 100,
  "photondrift.ui.showStatusBar": true,
  "photondrift.ui.theme": "auto",
  "photondrift.notifications.enabled": true,
  "photondrift.analytics.enabled": false
}
```

## Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| Initialize ADR Structure | - | Set up ADR directory and configuration |
| Create New ADR | `Ctrl+Shift+A` | Create new ADR from template |
| ADR Inventory | - | List all ADRs with metadata |
| Detect Architectural Drift | `Ctrl+Shift+D` | Run drift detection analysis |
| Generate ADR Proposals | - | Create ADR suggestions from detected drift |
| Generate ADR Index | - | Create comprehensive ADR index |
| Open Analytics Dashboard | - | View architecture health metrics |
| Toggle LSP Server | - | Enable/disable Language Server |

## File Types

- **`.md` files in ADR directories**: Full ADR support with syntax highlighting and LSP features
- **Configuration files**: `.adrscan.yml`, `.adrscan.yaml` support
- **Template files**: Custom ADR template support

## Themes

### PhotonDrift ADR Dark
Optimized dark theme with enhanced highlighting for:
- ADR titles and numbers
- Section headers (Status, Context, Decision, Consequences)
- Status values (Proposed, Accepted, Rejected, etc.)
- Cross-references and dates
- YAML frontmatter

### PhotonDrift ADR Light
Clean light theme with the same optimizations for daylight coding.

## Snippets

| Trigger | Description |
|---------|-------------|
| `adr-madr` | Full MADR template with frontmatter |
| `adr-basic` | Simple ADR template |
| `status` | Status section |
| `context` | Context section |
| `decision` | Decision section |
| `consequences` | Consequences section |
| `adr-ref` | ADR cross-reference |
| `supersedes` | Supersedes statement |

## Integration

### CI/CD Integration
The extension works with PhotonDrift's GitHub Action for automated architectural governance:

```yaml
- name: Architecture Check
  uses: tbowman01/PhotonDrift@main
  with:
    adr-directory: './docs/adr'
    fail-on-drift: true
```

### Team Workflows
- **ADR Reviews**: Built-in drift detection helps identify missing or outdated decisions
- **Architecture Compliance**: Real-time feedback ensures code aligns with documented decisions  
- **Decision Tracking**: Visual status indicators help teams track decision implementation

## Troubleshooting

### Common Issues

**LSP Server Not Starting**
- Verify PhotonDrift CLI is installed: `adrscan --version`
- Check server path in settings: `photondrift.lsp.serverPath`
- Enable debug logging: Set `RUST_LOG=debug` environment variable

**No Drift Detection Results**
- Ensure ADR directory exists and contains `.md` files
- Verify workspace contains `.adrscan.yml` configuration
- Check that files match include patterns in configuration

**Extension Not Activating**
- Verify VS Code version 1.74.0+
- Check for ADR files or configuration in workspace
- Use Command Palette to manually trigger PhotonDrift commands

### Debug Mode
Enable detailed logging by setting:
```json
{
  "photondrift.lsp.debug": true
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `npm test`
5. Submit a pull request

### Development Setup
```bash
git clone https://github.com/tbowman01/PhotonDrift
cd PhotonDrift/extensions/vscode
npm install
npm run compile
```

Press F5 to launch a new VS Code window with the extension loaded.

## Feedback & Support

- **Issues**: [GitHub Issues](https://github.com/tbowman01/PhotonDrift/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/tbowman01/PhotonDrift/discussions)
- **Documentation**: [PhotonDrift Docs](https://github.com/tbowman01/PhotonDrift)

## License

MIT License - see [LICENSE](../../LICENSE) for details.

---

**Enhance your architecture decisions with AI-powered insights! 🚀**