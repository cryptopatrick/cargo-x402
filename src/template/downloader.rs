//! Template downloading from GitHub

use crate::error::{Error, Result};
use std::path::Path;
use walkdir::WalkDir;

/// Downloads and extracts templates
pub struct Downloader {
    client: reqwest::Client,
}

impl Downloader {
    /// Create a new downloader
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Download template from GitHub URL and extract to destination
    pub async fn download(&self, template_url: &str, dest: &Path) -> Result<()> {
        // Normalize template URL
        let url = Self::normalize_github_url(template_url)?;
        let zipball_url = Self::github_to_zipball_url(&url)?;

        // Download ZIP file
        let response = self
            .client
            .get(&zipball_url)
            .header("User-Agent", "cargo-x402")
            .send()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to download template: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::NetworkError(format!(
                "Failed to download template: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| Error::NetworkError(format!("Failed to read response: {}", e)))?;

        // Extract ZIP to temporary location first
        let temp_extract = tempfile::TempDir::new()
            .map_err(|e| Error::FileSystemError(format!("Cannot create temp dir: {}", e)))?;

        let zip_data = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(zip_data)
            .map_err(|e| Error::FileSystemError(format!("Invalid ZIP file: {}", e)))?;

        archive
            .extract(temp_extract.path())
            .map_err(|e| Error::FileSystemError(format!("Failed to extract ZIP: {}", e)))?;

        // The extracted directory has a format like {repo-commit}/, find it
        let extracted_dir = self.find_extracted_directory(temp_extract.path())?;

        // Create destination and copy files (except .git)
        std::fs::create_dir_all(dest)
            .map_err(|e| Error::FileSystemError(format!("Cannot create destination: {}", e)))?;

        Self::copy_tree(&extracted_dir, dest)?;

        Ok(())
    }

    /// Find the extracted directory (usually named {repo}-{hash})
    fn find_extracted_directory(&self, temp_path: &Path) -> Result<std::path::PathBuf> {
        for entry in std::fs::read_dir(temp_path)
            .map_err(|e| Error::FileSystemError(format!("Cannot read temp dir: {}", e)))?
        {
            let entry = entry
                .map_err(|e| Error::FileSystemError(format!("Cannot read entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                return Ok(path);
            }
        }

        Err(Error::FileSystemError(
            "No directory found in extracted archive".to_string(),
        ))
    }

    /// Recursively copy directory tree, excluding .git
    fn copy_tree(src: &Path, dest: &Path) -> Result<()> {
        for entry in WalkDir::new(src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().file_name().map(|n| n != ".git").unwrap_or(true))
        {
            let rel_path = entry
                .path()
                .strip_prefix(src)
                .map_err(|e| Error::FileSystemError(e.to_string()))?;
            let dest_path = dest.join(rel_path);

            if entry.path().is_dir() {
                std::fs::create_dir_all(&dest_path)
                    .map_err(|e| Error::FileSystemError(format!("Cannot create dir: {}", e)))?;
            } else {
                std::fs::copy(entry.path(), &dest_path).map_err(|e| {
                    Error::FileSystemError(format!("Cannot copy file: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Normalize GitHub URL (handle shorthand and full URLs)
    fn normalize_github_url(url: &str) -> Result<String> {
        // If it's a shorthand (user/repo), convert to full GitHub URL
        if !url.contains('/') {
            return Err(Error::ValidationError {
                field: "template".to_string(),
                message: "Template URL must be in format 'user/repo' or full GitHub URL".to_string(),
            });
        }

        if url.starts_with("https://github.com/") {
            return Ok(url.to_string());
        }

        if url.contains('/') && !url.contains("://") {
            // Assume it's a shorthand like "user/repo"
            return Ok(format!("https://github.com/{}", url));
        }

        Err(Error::ValidationError {
            field: "template".to_string(),
            message: "Template URL must be a GitHub URL or 'user/repo' shorthand".to_string(),
        })
    }

    /// Convert GitHub URL to zipball URL for download
    fn github_to_zipball_url(github_url: &str) -> Result<String> {
        // Expected format: https://github.com/owner/repo
        let url = github_url.trim_end_matches('/');

        if !url.starts_with("https://github.com/") {
            return Err(Error::ValidationError {
                field: "template".to_string(),
                message: "Must be a GitHub URL".to_string(),
            });
        }

        // Extract owner/repo
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() < 5 {
            return Err(Error::ValidationError {
                field: "template".to_string(),
                message: "Invalid GitHub URL format".to_string(),
            });
        }

        let owner = parts[3];
        let repo = parts[4];

        // GitHub zipball URL for main branch
        Ok(format!(
            "https://github.com/{}/{}/archive/refs/heads/main.zip",
            owner, repo
        ))
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_shorthand_url() {
        let result = Downloader::normalize_github_url("user/repo").unwrap();
        assert_eq!(result, "https://github.com/user/repo");
    }

    #[test]
    fn test_normalize_full_url() {
        let url = "https://github.com/user/repo";
        let result = Downloader::normalize_github_url(url).unwrap();
        assert_eq!(result, url);
    }

    #[test]
    fn test_github_to_zipball_url() {
        let result =
            Downloader::github_to_zipball_url("https://github.com/user/my-repo").unwrap();
        assert!(result.contains("user/my-repo"));
        assert!(result.contains("archive"));
        assert!(result.contains(".zip"));
    }
}
