//! Template discovery caching with TTL

use super::TemplateInfo;
use crate::error::{Error, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CACHE_DIR_NAME: &str = "x402";
const CACHE_FILE_NAME: &str = "templates.json";
const DEFAULT_TTL_HOURS: i64 = 1;

/// Cached template list with timestamp
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedTemplates {
    /// Timestamp when cache was created
    pub last_updated: DateTime<Utc>,

    /// Cached template list
    pub templates: Vec<TemplateInfo>,
}

impl CachedTemplates {
    /// Check if cache is still valid
    pub fn is_fresh(&self, ttl_hours: i64) -> bool {
        let now = Utc::now();
        let ttl = Duration::hours(ttl_hours);
        now.signed_duration_since(self.last_updated) < ttl
    }

    /// Create new cache with current templates
    pub fn new(templates: Vec<TemplateInfo>) -> Self {
        Self {
            last_updated: Utc::now(),
            templates,
        }
    }
}

/// Cache for template discovery results
pub struct Cache {
    cache_dir: PathBuf,
    ttl_hours: i64,
}

impl Cache {
    /// Create a new cache instance
    pub fn new() -> Result<Self> {
        let cache_dir = Self::cache_directory()?;
        Ok(Self {
            cache_dir,
            ttl_hours: DEFAULT_TTL_HOURS,
        })
    }

    /// Create cache with custom TTL
    pub fn with_ttl(ttl_hours: i64) -> Result<Self> {
        let cache_dir = Self::cache_directory()?;
        Ok(Self { cache_dir, ttl_hours })
    }

    /// Get cache directory path
    fn cache_directory() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| Error::CacheError("Cannot determine cache directory".to_string()))?
            .join(CACHE_DIR_NAME);

        // Create cache directory if it doesn't exist
        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| Error::CacheError(format!("Cannot create cache directory: {}", e)))?;

        Ok(cache_dir)
    }

    /// Get cache file path
    fn cache_file_path(&self) -> PathBuf {
        self.cache_dir.join(CACHE_FILE_NAME)
    }

    /// Load templates from cache if fresh
    pub fn load(&self) -> Result<Option<Vec<TemplateInfo>>> {
        let cache_path = self.cache_file_path();

        // If cache file doesn't exist, return None
        if !cache_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&cache_path)
            .map_err(|e| Error::CacheError(format!("Cannot read cache: {}", e)))?;

        let cached: CachedTemplates = serde_json::from_str(&content)
            .map_err(|e| Error::CacheError(format!("Invalid cache format: {}", e)))?;

        // Check if cache is still fresh
        if cached.is_fresh(self.ttl_hours) {
            Ok(Some(cached.templates))
        } else {
            Ok(None)
        }
    }

    /// Save templates to cache
    pub fn save(&self, templates: &[TemplateInfo]) -> Result<()> {
        let cached = CachedTemplates::new(templates.to_vec());

        let content = serde_json::to_string_pretty(&cached)
            .map_err(|e| Error::CacheError(format!("Cannot serialize cache: {}", e)))?;

        let cache_path = self.cache_file_path();
        std::fs::write(&cache_path, content)
            .map_err(|e| Error::CacheError(format!("Cannot write cache: {}", e)))?;

        Ok(())
    }

    /// Clear the cache
    pub fn clear(&self) -> Result<()> {
        let cache_path = self.cache_file_path();
        if cache_path.exists() {
            std::fs::remove_file(&cache_path)
                .map_err(|e| Error::CacheError(format!("Cannot delete cache: {}", e)))?;
        }
        Ok(())
    }

    /// Get cache age in hours
    pub fn age_hours(&self) -> Result<Option<i64>> {
        let cache_path = self.cache_file_path();

        if !cache_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&cache_path)
            .map_err(|e| Error::CacheError(format!("Cannot read cache: {}", e)))?;

        let cached: CachedTemplates = serde_json::from_str(&content)
            .map_err(|e| Error::CacheError(format!("Invalid cache format: {}", e)))?;

        let age = Utc::now().signed_duration_since(cached.last_updated);
        Ok(Some(age.num_hours()))
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback in case cache directory fails
            Self {
                cache_dir: PathBuf::from("/tmp/x402-cache"),
                ttl_hours: DEFAULT_TTL_HOURS,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cached_templates_is_fresh() {
        let templates = vec![];
        let cached = CachedTemplates::new(templates);

        assert!(cached.is_fresh(1)); // Should be fresh within 1 hour
        assert!(cached.is_fresh(24)); // Should be fresh within 24 hours
    }

    #[test]
    fn test_cached_templates_is_stale() {
        let templates = vec![];
        let mut cached = CachedTemplates::new(templates);

        // Artificially age the cache
        cached.last_updated = Utc::now() - Duration::hours(2);

        assert!(!cached.is_fresh(1)); // Should be stale with 1-hour TTL
        assert!(cached.is_fresh(3)); // Should still be fresh with 3-hour TTL
    }
}
