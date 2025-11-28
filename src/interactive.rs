//! Interactive CLI prompts and user selection.
//!
//! This module provides a user-friendly terminal interface for:
//! - Selecting templates from a list
//! - Prompting for parameter values
//! - Displaying progress and success messages
//!
//! All prompts use the `dialoguer` crate for colored, formatted output
//! and support arrow-key navigation on supported terminals.
//!
//! ## Features
//!
//! - **Colored Output**: Professional-looking prompts with ANSI colors
//! - **Template Selection**: Interactive menu to browse and select templates
//! - **Parameter Prompts**: Type-aware prompts for string, boolean, and enum parameters
//! - **Validation Feedback**: Clear error messages for invalid input
//! - **Progress Indicators**: Visual feedback during template download and rendering
//!
//! ## Example
//!
//! ```no_run,ignore
//! use cargo_x402::interactive;
//! use cargo_x402::discovery::TemplateInfo;
//!
//! // See integration tests for actual usage examples
//! // Interactive prompts require terminal context
//! ```

use crate::discovery::TemplateInfo;
use crate::error::{Error, Result};
use crate::schema::Parameter;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::collections::HashMap;

/// Select a template from a list interactively.
///
/// Presents a navigable list of templates using arrow keys.
/// Returns the selected template or an error if the user cancels.
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
#[allow(dead_code)] // Intentionally public for library users
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

    // ToTitleCase Tests
    #[test]
    fn test_to_title_case() {
        assert_eq!("hello_world".to_title_case(), "Hello World");
        assert_eq!("project_name".to_title_case(), "Project Name");
        assert_eq!("db_type".to_title_case(), "Db Type");
    }

    #[test]
    fn test_to_title_case_single_word() {
        assert_eq!("name".to_title_case(), "Name");
        assert_eq!("database".to_title_case(), "Database");
    }

    #[test]
    fn test_to_title_case_multiple_underscores() {
        assert_eq!("my_project_name".to_title_case(), "My Project Name");
        assert_eq!("a_b_c".to_title_case(), "A B C");
    }

    #[test]
    fn test_to_title_case_empty_string() {
        assert_eq!("".to_title_case(), "");
    }

    #[test]
    fn test_to_title_case_only_underscores() {
        assert_eq!("___".to_title_case(), "   ");
    }

    // format_prompt Tests
    #[test]
    fn test_format_prompt_without_description() {
        let prompt = format_prompt("project_name", None);
        // to_title_case capitalizes first char: "project_name" -> "project name" -> "Project name"
        assert_eq!(prompt, "Project name");
    }

    #[test]
    fn test_format_prompt_with_description() {
        let prompt = format_prompt("database_type", Some("PostgreSQL or SQLite"));
        assert_eq!(prompt, "Database type (PostgreSQL or SQLite)");
    }

    #[test]
    fn test_format_prompt_single_word_with_description() {
        let prompt = format_prompt("port", Some("Server port number"));
        assert_eq!(prompt, "Port (Server port number)");
    }

    #[test]
    fn test_format_prompt_empty_description() {
        let prompt = format_prompt("name", Some(""));
        assert_eq!(prompt, "Name ()");
    }

    // Test select_template with empty list
    #[test]
    fn test_select_template_empty_list() {
        let result = select_template(&[]);
        assert!(result.is_err());
        match result {
            Err(Error::TemplateNotFound(msg)) => {
                assert_eq!(msg, "No templates available");
            }
            _ => panic!("Expected TemplateNotFound error"),
        }
    }

    // Test print functions (just verify they don't panic)
    #[test]
    fn test_print_success() {
        print_success("Operation completed");
        // Test passes if no panic
    }

    #[test]
    fn test_print_error() {
        print_error("An error occurred");
        // Test passes if no panic
    }

    #[test]
    fn test_print_warning() {
        print_warning("This is a warning");
        // Test passes if no panic
    }

    #[test]
    fn test_print_info() {
        print_info("Information message");
        // Test passes if no panic
    }

    #[test]
    fn test_print_next_steps() {
        print_next_steps("my-project");
        // Test passes if no panic
    }

    // Test format_prompt creates proper spacing
    #[test]
    fn test_format_prompt_formatting() {
        let prompt = format_prompt("my_long_field_name", Some("Help text"));
        // Underscores are replaced with spaces, then first char is capitalized
        assert_eq!(prompt, "My long field name (Help text)");
    }

    // Test that format_prompt correctly handles underscores
    #[test]
    fn test_format_prompt_underscore_handling() {
        let prompt = format_prompt("rust_version", Some(""));
        // After replace, becomes "rust version", to_title_case capitalizes first char
        assert_eq!(prompt, "Rust version ()");
    }

    // Test empty parameter name
    #[test]
    fn test_format_prompt_empty_name() {
        let prompt = format_prompt("", None);
        assert_eq!(prompt, "");
    }

    // Test that print functions accept various inputs
    #[test]
    fn test_print_functions_with_special_chars() {
        print_success("Success! ✨");
        print_error("Error! 🔴");
        print_warning("Warning! ⚠️");
        print_info("Info! ℹ️");
        // Test passes if no panic
    }

    // Test that next_steps displays all required steps
    #[test]
    fn test_print_next_steps_contains_all_steps() {
        // We can't capture stdout in a unit test easily,
        // but we verify the function exists and can be called
        print_next_steps("test-project");
        // Function should complete without error
    }

    // Test ToTitleCase trait implementation consistency
    #[test]
    fn test_to_title_case_consistency() {
        let input = "test_case_value";
        let result1 = input.to_title_case();
        let result2 = input.to_title_case();
        assert_eq!(result1, result2);
        assert_eq!(result1, "Test Case Value");
    }

    #[test]
    fn test_to_title_case_special_patterns() {
        // Test patterns that might appear in real parameter names
        assert_eq!("enable_feature".to_title_case(), "Enable Feature");
        assert_eq!("min_version".to_title_case(), "Min Version");
        assert_eq!("max_connections".to_title_case(), "Max Connections");
    }
}
