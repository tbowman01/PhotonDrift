use clap::Args;
use std::path::PathBuf;

use crate::{config::Config, error::AdrscanError};
type Result<T> = std::result::Result<T, AdrscanError>;

#[derive(Args)]
pub struct InitCommand {
    /// ADR directory path (default: docs/adr)
    #[arg(short, long)]
    pub adr_dir: Option<PathBuf>,

    /// Force initialization even if directory exists
    #[arg(short, long)]
    pub force: bool,
}

impl InitCommand {
    pub fn execute(&self, config: &Config) -> Result<()> {
        log::info!("Initializing ADR directory and configuration...");

        // Determine ADR directory path
        let adr_dir = self.adr_dir.as_ref()
            .unwrap_or(&config.adr_dir);

        // Check if directory already exists
        if adr_dir.exists() && !self.force {
            if adr_dir.read_dir()?.next().is_some() {
                return Err(AdrscanError::ValidationError(
                    format!("Directory '{}' already exists and is not empty. Use --force to initialize anyway.", 
                           adr_dir.display())
                ));
            }
        }

        // Create ADR directory
        std::fs::create_dir_all(adr_dir)?;
        log::info!("Created ADR directory: {}", adr_dir.display());

        // Create configuration file
        self.create_config_file(adr_dir)?;

        // Create ADR conventions document (ADR-0000)
        self.create_conventions_adr(adr_dir)?;

        // Create sample ADR template
        self.create_sample_template(adr_dir)?;

        println!("✅ ADR directory initialized successfully at: {}", adr_dir.display());
        println!("📝 Created configuration file: .adrscan.yml");
        println!("📋 Created ADR conventions: {}/0000-record-architecture-decisions.md", adr_dir.display());
        println!("📄 Created sample template: {}/template.md", adr_dir.display());
        println!("\n🚀 Next steps:");
        println!("  1. Review the configuration in .adrscan.yml");
        println!("  2. Read the conventions in ADR-0000");
        println!("  3. Use the template to create your first ADR");
        println!("  4. Run 'adrscan inventory' to see your ADRs");

        Ok(())
    }

    fn create_config_file(&self, adr_dir: &Path) -> Result<()> {
        let config_path = std::path::Path::new(".adrscan.yml");
        
        if config_path.exists() && !self.force {
            log::info!("Configuration file already exists, skipping");
            return Ok(());
        }

        let config = Config {
            adr_dir: adr_dir.to_path_buf(),
            ..Config::default()
        };

        config.save(config_path)?;
        log::info!("Created configuration file: {}", config_path.display());
        
        Ok(())
    }

    fn create_conventions_adr(&self, adr_dir: &Path) -> Result<()> {
        let adr_path = adr_dir.join("0000-record-architecture-decisions.md");
        
        if adr_path.exists() && !self.force {
            log::info!("Conventions ADR already exists, skipping");
            return Ok(());
        }

        let content = self.get_conventions_template();
        std::fs::write(&adr_path, content)?;
        log::info!("Created conventions ADR: {}", adr_path.display());
        
        Ok(())
    }

    fn create_sample_template(&self, adr_dir: &Path) -> Result<()> {
        let template_path = adr_dir.join("template.md");
        
        if template_path.exists() && !self.force {
            log::info!("Template already exists, skipping");
            return Ok(());
        }

        let content = self.get_sample_template();
        std::fs::write(&template_path, content)?;
        log::info!("Created sample template: {}", template_path.display());
        
        Ok(())
    }

    fn get_conventions_template(&self) -> String {
        format!(r#"---
id: "0000"
title: "Record architecture decisions"
status: "accepted"
date: "{}"
deciders: []
tags: ["process", "governance"]
---

# Record architecture decisions

## Status

Accepted

## Context

We need to record the architectural decisions made on this project.

## Decision

We will use Architecture Decision Records, as [described by Michael Nygard](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions).

## Consequences

See Michael Nygard's article, linked above. For a lightweight ADR toolset, see Nat Pryce's [adr-tools](https://github.com/npryce/adr-tools).

## Compliance

This ADR establishes the process for documenting architectural decisions. All significant architectural decisions should be documented using this format.

Use `adrscan propose` to automatically generate new ADRs when architectural drift is detected, or create them manually following this template.
"#, chrono::Utc::now().format("%Y-%m-%d"))
    }

    fn get_sample_template(&self) -> String {
        format!(r#"---
id: "XXXX"
title: "Title of the decision"
status: "proposed"
date: "{}"
deciders: ["Name of decision maker"]
tags: ["tag1", "tag2"]
supersedes: []
relates_to: []
---

# Title of the decision

## Status

What is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?

## Context

What is the issue that we're seeing that is motivating this decision or change?

## Decision

What is the change that we're proposing or have agreed to implement?

## Consequences

What becomes easier or more difficult to do and any risks introduced by this change?

## Compliance

How will this decision be validated and monitored for compliance?

### Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2  
- [ ] Criterion 3

### Implementation Notes

Any specific implementation details or considerations.

## References

- [Link to relevant documentation]
- [Link to related ADRs]
"#, chrono::Utc::now().format("%Y-%m-%d"))
    }
}

use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_conventions_template() {
        let cmd = InitCommand {
            adr_dir: None,
            force: false,
        };
        
        let template = cmd.get_conventions_template();
        
        assert!(template.contains("---"));
        assert!(template.contains("id: \"0000\""));
        assert!(template.contains("title: \"Record architecture decisions\""));
        assert!(template.contains("status: \"accepted\""));
        assert!(template.contains("# Record architecture decisions"));
    }

    #[test]
    fn test_get_sample_template() {
        let cmd = InitCommand {
            adr_dir: None,
            force: false,
        };
        
        let template = cmd.get_sample_template();
        
        assert!(template.contains("---"));
        assert!(template.contains("id: \"XXXX\""));
        assert!(template.contains("title: \"Title of the decision\""));
        assert!(template.contains("status: \"proposed\""));
        assert!(template.contains("## Status"));
        assert!(template.contains("## Context"));
        assert!(template.contains("## Decision"));
        assert!(template.contains("## Consequences"));
    }

    #[test]
    fn test_create_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().join("docs/adr");
        std::fs::create_dir_all(&adr_dir).unwrap();
        
        let cmd = InitCommand {
            adr_dir: Some(adr_dir.clone()),
            force: false,
        };
        
        // Change to temp directory
        let old_cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let result = cmd.create_config_file(&adr_dir);
        
        // Restore directory
        std::env::set_current_dir(old_cwd).unwrap();
        
        assert!(result.is_ok());
        assert!(temp_dir.path().join(".adrscan.yml").exists());
    }

    #[test]
    fn test_create_conventions_adr() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().join("docs/adr");
        std::fs::create_dir_all(&adr_dir).unwrap();
        
        let cmd = InitCommand {
            adr_dir: Some(adr_dir.clone()),
            force: false,
        };
        
        let result = cmd.create_conventions_adr(&adr_dir);
        
        assert!(result.is_ok());
        assert!(adr_dir.join("0000-record-architecture-decisions.md").exists());
        
        let content = std::fs::read_to_string(
            adr_dir.join("0000-record-architecture-decisions.md")
        ).unwrap();
        assert!(content.contains("Record architecture decisions"));
    }

    #[test]
    fn test_create_sample_template() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().join("docs/adr");
        std::fs::create_dir_all(&adr_dir).unwrap();
        
        let cmd = InitCommand {
            adr_dir: Some(adr_dir.clone()),
            force: false,
        };
        
        let result = cmd.create_sample_template(&adr_dir);
        
        assert!(result.is_ok());
        assert!(adr_dir.join("template.md").exists());
        
        let content = std::fs::read_to_string(adr_dir.join("template.md")).unwrap();
        assert!(content.contains("Title of the decision"));
    }
}