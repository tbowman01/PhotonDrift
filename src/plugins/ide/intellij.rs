//! IntelliJ IDEA specific plugin implementation

use crate::plugins::ide::{CommonIDEFeatures, IDECapabilities, TextSelection};
use crate::plugins::{
    ArgumentType, CommandArgument, IDEAction, IDECommand, IDEConfig, IDEEvent,
    IDEIntegrationPlugin, IDEResponse, IDEType, MessageLevel, Plugin, PluginCapability,
    PluginContext, PluginMetadata, PluginResponse,
};
use crate::{AdrscanError, Result};
use chrono::Utc;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// IntelliJ IDEA integration plugin
#[derive(Debug)]
pub struct IntelliJPlugin {
    metadata: PluginMetadata,
    config: Option<IDEConfig>,
    capabilities: IDECapabilities,
}

/// IntelliJ plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJConfig {
    pub project_path: Option<std::path::PathBuf>,
    pub plugin_id: String,
    pub enable_inspections: bool,
    pub enable_quick_fixes: bool,
    pub enable_refactoring: bool,
    pub auto_analysis_interval: u32,
}

/// IntelliJ plugin descriptor (plugin.xml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJPluginDescriptor {
    pub id: String,
    pub name: String,
    pub version: String,
    pub vendor: IntelliJVendor,
    pub description: String,
    pub change_notes: String,
    pub idea_version: IntelliJIdeaVersion,
    pub depends: Vec<String>,
    pub extensions: IntelliJExtensions,
}

/// IntelliJ vendor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJVendor {
    pub name: String,
    pub email: String,
    pub url: String,
}

/// IntelliJ IDEA version requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJIdeaVersion {
    pub since_build: String,
    pub until_build: String,
}

/// IntelliJ extensions configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJExtensions {
    pub actions: Vec<IntelliJAction>,
    pub inspections: Vec<IntelliJInspection>,
    pub file_types: Vec<IntelliJFileType>,
    pub tool_windows: Vec<IntelliJToolWindow>,
}

/// IntelliJ action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJAction {
    pub id: String,
    pub class: String,
    pub text: String,
    pub description: String,
    pub group_id: String,
}

/// IntelliJ inspection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJInspection {
    pub short_name: String,
    pub display_name: String,
    pub group_path: String,
    pub group_key: String,
    pub group_bundle: String,
    pub level: String,
    pub implementation_class: String,
}

/// IntelliJ file type configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJFileType {
    pub name: String,
    pub implementation_class: String,
    pub field_name: String,
    pub language: String,
    pub extensions: String,
}

/// IntelliJ tool window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelliJToolWindow {
    pub id: String,
    pub factory_class: String,
    pub anchor: String,
    pub secondary: bool,
}

impl IntelliJPlugin {
    /// Create a new IntelliJ plugin
    pub fn new() -> Self {
        let metadata = PluginMetadata {
            id: "intellij-integration".to_string(),
            name: "PhotonDrift IntelliJ Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "IntelliJ IDEA integration for PhotonDrift ADR analysis".to_string(),
            author: "PhotonDrift Team".to_string(),
            license: "MIT".to_string(),
            homepage: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            repository: Some("https://github.com/tbowman01/PhotonDrift".to_string()),
            keywords: vec![
                "intellij".to_string(),
                "adr".to_string(),
                "photondrift".to_string(),
            ],
            api_version: crate::plugins::PLUGIN_API_VERSION.to_string(),
            min_adrscan_version: "0.2.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let capabilities = IDECapabilities {
            supports_lsp: false,
            supports_debugging: true,
            supports_extensions: true,
            supports_git_integration: true,
            supports_terminal: true,
            supported_languages: vec![
                "markdown".to_string(),
                "yaml".to_string(),
                "json".to_string(),
                "java".to_string(),
                "kotlin".to_string(),
            ],
        };

        Self {
            metadata,
            config: None,
            capabilities,
        }
    }

    /// Generate IntelliJ plugin descriptor
    pub fn generate_plugin_descriptor(&self) -> IntelliJPluginDescriptor {
        IntelliJPluginDescriptor {
            id: "com.photondrift.intellij".to_string(),
            name: "PhotonDrift ADR Analyzer".to_string(),
            version: "1.0.0".to_string(),
            vendor: IntelliJVendor {
                name: "PhotonDrift Team".to_string(),
                email: "support@photondrift.io".to_string(),
                url: "https://github.com/tbowman01/PhotonDrift".to_string(),
            },
            description: "Architecture Decision Record analysis and management for IntelliJ IDEA"
                .to_string(),
            change_notes: "Initial release with ADR analysis capabilities".to_string(),
            idea_version: IntelliJIdeaVersion {
                since_build: "203".to_string(),
                until_build: "233.*".to_string(),
            },
            depends: vec![
                "com.intellij.modules.platform".to_string(),
                "com.intellij.modules.lang".to_string(),
            ],
            extensions: self.generate_extensions(),
        }
    }

    /// Generate Java plugin implementation
    pub fn generate_plugin_java_code(&self) -> HashMap<String, String> {
        let mut files = HashMap::new();

        // Main plugin class
        files.insert(
            "PhotonDriftPlugin.java".to_string(),
            r#"
package com.photondrift.intellij;

import com.intellij.openapi.components.ApplicationComponent;
import com.intellij.openapi.diagnostic.Logger;

public class PhotonDriftPlugin implements ApplicationComponent {
    private static final Logger LOG = Logger.getInstance(PhotonDriftPlugin.class);

    @Override
    public void initComponent() {
        LOG.info("PhotonDrift plugin initialized");
    }

    @Override
    public void disposeComponent() {
        LOG.info("PhotonDrift plugin disposed");
    }

    @Override
    public String getComponentName() {
        return "PhotonDriftPlugin";
    }
}
"#
            .to_string(),
        );

        // ADR Analysis Action
        files.insert(
            "AnalyzeADRAction.java".to_string(),
            r#"
package com.photondrift.intellij.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.vfs.VirtualFile;
import com.photondrift.intellij.services.PhotonDriftService;

public class AnalyzeADRAction extends AnAction {

    @Override
    public void actionPerformed(AnActionEvent e) {
        Project project = e.getProject();
        VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
        
        if (project == null || file == null) {
            Messages.showErrorDialog("No file selected", "PhotonDrift Error");
            return;
        }

        if (!file.getName().endsWith(".md")) {
            Messages.showWarningDialog("PhotonDrift analysis is optimized for Markdown files", 
                                     "PhotonDrift Warning");
            return;
        }

        PhotonDriftService service = project.getService(PhotonDriftService.class);
        try {
            service.analyzeFile(file);
            Messages.showInfoMessage("ADR analysis completed successfully", "PhotonDrift");
        } catch (Exception ex) {
            Messages.showErrorDialog("Analysis failed: " + ex.getMessage(), "PhotonDrift Error");
        }
    }

    @Override
    public void update(AnActionEvent e) {
        VirtualFile file = e.getData(CommonDataKeys.VIRTUAL_FILE);
        e.getPresentation().setEnabledAndVisible(file != null && file.getName().endsWith(".md"));
    }
}
"#
            .to_string(),
        );

        // PhotonDrift Service
        files.insert(
            "PhotonDriftService.java".to_string(),
            r#"
package com.photondrift.intellij.services;

import com.intellij.execution.ExecutionException;
import com.intellij.execution.configurations.GeneralCommandLine;
import com.intellij.execution.process.ProcessOutput;
import com.intellij.execution.util.ExecUtil;
import com.intellij.openapi.components.Service;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.google.gson.Gson;
import com.google.gson.JsonObject;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

@Service
public final class PhotonDriftService {
    private static final Logger LOG = Logger.getInstance(PhotonDriftService.class);
    private final Project project;
    private final Gson gson = new Gson();

    public PhotonDriftService(Project project) {
        this.project = project;
    }

    public void analyzeFile(VirtualFile file) throws ExecutionException {
        GeneralCommandLine commandLine = new GeneralCommandLine();
        commandLine.setExePath("adrscan");
        commandLine.addParameter("analyze");
        commandLine.addParameter(file.getPath());
        commandLine.addParameter("--format");
        commandLine.addParameter("json");
        commandLine.setWorkDirectory(new File(project.getBasePath()));

        ProcessOutput output = ExecUtil.execAndGetOutput(commandLine, 30000);
        
        if (output.getExitCode() != 0) {
            throw new ExecutionException("PhotonDrift analysis failed: " + output.getStderr());
        }

        // Process analysis results
        JsonObject results = gson.fromJson(output.getStdout(), JsonObject.class);
        processAnalysisResults(file, results);
    }

    public List<VirtualFile> scanProject() throws ExecutionException {
        GeneralCommandLine commandLine = new GeneralCommandLine();
        commandLine.setExePath("adrscan");
        commandLine.addParameter("scan");
        commandLine.addParameter(".");
        commandLine.addParameter("--format");
        commandLine.addParameter("json");
        commandLine.setWorkDirectory(new File(project.getBasePath()));

        ProcessOutput output = ExecUtil.execAndGetOutput(commandLine, 60000);
        
        if (output.getExitCode() != 0) {
            throw new ExecutionException("PhotonDrift scan failed: " + output.getStderr());
        }

        JsonObject results = gson.fromJson(output.getStdout(), JsonObject.class);
        return processScanResults(results);
    }

    private void processAnalysisResults(VirtualFile file, JsonObject results) {
        // Process and display analysis results
        LOG.info("Analysis completed for " + file.getName());
    }

    private List<VirtualFile> processScanResults(JsonObject results) {
        // Process scan results and return list of ADR files
        return new ArrayList<>();
    }
}
"#
            .to_string(),
        );

        // ADR Inspection
        files.insert(
            "ADRInspection.java".to_string(),
            r#"
package com.photondrift.intellij.inspections;

import com.intellij.codeInspection.*;
import com.intellij.openapi.project.Project;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import org.jetbrains.annotations.NotNull;

public class ADRInspection extends LocalInspectionTool {

    @Override
    public ProblemDescriptor[] checkFile(@NotNull PsiFile file, 
                                       @NotNull InspectionManager manager,
                                       boolean isOnTheFly) {
        if (!file.getName().endsWith(".md")) {
            return ProblemDescriptor.EMPTY_ARRAY;
        }

        // Perform ADR-specific inspections
        return performADRInspection(file, manager, isOnTheFly);
    }

    private ProblemDescriptor[] performADRInspection(PsiFile file, 
                                                   InspectionManager manager,
                                                   boolean isOnTheFly) {
        // TODO: Implement actual ADR inspection logic
        return ProblemDescriptor.EMPTY_ARRAY;
    }

    @NotNull
    @Override
    public String getShortName() {
        return "ADRInspection";
    }

    @NotNull
    @Override
    public String getDisplayName() {
        return "ADR Structure Inspection";
    }

    @NotNull
    @Override
    public String getGroupDisplayName() {
        return "PhotonDrift";
    }
}
"#
            .to_string(),
        );

        // Tool Window
        files.insert(
            "PhotonDriftToolWindow.java".to_string(),
            r#"
package com.photondrift.intellij.toolwindow;

import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowFactory;
import com.intellij.ui.content.Content;
import com.intellij.ui.content.ContentFactory;

import javax.swing.*;
import java.awt.*;

public class PhotonDriftToolWindow implements ToolWindowFactory {

    @Override
    public void createToolWindowContent(Project project, ToolWindow toolWindow) {
        PhotonDriftToolWindowContent content = new PhotonDriftToolWindowContent(project);
        Content contentTab = ContentFactory.SERVICE.getInstance()
            .createContent(content.getContentPanel(), "", false);
        toolWindow.getContentManager().addContent(contentTab);
    }

    public static class PhotonDriftToolWindowContent {
        private final JPanel contentPanel;

        public PhotonDriftToolWindowContent(Project project) {
            contentPanel = new JPanel(new BorderLayout());
            
            JLabel titleLabel = new JLabel("PhotonDrift ADR Analysis");
            titleLabel.setHorizontalAlignment(SwingConstants.CENTER);
            contentPanel.add(titleLabel, BorderLayout.NORTH);

            JTextArea resultsArea = new JTextArea();
            resultsArea.setEditable(false);
            resultsArea.setText("No analysis results yet. Select an ADR file and run analysis.");
            
            JScrollPane scrollPane = new JScrollPane(resultsArea);
            contentPanel.add(scrollPane, BorderLayout.CENTER);

            JPanel buttonPanel = new JPanel(new FlowLayout());
            JButton analyzeButton = new JButton("Analyze Current File");
            JButton scanButton = new JButton("Scan Project");
            
            buttonPanel.add(analyzeButton);
            buttonPanel.add(scanButton);
            contentPanel.add(buttonPanel, BorderLayout.SOUTH);
        }

        public JPanel getContentPanel() {
            return contentPanel;
        }
    }
}
"#
            .to_string(),
        );

        files
    }

    /// Generate build.gradle file
    pub fn generate_build_gradle(&self) -> String {
        r#"
plugins {
    id 'java'
    id 'org.jetbrains.intellij' version '1.13.3'
}

group 'com.photondrift'
version '1.0.0'

repositories {
    mavenCentral()
}

dependencies {
    implementation 'com.google.code.gson:gson:2.10.1'
}

intellij {
    version = '2023.1'
    type = 'IC' // IntelliJ IDEA Community Edition
    
    plugins = ['com.intellij.java']
}

patchPluginXml {
    sinceBuild = '203'
    untilBuild = '233.*'
    
    changeNotes = """
        <h3>1.0.0</h3>
        <ul>
            <li>Initial release</li>
            <li>ADR file analysis</li>
            <li>Project-wide ADR scanning</li>
            <li>IntelliJ IDEA integration</li>
        </ul>
    """
}

signPlugin {
    certificateChain = System.getenv("CERTIFICATE_CHAIN")
    privateKey = System.getenv("PRIVATE_KEY")
    password = System.getenv("PRIVATE_KEY_PASSWORD")
}

publishPlugin {
    token = System.getenv("PUBLISH_TOKEN")
}
"#
        .to_string()
    }

    fn generate_extensions(&self) -> IntelliJExtensions {
        IntelliJExtensions {
            actions: vec![
                IntelliJAction {
                    id: "PhotonDrift.AnalyzeFile".to_string(),
                    class: "com.photondrift.intellij.actions.AnalyzeADRAction".to_string(),
                    text: "Analyze ADR File".to_string(),
                    description: "Analyze the current ADR file with PhotonDrift".to_string(),
                    group_id: "EditorPopupMenu".to_string(),
                },
                IntelliJAction {
                    id: "PhotonDrift.ScanProject".to_string(),
                    class: "com.photondrift.intellij.actions.ScanProjectAction".to_string(),
                    text: "Scan Project for ADRs".to_string(),
                    description: "Scan the entire project for ADR files".to_string(),
                    group_id: "ToolsMenu".to_string(),
                },
            ],
            inspections: vec![IntelliJInspection {
                short_name: "ADRInspection".to_string(),
                display_name: "ADR Structure Inspection".to_string(),
                group_path: "PhotonDrift".to_string(),
                group_key: "photondrift.inspections".to_string(),
                group_bundle: "messages.PhotonDriftBundle".to_string(),
                level: "WARNING".to_string(),
                implementation_class: "com.photondrift.intellij.inspections.ADRInspection"
                    .to_string(),
            }],
            file_types: vec![IntelliJFileType {
                name: "ADR_MARKDOWN".to_string(),
                implementation_class: "com.photondrift.intellij.filetypes.ADRFileType".to_string(),
                field_name: "INSTANCE".to_string(),
                language: "Markdown".to_string(),
                extensions: "adr.md".to_string(),
            }],
            tool_windows: vec![IntelliJToolWindow {
                id: "PhotonDrift".to_string(),
                factory_class: "com.photondrift.intellij.toolwindow.PhotonDriftToolWindow"
                    .to_string(),
                anchor: "bottom".to_string(),
                secondary: false,
            }],
        }
    }
}

impl Plugin for IntelliJPlugin {
    fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
        info!("Initializing IntelliJ plugin");

        self.config =
            Some(crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::IntelliJ));

        debug!("IntelliJ plugin initialized successfully");
        Ok(())
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![
            PluginCapability::IDEIntegration,
            PluginCapability::FileWatcher,
            PluginCapability::CommandExtension,
        ]
    }

    fn execute(&self, command: &str, params: &HashMap<String, String>) -> Result<PluginResponse> {
        debug!("Executing IntelliJ command: {}", command);

        match command {
            "generate_plugin" => {
                let java_files = self.generate_plugin_java_code();
                let build_gradle = self.generate_build_gradle();
                let descriptor = self.generate_plugin_descriptor();

                Ok(PluginResponse {
                    success: true,
                    data: Some(serde_json::json!({
                        "java_files": java_files,
                        "build_gradle": build_gradle,
                        "plugin_descriptor": descriptor
                    })),
                    message: Some("IntelliJ plugin files generated successfully".to_string()),
                    warnings: vec![],
                    errors: vec![],
                })
            }
            "analyze_file" => {
                let file_path = params.get("file_path").unwrap_or("");
                Ok(PluginResponse {
                    success: true,
                    data: Some(serde_json::json!({
                        "analyzed_file": file_path,
                        "inspections": []
                    })),
                    message: Some(format!("Analyzed file: {}", file_path)),
                    warnings: vec![],
                    errors: vec![],
                })
            }
            _ => Ok(PluginResponse {
                success: false,
                data: None,
                message: Some(format!("Unknown command: {}", command)),
                warnings: vec![],
                errors: vec![format!(
                    "Command '{}' not supported by IntelliJ plugin",
                    command
                )],
            }),
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down IntelliJ plugin");
        Ok(())
    }
}

impl IDEIntegrationPlugin for IntelliJPlugin {
    fn ide_type(&self) -> IDEType {
        IDEType::IntelliJ
    }

    fn setup_ide_integration(&self, config: &IDEConfig) -> Result<()> {
        info!("Setting up IntelliJ integration with config: {:?}", config);
        Ok(())
    }

    fn handle_ide_event(&self, event: &IDEEvent) -> Result<IDEResponse> {
        debug!("Handling IntelliJ event: {:?}", event);

        match event {
            IDEEvent::FileOpened { path } => Ok(IDEResponse {
                handled: true,
                actions: vec![IDEAction::ShowMessage {
                    level: MessageLevel::Info,
                    message: format!("PhotonDrift: File opened {}", path.display()),
                }],
                diagnostics: vec![],
            }),
            IDEEvent::ProjectOpened { root } => Ok(IDEResponse {
                handled: true,
                actions: vec![IDEAction::ShowMessage {
                    level: MessageLevel::Info,
                    message: "PhotonDrift plugin activated for project".to_string(),
                }],
                diagnostics: vec![],
            }),
            _ => Ok(IDEResponse {
                handled: false,
                actions: vec![],
                diagnostics: vec![],
            }),
        }
    }

    fn get_ide_config(&self) -> IDEConfig {
        self.config.clone().unwrap_or_else(|| {
            crate::plugins::ide::IDEPluginFactory::get_recommended_config(IDEType::IntelliJ)
        })
    }

    fn register_commands(&self) -> Vec<IDECommand> {
        vec![
            IDECommand {
                id: "photondrift.analyzeFile".to_string(),
                title: "Analyze ADR File".to_string(),
                category: "PhotonDrift".to_string(),
                description: Some("Analyze the current ADR file for patterns".to_string()),
                arguments: vec![CommandArgument {
                    name: "file_path".to_string(),
                    arg_type: ArgumentType::File,
                    description: Some("Path to the ADR file".to_string()),
                    required: false,
                    default_value: None,
                }],
            },
            IDECommand {
                id: "photondrift.scanProject".to_string(),
                title: "Scan Project".to_string(),
                category: "PhotonDrift".to_string(),
                description: Some("Scan the entire project for ADR files".to_string()),
                arguments: vec![],
            },
        ]
    }
}

impl CommonIDEFeatures for IntelliJPlugin {
    fn show_notification(&self, message: &str, level: MessageLevel) -> Result<()> {
        debug!("IntelliJ notification: {} (level: {:?})", message, level);
        Ok(())
    }

    fn open_file(&self, path: &Path, line: Option<u32>) -> Result<()> {
        debug!("IntelliJ open file: {} at line {:?}", path.display(), line);
        Ok(())
    }

    fn insert_text(&self, path: &Path, line: u32, column: u32, text: &str) -> Result<()> {
        debug!(
            "IntelliJ insert text at {}:{}:{}: {}",
            path.display(),
            line,
            column,
            text
        );
        Ok(())
    }

    fn get_selection(&self) -> Result<Option<TextSelection>> {
        Ok(None)
    }

    fn set_status_message(&self, message: &str) -> Result<()> {
        debug!("IntelliJ status message: {}", message);
        Ok(())
    }
}

impl Default for IntelliJPlugin {
    fn default() -> Self {
        Self::new()
    }
}
