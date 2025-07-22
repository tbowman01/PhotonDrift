import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export class AdrExplorerProvider implements vscode.TreeDataProvider<AdrItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<AdrItem | undefined | null | void> = new vscode.EventEmitter<AdrItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<AdrItem | undefined | null | void> = this._onDidChangeTreeData.event;

    constructor(private context: vscode.ExtensionContext) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: AdrItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: AdrItem): Promise<AdrItem[]> {
        if (!element) {
            return this.getAdrDirectories();
        }

        if (element.contextValue === 'adrDirectory') {
            return this.getAdrFiles(element.resourceUri!.fsPath);
        }

        return [];
    }

    private async getAdrDirectories(): Promise<AdrItem[]> {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            return [];
        }

        const adrDirectories: AdrItem[] = [];

        for (const folder of workspaceFolders) {
            const adrPath = path.join(folder.uri.fsPath, 'docs', 'adr');
            
            try {
                const stat = await vscode.workspace.fs.stat(vscode.Uri.file(adrPath));
                if (stat.type === vscode.FileType.Directory) {
                    const adrCount = await this.getAdrFileCount(adrPath);
                    const item = new AdrItem(
                        `ADRs (${adrCount})`,
                        vscode.TreeItemCollapsibleState.Expanded,
                        'adrDirectory',
                        vscode.Uri.file(adrPath)
                    );
                    item.iconPath = new vscode.ThemeIcon('folder');
                    item.description = `${adrCount} decision records`;
                    adrDirectories.push(item);
                }
            } catch {
                // ADR directory doesn't exist
            }
        }

        return adrDirectories;
    }

    private async getAdrFiles(adrDirPath: string): Promise<AdrItem[]> {
        try {
            const entries = await vscode.workspace.fs.readDirectory(vscode.Uri.file(adrDirPath));
            const adrFiles: AdrItem[] = [];

            const sortedEntries = entries
                .filter(([name, type]) => type === vscode.FileType.File && name.endsWith('.md'))
                .sort((a, b) => a[0].localeCompare(b[0]));

            for (const [name, type] of sortedEntries) {
                const filePath = path.join(adrDirPath, name);
                const adrInfo = await this.parseAdrFile(filePath);
                
                const item = new AdrItem(
                    adrInfo.title || name,
                    vscode.TreeItemCollapsibleState.None,
                    'adrFile',
                    vscode.Uri.file(filePath)
                );
                
                item.description = adrInfo.status || 'Unknown status';
                item.tooltip = this.createAdrTooltip(adrInfo, name);
                item.iconPath = this.getAdrIcon(adrInfo.status);
                
                // Add command to open the file
                item.command = {
                    command: 'vscode.open',
                    title: 'Open ADR',
                    arguments: [vscode.Uri.file(filePath)]
                };

                adrFiles.push(item);
            }

            return adrFiles;
        } catch (error) {
            console.error(`Error reading ADR directory: ${error}`);
            return [];
        }
    }

    private async getAdrFileCount(adrDirPath: string): Promise<number> {
        try {
            const entries = await vscode.workspace.fs.readDirectory(vscode.Uri.file(adrDirPath));
            return entries.filter(([name, type]) => 
                type === vscode.FileType.File && name.endsWith('.md')
            ).length;
        } catch {
            return 0;
        }
    }

    private async parseAdrFile(filePath: string): Promise<AdrInfo> {
        try {
            const content = await vscode.workspace.fs.readFile(vscode.Uri.file(filePath));
            const text = Buffer.from(content).toString('utf8');
            
            const info: AdrInfo = {
                title: this.extractTitle(text),
                status: this.extractStatus(text),
                date: this.extractDate(text),
                tags: this.extractTags(text)
            };

            return info;
        } catch (error) {
            console.error(`Error parsing ADR file ${filePath}: ${error}`);
            return {};
        }
    }

    private extractTitle(content: string): string | undefined {
        // Look for title in frontmatter or first heading
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?title:\s*(.+)\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].trim().replace(/['"]/g, '');
        }

        // Look for first heading
        const headingMatch = content.match(/^#\s*(.+)/m);
        if (headingMatch) {
            return headingMatch[1].trim();
        }

        return undefined;
    }

    private extractStatus(content: string): string | undefined {
        // Look for status in frontmatter
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?status:\s*(.+)\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].trim().replace(/['"]/g, '');
        }

        // Look for status section
        const statusMatch = content.match(/##\s*Status\s*\n\s*(.+)/i);
        if (statusMatch) {
            return statusMatch[1].trim();
        }

        return undefined;
    }

    private extractDate(content: string): string | undefined {
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?date:\s*(.+)\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].trim().replace(/['"]/g, '');
        }
        return undefined;
    }

    private extractTags(content: string): string[] {
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?tags:\s*\[(.+)\]\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].split(',').map(tag => tag.trim().replace(/['"]/g, ''));
        }
        return [];
    }

    private createAdrTooltip(adrInfo: AdrInfo, filename: string): vscode.MarkdownString {
        const tooltip = new vscode.MarkdownString();
        tooltip.isTrusted = true;
        
        tooltip.appendMarkdown(`**${adrInfo.title || filename}**\n\n`);
        
        if (adrInfo.status) {
            tooltip.appendMarkdown(`**Status:** ${adrInfo.status}\n\n`);
        }
        
        if (adrInfo.date) {
            tooltip.appendMarkdown(`**Date:** ${adrInfo.date}\n\n`);
        }
        
        if (adrInfo.tags && adrInfo.tags.length > 0) {
            tooltip.appendMarkdown(`**Tags:** ${adrInfo.tags.join(', ')}\n\n`);
        }
        
        tooltip.appendMarkdown(`**File:** \`${filename}\``);
        
        return tooltip;
    }

    private getAdrIcon(status?: string): vscode.ThemeIcon {
        switch (status?.toLowerCase()) {
            case 'accepted':
                return new vscode.ThemeIcon('check', new vscode.ThemeColor('charts.green'));
            case 'proposed':
                return new vscode.ThemeIcon('clock', new vscode.ThemeColor('charts.yellow'));
            case 'rejected':
                return new vscode.ThemeIcon('x', new vscode.ThemeColor('charts.red'));
            case 'superseded':
            case 'deprecated':
                return new vscode.ThemeIcon('archive', new vscode.ThemeColor('charts.gray'));
            default:
                return new vscode.ThemeIcon('file-text');
        }
    }
}

class AdrItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string,
        public readonly resourceUri?: vscode.Uri
    ) {
        super(label, collapsibleState);
    }
}

interface AdrInfo {
    title?: string;
    status?: string;
    date?: string;
    tags?: string[];
}