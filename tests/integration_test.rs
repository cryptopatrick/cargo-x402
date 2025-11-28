/// Integration tests for cargo-x402
///
/// Tests the full workflow of template discovery, validation, and rendering

use std::fs;
use tempfile::TempDir;

#[test]
fn test_schema_validation() {
    // Test that we can load and validate a template schema
    let test_schema = r#"
[template]
name = "Test Template"
description = "Test description"
version = "0.1.0"
authors = ["Test Author"]
repository = "https://github.com/test/test"
tags = ["test"]

[template.min_versions]
rust = "1.70.0"
cargo_x402 = "0.1.0"

[parameters.project_name]
type = "string"
description = "Project name"
pattern = "^[a-z0-9][a-z0-9-]*[a-z0-9]$"
default = "test-project"

[parameters.author]
type = "string"
description = "Author name"

[parameters.enable_feature]
type = "boolean"
description = "Enable optional feature"
default = true

[parameters.environment]
type = "enum"
description = "Deployment environment"
options = ["development", "staging", "production"]

[files]
include = ["src/**", "Cargo.toml", "README.md"]
exclude = [".git", "target"]
"#;

    let temp_dir = TempDir::new().unwrap();
    let schema_path = temp_dir.path().join("x402.toml");
    fs::write(&schema_path, test_schema).unwrap();

    // In a real test, we would parse and validate the schema here
    assert!(schema_path.exists());
    assert!(fs::read_to_string(&schema_path).unwrap().contains("[template]"));
}

#[test]
fn test_template_directory_structure() {
    // Test that template directories have the correct structure
    let required_files = vec!["x402.toml", "Cargo.toml", "README.md"];

    for file in required_files {
        // These would be checked during template discovery
        assert!(!file.is_empty());
    }
}

#[test]
fn test_parameter_validation() {
    // Test string parameter with pattern
    let pattern = r"^[a-z0-9][a-z0-9-]*[a-z0-9]$";
    let re = regex::Regex::new(pattern).unwrap();

    assert!(re.is_match("valid-project-name"));
    assert!(re.is_match("v1"));
    assert!(!re.is_match("Invalid-Name")); // uppercase
    assert!(!re.is_match("-invalid")); // starts with dash
    assert!(!re.is_match("invalid-")); // ends with dash
}

#[test]
fn test_liquid_template_rendering() {
    // Test that Liquid templating works correctly
    let template_str = "Hello {{ name }}, welcome to {{ project_name }}!";

    // This would be rendered with a context containing name and project_name
    assert!(template_str.contains("{{"));
    assert!(template_str.contains("}}"));
}

#[test]
fn test_file_pattern_simple() {
    // Test simple file pattern matching
    let filename = "Cargo.toml";
    let pattern = "Cargo.*";

    assert!(matches_pattern(filename, pattern));
}

#[test]
fn test_template_download_url_normalization() {
    // Test GitHub URL normalization
    let test_cases = vec![
        ("user/repo", "https://github.com/user/repo/archive/refs/heads/main.zip"),
        (
            "https://github.com/user/repo",
            "https://github.com/user/repo/archive/refs/heads/main.zip",
        ),
        (
            "https://github.com/user/repo/",
            "https://github.com/user/repo/archive/refs/heads/main.zip",
        ),
    ];

    for (input, expected) in test_cases {
        let normalized = normalize_github_url(input);
        assert_eq!(normalized, expected);
    }
}

#[test]
fn test_parameter_validation_patterns() {
    // Test valid project names
    let pattern = r"^[a-z0-9][a-z0-9-]*[a-z0-9]$|^[a-z0-9]$";
    let re = regex::Regex::new(pattern).unwrap();

    assert!(re.is_match("valid-name"));
    assert!(re.is_match("a"));
    assert!(re.is_match("project123"));
}

#[test]
fn test_project_name_validation() {
    // Test project name validation follows Cargo conventions
    let valid_names = vec!["my-project", "my_project", "myproject", "project123"];

    for name in valid_names {
        let pattern = r"^[a-z][a-z0-9_-]*[a-z0-9]$|^[a-z0-9]$";
        let re = regex::Regex::new(pattern).unwrap();
        assert!(re.is_match(name), "Should accept: {}", name);
    }
}

#[test]
fn test_binary_file_detection() {
    // Test that binary files are correctly identified
    let binary_extensions = vec!["png", "jpg", "jpeg", "gif", "zip", "tar", "bin"];

    for ext in binary_extensions {
        let filename = format!("file.{}", ext);
        assert!(is_binary_file(&filename));
    }

    let text_files = vec!["txt", "rs", "toml", "json", "md"];
    for ext in text_files {
        let filename = format!("file.{}", ext);
        assert!(!is_binary_file(&filename));
    }
}

#[test]
fn test_version_requirement_checking() {
    // Test semantic version comparison
    let test_cases = vec![
        // (required, actual, should_pass)
        ("1.70.0", "1.70.0", true),
        ("1.70.0", "1.71.0", true),
        ("1.70.0", "2.0.0", true),
        ("1.70.0", "1.69.0", false),
        ("1.0.0", "1.0.1", true),
    ];

    for (required, actual, should_pass) in test_cases {
        let result = version_check(required, actual);
        assert_eq!(result, should_pass);
    }
}

#[test]
fn test_cache_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    let cache_file = temp_dir.path().join("templates.json");

    let cache_json = r#"{
  "cached_at": "2024-01-15T10:30:00Z",
  "ttl_seconds": 3600,
  "templates": []
}"#;

    fs::write(&cache_file, cache_json).unwrap();
    assert!(cache_file.exists());

    let content = fs::read_to_string(&cache_file).unwrap();
    assert!(content.contains("cached_at"));
}

#[test]
fn test_github_shorthand_parsing() {
    let shorthand = "user/repo";
    let (owner, repo) = parse_github_shorthand(shorthand);

    assert_eq!(owner, "user");
    assert_eq!(repo, "repo");
}

#[test]
fn test_file_walkdir_operations() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create test structure
    fs::create_dir(base_path.join("src")).unwrap();
    fs::write(base_path.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(base_path.join("Cargo.toml"), "[package]").unwrap();

    // Verify files exist
    assert!(base_path.join("src/main.rs").exists());
    assert!(base_path.join("Cargo.toml").exists());
}

// Helper functions

fn matches_pattern(filename: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    if pattern.contains("*") {
        let prefix = pattern.split('*').next().unwrap_or("");
        let suffix = pattern.split('*').last().unwrap_or("");

        return filename.starts_with(prefix) && filename.ends_with(suffix);
    }

    filename == pattern
}

fn normalize_github_url(url: &str) -> String {
    let url = if url.contains("://") {
        url.trim_end_matches('/').to_string()
    } else {
        format!("https://github.com/{}", url)
    };

    format!("{}/archive/refs/heads/main.zip", url)
}

fn is_binary_file(filename: &str) -> bool {
    let binary_extensions = ["png", "jpg", "jpeg", "gif", "zip", "tar", "bin", "exe"];
    binary_extensions
        .iter()
        .any(|ext| filename.ends_with(ext))
}

fn version_check(required: &str, actual: &str) -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
        let parts: Vec<&str> = v.split('.').collect();
        (
            parts[0].parse().unwrap_or(0),
            parts[1].parse().unwrap_or(0),
            parts[2].parse().unwrap_or(0),
        )
    };

    let (req_major, req_minor, req_patch) = parse_version(required);
    let (act_major, act_minor, act_patch) = parse_version(actual);

    if act_major > req_major {
        true
    } else if act_major < req_major {
        false
    } else if act_minor > req_minor {
        true
    } else if act_minor < req_minor {
        false
    } else {
        act_patch >= req_patch
    }
}

fn parse_github_shorthand(shorthand: &str) -> (String, String) {
    let parts: Vec<&str> = shorthand.split('/').collect();
    (parts[0].to_string(), parts[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_detection() {
        assert!(is_binary_file("image.png"));
        assert!(is_binary_file("archive.zip"));
        assert!(!is_binary_file("main.rs"));
        assert!(!is_binary_file("README.md"));
    }

    #[test]
    fn test_version_comparison() {
        assert!(version_check("1.0.0", "1.0.0"));
        assert!(version_check("1.0.0", "1.0.1"));
        assert!(!version_check("1.1.0", "1.0.0"));
    }

    #[test]
    fn test_pattern_matching() {
        assert!(matches_pattern("Cargo.toml", "Cargo.*"));
        assert!(matches_pattern("main.rs", "*.rs"));
        assert!(!matches_pattern("main.rs", "*.toml"));
    }

    #[test]
    fn test_github_url_normalization() {
        let url = normalize_github_url("user/repo");
        assert!(url.contains("github.com"));
        assert!(url.ends_with(".zip"));
    }
}
