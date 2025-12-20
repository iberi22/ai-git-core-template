use clap::Args;
use color_eyre::Result;
use gc_core::ports::{SystemPort, GitHubPort};
use console::style;
use crate::commands::{validate, report};

#[derive(Args, Debug)]
pub struct FinishArgs {
    /// Skip validation (Not recommended)
    #[arg(long)]
    pub skip_validate: bool,

    /// Skip report generation
    #[arg(long)]
    pub skip_report: bool,
}

pub async fn execute(
    args: FinishArgs,
    system: &impl SystemPort,
    github: &impl GitHubPort,
) -> Result<()> {
    println!("{} Finishing task...", style("🏁").cyan());

    // 1. Validate
    if !args.skip_validate {
        println!("\n{} Step 1: Validation", style("🔍").yellow());
        // We reuse validate command logic
        // Note: Validate currently relies on gc-validator crate logic which might be internal
        // For MVP we shell out or call internal if args allow.
        // ValidateCmd is simple `struct ValidateArgs {}`.
        // Let's perform a direct check or call the module.
        // Since validate::execute is async and public, we can call it.
        println!("   Running `gc validate`...");
        validate::execute(validate::ValidateCmd::Run {
            run_id: "latest".to_string(),
            last_hours: None,
            create_pr: false, // Don't create PR from validator, we will do it in finish flow or manually
        }).await?;
    } else {
        println!("   (Skipping validation)");
    }

    // 2. Git Status Check
    // Ensure we have commits to push
    let status_args = vec!["status".to_string(), "--porcelain".to_string()];
    let status = system.run_command_output("git", &status_args).await?;
    if !status.trim().is_empty() {
        println!("\n{} Warning: You have uncommitted changes.", style("⚠️").yellow());
        println!("   Please commit your changes before finishing.");
        // We could offer to auto-commit here in the future
        return Ok(());
    }

    // 3. Push
    println!("\n{} Step 2: Push to Remote", style("⬆️").blue());
    let branch_args = vec!["branch".to_string(), "--show-current".to_string()];
    let branch = system.run_command_output("git", &branch_args).await?;
    let branch = branch.trim();

    if branch.is_empty() {
        eprintln!("   Error: Detached HEAD or no branch.");
        return Ok(());
    }

    println!("   Pushing {}...", branch);
    let push_args = vec!["push".to_string(), "origin".to_string(), branch.to_string()];
    match system.run_command("git", &push_args).await {
        Ok(_) => println!("   {} Pushed successfully.", style("✓").green()),
        Err(e) => {
            eprintln!("   {} Push failed: {}", style("❌").red(), e);
            // Hint: maybe upstream is missing
             println!("   Tip: Try `git push --set-upstream origin {}` manually if this is a new branch.", branch);
             return Err(e.into());
        }
    }

    // 4. Report
    if !args.skip_report {
        println!("\n{} Step 3: AI Report", style("🤖").magenta());
        // Use Full report by default
        let report_cmd = report::ReportCmd::Full {
            pr: None, // Auto-detect
        };

        report::execute(report_cmd, system, github).await?;
    }

    println!("\n{} Task Finish Sequence Complete!", style("✨").green());
    Ok(())
}
