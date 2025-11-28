//! Error types and handling for cargo-x402.
//!
//! This module defines the [`Error`] enum which represents all possible error conditions
//! that can occur during template discovery, validation, and rendering operations.
//!
//! ## Error Handling
//!
//! All operations in cargo-x402 return a [`Result<T>`], which is a type alias for
//! `std::result::Result<T, Error>`. Errors should be handled gracefully and provide
//! helpful guidance to users.
//!
//! ### Example
//!
//! ```no_run
//! use cargo_x402::error::{Error, Result};
//!
//! fn find_template(name: &str) -> Result<String> {
//!     if name.is_empty() {
//!         Err(Error::ParameterError("template name cannot be empty".to_string()))
//!     } else {
//!         Ok(name.to_string())
//!     }
//! }
//! ```
//!
//! ## Error Variants
//!
//! - **TemplateNotFound**: Template not found in discovery results
//! - **InvalidSchema**: Template manifest (x402.toml) has invalid schema
//! - **ValidationError**: Field validation failed with specific context
//! - **NetworkError**: Network operation failed (DNS, connection, etc.)
//! - **FileSystemError**: File I/O operation failed
//! - **ParameterError**: User parameter validation or processing failed
//! - **RenderError**: Liquid template rendering failed
//! - **GitHubApiError**: GitHub API request failed
//! - **TomlError**: TOML/JSON parsing failed
//! - **CacheError**: Cache directory operation failed
//! - **Cancelled**: User cancelled operation (e.g., interactive prompt)
//! - **Other**: Generic error for miscellaneous cases
//!
//! ## Display and Debug
//!
//! Errors implement both `Display` and `Debug` traits:
//! - `Display`: User-friendly message with helpful guidance
//! - `Debug`: Detailed error information for troubleshooting

use std::fmt;

/// Custom error type for cargo-x402
///
/// Represents all possible error conditions that can occur during
/// template discovery, validation, rendering, and project creation.
///
/// Each variant includes appropriate context and helpful error messages
/// for users to understand what went wrong and how to fix it.
#[derive(Debug)]
pub enum Error {
    /// Template not found in discovery results
    TemplateNotFound(String),

    /// Invalid x402.toml schema
    InvalidSchema(String),

    /// Field validation error
    ValidationError {
        /// The name of the field that failed validation
        field: String,
        /// The validation error message
        message: String,
    },

    /// Network-related error
    NetworkError(String),

    /// File system operation error
    FileSystemError(String),

    /// User input/parameter error
    ParameterError(String),

    /// Template rendering error
    RenderError(String),

    /// GitHub API error
    GitHubApiError(String),

    /// TOML parsing error
    TomlError(String),

    /// Cache operation error
    CacheError(String),

    /// User cancelled operation
    Cancelled,

    /// Other generic error
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TemplateNotFound(name) => {
                write!(f, "Template '{}' not found\n\nRun 'cargo-x402 list' to see available templates", name)
            }
            Error::InvalidSchema(msg) => {
                write!(f, "Invalid x402.toml schema: {}\n\nSee TEMPLATE_SCHEMA.md for details", msg)
            }
            Error::ValidationError { field, message } => {
                write!(f, "Validation error in '{}': {}", field, message)
            }
            Error::NetworkError(msg) => {
                write!(f, "Network error: {}\n\nMake sure you have internet connectivity", msg)
            }
            Error::FileSystemError(msg) => {
                write!(f, "File system error: {}", msg)
            }
            Error::ParameterError(msg) => {
                write!(f, "Parameter error: {}", msg)
            }
            Error::RenderError(msg) => {
                write!(f, "Template rendering error: {}", msg)
            }
            Error::GitHubApiError(msg) => {
                write!(f, "GitHub API error: {}\n\nCheck your internet connection or rate limits", msg)
            }
            Error::TomlError(msg) => {
                write!(f, "TOML parsing error: {}", msg)
            }
            Error::CacheError(msg) => {
                write!(f, "Cache error: {}", msg)
            }
            Error::Cancelled => {
                write!(f, "Operation cancelled by user")
            }
            Error::Other(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for Error {}

// Conversion implementations

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::TomlError(format!("JSON error: {}", err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlError(format!("TOML parsing error: {}", err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileSystemError(format!("IO error: {}", err))
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::ValidationError {
            field: "pattern".to_string(),
            message: err.to_string(),
        }
    }
}

/// Result type alias for cargo-x402 operations.
///
/// This is a convenience type alias for `std::result::Result<T, Error>`.
/// All fallible operations in cargo-x402 return this type.
///
/// # Examples
///
/// ```no_run
/// use cargo_x402::error::Result;
///
/// fn create_project(name: &str) -> Result<String> {
///     if name.is_empty() {
///         return Err(cargo_x402::error::Error::ParameterError(
///             "project name cannot be empty".to_string()
///         ));
///     }
///     Ok(format!("Project '{}' created successfully", name))
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_template_not_found() {
        let err = Error::TemplateNotFound("my-template".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Template 'my-template' not found"));
        assert!(msg.contains("cargo-x402 list"));
    }

    #[test]
    fn test_error_invalid_schema() {
        let err = Error::InvalidSchema("missing required field".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Invalid x402.toml schema"));
        assert!(msg.contains("missing required field"));
        assert!(msg.contains("TEMPLATE_SCHEMA.md"));
    }

    #[test]
    fn test_error_validation_error() {
        let err = Error::ValidationError {
            field: "project_name".to_string(),
            message: "must not contain spaces".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Validation error in 'project_name'"));
        assert!(msg.contains("must not contain spaces"));
    }

    #[test]
    fn test_error_network_error() {
        let err = Error::NetworkError("connection timeout".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Network error"));
        assert!(msg.contains("connection timeout"));
        assert!(msg.contains("internet connectivity"));
    }

    #[test]
    fn test_error_filesystem_error() {
        let err = Error::FileSystemError("permission denied".to_string());
        let msg = err.to_string();
        assert!(msg.contains("File system error"));
        assert!(msg.contains("permission denied"));
    }

    #[test]
    fn test_error_parameter_error() {
        let err = Error::ParameterError("invalid enum value".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Parameter error"));
        assert!(msg.contains("invalid enum value"));
    }

    #[test]
    fn test_error_render_error() {
        let err = Error::RenderError("undefined variable".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Template rendering error"));
        assert!(msg.contains("undefined variable"));
    }

    #[test]
    fn test_error_github_api_error() {
        let err = Error::GitHubApiError("rate limit exceeded".to_string());
        let msg = err.to_string();
        assert!(msg.contains("GitHub API error"));
        assert!(msg.contains("rate limit exceeded"));
        assert!(msg.contains("internet connection"));
        assert!(msg.contains("rate limits"));
    }

    #[test]
    fn test_error_toml_error() {
        let err = Error::TomlError("invalid syntax".to_string());
        let msg = err.to_string();
        assert!(msg.contains("TOML parsing error"));
        assert!(msg.contains("invalid syntax"));
    }

    #[test]
    fn test_error_cache_error() {
        let err = Error::CacheError("cache directory not writable".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Cache error"));
        assert!(msg.contains("cache directory not writable"));
    }

    #[test]
    fn test_error_cancelled() {
        let err = Error::Cancelled;
        let msg = err.to_string();
        assert_eq!(msg, "Operation cancelled by user");
    }

    #[test]
    fn test_error_other() {
        let err = Error::Other("something went wrong".to_string());
        let msg = err.to_string();
        assert_eq!(msg, "something went wrong");
    }

    #[test]
    fn test_error_debug_format() {
        let err = Error::TemplateNotFound("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("TemplateNotFound"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_error_is_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(Error::Cancelled);
        assert_eq!(err.to_string(), "Operation cancelled by user");
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::from(io_err);
        let msg = err.to_string();
        assert!(msg.contains("File system error"));
        assert!(msg.contains("IO error"));
    }

    #[test]
    fn test_error_from_toml_error() {
        let toml_str = "invalid = [";
        let toml_err = toml::from_str::<toml::Value>(toml_str).err().unwrap();
        let err = Error::from(toml_err);
        let msg = err.to_string();
        assert!(msg.contains("TOML parsing error"));
    }

    #[test]
    fn test_error_from_serde_json_error() {
        let json_str = "{invalid json}";
        let json_err = serde_json::from_str::<serde_json::Value>(json_str)
            .err()
            .unwrap();
        let err = Error::from(json_err);
        let msg = err.to_string();
        assert!(msg.contains("TOML parsing error"));
        assert!(msg.contains("JSON error"));
    }

    #[test]
    fn test_error_from_regex_error() {
        let regex_err = regex::Regex::new("[invalid").err().unwrap();
        let err = Error::from(regex_err);
        match err {
            Error::ValidationError { field, message } => {
                assert_eq!(field, "pattern");
                assert!(!message.is_empty());
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(Error::Cancelled);
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.to_string(), "Operation cancelled by user");
    }

    #[test]
    fn test_error_display_helpful_for_template_not_found() {
        let err = Error::TemplateNotFound("nonexistent".to_string());
        let msg = err.to_string();
        // Verify helpful next steps are included
        assert!(msg.contains("Run"));
        assert!(msg.contains("list"));
    }

    #[test]
    fn test_error_display_helpful_for_network() {
        let err = Error::NetworkError("dns resolution failed".to_string());
        let msg = err.to_string();
        // Verify helpful troubleshooting guidance
        assert!(msg.contains("internet connectivity"));
    }
}
