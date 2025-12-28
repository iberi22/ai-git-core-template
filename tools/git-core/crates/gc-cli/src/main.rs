use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gc", version, about = "Git-Core Protocol CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

mod commands;
use commands::{InitArgs, ContextCmd, ReportCmd, ValidateCmd, TelemetryArgs, CiDetectArgs, TaskArgs, FinishArgs, IssueArgs, PrArgs, GitArgs, InfoArgs, CheckArgs, NextArgs, WorkflowArgs};

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init(InitArgs),
    /// Manage Agent Context
    Context {
        #[command(subcommand)]
        subcmd: ContextCmd,
    },
    /// Generate AI Reports
    #[command(subcommand)]
    Report(ReportCmd),
    /// Collect and Send Telemetry
    Telemetry(TelemetryArgs),
    /// Detect CI Environment
    CiDetect(CiDetectArgs),
    /// Validate Workflows
    #[command(subcommand)]
    Validate(ValidateCmd),
    /// Execute Workflows
    Workflow(WorkflowArgs),
    /// Start a new Task (Simplicity)
    Task(TaskArgs),
    /// Finish current Task (Automation)
    Finish(FinishArgs),
    /// Manage Issues
    Issue(IssueArgs),
    /// Manage Pull Requests
    Pr(PrArgs),
    /// Git Context
    Git(GitArgs),
    /// Project Info
    Info(InfoArgs),
    /// Verify Environment Health
    Check(CheckArgs),
    /// Select Next Task (Dispatcher)
    Next(NextArgs),
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            commands::init::execute(args, &fs, &system).await?;
        }
        Commands::Context { subcmd } => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let github = gc_adapter_github::OctocrabGitHub::new();
            commands::context::execute(subcmd, &fs, &github).await?;
        }
        Commands::Report(args) => {
            let _fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            let github = gc_adapter_github::OctocrabGitHub::new();
            // TODO: Refactor adapter instantiation to be shared or dependency injection container
            commands::report::execute(args, &system, &github).await?;
        }
        Commands::Telemetry(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::telemetry::execute(args, &system).await?;
        }
        Commands::CiDetect(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::ci_detect::execute(args, &system).await?;
        }
        Commands::Validate(args) => {
            commands::validate::execute(args).await?;
        }
        Commands::Workflow(args) => {
            let fs = gc_adapter_fs::TokioFileSystem;
            commands::workflow::execute(args, &fs).await?;
        }
        Commands::Task(args) => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            // Reusing context logic for auto-equip
            let github = gc_adapter_github::OctocrabGitHub::new();
            commands::task::execute(args, &fs, &system, &github).await?;
        }
        Commands::Finish(args) => {
            let system = gc_adapter_system::TokioSystem;
            let github = gc_adapter_github::OctocrabGitHub::new();
            commands::finish::execute(args, &system, &github).await?;
        }
        Commands::Issue(args) => {
            let github = gc_adapter_github::OctocrabGitHub::new();
            let system = gc_adapter_system::TokioSystem;
            commands::issue::execute(args, &github, &system).await?;
        }
        Commands::Pr(args) => {
            let github = gc_adapter_github::OctocrabGitHub::new();
            let system = gc_adapter_system::TokioSystem;
            commands::pr::execute(args, &github, &system).await?;
        }
        Commands::Git(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::git::execute(args, &system).await?;
        }
        Commands::Info(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::info::execute(args, &system).await?;
        }
        Commands::Check(args) => {
            let system = gc_adapter_system::TokioSystem;
            commands::check::execute(args, &system).await?;
        }
        Commands::Next(args) => {
            let fs = gc_adapter_fs::TokioFileSystem;
            let system = gc_adapter_system::TokioSystem;
            let github = gc_adapter_github::OctocrabGitHub::new(); // Or Stub if not needed mostly
            commands::next::execute(args, &fs, &system, &github).await?;
        }
    }

    Ok(())
}
