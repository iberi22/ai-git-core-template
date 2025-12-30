use clap::Args;
use color_eyre::Result;
use gc_core::ports::{SystemPort, FileSystemPort, GitHubPort};
use console::style;
use serde::Serialize;

#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

#[derive(Serialize)]
struct CheckOutput {
    git_installed: bool,
    gh_cli_installed: bool,
    in_git_repo: bool,
    has_gh_token: bool,
    protocol_version: String,
    latest_protocol_version: String,
    update_available: bool,
    all_passed: bool,
}

pub async fn execute(
    args: CheckArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    github: &impl GitHubPort
) -> Result<()> {
    if !args.json {
        println!("{} Checking environment health...", style("hz").cyan()); // Heartbeat/Health icon
    }

    // 1. Check Git
    let git_version = system.run_command_output("git", &["--version".to_string()]).await;
    let git_installed = git_version.is_ok();

    if !args.json {
        print_status("Git Installed", git_installed);
    }

    // 2. Check Inside Git Repo
    let git_repo = system.run_command_output("git", &["rev-parse".to_string(), "--is-inside-work-tree".to_string()]).await;
    let in_git_repo = git_repo.is_ok();

    if !args.json {
        print_status("Inside Git Repo", in_git_repo);
    }

    // 3. Check GitHub CLI (gh) - Optional but recommended
    let gh_version = system.run_command_output("gh", &["--version".to_string()]).await;
    let gh_cli_installed = gh_version.is_ok();

    if !args.json {
        print_status("GitHub CLI (gh)", gh_cli_installed);
    }

    // 4. Check Environment Variables (simulate token check)
    // In a real scenario, we might check specifically for GITHUB_TOKEN or similar
    // For now, we assume if `gh` works, we might have auth, but let's check basic var
    let has_gh_token = std::env::var("GITHUB_TOKEN").is_ok() || std::env::var("GH_TOKEN").is_ok();

    if !args.json {
        print_status("GITHUB_TOKEN Set", has_gh_token);
        if !has_gh_token && gh_cli_installed {
             println!("      (Reliant on 'gh' auth status if token is missing)");
        }
    }

    // 5. Check Protocol Version
    let version_file = ".git-core-protocol-version";
    let protocol_version = if fs.exists(version_file).await.unwrap_or(false) {
        fs.read_file(version_file).await.unwrap_or_else(|_| "0.0.0".to_string()).trim().to_string()
    } else {
        "0.0.0".to_string()
    };

    if !args.json {
        println!("   {} Protocol Version: {}", style("ℹ").blue(), protocol_version);
    }

    // 6. Check Latest Version (Remote)
    let latest_protocol_version = github.get_file_content(
        "iberi22",
        "Git-Core-Protocol",
        "main",
        ".git-core-protocol-version"
    ).await.unwrap_or_else(|_| "unknown".to_string()).trim().to_string();

    let update_available = latest_protocol_version != "unknown" && protocol_version != latest_protocol_version;

    if !args.json {
        if latest_protocol_version == "unknown" {
            println!("   {} Could not fetch latest version (check internet/token)", style("!").yellow());
        } else if update_available {
            println!("   {} Update Available: {}", style("!").yellow(), latest_protocol_version);
            println!("      (Run 'gc update' to upgrade)");
        } else {
            println!("   {} Protocol is up to date", style("✓").green());
        }
    }

    let all_passed = git_installed && in_git_repo && !update_available; // Requirement includes being up to date

    if args.json {
        let output = CheckOutput {
            git_installed,
            gh_cli_installed,
            in_git_repo,
            has_gh_token,
            protocol_version,
            latest_protocol_version,
            update_available,
            all_passed,
        };
        println!("{}", serde_json::to_string(&output)?);
    } else {
        if all_passed {
            println!("\n{} Environment looks healthy!", style("✅").green());
        } else {
            println!("\n{} Environment has issues.", style("⚠️").yellow());
        }
    }

    Ok(())
}

fn print_status(name: &str, passed: bool) {
    if passed {
        println!("   {} {}", style("✓").green(), name);
    } else {
        println!("   {} {}", style("✗").red(), name);
    }
}
