//! High-level operations exposed via the CLI.
//!
//! This module provides the main entry points for cargo-x402 functionality:
//! discovering and listing templates, and creating new projects from them.
//!
//! ## Commands
//!
//! ### list
//!
//! List available templates from GitHub with optional filtering by tags.
//! Results are cached locally and can be refreshed with `--refresh` flag.
//!
//! ### create
//!
//! Create a new project from a template by downloading, validating, and rendering it.
//! Supports interactive prompts or non-interactive specification via flags.
//!
//! ## Submodules
//!
//! - [`list`]: Template discovery and filtering
//! - [`create`]: Project creation from templates
//!
//! ## Example
//!
//! ```no_run,ignore
//! use cargo_x402::commands::list;
//! use cargo_x402::commands::create;
//!
//! // See integration tests for actual usage examples
//! // The CLI provides the primary interface for these operations
//! ```

pub mod create;
pub mod list;
