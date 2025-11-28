//! Template discovery and caching system.
//!
//! This module handles discovering templates from GitHub by searching for repositories
//! tagged with the `x402-template` topic. It includes an intelligent caching system to
//! reduce network requests and improve performance.
//!
//! ## Submodules
//!
//! - [`github`]: GitHub API integration for template discovery
//! - [`cache`]: Local caching of discovered templates
//!
//! ## Overview
//!
//! The discovery process:
//! 1. Check local cache for recently discovered templates
//! 2. If cache is fresh (< 1 hour old), return cached results
//! 3. If cache is stale or missing, query GitHub API
//! 4. Update cache with new results
//! 5. Return template list to caller
//!
//! This minimizes GitHub API requests while keeping templates reasonably up-to-date.
//!
//! ## Example
//!
//! ```no_run,ignore
//! use cargo_x402::discovery::github::GitHubDiscovery;
//!
//! // See integration tests for actual async usage examples
//! // The discovery system requires async runtime context
//! ```

pub mod cache;
pub mod github;

pub use github::GitHubDiscovery;
pub use cache::Cache;

use serde::{Deserialize, Serialize};

/// Information about a discoverable template from GitHub.
///
/// Represents the metadata of a template repository that was discovered
/// via the GitHub `x402-template` topic search.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemplateInfo {
    /// Template name
    pub name: String,

    /// One-line description
    pub description: String,

    /// Full GitHub URL
    pub url: String,

    /// Repository owner
    pub owner: String,

    /// Repository name
    pub repo: String,

    /// Number of GitHub stars
    pub stars: u32,

    /// Primary language
    pub language: String,

    /// GitHub topics
    #[serde(default)]
    pub topics: Vec<String>,
}

impl TemplateInfo {
    /// Get shorthand reference (owner/repo)
    #[allow(dead_code)] // Intentionally public for library users
    pub fn shorthand(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }

    /// Check if template matches filter tags
    pub fn matches_tags(&self, tags: &[String]) -> bool {
        if tags.is_empty() {
            return true;
        }
        tags.iter().any(|tag| self.topics.contains(tag))
    }
}
