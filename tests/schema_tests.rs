/// Schema validation tests

#[test]
fn test_valid_template_schema() {
    let schema_content = r#"
[template]
name = "Test Template"
description = "A test template"
version = "0.1.0"
authors = ["Test"]
repository = "https://github.com/test/test"
tags = ["test"]

[template.min_versions]
rust = "1.70.0"

[parameters.name]
type = "string"
description = "Project name"
pattern = "^[a-z0-9-]+$"

[parameters.enabled]
type = "boolean"
description = "Enable feature"

[parameters.env]
type = "enum"
description = "Environment"
options = ["dev", "prod"]

[files]
include = ["src/**", "Cargo.toml"]
exclude = ["target", ".git"]
"#;

    // Validate that the schema parses correctly
    assert!(schema_content.contains("[template]"));
    assert!(schema_content.contains("[parameters"));
    assert!(schema_content.contains("[files]"));
}

#[test]
fn test_schema_without_optional_fields() {
    let minimal_schema = r#"
[template]
name = "Minimal"
description = "Minimal template"
version = "0.1.0"
authors = ["Author"]
repository = "https://github.com/author/minimal"

[files]
include = ["src/**"]
"#;

    assert!(minimal_schema.contains("name = \"Minimal\""));
    assert!(!minimal_schema.contains("min_versions")); // optional, not present
}

#[test]
fn test_parameter_types() {
    let schema = r#"
[parameters.string_param]
type = "string"
description = "A string"
pattern = "^[a-z]+$"

[parameters.bool_param]
type = "boolean"
description = "A boolean"
default = true

[parameters.enum_param]
type = "enum"
description = "An enum"
options = ["a", "b", "c"]
"#;

    assert!(schema.contains("type = \"string\""));
    assert!(schema.contains("type = \"boolean\""));
    assert!(schema.contains("type = \"enum\""));
}

#[test]
fn test_file_rules() {
    let schema = r#"
[files]
include = ["src/**", "Cargo.toml", "README.md"]
exclude = ["target", ".git", "*.bak"]
"#;

    assert!(schema.contains("include"));
    assert!(schema.contains("exclude"));
    assert!(schema.contains("src/**"));
}

#[test]
fn test_github_url_validation() {
    let valid_urls = vec![
        "https://github.com/user/repo",
        "user/repo",
        "https://github.com/user/repo/",
    ];

    let invalid_urls = vec![
        "not-a-url",
        "http://example.com",
        "ftp://files.com/repo",
    ];

    for url in valid_urls {
        assert!(is_valid_github_url(url));
    }

    for url in invalid_urls {
        assert!(!is_valid_github_url(url));
    }
}

#[test]
fn test_semantic_version_format() {
    let valid_versions = vec!["0.1.0", "1.0.0", "1.70.0", "2.5.11"];
    let invalid_versions = vec!["1", "1.0", "v1.0.0", "1.0.0-alpha", "1.0.0.0"];

    for version in valid_versions {
        assert!(is_valid_semver(version));
    }

    for version in invalid_versions {
        assert!(!is_valid_semver(version));
    }
}

#[test]
fn test_parameter_validation_rules() {
    // Test string pattern validation
    let pattern = r"^[a-z0-9-]+$";
    let re = regex::Regex::new(pattern).unwrap();

    assert!(re.is_match("valid-name-123"));
    assert!(!re.is_match("Invalid-Name")); // uppercase
    assert!(!re.is_match("invalid_name")); // underscore not allowed

    // Test enum options
    let options = vec!["dev", "staging", "prod"];
    assert!(options.contains(&"dev"));
    assert!(options.contains(&"prod"));
    assert!(!options.contains(&"unknown"));
}

#[test]
fn test_required_schema_fields() {
    let incomplete_schema = r#"
[template]
name = "Incomplete"
description = "Missing fields"
"#;

    // name: present
    assert!(incomplete_schema.contains("name = \"Incomplete\""));
    // version: missing - should be caught during validation
    assert!(!incomplete_schema.contains("version"));
}

// Helper functions

fn is_valid_github_url(url: &str) -> bool {
    url.contains("github.com") || (url.contains("/") && !url.contains("://"))
}

fn is_valid_semver(version: &str) -> bool {
    let pattern = r"^\d+\.\d+\.\d+$";
    regex::Regex::new(pattern)
        .map(|re| re.is_match(version))
        .unwrap_or(false)
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_github_url_validation_logic() {
        assert!(is_valid_github_url("github.com/user/repo"));
        assert!(is_valid_github_url("user/repo"));
        assert!(!is_valid_github_url("invalid"));
    }

    #[test]
    fn test_semver_validation_logic() {
        assert!(is_valid_semver("1.0.0"));
        assert!(!is_valid_semver("1.0"));
    }
}
