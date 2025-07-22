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
exports.ConfigurationManager = void 0;
const vscode = __importStar(require("vscode"));
class ConfigurationManager {
    constructor() {
        this.config = vscode.workspace.getConfiguration('photondrift');
    }
    reload() {
        this.config = vscode.workspace.getConfiguration('photondrift');
    }
    // LSP Configuration
    isLspEnabled() {
        return this.config.get('lsp.enabled', true);
    }
    getLspServerPath() {
        return this.config.get('lsp.serverPath', 'adrscan-lsp');
    }
    getMaxDiagnostics() {
        return this.config.get('lsp.maxDiagnostics', 100);
    }
    // ADR Configuration
    getAdrDirectory() {
        return this.config.get('adr.directory', 'docs/adr');
    }
    getAdrTemplate() {
        return this.config.get('adr.template', 'madr');
    }
    // Drift Detection Configuration
    isDriftEnabled() {
        return this.config.get('drift.enabled', true);
    }
    isWatchModeEnabled() {
        return this.config.get('drift.watchMode', true);
    }
    // ML Configuration
    isMlEnabled() {
        return this.config.get('ml.enabled', true);
    }
    getMlModel() {
        return this.config.get('ml.model', 'Ensemble');
    }
    // UI Configuration
    shouldShowStatusBar() {
        return this.config.get('ui.showStatusBar', true);
    }
    getTheme() {
        return this.config.get('ui.theme', 'auto');
    }
    // Notifications Configuration
    areNotificationsEnabled() {
        return this.config.get('notifications.enabled', true);
    }
    // Analytics Configuration
    areAnalyticsEnabled() {
        return this.config.get('analytics.enabled', false);
    }
    // CLI Configuration
    getCliPath() {
        return this.config.get('cli.path', 'adrscan');
    }
}
exports.ConfigurationManager = ConfigurationManager;
//# sourceMappingURL=configuration.js.map