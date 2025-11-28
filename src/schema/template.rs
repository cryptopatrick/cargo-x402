//! Template schema structures matching x402.toml format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete x402 template schema from x402.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSchema {
    /// Template metadata section
    pub template: TemplateMetadata,
    /// Customizable parameters for template rendering
    pub parameters: Option<HashMap<String, Parameter>>,
    /// File inclusion/exclusion rules
    pub files: Option<FileRules>,
}

/// Template metadata from `[template]` section of x402.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Human-readable template name
    pub name: String,

    /// One-line description of the template
    pub description: String,

    /// Semantic version (MAJOR.MINOR.PATCH)
    pub version: String,

    /// List of template authors/maintainers
    pub authors: Vec<String>,

    /// HTTPS GitHub repository URL
    pub repository: String,

    /// Optional searchable tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Minimum Rust version required
    #[serde(default)]
    pub min_rust_version: Option<String>,

    /// Minimum cargo-x402 CLI version required
    #[serde(default)]
    pub min_x402_cli_version: Option<String>,
}

/// Parameter definition for template customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Parameter {
    /// String parameter with optional pattern validation
    #[serde(rename = "string")]
    String {
        /// Default value for this parameter
        default: String,
        /// Regex pattern for validation (optional)
        #[serde(default)]
        pattern: Option<String>,
        /// Description of the parameter
        #[serde(default)]
        description: Option<String>,
    },

    /// Boolean parameter
    #[serde(rename = "boolean")]
    Boolean {
        /// Default value (true or false)
        default: bool,
        /// Description of the parameter
        #[serde(default)]
        description: Option<String>,
    },

    /// Enumeration parameter with fixed choices
    #[serde(rename = "enum")]
    Enum {
        /// Allowed values for this parameter
        #[serde(rename = "enum")]
        choices: Vec<String>,
        /// Default choice
        default: String,
        /// Description of the parameter
        #[serde(default)]
        description: Option<String>,
    },
}

impl Parameter {
    /// Validate a value against this parameter's constraints
    pub fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            Parameter::String { pattern, .. } => {
                if let Some(pattern) = pattern {
                    let regex = regex::Regex::new(pattern)
                        .map_err(|e| format!("Invalid pattern: {}", e))?;
                    if !regex.is_match(value) {
                        return Err(format!(
                            "Value '{}' does not match pattern '{}'",
                            value, pattern
                        ));
                    }
                }
                Ok(())
            }
            Parameter::Boolean { .. } => {
                if !["true", "false", "yes", "no", "1", "0"].contains(&value.to_lowercase().as_str()) {
                    Err(format!("Expected boolean value, got '{}'", value))
                } else {
                    Ok(())
                }
            }
            Parameter::Enum { choices, .. } => {
                if !choices.contains(&value.to_string()) {
                    Err(format!(
                        "Value '{}' not in allowed options: {}",
                        value,
                        choices.join(", ")
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// File inclusion/exclusion rules from `[files]` section of x402.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRules {
    /// Glob patterns of files to include
    #[serde(default)]
    pub include: Vec<String>,

    /// Glob patterns of files to exclude
    #[serde(default)]
    pub exclude: Vec<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_parameter_validation() {
        let param = Parameter::String {
            default: "my-app".to_string(),
            pattern: Some("^[a-z][a-z0-9-]*$".to_string()),
            description: None,
        };

        assert!(param.validate("my-app").is_ok());
        assert!(param.validate("MyApp").is_err());
        assert!(param.validate("1-app").is_err());
    }

    #[test]
    fn test_enum_parameter_validation() {
        let param = Parameter::Enum {
            choices: vec!["postgres".to_string(), "sqlite".to_string()],
            default: "postgres".to_string(),
            description: None,
        };

        assert!(param.validate("postgres").is_ok());
        assert!(param.validate("mysql").is_err());
    }

    #[test]
    fn test_boolean_parameter_validation() {
        let param = Parameter::Boolean {
            default: true,
            description: None,
        };

        assert!(param.validate("true").is_ok());
        assert!(param.validate("false").is_ok());
        assert!(param.validate("invalid").is_err());
    }
}
