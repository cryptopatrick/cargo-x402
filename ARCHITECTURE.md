# cargo-x402: Architecture & Design (Option C - Pluggable Templates)

## Overview

`cargo-x402` is a lean Rust CLI tool for scaffolding x402 payment projects. Unlike traditional approaches with bundled templates, this tool uses a **GitHub topic-based discovery system** where templates are independent repositories that define their interface via a standardized `x402.toml` schema.

## Design Philosophy

1. **Lean Core**: The CLI tool itself is minimal (~2-3 MB binary). No templates bundled.
2. **Community-Driven**: Templates are published by anyone, discovered via GitHub topics.
3. **Quality Control**: All templates must declare their requirements via a schema.
4. **Flexibility**: Users can reference official templates, community templates, or private URLs.
5. **Pure Rust**: All templates scaffold Rust projects using Axum/SQLx (Phase 1).

## System Architecture

### High-Level Flow

```
User runs: cargo-x402 create
    │
    ├─→ Query GitHub API for repos tagged 'x402-template'
    │
    ├─→ (Cache results locally, 1-hour TTL)
    │
    ├─→ Display interactive menu with template metadata
    │
    ├─→ User selects template or provides custom URL
    │
    ├─→ Download template repository (ZIP, without .git)
    │
    ├─→ Validate x402.toml against schema
    │       └─→ Error if invalid → display helpful message
    │
    ├─→ Prompt user for parameter values (if template defines any)
    │
    ├─→ Render all template files using Liquid templating
    │       └─→ Substitute variables, conditionals, filters
    │
    ├─→ Write rendered files to output directory
    │
    ├─→ Initialize new git repository
    │
    └─→ Display next steps for user
```

---

## Component Architecture

### 1. CLI Interface (main.rs + commands/)

**Entry point:** `main.rs`
- Parse command-line arguments using `clap`
- Route to appropriate subcommand
- Handle errors and display helpful messages

**Commands:**

#### `cargo-x402 list`
- Lists all discoverable templates from GitHub topic `x402-template`
- Shows: name, description, author, stars, language
- Options:
  - `--refresh`: Force refresh cache (bypass 1-hour TTL)
  - `--tags`: Filter by tags (e.g., `--tags payment,database`)

**Implementation file:** `src/commands/list.rs`

#### `cargo-x402 create`
- Interactive or non-interactive template scaffolding
- Options:
  - `--template <URL>`: Specify template (GitHub URL or shorthand)
  - `--name <NAME>`: Project name
- If no options provided: Interactive mode

**Implementation file:** `src/commands/create.rs`

#### `cargo-x402 --version`
- Display CLI version from Cargo.toml

---

### 2. Template Discovery Service (discovery/)

**Purpose:** Query GitHub, cache results, filter by criteria

**File:** `src/discovery/github.rs`

**Key Functions:**
```rust
pub async fn discover_templates() -> Result<Vec<TemplateInfo>>
pub async fn get_template_metadata(owner: &str, repo: &str) -> Result<TemplateInfo>
pub fn filter_by_version_constraints(templates: Vec<TemplateInfo>, constraints: &Constraints) -> Vec<TemplateInfo>
```

**GitHub API Integration:**
- Endpoint: `GET /search/repositories?q=topic:x402-template`
- Rate limits: 60 req/hour (unauthenticated), 5000 (authenticated with token)
- Error handling: Graceful fallback if API unavailable
- Caching: Store results in `~/.cache/x402/templates.json` with timestamp

**TemplateInfo Structure:**
```rust
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub url: String,
    pub owner: String,
    pub repo: String,
    pub stars: u32,
    pub language: String,
    pub topics: Vec<String>,
}
```

**File:** `src/discovery/cache.rs`

**Key Functions:**
```rust
pub fn load_cache() -> Result<CachedTemplates>
pub fn save_cache(templates: &CachedTemplates) -> Result<()>
pub fn is_cache_stale(cache_age: Duration) -> bool
```

---

### 3. Template Schema & Validation (schema/)

**Purpose:** Define what every template must contain, validate templates

**Schema File Format:** `x402.toml` in template root

```toml
[template]
name = "x402 Axum API"
description = "Payment API using Axum and PostgreSQL"
version = "1.0.0"
authors = ["Your Name"]
repository = "https://github.com/user/x402-template-axum"
tags = ["axum", "payment", "database"]
min_rust_version = "1.70"
min_x402_cli_version = "0.1.0"

[parameters]
# Optional: declare what variables the template uses
project_name = { type = "string", default = "my-project", pattern = "^[a-z][a-z0-9-]*$" }
author = { type = "string", default = "Your Name" }
enable_docker = { type = "boolean", default = true }
db_type = { type = "string", enum = ["postgres", "sqlite"], default = "postgres" }

[files]
include = ["src/**/*", "Cargo.toml", ".env.example"]
exclude = ["target/**/*", ".git/**/*", "*.log"]
```

**Schema Struct:**

**File:** `src/schema/template.rs`

```rust
pub struct TemplateSchema {
    pub template: TemplateMetadata,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub files: FileRules,
}

pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub authors: Vec<String>,
    pub repository: String,
    pub tags: Option<Vec<String>>,
    pub min_rust_version: Option<String>,
    pub min_x402_cli_version: Option<String>,
}

pub enum ParameterType {
    String { default: String, pattern: Option<String> },
    Boolean { default: bool },
    Enum { choices: Vec<String>, default: String },
}

pub struct Parameter {
    pub type_: ParameterType,
    pub description: Option<String>,
}
```

**Validation:**

**File:** `src/schema/validator.rs`

```rust
pub fn validate_schema(path: &Path) -> Result<TemplateSchema, ValidationError>
pub fn validate_version_constraint(constraint: &str, version: &str) -> bool
pub fn validate_parameter_value(param: &Parameter, value: &str) -> Result<()>
```

**Validation Rules:**
1. `[template]` section is required
2. Required fields: `name`, `description`, `version`, `authors`, `repository`
3. `version` must follow semantic versioning
4. `repository` must be a valid HTTPS GitHub URL
5. If `min_rust_version` provided: must be valid semver
6. If `min_x402_cli_version` provided: must match CLI version (or fail gracefully)
7. Parameter types must be valid (string, boolean, enum)
8. File patterns must be valid glob expressions

**Error Messages:**
- Clear, actionable error messages
- Suggest fixes (e.g., "Add `[template]` section to x402.toml")

---

### 4. Template Rendering (template/)

**Purpose:** Download templates and render them with user variables

**Template Downloader:**

**File:** `src/template/downloader.rs`

```rust
pub async fn download_template(url: &str, dest: &Path) -> Result<()>
pub fn cleanup_git_history(dest: &Path) -> Result<()>
```

**Strategy:**
1. Support GitHub URLs (convert to zipball URL)
2. Support direct zip URLs
3. Download and extract ZIP
4. Remove `.git` directory (fresh history)
5. Validate `x402.toml` exists

**Template Renderer:**

**File:** `src/template/render.rs`

```rust
pub async fn render_template(
    template_path: &Path,
    parameters: &HashMap<String, String>,
    output_path: &Path,
) -> Result<()>
```

**Process:**
1. Load `x402.toml` and validate
2. For each file in template:
   - Read file content
   - Apply Liquid template engine
   - Write to output directory
3. Handle binary files (images, .git) - copy without rendering
4. Clean up temporary files

**Liquid Templating:**

Standard Liquid syntax:
```liquid
{{ project_name }}                              # Variable substitution
{{ variable | lowercase }}                      # Filters
{% if enable_docker %}...{% endif %}            # Conditionals
{% for item in array %}...{% endfor %}          # Loops
```

**Default variables available:**
- `project_name`: User-provided project name
- `author`: From parameters
- `version`: Current CLI version
- `date`: Current date

**Rust crate:** `liquid` (or `tera` as alternative)

---

### 5. User Input Handling (interactive)

**File:** `src/interactive.rs`

**Purpose:** Prompt users for template selection and parameters

**Functions:**
```rust
pub async fn select_template() -> Result<String>  // Interactive menu
pub fn prompt_for_parameters(schema: &TemplateSchema) -> Result<HashMap<String, String>>
pub fn validate_project_name(name: &str) -> Result<()>
```

**Libraries:** `dialoguer` (for menus), `inquire` (alternative)

---

### 6. Error Handling

**Custom error types:**

**File:** `src/error.rs`

```rust
pub enum Error {
    TemplateNotFound(String),
    InvalidSchema(String),
    ValidationError { field: String, message: String },
    NetworkError(String),
    FileSystemError(String),
    ParameterError(String),
}

impl fmt::Display for Error { ... }
impl std::error::Error for Error { ... }
```

**Error messages are user-friendly and actionable:**
- Show what went wrong
- Explain why it happened
- Suggest how to fix it

Example:
```
❌ Template validation failed

Missing required field: 'repository'

Fix: Add the following to your x402.toml:
  repository = "https://github.com/user/template-name"
```

---

## Data Flow Example

### Scenario: User creates a project from GitHub template

```
User: $ cargo-x402 create --template user/x402-template-starter-kit --name my-app

1. Validate CLI args
   └─→ template URL: "user/x402-template-starter-kit"
   └─→ project name: "my-app"

2. Resolve template reference
   └─→ Convert shorthand to: "https://github.com/user/x402-template-starter-kit"
   └─→ Fetch: https://api.github.com/repos/user/x402-template-starter-kit

3. Download template
   └─→ Convert to zipball URL
   └─→ Download ZIP
   └─→ Extract to temporary directory
   └─→ Remove .git directory

4. Load and validate x402.toml
   ├─→ Parse TOML
   ├─→ Validate schema
   └─→ Extract parameters

5. Prompt for parameters
   ├─→ Show parameter name: "Project Name"
   ├─→ Current value: "my-app" (already provided)
   ├─→ Show parameter: "Author"
   ├─→ Use default: "Your Name"
   └─→ Collect into HashMap

6. Render template
   ├─→ For each file in template:
   │   ├─→ Read file
   │   ├─→ Apply Liquid rendering
   │   └─→ Write to output/my-app/
   └─→ Result: Rendered project

7. Initialize git
   └─→ git init
   └─→ git add .
   └─→ (Don't commit, let user do that)

8. Display success message
   ✅ Project created: ./my-app/

   Next steps:
   $ cd my-app
   $ cargo build
   $ cp .env.example .env  # Configure
   $ cargo run
```

---

## File Structure

```
cargo-x402/
├── Cargo.toml                         # Project manifest
├── src/
│   ├── main.rs                        # CLI entry point
│   ├── lib.rs                         # Library exports
│   ├── error.rs                       # Error types
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs                    # `cargo-x402 list` command
│   │   └── create.rs                  # `cargo-x402 create` command
│   ├── discovery/
│   │   ├── mod.rs
│   │   ├── github.rs                  # GitHub API integration
│   │   └── cache.rs                   # Caching logic
│   ├── schema/
│   │   ├── mod.rs
│   │   ├── template.rs                # Schema structures
│   │   └── validator.rs               # Validation logic
│   ├── template/
│   │   ├── mod.rs
│   │   ├── downloader.rs              # Template downloading
│   │   └── render.rs                  # Liquid rendering
│   ├── interactive.rs                 # User prompts
│   └── utils.rs                       # Helpers
├── tests/
│   ├── integration_tests.rs           # End-to-end tests
│   └── fixtures/                      # Test templates
└── README.md
```

---

## Dependencies (Cargo.toml)

**Essential:**
- `clap`: CLI argument parsing with derive macros
- `tokio`: Async runtime
- `serde` + `toml`: TOML parsing and serialization
- `reqwest`: HTTP client for GitHub API
- `liquid`: Template rendering engine

**UI/Feedback:**
- `dialoguer` or `inquire`: Interactive menu selection
- `indicatif`: Progress bars and spinners
- `colored`: Terminal color output

**Utilities:**
- `regex`: Pattern validation
- `chrono`: Date/time handling
- `dirs`: Cross-platform directory handling
- `serde_json`: JSON for caching

**Optional:**
- `tracing`: Debug logging
- `anyhow`: Error handling

---

## Template Structure (What Templates Provide)

Every template repository has this structure:

```
x402-template-axum/
├── x402.toml                  # Schema definition (REQUIRED)
├── src/
│   └── main.rs               # Rust source
│   └── ... (template files)
├── Cargo.toml                # Cargo manifest (will be rendered)
├── .env.example              # Environment template
├── README.md                 # Template documentation
└── .gitignore
```

**Key:** All files can contain Liquid template syntax.

**Special handling:**
- `x402.toml` is NOT rendered (it's metadata)
- Binary files are copied as-is
- `.git` directory is removed after download

---

## Caching Strategy

**Location:** `~/.cache/x402/templates.json`

**Structure:**
```json
{
  "last_updated": "2024-11-27T10:30:00Z",
  "templates": [
    {
      "name": "x402 Starter Kit",
      "url": "https://github.com/coinbase/x402-template-starter-kit",
      "...": "..."
    }
  ]
}
```

**TTL:** 1 hour (configurable)

**Refresh:**
- Automatic: On startup, check if cache is older than 1 hour
- Manual: `cargo-x402 list --refresh`

**Benefits:**
- Reduces GitHub API calls (rate limit friendly)
- Faster `list` command for offline/slow networks
- Graceful fallback if GitHub API is unavailable

---

## Error Handling Strategy

**Layers:**

1. **User Input Validation**
   - Validate project name format
   - Validate parameter values against schema

2. **Schema Validation**
   - Check `x402.toml` structure
   - Verify required fields
   - Validate version constraints

3. **Network Errors**
   - Graceful fallback to cached data
   - Clear error messages about connectivity

4. **File System Errors**
   - Check target directory exists
   - Ensure write permissions
   - Clean up on failure

5. **Template Rendering Errors**
   - Liquid syntax errors → show template context
   - Missing variables → suggest fixes
   - Invalid patterns → clear error messages

---

## Testing Strategy

**Unit Tests:**
- Schema validation logic
- Parameter value validation
- Version constraint checking
- Cache TTL logic

**Integration Tests:**
- Full `cargo-x402 create` flow with test templates
- GitHub API mocking (using `mockito` or similar)
- Template rendering accuracy

**Test Fixtures:**
- Minimal valid template in `tests/fixtures/`
- Invalid template (missing required fields)
- Template with all parameter types

---

## Future Enhancements (v1.1+)

1. **Template Registry**: Centralized index for better discovery
2. **Remote Validation**: Templates validate via GitHub Actions CI
3. **Search & Filters**: Filter templates by tags, language, features
4. **Template Updates**: Notify users when templates are updated
5. **Local Testing**: `cargo-x402 test <template-path>` for template authors
6. **Community Templates**: Official registry for community templates
7. **Custom Parameters**: File upload, checkboxes, date pickers in prompts
8. **Streaming Output**: Real-time feedback during rendering

---

## Performance Considerations

**Startup Time:**
- Should be < 1 second for list (with cache)
- < 5 seconds for create (download + render)

**Memory:**
- Template cache: < 1 MB for 100 templates
- In-flight template: < 50 MB (typical)

**Binary Size:**
- Target: < 5 MB (release build)
- Achievable with careful dependency selection

**API Calls:**
- Minimum: 1 per hour (cache refresh)
- Maximum: 60 per hour (GitHub rate limit)

---

## Security Considerations

1. **URL Validation**: Only allow HTTPS GitHub URLs
2. **File Path Validation**: Prevent directory traversal in template files
3. **Liquid Sandbox**: Liquid templates are non-code (safe rendering)
4. **No Script Execution**: Templates don't execute arbitrary code (unlike some tools)
5. **Checksum Validation**: (Future) Verify template integrity

---

## Success Metrics

- CLI binary size < 5 MB
- `list` command < 2 seconds
- `create` command < 30 seconds (download + render)
- Schema validation catches 100% of invalid templates
- 5 official templates at launch
- Community templates tagged `x402-template` discoverable
- Zero template-related security issues
