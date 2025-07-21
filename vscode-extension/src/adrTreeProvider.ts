import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

export class ADRTreeProvider implements vscode.TreeDataProvider<ADRItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<ADRItem | undefined | null | void> = new vscode.EventEmitter<ADRItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<ADRItem | undefined | null | void> = this._onDidChangeTreeData.event;

    constructor(private context: vscode.ExtensionContext) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: ADRItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: ADRItem): Thenable<ADRItem[]> {
        if (!element) {
            // Root level - return ADR directories and files
            return this.getADRItems();
        } else if (element.type === 'directory') {
            // Directory level - return ADR files in this directory
            return this.getADRFilesInDirectory(element.resourceUri!.fsPath);
        }
        
        return Promise.resolve([]);
    }

    private async getADRItems(): Promise<ADRItem[]> {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            return [];
        }

        const config = vscode.workspace.getConfiguration('photondrift');
        const adrDirectory = config.get<string>('adrDirectory', 'docs/adr');
        const adrPath = path.join(workspaceFolder.uri.fsPath, adrDirectory);

        try {
            // Check if ADR directory exists
            if (!fs.existsSync(adrPath)) {
                return [];
            }

            const items: ADRItem[] = [];
            const entries = fs.readdirSync(adrPath, { withFileTypes: true });

            // Add directories first
            for (const entry of entries) {
                if (entry.isDirectory()) {
                    const dirPath = path.join(adrPath, entry.name);
                    const dirItem = new ADRItem(
                        entry.name,
                        vscode.TreeItemCollapsibleState.Collapsed,
                        'directory',
                        vscode.Uri.file(dirPath)
                    );
                    dirItem.iconPath = new vscode.ThemeIcon('folder');
                    dirItem.contextValue = 'adrDirectory';
                    items.push(dirItem);
                }
            }

            // Add ADR files
            const adrFiles = await this.getADRFilesInDirectory(adrPath);
            items.push(...adrFiles);

            return items;
        } catch (error) {
            console.error('Error reading ADR directory:', error);
            return [];
        }
    }

    private async getADRFilesInDirectory(dirPath: string): Promise<ADRItem[]> {
        try {
            if (!fs.existsSync(dirPath)) {
                return [];
            }

            const files = fs.readdirSync(dirPath);
            const adrFiles = files.filter(file => file.endsWith('.adr.md') || file.endsWith('.md'));
            
            const items: ADRItem[] = [];

            for (const file of adrFiles) {
                const filePath = path.join(dirPath, file);
                const stats = fs.statSync(filePath);
                
                // Parse ADR metadata
                const metadata = await this.parseADRMetadata(filePath);
                
                const item = new ADRItem(
                    metadata.title || path.basename(file, '.adr.md'),
                    vscode.TreeItemCollapsibleState.None,
                    'adr',
                    vscode.Uri.file(filePath)
                );

                item.description = metadata.status || 'Unknown Status';
                item.tooltip = new vscode.MarkdownString([
                    `**Title:** ${metadata.title || 'Untitled'}`,
                    `**Status:** ${metadata.status || 'Unknown'}`,
                    `**Date:** ${metadata.date || stats.mtime.toDateString()}`,
                    `**File:** ${file}`,
                    metadata.summary ? `**Summary:** ${metadata.summary}` : ''
                ].filter(Boolean).join('\n\n'));

                // Set icon based on status
                item.iconPath = this.getStatusIcon(metadata.status);
                
                // Set context for right-click menu
                item.contextValue = 'adrFile';
                
                // Make it clickable
                item.command = {
                    command: 'vscode.open',
                    title: 'Open',
                    arguments: [vscode.Uri.file(filePath)]
                };

                items.push(item);
            }

            // Sort by date (newest first) or by filename
            items.sort((a, b) => {
                const aStats = fs.statSync(a.resourceUri!.fsPath);
                const bStats = fs.statSync(b.resourceUri!.fsPath);
                return bStats.mtime.getTime() - aStats.mtime.getTime();
            });

            return items;
        } catch (error) {
            console.error('Error reading ADR files:', error);
            return [];
        }
    }

    private async parseADRMetadata(filePath: string): Promise<ADRMetadata> {
        try {
            const content = fs.readFileSync(filePath, 'utf8');
            const metadata: ADRMetadata = {};

            // Extract title (first # header)
            const titleMatch = content.match(/^#\s+(.+)$/m);
            if (titleMatch) {
                metadata.title = titleMatch[1].trim();
            }

            // Extract status
            const statusMatch = content.match(/##\s+Status\s*\n\s*(.+)/i);
            if (statusMatch) {
                metadata.status = statusMatch[1].trim();
            }

            // Extract date
            const dateMatch = content.match(/##\s+Date\s*\n\s*(.+)/i) || 
                            content.match(/Date:\s*(.+)/i) ||
                            content.match(/Created:\s*(.+)/i);
            if (dateMatch) {
                metadata.date = dateMatch[1].trim();
            }

            // Extract summary/context (first paragraph under Context section)
            const contextMatch = content.match(/##\s+Context\s*\n\s*(.+?)(\n\n|\n##|$)/is);
            if (contextMatch) {
                metadata.summary = contextMatch[1].trim().substring(0, 200) + '...';
            }

            return metadata;
        } catch (error) {
            console.error('Error parsing ADR metadata:', error);
            return {};
        }
    }

    private getStatusIcon(status?: string): vscode.ThemeIcon {
        if (!status) {
            return new vscode.ThemeIcon('file-text');
        }

        const statusLower = status.toLowerCase();
        
        if (statusLower.includes('accepted') || statusLower.includes('approved')) {
            return new vscode.ThemeIcon('check', new vscode.ThemeColor('charts.green'));
        } else if (statusLower.includes('proposed') || statusLower.includes('draft')) {
            return new vscode.ThemeIcon('clock', new vscode.ThemeColor('charts.yellow'));
        } else if (statusLower.includes('deprecated') || statusLower.includes('superseded')) {
            return new vscode.ThemeIcon('archive', new vscode.ThemeColor('charts.red'));
        } else if (statusLower.includes('rejected')) {
            return new vscode.ThemeIcon('x', new vscode.ThemeColor('charts.red'));
        }

        return new vscode.ThemeIcon('file-text');
    }
}

interface ADRMetadata {
    title?: string;
    status?: string;
    date?: string;
    summary?: string;
}

class ADRItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly type: 'directory' | 'adr',
        public readonly resourceUri?: vscode.Uri
    ) {
        super(label, collapsibleState);
    }
}