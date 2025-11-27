//! GitHub API integration for template discovery

use super::TemplateInfo;
use crate::error::{Error, Result};
use serde::Deserialize;

const GITHUB_API_BASE: &str = "https://api.github.com";
const X402_TOPIC: &str = "x402-template";

/// GitHub API response for repository search
#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<RepositoryInfo>,
}

/// Repository information from GitHub API
#[derive(Debug, Deserialize)]
struct RepositoryInfo {
    name: String,
    description: Option<String>,
    html_url: String,
    owner: Owner,
    stargazers_count: u32,
    language: Option<String>,
    topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
}

/// GitHub-based template discoverer
pub struct GitHubDiscovery {
    client: reqwest::Client,
}

impl GitHubDiscovery {
    /// Create a new GitHub discoverer
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Discover templates from GitHub
    pub async fn discover(&self) -> Result<Vec<TemplateInfo>> {
        let url = format!(
            "{}/search/repositories?q=topic:{}&sort=stars&order=desc&per_page=100",
            GITHUB_API_BASE, X402_TOPIC
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "cargo-x402")
            .send()
            .await
            .map_err(|e| Error::GitHubApiError(format!("Failed to fetch templates: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::GitHubApiError(format!(
                "GitHub API returned {}: {}",
                status, error_text
            )));
        }

        let search_response: SearchResponse = response
            .json()
            .await
            .map_err(|e| Error::GitHubApiError(format!("Failed to parse response: {}", e)))?;

        let templates = search_response
            .items
            .into_iter()
            .map(|repo| {
                let description = repo.description.unwrap_or_default();
                TemplateInfo {
                    name: if description.is_empty() {
                        repo.name.clone()
                    } else {
                        description.clone()
                    },
                    description,
                    url: repo.html_url,
                    owner: repo.owner.login,
                    repo: repo.name,
                    stars: repo.stargazers_count,
                    language: repo.language.unwrap_or_else(|| "Unknown".to_string()),
                    topics: repo.topics,
                }
            })
            .collect();

        Ok(templates)
    }

    /// Get a specific template by owner/repo
    pub async fn get_template(&self, owner: &str, repo: &str) -> Result<TemplateInfo> {
        let url = format!("{}/repos/{}/{}", GITHUB_API_BASE, owner, repo);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "cargo-x402")
            .send()
            .await
            .map_err(|e| Error::GitHubApiError(format!("Failed to fetch template: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::TemplateNotFound(format!("{}/{}", owner, repo)));
        }

        let repo_info: RepositoryInfo = response
            .json()
            .await
            .map_err(|e| Error::GitHubApiError(format!("Failed to parse response: {}", e)))?;

        let description = repo_info.description.unwrap_or_default();
        Ok(TemplateInfo {
            name: if description.is_empty() {
                repo_info.name.clone()
            } else {
                description.clone()
            },
            description,
            url: repo_info.html_url,
            owner: repo_info.owner.login,
            repo: repo_info.name,
            stars: repo_info.stargazers_count,
            language: repo_info.language.unwrap_or_else(|| "Unknown".to_string()),
            topics: repo_info.topics,
        })
    }
}

impl Default for GitHubDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_info_shorthand() {
        let template = TemplateInfo {
            name: "Test".to_string(),
            description: "test".to_string(),
            url: "https://github.com/user/repo".to_string(),
            owner: "user".to_string(),
            repo: "repo".to_string(),
            stars: 0,
            language: "Rust".to_string(),
            topics: vec![],
        };

        assert_eq!(template.shorthand(), "user/repo");
    }

    #[test]
    fn test_template_matches_tags() {
        let template = TemplateInfo {
            name: "Test".to_string(),
            description: "test".to_string(),
            url: "https://github.com/user/repo".to_string(),
            owner: "user".to_string(),
            repo: "repo".to_string(),
            stars: 0,
            language: "Rust".to_string(),
            topics: vec!["axum".to_string(), "database".to_string()],
        };

        assert!(template.matches_tags(&[]));
        assert!(template.matches_tags(&["axum".to_string()]));
        assert!(!template.matches_tags(&["mongodb".to_string()]));
    }
}
