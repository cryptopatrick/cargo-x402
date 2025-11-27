# Template Author Guide

A comprehensive guide for creating and publishing custom templates for cargo-x402.

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [Template Structure](#template-structure)
- [Configuration (x402.toml)](#configuration-x402toml)
- [Parameters](#parameters)
- [File Rules](#file-rules)
- [Templating with Liquid](#templating-with-liquid)
- [Publishing Your Template](#publishing-your-template)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Overview

cargo-x402 is a template scaffolding tool that discovers and manages project templates from GitHub repositories tagged with the `x402-template` topic. This guide will help you create, test, and publish your own templates.

### What is a Template?

A template is a GitHub repository containing:
- **x402.toml**: Configuration file defining the template metadata and parameters
- **Template files**: Project source code with variable placeholders (Liquid syntax)
- **README.md**: Instructions for using the template
- **.gitignore**: Standard version control excludes

### Why Create a Template?

Templates allow you to:
- Provide standardized project structures
- Reduce setup time for new projects
- Enforce best practices and conventions
- Share reusable project patterns
- Distribute project configurations

## Getting Started

### 1. Create a Repository

```bash
git init my-x402-template
cd my-x402-template
```

### 2. Add Template Metadata

Create an `x402.toml` file:

```toml
[template]
name = "My Template"
description = "A description of what this template provides"
version = "0.1.0"
authors = ["Your Name"]
repository = "https://github.com/yourname/my-x402-template"
tags = ["tag1", "tag2"]

[template.min_versions]
rust = "1.70.0"
cargo_x402 = "0.1.0"

[parameters.project_name]
type = "string"
description = "Name of the project"
pattern = "^[a-z0-9][a-z0-9-]*[a-z0-9]$"
default = "my-project"

[files]
include = ["src/**", "Cargo.toml", "README.md"]
exclude = [".git", "target", "*.lock"]
```

### 3. Add Template Files

Create your project structure with Liquid placeholders:

```rust
// src/main.rs
fn main() {
    println!("Welcome to {{ project_name }}!");
}
```

```toml
# Cargo.toml
[package]
name = "{{ project_name }}"
version = "{{ version }}"
edition = "2021"
authors = ["{{ author }}"]
```

### 4. Add GitHub Topic

In your repository settings:
1. Go to Settings → About
2. Add topic: `x402-template`
3. Save changes

cargo-x402 will discover your template within 15 minutes.

## Template Structure

### Minimal Template

```
my-template/
├── x402.toml          # Required: Template metadata
├── Cargo.toml         # Template files with {{ placeholders }}
├── src/
│   └── main.rs        # Can use {{ variables }}
├── README.md          # Documentation
├── .gitignore         # Standard git excludes
└── .git/              # Git repository (required)
```

### Full-Featured Template

```
my-template/
├── x402.toml                 # Template configuration
├── Cargo.toml                # Rust project manifest
├── Cargo.lock                # Optional lock file
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library (optional)
│   └── modules/             # Code organization
├── tests/                    # Test directory
│   └── integration.rs
├── examples/                 # Example code
│   └── example.rs
├── migrations/               # Database migrations (if applicable)
│   └── 20240101_init.sql
├── .env.example             # Environment template
├── docker-compose.yml       # Container orchestration
├── Dockerfile               # Container image
├── .eslintrc.cjs            # Linting (if applicable)
├── tsconfig.json            # TypeScript config (if applicable)
├── README.md                # Comprehensive guide
├── ARCHITECTURE.md          # Design documentation
├── .gitignore               # Git excludes
└── .git/                    # Git repository
```

## Configuration (x402.toml)

### Template Section

Required metadata about your template:

```toml
[template]
name = "My Template"              # Display name
description = "What it does"      # One-line description
version = "0.1.0"                 # Semantic version
authors = ["Author Name"]         # Author(s)
repository = "https://github.com/..." # GitHub URL
tags = ["rust", "axum", "api"]    # Discovery tags
```

### Minimum Versions

Specify version requirements (optional):

```toml
[template.min_versions]
rust = "1.70.0"
cargo_x402 = "0.1.0"
```

Version format must be semantic: `MAJOR.MINOR.PATCH`

### File Rules

Control which files are included:

```toml
[files]
include = [
    "src/**",           # All Rust source files
    "Cargo.toml",       # Project manifest
    "README.md",        # Documentation
    "LICENSE",          # License file
    "Dockerfile",       # Container file
    ".env.example"      # Environment template
]

exclude = [
    ".git",             # Git metadata
    "target",           # Build artifacts
    "node_modules",     # NPM dependencies
    "*.lock",           # Lock files
    ".DS_Store",        # macOS metadata
    "Thumbs.db"         # Windows metadata
]
```

**Patterns:**
- `*.txt` - Matches all `.txt` files in root directory
- `src/**/*.rs` - Matches all `.rs` files recursively in `src/`
- `**/test/**` - Matches `test` directories anywhere
- `Cargo.*` - Matches `Cargo.toml` and `Cargo.lock`

## Parameters

Parameters allow users to customize templates during project creation. Define them in x402.toml:

### String Parameter

```toml
[parameters.project_name]
type = "string"
description = "Name of your project"
pattern = "^[a-z0-9][a-z0-9-]*[a-z0-9]$"  # Regex pattern (optional)
default = "my-project"                      # Default value (optional)
```

Usage in templates:
```
Project name: {{ project_name }}
```

### Boolean Parameter

```toml
[parameters.enable_database]
type = "boolean"
description = "Include database support?"
default = true
```

Usage in templates:
```liquid
{% if enable_database %}
Database configuration enabled
{% endif %}
```

### Enum Parameter

```toml
[parameters.environment]
type = "enum"
description = "Deployment environment"
options = ["development", "staging", "production"]
```

Usage in templates:
```liquid
{% if environment == "production" %}
Production-grade security settings
{% else %}
Development-mode logging
{% endif %}
```

## File Rules

### Include Patterns

Include files matching glob patterns:

```toml
include = [
    "src/**",           # Include all files in src/
    "Cargo.toml",       # Specific file
    "*.md",             # All markdown files in root
    "tests/**/*.rs",    # Rust test files
]
```

### Exclude Patterns

Exclude files (takes precedence):

```toml
exclude = [
    ".git",             # Version control
    "target",           # Build output
    "*.lock",           # Dependency locks
    ".env",             # Secrets (keep .env.example)
]
```

### Special Files

Files treated specially:

| File | Behavior |
|------|----------|
| `.git/` | Always excluded from template |
| `x402.toml` | Never rendered, always excluded |
| `.env` | Usually excluded for security |
| `.env.example` | Usually included, rendered |
| Binary files | Never rendered (`.png`, `.zip`, etc.) |

## Templating with Liquid

Templates use [Liquid](https://github.com/Shopify/liquid) syntax for variable substitution.

### Variables

Inject variables into your template files:

```rust
// src/main.rs
fn main() {
    println!("Project: {{ project_name }}");
    println!("Author: {{ author }}");
    println!("Version: {{ version }}");
    println!("Date: {{ date }}");
}
```

### Built-in Variables

Available to all templates:

- `{{ project_name }}` - User-provided project name
- `{{ author }}` - User-provided author name
- `{{ version }}` - Template version (from x402.toml)
- `{{ date }}` - Current date (ISO 8601 format)

### Custom Variables

Define custom variables in parameters:

```toml
[parameters.author_github]
type = "string"
description = "GitHub username"
default = "github-user"
```

Usage:
```markdown
Created by [@{{ author_github }}](https://github.com/{{ author_github }})
```

### Conditional Blocks

Show/hide content based on parameters:

```liquid
{% if enable_database %}
# Database Setup

## Connection String

Set your DATABASE_URL environment variable:
```bash
export DATABASE_URL="postgresql://user:pass@localhost/db"
```
{% endif %}
```

### Loops

Iterate over values:

```liquid
{% for env in environments %}
- {{ env }}
{% endfor %}
```

### Filters

Transform values:

```liquid
{{ project_name | upcase }}     # UPPERCASE
{{ project_name | downcase }}   # lowercase
{{ description | truncate: 20 }} # Truncate to 20 chars
```

### Escaping

Prevent rendering when you want literal braces:

```liquid
{% raw %}
Code example: {{ variable }}
{% endraw %}
```

## Publishing Your Template

### Pre-Publication Checklist

- [ ] Repository is public on GitHub
- [ ] Added `x402-template` topic to repository
- [ ] Created comprehensive README.md
- [ ] Included examples of all parameters
- [ ] Tested with `cargo-x402 create --template yourname/repo`
- [ ] All file patterns are correct
- [ ] Security files (.env) are excluded
- [ ] Version follows semantic versioning
- [ ] License is specified

### Publication Process

1. **Push to GitHub**
   ```bash
   git add .
   git commit -m "Initial template release"
   git push -u origin main
   ```

2. **Add Topic**
   - GitHub repository Settings → About
   - Add topic: `x402-template`
   - Save changes

3. **Discovery**
   - cargo-x402 discovers templates with topic `x402-template`
   - Discovery happens hourly
   - Your template is visible within 15 minutes

4. **User Access**
   ```bash
   cargo-x402 create --template your-github/your-template
   ```

### Updating Your Template

Push updates to your repository:

```bash
# Make changes
git add .
git commit -m "Update template with new feature"
git push origin main

# Update version in x402.toml
```

Users will see the latest version when they create projects.

## Best Practices

### Template Design

1. **Keep It Simple**: Focus on essentials, avoid over-engineering
2. **Follow Conventions**: Match language/ecosystem standards
3. **Document Everything**: Clear README and inline comments
4. **Provide Examples**: Include working code examples
5. **Validate Inputs**: Use regex patterns for string parameters

### File Organization

1. **Clear Structure**: Logical directory layout
2. **Minimize Files**: Only include necessary files
3. **Exclude Build Artifacts**: Use `.gitignore` effectively
4. **Separate Config**: Use `.env.example` for configuration

### Documentation

1. **README Coverage**:
   - Quick start guide
   - Installation instructions
   - Usage examples
   - Configuration reference
   - Common troubleshooting

2. **Comments**:
   - Explain non-obvious decisions
   - Reference external docs
   - Note version requirements

3. **Examples**:
   - Runnable code samples
   - Configuration templates
   - Deployment guides

### Version Management

Use semantic versioning:
- `MAJOR` for breaking changes
- `MINOR` for new features
- `PATCH` for bug fixes

Example: `0.1.0` → `0.2.0` (new feature) → `0.2.1` (bug fix)

### Naming Conventions

- Repository: `x402-template-<name>`
- Package: `<name>` (in Cargo.toml)
- Parameters: `snake_case`
- Tags: lowercase, hyphens allowed

### Security

1. **Never commit secrets**:
   - Exclude `.env` files
   - Use `.env.example` instead
   - Document required variables

2. **Validate user input**:
   - Use regex patterns
   - Enum known options
   - Range check numbers

3. **Safe defaults**:
   - Conservative security settings
   - Disable debug features
   - Use minimal dependencies

## Examples

### Minimal Rust API Template

**x402.toml**:
```toml
[template]
name = "Minimal Rust API"
version = "0.1.0"
authors = ["You"]
repository = "https://github.com/you/x402-minimal-api"
tags = ["rust", "api", "minimal"]

[parameters.project_name]
type = "string"
pattern = "^[a-z0-9-]+$"
default = "my-api"

[files]
include = ["src/**", "Cargo.toml", "README.md"]
exclude = [".git", "target"]
```

**Cargo.toml**:
```toml
[package]
name = "{{ project_name }}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Full-Stack Web Template

**x402.toml**:
```toml
[template]
name = "Full-Stack Web"
version = "0.1.0"
authors = ["You"]
repository = "https://github.com/you/x402-fullstack"
tags = ["rust", "react", "fullstack", "web"]

[parameters.project_name]
type = "string"
pattern = "^[a-z0-9-]+$"

[parameters.backend_port]
type = "string"
pattern = "^[0-9]{4,5}$"
default = "3000"

[parameters.frontend_port]
type = "string"
pattern = "^[0-9]{4,5}$"
default = "5173"

[parameters.use_database]
type = "boolean"
default = true

[files]
include = ["backend/**", "frontend/**", "docker-compose.yml"]
exclude = [".git", "target", "node_modules"]
```

## Troubleshooting

### Template Not Discovered

**Problem**: Template doesn't appear in `cargo-x402 list`

**Solutions**:
- [ ] Verify `x402-template` topic is added to repository
- [ ] Repository is public (not private)
- [ ] Repository exists on GitHub
- [ ] Wait 15+ minutes for discovery
- [ ] Try `cargo-x402 list --refresh` to bypass cache

### Variable Not Rendering

**Problem**: `{{ project_name }}` appears literally in output

**Solutions**:
- [ ] Check file is included in `[files]` section
- [ ] File is not binary (`.png`, `.zip`, etc.)
- [ ] Use valid Liquid syntax: `{{ variable }}`
- [ ] Variable is defined in `[parameters]` section
- [ ] No typos in variable names

### Parameter Validation Fails

**Problem**: User can't enter certain values

**Solutions**:
- [ ] Review regex pattern (test with online tools)
- [ ] Provide default value
- [ ] Add helpful description
- [ ] Enum options match exactly (case-sensitive)

### Files Not Included

**Problem**: Expected files are missing from generated project

**Solutions**:
- [ ] Check `[files.include]` patterns
- [ ] Patterns are relative to repository root
- [ ] Use `**` for recursive directories: `src/**`
- [ ] Exclude patterns take precedence
- [ ] Remove files from `[files.exclude]`

### Cargo.toml Parse Error

**Problem**: Generated Cargo.toml has invalid syntax

**Solutions**:
- [ ] Don't use `{{ }}` directly in Cargo.toml keys
- [ ] Place variables in values only: `name = "{{ project_name }}"`
- [ ] TOML is parsed before Liquid rendering
- [ ] Test with sample values

### Git History Removed

**Problem**: Generated projects don't have git history

**Solutions**:
- [ ] This is intentional behavior
- [ ] Templates initialize new git repository
- [ ] First commit contains scaffold
- [ ] User can selectively cherry-pick commits if needed

## Additional Resources

- [Liquid Template Language](https://shopify.github.io/liquid/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Topics](https://github.com/topics)
- [cargo-x402 Repository](https://github.com/cryptopatrick/cargo-x402)

## Getting Help

For questions or issues:

1. Check [cargo-x402 GitHub Issues](https://github.com/cryptopatrick/cargo-x402/issues)
2. Review this guide thoroughly
3. Look at [official templates](https://github.com/topics/x402-template) for examples
4. Open an issue with detailed information

---

Happy templating! 🚀
