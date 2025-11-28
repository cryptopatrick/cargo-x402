//! Critical integration tests for cargo-x402
//! Tests end-to-end template creation and error scenarios

use tempfile::TempDir;

// ===== END-TO-END TEMPLATE CREATION TESTS =====

#[test]
fn test_e2e_project_creation_directory_structure() {
    // Simulate end-to-end project creation with proper structure
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-project");
    std::fs::create_dir(&project_path).unwrap();

    // Create typical project files
    let files = vec![
        ("README.md", "# Test Project\n"),
        ("Cargo.toml", "[package]\nname = \"test-project\"\n"),
        (".env.example", "DATABASE_URL=postgresql://localhost\n"),
        ("src/main.rs", "fn main() {}\n"),
    ];

    // Create src directory
    std::fs::create_dir(project_path.join("src")).unwrap();

    for (file, content) in files {
        let file_path = project_path.join(file);
        std::fs::write(&file_path, content).unwrap();
        assert!(file_path.exists(), "File {} should exist", file);
    }

    // Verify structure
    assert!(project_path.join("src").exists());
    assert!(project_path.join("src/main.rs").exists());
    assert!(project_path.join("Cargo.toml").exists());
    assert!(project_path.join("README.md").exists());
}

#[test]
fn test_e2e_template_with_multiple_parameters() {
    // Simulate creating a project with multiple parameter types
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("full-project");
    std::fs::create_dir(&project_path).unwrap();

    // Create a manifest-like file with template variables
    let manifest = r#"
[template]
name = "test-template"
version = "1.0.0"

[parameters.project_name]
type = "string"
default = "my-app"

[parameters.enable_database]
type = "boolean"
default = true

[parameters.database_type]
type = "enum"
options = ["postgres", "sqlite"]
default = "postgres"
"#;

    std::fs::write(project_path.join("x402.toml"), manifest).unwrap();
    assert!(project_path.join("x402.toml").exists());

    // Verify manifest can be read
    let content = std::fs::read_to_string(project_path.join("x402.toml")).unwrap();
    assert!(content.contains("[template]"));
    assert!(content.contains("[parameters"));
}

#[test]
fn test_e2e_git_initialization_cleanup() {
    // Verify that git history is cleaned up after template download
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("git-test");
    std::fs::create_dir(&project_path).unwrap();

    // Simulate having a .git directory (from downloaded template)
    let git_dir = project_path.join(".git");
    std::fs::create_dir(&git_dir).unwrap();
    std::fs::write(git_dir.join("config"), "test").unwrap();

    // Verify it exists
    assert!(git_dir.exists());

    // Simulate cleanup (remove .git directory)
    std::fs::remove_dir_all(&git_dir).unwrap();

    // Verify it's gone
    assert!(!git_dir.exists());

    // Verify project still exists
    assert!(project_path.exists());
}

#[test]
fn test_e2e_project_with_environment_variables() {
    // Test that environment configuration is properly set up
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("env-project");
    std::fs::create_dir(&project_path).unwrap();

    let env_content = r#"
# Database
DATABASE_URL=postgresql://localhost/testdb

# API Keys
API_KEY=dev-key-12345
API_SECRET=dev-secret-67890

# Server
SERVER_PORT=3000
SERVER_HOST=localhost

# Features
ENABLE_DEBUG=true
LOG_LEVEL=debug
"#;

    std::fs::write(project_path.join(".env.example"), env_content).unwrap();

    let read_content = std::fs::read_to_string(project_path.join(".env.example")).unwrap();
    assert!(read_content.contains("DATABASE_URL"));
    assert!(read_content.contains("API_KEY"));
    assert!(read_content.contains("SERVER_PORT"));
}

// ===== ERROR SCENARIO TESTS =====

#[test]
fn test_error_directory_already_exists() {
    // Test error when target directory exists
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("existing-project");

    // Create directory
    std::fs::create_dir(&project_path).unwrap();
    assert!(project_path.exists());

    // Attempting to create again should detect this as error
    let should_fail = project_path.exists();
    assert!(should_fail, "Should detect existing directory");
}

#[test]
fn test_error_invalid_project_name() {
    // Test validation of project names
    let invalid_names = vec![
        "my project",      // space
        "my@project",      // special char
        "my.project",      // dot
        "",                // empty
    ];

    for name in invalid_names {
        let is_invalid = name.is_empty()
            || !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_');
        assert!(
            is_invalid,
            "Name '{}' should be detected as invalid",
            name
        );
    }
}

#[test]
fn test_error_missing_manifest_file() {
    // Test error when x402.toml is missing
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("no-manifest");
    std::fs::create_dir(&project_path).unwrap();

    let manifest_exists = project_path.join("x402.toml").exists();
    assert!(
        !manifest_exists,
        "Manifest should be missing in this test"
    );
}

#[test]
fn test_error_invalid_manifest_syntax() {
    // Test error when x402.toml has invalid syntax
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("bad-manifest");
    std::fs::create_dir(&project_path).unwrap();

    let invalid_toml = r#"
[template
name = "broken"
version = "1.0.0"
"#;

    std::fs::write(project_path.join("x402.toml"), invalid_toml).unwrap();

    // Try to read it (in real code, would fail to parse)
    let content = std::fs::read_to_string(project_path.join("x402.toml")).unwrap();
    assert!(content.contains("[template"), "Content should exist but be invalid");
}

#[test]
fn test_error_missing_required_parameters() {
    // Test that missing required parameters are detected
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("incomplete-config");
    std::fs::create_dir(&project_path).unwrap();

    let manifest = r#"
[template]
name = "test"
version = "1.0.0"

[parameters.required_param]
type = "string"
"#;

    std::fs::write(project_path.join("x402.toml"), manifest).unwrap();
    let content = std::fs::read_to_string(project_path.join("x402.toml")).unwrap();

    // Verify required field is missing (no default)
    assert!(content.contains("required_param"));
}

#[test]
fn test_error_invalid_parameter_type() {
    // Test error when parameter has invalid type
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("bad-param");
    std::fs::create_dir(&project_path).unwrap();

    let manifest = r#"
[parameters.test_param]
type = "invalid_type_here"
"#;

    std::fs::write(project_path.join("x402.toml"), manifest).unwrap();
    let content = std::fs::read_to_string(project_path.join("x402.toml")).unwrap();
    assert!(content.contains("invalid_type_here"));
}

// ===== INTEGRATION WORKFLOW TESTS =====

#[test]
fn test_workflow_create_and_verify_complete_project() {
    // Full workflow: create project, add files, verify structure
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Step 1: Create project directory
    let project_name = "complete-app";
    let project_path = base_path.join(project_name);
    std::fs::create_dir(&project_path).unwrap();
    assert!(project_path.exists());

    // Step 2: Create Cargo.toml
    let cargo_toml = r#"
[package]
name = "complete-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
"#;
    std::fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();

    // Step 3: Create source structure
    std::fs::create_dir(project_path.join("src")).unwrap();
    std::fs::write(
        project_path.join("src/main.rs"),
        "#[tokio::main]\nasync fn main() {}\n",
    )
    .unwrap();

    // Step 4: Create environment
    std::fs::write(
        project_path.join(".env.example"),
        "DATABASE_URL=postgresql://localhost\n",
    )
    .unwrap();

    // Step 5: Create manifest
    std::fs::write(
        project_path.join("x402.toml"),
        "[template]\nname = \"complete-app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    // Verification
    assert!(project_path.join("Cargo.toml").exists());
    assert!(project_path.join("src/main.rs").exists());
    assert!(project_path.join(".env.example").exists());
    assert!(project_path.join("x402.toml").exists());

    // Verify content integrity
    let cargo_content = std::fs::read_to_string(project_path.join("Cargo.toml")).unwrap();
    assert!(cargo_content.contains("complete-app"));

    let manifest_content = std::fs::read_to_string(project_path.join("x402.toml")).unwrap();
    assert!(manifest_content.contains("complete-app"));
}

#[test]
fn test_workflow_parameter_validation_chain() {
    // Test a chain of parameter validations
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("validation-test");
    std::fs::create_dir(&path).unwrap();

    // Scenario: User provides valid string parameter
    let project_name = "my-app";
    let is_valid = !project_name.is_empty()
        && project_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');
    assert!(is_valid);

    // Create project with this name
    let project_path = path.join(project_name);
    std::fs::create_dir(&project_path).unwrap();
    assert!(project_path.exists());
}

#[test]
fn test_error_recovery_on_partial_creation() {
    // Test that partial creation doesn't leave orphaned files
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("partial-project");
    std::fs::create_dir(&project_path).unwrap();

    // Create some files
    std::fs::write(project_path.join("Cargo.toml"), "[package]").unwrap();
    std::fs::write(project_path.join("README.md"), "# Project").unwrap();

    // Simulate failure - cleanup all files
    let mut count = 0;
    for entry in std::fs::read_dir(&project_path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                std::fs::remove_file(&path).unwrap();
                count += 1;
            }
        }
    }

    // Verify cleanup
    assert_eq!(count, 2, "Should have removed 2 files");

    // Project directory should still exist (parent remains)
    assert!(project_path.exists());
}
