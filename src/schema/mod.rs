//! Template schema definitions and structures
//!
//! Defines the x402.toml schema that every template must implement.

pub mod template;
pub mod validator;

pub use template::{Parameter, TemplateMetadata, TemplateSchema};
pub use validator::Validator;
