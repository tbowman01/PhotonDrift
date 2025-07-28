//! VS Code specific plugin implementation

use crate::plugins::ide::{CommonIDEFeatures, IDECapabilities, TextSelection};
use crate::plugins::{
    ArgumentType, CommandArgument, DiagnosticSeverity, IDEAction, IDECommand, IDEConfig,
    IDEDiagnostic, IDEEvent, IDEIntegrationPlugin, IDEResponse, IDEType, MessageLevel, Plugin,
    PluginCapability, PluginContext, PluginMetadata, PluginResponse,
};
use crate::{AdrscanError, Result};
use chrono::Utc;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// VS Code integration plugin
#[derive(Debug)]
pub struct VSCodePlugin {
    metadata: PluginMetadata,
    config: Option<IDEConfig>,
    capabilities: IDECapabilities,
}

/// VS Code specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeConfig {
    pub workspace_path: Option<std::path::PathBuf>,
    pub extension_id: String,
    pub enable_code_lens: bool,
    pub enable_hover_info: bool,
    pub enable_diagnostics: bool,
    pub auto_refresh_interval: u32,
}

/// VS Code extension manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeManifest {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub publisher: String,
    pub description: String,
    pub engines: VSCodeEngines,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub contributes: VSCodeContributes,
}

/// VS Code engine requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeEngines {
    pub vscode: String,
}

/// VS Code contribution points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeContributes {
    pub commands: Vec<VSCodeCommand>,
    pub languages: Vec<VSCodeLanguage>,
    pub grammars: Vec<VSCodeGrammar>,
    pub configuration: VSCodeConfiguration,
}

/// VS Code command contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeCommand {
    pub command: String,
    pub title: String,
    pub category: String,
    pub icon: Option<String>,
}

/// VS Code language contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeLanguage {
    pub id: String,
    pub aliases: Vec<String>,
    pub extensions: Vec<String>,
    pub configuration: String,
}

/// VS Code grammar contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeGrammar {
    pub language: String,
    pub scope_name: String,
    pub path: String,
}

/// VS Code configuration contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeConfiguration {
    pub title: String,
    pub properties: HashMap<String, VSCodeProperty>,
}

/// VS Code configuration property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VSCodeProperty {
    pub property_type: String,
    pub default: serde_json::Value,
    pub description: String,
    pub scope: Option<String>,
}

impl VSCodePlugin {
    /// Create a new VS Code plugin
    pub fn new() -> Self {
        let metadata = PluginMetadata {
            id: "vscode-integration".to_string(),
            name: "PhotonDrift VS Code Extension".to_string(),
            version: "1.0.0".to_string(),
            description: "VS Code integration for PhotonDrift ADR analysis".to_string(),
            author: "PhotonDrift Team".to_string(),
            license: "MIT".to_string(),
            homepage: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            repository: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            keywords: vec![
                "vscode".to_string(),
                "adr".to_string(),
                "photondrift".to_string(),
            ],
            api_version: crate::plugins::PLUGIN_API_VERSION.to_string(),
            min_adrscan_version: "0.2.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let capabilities = IDECapabilities {
            supports_lsp: true,
            supports_debugging: false,
            supports_extensions: true,
            supports_git_integration: true,
            supports_terminal: true,
            supported_languages: vec![
                "markdown".to_string(),
                "yaml".to_string(),
                "json".to_string(),
            ],
        };

        Self {
            metadata,
            config: None,
            capabilities,
        }
    }

    /// Generate VS Code extension manifest
    pub fn generate_manifest(&self) -> VSCodeManifest {
        VSCodeManifest {
            name: "photondrift-vscode".to_string(),
            display_name: "PhotonDrift ADR Analyzer".to_string(),
            version: "1.0.0".to_string(),
            publisher: "photondrift-team".to_string(),
            description: "Architecture Decision Record analysis and management for VS Code"
                .to_string(),
            engines: VSCodeEngines {
                vscode: "^1.60.0".to_string(),
            },
            categories: vec![
                "Other".to_string(),
                "Linters".to_string(),
                "Documentation".to_string(),
            ],
            keywords: vec![
                "adr".to_string(),
                "architecture".to_string(),
                "documentation".to_string(),
                "analysis".to_string(),
            ],
            contributes: self.generate_contributes(),
        }
    }

    /// Generate TypeScript extension code
    pub fn generate_extension_code(&self) -> String {
        r#"import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export function activate(context: vscode.ExtensionContext) {
    console.log('PhotonDrift extension is now active!');

    // Register commands
    const analyzeCommand = vscode.commands.registerCommand(
        'photondrift.analyzeCurrentFile',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('No active editor');
                return;
            }

            const document = editor.document;
            if (!document.fileName.endsWith('.md')) {
                vscode.window.showWarningMessage('PhotonDrift analysis is optimized for Markdown files');
                return;
            }

            try {
                await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: 'Analyzing ADR with PhotonDrift...',
                }, async (progress) => {
                    const { stdout, stderr } = await execAsync(
                        `adrscan analyze "${document.fileName}"`,
                        { cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath }
                    );

                    if (stderr) {
                        console.error('PhotonDrift stderr:', stderr);
                    }

                    // Parse analysis results
                    const results = JSON.parse(stdout);
                    showAnalysisResults(results);
                });
            } catch (error) {
                vscode.window.showErrorMessage(`PhotonDrift analysis failed: ${error}`);
            }
        }
    );

    const scanProjectCommand = vscode.commands.registerCommand(
        'photondrift.scanProject',
        async () => {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (!workspaceFolder) {
                vscode.window.showErrorMessage('No workspace folder open');
                return;
            }

            try {
                await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: 'Scanning project for ADRs...',
                }, async (progress) => {
                    const { stdout } = await execAsync(
                        'adrscan scan .',
                        { cwd: workspaceFolder.uri.fsPath }
                    );

                    const results = JSON.parse(stdout);
                    showScanResults(results);
                });
            } catch (error) {
                vscode.window.showErrorMessage(`PhotonDrift scan failed: ${error}`);
            }
        }
    );

    const initProjectCommand = vscode.commands.registerCommand(
        'photondrift.initProject',
        async () => {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (!workspaceFolder) {
                vscode.window.showErrorMessage('No workspace folder open');
                return;
            }

            try {
                await execAsync(
                    'adrscan init',
                    { cwd: workspaceFolder.uri.fsPath }
                );

                vscode.window.showInformationMessage('PhotonDrift initialized for this project');
            } catch (error) {
                vscode.window.showErrorMessage(`PhotonDrift init failed: ${error}`);
            }
        }
    );

    // Register hover provider for ADR links
    const hoverProvider = vscode.languages.registerHoverProvider(
        'markdown',
        new PhotonDriftHoverProvider()
    );

    // Register diagnostic provider
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('photondrift');
    context.subscriptions.push(diagnosticCollection);

    // Auto-analyze on save
    const onSave = vscode.workspace.onDidSaveTextDocument(async (document) => {
        if (document.fileName.endsWith('.md')) {
            await updateDiagnostics(document, diagnosticCollection);
        }
    });

    context.subscriptions.push(
        analyzeCommand,
        scanProjectCommand,
        initProjectCommand,
        hoverProvider,
        onSave
    );
}

class PhotonDriftHoverProvider implements vscode.HoverProvider {
    async provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): Promise<vscode.Hover | undefined> {
        const range = document.getWordRangeAtPosition(position);
        if (!range) return;

        const word = document.getText(range);
        
        // Check if hovering over ADR reference
        if (word.match(/ADR-?\d+/i)) {
            try {
                const { stdout } = await execAsync(
                    `adrscan info "${word}"`,
                    { cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath }
                );

                const info = JSON.parse(stdout);
                const markdown = new vscode.MarkdownString();
                markdown.appendMarkdown(`**${info.title}**\n\n`);
                markdown.appendMarkdown(`Status: ${info.status}\n\n`);
                markdown.appendMarkdown(`${info.summary}`);

                return new vscode.Hover(markdown);
            } catch (error) {
                console.error('Failed to get ADR info:', error);
            }
        }
    }
}

async function updateDiagnostics(
    document: vscode.TextDocument,
    collection: vscode.DiagnosticCollection
) {
    try {
        const { stdout } = await execAsync(
            `adrscan analyze "${document.fileName}" --format json`,
            { cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath }
        );

        const analysis = JSON.parse(stdout);
        const diagnostics: vscode.Diagnostic[] = [];

        for (const issue of analysis.issues || []) {
            const range = new vscode.Range(
                issue.line - 1, issue.column,
                issue.line - 1, issue.column + issue.length
            );

            const severity = issue.severity === 'error' 
                ? vscode.DiagnosticSeverity.Error
                : issue.severity === 'warning'
                ? vscode.DiagnosticSeverity.Warning
                : vscode.DiagnosticSeverity.Information;

            const diagnostic = new vscode.Diagnostic(
                range,
                issue.message,
                severity
            );

            diagnostic.source = 'PhotonDrift';
            diagnostic.code = issue.code;
            diagnostics.push(diagnostic);
        }

        collection.set(document.uri, diagnostics);
    } catch (error) {
        console.error('Failed to update diagnostics:', error);
    }
}

function showAnalysisResults(results: any) {
    const panel = vscode.window.createWebviewPanel(
        'photondriftAnalysis',
        'PhotonDrift Analysis Results',
        vscode.ViewColumn.Two,
        { enableScripts: true }
    );

    panel.webview.html = generateAnalysisHTML(results);
}

function showScanResults(results: any) {
    const panel = vscode.window.createWebviewPanel(
        'photondriftScan',
        'PhotonDrift Project Scan',
        vscode.ViewColumn.Two,
        { enableScripts: true }
    );

    panel.webview.html = generateScanHTML(results);
}

function generateAnalysisHTML(results: any): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <title>PhotonDrift Analysis</title>
        <style>
            body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
            .header { background: #007acc; color: white; padding: 16px; margin: -16px -16px 16px -16px; }
            .issue { margin: 8px 0; padding: 8px; border-left: 4px solid #ccc; }
            .issue.error { border-color: #e74c3c; }
            .issue.warning { border-color: #f39c12; }
            .issue.info { border-color: #3498db; }
        </style>
    </head>
    <body>
        <div class="header">
            <h1>PhotonDrift Analysis Results</h1>
        </div>
        <div class="content">
            ${results.issues?.map((issue: any) => `
                <div class="issue ${issue.severity}">
                    <strong>${issue.message}</strong><br>
                    <small>Line ${issue.line}: ${issue.description}</small>
                </div>
            `).join('') || '<p>No issues found.</p>'}
        </div>
    </body>
    </html>`;
}

function generateScanHTML(results: any): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <title>PhotonDrift Scan Results</title>
        <style>
            body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
            .header { background: #007acc; color: white; padding: 16px; margin: -16px -16px 16px -16px; }
            .adr { margin: 8px 0; padding: 12px; border: 1px solid #ddd; border-radius: 4px; }
            .status { display: inline-block; padding: 2px 8px; border-radius: 3px; font-size: 12px; }
            .status.accepted { background: #2ecc71; color: white; }
            .status.proposed { background: #f39c12; color: white; }
            .status.deprecated { background: #e74c3c; color: white; }
        </style>
    </head>
    <body>
        <div class="header">
            <h1>PhotonDrift Project Scan</h1>
            <p>Found ${results.total || 0} ADRs</p>
        </div>
        <div class="content">
            ${results.adrs?.map((adr: any) => `
                <div class="adr">
                    <h3>${adr.title}</h3>
                    <span class="status ${adr.status.toLowerCase()}">${adr.status}</span>
                    <p>${adr.summary}</p>
                    <small>File: ${adr.file}</small>
                </div>
            `).join('') || '<p>No ADRs found in this project.</p>'}
        </div>
    </body>
    </html>`;
}

export function deactivate() {}
"#.to_string()
    }

    /// Generate package.json for the extension
    pub fn generate_package_json(&self) -> String {
        serde_json::to_string_pretty(&self.generate_manifest()).unwrap_or_default()
    }

    fn generate_contributes(&self) -> VSCodeContributes {
        VSCodeContributes {
            commands: vec![
                VSCodeCommand {
                    command: "photondrift.analyzeCurrentFile".to_string(),
                    title: "Analyze Current ADR File".to_string(),
                    category: "PhotonDrift".to_string(),
                    icon: Some("$(search)".to_string()),
                },
                VSCodeCommand {
                    command: "photondrift.scanProject".to_string(),
                    title: "Scan Project for ADRs".to_string(),
                    category: "PhotonDrift".to_string(),
                    icon: Some("$(file-directory)".to_string()),
                },
                VSCodeCommand {
                    command: "photondrift.initProject".to_string(),
                    title: "Initialize PhotonDrift".to_string(),
                    category: "PhotonDrift".to_string(),
                    icon: Some("$(rocket)".to_string()),
                },
            ],
            languages: vec![VSCodeLanguage {
                id: "adr-markdown".to_string(),
                aliases: vec!["ADR Markdown".to_string(), "adr".to_string()],
                extensions: vec![".adr.md".to_string(), ".md".to_string()],
                configuration: "./language-configuration.json".to_string(),
            }],
            grammars: vec![VSCodeGrammar {
                language: "adr-markdown".to_string(),
                scope_name: "text.html.markdown.adr".to_string(),
                path: "./syntaxes/adr-markdown.tmGrammar.json".to_string(),
            }],
            configuration: VSCodeConfiguration {
                title: "PhotonDrift".to_string(),
                properties: HashMap::from([
                    (
                        "photondrift.enableRealTimeAnalysis".to_string(),
                        VSCodeProperty {
                            property_type: "boolean".to_string(),
                            default: serde_json::Value::Bool(true),
                            description: "Enable real-time ADR analysis as you type".to_string(),
                            scope: Some("resource".to_string()),
                        },
                    ),
                    (
                        "photondrift.autoScanInterval".to_string(),
                        VSCodeProperty {
                            property_type: "number".to_string(),
                            default: serde_json::Value::Number(serde_json::Number::from(30)),
                            description: "Auto-scan interval in seconds (0 to disable)".to_string(),
                            scope: Some("resource".to_string()),
                        },
                    ),
                ]),
            },
        }
    }
}

impl Plugin for VSCodePlugin {
    fn initialize(&mut self, context: &PluginContext) -> Result<()> {
        info!("Initializing VS Code plugin");

        // Set default configuration
        self.config =
            Some(crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::VSCode));

        debug!("VS Code plugin initialized successfully");
        Ok(())
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![
            PluginCapability::IDEIntegration,
            PluginCapability::FileWatcher,
            PluginCapability::CommandExtension,
        ]
    }

    fn execute(&self, command: &str, params: &HashMap<String, String>) -> Result<PluginResponse> {
        debug!("Executing VS Code command: {}", command);

        match command {
            "generate_extension" => {
                let extension_code = self.generate_extension_code();
                let package_json = self.generate_package_json();

                Ok(PluginResponse {
                    success: true,
                    data: Some(serde_json::json!({
                        "extension_code": extension_code,
                        "package_json": package_json,
                        "manifest": self.generate_manifest()
                    })),
                    message: Some("VS Code extension files generated successfully".to_string()),
                    warnings: vec![],
                    errors: vec![],
                })
            }
            "analyze_file" => {
                let file_path = params.get("file_path").unwrap_or("");
                Ok(PluginResponse {
                    success: true,
                    data: Some(serde_json::json!({
                        "analyzed_file": file_path,
                        "diagnostics": []
                    })),
                    message: Some(format!("Analyzed file: {}", file_path)),
                    warnings: vec![],
                    errors: vec![],
                })
            }
            _ => Ok(PluginResponse {
                success: false,
                data: None,
                message: Some(format!("Unknown command: {}", command)),
                warnings: vec![],
                errors: vec![format!(
                    "Command '{}' not supported by VS Code plugin",
                    command
                )],
            }),
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down VS Code plugin");
        Ok(())
    }
}

impl IDEIntegrationPlugin for VSCodePlugin {
    fn ide_type(&self) -> IDEType {
        IDEType::VSCode
    }

    fn setup_ide_integration(&self, config: &IDEConfig) -> Result<()> {
        info!("Setting up VS Code integration with config: {:?}", config);
        // Implementation would set up VS Code specific integration
        Ok(())
    }

    fn handle_ide_event(&self, event: &IDEEvent) -> Result<IDEResponse> {
        debug!("Handling VS Code event: {:?}", event);

        match event {
            IDEEvent::FileOpened { path } => Ok(IDEResponse {
                handled: true,
                actions: vec![IDEAction::ShowMessage {
                    level: MessageLevel::Info,
                    message: format!("PhotonDrift: Analyzing {}", path.display()),
                }],
                diagnostics: vec![],
            }),
            IDEEvent::FileSaved { path } => Ok(IDEResponse {
                handled: true,
                actions: vec![IDEAction::ShowProgress {
                    title: "PhotonDrift Analysis".to_string(),
                    message: "Analyzing saved ADR file...".to_string(),
                    percentage: None,
                }],
                diagnostics: vec![],
            }),
            _ => Ok(IDEResponse {
                handled: false,
                actions: vec![],
                diagnostics: vec![],
            }),
        }
    }

    fn get_ide_config(&self) -> IDEConfig {
        self.config.clone().unwrap_or_else(|| {
            crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::VSCode)
        })
    }

    fn register_commands(&self) -> Vec<IDECommand> {
        vec![
            IDECommand {
                id: "photondrift.analyzeFile".to_string(),
                title: "Analyze ADR File".to_string(),
                category: "PhotonDrift".to_string(),
                description: Some("Analyze the current ADR file for drift patterns".to_string()),
                arguments: vec![CommandArgument {
                    name: "file_path".to_string(),
                    arg_type: ArgumentType::File,
                    description: Some("Path to the ADR file".to_string()),
                    required: false,
                    default_value: None,
                }],
            },
            IDECommand {
                id: "photondrift.scanProject".to_string(),
                title: "Scan Project".to_string(),
                category: "PhotonDrift".to_string(),
                description: Some("Scan the entire project for ADR files".to_string()),
                arguments: vec![],
            },
        ]
    }
}

impl CommonIDEFeatures for VSCodePlugin {
    fn show_notification(&self, message: &str, level: MessageLevel) -> Result<()> {
        debug!("VS Code notification: {} (level: {:?})", message, level);
        // In a real implementation, this would send commands to VS Code
        Ok(())
    }

    fn open_file(&self, path: &Path, line: Option<u32>) -> Result<()> {
        debug!("VS Code open file: {} at line {:?}", path.display(), line);
        // In a real implementation, this would send commands to VS Code
        Ok(())
    }

    fn insert_text(&self, path: &Path, line: u32, column: u32, text: &str) -> Result<()> {
        debug!(
            "VS Code insert text at {}:{}:{}: {}",
            path.display(),
            line,
            column,
            text
        );
        // In a real implementation, this would send commands to VS Code
        Ok(())
    }

    fn get_selection(&self) -> Result<Option<TextSelection>> {
        // In a real implementation, this would get the current selection from VS Code
        Ok(None)
    }

    fn set_status_message(&self, message: &str) -> Result<()> {
        debug!("VS Code status message: {}", message);
        // In a real implementation, this would update the VS Code status bar
        Ok(())
    }
}

impl Default for VSCodePlugin {
    fn default() -> Self {
        Self::new()
    }
}
