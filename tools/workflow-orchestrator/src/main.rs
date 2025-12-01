//! # Workflow Orchestrator
//! 
//! High-performance parallel workflow orchestrator for Git-Core Protocol.
//! Executes GitHub Actions analysis, validation, and continuous improvement tasks
//! with maximum parallelism using Tokio.

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod github;
mod analyzer;
mod validator;
mod reporter;
mod parallel;

#[derive(Parser, Debug)]
#[command(
    name = "workflow-orchestrator",
    author = "Git-Core Protocol",
    version,
    about = "High-performance workflow orchestrator with parallel execution",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// GitHub repository (owner/repo)
    #[arg(short, long, env = "GITHUB_REPOSITORY")]
    repo: Option<String>,

    /// GitHub token
    #[arg(short, long, env = "GITHUB_TOKEN")]
    token: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// Output format (json, markdown, terminal)
    #[arg(short, long, default_value = "terminal")]
    output: String,

    /// Maximum parallel tasks
    #[arg(long, default_value = "10")]
    max_parallel: usize,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Validate workflow runs and generate improvement report
    Validate {
        /// Workflow run ID to validate (or "latest" for most recent)
        #[arg(short, long, default_value = "latest")]
        run_id: String,

        /// Validate all workflows from last N hours
        #[arg(long)]
        last_hours: Option<u64>,

        /// Create PR with validation results
        #[arg(long, default_value = "true")]
        create_pr: bool,
    },

    /// Analyze all workflows in parallel
    Analyze {
        /// Types of analysis to run
        #[arg(short, long, value_delimiter = ',', default_value = "errors,performance,security")]
        types: Vec<String>,

        /// Include successful runs in analysis
        #[arg(long, default_value = "false")]
        include_success: bool,
    },

    /// Execute post-run validation (continuous improvement)
    PostRun {
        /// Workflow run ID that just completed
        #[arg(short, long)]
        run_id: String,

        /// Request AI reviews (CodeRabbit, Gemini, Copilot)
        #[arg(long, default_value = "true")]
        ai_review: bool,
    },

    /// Generate comprehensive report
    Report {
        /// Report type (summary, detailed, diff)
        #[arg(short, long, default_value = "detailed")]
        report_type: String,

        /// Time range in hours
        #[arg(long, default_value = "168")]
        hours: u64,
    },

    /// Health check for all workflows
    Health {
        /// Quick check (only status, no deep analysis)
        #[arg(long, default_value = "false")]
        quick: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();

    // Setup logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .compact()
        .init();

    info!("🚀 Workflow Orchestrator v{}", env!("CARGO_PKG_VERSION"));

    let token = cli.token.or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .expect("GITHUB_TOKEN required");
    
    let repo = cli.repo.or_else(|| std::env::var("GITHUB_REPOSITORY").ok())
        .expect("Repository required (--repo or GITHUB_REPOSITORY)");

    let github_client = github::GitHubClient::new(&token, &repo, cli.max_parallel);

    match cli.command {
        Commands::Validate { run_id, last_hours, create_pr } => {
            validator::run_validation(&github_client, &run_id, last_hours, create_pr, &cli.output).await?;
        }
        Commands::Analyze { types, include_success } => {
            analyzer::run_analysis(&github_client, &types, include_success, &cli.output).await?;
        }
        Commands::PostRun { run_id, ai_review } => {
            validator::post_run_validation(&github_client, &run_id, ai_review).await?;
        }
        Commands::Report { report_type, hours } => {
            reporter::generate_report(&github_client, &report_type, hours, &cli.output).await?;
        }
        Commands::Health { quick } => {
            analyzer::health_check(&github_client, quick).await?;
        }
    }

    info!("✅ Orchestrator completed successfully");
    Ok(())
}
