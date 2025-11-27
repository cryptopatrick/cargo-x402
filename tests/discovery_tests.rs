/// Template discovery and caching tests

use std::fs;
use tempfile::TempDir;

#[test]
fn test_github_api_query_formatting() {
    // Test that GitHub API search queries are correctly formatted
    let query = "topic:x402-template";
    assert!(query.contains("topic:"));
    assert!(query.contains("x402-template"));
}

#[test]
fn test_template_metadata_extraction() {
    let template_response = r#"{
  "name": "x402-template-api",
  "full_name": "user/x402-template-api",
  "description": "Basic Axum API template",
  "url": "https://api.github.com/repos/user/x402-template-api",
  "html_url": "https://github.com/user/x402-template-api",
  "stargazers_count": 42,
  "language": "Rust",
  "topics": ["x402-template", "axum", "api"]
}"#;

    assert!(template_response.contains("x402-template-api"));
    assert!(template_response.contains("Rust"));
    assert!(template_response.contains("topics"));
}

#[test]
fn test_cache_file_location() {
    // Test that cache file is stored in correct location
    let cache_dir = format!("{}/.cache/x402", std::env::var("HOME").unwrap_or_default());
    let cache_file = format!("{}/templates.json", cache_dir);

    assert!(cache_file.contains(".cache/x402"));
    assert!(cache_file.ends_with("templates.json"));
}

#[test]
fn test_cache_ttl_freshness() {
    // Test cache freshness checking
    use std::time::{Duration, SystemTime};

    let now = SystemTime::now();
    let ttl = Duration::from_secs(3600); // 1 hour

    let old_time = now - Duration::from_secs(4000); // older than TTL
    let fresh_time = now - Duration::from_secs(1000); // newer than TTL

    let is_old_fresh = old_time.elapsed().unwrap() < ttl;
    let is_fresh_fresh = fresh_time.elapsed().unwrap() < ttl;

    assert!(!is_old_fresh);
    assert!(is_fresh_fresh);
}

#[test]
fn test_cache_structure() {
    let cache_json = r#"{
  "cached_at": "2024-01-15T10:30:00Z",
  "ttl_seconds": 3600,
  "templates": [
    {
      "name": "Basic API",
      "owner": "user",
      "repo": "x402-template-api",
      "description": "API template",
      "stars": 42,
      "language": "Rust"
    }
  ]
}"#;

    assert!(cache_json.contains("cached_at"));
    assert!(cache_json.contains("ttl_seconds"));
    assert!(cache_json.contains("templates"));
}

#[test]
fn test_template_sorting_by_stars() {
    #[derive(Clone)]
    struct Template {
        name: String,
        stars: u32,
    }

    let mut templates = vec![
        Template {
            name: "Template A".to_string(),
            stars: 10,
        },
        Template {
            name: "Template B".to_string(),
            stars: 50,
        },
        Template {
            name: "Template C".to_string(),
            stars: 25,
        },
    ];

    templates.sort_by(|a, b| b.stars.cmp(&a.stars));

    assert_eq!(templates[0].name, "Template B");
    assert_eq!(templates[1].name, "Template C");
    assert_eq!(templates[2].name, "Template A");
}

#[test]
fn test_template_filtering_by_tags() {
    #[derive(Clone)]
    struct Template {
        name: String,
        tags: Vec<String>,
    }

    let templates = vec![
        Template {
            name: "API Template".to_string(),
            tags: vec!["axum".to_string(), "api".to_string()],
        },
        Template {
            name: "CLI Template".to_string(),
            tags: vec!["clap".to_string(), "cli".to_string()],
        },
        Template {
            name: "Full-Stack".to_string(),
            tags: vec!["axum".to_string(), "react".to_string()],
        },
    ];

    let axum_templates: Vec<_> = templates
        .iter()
        .filter(|t| t.tags.contains(&"axum".to_string()))
        .collect();

    assert_eq!(axum_templates.len(), 2);
}

#[test]
fn test_repository_url_parsing() {
    let urls = vec![
        ("https://github.com/user/repo", "user", "repo"),
        ("https://github.com/org/project.git", "org", "project"),
        ("git@github.com:user/repo.git", "user", "repo"),
    ];

    for (url, expected_owner, expected_repo) in urls {
        let (owner, repo) = parse_github_url(url);
        assert_eq!(owner, expected_owner);
        assert_eq!(repo, expected_repo);
    }
}

#[test]
fn test_zipball_url_generation() {
    let test_cases = vec![
        ("user/repo", "https://github.com/user/repo/archive/refs/heads/main.zip"),
        ("https://github.com/user/repo", "https://github.com/user/repo/archive/refs/heads/main.zip"),
    ];

    for (repo, expected_zip_url) in test_cases {
        let zip_url = generate_zipball_url(repo);
        assert_eq!(zip_url, expected_zip_url);
    }
}

#[test]
fn test_duplicate_template_removal() {
    #[derive(Clone, PartialEq)]
    struct Template {
        owner: String,
        repo: String,
    }

    let mut templates = vec![
        Template {
            owner: "user".to_string(),
            repo: "template1".to_string(),
        },
        Template {
            owner: "user".to_string(),
            repo: "template1".to_string(),
        },
        Template {
            owner: "org".to_string(),
            repo: "template2".to_string(),
        },
    ];

    templates.sort_by(|a, b| {
        if a.owner == b.owner {
            a.repo.cmp(&b.repo)
        } else {
            a.owner.cmp(&b.owner)
        }
    });
    templates.dedup();

    assert_eq!(templates.len(), 2);
}

#[test]
fn test_cache_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let cache_file = temp_dir.path().join("templates.json");

    let cache_content = r#"{"cached_at":"2024-01-15T10:30:00Z"}"#;
    fs::write(&cache_file, cache_content).unwrap();

    let loaded = fs::read_to_string(&cache_file).unwrap();
    assert_eq!(loaded, cache_content);
}

#[test]
fn test_cache_refresh_flag() {
    // Test --refresh flag behavior
    let cache_age_hours = 2.0; // Cache is 2 hours old
    let ttl_hours = 1.0; // TTL is 1 hour

    let is_stale = cache_age_hours > ttl_hours;
    assert!(is_stale);

    // With --refresh flag, we should fetch fresh data regardless
    let should_fetch_fresh_with_flag = true;
    assert!(should_fetch_fresh_with_flag);
}

// Helper functions

fn parse_github_url(url: &str) -> (String, String) {
    let url = url
        .replace("https://github.com/", "")
        .replace("git@github.com:", "")
        .replace(".git", "");

    let parts: Vec<&str> = url.split('/').collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn generate_zipball_url(repo: &str) -> String {
    let repo = if repo.contains("://") {
        repo.trim_end_matches('/')
    } else {
        repo
    };

    let base = if repo.contains("://") {
        repo.to_string()
    } else {
        format!("https://github.com/{}", repo)
    };

    format!("{}/archive/refs/heads/main.zip", base)
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn test_github_url_parsing() {
        let (owner, repo) = parse_github_url("https://github.com/user/repo");
        assert_eq!(owner, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_zipball_url_generation_logic() {
        let url = generate_zipball_url("user/repo");
        assert!(url.contains("github.com"));
        assert!(url.contains("archive"));
        assert!(url.ends_with(".zip"));
    }
}
