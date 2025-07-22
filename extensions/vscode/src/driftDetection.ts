import * as vscode from 'vscode';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export class DriftDetectionProvider implements vscode.TreeDataProvider<DriftItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<DriftItem | undefined | null | void> = new vscode.EventEmitter<DriftItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<DriftItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private driftItems: DriftItem[] = [];
    private isRunning: boolean = false;

    constructor(private context: vscode.ExtensionContext) {
        this.runDriftDetection();
    }

    refresh(): void {
        this.runDriftDetection();
    }

    getTreeItem(element: DriftItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: DriftItem): DriftItem[] {
        if (!element) {
            return this.driftItems;
        }
        return element.children || [];
    }

    private async runDriftDetection(): Promise<void> {
        if (this.isRunning) {
            return;
        }

        this.isRunning = true;
        this.driftItems = [];
        this._onDidChangeTreeData.fire();

        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            this.isRunning = false;
            return;
        }

        try {
            // Add loading item
            const loadingItem = new DriftItem(
                'Scanning for architectural drift...',
                vscode.TreeItemCollapsibleState.None,
                'loading'
            );
            loadingItem.iconPath = new vscode.ThemeIcon('loading~spin');
            this.driftItems = [loadingItem];
            this._onDidChangeTreeData.fire();

            for (const workspace of workspaceFolders) {
                await this.scanWorkspaceForDrift(workspace);
            }
        } catch (error) {
            console.error('Error running drift detection:', error);
            this.driftItems = [
                new DriftItem(
                    `Error: ${error}`,
                    vscode.TreeItemCollapsibleState.None,
                    'error'
                )
            ];
        } finally {
            this.isRunning = false;
            this._onDidChangeTreeData.fire();
        }
    }

    private async scanWorkspaceForDrift(workspace: vscode.WorkspaceFolder): Promise<void> {
        const workspacePath = workspace.uri.fsPath;
        
        try {
            // Check if adrscan is available
            const adrscanPath = this.getAdrscanPath();
            
            // Run drift detection
            const { stdout, stderr } = await execAsync(
                `"${adrscanPath}" diff --format json`,
                { cwd: workspacePath, timeout: 30000 }
            );

            if (stderr) {
                console.warn('ADRScan stderr:', stderr);
            }

            // Parse results
            const driftData = this.parseDriftOutput(stdout);
            this.processDriftData(driftData, workspace.name);
            
        } catch (error: any) {
            console.error(`Error scanning ${workspace.name}:`, error);
            
            if (error.code === 'ENOENT') {
                this.driftItems = [
                    new DriftItem(
                        'PhotonDrift not found',
                        vscode.TreeItemCollapsibleState.None,
                        'error'
                    )
                ];
            } else {
                this.driftItems.push(
                    new DriftItem(
                        `Error in ${workspace.name}: ${error.message}`,
                        vscode.TreeItemCollapsibleState.None,
                        'error'
                    )
                );
            }
        }
    }

    private getAdrscanPath(): string {
        const config = vscode.workspace.getConfiguration('photondrift');
        return config.get('cli.path', 'adrscan');
    }

    private parseDriftOutput(output: string): DriftData {
        try {
            if (!output.trim()) {
                return { items: [] };
            }
            return JSON.parse(output);
        } catch (error) {
            console.error('Error parsing drift output:', error);
            // Try to parse as plain text
            return this.parseTextOutput(output);
        }
    }

    private parseTextOutput(output: string): DriftData {
        const lines = output.split('\n').filter(line => line.trim());
        const items: DriftDataItem[] = [];

        for (const line of lines) {
            if (line.includes('DRIFT DETECTED') || line.includes('WARNING') || line.includes('ERROR')) {
                items.push({
                    type: 'drift',
                    severity: this.extractSeverity(line),
                    message: line,
                    file: this.extractFilePath(line),
                    category: 'general'
                });
            }
        }

        return { items };
    }

    private extractSeverity(line: string): string {
        if (line.includes('ERROR')) return 'error';
        if (line.includes('WARNING')) return 'warning';
        if (line.includes('HIGH')) return 'high';
        if (line.includes('MEDIUM')) return 'medium';
        return 'low';
    }

    private extractFilePath(line: string): string | undefined {
        const match = line.match(/([^\s]+\.(rs|js|ts|py|md))/);
        return match ? match[1] : undefined;
    }

    private processDriftData(driftData: DriftData, workspaceName: string): void {
        if (!driftData.items || driftData.items.length === 0) {
            this.driftItems = [
                new DriftItem(
                    '✅ No architectural drift detected',
                    vscode.TreeItemCollapsibleState.None,
                    'success'
                )
            ];
            return;
        }

        // Group by category
        const categories = new Map<string, DriftDataItem[]>();
        
        for (const item of driftData.items) {
            const category = item.category || 'General';
            if (!categories.has(category)) {
                categories.set(category, []);
            }
            categories.get(category)!.push(item);
        }

        this.driftItems = [];

        // Summary item
        const totalCount = driftData.items.length;
        const highSeverityCount = driftData.items.filter(item => 
            item.severity === 'high' || item.severity === 'error'
        ).length;
        
        const summaryItem = new DriftItem(
            `🚨 ${totalCount} drift item${totalCount !== 1 ? 's' : ''} detected`,
            vscode.TreeItemCollapsibleState.Expanded,
            'summary'
        );
        summaryItem.description = `${highSeverityCount} high priority`;
        summaryItem.iconPath = new vscode.ThemeIcon('warning', 
            highSeverityCount > 0 ? new vscode.ThemeColor('charts.red') : new vscode.ThemeColor('charts.yellow')
        );
        this.driftItems.push(summaryItem);

        // Create category items
        for (const [category, items] of categories) {
            const categoryItem = new DriftItem(
                `${category} (${items.length})`,
                vscode.TreeItemCollapsibleState.Expanded,
                'category'
            );
            categoryItem.iconPath = new vscode.ThemeIcon('folder');
            
            // Create children for each drift item
            categoryItem.children = items.map(item => {
                const driftItem = new DriftItem(
                    item.message || 'Unknown drift',
                    vscode.TreeItemCollapsibleState.None,
                    'driftItem'
                );
                
                driftItem.description = item.severity?.toUpperCase();
                driftItem.iconPath = this.getSeverityIcon(item.severity);
                driftItem.tooltip = this.createDriftTooltip(item);
                
                if (item.file) {
                    const filePath = path.isAbsolute(item.file) 
                        ? item.file 
                        : path.join(workspaceName, item.file);
                    
                    driftItem.resourceUri = vscode.Uri.file(filePath);
                    driftItem.command = {
                        command: 'vscode.open',
                        title: 'Open File',
                        arguments: [driftItem.resourceUri]
                    };
                }
                
                return driftItem;
            });
            
            this.driftItems.push(categoryItem);
        }
    }

    private getSeverityIcon(severity?: string): vscode.ThemeIcon {
        switch (severity) {
            case 'error':
            case 'high':
                return new vscode.ThemeIcon('error', new vscode.ThemeColor('charts.red'));
            case 'warning':
            case 'medium':
                return new vscode.ThemeIcon('warning', new vscode.ThemeColor('charts.orange'));
            case 'low':
            case 'info':
                return new vscode.ThemeIcon('info', new vscode.ThemeColor('charts.blue'));
            default:
                return new vscode.ThemeIcon('circle-filled');
        }
    }

    private createDriftTooltip(item: DriftDataItem): vscode.MarkdownString {
        const tooltip = new vscode.MarkdownString();
        tooltip.isTrusted = true;
        
        tooltip.appendMarkdown(`**${item.type?.toUpperCase() || 'DRIFT'}**\n\n`);
        tooltip.appendMarkdown(`**Severity:** ${item.severity?.toUpperCase() || 'UNKNOWN'}\n\n`);
        
        if (item.message) {
            tooltip.appendMarkdown(`**Message:** ${item.message}\n\n`);
        }
        
        if (item.file) {
            tooltip.appendMarkdown(`**File:** \`${item.file}\`\n\n`);
        }
        
        if (item.category) {
            tooltip.appendMarkdown(`**Category:** ${item.category}\n\n`);
        }
        
        tooltip.appendMarkdown('Click to open the affected file.');
        
        return tooltip;
    }
}

class DriftItem extends vscode.TreeItem {
    public children?: DriftItem[];

    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string
    ) {
        super(label, collapsibleState);
    }
}

interface DriftData {
    items: DriftDataItem[];
}

interface DriftDataItem {
    type?: string;
    severity?: string;
    message?: string;
    file?: string;
    category?: string;
}