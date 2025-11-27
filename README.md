<h1 align="center">
  <br>
    <img
      src="https://github.com/cryptopatrick/cargo-x402/blob/main/img/cargo-x402-logo.png"
      alt="cargo-x402"
      width="200"
    />
  <br>
  cargo-x402
  <br>
</h1>

<h4 align="center">
  Scaffold production-ready Rust projects instantly with
  <a href="https://github.com/topics/x402-template" target="_blank">
    community templates</a>.</h4>

<p align="center">
  <a href="https://crates.io/crates/cargo-x402" target="_blank">
    <img src="https://img.shields.io/crates/v/cargo-x402.svg" alt="Crates.io"/>
  </a>
  <a href="https://crates.io/crates/cargo-x402" target="_blank">
    <img src="https://img.shields.io/crates/d/cargo-x402.svg" alt="Downloads"/>
  </a>
  <a href="https://github.com/cryptopatrick/cargo-x402/actions" target="_blank">
    <img src="https://img.shields.io/github/actions/workflow/status/cryptopatrick/cargo-x402/ci.yml" alt="CI Status"/>
  </a>
  <a href="https://docs.rs/cargo-x402" target="_blank">
    <img src="https://docs.rs/cargo-x402/badge.svg" alt="Documentation"/>
  </a>
  <a href="LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"/>
  </a>
</p>

<b>Author:</b> 👋 Hi, I'm <a href="https://x.com/cryptopatrick">CryptoPatrick</a>! I create tools and frameworks for the Rust ecosystem. If you have questions or want to connect, reach out on <a href="https://x.com/cryptopatrick">X/Twitter</a>.

---

<p align="center">
  <a href="#-what-is-cargo-x402">What is cargo-x402</a> •
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-templates">Templates</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="#-license">License</a>
</p>

## 🛎 Important Notices

* **v0.2.0-beta** - Early release, features may change
* Templates must be published to GitHub with `x402-template` topic
* Requires internet connection for template discovery
* Fully open-source and community-driven

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-cargo-x402"> What is cargo-x402</a>
      <ul>
        <li><a href="#use-cases"> Use Cases</a></li>
        <li><a href="#how-it-works"> How It Works</a></li>
      </ul>
    </li>
    <li><a href="#-features"> Features</a>
      <ul>
        <li><a href="#-core-capabilities"> Core Capabilities</a></li>
        <li><a href="#-template-system">Template System</a></li>
        <li><a href="#-developer-experience">Developer Experience</a></li>
      </ul>
    </li>
    <li><a href="#-quick-start"> Quick Start</a></li>
    <li><a href="#-templates"> Official Templates</a></li>
    <li><a href="#-documentation"> Documentation</a></li>
    <li><a href="#-roadmap"> Roadmap</a></li>
    <li><a href="#-author"> Author</a></li>
    <li><a href="#-support"> Support</a></li>
    <li><a href="#-contributing"> Contributing</a></li>
    <li><a href="#-license">License</a></li>
  </ol>
</details>

## 🤔 What is cargo-x402

`cargo-x402` is a template scaffolding tool that helps you create production-ready Rust projects in seconds. It automatically discovers templates from GitHub repositories tagged with `x402-template` and provides an interactive CLI for creating projects with custom parameters.

Instead of manually setting up boilerplate code, copying files, and configuring projects, cargo-x402 handles it all instantly—and it's customizable for your needs.

### Use Cases

- **Rapid Prototyping**: Start API projects in seconds with batteries included
- **Standardized Architecture**: Enforce consistent project structure across teams
- **Best Practices**: Include production-grade patterns (logging, observability, database)
- **Full-Stack Development**: Scaffold complete applications (frontend + backend)
- **CLI Tools**: Create command-line utilities with proper argument parsing
- **Microservices**: Deploy production-ready services with observability built-in

### How It Works

```
1. List templates          cargo-x402 list
2. Choose template         Interactive selection
3. Enter project details   Project name, author, options
4. Done!                   Full project ready to develop
```

## 📷 Features

###  Core Capabilities

- **Template Discovery**: Automatic discovery from GitHub via `x402-template` topic
- **Interactive CLI**: User-friendly terminal interface with colored output
- **Smart Caching**: 1-hour cache with manual refresh option
- **Parameter Handling**: Support for string, boolean, and enum parameters
- **Schema Validation**: Comprehensive validation of template configuration
- **Liquid Templating**: Safe variable substitution with conditionals and loops

###  **Template System**

- **x402.toml Schema**: Declarative template configuration
- **File Rules**: Include/exclude patterns for flexible file management
- **Version Constraints**: Specify minimum Rust and cargo-x402 versions
- **Parameter Validation**: Regex patterns and enum options for user input
- **Metadata Support**: Template name, description, authors, and tags
- **GitHub Integration**: Seamless discovery and download from GitHub

###  **Developer Experience**

- **Zero Configuration**: Works out of the box
- **Progress Indicators**: Visual feedback for all operations
- **Helpful Error Messages**: Clear guidance when something goes wrong
- **Smart Defaults**: Sensible defaults reduce the number of prompts
- **Git Integration**: Automatic git repository initialization
- **Template Examples**: 5 official templates to get started

## 🚙 Quick Start

### Installation

Install from crates.io:

```bash
cargo install cargo-x402
```

Or build from source:

```bash
git clone https://github.com/cryptopatrick/cargo-x402.git
cd cargo-x402
cargo install --path .
```

### Your First Project (30 seconds)

**Option 1: Interactive Mode**

```bash
# Start interactive menu
cargo-x402

# Follow prompts to:
# 1. Browse available templates
# 2. Select template
# 3. Enter project name and options
# 4. Confirm and create
```

**Option 2: Direct Command**

```bash
# Create project directly
cargo-x402 create --template cryptopatrick/x402-template-basic-api

# Follow prompts for project details
```

**Option 3: List Templates First**

```bash
# See all available templates
cargo-x402 list

# Show templates matching tags
cargo-x402 list --tags rust,api
```

### What You Get

After running `cargo-x402 create`, you'll have:

```
my-awesome-api/
├── src/
│   └── main.rs              # Ready-to-run code
├── Cargo.toml               # Configured dependencies
├── README.md                # Project documentation
├── .gitignore               # Git configuration
├── .env.example             # Environment template
├── Dockerfile               # Container support
└── .git/                    # Initialized git repo
```

Everything is ready to build and run:

```bash
cd my-awesome-api
cargo build
cargo run
```

## 📦 Templates

### Official Templates (v0.2.0)

#### 1. **Basic Axum API**
Production-ready REST API with Axum framework

**Features:**
- Health check endpoints
- Request/response handling
- Structured logging
- Error handling

**Use for:** Quick API prototypes, microservices, REST backends

#### 2. **Axum + PostgreSQL**
Full-featured API with database support

**Features:**
- SQLx with Postgres
- Database migrations
- Connection pooling
- CRUD operations

**Use for:** Data-driven applications, persistent storage requirements

#### 3. **Full-Stack (Axum + React)**
Complete web application with backend and frontend

**Features:**
- Axum REST API backend
- React 18 frontend with TypeScript
- Vite build system
- CORS configuration
- Docker Compose setup

**Use for:** Web applications, SPA + API combos, full-stack projects

#### 4. **Microservice with Observability**
Production-grade microservice with tracing and metrics

**Features:**
- Jaeger distributed tracing
- Prometheus metrics
- Structured logging
- JSON output
- Health checks

**Use for:** Cloud deployments, observability needs, production services

#### 5. **CLI Tool**
Professional command-line tool template

**Features:**
- Clap argument parsing
- Colored output
- Progress indicators
- File operations
- Error handling

**Use for:** CLI utilities, scripts, command-line tools

### Create Your Own Template

See [TEMPLATE_AUTHOR_GUIDE.md](./TEMPLATE_AUTHOR_GUIDE.md) for:
- Template structure and format
- Parameter configuration
- Publishing on GitHub
- Best practices

## 📚 Documentation

### For Users

- **[USER_GUIDE.md](./USER_GUIDE.md)** - Complete usage guide
  - Installation and setup
  - Command reference
  - Creating projects
  - Troubleshooting

### For Template Authors

- **[TEMPLATE_AUTHOR_GUIDE.md](./TEMPLATE_AUTHOR_GUIDE.md)** - Create templates
  - Template structure
  - x402.toml configuration
  - Liquid templating
  - Publishing templates

### Additional Resources

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Technical design and implementation
- **[RELEASE_NOTES_BETA.md](./RELEASE_NOTES_BETA.md)** - Beta release information and roadmap
- **[API Documentation](https://docs.rs/cargo-x402)** - Rustdoc on docs.rs

## 🗺️ Roadmap

### v0.3.0 (Q1 2025)
- [ ] Local template support (non-GitHub templates)
- [ ] Template marketplace/registry
- [ ] GraphQL API client for faster discovery
- [ ] Custom template hooks (pre/post generation)

### v0.4.0 (Q2 2025)
- [ ] Private repository support
- [ ] Template signing and verification
- [ ] IDE integrations (VS Code, IntelliJ)
- [ ] Performance optimizations

### v1.0.0 (Q3 2025)
- [ ] Stable public API
- [ ] Official template registry
- [ ] Production-grade reliability SLA
- [ ] Comprehensive ecosystem

## 🖊 Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a> - Rust ecosystem tools & frameworks

## 🐣 Support

**Star** ⭐ this project if you find it useful!

### Report Issues
- [Open GitHub Issue](https://github.com/cryptopatrick/cargo-x402/issues)
- Include: command, error message, OS/Rust version

### Get Help
- Check [USER_GUIDE.md](./USER_GUIDE.md) for common questions
- Review [TEMPLATE_AUTHOR_GUIDE.md](./TEMPLATE_AUTHOR_GUIDE.md) for template creation
- Search existing [GitHub Issues](https://github.com/cryptopatrick/cargo-x402/issues)

## 🤝 Contributing

Found a bug? Want a feature? Have a template idea?

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for:
- Code style and standards
- Testing requirements
- Submitting PRs
- Development setup

### Ways to Contribute

1. **Report Bugs** - Found an issue? Open a GitHub issue
2. **Suggest Features** - Have an idea? Share it in discussions
3. **Create Templates** - Build and publish templates to GitHub
4. **Improve Docs** - Help improve guides and documentation
5. **Submit Code** - Fix bugs or implement features

## 🗄 License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ by <a href="https://x.com/cryptopatrick">CryptoPatrick</a>
  <br>
  <a href="https://github.com/cryptopatrick/cargo-x402">GitHub</a> •
  <a href="https://crates.io/crates/cargo-x402">Crates.io</a> •
  <a href="https://docs.rs/cargo-x402">Docs.rs</a>
</p>
