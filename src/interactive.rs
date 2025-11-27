//! Interactive user prompts and selection

use crate::discovery::TemplateInfo;
use crate::error::{Error, Result};
use crate::schema::Parameter;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::collections::HashMap;

/// Select a template from a list
pub fn select_template(templates: &[TemplateInfo]) -> Result<TemplateInfo> {
    if templates.is_empty() {
        return Err(Error::TemplateNotFound(
            "No templates available".to_string(),
        ));
    }

    let items: Vec<String> = templates
        .iter()
        .map(|t| format!("{:<20} {:<40} ⭐ {}", t.owner, t.name, t.stars))
        .collect();

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("Select a template")
        .default(0)
        .items(&items)
        .interact()
        .map_err(|_| Error::Cancelled)?;

    Ok(templates[selection].clone())
}

/// Prompt for a project name with validation
pub fn prompt_project_name(default: Option<&str>) -> Result<String> {
    let theme = ColorfulTheme::default();
    let mut input = Input::with_theme(&theme)
        .with_prompt("Project name")
        .validate_with(|name: &String| {
            if name.is_empty() {
                Err("Project name cannot be empty")
            } else if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                Err("Project name can only contain alphanumeric characters, dashes, and underscores")
            } else {
                Ok(())
            }
        });

    if let Some(default_name) = default {
        input = input.default(default_name.to_string());
    }

    input.interact_text().map_err(|_| Error::Cancelled)
}

/// Prompt for parameter values based on template parameters
pub fn prompt_for_parameters(parameters: &HashMap<String, Parameter>) -> Result<HashMap<String, String>> {
    let mut values = HashMap::new();

    for (name, param) in parameters {
        let value = prompt_for_parameter(name, param)?;
        values.insert(name.clone(), value);
    }

    Ok(values)
}

/// Prompt for a single parameter value
fn prompt_for_parameter(name: &str, param: &Parameter) -> Result<String> {
    let theme = ColorfulTheme::default();

    match param {
        Parameter::String {
            default,
            pattern: ref pattern_opt,
            description,
        } => {
            let mut input = Input::with_theme(&theme)
                .with_prompt(format_prompt(name, description.as_deref()))
                .default(default.clone());

            if let Some(pattern) = pattern_opt {
                let pattern_clone = pattern.clone();
                input = input.validate_with(move |value: &String| {
                    let test_param = crate::schema::Parameter::String {
                        default: value.clone(),
                        pattern: Some(pattern_clone.clone()),
                        description: None,
                    };
                    if let Err(e) = test_param.validate(value) {
                        Err(e)
                    } else {
                        Ok(())
                    }
                });
            }

            input.interact_text().map_err(|_| Error::Cancelled)
        }

        Parameter::Boolean { default, description } => {
            let theme = ColorfulTheme::default();
            let choices = vec!["Yes", "No"];
            let selection = Select::with_theme(&theme)
                .with_prompt(format_prompt(name, description.as_deref()))
                .default(if *default { 0 } else { 1 })
                .items(&choices)
                .interact()
                .map_err(|_| Error::Cancelled)?;

            Ok((selection == 0).to_string())
        }

        Parameter::Enum {
            choices,
            default,
            description,
        } => {
            let default_idx = choices
                .iter()
                .position(|c| c == default)
                .unwrap_or(0);

            let selection = Select::with_theme(&theme)
                .with_prompt(format_prompt(name, description.as_deref()))
                .default(default_idx)
                .items(choices)
                .interact()
                .map_err(|_| Error::Cancelled)?;

            Ok(choices[selection].clone())
        }
    }
}

/// Format prompt text with description
fn format_prompt(name: &str, description: Option<&str>) -> String {
    let formatted_name = name.replace('_', " ").to_title_case();

    match description {
        Some(desc) => format!("{} ({})", formatted_name, desc),
        None => formatted_name,
    }
}

/// Display a success message
pub fn print_success(message: &str) {
    println!("\n{} {}", "✅".green(), message.green());
}

/// Display an error message
pub fn print_error(message: &str) {
    println!("{} {}", "❌".red(), message.red());
}

/// Display a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message.yellow());
}

/// Display an info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message);
}

/// Display next steps
pub fn print_next_steps(project_name: &str) {
    println!("\n{}", "Next steps:".cyan().bold());
    println!("  {} cd {}", "$".dimmed(), project_name);
    println!("  {} Configure your .env file", "$".dimmed());
    println!("  {} cargo build", "$".dimmed());
    println!("  {} cargo run", "$".dimmed());
}

trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_title_case() {
        assert_eq!("hello_world".to_title_case(), "Hello World");
        assert_eq!("project_name".to_title_case(), "Project Name");
        assert_eq!("db_type".to_title_case(), "Db Type");
    }
}
