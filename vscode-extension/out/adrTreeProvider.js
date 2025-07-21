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
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.ADRTreeProvider = void 0;
const vscode = __importStar(require("vscode"));
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
class ADRTreeProvider {
    context;
    _onDidChangeTreeData = new vscode.EventEmitter();
    onDidChangeTreeData = this._onDidChangeTreeData.event;
    constructor(context) {
        this.context = context;
    }
    refresh() {
        this._onDidChangeTreeData.fire();
    }
    getTreeItem(element) {
        return element;
    }
    getChildren(element) {
        if (!element) {
            // Root level - return ADR directories and files
            return this.getADRItems();
        }
        else if (element.type === 'directory') {
            // Directory level - return ADR files in this directory
            return this.getADRFilesInDirectory(element.resourceUri.fsPath);
        }
        return Promise.resolve([]);
    }
    async getADRItems() {
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            return [];
        }
        const config = vscode.workspace.getConfiguration('photondrift');
        const adrDirectory = config.get('adrDirectory', 'docs/adr');
        const adrPath = path.join(workspaceFolder.uri.fsPath, adrDirectory);
        try {
            // Check if ADR directory exists
            if (!fs.existsSync(adrPath)) {
                return [];
            }
            const items = [];
            const entries = fs.readdirSync(adrPath, { withFileTypes: true });
            // Add directories first
            for (const entry of entries) {
                if (entry.isDirectory()) {
                    const dirPath = path.join(adrPath, entry.name);
                    const dirItem = new ADRItem(entry.name, vscode.TreeItemCollapsibleState.Collapsed, 'directory', vscode.Uri.file(dirPath));
                    dirItem.iconPath = new vscode.ThemeIcon('folder');
                    dirItem.contextValue = 'adrDirectory';
                    items.push(dirItem);
                }
            }
            // Add ADR files
            const adrFiles = await this.getADRFilesInDirectory(adrPath);
            items.push(...adrFiles);
            return items;
        }
        catch (error) {
            console.error('Error reading ADR directory:', error);
            return [];
        }
    }
    async getADRFilesInDirectory(dirPath) {
        try {
            if (!fs.existsSync(dirPath)) {
                return [];
            }
            const files = fs.readdirSync(dirPath);
            const adrFiles = files.filter(file => file.endsWith('.adr.md') || file.endsWith('.md'));
            const items = [];
            for (const file of adrFiles) {
                const filePath = path.join(dirPath, file);
                const stats = fs.statSync(filePath);
                // Parse ADR metadata
                const metadata = await this.parseADRMetadata(filePath);
                const item = new ADRItem(metadata.title || path.basename(file, '.adr.md'), vscode.TreeItemCollapsibleState.None, 'adr', vscode.Uri.file(filePath));
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
                const aStats = fs.statSync(a.resourceUri.fsPath);
                const bStats = fs.statSync(b.resourceUri.fsPath);
                return bStats.mtime.getTime() - aStats.mtime.getTime();
            });
            return items;
        }
        catch (error) {
            console.error('Error reading ADR files:', error);
            return [];
        }
    }
    async parseADRMetadata(filePath) {
        try {
            const content = fs.readFileSync(filePath, 'utf8');
            const metadata = {};
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
        }
        catch (error) {
            console.error('Error parsing ADR metadata:', error);
            return {};
        }
    }
    getStatusIcon(status) {
        if (!status) {
            return new vscode.ThemeIcon('file-text');
        }
        const statusLower = status.toLowerCase();
        if (statusLower.includes('accepted') || statusLower.includes('approved')) {
            return new vscode.ThemeIcon('check', new vscode.ThemeColor('charts.green'));
        }
        else if (statusLower.includes('proposed') || statusLower.includes('draft')) {
            return new vscode.ThemeIcon('clock', new vscode.ThemeColor('charts.yellow'));
        }
        else if (statusLower.includes('deprecated') || statusLower.includes('superseded')) {
            return new vscode.ThemeIcon('archive', new vscode.ThemeColor('charts.red'));
        }
        else if (statusLower.includes('rejected')) {
            return new vscode.ThemeIcon('x', new vscode.ThemeColor('charts.red'));
        }
        return new vscode.ThemeIcon('file-text');
    }
}
exports.ADRTreeProvider = ADRTreeProvider;
class ADRItem extends vscode.TreeItem {
    label;
    collapsibleState;
    type;
    resourceUri;
    constructor(label, collapsibleState, type, resourceUri) {
        super(label, collapsibleState);
        this.label = label;
        this.collapsibleState = collapsibleState;
        this.type = type;
        this.resourceUri = resourceUri;
    }
}
//# sourceMappingURL=adrTreeProvider.js.map