import * as vscode from 'vscode';
import { PhotonDriftCLI } from './photonDriftCLI';
import { ADRTreeProvider } from './adrTreeProvider';
import { DriftStatusBar } from './driftStatusBar';
import { ADRDiagnosticProvider } from './diagnosticProvider';
import { ADRCompletionProvider } from './completionProvider';

let cli: PhotonDriftCLI;
let adrTreeProvider: ADRTreeProvider;
let statusBar: DriftStatusBar;
let diagnosticProvider: ADRDiagnosticProvider;

export function activate(context: vscode.ExtensionContext) {
    console.log('PhotonDrift ADR Manager is now active!');

    // Initialize CLI wrapper
    cli = new PhotonDriftCLI();

    // Initialize tree provider for ADR files
    adrTreeProvider = new ADRTreeProvider(context);
    vscode.window.registerTreeDataProvider('photondriftAdrTree', adrTreeProvider);

    // Initialize status bar
    statusBar = new DriftStatusBar();
    context.subscriptions.push(statusBar);

    // Initialize diagnostic provider for inline warnings
    diagnosticProvider = new ADRDiagnosticProvider();
    context.subscriptions.push(diagnosticProvider);

    // Register completion provider for ADR templates
    const completionProvider = new ADRCompletionProvider();
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider(
            { language: 'markdown' },
            completionProvider,
            '#' // Trigger on # for ADR headers
        )
    );

    // Register commands
    registerCommands(context);

    // Watch for ADR file changes
    setupFileWatching(context);

    // Update context for when ADR files are present
    updateAdrContext();
}

function registerCommands(context: vscode.ExtensionContext) {
    // Initialize ADR structure
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.init', async () => {
            try {
                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: 'Initializing ADR structure...',
                    cancellable: false
                }, async () => {
                    await cli.init(workspaceFolder.uri.fsPath);
                });

                vscode.window.showInformationMessage('ADR structure initialized successfully');
                adrTreeProvider.refresh();
                updateAdrContext();
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to initialize ADR structure: ${error}`);
            }
        })
    );

    // Create new ADR
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.createAdr', async (uri?: vscode.Uri) => {
            try {
                const title = await vscode.window.showInputBox({
                    prompt: 'Enter ADR title',
                    placeHolder: 'e.g., Use React for frontend framework'
                });

                if (!title) {
                    return;
                }

                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                const adrPath = await cli.createAdr(workspaceFolder.uri.fsPath, title);
                
                // Open the newly created ADR
                const document = await vscode.workspace.openTextDocument(adrPath);
                await vscode.window.showTextDocument(document);

                adrTreeProvider.refresh();
                vscode.window.showInformationMessage(`ADR created: ${adrPath}`);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to create ADR: ${error}`);
            }
        })
    );

    // Run drift detection
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.runDriftDetection', async () => {
            try {
                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                statusBar.setScanning(true);

                const results = await vscode.window.withProgress({
                    location: vscode.ProgressLocation.Notification,
                    title: 'Running drift detection...',
                    cancellable: true
                }, async (progress, token) => {
                    return await cli.scanForDrift(workspaceFolder.uri.fsPath, token);
                });

                statusBar.setScanning(false);
                statusBar.updateDriftCount(results.length);

                if (results.length > 0) {
                    const action = await vscode.window.showInformationMessage(
                        `Found ${results.length} drift item(s)`,
                        'View Results',
                        'Generate Report'
                    );

                    if (action === 'View Results') {
                        await showDriftResults(results);
                    } else if (action === 'Generate Report') {
                        await generateDriftReport(results);
                    }
                } else {
                    vscode.window.showInformationMessage('No drift detected');
                }

                // Update diagnostics
                await diagnosticProvider.updateDiagnostics(results);
            } catch (error) {
                statusBar.setScanning(false);
                vscode.window.showErrorMessage(`Drift detection failed: ${error}`);
            }
        })
    );

    // Generate ADR index
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.generateIndex', async () => {
            try {
                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                const indexPath = await cli.generateIndex(workspaceFolder.uri.fsPath);
                
                // Open the generated index
                const document = await vscode.workspace.openTextDocument(indexPath);
                await vscode.window.showTextDocument(document);

                vscode.window.showInformationMessage(`ADR index generated: ${indexPath}`);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to generate index: ${error}`);
            }
        })
    );

    // Show ADR inventory
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.showInventory', async () => {
            try {
                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                const inventory = await cli.getInventory(workspaceFolder.uri.fsPath);
                await showInventoryWebview(inventory);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to show inventory: ${error}`);
            }
        })
    );

    // Propose ADR for selected text/changes
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.proposeAdr', async () => {
            try {
                const editor = vscode.window.activeTextEditor;
                if (!editor) {
                    vscode.window.showErrorMessage('No active editor');
                    return;
                }

                const selection = editor.selection;
                const selectedText = editor.document.getText(selection);

                if (!selectedText.trim()) {
                    vscode.window.showErrorMessage('Please select some text to analyze');
                    return;
                }

                const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                if (!workspaceFolder) {
                    vscode.window.showErrorMessage('No workspace folder open');
                    return;
                }

                const proposal = await cli.proposeAdr(workspaceFolder.uri.fsPath, selectedText);
                await showProposalWebview(proposal);
            } catch (error) {
                vscode.window.showErrorMessage(`Failed to generate proposal: ${error}`);
            }
        })
    );

    // Open settings
    context.subscriptions.push(
        vscode.commands.registerCommand('photondrift.openSettings', () => {
            vscode.commands.executeCommand('workbench.action.openSettings', 'photondrift');
        })
    );
}

function setupFileWatching(context: vscode.ExtensionContext) {
    // Watch for ADR file changes
    const adrWatcher = vscode.workspace.createFileSystemWatcher('**/*.adr.md');
    
    adrWatcher.onDidCreate(() => {
        adrTreeProvider.refresh();
        updateAdrContext();
    });

    adrWatcher.onDidDelete(() => {
        adrTreeProvider.refresh();
        updateAdrContext();
    });

    adrWatcher.onDidChange(async (uri) => {
        const config = vscode.workspace.getConfiguration('photondrift');
        if (config.get('autoDetectDrift', true)) {
            // Run drift detection on the changed file
            setTimeout(async () => {
                try {
                    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
                    if (workspaceFolder) {
                        const results = await cli.scanSingleFile(uri.fsPath);
                        await diagnosticProvider.updateDiagnostics(results);
                    }
                } catch (error) {
                    console.error('Auto drift detection failed:', error);
                }
            }, 1000); // Debounce 1 second
        }
    });

    context.subscriptions.push(adrWatcher);

    // Watch for general file changes that might affect ADRs
    const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*');
    
    fileWatcher.onDidChange(async () => {
        const config = vscode.workspace.getConfiguration('photondrift');
        if (config.get('autoDetectDrift', true)) {
            // Debounced drift detection
            setTimeout(() => {
                vscode.commands.executeCommand('photondrift.runDriftDetection');
            }, 5000); // 5 second debounce for general changes
        }
    });

    context.subscriptions.push(fileWatcher);
}

async function updateAdrContext() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        return;
    }

    try {
        const adrFiles = await vscode.workspace.findFiles('**/*.adr.md');
        await vscode.commands.executeCommand('setContext', 'workspaceHasAdrFiles', adrFiles.length > 0);
    } catch (error) {
        console.error('Failed to update ADR context:', error);
    }
}

async function showDriftResults(results: any[]) {
    // Create a new document with drift results
    const content = formatDriftResults(results);
    const document = await vscode.workspace.openTextDocument({
        content,
        language: 'markdown'
    });
    await vscode.window.showTextDocument(document);
}

async function generateDriftReport(results: any[]) {
    try {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            return;
        }

        const reportPath = await cli.generateReport(workspaceFolder.uri.fsPath, results);
        
        // Open the generated report
        const document = await vscode.workspace.openTextDocument(reportPath);
        await vscode.window.showTextDocument(document);

        vscode.window.showInformationMessage(`Drift report generated: ${reportPath}`);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to generate report: ${error}`);
    }
}

async function showInventoryWebview(inventory: any) {
    const panel = vscode.window.createWebviewPanel(
        'photondriftInventory',
        'ADR Inventory',
        vscode.ViewColumn.Two,
        {
            enableScripts: true,
            retainContextWhenHidden: true
        }
    );

    panel.webview.html = generateInventoryHTML(inventory);
}

async function showProposalWebview(proposal: any) {
    const panel = vscode.window.createWebviewPanel(
        'photondriftProposal',
        'ADR Proposal',
        vscode.ViewColumn.Two,
        {
            enableScripts: true,
            retainContextWhenHidden: true
        }
    );

    panel.webview.html = generateProposalHTML(proposal);
}

function formatDriftResults(results: any[]): string {
    let content = '# Drift Detection Results\n\n';
    content += `Found ${results.length} drift item(s)\n\n`;

    results.forEach((result, index) => {
        content += `## ${index + 1}. ${result.title || 'Unnamed Drift'}\n`;
        content += `**Severity:** ${result.severity}\n`;
        content += `**Category:** ${result.category}\n`;
        content += `**File:** ${result.location?.file || 'Unknown'}\n`;
        if (result.location?.line) {
            content += `**Line:** ${result.location.line}\n`;
        }
        content += `**Description:** ${result.description}\n\n`;
        if (result.suggestion) {
            content += `**Suggestion:** ${result.suggestion}\n\n`;
        }
        content += '---\n\n';
    });

    return content;
}

function generateInventoryHTML(inventory: any): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>ADR Inventory</title>
        <style>
            body { font-family: Arial, sans-serif; padding: 20px; }
            .adr-item { border: 1px solid #ccc; margin: 10px 0; padding: 15px; border-radius: 5px; }
            .adr-title { font-size: 18px; font-weight: bold; color: #0066cc; }
            .adr-status { display: inline-block; padding: 3px 8px; border-radius: 3px; font-size: 12px; }
            .status-accepted { background-color: #d4edda; color: #155724; }
            .status-proposed { background-color: #fff3cd; color: #856404; }
            .status-deprecated { background-color: #f8d7da; color: #721c24; }
        </style>
    </head>
    <body>
        <h1>ADR Inventory</h1>
        <p>Total ADRs: ${inventory.length || 0}</p>
        ${inventory.map((adr: any) => `
            <div class="adr-item">
                <div class="adr-title">${adr.title || 'Untitled'}</div>
                <div class="adr-status status-${adr.status?.toLowerCase() || 'unknown'}">${adr.status || 'Unknown'}</div>
                <p><strong>Date:</strong> ${adr.date || 'Unknown'}</p>
                <p><strong>File:</strong> ${adr.file || 'Unknown'}</p>
                ${adr.summary ? `<p><strong>Summary:</strong> ${adr.summary}</p>` : ''}
            </div>
        `).join('')}
    </body>
    </html>`;
}

function generateProposalHTML(proposal: any): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>ADR Proposal</title>
        <style>
            body { font-family: Arial, sans-serif; padding: 20px; }
            .proposal-section { margin: 20px 0; }
            .section-title { font-size: 18px; font-weight: bold; color: #0066cc; margin-bottom: 10px; }
            .code-block { background-color: #f5f5f5; padding: 10px; border-radius: 5px; font-family: monospace; }
            .confidence { display: inline-block; padding: 3px 8px; border-radius: 3px; font-size: 12px; }
            .high-confidence { background-color: #d4edda; color: #155724; }
            .medium-confidence { background-color: #fff3cd; color: #856404; }
            .low-confidence { background-color: #f8d7da; color: #721c24; }
        </style>
    </head>
    <body>
        <h1>ADR Proposal</h1>
        
        <div class="proposal-section">
            <div class="section-title">Suggested Title</div>
            <p>${proposal.title || 'Generated ADR Proposal'}</p>
        </div>

        <div class="proposal-section">
            <div class="section-title">Context</div>
            <p>${proposal.context || 'Context analysis based on selected code/text'}</p>
        </div>

        <div class="proposal-section">
            <div class="section-title">Decision</div>
            <p>${proposal.decision || 'Proposed decision based on analysis'}</p>
        </div>

        <div class="proposal-section">
            <div class="section-title">Consequences</div>
            <p>${proposal.consequences || 'Potential consequences of this decision'}</p>
        </div>

        ${proposal.confidence ? `
        <div class="proposal-section">
            <div class="section-title">Confidence</div>
            <span class="confidence ${proposal.confidence.toLowerCase()}-confidence">${proposal.confidence}</span>
        </div>` : ''}

        <div class="proposal-section">
            <button onclick="copyToClipboard()">Copy ADR Template</button>
        </div>

        <script>
            function copyToClipboard() {
                const template = \`# ${proposal.title || 'ADR Title'}

## Status
Proposed

## Context
${proposal.context || 'Context description'}

## Decision
${proposal.decision || 'Decision description'}

## Consequences
${proposal.consequences || 'Consequences description'}\`;
                
                navigator.clipboard.writeText(template).then(() => {
                    alert('ADR template copied to clipboard!');
                });
            }
        </script>
    </body>
    </html>`;
}

export function deactivate() {
    if (statusBar) {
        statusBar.dispose();
    }
    console.log('PhotonDrift ADR Manager deactivated');
}