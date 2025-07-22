# Change Log

All notable changes to the PhotonDrift ADR Manager extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2024-07-22

### Updated
- **Node.js Compatibility**: Extended support to Node.js 20.x, 22.x, and 24.x for better future compatibility
- **CI/CD Pipeline**: Updated GitHub Actions workflow to test against latest Node.js LTS versions
- **Build Tools**: Updated to @vscode/vsce for secure extension packaging and publishing
- **Dependencies**: Updated @types/node to 20.x for improved TypeScript definitions

### Performance
- **Faster Builds**: Optimized for newer Node.js versions with improved performance
- **Better Caching**: Enhanced npm cache strategy in CI/CD pipeline

## [1.0.0] - 2024-07-22

### Added
- **Language Server Protocol (LSP) Integration**: Full LSP support for ADR files with real-time diagnostics, hover information, and code completion
- **AI-Powered Drift Detection**: Machine learning models for detecting architectural drift patterns
- **Five ML Algorithms**: IsolationForest, OneClassSVM, LOF, Statistical, and Ensemble models for comprehensive drift analysis
- **Advanced Syntax Highlighting**: Custom TextMate grammar for Architecture Decision Records
- **ADR Explorer View**: Tree view for browsing and managing ADRs in your project
- **Drift Detection View**: Real-time monitoring of architectural drift with detailed insights
- **Smart Code Completion**: Context-aware suggestions for ADR content and structure
- **Template System**: Built-in templates for MADR, basic, and custom ADR formats
- **Command Palette Integration**: Full set of commands for ADR lifecycle management
- **Status Bar Integration**: Real-time status updates and drift alerts
- **Dark/Light Themes**: Custom themes optimized for ADR content
- **Keyboard Shortcuts**: Efficient shortcuts for common ADR operations
- **File Watching**: Automatic drift detection on file changes
- **Analytics Dashboard**: Comprehensive analytics and reporting for architectural decisions

### Features
- **Initialize ADR Structure** (`photondrift.init`): Set up ADR directory structure in any project
- **ADR Inventory** (`photondrift.inventory`): Generate comprehensive ADR inventory
- **Detect Architectural Drift** (`photondrift.diff`): Real-time drift detection with ML analysis
- **Generate ADR Proposals** (`photondrift.propose`): AI-assisted ADR generation
- **Generate ADR Index** (`photondrift.index`): Automatic index generation and maintenance
- **Create New ADR** (`photondrift.newAdr`): Template-based ADR creation
- **Toggle LSP Server** (`photondrift.toggleLsp`): Control LSP server state
- **Open Analytics Dashboard** (`photondrift.openDashboard`): Comprehensive project analytics

### Language Support
- **ADR Markdown**: Full language support for Architecture Decision Records
- **Custom File Associations**: Automatic detection of ADR files
- **Syntax Validation**: Real-time validation of ADR structure and content
- **Content Assistance**: Smart suggestions and auto-completion

### ML & AI Features
- **Ensemble Model**: Default AI model combining multiple drift detection algorithms
- **Real-time Analysis**: Live drift detection as you type
- **Pattern Recognition**: Advanced pattern detection for architectural inconsistencies
- **Predictive Insights**: ML-powered predictions for potential architectural issues

### Configuration Options
- **LSP Integration**: Enable/disable Language Server Protocol features
- **Server Path**: Configurable path to PhotonDrift LSP server
- **Drift Detection**: Toggle real-time drift monitoring
- **Watch Mode**: Enable file watching for automatic analysis
- **ML Models**: Choose from 5 different machine learning algorithms
- **Templates**: Customizable ADR template formats
- **UI Preferences**: Theme, notifications, and display options
- **Analytics**: Optional usage analytics and telemetry

### Integration
- **Explorer Context Menus**: Right-click actions for ADR operations
- **Editor Context Menus**: In-editor ADR-specific actions
- **Command Palette**: Full command integration
- **Keybindings**: Customizable keyboard shortcuts
- **Status Bar**: Real-time status and alerts
- **View Containers**: Dedicated ADR and drift detection views

### Performance
- **Efficient Processing**: Optimized for large projects with hundreds of ADRs
- **Background Analysis**: Non-blocking drift detection
- **Caching**: Intelligent caching for improved performance
- **Resource Management**: Minimal resource usage and memory footprint

### Security
- **Secure Analysis**: Safe processing of sensitive architectural documents
- **Local Processing**: All analysis performed locally, no data sent to external servers
- **Privacy First**: No tracking or data collection without explicit consent

## [Unreleased]

### Planned
- **Plugin System**: Extensible plugin architecture for custom analysis
- **Team Collaboration**: Multi-user ADR management and approval workflows
- **Integration APIs**: REST APIs for external tool integration
- **Advanced Reporting**: Enhanced analytics and reporting capabilities
- **Cloud Sync**: Optional cloud synchronization for team environments
- **Version Control Integration**: Deep Git integration for ADR versioning
- **Compliance Frameworks**: Built-in support for enterprise compliance requirements