import * as vscode from 'vscode';
import * as path from 'path';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';
import { AdrExplorerProvider } from './adrExplorer';
import { DriftDetectionProvider } from './driftDetection';
import { AdrCommandProvider } from './commands';
import { StatusBarManager } from './statusBar';
import { ConfigurationManager } from './configuration';

let client: LanguageClient;
let adrExplorerProvider: AdrExplorerProvider;
let driftDetectionProvider: DriftDetectionProvider;
let statusBarManager: StatusBarManager;

export function activate(context: vscode.ExtensionContext) {
    console.log('PhotonDrift ADR Manager is now active!');

    // Initialize configuration
    const config = new ConfigurationManager();
    
    // Initialize status bar
    statusBarManager = new StatusBarManager(context);
    statusBarManager.show();
    
    // Initialize tree view providers
    adrExplorerProvider = new AdrExplorerProvider(context);
    driftDetectionProvider = new DriftDetectionProvider(context);
    
    // Register tree views
    const adrTreeView = vscode.window.createTreeView('photondrift.adrExplorer', {
        treeDataProvider: adrExplorerProvider,
        showCollapseAll: true
    });
    
    const driftTreeView = vscode.window.createTreeView('photondrift.driftDetection', {
        treeDataProvider: driftDetectionProvider,
        showCollapseAll: true
    });
    
    // Initialize command provider
    const commandProvider = new AdrCommandProvider(context, adrExplorerProvider, driftDetectionProvider);
    
    // Register all commands
    const commands = [
        vscode.commands.registerCommand('photondrift.init', () => commandProvider.initAdrStructure()),
        vscode.commands.registerCommand('photondrift.inventory', () => commandProvider.runInventory()),
        vscode.commands.registerCommand('photondrift.diff', () => commandProvider.runDriftDetection()),
        vscode.commands.registerCommand('photondrift.propose', () => commandProvider.generateProposals()),
        vscode.commands.registerCommand('photondrift.index', () => commandProvider.generateIndex()),
        vscode.commands.registerCommand('photondrift.newAdr', () => commandProvider.createNewAdr()),
        vscode.commands.registerCommand('photondrift.toggleLsp', () => commandProvider.toggleLspServer()),
        vscode.commands.registerCommand('photondrift.openDashboard', () => commandProvider.openDashboard()),
        vscode.commands.registerCommand('photondrift.refreshAdr', () => adrExplorerProvider.refresh()),
        vscode.commands.registerCommand('photondrift.refreshDrift', () => driftDetectionProvider.refresh()),
        vscode.commands.registerCommand('photondrift.openAdr', (adr) => commandProvider.openAdr(adr)),
        vscode.commands.registerCommand('photondrift.editAdr', (adr) => commandProvider.editAdr(adr)),
        vscode.commands.registerCommand('photondrift.deleteAdr', (adr) => commandProvider.deleteAdr(adr))
    ];
    
    // Add all commands to context
    context.subscriptions.push(...commands, adrTreeView, driftTreeView);
    
    // Set up context for views
    updateContext();
    
    // Watch for workspace changes
    const watcher = vscode.workspace.createFileSystemWatcher('**/.adrscan.{yml,yaml}');
    watcher.onDidCreate(() => updateContext());
    watcher.onDidDelete(() => updateContext());
    watcher.onDidChange(() => updateContext());
    context.subscriptions.push(watcher);
    
    // Watch for ADR file changes
    const adrWatcher = vscode.workspace.createFileSystemWatcher('**/docs/adr/**/*.md');
    adrWatcher.onDidCreate(() => adrExplorerProvider.refresh());
    adrWatcher.onDidDelete(() => adrExplorerProvider.refresh());
    adrWatcher.onDidChange(() => {
        adrExplorerProvider.refresh();
        if (config.isDriftEnabled()) {
            driftDetectionProvider.refresh();
        }
    });
    context.subscriptions.push(adrWatcher);
    
    // Initialize Language Server if enabled
    if (config.isLspEnabled()) {
        startLanguageServer(context, config);
    }
    
    // Listen for configuration changes
    vscode.workspace.onDidChangeConfiguration(event => {
        if (event.affectsConfiguration('photondrift')) {
            config.reload();
            statusBarManager.update();
            
            if (event.affectsConfiguration('photondrift.lsp.enabled')) {
                if (config.isLspEnabled() && !client) {
                    startLanguageServer(context, config);
                } else if (!config.isLspEnabled() && client) {
                    stopLanguageServer();
                }
            }
        }
    });
    
    // Update status bar periodically
    const statusInterval = setInterval(() => {
        statusBarManager.update();
    }, 30000); // Update every 30 seconds
    
    context.subscriptions.push({
        dispose: () => clearInterval(statusInterval)
    });
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function startLanguageServer(context: vscode.ExtensionContext, config: ConfigurationManager) {
    try {
        const serverPath = config.getLspServerPath();
        
        // Check if server executable exists
        if (!await isExecutableAvailable(serverPath)) {
            vscode.window.showErrorMessage(
                `PhotonDrift LSP server not found at: ${serverPath}. Please install PhotonDrift or update the server path in settings.`,
                'Open Settings'
            ).then(selection => {
                if (selection === 'Open Settings') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'photondrift.lsp.serverPath');
                }
            });
            return;
        }
        
        // Server options
        const serverOptions: ServerOptions = {
            run: { command: serverPath, transport: TransportKind.stdio },
            debug: { 
                command: serverPath, 
                transport: TransportKind.stdio,
                options: { env: { ...process.env, RUST_LOG: 'debug' } }
            }
        };
        
        // Client options
        const clientOptions: LanguageClientOptions = {
            documentSelector: [
                { scheme: 'file', language: 'markdown' },
                { scheme: 'file', pattern: '**/adr/**/*.md' },
                { scheme: 'file', pattern: '**/*adr*.md' }
            ],
            synchronize: {
                fileEvents: [
                    vscode.workspace.createFileSystemWatcher('**/.adrscan.{yml,yaml}'),
                    vscode.workspace.createFileSystemWatcher('**/adr/**/*.md'),
                    vscode.workspace.createFileSystemWatcher('**/*adr*.md')
                ]
            },
            initializationOptions: {
                adrDirectory: config.getAdrDirectory(),
                maxDiagnostics: config.getMaxDiagnostics(),
                mlEnabled: config.isMlEnabled(),
                mlModel: config.getMlModel()
            }
        };
        
        // Create and start the client
        client = new LanguageClient(
            'photondrift-lsp',
            'PhotonDrift Language Server',
            serverOptions,
            clientOptions
        );
        
        await client.start();
        
        vscode.window.showInformationMessage('PhotonDrift Language Server started successfully!');
        
        // Update status bar
        statusBarManager.setLspStatus('running');
        
    } catch (error) {
        const message = `Failed to start PhotonDrift Language Server: ${error}`;
        console.error(message);
        vscode.window.showErrorMessage(message);
        statusBarManager.setLspStatus('error');
    }
}

async function stopLanguageServer() {
    if (client) {
        await client.stop();
        client = undefined;
        statusBarManager.setLspStatus('stopped');
        vscode.window.showInformationMessage('PhotonDrift Language Server stopped.');
    }
}

async function isExecutableAvailable(command: string): Promise<boolean> {
    try {
        const { exec } = require('child_process');
        return new Promise((resolve) => {
            exec(`${command} --help`, (error: any) => {
                resolve(!error);
            });
        });
    } catch {
        return false;
    }
}

async function updateContext() {
    // Check if workspace has ADR directory or config
    const workspaces = vscode.workspace.workspaceFolders;
    if (!workspaces) {
        vscode.commands.executeCommand('setContext', 'photondrift:hasAdrDirectory', false);
        vscode.commands.executeCommand('setContext', 'photondrift:driftEnabled', false);
        return;
    }
    
    let hasAdrDirectory = false;
    let hasDriftConfig = false;
    
    for (const workspace of workspaces) {
        // Check for ADR directory
        try {
            const adrPath = path.join(workspace.uri.fsPath, 'docs', 'adr');
            const stat = await vscode.workspace.fs.stat(vscode.Uri.file(adrPath));
            if (stat.type === vscode.FileType.Directory) {
                hasAdrDirectory = true;
            }
        } catch {
            // Directory doesn't exist
        }
        
        // Check for configuration files
        try {
            const configPaths = [
                path.join(workspace.uri.fsPath, '.adrscan.yml'),
                path.join(workspace.uri.fsPath, '.adrscan.yaml')
            ];
            
            for (const configPath of configPaths) {
                try {
                    await vscode.workspace.fs.stat(vscode.Uri.file(configPath));
                    hasDriftConfig = true;
                    break;
                } catch {
                    // File doesn't exist
                }
            }
        } catch {
            // Error checking config
        }
    }
    
    vscode.commands.executeCommand('setContext', 'photondrift:hasAdrDirectory', hasAdrDirectory);
    vscode.commands.executeCommand('setContext', 'photondrift:driftEnabled', hasDriftConfig);
}