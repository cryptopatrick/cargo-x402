//! cargo-x402: Scaffold x402 projects from pluggable GitHub templates
//!
//! This library provides the core functionality for discovering, validating, and rendering
//! x402 project templates stored as GitHub repositories.

pub mod commands;
pub mod discovery;
pub mod error;
pub mod interactive;
pub mod schema;
pub mod template;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
