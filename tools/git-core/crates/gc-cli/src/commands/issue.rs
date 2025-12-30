use clap::{Args, Subcommand};
use gc_core::ports::{GitHubPort, SystemPort, FileSystemPort};
use serde::Deserialize;
use console::style;

#[derive(Args, Debug)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueCommands,
}

#[derive(Subcommand, Debug)]
pub enum IssueCommands {
    /// List issues
    List {
        /// Filter by state (open, closed, all)
        #[arg(short, long, default_value = "open")]
        state: String,

        /// Filter by assignee
        #[arg(short, long)]
        assignee: Option<String>,

        /// Filter by assigned to me
        #[arg(long)]
        assigned_to_me: bool,

        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Create a new issue
    Create {
        /// Issue title
        #[arg(short, long)]
        title: String,

        /// Issue body
        #[arg(short, long)]
        body: Option<String>,

        /// Labels (comma separated)
        #[arg(short, long)]
        labels: Option<String>,
    },
    /// Sync local issue files (.github/issues/*.md) to GitHub
    Sync {
        /// Dry run (show what would be synced)
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Debug, Deserialize)]
struct IssueFrontmatter {
    title: String,
    labels: Option<Vec<String>>,
    #[allow(dead_code)]
    assignees: Option<Vec<String>>,
}

pub async fn execute(
    args: IssueArgs,
    github: &impl GitHubPort,
    system: &impl SystemPort,
    fs: &impl FileSystemPort,
) -> color_eyre::Result<()> {
    match args.command {
        IssueCommands::List { state, assignee, assigned_to_me, limit } => {
            // Detect repo
            let output = system.run_command_output("git", &["remote", "get-url", "origin"].map(|s| s.to_string())).await?;
            let (owner, repo) = parse_repo_from_url(&output)?;

            println!("{}", style(format!("Fetching issues for {}/{}...", owner, repo)).dim());

            let current_user;
            let effective_assignee: Option<String> = if assigned_to_me {
                current_user = github.check_auth().await?;
                Some(current_user)
            } else {
                assignee.clone()
            };

            let issues = github.list_issues(&owner, &repo, Some(state.clone()), effective_assignee).await?;

            if issues.is_empty() {
                println!("No issues found.");
                return Ok(());
            }

            for issue in issues.iter().take(limit) {
                let labels = issue.labels.join(", ");
                println!("#{} {} {} {}",
                    style(issue.number).green().bold(),
                    issue.title,
                    style(&issue.state).dim(),
                    if !labels.is_empty() { style(format!("[{}]", labels)).blue() } else { style("".to_string()) }
                );
            }
        }
        IssueCommands::Create { title, body, labels } => {
            let output = system.run_command_output("git", &["remote", "get-url", "origin"].map(|s| s.to_string())).await?;
            let (owner, repo) = parse_repo_from_url(&output)?;

            let labels_vec: Vec<String> = labels
                .map(|l| l.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_else(Vec::new);

            println!("🚀 Creating issue: {}...", style(&title).cyan());
            github.create_issue(&owner, &repo, &title, body.as_deref().unwrap_or(""), &labels_vec).await?;
            println!("✅ Issue created successfully!");
        }
        IssueCommands::Sync { dry_run } => {
            let output = system.run_command_output("git", &["remote", "get-url", "origin"].map(|s| s.to_string())).await?;
            let (owner, repo) = parse_repo_from_url(&output)?;

            let repo_root = system.run_command_output("git", &["rev-parse".into(), "--show-toplevel".into()]).await?.trim().to_string();
            println!("{}", style(format!("🔍 Scanning for local issue files in {}/.github/issues/...", repo_root)).dim());

            let files_output = if cfg!(windows) {
                system.run_command_output("powershell", &[
                    "-NoProfile".into(),
                    "-ExecutionPolicy".into(), "Bypass".into(),
                    "-Command".into(), format!("Get-ChildItem '{}/.github/issues/*.md' -Name", repo_root)
                ]).await?
            } else {
                system.run_command_output("find", &[format!("{}/.github/issues", repo_root), "-name".into(), "*.md".into()]).await?
            };

            for file in files_output.lines() {
                if file.starts_with('_') || file.starts_with('.') {
                    continue;
                }
                let path = if cfg!(windows) {
                    format!("{}/.github/issues/{}", repo_root, file)
                } else {
                    format!("{}/.github/issues/{}", repo_root, file)
                };
                let content = fs.read_file(&path).await?;

                // Simple frontmatter parser
                if content.starts_with("---") {
                    let parts: Vec<&str> = content.split("---").collect();
                    if parts.len() >= 3 {
                        let yaml = parts[1];
                        let body = parts[2..].join("---");
                        let frontmatter: IssueFrontmatter = serde_yaml::from_str(yaml)?;

                        if dry_run {
                            println!("Test Sync: {} -> {}", style(file).yellow(), style(&frontmatter.title).cyan());
                        } else {
                            println!("Syncing: {} -> {}...", style(file).yellow(), style(&frontmatter.title).cyan());
                            github.create_issue(
                                &owner,
                                &repo,
                                &frontmatter.title,
                                body.trim(),
                                frontmatter.labels.as_ref().unwrap_or(&vec![]),
                            ).await?;
                            println!("✅ Synced!");
                            // TODO: Move file to a 'synced' folder or add 'synced: true' to frontmatter to avoid duplicates
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_repo_from_url(url: &str) -> color_eyre::Result<(String, String)> {
    let url = url.trim();
    // Supports:
    // https://github.com/owner/repo.git
    // git@github.com:owner/repo.git

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
