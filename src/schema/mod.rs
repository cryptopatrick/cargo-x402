//! Template schema definitions and validation.
//!
//! This module defines the TOML schema that every x402 template must implement.
//! It includes structures for parsing template metadata and parameters, as well as
//! validation logic to ensure templates are well-formed.
//!
//! ## The x402.toml Manifest
//!
//! Every template requires an `x402.toml` file at the repository root with:
//!
//! ```toml
//! [template]
//! name = "template-name"
//! version = "1.0.0"
//! description = "A brief description"
//! repository = "https://github.com/owner/template"
//! license = "MIT"
//!
//! [parameters.param_name]
//! type = "string"    # or "boolean", "enum"
//! prompt = "Prompt text for user"
//! description = "Longer explanation"
//! default = "value"  # optional
//! ```
//!
//! ## Submodules
//!
//! - [`template`]: Core schema types (`TemplateSchema`, `Parameter`, etc.)
//! - [`validator`]: Validation logic for templates and parameters
//!
//! ## Example
//!
//! ```no_run
//! use cargo_x402::schema::TemplateSchema;
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let path = Path::new("x402.toml");
//! let schema = TemplateSchema::from_file(path)?;
//! println!("Template: {}", schema.metadata.name);
//! # Ok(())
//! # }
//! ```

pub mod template;
pub mod validator;

pub use template::{Parameter, TemplateMetadata, TemplateSchema};
pub use validator::Validator;
