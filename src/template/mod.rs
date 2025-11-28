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
//! ```no_run,ignore
//! use cargo_x402::template;
//!
//! // See integration tests for actual usage examples
//! // Template operations are coordinated through the commands module
//! ```

pub mod downloader;
pub mod render;

pub use downloader::Downloader;
pub use render::Renderer;
