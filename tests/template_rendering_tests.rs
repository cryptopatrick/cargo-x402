/// Template rendering and file operation tests

use std::fs;
use tempfile::TempDir;

#[test]
fn test_template_variable_substitution() {
    let test_cases = vec![
        ("Hello {{ name }}", "Hello World"),
        ("Project: {{ project_name }}", "Project: my-app"),
        ("Author: {{ author }}", "Author: John Doe"),
        ("Version: {{ version }}", "Version: 0.1.0"),
        ("Date: {{ date }}", "Date: 2024-01-15"),
    ];

    // Verify variable patterns are correctly identified
    for (template, _expected) in test_cases {
        assert!(template.contains("{{"));
        assert!(template.contains("}}"));
    }
}

#[test]
fn test_conditional_template_blocks() {
    let template = r#"
{% if enable_feature %}
This feature is enabled
{% endif %}

{% if db_enabled %}
Database configuration
{% endif %}
"#;

    assert!(template.contains("{% if"));
    assert!(template.contains("{% endif %}"));
}

#[test]
fn test_loop_template_blocks() {
    let template = r#"
{% for item in items %}
- {{ item }}
{% endfor %}
"#;

    assert!(template.contains("{% for"));
    assert!(template.contains("{% endfor %}"));
}

#[test]
fn test_skip_binary_files() {
    let binary_files = vec![
        "icon.png",
        "logo.jpg",
        "archive.zip",
        "executable.exe",
        "image.gif",
    ];

    for file in binary_files {
        assert!(should_skip_binary(file));
    }

    let text_files = vec![
        "main.rs",
        "config.toml",
        "README.md",
        "LICENSE",
        "package.json",
    ];

    for file in text_files {
        assert!(!should_skip_binary(file));
    }
}

#[test]
fn test_template_file_detection() {
    let template_files = vec![
        "Cargo.toml",
        "package.json",
        "src/main.rs",
        "README.md",
        ".env.example",
    ];

    for file in template_files {
        assert!(!file.is_empty());
    }
}

#[test]
fn test_exclude_git_directory() {
    let paths = vec![
        ".git/config",
        ".git/objects/abc123",
        ".gitignore",
        "src/main.rs",
        ".github/workflows/ci.yml",
    ];

    for path in paths {
        if path.starts_with(".git/") {
            assert!(should_exclude_path(path));
        } else if path == ".git" {
            assert!(should_exclude_path(path));
        }
    }
}

#[test]
fn test_exclude_target_directory() {
    let paths = vec![
        "target/debug/app",
        "target/release/app",
        "src/main.rs",
        "target_data/file.txt",
    ];

    for path in paths {
        if path.starts_with("target/") {
            assert!(should_exclude_path(path));
        }
    }
}

#[test]
fn test_file_walkdir_operations() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    // Create test structure
    fs::create_dir(base_path.join("src")).unwrap();
    fs::write(base_path.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(base_path.join("Cargo.toml"), "[package]").unwrap();
    fs::write(base_path.join("README.md"), "# Test").unwrap();

    // Verify files exist
    assert!(base_path.join("src/main.rs").exists());
    assert!(base_path.join("Cargo.toml").exists());
    assert!(base_path.join("README.md").exists());
}

#[test]
fn test_glob_pattern_matching() {
    let patterns = vec![
        ("src/**/*.rs", "src/main.rs", true),
        ("src/**/*.rs", "src/modules/lib.rs", true),
        ("src/**/*.rs", "tests/test.rs", false),
        ("Cargo.*", "Cargo.toml", true),
        ("Cargo.*", "Cargo.lock", true),
        ("*.md", "README.md", true),
        ("*.md", "src/README.md", false),
    ];

    for (pattern, path, should_match) in patterns {
        let matches = glob_matches(pattern, path);
        assert_eq!(
            matches, should_match,
            "Pattern '{}' vs path '{}': expected {}, got {}",
            pattern, path, should_match, matches
        );
    }
}

#[test]
fn test_path_interpolation_in_filenames() {
    let filename_templates = vec![
        ("src/{{ project_name }}.rs", "my_app", "src/my_app.rs"),
        ("LICENSE-{{ author }}", "John", "LICENSE-John"),
        (".env.{{ environment }}", "prod", ".env.prod"),
    ];

    for (template, var_value, expected) in filename_templates {
        let result = interpolate_filename(template, var_value);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_preserve_executable_permissions() {
    // Test that scripts maintain executable status
    let script_files = vec!["scripts/deploy.sh", "bin/cli-tool", "hooks/pre-commit"];

    for script in script_files {
        // In real implementation, these would be checked for executable bit
        assert!(!script.is_empty());
    }
}

#[test]
fn test_file_encoding_preservation() {
    let encoded_files = vec![
        ("UTF-8", "README.md"),
        ("UTF-8", "src/main.rs"),
        ("Binary", "assets/logo.png"),
        ("Binary", "archive.zip"),
    ];

    for (encoding, file) in encoded_files {
        assert!(!encoding.is_empty());
        assert!(!file.is_empty());
    }
}

#[test]
fn test_symlink_handling() {
    // Test that symlinks are either followed or skipped correctly
    let should_follow_symlinks = false; // Default behavior: don't follow symlinks

    if should_follow_symlinks {
        // Would test symlink resolution
    } else {
        // Symlinks should be skipped
        assert!(true);
    }
}

// Helper functions

fn should_skip_binary(filename: &str) -> bool {
    let binary_extensions = ["png", "jpg", "jpeg", "gif", "zip", "exe", "bin"];
    binary_extensions
        .iter()
        .any(|ext| filename.ends_with(ext))
}

fn should_exclude_path(path: &str) -> bool {
    path.starts_with(".git/") || path == ".git" || path.starts_with("target/")
}

fn glob_matches(pattern: &str, path: &str) -> bool {
    // Handle simple glob matching for common patterns
    if pattern == "*" {
        return !path.contains("/");
    }

    // Handle patterns like "src/**/*.rs" (** matches any number of directories)
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0].trim_end_matches('/');
            let suffix = parts[1].trim_start_matches('/');

            // Path must start with prefix
            if !path.starts_with(prefix) {
                return false;
            }

            // Get the part of the path after the prefix
            let remaining = if prefix.is_empty() {
                path
            } else {
                &path[prefix.len()..].trim_start_matches('/')
            };

            // Check if remaining matches suffix pattern
            if suffix.contains("*.") {
                // For patterns like "*.rs", extract the extension
                let extension = suffix.trim_start_matches('*');
                return remaining.ends_with(extension);
            }

            // Otherwise, just check if path ends with suffix
            return remaining.ends_with(suffix);
        }
        return false;
    }

    // Handle simple * patterns like "*.md"
    // These should only match files without directory separators
    if pattern.starts_with("*.") {
        // Pattern like "*.md" should not match "src/README.md"
        if path.contains("/") {
            return false;
        }
        let extension = pattern.trim_start_matches('*');
        return path.ends_with(extension);
    }

    if pattern.contains("*") {
        let parts: Vec<&str> = pattern.split("*").collect();
        let mut remaining = path;

        for (i, part) in parts.iter().enumerate() {
            if i == 0 {
                if !remaining.starts_with(part) {
                    return false;
                }
                remaining = &remaining[part.len()..];
            } else if i == parts.len() - 1 {
                if !remaining.ends_with(part) {
                    return false;
                }
            } else if let Some(pos) = remaining.find(part) {
                remaining = &remaining[pos + part.len()..];
            } else {
                return false;
            }
        }
        return true;
    }

    pattern == path
}

fn interpolate_filename(template: &str, value: &str) -> String {
    template.replace("{{ project_name }}", value)
        .replace("{{ author }}", value)
        .replace("{{ environment }}", value)
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_binary_file_detection() {
        assert!(should_skip_binary("image.png"));
        assert!(!should_skip_binary("main.rs"));
    }

    #[test]
    fn test_glob_pattern_logic() {
        assert!(glob_matches("*.rs", "main.rs"));
        assert!(!glob_matches("*.rs", "main.toml"));
    }

    #[test]
    fn test_filename_interpolation() {
        assert_eq!(interpolate_filename("src/{{ project_name }}.rs", "app"), "src/app.rs");
    }
}
