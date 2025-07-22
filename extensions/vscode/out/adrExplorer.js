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
exports.AdrExplorerProvider = void 0;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
class AdrExplorerProvider {
    constructor(context) {
        this.context = context;
        this._onDidChangeTreeData = new vscode.EventEmitter();
        this.onDidChangeTreeData = this._onDidChangeTreeData.event;
    }
    refresh() {
        this._onDidChangeTreeData.fire();
    }
    getTreeItem(element) {
        return element;
    }
    async getChildren(element) {
        if (!element) {
            return this.getAdrDirectories();
        }
        if (element.contextValue === 'adrDirectory') {
            return this.getAdrFiles(element.resourceUri.fsPath);
        }
        return [];
    }
    async getAdrDirectories() {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            return [];
        }
        const adrDirectories = [];
        for (const folder of workspaceFolders) {
            const adrPath = path.join(folder.uri.fsPath, 'docs', 'adr');
            try {
                const stat = await vscode.workspace.fs.stat(vscode.Uri.file(adrPath));
                if (stat.type === vscode.FileType.Directory) {
                    const adrCount = await this.getAdrFileCount(adrPath);
                    const item = new AdrItem(`ADRs (${adrCount})`, vscode.TreeItemCollapsibleState.Expanded, 'adrDirectory', vscode.Uri.file(adrPath));
                    item.iconPath = new vscode.ThemeIcon('folder');
                    item.description = `${adrCount} decision records`;
                    adrDirectories.push(item);
                }
            }
            catch {
                // ADR directory doesn't exist
            }
        }
        return adrDirectories;
    }
    async getAdrFiles(adrDirPath) {
        try {
            const entries = await vscode.workspace.fs.readDirectory(vscode.Uri.file(adrDirPath));
            const adrFiles = [];
            const sortedEntries = entries
                .filter(([name, type]) => type === vscode.FileType.File && name.endsWith('.md'))
                .sort((a, b) => a[0].localeCompare(b[0]));
            for (const [name, type] of sortedEntries) {
                const filePath = path.join(adrDirPath, name);
                const adrInfo = await this.parseAdrFile(filePath);
                const item = new AdrItem(adrInfo.title || name, vscode.TreeItemCollapsibleState.None, 'adrFile', vscode.Uri.file(filePath));
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
        }
        catch (error) {
            console.error(`Error reading ADR directory: ${error}`);
            return [];
        }
    }
    async getAdrFileCount(adrDirPath) {
        try {
            const entries = await vscode.workspace.fs.readDirectory(vscode.Uri.file(adrDirPath));
            return entries.filter(([name, type]) => type === vscode.FileType.File && name.endsWith('.md')).length;
        }
        catch {
            return 0;
        }
    }
    async parseAdrFile(filePath) {
        try {
            const content = await vscode.workspace.fs.readFile(vscode.Uri.file(filePath));
            const text = Buffer.from(content).toString('utf8');
            const info = {
                title: this.extractTitle(text),
                status: this.extractStatus(text),
                date: this.extractDate(text),
                tags: this.extractTags(text)
            };
            return info;
        }
        catch (error) {
            console.error(`Error parsing ADR file ${filePath}: ${error}`);
            return {};
        }
    }
    extractTitle(content) {
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
    extractStatus(content) {
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
    extractDate(content) {
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?date:\s*(.+)\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].trim().replace(/['"]/g, '');
        }
        return undefined;
    }
    extractTags(content) {
        const frontmatterMatch = content.match(/^---\s*\n[\s\S]*?tags:\s*\[(.+)\]\s*\n[\s\S]*?---/);
        if (frontmatterMatch) {
            return frontmatterMatch[1].split(',').map(tag => tag.trim().replace(/['"]/g, ''));
        }
        return [];
    }
    createAdrTooltip(adrInfo, filename) {
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
    getAdrIcon(status) {
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
exports.AdrExplorerProvider = AdrExplorerProvider;
class AdrItem extends vscode.TreeItem {
    constructor(label, collapsibleState, contextValue, resourceUri) {
        super(label, collapsibleState);
        this.label = label;
        this.collapsibleState = collapsibleState;
        this.contextValue = contextValue;
        this.resourceUri = resourceUri;
    }
}
//# sourceMappingURL=adrExplorer.js.map