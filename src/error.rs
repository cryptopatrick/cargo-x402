use std::fmt;

/// Custom error type for cargo-x402
#[derive(Debug)]
pub enum Error {
    /// Template not found in discovery results
    TemplateNotFound(String),

    /// Invalid x402.toml schema
    InvalidSchema(String),

    /// Field validation error
    ValidationError { field: String, message: String },

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

/// Result type alias for cargo-x402 operations
pub type Result<T> = std::result::Result<T, Error>;
