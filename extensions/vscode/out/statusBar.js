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
exports.StatusBarManager = void 0;
const vscode = __importStar(require("vscode"));
class StatusBarManager {
    constructor(context) {
        this.context = context;
        this.lspStatus = 'stopped';
        this.lastDriftCount = 0;
        this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 10);
        this.statusBarItem.command = 'photondrift.openDashboard';
        this.context.subscriptions.push(this.statusBarItem);
        this.update();
    }
    show() {
        const config = vscode.workspace.getConfiguration('photondrift');
        if (config.get('ui.showStatusBar', true)) {
            this.statusBarItem.show();
        }
    }
    hide() {
        this.statusBarItem.hide();
    }
    setLspStatus(status) {
        this.lspStatus = status;
        this.update();
    }
    setDriftCount(count) {
        this.lastDriftCount = count;
        this.update();
    }
    update() {
        const config = vscode.workspace.getConfiguration('photondrift');
        if (!config.get('ui.showStatusBar', true)) {
            this.hide();
            return;
        }
        // Base text
        let text = '$(organization) PhotonDrift';
        let tooltip = 'PhotonDrift ADR Manager';
        let backgroundColor;
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
            }
            else if (!backgroundColor) {
                backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
            }
        }
        else {
            text += ' $(check)';
            tooltip += '\nNo drift detected';
        }
        this.statusBarItem.text = text;
        this.statusBarItem.tooltip = tooltip;
        this.statusBarItem.backgroundColor = backgroundColor;
        this.show();
    }
}
exports.StatusBarManager = StatusBarManager;
//# sourceMappingURL=statusBar.js.map