use clap::{Parser, Subcommand};
use colored::*;
use std::process;

mod commands;
mod discovery;
mod error;
mod interactive;
mod schema;
mod template;

use error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "cargo-x402",
    about = "Scaffold x402 payment projects from pluggable templates",
    version = VERSION,
    author = "x402 Community"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available x402 templates
    #[command(visible_alias = "ls")]
    List {
        /// Force refresh template cache (bypass TTL)
        #[arg(long)]
        refresh: bool,

        /// Filter templates by tags
        #[arg(long)]
        tags: Option<Vec<String>>,
    },

    /// Create a new x402 project from a template
    #[command(visible_alias = "new")]
    Create {
        /// Template GitHub URL or shorthand (e.g., user/repo or https://github.com/user/repo)
        #[arg(short, long)]
        template: Option<String>,

        /// Project name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Show version information
    #[command(visible_alias = "v")]
    Version,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::List { refresh, tags }) => commands::list::execute(refresh, tags).await,
        Some(Commands::Create { template, name }) => commands::create::execute(template, name).await,
        Some(Commands::Version) => {
            println!("cargo-x402 {}", VERSION);
            Ok(())
        }
        None => {
            // Interactive mode if no command provided
            println!("\n{}", "cargo-x402 • Scaffold x402 projects".cyan().bold());
            println!(
                "{}\n",
                "Use 'cargo-x402 --help' to see all options".dimmed()
            );
            commands::create::execute(None, None).await
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "❌".red(), e);
        process::exit(1);
    }
}
