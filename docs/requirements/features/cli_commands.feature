Feature: CLI Command System
  As a developer
  I want a comprehensive command-line interface
  So that I can manage ADRs and detect drift efficiently

  Background:
    Given I have PhotonDrift installed
    And I am in a project directory

  Scenario: Initialize ADR structure
    When I run "adrscan init ./docs/adr"
    Then the system should create an ADR directory structure
    And create a first ADR "Record Architecture Decisions"
    And generate a configuration file ".adrscan.yaml"
    And provide usage instructions in README
    And the exit code should be 0

  Scenario: Initialize with custom template
    Given I have a custom ADR template at "./templates/custom.md"
    When I run "adrscan init ./docs/adr --template ./templates/custom.md"
    Then the system should use the custom template
    And the first ADR should follow the custom format
    And the configuration should reference the custom template

  Scenario: Inventory existing ADRs
    Given I have ADRs in "./docs/adr"
    When I run "adrscan inventory"
    Then the system should scan all ADR files
    And parse frontmatter metadata
    And display a structured list of ADRs
    And show status, dates, and relationships
    And the output should be sorted by ADR number

  Scenario: Inventory with JSON output
    Given I have 5 ADRs in "./docs/adr"
    When I run "adrscan inventory --format json"
    Then the output should be valid JSON
    And contain exactly 5 ADR entries
    And each entry should have required fields: id, title, status, date
    And the JSON should be pretty-printed

  Scenario: Detect architectural drift
    Given I have established ADRs
    And I have a codebase with architectural violations
    When I run "adrscan diff"
    Then the system should compare current code against ADRs
    And identify violations with confidence scores
    And categorize drift by type and severity
    And provide actionable recommendations
    And display summary statistics

  Scenario: Detect drift with ML enhancement
    Given I have ML features enabled in configuration
    And I have sufficient training data
    When I run "adrscan diff --ml-enhanced"
    Then the system should use ML models for detection
    And provide confidence scores between 0.0 and 1.0
    And include explanations for each detection
    And filter results based on confidence threshold

  Scenario: Generate ADR proposals
    Given drift has been detected
    When I run "adrscan propose"
    Then the system should generate draft ADRs
    And include context about detected changes
    And suggest appropriate ADR templates
    And provide implementation guidance
    And save drafts to the ADR directory

  Scenario: Generate proposals with dry-run
    Given drift has been detected
    When I run "adrscan propose --dry-run"
    Then the system should generate draft ADRs
    But not save them to disk
    And display what would be created
    And show file paths where ADRs would be saved

  Scenario: Create ADR index
    Given I have multiple ADRs with different statuses
    When I run "adrscan index"
    Then the system should generate a comprehensive index
    And organize ADRs by category and status
    And create cross-references between related ADRs
    And maintain searchable metadata
    And update existing index files

  Scenario Outline: Output format flexibility
    Given I have drift detection results
    When I run "adrscan diff --format <format>"
    Then the output should be in the specified format
    And maintain consistent data structure
    And be parseable by appropriate tools
    
    Examples:
      | format  | expected_structure |
      | console | human-readable     |
      | json    | machine-parseable  |
      | yaml    | configuration-like |
      | csv     | spreadsheet-ready  |

  Scenario: Configuration file override
    Given I have a custom configuration file at "./custom-config.yaml"
    When I run "adrscan diff --config ./custom-config.yaml"
    Then the system should use the custom configuration
    And ignore default configuration files
    And respect all settings from the custom file

  Scenario: Verbose logging mode
    Given I want detailed logging information
    When I run "adrscan diff --verbose"
    Then the system should display debug-level logs
    And show processing steps
    And include timing information
    And display file processing details

  Scenario: Help documentation
    When I run "adrscan --help"
    Then the system should display usage information
    And list all available commands
    And show global options
    And provide examples of common usage

  Scenario: Command-specific help
    When I run "adrscan diff --help"
    Then the system should display help for the diff command
    And show all available options
    And provide usage examples
    And explain output formats

  Scenario: Version information
    When I run "adrscan --version"
    Then the system should display the current version
    And the format should be semantic versioning
    And include build information if available

  Scenario: Invalid command handling
    When I run "adrscan invalid-command"
    Then the system should display an error message
    And suggest similar valid commands
    And show basic usage information
    And exit with non-zero code

  Scenario: Configuration validation
    Given I have an invalid configuration file
    When I run any adrscan command
    Then the system should detect configuration errors
    And display clear error messages
    And suggest corrections
    And exit with appropriate error code

  Scenario: Large project handling
    Given I have a project with 10,000+ files
    When I run "adrscan diff"
    Then the system should process files efficiently
    And display progress indicators
    And complete within reasonable time limits
    And not exceed memory constraints

  Scenario: Concurrent execution safety
    Given I run "adrscan diff" in one terminal
    When I run "adrscan inventory" in another terminal
    Then both commands should execute successfully
    And not interfere with each other
    And maintain data consistency
    And use appropriate file locking if needed