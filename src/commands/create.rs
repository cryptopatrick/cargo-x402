//! Create a new project from a template

use crate::discovery::{Cache, GitHubDiscovery, TemplateInfo};
use crate::error::{Error, Result};
use crate::interactive as ui;
use crate::schema::Validator;
use crate::template::{Downloader, Renderer};
use colored::*;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::path::Path;

/// Execute the create command
pub async fn execute(template_arg: Option<String>, name_arg: Option<String>) -> Result<()> {
    // Step 1: Select or resolve template
    let template = if let Some(template_str) = template_arg {
        resolve_template(&template_str).await?
    } else {
        // Interactive mode: fetch and display available templates
        let templates = fetch_templates().await?;
        ui::select_template(&templates)?
    };

    println!(
        "\n{} Selected template: {} {}",
        "📦".cyan(),
        template.name.bold(),
        format!("({})", template.stars).dimmed()
    );

    // Step 2: Get project name
    let project_name = if let Some(name) = name_arg {
        name
    } else {
        let default_name = template.repo.replace('_', "-").to_lowercase();
        ui::prompt_project_name(Some(&default_name))?
    };

    // Check if directory already exists
    if Path::new(&project_name).exists() {
        return Err(Error::FileSystemError(format!(
            "Directory '{}' already exists",
            project_name
        )));
    }

    // Step 3: Download template
    println!("\n{} Downloading template...", "⬇️".cyan());
    let temp_dir = tempfile::TempDir::new()
        .map_err(|e| Error::FileSystemError(format!("Cannot create temp directory: {}", e)))?;

    let downloader = Downloader::new();
    downloader
        .download(&template.url, temp_dir.path())
        .await?;

    println!("{} Template downloaded", "✅".green());

    // Step 4: Load and validate schema
    println!("{} Validating template...", "🔍".cyan());
    let schema_path = temp_dir.path().join("x402.toml");

    if !schema_path.exists() {
        return Err(Error::InvalidSchema(
            "Template does not contain x402.toml".to_string(),
        ));
    }

    let schema = Validator::load_and_validate(&schema_path)?;
    println!("{} Template validated", "✅".green());

    // Step 5: Prompt for parameters if defined
    let mut parameters: HashMap<String, String> = HashMap::new();

    // Add default parameters
    parameters.insert("project_name".to_string(), project_name.clone());
    parameters.insert("author".to_string(), whoami::realname());
    parameters.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    parameters.insert(
        "date".to_string(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    // Prompt for custom parameters
    if let Some(schema_params) = &schema.parameters {
        if !schema_params.is_empty() {
            println!("\n{} Configure template parameters", "⚙️".cyan());
            let custom_params = ui::prompt_for_parameters(schema_params)?;
            parameters.extend(custom_params);
        }
    }

    // Step 6: Render templates
    println!("{} Rendering template files...", "✨".cyan());
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Processing files...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    Renderer::render(temp_dir.path(), Path::new(&project_name), &parameters)?;

    spinner.finish_and_clear();
    println!("{} Template rendered", "✅".green());

    // Step 7: Initialize git repository
    println!("{} Initializing git repository...", "🔧".cyan());
    initialize_git(&project_name)?;
    println!("{} Git repository initialized", "✅".green());

    // Step 8: Success message
    ui::print_success(&format!("Project created: {}", project_name));
    ui::print_next_steps(&project_name);

    Ok(())
}

/// Fetch templates from GitHub (with caching)
async fn fetch_templates() -> Result<Vec<TemplateInfo>> {
    let cache = Cache::new()?;

    // Try cache first
    if let Some(templates) = cache.load()? {
        if let Ok(Some(age)) = cache.age_hours() {
            ui::print_info(&format!(
                "Using cached templates ({}h old, run with --refresh to update)",
                age
            ));
        }
        return Ok(templates);
    }

    // Fetch from GitHub
    ui::print_info("Fetching templates from GitHub...");
    let discovery = GitHubDiscovery::new();

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Connecting to GitHub...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let templates = discovery.discover().await;
    spinner.finish_and_clear();

    let templates = templates?;

    // Save to cache
    if let Err(e) = cache.save(&templates) {
        ui::print_warning(&format!("Could not cache templates: {}", e));
    }

    Ok(templates)
}

/// Resolve a template reference (URL, shorthand, or name)
async fn resolve_template(template_ref: &str) -> Result<TemplateInfo> {
    // If it looks like a full GitHub URL or shorthand, use it directly
    if template_ref.starts_with("https://github.com/") || template_ref.contains('/') {
        // Parse owner/repo from shorthand or URL
        let (owner, repo) = if template_ref.starts_with("https://") {
            let parts: Vec<&str> = template_ref.split('/').collect();
            if parts.len() < 5 {
                return Err(Error::ValidationError {
                    field: "template".to_string(),
                    message: "Invalid GitHub URL".to_string(),
                });
            }
            (parts[3], parts[4])
        } else {
            let parts: Vec<&str> = template_ref.split('/').collect();
            if parts.len() != 2 {
                return Err(Error::ValidationError {
                    field: "template".to_string(),
                    message: "Template reference must be in format 'owner/repo'".to_string(),
                });
            }
            (parts[0], parts[1])
        };

        // Fetch template info from GitHub
        let discovery = GitHubDiscovery::new();
        return discovery.get_template(owner, repo).await;
    }

    // Otherwise, search for it in available templates
    let templates = fetch_templates().await?;
    templates
        .into_iter()
        .find(|t| t.repo == template_ref || t.name.to_lowercase() == template_ref.to_lowercase())
        .ok_or_else(|| Error::TemplateNotFound(template_ref.to_string()))
}

/// Initialize git repository in the new project
fn initialize_git(project_path: &str) -> Result<()> {
    use std::process::Command;

    Command::new("git")
        .args(&["init", project_path])
        .output()
        .map_err(|e| {
            Error::FileSystemError(format!("Failed to initialize git repository: {}", e))
        })?;

    // Add initial files
    Command::new("git")
        .args(&["-C", project_path, "add", "."])
        .output()
        .map_err(|e| Error::FileSystemError(format!("Failed to stage files: {}", e)))?;

    // Create initial commit
    Command::new("git")
        .args(&[
            "-C",
            project_path,
            "commit",
            "-m",
            "Initial commit from x402 template",
        ])
        .output()
        .map_err(|e| Error::FileSystemError(format!("Failed to create commit: {}", e)))?;

    Ok(())
}
