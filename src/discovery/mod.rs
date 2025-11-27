//! Template discovery via GitHub topics

pub mod cache;
pub mod github;

pub use github::GitHubDiscovery;
pub use cache::Cache;

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Information about a discoverable template
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

/// Template discovery trait
pub trait Discoverer: Send + Sync {
    /// Discover all x402 templates
    fn discover(&self) -> Result<Vec<TemplateInfo>>;

    /// Get information about a specific template
    fn get_template(&self, shorthand: &str) -> Result<Option<TemplateInfo>>;
}
