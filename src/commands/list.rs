//! List available templates

use crate::discovery::{Cache, GitHubDiscovery};
use crate::error::Result;
use crate::interactive as ui;
use colored::*;
use indicatif::ProgressBar;

/// Execute the list command
pub async fn execute(refresh: bool, tags: Option<Vec<String>>) -> Result<()> {
    // Initialize cache
    let cache = Cache::new()?;

    // Load templates from cache or GitHub
    let templates = if refresh {
        ui::print_info("Refreshing template list...");
        load_from_github(&cache).await?
    } else {
        // Try cache first
        match cache.load()? {
            Some(templates) => {
                if let Ok(Some(age)) = cache.age_hours() {
                    ui::print_info(&format!("Using cached templates ({}h old, use --refresh to update)", age));
                }
                templates
            }
            None => {
                ui::print_info("Loading templates from GitHub...");
                load_from_github(&cache).await?
            }
        }
    };

    if templates.is_empty() {
        ui::print_warning("No templates found");
        return Ok(());
    }

    // Filter by tags if provided
    let filtered_templates = if let Some(ref tag_filter) = tags {
        templates
            .into_iter()
            .filter(|t| t.matches_tags(tag_filter))
            .collect()
    } else {
        templates
    };

    if filtered_templates.is_empty() {
        ui::print_warning(&format!(
            "No templates found matching tags: {}",
            tags.unwrap().join(", ")
        ));
        return Ok(());
    }

    // Display templates
    println!("\n{}", "Available x402 Templates".cyan().bold());
    println!("{}", "─".repeat(100));
    println!(
        "{:<25} {:<50} {:<10} {:<10}",
        "NAME", "DESCRIPTION", "STARS", "LANGUAGE"
    );
    println!("{}", "─".repeat(100));

    for template in &filtered_templates {
        let name = format!("{}‌/{}", template.owner, template.repo);
        let description = if template.description.len() > 47 {
            format!("{}…", &template.description[..47])
        } else {
            template.description.clone()
        };

        println!(
            "{:<25} {:<50} {:<10} {:<10}",
            name.cyan(),
            description,
            template.stars.to_string().yellow(),
            template.language
        );
    }

    println!("{}", "─".repeat(100));
    println!(
        "\nTip: Use {} to create a project from a template",
        "cargo-x402 create".bold()
    );
    println!(
        "Tip: Filter by tags with {}",
        "--tags axum,database".bold()
    );

    Ok(())
}

/// Load templates from GitHub and cache them
async fn load_from_github(cache: &Cache) -> Result<Vec<crate::discovery::TemplateInfo>> {
    let discovery = GitHubDiscovery::new();
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Fetching templates from GitHub...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let result = discovery.discover().await;
    spinner.finish_and_clear();

    let templates = result?;

    // Save to cache
    if let Err(e) = cache.save(&templates) {
        ui::print_warning(&format!("Failed to cache templates: {}", e));
    }

    Ok(templates)
}
