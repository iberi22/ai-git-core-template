use clap::{Args, Subcommand};
use gc_core::ports::{GitHubPort, SystemPort};
use console::style;

#[derive(Args, Debug)]
pub struct PrArgs {
    #[command(subcommand)]
    pub command: PrCommands,
}

#[derive(Subcommand, Debug)]
pub enum PrCommands {
    /// List pull requests
    List {
        /// Filter by state (open, closed, all)
        #[arg(short, long, default_value = "open")]
        state: String,

        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}

pub async fn execute(
    args: PrArgs,
    github: &impl GitHubPort,
    system: &impl SystemPort,
) -> color_eyre::Result<()> {
    match args.command {
        PrCommands::List { state, limit } => {
            // Detect repo
            let output = system.run_command_output("git", &["remote", "get-url", "origin"].map(|s| s.to_string())).await?;
            let (owner, repo) = parse_repo_from_url(&output)?;

            println!("{}", style(format!("Fetching PRs for {}/{}...", owner, repo)).dim());

            let prs = github.list_prs(&owner, &repo, Some(state.clone())).await?;

            if prs.is_empty() {
                println!("No PRs found.");
                return Ok(());
            }

            for pr in prs.iter().take(limit) {
                println!("#{} {} [{}] ({})",
                    style(pr.number).green().bold(),
                    pr.title,
                    style(&pr.state).cyan(),
                    style(&pr.head_ref).dim()
                );
            }
        }
    }
    Ok(())
}

fn parse_repo_from_url(url: &str) -> color_eyre::Result<(String, String)> {
    let url = url.trim();
    let parts: Vec<&str> = if url.starts_with("git@") {
        url.split(':').nth(1).unwrap_or("").split('/').collect()
    } else {
        url.split("github.com/").nth(1).unwrap_or("").split('/').collect()
    };

    if parts.len() < 2 {
        return Err(color_eyre::eyre::eyre!("Could not parse repo from URL: {}", url));
    }

    let owner = parts[0].to_string();
    let repo = parts[1].trim_end_matches(".git").to_string();

    Ok((owner, repo))
}
