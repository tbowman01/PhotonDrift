"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const node_1 = require("vscode-languageclient/node");
const adrExplorer_1 = require("./adrExplorer");
const driftDetection_1 = require("./driftDetection");
const commands_1 = require("./commands");
const statusBar_1 = require("./statusBar");
const configuration_1 = require("./configuration");
let client;
let adrExplorerProvider;
let driftDetectionProvider;
let statusBarManager;
function activate(context) {
    console.log('PhotonDrift ADR Manager is now active!');
    // Initialize configuration
    const config = new configuration_1.ConfigurationManager();
    // Initialize status bar
    statusBarManager = new statusBar_1.StatusBarManager(context);
    statusBarManager.show();
    // Initialize tree view providers
    adrExplorerProvider = new adrExplorer_1.AdrExplorerProvider(context);
    driftDetectionProvider = new driftDetection_1.DriftDetectionProvider(context);
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
    const commandProvider = new commands_1.AdrCommandProvider(context, adrExplorerProvider, driftDetectionProvider);
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
                }
                else if (!config.isLspEnabled() && client) {
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
exports.activate = activate;
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
exports.deactivate = deactivate;
async function startLanguageServer(context, config) {
    try {
        const serverPath = config.getLspServerPath();
        // Check if server executable exists
        if (!await isExecutableAvailable(serverPath)) {
            vscode.window.showErrorMessage(`PhotonDrift LSP server not found at: ${serverPath}. Please install PhotonDrift or update the server path in settings.`, 'Open Settings').then(selection => {
                if (selection === 'Open Settings') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'photondrift.lsp.serverPath');
                }
            });
            return;
        }
        // Server options
        const serverOptions = {
            run: { command: serverPath, transport: node_1.TransportKind.stdio },
            debug: {
                command: serverPath,
                transport: node_1.TransportKind.stdio,
                options: { env: { ...process.env, RUST_LOG: 'debug' } }
            }
        };
        // Client options
        const clientOptions = {
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
        client = new node_1.LanguageClient('photondrift-lsp', 'PhotonDrift Language Server', serverOptions, clientOptions);
        await client.start();
        vscode.window.showInformationMessage('PhotonDrift Language Server started successfully!');
        // Update status bar
        statusBarManager.setLspStatus('running');
    }
    catch (error) {
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
async function isExecutableAvailable(command) {
    try {
        const { exec } = require('child_process');
        return new Promise((resolve) => {
            exec(`${command} --help`, (error) => {
                resolve(!error);
            });
        });
    }
    catch {
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
        }
        catch {
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
                }
                catch {
                    // File doesn't exist
                }
            }
        }
        catch {
            // Error checking config
        }
    }
    vscode.commands.executeCommand('setContext', 'photondrift:hasAdrDirectory', hasAdrDirectory);
    vscode.commands.executeCommand('setContext', 'photondrift:driftEnabled', hasDriftConfig);
}
//# sourceMappingURL=extension.js.map