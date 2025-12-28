use clap::Args;
use color_eyre::Result;
use gc_core::ports::SystemPort;
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
    system: &impl SystemPort,
) -> Result<()> {
    println!("{}", style("🔄 Upgrading Git-Core Protocol...").cyan());

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
