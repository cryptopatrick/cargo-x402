//! Template schema validation

use super::TemplateSchema;
use crate::error::{Error, Result};
use regex::Regex;
use semver::Version;
use std::path::Path;

/// Validates x402.toml against schema requirements
pub struct Validator;

impl Validator {
    /// Load and validate a template's x402.toml file
    pub fn load_and_validate(path: &Path) -> Result<TemplateSchema> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::FileSystemError(format!("Cannot read x402.toml: {}", e)))?;

        let schema: TemplateSchema = toml::from_str(&content)
            .map_err(|e| Error::TomlError(format!("Invalid TOML: {}", e)))?;

        Self::validate_schema(&schema)?;
        Ok(schema)
    }

    /// Validate schema structure and constraints
    fn validate_schema(schema: &TemplateSchema) -> Result<()> {
        let meta = &schema.template;

        // Required fields
        if meta.name.is_empty() {
            return Err(Error::ValidationError {
                field: "template.name".to_string(),
                message: "Template name is required".to_string(),
            });
        }

        if meta.name.len() > 100 {
            return Err(Error::ValidationError {
                field: "template.name".to_string(),
                message: "Template name must be 100 characters or less".to_string(),
            });
        }

        if meta.description.is_empty() {
            return Err(Error::ValidationError {
                field: "template.description".to_string(),
                message: "Template description is required".to_string(),
            });
        }

        if meta.description.len() < 10 || meta.description.len() > 200 {
            return Err(Error::ValidationError {
                field: "template.description".to_string(),
                message: "Description must be 10-200 characters".to_string(),
            });
        }

        // Version validation
        if let Err(e) = Version::parse(&meta.version) {
            return Err(Error::ValidationError {
                field: "template.version".to_string(),
                message: format!("Invalid semantic version: {}", e),
            });
        }

        // Authors validation
        if meta.authors.is_empty() {
            return Err(Error::ValidationError {
                field: "template.authors".to_string(),
                message: "At least one author is required".to_string(),
            });
        }

        // Repository validation
        if !meta.repository.starts_with("https://github.com/") {
            return Err(Error::ValidationError {
                field: "template.repository".to_string(),
                message: "Repository must be an HTTPS GitHub URL (https://github.com/...)".to_string(),
            });
        }

        // Min version validations
        if let Some(ref rust_version) = meta.min_rust_version {
            if Version::parse(rust_version).is_err() {
                return Err(Error::ValidationError {
                    field: "template.min_rust_version".to_string(),
                    message: format!("Invalid semantic version: {}", rust_version),
                });
            }
        }

        if let Some(ref cli_version) = meta.min_x402_cli_version {
            if Version::parse(cli_version).is_err() {
                return Err(Error::ValidationError {
                    field: "template.min_x402_cli_version".to_string(),
                    message: format!("Invalid semantic version: {}", cli_version),
                });
            }
        }

        // Parameters validation
        if let Some(ref params) = schema.parameters {
            for (name, param) in params {
                Self::validate_parameter(name, param)?;
            }
        }

        // Files validation
        if let Some(ref files) = schema.files {
            if !files.include.is_empty() {
                for pattern in &files.include {
                    Self::validate_glob_pattern(pattern, "include")?;
                }
            }
            if !files.exclude.is_empty() {
                for pattern in &files.exclude {
                    Self::validate_glob_pattern(pattern, "exclude")?;
                }
            }
        }

        Ok(())
    }

    /// Validate a single parameter definition
    fn validate_parameter(name: &str, param: &crate::schema::Parameter) -> Result<()> {
        use crate::schema::Parameter;

        match param {
            Parameter::String { default, pattern, .. } => {
                // Validate pattern if provided
                if let Some(p) = pattern {
                    Regex::new(p).map_err(|e| Error::ValidationError {
                        field: format!("parameters.{}.pattern", name),
                        message: format!("Invalid regex: {}", e),
                    })?;
                }

                // Validate default against pattern
                if let Some(_p) = pattern {
                    if let Err(e) = param.validate(default) {
                        return Err(Error::ValidationError {
                            field: format!("parameters.{}.default", name),
                            message: e,
                        });
                    }
                }
            }

            Parameter::Enum {
                choices,
                default,
                ..
            } => {
                if choices.is_empty() {
                    return Err(Error::ValidationError {
                        field: format!("parameters.{}.enum", name),
                        message: "Enum must have at least one choice".to_string(),
                    });
                }

                if !choices.contains(default) {
                    return Err(Error::ValidationError {
                        field: format!("parameters.{}.default", name),
                        message: format!(
                            "Default value '{}' not in enum choices",
                            default
                        ),
                    });
                }
            }

            Parameter::Boolean { .. } => {
                // Boolean parameters are always valid
            }
        }

        Ok(())
    }

    /// Validate glob pattern syntax
    fn validate_glob_pattern(pattern: &str, context: &str) -> Result<()> {
        // Simple validation: check for common glob patterns
        // More sophisticated validation could use the glob crate
        if pattern.is_empty() {
            return Err(Error::ValidationError {
                field: format!("files.{}", context),
                message: "Glob pattern cannot be empty".to_string(),
            });
        }

        // Valid glob patterns should contain at least one path component
        if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
            // It's a literal path, which is fine
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_schema_missing_name() {
        let schema = TemplateSchema {
            template: crate::schema::TemplateMetadata {
                name: String::new(),
                description: "test description".to_string(),
                version: "1.0.0".to_string(),
                authors: vec!["test".to_string()],
                repository: "https://github.com/test/test".to_string(),
                tags: vec![],
                min_rust_version: None,
                min_x402_cli_version: None,
            },
            parameters: None,
            files: None,
        };

        assert!(Validator::validate_schema(&schema).is_err());
    }

    #[test]
    fn test_validate_invalid_repository() {
        let schema = TemplateSchema {
            template: crate::schema::TemplateMetadata {
                name: "test".to_string(),
                description: "test description".to_string(),
                version: "1.0.0".to_string(),
                authors: vec!["test".to_string()],
                repository: "http://github.com/test/test".to_string(),
                tags: vec![],
                min_rust_version: None,
                min_x402_cli_version: None,
            },
            parameters: None,
            files: None,
        };

        assert!(Validator::validate_schema(&schema).is_err());
    }
}
