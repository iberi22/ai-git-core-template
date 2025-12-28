use clap::Args;
use color_eyre::Result;
use gc_core::ports::{SystemPort, FileSystemPort, GitHubPort};
use console::style;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Force update (overwrites ARCHITECTURE.md)
    #[arg(long)]
    pub force: bool,

    /// Non-interactive mode
    #[arg(short, long)]
    pub auto: bool,
}

pub async fn execute(
    args: UpdateArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    github: &impl GitHubPort,
) -> Result<()> {
    println!("{}", style("🔄 Upgrading Git-Core Protocol...").cyan());

    // 1. Version Check
    let version_file = ".git-core-protocol-version";
    let local_version = if fs.exists(version_file).await.unwrap_or(false) {
        fs.read_file(version_file).await.unwrap_or_else(|_| "0.0.0".to_string()).trim().to_string()
    } else {
        "0.0.0".to_string()
    };

    let latest_version = github.get_file_content(
        "iberi22",
        "Git-Core-Protocol",
        "main",
        ".git-core-protocol-version"
    ).await.unwrap_or_else(|_| "unknown".to_string()).trim().to_string();

    if local_version == latest_version && !args.force {
        println!("{} Protocol is already at version {} (latest).", style("✅").green(), local_version);
        println!("   Use --force if you want to reinstall anyway.");
        return Ok(());
    }

    if latest_version != "unknown" {
        println!("{} Update available: {} → {}", style("ℹ").blue(), local_version, latest_version);
    }


    // We use the remote installer as the source of truth for the upgrade logic
    // This ensures we always get the latest files and logic without updating the CLI first

    // Set environment variables for the child process
    if args.force {
        std::env::set_var("GIT_CORE_FORCE", "1");
        println!("{}", style("⚠️  Force mode enabled: All files will be overwritten.").red());
    } else {
        std::env::set_var("GIT_CORE_UPGRADE", "1");
    }

    if args.auto {
        std::env::set_var("GIT_CORE_AUTO", "1");
    }

    let cmd = "powershell";
    let ps_args = vec![
        "-ExecutionPolicy".to_string(),
        "Bypass".to_string(),
        "-Command".to_string(),
        "irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex".to_string()
    ];

    println!("{}", style("📥 Fetching and running remote installer...").yellow());

    // Run the command. The installer handles output.
    system.run_command(cmd, &ps_args).await?;

    Ok(())
}
