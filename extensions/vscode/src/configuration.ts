import * as vscode from 'vscode';

export class ConfigurationManager {
    private config: vscode.WorkspaceConfiguration;

    constructor() {
        this.config = vscode.workspace.getConfiguration('photondrift');
    }

    reload(): void {
        this.config = vscode.workspace.getConfiguration('photondrift');
    }

    // LSP Configuration
    isLspEnabled(): boolean {
        return this.config.get('lsp.enabled', true);
    }

    getLspServerPath(): string {
        return this.config.get('lsp.serverPath', 'adrscan-lsp');
    }

    getMaxDiagnostics(): number {
        return this.config.get('lsp.maxDiagnostics', 100);
    }

    // ADR Configuration
    getAdrDirectory(): string {
        return this.config.get('adr.directory', 'docs/adr');
    }

    getAdrTemplate(): string {
        return this.config.get('adr.template', 'madr');
    }

    // Drift Detection Configuration
    isDriftEnabled(): boolean {
        return this.config.get('drift.enabled', true);
    }

    isWatchModeEnabled(): boolean {
        return this.config.get('drift.watchMode', true);
    }

    // ML Configuration
    isMlEnabled(): boolean {
        return this.config.get('ml.enabled', true);
    }

    getMlModel(): string {
        return this.config.get('ml.model', 'Ensemble');
    }

    // UI Configuration
    shouldShowStatusBar(): boolean {
        return this.config.get('ui.showStatusBar', true);
    }

    getTheme(): string {
        return this.config.get('ui.theme', 'auto');
    }

    // Notifications Configuration
    areNotificationsEnabled(): boolean {
        return this.config.get('notifications.enabled', true);
    }

    // Analytics Configuration
    areAnalyticsEnabled(): boolean {
        return this.config.get('analytics.enabled', false);
    }

    // CLI Configuration
    getCliPath(): string {
        return this.config.get('cli.path', 'adrscan');
    }
}