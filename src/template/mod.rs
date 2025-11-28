//! Template downloading and rendering operations.
//!
//! This module handles the lifecycle of templates after discovery:
//! downloading from GitHub and rendering with user-provided parameters.
//!
//! ## Workflow
//!
//! 1. **Download** (`downloader`): Clone template repository to temporary location
//! 2. **Validate**: Parse and validate x402.toml manifest
//! 3. **Render** (`render`): Process Liquid templates with user parameters
//! 4. **Finalize**: Copy processed files to destination, cleanup .git directory
//!
//! ## Submodules
//!
//! - [`downloader`]: GitHub template repository cloning
//! - [`render`]: Liquid template rendering with parameter substitution
//!
//! ## Example
//!
//! ```no_run
//! use cargo_x402::template::{Downloader, Renderer};
//! use std::collections::HashMap;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Download template from GitHub
//! let downloader = Downloader::new();
//! let temp_path = downloader.download("xForth/x402-template-basic-api")?;
//!
//! // Render template with parameters
//! let mut params = HashMap::new();
//! params.insert("project_name".to_string(), "my-project".to_string());
//!
//! let renderer = Renderer::new();
//! let output_path = renderer.render(&temp_path, &params)?;
//! println!("Project created at: {}", output_path.display());
//! # Ok(())
//! # }
//! ```

pub mod downloader;
pub mod render;

pub use downloader::Downloader;
pub use render::Renderer;
