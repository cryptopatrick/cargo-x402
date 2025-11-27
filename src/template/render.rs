//! Template rendering with Liquid

use crate::error::{Error, Result};
use liquid::model::Value;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// Renders templates using Liquid templating engine
pub struct Renderer;

impl Renderer {
    /// Render all template files with provided parameters
    pub fn render(
        template_path: &Path,
        output_path: &Path,
        parameters: &HashMap<String, String>,
    ) -> Result<()> {
        // Ensure output directory exists
        std::fs::create_dir_all(output_path).map_err(|e| {
            Error::FileSystemError(format!("Cannot create output directory: {}", e))
        })?;

        // Walk through template directory
        for entry in WalkDir::new(template_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .file_name()
                    .map(|n| n != ".git" && n != "x402.toml")
                    .unwrap_or(true)
            })
        {
            let rel_path = entry
                .path()
                .strip_prefix(template_path)
                .map_err(|e| Error::FileSystemError(e.to_string()))?;
            let dest_path = output_path.join(rel_path);

            if entry.path().is_dir() {
                std::fs::create_dir_all(&dest_path)
                    .map_err(|e| Error::FileSystemError(format!("Cannot create dir: {}", e)))?;
            } else {
                Self::render_file(entry.path(), &dest_path, parameters)?;
            }
        }

        Ok(())
    }

    /// Render a single file
    fn render_file(src: &Path, dest: &Path, parameters: &HashMap<String, String>) -> Result<()> {
        // Skip binary files
        if Self::is_binary_file(src) {
            std::fs::copy(src, dest)
                .map_err(|e| Error::FileSystemError(format!("Cannot copy file: {}", e)))?;
            return Ok(());
        }

        // Read file content
        let content = std::fs::read_to_string(src)
            .map_err(|e| Error::RenderError(format!("Cannot read file: {}", e)))?;

        // Render with Liquid
        let rendered = Self::render_content(&content, parameters)?;

        // Write rendered content
        std::fs::write(dest, rendered)
            .map_err(|e| Error::FileSystemError(format!("Cannot write file: {}", e)))?;

        Ok(())
    }

    /// Render content string with Liquid
    fn render_content(content: &str, parameters: &HashMap<String, String>) -> Result<String> {
        // Parse Liquid template
        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|e| Error::RenderError(format!("Failed to build parser: {}", e)))?
            .parse(content)
            .map_err(|e| Error::RenderError(format!("Failed to parse template: {}", e)))?;

        // Prepare globals map for rendering
        let mut globals = liquid::Object::new();
        for (key, value) in parameters {
            globals.insert(
                key.clone().into(),
                Value::scalar(value.clone()),
            );
        }

        // Render
        template
            .render(&globals)
            .map_err(|e| Error::RenderError(format!("Failed to render template: {}", e)))
    }

    /// Check if a file is binary
    fn is_binary_file(path: &Path) -> bool {
        let binary_extensions = ["png", "jpg", "jpeg", "gif", "ico", "bin", "zip", "tar", "gz"];

        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| binary_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_template() {
        let mut params = HashMap::new();
        params.insert("project_name".to_string(), "my-app".to_string());

        let content = "Project: {{ project_name }}";
        let result = Renderer::render_content(content, &params).unwrap();

        assert_eq!(result, "Project: my-app");
    }

    #[test]
    fn test_render_conditional_template() {
        let mut params = HashMap::new();
        params.insert("enable_docker".to_string(), "true".to_string());

        let content = "{% if enable_docker %}Docker enabled{% endif %}";
        let result = Renderer::render_content(content, &params).unwrap();

        assert_eq!(result, "Docker enabled");
    }

    #[test]
    fn test_is_binary_file() {
        assert!(Renderer::is_binary_file(Path::new("image.png")));
        assert!(Renderer::is_binary_file(Path::new("archive.zip")));
        assert!(!Renderer::is_binary_file(Path::new("main.rs")));
        assert!(!Renderer::is_binary_file(Path::new("Cargo.toml")));
    }
}
