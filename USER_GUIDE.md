# cargo-x402 User Guide

Quick-start guide for using cargo-x402 to scaffold new projects from templates.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Creating Projects](#creating-projects)
- [Browsing Templates](#browsing-templates)
- [Interactive Mode](#interactive-mode)
- [FAQ](#faq)
- [Troubleshooting](#troubleshooting)

## Installation

### From crates.io

```bash
cargo install cargo-x402
```

### From Source

```bash
git clone https://github.com/cryptopatrick/cargo-x402.git
cd cargo-x402
cargo install --path .
```

### Verify Installation

```bash
cargo-x402 --version
cargo-x402 --help
```

## Quick Start

### Create Your First Project

```bash
# Interactive mode (default)
cargo-x402

# Or with direct command
cargo-x402 create --template cryptopatrick/x402-template-basic-api
```

Follow the prompts to customize your project.

### List Available Templates

```bash
cargo-x402 list
```

Shows all available templates with star counts and descriptions.

## Commands

### Interactive Mode

```bash
cargo-x402
```

Launches interactive menu for:
- Browsing templates
- Selecting template
- Entering project details
- Creating project

**Best for**: Discovering templates, learning

### List Command

```bash
cargo-x402 list [OPTIONS]
```

**Options**:
- `--refresh` - Bypass cache, fetch fresh list from GitHub
- `--tags <TAGS>` - Filter by tags (comma-separated)

**Examples**:
```bash
# Show all templates
cargo-x402 list

# Refresh list
cargo-x402 list --refresh

# Show only Axum templates
cargo-x402 list --tags axum

# Show Rust API templates
cargo-x402 list --tags rust,api
```

### Create Command

```bash
cargo-x402 create [OPTIONS] --template <TEMPLATE>
```

**Options**:
- `--template <REPO>` - Repository (format: `owner/repo` or full URL)
- `--directory <DIR>` - Output directory (default: current)
- `--name <NAME>` - Project name (otherwise prompted)

**Examples**:
```bash
# Create from template
cargo-x402 create --template user/my-template

# Custom directory
cargo-x402 create --template user/my-template --directory ~/projects

# Shorthand
cargo-x402 create -t user/my-template
```

### Version Command

```bash
cargo-x402 --version
```

Shows installed version.

## Creating Projects

### Step-by-Step Process

1. **Choose Template**
   - View list: `cargo-x402 list`
   - Select template of interest

2. **Provide Project Details**
   - Project name (validated)
   - Author name
   - Any custom parameters

3. **Project Created**
   - Files copied to new directory
   - Variables substituted
   - Git initialized
   - Ready to develop

### Example Workflow

```bash
# 1. List available templates
$ cargo-x402 list
My API Template              ⭐ 42
My Web Framework             ⭐ 28
...

# 2. Create project
$ cargo-x402 create --template myuser/x402-template-api

# 3. Answer prompts
Project name: my-awesome-api
Author name: Your Name
Enable database: [y/n] y

# 4. Success!
✓ Project created in ./my-awesome-api
Next steps:
  cd my-awesome-api
  cargo build
  cargo run
```

### Custom Parameters

Templates can require additional information:

**String Parameters**
```
Project name: my-project
```
Must match pattern (if defined)

**Boolean Parameters**
```
Enable database support? [y/n]: y
```
Enter `y`, `yes`, `true`, or `n`, `no`, `false`

**Enum Parameters**
```
Environment: (1) development, (2) staging, (3) production
Select option [1-3]: 1
```
Choose numbered option

## Browsing Templates

### View All Templates

```bash
cargo-x402 list
```

Shows:
- Template name
- Stars (popularity)
- Description
- Author

### Filter by Tags

```bash
cargo-x402 list --tags rust,api
```

Common tags:
- `rust` - Rust projects
- `api` - REST APIs
- `web` - Web applications
- `cli` - Command-line tools
- `axum` - Axum web framework
- `react` - React frontend
- `fullstack` - Full-stack applications

### Find Popular Templates

Star count indicates community interest. Sort by checking list output.

### Check Template Details

1. Visit on GitHub: `https://github.com/owner/repo`
2. Review README for:
   - Features
   - Parameters
   - Usage instructions
   - Examples

## Interactive Mode

### Starting Interactive Mode

```bash
cargo-x402
```

Or just run without arguments.

### Navigation

```
Welcome to cargo-x402!

1. Create project from template
2. Browse available templates
3. View configuration
4. Exit

Select option [1-4]:
```

### Creating from Interactive Mode

1. Select "Create project" (option 1)
2. Browse and select template
3. Enter project name
4. Answer parameter prompts
5. Confirm and create

**Advantages**:
- Discover templates visually
- Guided through parameters
- Helpful error messages

## FAQ

### Where do templates come from?

Templates are GitHub repositories tagged with `x402-template`. They're discovered automatically.

### How do I create my own template?

See [TEMPLATE_AUTHOR_GUIDE.md](./TEMPLATE_AUTHOR_GUIDE.md)

### Can I use local templates?

Not yet. Templates must be published to GitHub with the `x402-template` topic.

### How often is the template list updated?

The list is cached for 1 hour locally. Use `--refresh` to get latest immediately.

### Can I modify generated projects?

Yes! Generated projects are fully yours to customize. Templates just provide starting points.

### Do I need a GitHub account?

Only if you want to publish your own templates. Using templates requires no account.

### How do I report template issues?

Contact the template author or open issue on the template's GitHub repository.

### Can templates use my sensitive data?

No. Templates only receive information you explicitly provide (project name, etc.). They cannot access environment variables or secrets unless you provide them.

## Troubleshooting

### Command Not Found

**Problem**: `cargo-x402: command not found`

**Solution**:
```bash
# Reinstall
cargo install cargo-x402

# Verify installation
cargo-x402 --version

# Check PATH
echo $PATH | grep ".cargo/bin"
```

### Template Not Showing Up

**Problem**: Template doesn't appear in list

**Causes**:
- Not yet discovered (wait 15 minutes after publishing)
- Missing `x402-template` topic
- Repository is private
- Network issues

**Solutions**:
```bash
# Refresh cache
cargo-x402 list --refresh

# Try again in a few minutes
```

### Invalid Project Name

**Problem**: "Project name does not match pattern"

**Causes**:
- Contains invalid characters
- Template has strict naming rules

**Solutions**:
- Use lowercase letters, numbers, hyphens
- Avoid starting/ending with hyphens
- No spaces or special characters
- Example: `my-awesome-project`

### Template Download Failed

**Problem**: "Failed to download template"

**Causes**:
- Network connectivity issue
- Template repository deleted
- GitHub rate limited

**Solutions**:
```bash
# Wait a moment and try again
cargo-x402 create --template owner/repo

# Check internet connection
ping github.com

# Try different template
```

### Git Initialization Failed

**Problem**: "Failed to initialize git repository"

**Solutions**:
```bash
# Verify Git is installed
git --version

# Check directory permissions
ls -la

# Create project in different directory
cargo-x402 create -t owner/repo -d ~/tmp/my-project
```

### Parameter Validation Error

**Problem**: "Invalid value for parameter"

**Solutions**:
- Reread error message for requirements
- Check parameter type (string, boolean, enum)
- Follow regex pattern if specified
- For enums, select from provided options

### Liquid Rendering Error

**Problem**: Malformed variables in generated files

**Causes**:
- Template has invalid Liquid syntax
- Variable doesn't exist
- Reserved keywords conflict

**Solution**:
- Report to template author
- Try different template
- Edit files manually after creation

### Permission Denied

**Problem**: "Permission denied" when creating project

**Causes**:
- Directory not writable
- File permission issues

**Solutions**:
```bash
# Create in home directory
cargo-x402 create -t owner/repo -d ~

# Check permissions
ls -la /path/to/directory

# Use sudo (not recommended)
sudo cargo-x402 create -t owner/repo
```

### Large Project Creation Slow

**Problem**: Project creation takes a long time

**Causes**:
- Slow internet connection
- Large template with many files
- GitHub rate limiting

**Solutions**:
- Be patient (first run downloads files)
- Retry if network interrupted
- Use `--refresh` less frequently

### Cache Issues

**Problem**: Getting old template version

**Solution**:
```bash
# Clear cache and refresh
cargo-x402 list --refresh
```

Cache location: `~/.cache/x402/templates.json`

## Getting Help

### Official Resources

- [GitHub Repository](https://github.com/cryptopatrick/cargo-x402)
- [Template Author Guide](./TEMPLATE_AUTHOR_GUIDE.md)
- [Official Templates](https://github.com/topics/x402-template)

### Report Issues

- [Open GitHub Issue](https://github.com/cryptopatrick/cargo-x402/issues)
- Provide:
  - Command you ran
  - Error message
  - System info (`rustc --version`)
  - OS and version

### Ask Questions

- GitHub Discussions (if available)
- Stack Overflow tag: `cargo-x402`

---

Happy scaffolding! 🚀
