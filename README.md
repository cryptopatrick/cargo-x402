<h1 align="center">
  <br>
    <img
      src="https://github.com/xForth/cargo-x402/blob/main/img/cargo-x402-logo.png"
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
  <a href="https://github.com/xForth/cargo-x402/actions" target="_blank">
    <img src="https://img.shields.io/github/actions/workflow/status/xForth/cargo-x402/test.yml" alt="CI Status"/>
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

## 🛎 Status

* **v1.0.0** - Production-ready release
* **208+ tests** passing across all platforms
* **5 official templates** included and maintained
* **Multi-platform support** - macOS, Linux, Windows
* Fully **open-source** and community-driven

<!-- TABLE OF CONTENTS -->
<h2 id="table-of-contents"> :pushpin: Table of Contents</h2>

<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#-what-is-cargo-x402"> What is cargo-x402</a>
      <ul>
        <li><a href="#use-cases"> Use Cases</a></li>
      </ul>
    </li>
    <li><a href="#-features"> Features</a>
      <ul>
        <li><a href="#-core-capabilities"> Core Capabilities</a></li>
        <li><a href="#-template-system">Template System</a></li>
        <li><a href="#-developer-experience">Developer Experience</a></li>
      </ul>
    </li>
    <li><a href="#-architecture"> Architecture</a></li>
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

## 📐 Architecture

1. 🔄 **Overall Workflow**

```
┌──────────────────────────────────────────────────────────┐
│         User Command: cargo-x402 create                  │
│              (Interactive or Direct)                      │
└──────────────────────┬───────────────────────────────────┘
                       │
┌──────────────────────▼───────────────────────────────────┐
│             Discovery Module                             │
│  • Check local cache (< 1 hour)                          │
│  • If stale, query GitHub API for x402-template topic   │
│  • Cache results locally                                 │
└──────────────────────┬───────────────────────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Interactive Selection      │
         │   (or use --template flag)   │
         │   User chooses template      │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Parameter Input            │
         │   (Project name, options)    │
         │   Validation & prompts       │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Download & Extract         │
         │   Template from GitHub       │
         │   Clone to temp directory    │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Validate x402.toml         │
         │   Schema validation          │
         │   Parameter verification     │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Liquid Rendering           │
         │   Substitute parameters      │
         │   Process conditionals       │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Project Creation           │
         │   Copy files to destination  │
         │   Clean up .git directory    │
         │   Initialize git repo        │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │   Success! ✓                 │
         │   Ready to develop           │
         │   Next steps displayed       │
         └─────────────────────────────┘
```

2. 🔍 **Discovery & Caching Flow**

```
┌──────────────────────────────────────────────────────────┐
│         cargo-x402 list (or discover templates)          │
└──────────────────────┬───────────────────────────────────┘
                       │
              ┌────────▼────────┐
              │  Check Local    │
              │  Cache (1hr)    │
              └────────┬────────┘
                       │
           ┌───────────┴───────────┐
           │                       │
    ┌──────▼──────┐         ┌──────▼──────┐
    │ Cache Valid │         │Cache Stale  │
    │ Return!     │         │or Missing   │
    │ (Fast)      │         │             │
    └─────────────┘    ┌────▼──────────┐
                       │  Query GitHub │
                       │  API (topic)  │
                       └────┬──────────┘
                            │
                    ┌───────▼────────┐
                    │  Parse Results │
                    │  Extract Metadata
                    │  (name, description,
                    │   owner, stars)  │
                    └───────┬────────┘
                            │
                    ┌───────▼────────┐
                    │  Update Cache  │
                    │  + timestamp   │
                    └───────┬────────┘
                            │
                    ┌───────▼────────┐
                    │  Display List  │
                    │  to User       │
                    └────────────────┘
```

3. 📝 **Template Creation Flow**

```
┌──────────────────────────────────────────────────────────┐
│         Selected Template Details                         │
│  • Name, description, URL                                │
│  • x402.toml manifest                                    │
│  • Parameters (string, boolean, enum)                    │
└──────────────────────┬───────────────────────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  1. Download Template        │
         │  • Clone from GitHub         │
         │  • Temporary directory       │
         │  • Extract files             │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  2. Parse x402.toml          │
         │  • Read manifest             │
         │  • Extract parameters        │
         │  • Validate schema           │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  3. Prompt for Parameters    │
         │  (or use defaults)           │
         │  • Validate input            │
         │  • Type checking             │
         │  • Regex patterns            │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  4. Render with Liquid       │
         │  • Variable substitution     │
         │  • Conditionals              │
         │  • File processing           │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  5. Create Project Directory │
         │  • Create output folder      │
         │  • Check for collisions      │
         │  • Copy rendered files       │
         └─────────────┬────────────────┘
                       │
         ┌─────────────▼────────────────┐
         │  6. Finalization             │
         │  • Remove .git (template)    │
         │  • Initialize new .git       │
         │  • First commit              │
         │  • Display success           │
         └─────────────────────────────┘
```

4. ⚙️ **Module Architecture**

```
┌────────────────────────────────────────────────────────────────┐
│                        cargo-x402 CLI                          │
│                      (src/main.rs)                             │
└────────────────────────┬──────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   ┌────▼─────┐  ┌──────▼──────┐  ┌─────▼──────┐
   │ Discovery│  │   Schema    │  │  Template  │
   │  Module  │  │  Validation │  │   Render   │
   │          │  │             │  │            │
   │ • GitHub │  │ • x402.toml │  │ • Liquid   │
   │ • Cache  │  │ • Validator │  │ • File     │
   │ • Topics │  │ • Parameter │  │   copy     │
   │          │  │   types     │  │            │
   └────┬─────┘  └──────┬──────┘  └─────┬──────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   ┌────▼──────┐  ┌──────▼──────┐  ┌─────▼──────┐
   │ Commands  │  │ Interactive │  │    Error   │
   │           │  │   Prompts   │  │  Handling  │
   │ • list    │  │             │  │            │
   │ • create  │  │ • Dialoguer │  │ • Error    │
   │           │  │ • Colored   │  │   types    │
   │           │  │   output    │  │ • Helpful  │
   │           │  │ • Validation│  │   messages │
   └───────────┘  └─────────────┘  └────────────┘
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
git clone https://github.com/xForth/cargo-x402.git
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
cargo-x402 create --template xForth/x402-template-basic-api --name my-api

# Skips template selection and uses defaults
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

- **[QUICKSTART.md](./_dev/QUICKSTART.md)** - Get started in 10 minutes
  - Installation methods
  - Your first project
  - Common customizations
  - FAQs and troubleshooting

### For Template Authors

- **[TEMPLATE_AUTHOR_GUIDE.md](./_dev/TEMPLATE_AUTHOR_GUIDE.md)** - Create custom templates
  - Complete template structure guide
  - x402.toml manifest specification
  - Liquid templating syntax and examples
  - Parameter validation and file handling
  - Publishing and GitHub discovery
  - Best practices and troubleshooting

### For Developers

- **[API Documentation](https://docs.rs/cargo-x402)** - Full Rustdoc reference
- **GitHub Repository** - [xForth/cargo-x402](https://github.com/xForth/cargo-x402)
- **Official Templates** - [xForth Templates](https://github.com/xForth)

## 🗺️ Roadmap

### v1.0.0 ✅ (Current - Production Ready)
- [x] Stable public API
- [x] 5 official maintained templates
- [x] Comprehensive test coverage (208+ tests)
- [x] Full documentation and guides
- [x] Multi-platform support (macOS, Linux, Windows)

### v1.1.0 (Q1 2025)
- [ ] Enhanced template caching and offline mode
- [ ] Template repository metadata indexing
- [ ] Improved parameter validation with custom validators
- [ ] Template dependency resolution

### v1.2.0 (Q2 2025)
- [ ] Local template support (non-GitHub templates)
- [ ] Custom template hooks (pre/post generation)
- [ ] Private repository template support
- [ ] Performance profiling and optimization

### v2.0.0 (Q3-Q4 2025)
- [ ] Official template marketplace/registry
- [ ] IDE integrations (VS Code, IntelliJ)
- [ ] Template signing and verification
- [ ] Advanced dependency management

## 🖊 Author

<a href="https://x.com/cryptopatrick">CryptoPatrick</a> - Rust ecosystem tools & frameworks

## 🐣 Support

**Star** ⭐ this project if you find it useful!

### Report Issues
- [Open GitHub Issue](https://github.com/xForth/cargo-x402/issues)
- Include: command, error message, OS/Rust version

### Get Help
- Check [QUICKSTART.md](./_dev/QUICKSTART.md) for common questions
- Review [TEMPLATE_AUTHOR_GUIDE.md](./_dev/TEMPLATE_AUTHOR_GUIDE.md) for template creation
- Search existing [GitHub Issues](https://github.com/xForth/cargo-x402/issues)

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
  Made with ❤️ by <a href="https://x.com/cryptopatrick">CryptoPatrick</a> & <a href="https://github.com/xForth">xForth</a>
  <br>
  <a href="https://github.com/xForth/cargo-x402">GitHub</a> •
  <a href="https://crates.io/crates/cargo-x402">Crates.io</a> •
  <a href="https://docs.rs/cargo-x402">Docs.rs</a>
</p>
