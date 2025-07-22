import * as vscode from 'vscode';

export class StatusBarManager {
    private statusBarItem: vscode.StatusBarItem;
    private lspStatus: 'stopped' | 'starting' | 'running' | 'error' = 'stopped';
    private lastDriftCount: number = 0;

    constructor(private context: vscode.ExtensionContext) {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            10
        );
        this.statusBarItem.command = 'photondrift.openDashboard';
        this.context.subscriptions.push(this.statusBarItem);
        this.update();
    }

    show(): void {
        const config = vscode.workspace.getConfiguration('photondrift');
        if (config.get('ui.showStatusBar', true)) {
            this.statusBarItem.show();
        }
    }

    hide(): void {
        this.statusBarItem.hide();
    }

    setLspStatus(status: 'stopped' | 'starting' | 'running' | 'error'): void {
        this.lspStatus = status;
        this.update();
    }

    setDriftCount(count: number): void {
        this.lastDriftCount = count;
        this.update();
    }

    update(): void {
        const config = vscode.workspace.getConfiguration('photondrift');
        
        if (!config.get('ui.showStatusBar', true)) {
            this.hide();
            return;
        }

        // Base text
        let text = '$(organization) PhotonDrift';
        let tooltip = 'PhotonDrift ADR Manager';
        let backgroundColor: vscode.ThemeColor | undefined;

        // LSP status indicator
        switch (this.lspStatus) {
            case 'running':
                text += ' $(check)';
                tooltip += '\nLSP: Running';
                break;
            case 'starting':
                text += ' $(loading~spin)';
                tooltip += '\nLSP: Starting...';
                backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
                break;
            case 'error':
                text += ' $(error)';
                tooltip += '\nLSP: Error';
                backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
                break;
            case 'stopped':
                text += ' $(circle-slash)';
                tooltip += '\nLSP: Stopped';
                break;
        }

        // Drift count indicator
        if (this.lastDriftCount > 0) {
            text += ` $(warning) ${this.lastDriftCount}`;
            tooltip += `\nDrift Items: ${this.lastDriftCount}`;
            if (this.lastDriftCount > 5) {
                backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
            } else if (!backgroundColor) {
                backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
            }
        } else {
            text += ' $(check)';
            tooltip += '\nNo drift detected';
        }

        this.statusBarItem.text = text;
        this.statusBarItem.tooltip = tooltip;
        this.statusBarItem.backgroundColor = backgroundColor;
        
        this.show();
    }
}