use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_init_command_creates_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    let adr_dir = temp_dir.path().join("docs/adr");

    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init")
        .arg("--adr-dir")
        .arg(&adr_dir)
        .current_dir(&temp_dir);

    cmd.assert().success().stdout(predicate::str::contains(
        "ADR directory initialized successfully",
    ));

    // Check that files were created
    assert!(adr_dir.exists());
    assert!(adr_dir
        .join("0000-record-architecture-decisions.md")
        .exists());
    assert!(adr_dir.join("template.md").exists());
    assert!(temp_dir.path().join(".adrscan.yml").exists());
}

#[test]
fn test_init_command_with_existing_directory_fails() {
    let temp_dir = TempDir::new().unwrap();
    let adr_dir = temp_dir.path().join("docs/adr");

    // Create directory with existing content
    std::fs::create_dir_all(&adr_dir).unwrap();
    std::fs::write(adr_dir.join("existing.md"), "content").unwrap();

    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init")
        .arg("--adr-dir")
        .arg(&adr_dir)
        .current_dir(&temp_dir);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already exists and is not empty"));
}

#[test]
fn test_init_command_with_force_flag() {
    let temp_dir = TempDir::new().unwrap();
    let adr_dir = temp_dir.path().join("docs/adr");

    // Create directory with existing content
    std::fs::create_dir_all(&adr_dir).unwrap();
    std::fs::write(adr_dir.join("existing.md"), "content").unwrap();

    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init")
        .arg("--adr-dir")
        .arg(&adr_dir)
        .arg("--force")
        .current_dir(&temp_dir);

    cmd.assert().success().stdout(predicate::str::contains(
        "ADR directory initialized successfully",
    ));

    // Check that new files were created alongside existing
    assert!(adr_dir.join("existing.md").exists());
    assert!(adr_dir
        .join("0000-record-architecture-decisions.md")
        .exists());
    assert!(adr_dir.join("template.md").exists());
}

#[test]
fn test_init_command_creates_valid_adr_template() {
    let temp_dir = TempDir::new().unwrap();
    let adr_dir = temp_dir.path().join("docs/adr");

    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init")
        .arg("--adr-dir")
        .arg(&adr_dir)
        .current_dir(&temp_dir);

    cmd.assert().success();

    // Check ADR-0000 content
    let adr_content =
        std::fs::read_to_string(adr_dir.join("0000-record-architecture-decisions.md")).unwrap();

    assert!(adr_content.contains("---"));
    assert!(adr_content.contains("id: \"0000\""));
    assert!(adr_content.contains("title: \"Record architecture decisions\""));
    assert!(adr_content.contains("status: \"accepted\""));
    assert!(adr_content.contains("# Record architecture decisions"));
}

#[test]
fn test_init_command_creates_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    let adr_dir = temp_dir.path().join("docs/adr");

    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init")
        .arg("--adr-dir")
        .arg(&adr_dir)
        .current_dir(&temp_dir);

    cmd.assert().success();

    // Check config file content
    let config_content = std::fs::read_to_string(temp_dir.path().join(".adrscan.yml")).unwrap();

    assert!(config_content.contains("adr_dir:"));
    assert!(config_content.contains("include_patterns:"));
    assert!(config_content.contains("exclude_patterns:"));
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Architecture Decision Record"));
}

#[test]
fn test_init_help_command() {
    let mut cmd = Command::cargo_bin("adrscan").unwrap();
    cmd.arg("init").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize ADR directory"));
}
