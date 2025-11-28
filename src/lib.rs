#![warn(missing_docs)]
//! # cargo-x402
//!
//! A fast and flexible project scaffolder for Rust applications using GitHub-hosted templates.
//!
//! This library provides the core functionality for discovering, validating, and rendering
//! x402 project templates stored as GitHub repositories. It supports:
//!
//! - **Template Discovery**: Automatic discovery of public templates via GitHub's `x402-template` topic
//! - **Schema Validation**: TOML-based template manifests with comprehensive validation
//! - **Parameter Handling**: Support for string, boolean, and enum parameters with validation
//! - **Liquid Templating**: Safe variable substitution and conditional rendering
//! - **Interactive CLI**: User-friendly prompts and progress feedback
//!
//! ## Architecture
//!
//! The library is organized into distinct modules:
//!
//! - [`discovery`]: Finding and caching templates from GitHub
//! - [`schema`]: Template manifest parsing and validation
//! - [`template`]: Downloading and rendering templates
//! - [`interactive`]: User interaction and prompts
//! - [`commands`]: High-level operations (list, create)
//! - [`error`]: Error types and handling
//!
//! ## Quick Example
//!
//! ```no_run
//! use cargo_x402::commands::list::list_templates;
//! use cargo_x402::discovery::github::GitHubDiscovery;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // List available templates
//! let templates = list_templates(false)?;
//! println!("Found {} templates", templates.len());
//! # Ok(())
//! # }
//! ```

pub mod commands;
pub mod discovery;
pub mod error;
pub mod interactive;
pub mod schema;
pub mod template;

/// The version of cargo-x402 being used
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
