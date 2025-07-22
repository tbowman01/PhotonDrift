//! Universal LSP (Language Server Protocol) plugin implementation

use crate::plugins::{
    Plugin, IDEIntegrationPlugin, PluginMetadata, PluginCapability, PluginContext,
    PluginResponse, IDEType, IDEConfig, IDEEvent, IDEResponse, IDEAction, IDECommand,
    MessageLevel, ArgumentType, CommandArgument, IDEDiagnostic, DiagnosticSeverity,
    TextRange, TextPosition
};
use crate::plugins::ide::{CommonIDEFeatures, TextSelection, IDECapabilities};
use crate::{Result, AdrscanError};
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use chrono::Utc;
use log::{info, debug, warn, error};

/// Universal Language Server Protocol plugin for broad IDE compatibility
#[derive(Debug)]
pub struct UniversalLSPPlugin {
    metadata: PluginMetadata,
    config: Option<IDEConfig>,
    capabilities: IDECapabilities,
    server_capabilities: LSPServerCapabilities,
}

/// LSP Server Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPServerCapabilities {
    pub text_document_sync: TextDocumentSyncKind,
    pub hover_provider: bool,
    pub completion_provider: Option<CompletionOptions>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub definition_provider: bool,
    pub type_definition_provider: bool,
    pub implementation_provider: bool,
    pub references_provider: bool,
    pub document_highlight_provider: bool,
    pub document_symbol_provider: bool,
    pub workspace_symbol_provider: bool,
    pub code_action_provider: Option<CodeActionOptions>,
    pub code_lens_provider: Option<CodeLensOptions>,
    pub document_formatting_provider: bool,
    pub document_range_formatting_provider: bool,
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,
    pub rename_provider: Option<RenameOptions>,
    pub document_link_provider: Option<DocumentLinkOptions>,
    pub color_provider: bool,
    pub folding_range_provider: bool,
    pub diagnostic_provider: Option<DiagnosticOptions>,
}

/// Text Document Sync Kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextDocumentSyncKind {
    None,
    Full,
    Incremental,
}

/// LSP Completion Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub resolve_provider: bool,
    pub trigger_characters: Vec<String>,
    pub all_commit_characters: Vec<String>,
}

/// LSP Signature Help Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelpOptions {
    pub trigger_characters: Vec<String>,
    pub retrigger_characters: Vec<String>,
}

/// LSP Code Action Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionOptions {
    pub code_action_kinds: Vec<String>,
    pub resolve_provider: bool,
}

/// LSP Code Lens Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLensOptions {
    pub resolve_provider: bool,
}

/// Document On Type Formatting Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentOnTypeFormattingOptions {
    pub first_trigger_character: String,
    pub more_trigger_character: Vec<String>,
}

/// Rename Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameOptions {
    pub prepare_provider: bool,
}

/// Document Link Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLinkOptions {
    pub resolve_provider: bool,
}

/// Diagnostic Options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticOptions {
    pub identifier: Option<String>,
    pub inter_file_dependencies: bool,
    pub workspace_diagnostics: bool,
}

/// LSP Message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum LSPMessage {
    #[serde(rename = "initialize")]
    Initialize { params: InitializeParams },
    #[serde(rename = "textDocument/didOpen")]
    DidOpen { params: DidOpenTextDocumentParams },
    #[serde(rename = "textDocument/didChange")]
    DidChange { params: DidChangeTextDocumentParams },
    #[serde(rename = "textDocument/didSave")]
    DidSave { params: DidSaveTextDocumentParams },
    #[serde(rename = "textDocument/didClose")]
    DidClose { params: DidCloseTextDocumentParams },
    #[serde(rename = "textDocument/hover")]
    Hover { params: HoverParams },
    #[serde(rename = "textDocument/completion")]
    Completion { params: CompletionParams },
    #[serde(rename = "textDocument/publishDiagnostics")]
    PublishDiagnostics { params: PublishDiagnosticsParams },
}

/// Initialize Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub process_id: Option<u32>,
    pub client_info: Option<ClientInfo>,
    pub locale: Option<String>,
    pub root_path: Option<String>,
    pub root_uri: Option<String>,
    pub initialization_options: Option<Value>,
    pub capabilities: ClientCapabilities,
    pub trace: Option<String>,
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

/// Client Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

/// Client Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub workspace: Option<WorkspaceCapabilities>,
    pub text_document: Option<TextDocumentCapabilities>,
    pub general: Option<GeneralCapabilities>,
}

/// Workspace Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCapabilities {
    pub apply_edit: Option<bool>,
    pub workspace_edit: Option<WorkspaceEditCapabilities>,
    pub did_change_configuration: Option<DidChangeConfigurationCapabilities>,
    pub did_change_watched_files: Option<DidChangeWatchedFilesCapabilities>,
    pub symbol: Option<WorkspaceSymbolCapabilities>,
}

/// Workspace Edit Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEditCapabilities {
    pub document_changes: Option<bool>,
    pub resource_operations: Option<Vec<String>>,
    pub failure_handling: Option<String>,
}

/// Did Change Configuration Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidChangeConfigurationCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Did Change Watched Files Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidChangeWatchedFilesCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Workspace Symbol Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSymbolCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Text Document Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentCapabilities {
    pub synchronization: Option<TextDocumentSyncCapabilities>,
    pub completion: Option<CompletionCapabilities>,
    pub hover: Option<HoverCapabilities>,
    pub signature_help: Option<SignatureHelpCapabilities>,
    pub references: Option<ReferencesCapabilities>,
    pub document_highlight: Option<DocumentHighlightCapabilities>,
    pub document_symbol: Option<DocumentSymbolCapabilities>,
    pub formatting: Option<DocumentFormattingCapabilities>,
    pub range_formatting: Option<DocumentRangeFormattingCapabilities>,
    pub on_type_formatting: Option<DocumentOnTypeFormattingCapabilities>,
    pub definition: Option<DefinitionCapabilities>,
    pub code_action: Option<CodeActionCapabilities>,
    pub code_lens: Option<CodeLensCapabilities>,
    pub document_link: Option<DocumentLinkCapabilities>,
    pub rename: Option<RenameCapabilities>,
}

/// Text Document Sync Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentSyncCapabilities {
    pub dynamic_registration: Option<bool>,
    pub will_save: Option<bool>,
    pub will_save_wait_until: Option<bool>,
    pub did_save: Option<bool>,
}

/// Completion Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionCapabilities {
    pub dynamic_registration: Option<bool>,
    pub completion_item: Option<CompletionItemCapabilities>,
    pub completion_item_kind: Option<CompletionItemKindCapabilities>,
    pub context_support: Option<bool>,
}

/// Completion Item Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemCapabilities {
    pub snippet_support: Option<bool>,
    pub commit_characters_support: Option<bool>,
    pub documentation_format: Option<Vec<String>>,
    pub deprecated_support: Option<bool>,
    pub preselect_support: Option<bool>,
}

/// Completion Item Kind Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemKindCapabilities {
    pub value_set: Option<Vec<u32>>,
}

/// Hover Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverCapabilities {
    pub dynamic_registration: Option<bool>,
    pub content_format: Option<Vec<String>>,
}

/// Signature Help Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelpCapabilities {
    pub dynamic_registration: Option<bool>,
    pub signature_information: Option<SignatureInformationCapabilities>,
    pub context_support: Option<bool>,
}

/// Signature Information Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInformationCapabilities {
    pub documentation_format: Option<Vec<String>>,
    pub parameter_information: Option<ParameterInformationCapabilities>,
}

/// Parameter Information Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformationCapabilities {
    pub label_offset_support: Option<bool>,
}

/// References Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencesCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Document Highlight Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHighlightCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Document Symbol Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSymbolCapabilities {
    pub dynamic_registration: Option<bool>,
    pub symbol_kind: Option<SymbolKindCapabilities>,
    pub hierarchical_document_symbol_support: Option<bool>,
}

/// Symbol Kind Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolKindCapabilities {
    pub value_set: Option<Vec<u32>>,
}

/// Document Formatting Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFormattingCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Document Range Formatting Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRangeFormattingCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Document On Type Formatting Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentOnTypeFormattingCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Definition Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionCapabilities {
    pub dynamic_registration: Option<bool>,
    pub link_support: Option<bool>,
}

/// Code Action Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionCapabilities {
    pub dynamic_registration: Option<bool>,
    pub code_action_literal_support: Option<CodeActionLiteralSupport>,
    pub is_preferred_support: Option<bool>,
    pub disabled_support: Option<bool>,
    pub data_support: Option<bool>,
}

/// Code Action Literal Support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionLiteralSupport {
    pub code_action_kind: CodeActionKindCapabilities,
}

/// Code Action Kind Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionKindCapabilities {
    pub value_set: Vec<String>,
}

/// Code Lens Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLensCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// Document Link Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLinkCapabilities {
    pub dynamic_registration: Option<bool>,
    pub tooltip_support: Option<bool>,
}

/// Rename Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameCapabilities {
    pub dynamic_registration: Option<bool>,
    pub prepare_support: Option<bool>,
}

/// General Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralCapabilities {
    pub regular_expressions: Option<RegularExpressionsCapabilities>,
    pub markdown: Option<MarkdownCapabilities>,
}

/// Regular Expressions Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegularExpressionsCapabilities {
    pub engine: String,
    pub version: Option<String>,
}

/// Markdown Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownCapabilities {
    pub parser: String,
    pub version: Option<String>,
}

/// Workspace Folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

/// Did Open Text Document Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidOpenTextDocumentParams {
    pub text_document: TextDocumentItem,
}

/// Text Document Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

/// Did Change Text Document Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidChangeTextDocumentParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// Versioned Text Document Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

/// Text Document Content Change Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    pub range_length: Option<u32>,
    pub text: String,
}

/// Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Did Save Text Document Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidSaveTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
    pub text: Option<String>,
}

/// Text Document Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

/// Did Close Text Document Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidCloseTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
}

/// Hover Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

/// Completion Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub context: Option<CompletionContext>,
}

/// Completion Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionContext {
    pub trigger_kind: u32,
    pub trigger_character: Option<String>,
}

/// Publish Diagnostics Parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub version: Option<i32>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<u32>,
    pub code: Option<Value>,
    pub code_description: Option<CodeDescription>,
    pub source: Option<String>,
    pub message: String,
    pub tags: Option<Vec<u32>>,
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
    pub data: Option<Value>,
}

/// Code Description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDescription {
    pub href: String,
}

/// Diagnostic Related Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

/// Location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

impl UniversalLSPPlugin {
    /// Create a new Universal LSP plugin
    pub fn new() -> Self {
        let metadata = PluginMetadata {
            id: "universal-lsp".to_string(),
            name: "PhotonDrift LSP Server".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal Language Server Protocol implementation for PhotonDrift".to_string(),
            author: "PhotonDrift Team".to_string(),
            license: "MIT".to_string(),
            homepage: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            repository: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            keywords: vec!["lsp".to_string(), "adr".to_string(), "photondrift".to_string()],
            api_version: crate::plugins::PLUGIN_API_VERSION.to_string(),
            min_adrscan_version: "0.2.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let capabilities = IDECapabilities {
            supports_lsp: true,
            supports_debugging: false,
            supports_extensions: false,
            supports_git_integration: false,
            supports_terminal: false,
            supported_languages: vec![
                "markdown".to_string(),
                "yaml".to_string(),
                "json".to_string(),
            ],
        };
        
        let server_capabilities = LSPServerCapabilities {
            text_document_sync: TextDocumentSyncKind::Incremental,
            hover_provider: true,
            completion_provider: Some(CompletionOptions {
                resolve_provider: true,
                trigger_characters: vec!["#".to_string(), "[".to_string()],
                all_commit_characters: vec![],
            }),
            signature_help_provider: None,
            definition_provider: true,
            type_definition_provider: false,
            implementation_provider: false,
            references_provider: true,
            document_highlight_provider: true,
            document_symbol_provider: true,
            workspace_symbol_provider: true,
            code_action_provider: Some(CodeActionOptions {
                code_action_kinds: vec![
                    "quickfix".to_string(),
                    "refactor".to_string(),
                ],
                resolve_provider: false,
            }),
            code_lens_provider: Some(CodeLensOptions {
                resolve_provider: false,
            }),
            document_formatting_provider: true,
            document_range_formatting_provider: true,
            document_on_type_formatting_provider: None,
            rename_provider: Some(RenameOptions {
                prepare_provider: true,
            }),
            document_link_provider: Some(DocumentLinkOptions {
                resolve_provider: false,
            }),
            color_provider: false,
            folding_range_provider: true,
            diagnostic_provider: Some(DiagnosticOptions {
                identifier: Some("photondrift".to_string()),
                inter_file_dependencies: true,
                workspace_diagnostics: true,
            }),
        };
        
        Self {
            metadata,
            config: None,
            capabilities,
            server_capabilities,
        }
    }
    
    /// Handle LSP message
    pub fn handle_lsp_message(&self, message: &LSPMessage) -> Result<Option<Value>> {
        debug!("Handling LSP message: {:?}", message);
        
        match message {
            LSPMessage::Initialize { params } => {
                self.handle_initialize(params)
            }
            LSPMessage::DidOpen { params } => {
                self.handle_did_open(params)
            }
            LSPMessage::DidChange { params } => {
                self.handle_did_change(params)
            }
            LSPMessage::DidSave { params } => {
                self.handle_did_save(params)
            }
            LSPMessage::DidClose { params } => {
                self.handle_did_close(params)
            }
            LSPMessage::Hover { params } => {
                self.handle_hover(params)
            }
            LSPMessage::Completion { params } => {
                self.handle_completion(params)
            }
            LSPMessage::PublishDiagnostics { params } => {
                self.handle_publish_diagnostics(params)
            }
        }
    }
    
    /// Generate LSP server configuration
    pub fn generate_lsp_config(&self) -> Value {
        json!({
            "name": "photondrift-lsp",
            "command": "adrscan",
            "args": ["lsp"],
            "filetypes": ["markdown"],
            "rootPatterns": [".git", ".adrscan", "docs/adr/", "adr/"],
            "settings": {
                "enableDiagnostics": true,
                "enableCompletion": true,
                "enableHover": true,
                "enableFormatting": true
            }
        })
    }
    
    /// Generate LSP server executable
    pub fn generate_lsp_server_code(&self) -> String {
        r#"#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

// LSP Server for PhotonDrift
class PhotonDriftLSPServer {
    constructor() {
        this.documents = new Map();
        this.capabilities = {
            textDocumentSync: 2, // Incremental
            hoverProvider: true,
            completionProvider: {
                resolveProvider: true,
                triggerCharacters: ['#', '[']
            },
            definitionProvider: true,
            referencesProvider: true,
            documentSymbolProvider: true,
            workspaceSymbolProvider: true,
            codeActionProvider: {
                codeActionKinds: ['quickfix', 'refactor']
            },
            codeLensProvider: {
                resolveProvider: false
            },
            documentFormattingProvider: true,
            documentRangeFormattingProvider: true,
            renameProvider: {
                prepareProvider: true
            },
            documentLinkProvider: {
                resolveProvider: false
            },
            foldingRangeProvider: true,
            diagnosticProvider: {
                identifier: 'photondrift',
                interFileDependencies: true,
                workspaceDiagnostics: true
            }
        };
    }

    start() {
        process.stdin.resume();
        process.stdin.setEncoding('utf8');
        
        let buffer = '';
        process.stdin.on('data', (data) => {
            buffer += data;
            this.processBuffer(buffer);
        });
    }

    processBuffer(buffer) {
        const lines = buffer.split('\n');
        buffer = lines.pop(); // Keep incomplete line in buffer
        
        for (const line of lines) {
            if (line.trim()) {
                this.handleMessage(line);
            }
        }
    }

    handleMessage(line) {
        try {
            const message = JSON.parse(line);
            this.processMessage(message);
        } catch (error) {
            this.sendError('Parse error', error.message);
        }
    }

    async processMessage(message) {
        const { id, method, params } = message;
        
        try {
            let result;
            
            switch (method) {
                case 'initialize':
                    result = await this.initialize(params);
                    break;
                case 'textDocument/didOpen':
                    await this.didOpen(params);
                    return;
                case 'textDocument/didChange':
                    await this.didChange(params);
                    return;
                case 'textDocument/didSave':
                    await this.didSave(params);
                    return;
                case 'textDocument/didClose':
                    await this.didClose(params);
                    return;
                case 'textDocument/hover':
                    result = await this.hover(params);
                    break;
                case 'textDocument/completion':
                    result = await this.completion(params);
                    break;
                case 'textDocument/definition':
                    result = await this.definition(params);
                    break;
                case 'textDocument/references':
                    result = await this.references(params);
                    break;
                case 'textDocument/documentSymbol':
                    result = await this.documentSymbol(params);
                    break;
                case 'workspace/symbol':
                    result = await this.workspaceSymbol(params);
                    break;
                case 'textDocument/codeAction':
                    result = await this.codeAction(params);
                    break;
                case 'textDocument/codeLens':
                    result = await this.codeLens(params);
                    break;
                case 'textDocument/formatting':
                    result = await this.formatting(params);
                    break;
                case 'textDocument/rangeFormatting':
                    result = await this.rangeFormatting(params);
                    break;
                default:
                    this.sendError('Method not found', `Unknown method: ${method}`);
                    return;
            }
            
            if (id !== undefined) {
                this.sendResponse(id, result);
            }
        } catch (error) {
            if (id !== undefined) {
                this.sendError('Internal error', error.message, id);
            }
        }
    }

    async initialize(params) {
        return {
            capabilities: this.capabilities,
            serverInfo: {
                name: 'photondrift-lsp',
                version: '1.0.0'
            }
        };
    }

    async didOpen(params) {
        const { textDocument } = params;
        this.documents.set(textDocument.uri, textDocument);
        await this.publishDiagnostics(textDocument.uri);
    }

    async didChange(params) {
        const { textDocument, contentChanges } = params;
        const document = this.documents.get(textDocument.uri);
        
        if (document) {
            // Apply changes
            for (const change of contentChanges) {
                if (change.range) {
                    // Incremental change
                    // TODO: Apply incremental changes
                } else {
                    // Full document change
                    document.text = change.text;
                }
            }
            document.version = textDocument.version;
        }
    }

    async didSave(params) {
        const { textDocument } = params;
        await this.publishDiagnostics(textDocument.uri);
    }

    async didClose(params) {
        const { textDocument } = params;
        this.documents.delete(textDocument.uri);
    }

    async hover(params) {
        const { textDocument, position } = params;
        const document = this.documents.get(textDocument.uri);
        
        if (!document) return null;
        
        // Get word at position
        const lines = document.text.split('\n');
        const line = lines[position.line];
        const word = this.getWordAtPosition(line, position.character);
        
        if (word && word.match(/ADR-?\d+/i)) {
            return {
                contents: {
                    kind: 'markdown',
                    value: `**${word}**: Architecture Decision Record reference`
                }
            };
        }
        
        return null;
    }

    async completion(params) {
        const { textDocument, position } = params;
        const document = this.documents.get(textDocument.uri);
        
        if (!document) return null;
        
        const items = [
            {
                label: 'ADR Template',
                kind: 14, // Snippet
                insertText: '# ADR-${1:number}: ${2:title}\n\n## Status\n\n${3:Proposed}\n\n## Context\n\n${4:Context}\n\n## Decision\n\n${5:Decision}\n\n## Consequences\n\n${6:Consequences}',
                insertTextFormat: 2 // Snippet format
            },
            {
                label: 'Status: Proposed',
                kind: 12, // Value
                insertText: 'Proposed'
            },
            {
                label: 'Status: Accepted',
                kind: 12, // Value
                insertText: 'Accepted'
            },
            {
                label: 'Status: Deprecated',
                kind: 12, // Value
                insertText: 'Deprecated'
            }
        ];
        
        return { items };
    }

    async publishDiagnostics(uri) {
        try {
            const result = await this.runADRScan(uri);
            const diagnostics = this.parseDiagnostics(result);
            
            this.sendNotification('textDocument/publishDiagnostics', {
                uri,
                diagnostics
            });
        } catch (error) {
            console.error('Failed to run analysis:', error);
        }
    }

    async runADRScan(uri) {
        return new Promise((resolve, reject) => {
            const filePath = uri.replace('file://', '');
            const child = spawn('adrscan', ['analyze', filePath, '--format', 'json']);
            
            let stdout = '';
            let stderr = '';
            
            child.stdout.on('data', (data) => {
                stdout += data;
            });
            
            child.stderr.on('data', (data) => {
                stderr += data;
            });
            
            child.on('close', (code) => {
                if (code === 0) {
                    try {
                        resolve(JSON.parse(stdout));
                    } catch (error) {
                        reject(new Error('Failed to parse analysis results'));
                    }
                } else {
                    reject(new Error(`adrscan failed: ${stderr}`));
                }
            });
        });
    }

    parseDiagnostics(analysisResult) {
        const diagnostics = [];
        
        if (analysisResult.issues) {
            for (const issue of analysisResult.issues) {
                const severity = issue.severity === 'error' ? 1 
                              : issue.severity === 'warning' ? 2 
                              : 3; // information
                
                diagnostics.push({
                    range: {
                        start: { line: (issue.line || 1) - 1, character: issue.column || 0 },
                        end: { line: (issue.line || 1) - 1, character: (issue.column || 0) + (issue.length || 1) }
                    },
                    severity,
                    source: 'PhotonDrift',
                    message: issue.message,
                    code: issue.code
                });
            }
        }
        
        return diagnostics;
    }

    getWordAtPosition(line, character) {
        const beforeCursor = line.substring(0, character);
        const afterCursor = line.substring(character);
        
        const wordBefore = beforeCursor.match(/[\w-]+$/);
        const wordAfter = afterCursor.match(/^[\w-]+/);
        
        const before = wordBefore ? wordBefore[0] : '';
        const after = wordAfter ? wordAfter[0] : '';
        
        return before + after;
    }

    sendResponse(id, result) {
        const response = { jsonrpc: '2.0', id, result };
        this.send(response);
    }

    sendError(message, data, id = null) {
        const error = { 
            jsonrpc: '2.0', 
            error: { code: -32603, message, data }
        };
        if (id !== null) error.id = id;
        this.send(error);
    }

    sendNotification(method, params) {
        const notification = { jsonrpc: '2.0', method, params };
        this.send(notification);
    }

    send(message) {
        const content = JSON.stringify(message);
        process.stdout.write(`Content-Length: ${Buffer.byteLength(content)}\r\n\r\n${content}`);
    }
}

// Start the LSP server
const server = new PhotonDriftLSPServer();
server.start();
"#.to_string()
    }
    
    // Private handler methods
    
    fn handle_initialize(&self, _params: &InitializeParams) -> Result<Option<Value>> {
        Ok(Some(json!({
            "capabilities": self.server_capabilities,
            "serverInfo": {
                "name": "photondrift-lsp",
                "version": "1.0.0"
            }
        })))
    }
    
    fn handle_did_open(&self, params: &DidOpenTextDocumentParams) -> Result<Option<Value>> {
        debug!("Document opened: {}", params.text_document.uri);
        // In a real implementation, this would store the document and trigger analysis
        Ok(None)
    }
    
    fn handle_did_change(&self, params: &DidChangeTextDocumentParams) -> Result<Option<Value>> {
        debug!("Document changed: {}", params.text_document.uri);
        // In a real implementation, this would update the document and trigger incremental analysis
        Ok(None)
    }
    
    fn handle_did_save(&self, params: &DidSaveTextDocumentParams) -> Result<Option<Value>> {
        debug!("Document saved: {}", params.text_document.uri);
        // In a real implementation, this would trigger full analysis and publish diagnostics
        Ok(None)
    }
    
    fn handle_did_close(&self, params: &DidCloseTextDocumentParams) -> Result<Option<Value>> {
        debug!("Document closed: {}", params.text_document.uri);
        // In a real implementation, this would clean up document state
        Ok(None)
    }
    
    fn handle_hover(&self, params: &HoverParams) -> Result<Option<Value>> {
        debug!("Hover request at {}:{}", params.position.line, params.position.character);
        // In a real implementation, this would provide hover information
        Ok(Some(json!({
            "contents": {
                "kind": "markdown",
                "value": "PhotonDrift ADR Analysis"
            }
        })))
    }
    
    fn handle_completion(&self, params: &CompletionParams) -> Result<Option<Value>> {
        debug!("Completion request at {}:{}", params.position.line, params.position.character);
        
        let items = vec![
            json!({
                "label": "ADR Template",
                "kind": 14, // Snippet
                "insertText": "# ADR-${1:number}: ${2:title}\n\n## Status\n\n${3:Proposed}\n\n## Context\n\n${4:Context}\n\n## Decision\n\n${5:Decision}\n\n## Consequences\n\n${6:Consequences}",
                "insertTextFormat": 2 // Snippet format
            }),
            json!({
                "label": "Status: Proposed",
                "kind": 12, // Value
                "insertText": "Proposed"
            }),
            json!({
                "label": "Status: Accepted", 
                "kind": 12, // Value
                "insertText": "Accepted"
            })
        ];
        
        Ok(Some(json!({ "items": items })))
    }
    
    fn handle_publish_diagnostics(&self, params: &PublishDiagnosticsParams) -> Result<Option<Value>> {
        debug!("Publishing diagnostics for {}", params.uri);
        // In a real implementation, this would be triggered by analysis results
        Ok(None)
    }
}

impl Plugin for UniversalLSPPlugin {
    fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
        info!("Initializing Universal LSP plugin");
        
        self.config = Some(crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::Universal));
        
        debug!("Universal LSP plugin initialized successfully");
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
            PluginCapability::NetworkAccess,
        ]
    }
    
    fn execute(&self, command: &str, params: &HashMap<String, String>) -> Result<PluginResponse> {
        debug!("Executing Universal LSP command: {}", command);
        
        match command {
            "generate_lsp_server" => {
                let server_code = self.generate_lsp_server_code();
                let config = self.generate_lsp_config();
                
                Ok(PluginResponse {
                    success: true,
                    data: Some(json!({
                        "server_code": server_code,
                        "config": config,
                        "capabilities": self.server_capabilities
                    })),
                    message: Some("LSP server files generated successfully".to_string()),
                    warnings: vec![],
                    errors: vec![],
                })
            }
            "handle_lsp" => {
                let message_str = params.get("message").unwrap_or("{}");
                match serde_json::from_str::<LSPMessage>(message_str) {
                    Ok(message) => {
                        match self.handle_lsp_message(&message) {
                            Ok(response) => Ok(PluginResponse {
                                success: true,
                                data: response,
                                message: Some("LSP message handled successfully".to_string()),
                                warnings: vec![],
                                errors: vec![],
                            }),
                            Err(e) => Ok(PluginResponse {
                                success: false,
                                data: None,
                                message: Some("LSP message handling failed".to_string()),
                                warnings: vec![],
                                errors: vec![e.to_string()],
                            })
                        }
                    }
                    Err(e) => Ok(PluginResponse {
                        success: false,
                        data: None,
                        message: Some("Invalid LSP message format".to_string()),
                        warnings: vec![],
                        errors: vec![e.to_string()],
                    })
                }
            }
            _ => Ok(PluginResponse {
                success: false,
                data: None,
                message: Some(format!("Unknown command: {}", command)),
                warnings: vec![],
                errors: vec![format!("Command '{}' not supported by Universal LSP plugin", command)],
            })
        }
    }
    
    fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down Universal LSP plugin");
        Ok(())
    }
}

impl IDEIntegrationPlugin for UniversalLSPPlugin {
    fn ide_type(&self) -> IDEType {
        IDEType::Universal
    }
    
    fn setup_ide_integration(&self, config: &IDEConfig) -> Result<()> {
        info!("Setting up Universal LSP integration with config: {:?}", config);
        Ok(())
    }
    
    fn handle_ide_event(&self, event: &IDEEvent) -> Result<IDEResponse> {
        debug!("Handling Universal LSP event: {:?}", event);
        
        match event {
            IDEEvent::FileOpened { path } => {
                Ok(IDEResponse {
                    handled: true,
                    actions: vec![],
                    diagnostics: vec![],
                })
            }
            _ => Ok(IDEResponse {
                handled: false,
                actions: vec![],
                diagnostics: vec![],
            })
        }
    }
    
    fn get_ide_config(&self) -> IDEConfig {
        self.config.clone().unwrap_or_else(|| {
            crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::Universal)
        })
    }
    
    fn register_commands(&self) -> Vec<IDECommand> {
        vec![
            IDECommand {
                id: "photondrift.analyzeFile".to_string(),
                title: "Analyze ADR File".to_string(),
                category: "PhotonDrift".to_string(),
                description: Some("Analyze the current ADR file via LSP".to_string()),
                arguments: vec![],
            },
        ]
    }
}

impl CommonIDEFeatures for UniversalLSPPlugin {
    fn show_notification(&self, message: &str, level: MessageLevel) -> Result<()> {
        debug!("Universal LSP notification: {} (level: {:?})", message, level);
        Ok(())
    }
    
    fn open_file(&self, path: &Path, line: Option<u32>) -> Result<()> {
        debug!("Universal LSP open file: {} at line {:?}", path.display(), line);
        Ok(())
    }
    
    fn insert_text(&self, path: &Path, line: u32, column: u32, text: &str) -> Result<()> {
        debug!("Universal LSP insert text at {}:{}:{}: {}", path.display(), line, column, text);
        Ok(())
    }
    
    fn get_selection(&self) -> Result<Option<TextSelection>> {
        Ok(None)
    }
    
    fn set_status_message(&self, message: &str) -> Result<()> {
        debug!("Universal LSP status message: {}", message);
        Ok(())
    }
}

impl Default for UniversalLSPPlugin {
    fn default() -> Self {
        Self::new()
    }
}