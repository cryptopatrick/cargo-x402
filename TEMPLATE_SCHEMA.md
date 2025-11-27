# x402 Template Schema Specification

This document defines the **x402.toml** format that every x402 template must include.

## Overview

Every x402 template repository must contain an **x402.toml** file in the root directory. This file declares template metadata, version constraints, and customizable parameters.

```toml
[template]
name = "x402 Axum Starter"
description = "Full-featured payment API with Axum and PostgreSQL"
version = "1.0.0"
authors = ["John Doe"]
repository = "https://github.com/user/x402-template-axum"
tags = ["axum", "payment-api", "database"]
min_rust_version = "1.70"
min_x402_cli_version = "0.1.0"

[parameters]
project_name = { type = "string", default = "my-project", pattern = "^[a-z][a-z0-9-]*$" }
author = { type = "string", default = "Your Name" }
enable_docker = { type = "boolean", default = true }
db_type = { type = "string", enum = ["postgres", "sqlite"], default = "postgres" }

[files]
include = ["src/**/*", "Cargo.toml", ".env.example", "docker-compose.yml"]
exclude = ["target/**/*", ".git/**/*", "*.log", "node_modules/**/*"]
```

---

## Section: [template]

The **[template]** section is **REQUIRED** and contains template metadata.

### Required Fields

#### `name` (string)
**Human-readable name of the template.**

- **Type:** String
- **Required:** Yes
- **Length:** 1-100 characters
- **Examples:**
  ```toml
  name = "x402 Axum API"
  name = "Full-stack Payment App"
  ```

#### `description` (string)
**One-line description of what the template provides.**

- **Type:** String
- **Required:** Yes
- **Length:** 10-200 characters
- **Examples:**
  ```toml
  description = "Express server with x402 payment middleware"
  description = "Production-ready microservice with observability"
  ```

#### `version` (string)
**Semantic version of the template.**

- **Type:** String
- **Required:** Yes
- **Format:** Must follow Semantic Versioning 2.0.0 (MAJOR.MINOR.PATCH)
- **Examples:**
  ```toml
  version = "1.0.0"
  version = "0.5.2"
  version = "2.1.0-alpha"
  ```
- **Why it matters:** Users can pin to specific template versions for reproducibility

#### `authors` (array of strings)
**List of template authors/maintainers.**

- **Type:** Array of strings
- **Required:** Yes
- **Length:** At least 1 author
- **Format:** Can include email: `"John Doe <john@example.com>"`
- **Examples:**
  ```toml
  authors = ["John Doe"]
  authors = ["Jane Smith <jane@coinbase.com>", "Bob Johnson"]
  ```

#### `repository` (string)
**HTTPS URL to the template's GitHub repository.**

- **Type:** String (URL)
- **Required:** Yes
- **Format:** Must be HTTPS GitHub URL
- **Examples:**
  ```toml
  repository = "https://github.com/user/x402-template-axum"
  repository = "https://github.com/coinbase/x402-templates-starter-kit"
  ```
- **Validation:** Must start with `https://github.com/`

### Optional Fields

#### `tags` (array of strings)
**Searchable tags to categorize the template.**

- **Type:** Array of strings
- **Required:** No
- **Length:** 0-10 tags, each 1-20 characters
- **Purpose:** Help users find templates via filtering
- **Examples:**
  ```toml
  tags = ["axum", "payment-api"]
  tags = ["typescript", "react", "nextjs", "payments"]
  tags = ["cli", "x402", "merchant"]
  ```
- **Recommended tags:**
  - Framework: `axum`, `express`, `actix`, `hono`
  - Language: `rust`, `typescript`, `python`
  - Features: `database`, `authentication`, `api`, `cli`, `microservice`
  - Type: `fullstack`, `backend`, `frontend`, `example`

#### `min_rust_version` (string)
**Minimum Rust version required by the template.**

- **Type:** String (Semantic version)
- **Required:** No
- **Default:** None (any version)
- **Format:** MAJOR.MINOR.PATCH (e.g., "1.70.0")
- **Example:**
  ```toml
  min_rust_version = "1.70.0"
  ```
- **Usage:** cargo-x402 will warn users if their Rust version is too old

#### `min_x402_cli_version` (string)
**Minimum cargo-x402 CLI version required to use this template.**

- **Type:** String (Semantic version)
- **Required:** No
- **Default:** None (any version)
- **Example:**
  ```toml
  min_x402_cli_version = "0.1.0"
  ```
- **Usage:** Allows templates to depend on CLI features
- **Validation:** cargo-x402 will error if version is too old

---

## Section: [parameters]

The **[parameters]** section is **OPTIONAL** and defines customizable variables that users can provide.

If omitted, templates use only default variables: `{{ project_name }}`, `{{ author }}`, `{{ version }}`, `{{ date }}`

### Parameter Types

All parameters must be TOML inline tables with specific structure for each type.

#### 1. String Parameter

**Define:** User can provide any text string

```toml
[parameters]
my_param = { type = "string", default = "value" }
```

**Valid attributes:**
- `type`: Must be `"string"`
- `default`: Default value if user doesn't provide one
- `pattern`: (Optional) Regex pattern for validation
- `description`: (Optional) Help text shown to user

**Examples:**

```toml
[parameters]
# Simple string
author = { type = "string", default = "Your Name" }

# String with pattern validation (project names)
project_name = { type = "string", default = "my-project", pattern = "^[a-z][a-z0-9-]*$" }

# String with description
company = { type = "string", default = "Acme Inc", description = "Your company name" }
```

**Usage in templates:**
```liquid
Author: {{ author }}
Project: {{ project_name }}
```

#### 2. Boolean Parameter

**Define:** User chooses true/false

```toml
[parameters]
my_param = { type = "boolean", default = false }
```

**Valid attributes:**
- `type`: Must be `"boolean"`
- `default`: `true` or `false`
- `description`: (Optional) Help text

**Examples:**

```toml
[parameters]
enable_docker = { type = "boolean", default = true }
enable_tests = { type = "boolean", default = false }
include_monitoring = { type = "boolean", default = true, description = "Include Prometheus metrics" }
```

**Usage in templates:**
```liquid
{% if enable_docker %}
# Docker is enabled
FROM rust:latest
{% endif %}
```

#### 3. Enum Parameter

**Define:** User chooses from predefined options

```toml
[parameters]
my_param = { type = "enum", enum = ["option1", "option2"], default = "option1" }
```

**Valid attributes:**
- `type`: Must be `"enum"`
- `enum`: Array of valid choices (2-10 options)
- `default`: Must be one of the enum values
- `description`: (Optional) Help text

**Examples:**

```toml
[parameters]
# Database choice
db_type = { type = "enum", enum = ["postgres", "sqlite", "mysql"], default = "postgres" }

# License choice
license = { type = "enum", enum = ["MIT", "Apache-2.0", "GPL-3.0"], default = "MIT" }

# Environment
env = { type = "enum", enum = ["dev", "staging", "prod"], default = "dev", description = "Deployment environment" }
```

**Usage in templates:**
```liquid
{% if db_type == "postgres" %}
# PostgreSQL configuration
DATABASE_URL=postgresql://...
{% elsif db_type == "sqlite" %}
# SQLite configuration
DATABASE_URL=sqlite://db.sqlite
{% endif %}
```

### Default Variables (Always Available)

Users can use these variables without declaring them in `[parameters]`:

```toml
[parameters]
# These are automatically available:
# {{ project_name }}    - The project name provided by user
# {{ author }}          - Author name (user provides or default)
# {{ version }}         - Current CLI version
# {{ date }}            - Current date in ISO format
# {{ timestamp }}       - Current Unix timestamp
```

---

## Section: [files]

The **[files]** section is **OPTIONAL** and controls which template files are included/excluded.

### `include` (array of glob patterns)
**Files to include in the template.**

- **Type:** Array of glob patterns
- **Required:** No
- **Default:** All files except those matching `exclude`
- **Patterns:** Standard glob syntax (`**/*`, `*.rs`, etc.)

**Examples:**
```toml
[files]
include = ["src/**/*", "Cargo.toml", ".env.example", "README.md"]
```

### `exclude` (array of glob patterns)
**Files to exclude from the template.**

- **Type:** Array of glob patterns
- **Required:** No
- **Default:** `["target/**/*", ".git/**/*", "*.log"]`
- **Patterns:** Standard glob syntax

**Examples:**
```toml
[files]
exclude = ["target/**/*", ".git/**/*", "*.log", "node_modules/**/*", ".DS_Store"]
```

### Processing Rules

1. Files matching `include` patterns are included
2. Files matching `exclude` patterns are skipped
3. If `include` is not specified: all files (except excluded) are included
4. If both `include` and `exclude` specified: include wins first, then exclude filters

---

## Complete Example: Full-Featured Template

```toml
[template]
name = "x402 Full-Stack Starter Kit"
description = "Complete payment API with database, auth, and React frontend"
version = "1.2.0"
authors = ["Alice Smith <alice@example.com>", "Bob Johnson"]
repository = "https://github.com/coinbase/x402-templates-fullstack"
tags = ["fullstack", "axum", "react", "postgres", "payment-api"]
min_rust_version = "1.70.0"
min_x402_cli_version = "0.1.0"

[parameters]
# Project identification
project_name = {
    type = "string",
    default = "my-app",
    pattern = "^[a-z][a-z0-9-]*$",
    description = "Your project name (lowercase, alphanumeric and dashes only)"
}

author = {
    type = "string",
    default = "Your Name",
    description = "Your name (for Cargo.toml)"
}

# Feature flags
enable_docker = {
    type = "boolean",
    default = true,
    description = "Include Docker/docker-compose configuration"
}

include_tests = {
    type = "boolean",
    default = true,
    description = "Include integration tests"
}

# Configuration choices
db_type = {
    type = "enum",
    enum = ["postgres", "sqlite"],
    default = "postgres",
    description = "Database engine to use"
}

auth_provider = {
    type = "enum",
    enum = ["none", "jwt", "session"],
    default = "jwt",
    description = "Authentication method"
}

[files]
include = [
    "src/**/*",
    "frontend/**/*",
    "Cargo.toml",
    "package.json",
    ".env.example",
    ".gitignore",
    "docker-compose.yml",
    "README.md"
]

exclude = [
    "target/**/*",
    "node_modules/**/*",
    ".git/**/*",
    "*.log",
    ".DS_Store",
    "dist/**/*",
    "build/**/*"
]
```

---

## Validation Rules

The cargo-x402 CLI validates templates against these rules:

### Schema Validation

1. ✓ File must be valid TOML
2. ✓ `[template]` section must exist
3. ✓ Required fields (`name`, `description`, `version`, `authors`, `repository`) must be present
4. ✓ Field types must match specification
5. ✓ `version` must follow Semantic Versioning
6. ✓ `repository` must be HTTPS GitHub URL
7. ✓ `authors` array must not be empty
8. ✓ Pattern strings (if provided) must be valid regex
9. ✓ Enum choices must not be empty
10. ✓ Default values must match parameter type

### Runtime Validation

1. ✓ User-provided parameter values must match patterns/enums
2. ✓ `min_rust_version` must be ≤ user's Rust version
3. ✓ `min_x402_cli_version` must be ≤ cargo-x402 version

### File Validation

1. ✓ Glob patterns must be valid
2. ✓ At least some files must match include/exclude rules
3. ✓ `.git` directory is always removed (even if in include)
4. ✓ `x402.toml` is never rendered (always copied as-is)

---

## Best Practices

### 1. **Clear Names and Descriptions**
```toml
# ✓ Good
name = "x402 Axum with PostgreSQL"
description = "Production-ready payment API using Axum framework and PostgreSQL"

# ✗ Avoid
name = "template"
description = "A template"
```

### 2. **Meaningful Tags**
```toml
# ✓ Good: Helps users find your template
tags = ["axum", "payment-api", "database", "x402", "microservice"]

# ✗ Avoid: Too vague
tags = ["rust", "template"]
```

### 3. **Provide Reasonable Defaults**
```toml
[parameters]
# ✓ Good: User can start immediately
enable_docker = { type = "boolean", default = true }

# ✗ Avoid: Confusing defaults
enable_docker = { type = "boolean", default = false }
```

### 4. **Use Patterns for Safety**
```toml
# ✓ Good: Prevents invalid Rust identifiers
project_name = { type = "string", default = "my-app", pattern = "^[a-z][a-z0-9_-]*$" }

# ✗ Avoid: Accept anything
project_name = { type = "string", default = "my-app" }
```

### 5. **Add Descriptions to Parameters**
```toml
[parameters]
# ✓ Good: User understands what each parameter does
db_type = {
    type = "enum",
    enum = ["postgres", "sqlite"],
    default = "postgres",
    description = "Choose your database engine"
}

# ✗ Avoid: Unclear purpose
db_type = { type = "enum", enum = ["postgres", "sqlite"], default = "postgres" }
```

### 6. **Keep Templates Focused**
```toml
# ✓ Good: One clear use case
name = "x402 Microservice Template"
tags = ["microservice", "payment-api"]

# ✗ Avoid: Trying to be everything
name = "Universal x402 Template"
tags = ["fullstack", "microservice", "cli", "web", "desktop"]
```

### 7. **Version Your Template**
```toml
# ✓ Good: Semantic versioning allows users to pin versions
version = "1.0.0"      # First release
version = "1.0.1"      # Bug fix
version = "1.1.0"      # New features
version = "2.0.0"      # Breaking changes

# ✗ Avoid: Non-standard versioning
version = "latest"
version = "1"
```

### 8. **Document Version Requirements**
```toml
# ✓ Good: Clear minimum requirements
min_rust_version = "1.70.0"
min_x402_cli_version = "0.1.0"

# ✓ Fine: No requirements (compatible with all)
# (omit these fields)

# ✗ Avoid: Vague versions
min_rust_version = "1.7"  # Should be "1.70.0"
```

---

## Examples: Different Template Types

### Example 1: Minimal API Template

```toml
[template]
name = "Basic x402 API"
description = "Minimal Axum HTTP server with x402 payment endpoint"
version = "1.0.0"
authors = ["Alice Developer"]
repository = "https://github.com/user/x402-template-basic-api"
tags = ["axum", "api", "minimal"]

# No [parameters] - uses defaults only
# No [files] - includes all files except common excludes
```

### Example 2: Full-Featured Database Template

```toml
[template]
name = "x402 API with Database"
description = "Axum + PostgreSQL template with SQLx for payments"
version = "2.0.1"
authors = ["Bob Engineer <bob@company.com>"]
repository = "https://github.com/user/x402-template-db-api"
tags = ["axum", "database", "postgres", "sqlx", "payment-api"]
min_rust_version = "1.70.0"

[parameters]
project_name = { type = "string", default = "my-api", pattern = "^[a-z][a-z0-9-]*$" }
db_type = { type = "enum", enum = ["postgres", "sqlite"], default = "postgres" }
include_migrations = { type = "boolean", default = true }

[files]
include = ["src/**/*", "migrations/**/*", "Cargo.toml", ".env.example"]
exclude = ["target/**/*", ".git/**/*"]
```

### Example 3: CLI Tool Template

```toml
[template]
name = "x402 CLI Tool"
description = "Command-line tool scaffold for x402 integration"
version = "1.0.0"
authors = ["Carol Coder"]
repository = "https://github.com/user/x402-template-cli"
tags = ["cli", "clap", "x402"]

[parameters]
tool_name = { type = "string", default = "my-tool", pattern = "^[a-z][a-z0-9-]*$" }
add_logging = { type = "boolean", default = false }

[files]
exclude = ["target/**/*", ".git/**/*"]
```

---

## Troubleshooting

### Q: My template fails validation with "Missing required field 'repository'"

**A:** Make sure you have:
```toml
[template]
repository = "https://github.com/username/repo-name"
```

The URL must be HTTPS and point to GitHub.

### Q: Users report "Invalid default value" error

**A:** Your default value doesn't match the parameter type:
```toml
# ✗ Wrong: default is number, type is string
param = { type = "string", default = 123 }

# ✓ Correct: default is string
param = { type = "string", default = "123" }

# ✗ Wrong: default not in enum
db = { type = "enum", enum = ["postgres"], default = "mysql" }

# ✓ Correct: default in enum
db = { type = "enum", enum = ["postgres", "mysql"], default = "postgres" }
```

### Q: Some files aren't being included in the generated project

**A:** Check your `[files]` section. Patterns are glob-based:
```toml
# ✗ Won't match subdirectories
include = ["src/*"]

# ✓ Will match subdirectories
include = ["src/**/*"]
```

### Q: My Liquid template variables aren't being substituted

**A:** Make sure the variable name matches:
```liquid
<!-- ✓ Correct: uses parameter name -->
{{ project_name }}

<!-- ✗ Wrong: typo in variable name -->
{{ project-name }}
{{ projectName }}
```

---

## Publishing Your Template

### 1. Create Repository

```bash
git init x402-template-mytemplate
cd x402-template-mytemplate
```

### 2. Add x402.toml

Create `x402.toml` in root with valid schema.

### 3. Structure Files

```
x402-template-mytemplate/
├── x402.toml
├── src/
├── Cargo.toml
├── .env.example
└── README.md
```

### 4. Push to GitHub

```bash
git add .
git commit -m "Initial template"
git push origin main
```

### 5. Add GitHub Topic

Go to repository Settings → About → Topics → Add **`x402-template`**

### 6. Done!

Users can now discover your template with:
```bash
cargo-x402 list
```

---

## Versioning Your Template

When you make changes:

```toml
# Bug fixes → patch version
version = "1.0.0" → "1.0.1"

# New features → minor version
version = "1.0.0" → "1.1.0"

# Breaking changes → major version
version = "1.0.0" → "2.0.0"
```

Tag your releases in Git:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Users can pin to specific versions:
```bash
cargo-x402 create --template user/template@1.0.0
```
