Feature: IDE Integration and Language Server Protocol
  As a developer
  I want ADR management integrated into my IDE
  So that I can maintain architectural compliance during development

  Background:
    Given I have PhotonDrift installed
    And I have a project with ADRs configured

  Scenario: Language Server Protocol initialization
    Given I have PhotonDrift LSP server installed
    When I open a project with ADRs in my IDE
    Then the LSP server should start successfully within 5 seconds
    And register for Markdown file types (.md)
    And register for YAML configuration files (.yaml, .yml)
    And parse existing ADR configuration from .adrscan.yaml
    And establish drift detection patterns from configuration
    And send initialization complete notification to IDE

  Scenario: LSP server capabilities advertisement
    Given the LSP server is initializing
    When exchanging capabilities with the IDE
    Then it should advertise hover support
    And advertise completion support
    And advertise diagnostic support
    And advertise document symbol support
    And advertise workspace symbol support
    And advertise code action support

  Scenario: Real-time drift warnings in IDE
    Given I am editing a source file in my IDE
    And the change violates an existing ADR about database usage
    When I save the file
    Then I should see an inline diagnostic warning within 2 seconds
    And the warning should include specific ADR reference
    And receive explanation of the architectural violation
    And get suggestions for bringing code into compliance
    And have option to create new ADR for exception

  Scenario: ADR file syntax highlighting
    Given I am editing an ADR Markdown file
    When I type ADR frontmatter
    Then YAML frontmatter should be highlighted correctly
    And Markdown content should have proper syntax highlighting
    And ADR-specific fields should be visually distinct
    And status values should be color-coded by status type

  Scenario: Auto-completion in ADR files
    Given I am editing an ADR file with frontmatter
    When I type "status: " in the frontmatter
    Then I should see completion suggestions: "proposed", "accepted", "rejected", "deprecated", "superseded"
    And completion should be triggered automatically
    And suggestions should be ranked by frequency of use
    And custom completion items should be supported via configuration

  Scenario: Hover information for ADR references
    Given I have code that references an ADR by number
    When I hover over "ADR-0001" in a comment
    Then I should see a hover popup with ADR details
    And the popup should show ADR title and status
    And include a brief summary of the decision
    And provide a link to open the full ADR file

  Scenario: Quick actions for drift violations
    Given I have drift warnings in my code
    When I trigger code actions (Ctrl+. in VS Code)
    Then I should see "Create ADR for this change" action
    And "Mark as architectural exception" action
    And "View related ADRs" action
    And "Apply automated fix" action (when available)
    And all actions should be executable and provide feedback

  Scenario: Document symbols for ADR navigation
    Given I have an ADR file open
    When I request document symbols
    Then the IDE should show ADR sections in outline view
    And display "Status", "Context", "Decision", "Consequences" sections
    And allow navigation to specific sections
    And show nested subsections if present

  Scenario: Workspace symbols for ADR search
    Given I have multiple ADRs in my workspace
    When I search for workspace symbols with "database"
    Then I should see ADRs related to database decisions
    And results should include ADR numbers and titles
    And be ranked by relevance to search term
    And allow quick navigation to found ADRs

  Scenario: VS Code extension specific features
    Given I have the PhotonDrift VS Code extension installed
    When I work with ADRs in VS Code
    Then I should have ADR file templates in command palette
    And preview ADR rendering in side panel
    And navigate ADR relationships via graph view
    And access drift analysis reports in problems panel
    And manage ADR status workflows via custom commands

  Scenario: Performance requirements for IDE integration
    Given I have the LSP server running
    When working with large projects (10k+ files)
    Then LSP server memory usage should stay below 200MB
    And response time for hover should be <200ms
    And completion response should be <100ms
    And diagnostics should update within 2 seconds of file changes
    And IDE should remain responsive during analysis

  Scenario: Error handling in LSP server
    Given the LSP server encounters various error conditions
    When processing malformed ADR files
    Then it should log errors clearly to LSP client
    And continue processing other files
    And provide helpful error messages to developers
    And not crash or become unresponsive

  Scenario: Configuration hot-reloading
    Given the LSP server is running
    When I modify .adrscan.yaml configuration
    Then the server should detect the change within 5 seconds
    And reload configuration automatically
    And update diagnostic rules based on new config
    And notify IDE of capability changes if needed

  Scenario: Multi-workspace support
    Given I have multiple workspace folders open
    And each has its own ADR configuration
    When working across workspaces
    Then each workspace should have independent ADR management
    And cross-workspace ADR references should be supported
    And workspace-specific settings should be respected

  Scenario Outline: Cross-IDE compatibility testing
    Given I am using <ide>
    When I install PhotonDrift LSP support
    Then I should have basic LSP functionality
    And receive drift notifications appropriately
    And access ADR navigation features
    And completion should work as expected
    
    Examples:
      | ide            | expected_features                    |
      | VS Code        | full_extension_plus_lsp             |
      | IntelliJ IDEA  | lsp_with_custom_plugin              |
      | Vim/Neovim     | lsp_client_integration              |
      | Emacs          | lsp_mode_integration                |
      | Sublime Text   | lsp_plugin_integration              |

  Scenario: Drift detection in real-time editing
    Given I have real-time drift detection enabled
    When I type code that introduces architectural violations
    Then violations should be detected as I type
    And warnings should appear with minimal delay (<1 second)
    And warnings should disappear when violations are resolved
    And performance should not impact typing responsiveness

  Scenario: ADR template insertion
    Given I want to create a new ADR
    When I use the "Insert ADR Template" command
    Then I should see a list of available templates
    And be able to select template by category (technical, business, etc.)
    And template should be inserted with placeholder values
    And placeholders should be tab-navigable for easy completion

  Scenario: Batch drift analysis reporting
    Given I have made multiple changes across many files
    When I trigger "Analyze All Changes" command
    Then the IDE should collect all drift violations
    And display consolidated report in problems panel
    And group violations by ADR and severity
    And provide bulk actions for multiple violations

  Scenario: Integration with version control
    Given I am working with Git-tracked files
    When analyzing drift in modified files only
    Then the system should focus on changed lines
    And compare against previous commit state
    And highlight new violations introduced in current changes
    And ignore existing violations not modified in current session

  Scenario: Accessibility support
    Given I use screen readers or other assistive technologies
    When working with PhotonDrift in my IDE
    Then all UI elements should have proper ARIA labels
    And keyboard navigation should be fully supported
    And visual indicators should have text alternatives
    And error messages should be accessible to screen readers