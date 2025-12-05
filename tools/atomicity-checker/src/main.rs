//! # Atomicity Checker
//!
//! High-performance commit atomicity analyzer for Git-Core Protocol.
//! Validates that commits follow the atomic commit principle:
//! "One commit, one concern."
//!
//! ## Usage
//! ```bash
//! atomicity-checker check --base main --head feature-branch
//! atomicity-checker report --format markdown
//! ```

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod git;
mod analyzer;
mod reporter;

#[derive(Parser, Debug)]
#[command(
    name = "atomicity-checker",
    author = "Git-Core Protocol",
    version,
    about = "High-performance commit atomicity checker",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// Output format (terminal, markdown, json)
    #[arg(short, long, default_value = "terminal")]
    output: String,

    /// Path to config file
    #[arg(short, long, default_value = ".github/atomicity-config.yml")]
    config: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check commits between two refs for atomicity
    Check {
        /// Base ref (e.g., main, origin/main)
        #[arg(short, long, env = "GITHUB_BASE_REF")]
        base: Option<String>,

        /// Head ref (defaults to HEAD)
        #[arg(short = 'H', long, default_value = "HEAD")]
        head: String,

        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo: String,
    },

    /// Analyze a single commit
    Analyze {
        /// Commit SHA to analyze
        #[arg(short, long)]
        commit: String,

        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo: String,
    },

    /// Generate a report for CI/CD
    Report {
        /// Base ref
        #[arg(short, long, env = "GITHUB_BASE_REF")]
        base: Option<String>,

        /// Head ref
        #[arg(short = 'H', long, default_value = "HEAD")]
        head: String,

        /// Repository path
        #[arg(short, long, default_value = ".")]
        repo: String,

        /// Output file (stdout if not specified)
        #[arg(short = 'f', long)]
        file: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    // Setup logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .compact()
        .init();

    info!("🔍 Atomicity Checker v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = config::Config::load(&cli.config)?;

    if !config.enabled {
        info!("⏭️ Atomicity check is disabled in configuration");
        return Ok(());
    }

    match cli.command {
        Commands::Check { base, head, repo } => {
            let base_ref = base.or_else(|| std::env::var("GITHUB_BASE_REF").ok())
                .unwrap_or_else(|| "main".to_string());

            let result = analyzer::check_atomicity(&repo, &base_ref, &head, &config).await?;
            reporter::print_result(&result, &cli.output, &config)?;

            // Exit with error code if issues found and mode is error
            if result.has_issues && config.mode == config::Mode::Error {
                std::process::exit(1);
            }
        }
        Commands::Analyze { commit, repo } => {
            let result = analyzer::analyze_single_commit(&repo, &commit, &config).await?;
            reporter::print_commit_analysis(&result, &cli.output)?;
        }
        Commands::Report { base, head, repo, file } => {
            let base_ref = base.or_else(|| std::env::var("GITHUB_BASE_REF").ok())
                .unwrap_or_else(|| "main".to_string());

            let result = analyzer::check_atomicity(&repo, &base_ref, &head, &config).await?;
            reporter::generate_report(&result, &cli.output, file.as_deref(), &config)?;
        }
    }

    info!("✅ Atomicity check completed");
    Ok(())
}
