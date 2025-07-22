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
exports.AdrCommandProvider = void 0;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const child_process_1 = require("child_process");
const util_1 = require("util");
const execAsync = (0, util_1.promisify)(child_process_1.exec);
class AdrCommandProvider {
    constructor(context, adrExplorer, driftDetection) {
        this.context = context;
        this.adrExplorer = adrExplorer;
        this.driftDetection = driftDetection;
    }
    async initAdrStructure() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }
        try {
            vscode.window.showInformationMessage('Initializing ADR structure...');
            const adrscanPath = this.getAdrscanPath();
            const { stdout, stderr } = await execAsync(`"${adrscanPath}" init`, { cwd: workspaceFolder.uri.fsPath });
            if (stderr) {
                console.warn('ADRScan stderr:', stderr);
            }
            vscode.window.showInformationMessage('ADR structure initialized successfully!');
            this.adrExplorer.refresh();
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to initialize ADR structure: ${error.message}`);
        }
    }
    async runInventory() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }
        try {
            const adrscanPath = this.getAdrscanPath();
            const { stdout } = await execAsync(`"${adrscanPath}" inventory --format json`, { cwd: workspaceFolder.uri.fsPath });
            // Display inventory results
            const inventoryData = JSON.parse(stdout);
            this.showInventoryResults(inventoryData);
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to run inventory: ${error.message}`);
        }
    }
    async runDriftDetection() {
        vscode.window.showInformationMessage('Running drift detection...');
        this.driftDetection.refresh();
    }
    async generateProposals() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }
        try {
            vscode.window.showInformationMessage('Generating ADR proposals...');
            const adrscanPath = this.getAdrscanPath();
            const { stdout } = await execAsync(`"${adrscanPath}" propose --format json`, { cwd: workspaceFolder.uri.fsPath });
            const proposals = JSON.parse(stdout);
            this.showProposals(proposals);
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to generate proposals: ${error.message}`);
        }
    }
    async generateIndex() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }
        try {
            vscode.window.showInformationMessage('Generating ADR index...');
            const adrscanPath = this.getAdrscanPath();
            const { stdout } = await execAsync(`"${adrscanPath}" index`, { cwd: workspaceFolder.uri.fsPath });
            vscode.window.showInformationMessage('ADR index generated successfully!');
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to generate index: ${error.message}`);
        }
    }
    async createNewAdr() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }
        // Get ADR number
        const adrNumber = await vscode.window.showInputBox({
            prompt: 'Enter ADR number (e.g., 0005)',
            validateInput: (value) => {
                if (!value)
                    return 'ADR number is required';
                if (!/^\d{4}$/.test(value))
                    return 'ADR number must be 4 digits (e.g., 0005)';
                return null;
            }
        });
        if (!adrNumber)
            return;
        // Get ADR title
        const title = await vscode.window.showInputBox({
            prompt: 'Enter ADR title',
            validateInput: (value) => {
                if (!value)
                    return 'Title is required';
                return null;
            }
        });
        if (!title)
            return;
        // Select template
        const template = await vscode.window.showQuickPick([
            { label: 'MADR', description: 'Markdown Architecture Decision Record template' },
            { label: 'Basic', description: 'Simple ADR template' },
            { label: 'Custom', description: 'Use custom template' }
        ], { placeHolder: 'Select ADR template' });
        if (!template)
            return;
        try {
            const adrDir = path.join(workspaceFolder.uri.fsPath, 'docs', 'adr');
            const filename = `${adrNumber}-${title.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '')}.md`;
            const filepath = path.join(adrDir, filename);
            const content = this.generateAdrContent(adrNumber, title, template.label);
            await vscode.workspace.fs.writeFile(vscode.Uri.file(filepath), Buffer.from(content, 'utf8'));
            // Open the new ADR file
            const document = await vscode.workspace.openTextDocument(filepath);
            await vscode.window.showTextDocument(document);
            this.adrExplorer.refresh();
            vscode.window.showInformationMessage(`ADR ${adrNumber} created successfully!`);
        }
        catch (error) {
            vscode.window.showErrorMessage(`Failed to create ADR: ${error.message}`);
        }
    }
    async toggleLspServer() {
        const config = vscode.workspace.getConfiguration('photondrift');
        const isEnabled = config.get('lsp.enabled', true);
        await config.update('lsp.enabled', !isEnabled, vscode.ConfigurationTarget.Workspace);
        const status = !isEnabled ? 'enabled' : 'disabled';
        vscode.window.showInformationMessage(`PhotonDrift LSP server ${status}`);
    }
    async openDashboard() {
        // Create and show a webview panel for the analytics dashboard
        const panel = vscode.window.createWebviewPanel('photondriftDashboard', 'PhotonDrift Analytics Dashboard', vscode.ViewColumn.One, {
            enableScripts: true,
            retainContextWhenHidden: true
        });
        panel.webview.html = this.getDashboardHtml();
        // Handle messages from the webview
        panel.webview.onDidReceiveMessage(message => {
            switch (message.command) {
                case 'refresh':
                    this.refreshDashboardData(panel);
                    break;
            }
        }, undefined, this.context.subscriptions);
        // Load initial data
        this.refreshDashboardData(panel);
    }
    async openAdr(adr) {
        if (adr.resourceUri) {
            const document = await vscode.workspace.openTextDocument(adr.resourceUri);
            await vscode.window.showTextDocument(document);
        }
    }
    async editAdr(adr) {
        await this.openAdr(adr);
    }
    async deleteAdr(adr) {
        if (!adr.resourceUri)
            return;
        const response = await vscode.window.showWarningMessage(`Are you sure you want to delete ${path.basename(adr.resourceUri.fsPath)}?`, 'Delete', 'Cancel');
        if (response === 'Delete') {
            try {
                await vscode.workspace.fs.delete(adr.resourceUri);
                this.adrExplorer.refresh();
                vscode.window.showInformationMessage('ADR deleted successfully');
            }
            catch (error) {
                vscode.window.showErrorMessage(`Failed to delete ADR: ${error.message}`);
            }
        }
    }
    getAdrscanPath() {
        const config = vscode.workspace.getConfiguration('photondrift');
        return config.get('cli.path', 'adrscan');
    }
    generateAdrContent(number, title, template) {
        const date = new Date().toISOString().split('T')[0];
        if (template === 'MADR') {
            return `---
title: ${title}
status: proposed
date: ${date}
---

# ADR-${number}: ${title}

## Status

Proposed

## Context

<!-- Describe the technical issue or problem and its technical context -->

## Decision Drivers

<!-- List the key factors that influence the decision -->

* <!-- Driver 1 -->
* <!-- Driver 2 -->

## Considered Options

<!-- List the architecture design options considered -->

* <!-- Option 1 -->
* <!-- Option 2 -->

## Decision

<!-- State the decision -->

## Consequences

<!-- Describe the resulting context and consequences of the decision -->

### Positive

* <!-- Positive consequence 1 -->

### Negative

* <!-- Negative consequence 1 -->

## Links

<!-- List any related ADRs or external links -->
`;
        }
        else {
            return `# ADR-${number}: ${title}

## Status

Proposed

## Context

<!-- What is the issue that we're seeing that is motivating this decision or change? -->

## Decision

<!-- What is the change that we're proposing or have agreed to implement? -->

## Consequences

<!-- What becomes easier or more difficult to do and any risks introduced by the change that will need to be mitigated. -->
`;
        }
    }
    showInventoryResults(inventoryData) {
        const panel = vscode.window.createWebviewPanel('adrInventory', 'ADR Inventory', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = this.getInventoryHtml(inventoryData);
    }
    showProposals(proposals) {
        if (!proposals || proposals.length === 0) {
            vscode.window.showInformationMessage('No ADR proposals needed');
            return;
        }
        const panel = vscode.window.createWebviewPanel('adrProposals', 'ADR Proposals', vscode.ViewColumn.One, { enableScripts: true });
        panel.webview.html = this.getProposalsHtml(proposals);
    }
    getDashboardHtml() {
        return `
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PhotonDrift Analytics Dashboard</title>
    <style>
        body { 
            font-family: var(--vscode-font-family);
            color: var(--vscode-foreground);
            background-color: var(--vscode-editor-background);
            margin: 0;
            padding: 20px;
        }
        .header {
            border-bottom: 1px solid var(--vscode-panel-border);
            padding-bottom: 10px;
            margin-bottom: 20px;
        }
        .metrics {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .metric-card {
            background: var(--vscode-editor-inactiveSelectionBackground);
            border: 1px solid var(--vscode-panel-border);
            border-radius: 4px;
            padding: 15px;
            text-align: center;
        }
        .metric-value {
            font-size: 2em;
            font-weight: bold;
            color: var(--vscode-charts-blue);
            margin-bottom: 5px;
        }
        .metric-label {
            color: var(--vscode-descriptionForeground);
            font-size: 0.9em;
        }
        .chart-placeholder {
            background: var(--vscode-editor-inactiveSelectionBackground);
            border: 1px solid var(--vscode-panel-border);
            border-radius: 4px;
            padding: 20px;
            margin: 20px 0;
            text-align: center;
            color: var(--vscode-descriptionForeground);
        }
        .refresh-btn {
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>📊 PhotonDrift Analytics Dashboard</h1>
        <button class="refresh-btn" onclick="refresh()">Refresh Data</button>
    </div>
    
    <div class="metrics" id="metrics">
        <div class="metric-card">
            <div class="metric-value">-</div>
            <div class="metric-label">Total ADRs</div>
        </div>
        <div class="metric-card">
            <div class="metric-value">-</div>
            <div class="metric-label">Drift Items</div>
        </div>
        <div class="metric-card">
            <div class="metric-value">-</div>
            <div class="metric-label">Coverage</div>
        </div>
        <div class="metric-card">
            <div class="metric-value">-</div>
            <div class="metric-label">Health Score</div>
        </div>
    </div>
    
    <div class="chart-placeholder">
        📈 Architecture Drift Timeline<br>
        <small>Real-time charts coming soon...</small>
    </div>
    
    <div class="chart-placeholder">
        🎯 Decision Coverage Map<br>
        <small>Interactive visualization coming soon...</small>
    </div>
    
    <script>
        const vscode = acquireVsCodeApi();
        
        function refresh() {
            vscode.postMessage({ command: 'refresh' });
        }
        
        window.addEventListener('message', event => {
            const message = event.data;
            if (message.command === 'updateData') {
                updateMetrics(message.data);
            }
        });
        
        function updateMetrics(data) {
            const metrics = document.getElementById('metrics');
            if (data) {
                metrics.innerHTML = \`
                    <div class="metric-card">
                        <div class="metric-value">\${data.totalAdrs || 0}</div>
                        <div class="metric-label">Total ADRs</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">\${data.driftItems || 0}</div>
                        <div class="metric-label">Drift Items</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">\${data.coverage || '0%'}</div>
                        <div class="metric-label">Coverage</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">\${data.healthScore || 'N/A'}</div>
                        <div class="metric-label">Health Score</div>
                    </div>
                \`;
            }
        }
    </script>
</body>
</html>`;
    }
    getInventoryHtml(inventoryData) {
        const adrs = inventoryData.adrs || [];
        const adrRows = adrs.map((adr) => `
            <tr>
                <td>${adr.number || ''}</td>
                <td>${adr.title || ''}</td>
                <td><span class="status-${(adr.status || '').toLowerCase()}">${adr.status || ''}</span></td>
                <td>${adr.date || ''}</td>
            </tr>
        `).join('');
        return `
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 8px; text-align: left; border-bottom: 1px solid var(--vscode-panel-border); }
        th { background-color: var(--vscode-editor-inactiveSelectionBackground); }
        .status-accepted { color: var(--vscode-charts-green); }
        .status-proposed { color: var(--vscode-charts-yellow); }
        .status-rejected { color: var(--vscode-charts-red); }
    </style>
</head>
<body>
    <h2>ADR Inventory (${adrs.length} records)</h2>
    <table>
        <thead>
            <tr><th>Number</th><th>Title</th><th>Status</th><th>Date</th></tr>
        </thead>
        <tbody>
            ${adrRows}
        </tbody>
    </table>
</body>
</html>`;
    }
    getProposalsHtml(proposals) {
        const proposalItems = proposals.map((proposal) => `
            <div class="proposal">
                <h3>${proposal.title || 'Unnamed Proposal'}</h3>
                <p>${proposal.description || 'No description'}</p>
                <small>Severity: ${proposal.severity || 'Unknown'}</small>
            </div>
        `).join('');
        return `
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); padding: 20px; }
        .proposal { 
            border: 1px solid var(--vscode-panel-border); 
            padding: 15px; 
            margin: 10px 0; 
            border-radius: 4px;
            background: var(--vscode-editor-inactiveSelectionBackground);
        }
        .proposal h3 { margin-top: 0; color: var(--vscode-charts-blue); }
    </style>
</head>
<body>
    <h2>📝 ADR Proposals (${proposals.length})</h2>
    ${proposalItems}
</body>
</html>`;
    }
    async refreshDashboardData(panel) {
        try {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (!workspaceFolder)
                return;
            const adrscanPath = this.getAdrscanPath();
            const [inventoryResult, driftResult] = await Promise.allSettled([
                execAsync(`"${adrscanPath}" inventory --format json`, { cwd: workspaceFolder.uri.fsPath }),
                execAsync(`"${adrscanPath}" diff --format json`, { cwd: workspaceFolder.uri.fsPath })
            ]);
            let totalAdrs = 0;
            let driftItems = 0;
            if (inventoryResult.status === 'fulfilled') {
                const inventoryData = JSON.parse(inventoryResult.value.stdout);
                totalAdrs = inventoryData.adrs?.length || 0;
            }
            if (driftResult.status === 'fulfilled') {
                const driftData = JSON.parse(driftResult.value.stdout);
                driftItems = driftData.items?.length || 0;
            }
            const coverage = totalAdrs > 0 ? Math.round((totalAdrs / (totalAdrs + driftItems)) * 100) + '%' : '0%';
            const healthScore = driftItems === 0 ? '100' : Math.max(0, 100 - (driftItems * 10));
            panel.webview.postMessage({
                command: 'updateData',
                data: {
                    totalAdrs,
                    driftItems,
                    coverage,
                    healthScore
                }
            });
        }
        catch (error) {
            console.error('Error refreshing dashboard data:', error);
        }
    }
}
exports.AdrCommandProvider = AdrCommandProvider;
//# sourceMappingURL=commands.js.map