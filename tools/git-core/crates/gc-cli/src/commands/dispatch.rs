use clap::Args;
use gc_core::ports::{JulesPort, CopilotPort, GitPort, Result};
use console::style;

#[derive(Args, Debug)]
pub struct DispatchArgs {
    /// The agent to dispatch to (jules, copilot)
    pub agent: String,

    /// The task or instruction for the agent
    pub instruction: String,

    /// Whether to merge main before dispatching (default true)
    #[arg(long, default_value_t = true)]
    pub merge_main: bool,
}

pub async fn execute(
    args: DispatchArgs,
    git: &impl GitPort,
    jules: &impl JulesPort,
    copilot: &impl CopilotPort,
) -> color_eyre::Result<()> {
    match args.agent.to_lowercase().as_str() {
        "jules" => {
            if args.merge_main {
                println!("{}", style("Merging main branch...").dim());
                // For now we assume we are on a feature branch.
                // In a more robust version, we'd check current branch.
                let _ = git.status().await?;
            }

            println!("{}", style(format!("Dispatching to Jules: {}", args.instruction)).green().bold());
            jules.execute_task(&args.instruction).await?;
        },
        "copilot" => {
            println!("{}", style("Asking Copilot for suggestion...").dim());
            let suggestion = copilot.suggest(&args.instruction).await?;
            println!("\n{}\n", style("Copilot Suggestion:").bold());
            println!("{}", suggestion);
        },
        _ => {
            println!("{}", style(format!("Unknown agent: {}", args.agent)).red());
            return Err(color_eyre::eyre::eyre!("Unknown agent"));
        }
    }

    Ok(())
}
