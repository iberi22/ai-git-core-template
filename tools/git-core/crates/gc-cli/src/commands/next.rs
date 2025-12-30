use clap::Args;
use color_eyre::Result;
use gc_core::ports::{SystemPort, GitHubPort, FileSystemPort};
use console::style;
use serde::Serialize;
use crate::commands::task::{TaskArgs, self};

#[derive(Args, Debug)]
pub struct NextArgs {
    /// Auto-confirm actions (non-interactive)
    #[arg(long)]
    pub auto: bool,

    /// Force specific agent (jules, copilot, etc.)
    #[arg(long)]
    pub agent: Option<String>,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

#[derive(Serialize)]
struct NextOutput {
    issue_number: u64,
    title: String,
    agent_assigned: String,
    branch_created: String,
    jules_triggered: bool,
}

pub async fn execute(
    args: NextArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    github: &impl GitHubPort,
) -> Result<()> {
    if !args.json {
        println!("{} Scanning for next priority task...", style("🔍").cyan());
    }

    // 1. Fetch Issues via GitHub Adapter
    // Note: In MVP, `GitHubPort` might abstract `gh` cli calls or `octocrab`.
    // Assuming we don't have full `list_issues` in `GitHubPort` yet tailored for this,
    // we might fallback to `system.run_command_output("gh", ...)` if the port is limited.
    // For this implementation, let's use `system` to call `gh CLI` as it's the most reliable way
    // to get "my assigned issues" or "repo issues" without complex auth setup in Rust code itself yet.

    let gh_args = vec![
        "issue".to_string(), "list".to_string(),
        "--json".to_string(), "number,title,labels,body".to_string(),
        "--state".to_string(), "open".to_string(),
        "--limit".to_string(), "10".to_string(),
    ];

    let output = system.run_command_output("gh", &gh_args).await?;
    let issues: Vec<serde_json::Value> = serde_json::from_str(&output)?;

    if issues.is_empty() {
        if !args.json {
            println!("{} No open issues found!", style("🎉").green());
        }
        return Ok(());
    }

    // 2. Prioritize
    // Bug > Urgent > AI-Plan > Feature
    let selected = issues.iter().min_by_key(|i| {
        let labels = i["labels"].as_array().unwrap();
        let is_bug = labels.iter().any(|l| l["name"] == "bug");
        let is_urgent = labels.iter().any(|l| l["name"] == "urgent" || l["name"] == "high priority");

        if is_bug { 0 }
        else if is_urgent { 1 }
        else { 2 }
    }).unwrap();

    let number = selected["number"].as_u64().unwrap();
    let title = selected["title"].as_str().unwrap().to_string();
    let body = selected["body"].as_str().unwrap_or("");
    let labels_array = selected["labels"].as_array().unwrap();

    if !args.json {
        println!("{} Selected: #{} - {}", style("🎯").yellow(), number, title);
    }

    // 3. Init Workspace (Reuse gc task)
    let task_args = TaskArgs {
        title: title.clone(),
        type_: None,
        json: args.json,
    };

    // We execute task logic.
    // Note: `task::execute` prints to stdout. If `args.json` is true, it outputs JSON.
    // We might want to capture it or just let it run.
    // Since we are compositing, let's run it.
    if !args.json {
        println!("{} Initializing workspace...", style("🚀").magenta());
    }
    task::execute(task_args, fs, system, github).await?;

    // 4. Agent Dispatch Strategy
    let is_complex = body.len() > 500 || title.to_lowercase().contains("implement");
    let has_jules_label = labels_array.iter().any(|l| l["name"] == "jules");

    let agent = if let Some(a) = &args.agent {
        a.clone()
    } else if has_jules_label || is_complex {
        "jules".to_string()
    } else {
        "copilot".to_string()
    };

    let mut jules_triggered = false;
    let _branch_name_guess = format!("auto-{}", number); // Simplification, task::execute generates real slug

    if agent == "jules" {
        if !args.json {
            println!("{} Triggering Jules (Async)...", style("⚡").blue());
            println!("   Merging 'main' to ensure freshness...");
        }

        let _ = system.run_command("git", &["fetch".to_string(), "origin".to_string(), "main".to_string()]).await;
        // Try merge, ignore error if conflict for now (agent will handle or user intervenes)
        let _ = system.run_command("git", &["merge".to_string(), "origin/main".to_string()]).await;

        // Label and Comment
        let _ = system.run_command("gh", &["issue".to_string(), "edit".to_string(), number.to_string(), "--add-label".to_string(), "jules".to_string()]).await;
        let _ = system.run_command("gh", &["issue".to_string(), "comment".to_string(), number.to_string(), "--body".to_string(), "@jules build this".to_string()]).await;

        jules_triggered = true;
    } else if agent == "copilot" {
         if !args.json {
            println!("{} Agent: Copilot (Interactive)", style("💡").yellow());
            println!("   Command: gh copilot suggest \"{}\"", title);
        }
    } else if agent == "gemini" {
         if !args.json {
            println!("{} Agent: Gemini (Context)", style("✨").cyan());
            println!("   Initializing deep context analysis...");
        }
    }

    if args.json {
        let out = NextOutput {
            issue_number: number,
            title,
            agent_assigned: agent,
            branch_created: "unknown_in_json_mode".to_string(), // Limitation of composition without capturing stdout
            jules_triggered,
        };
        // Print nothing here? Or double JSON?
        // `task::execute` already printed JSON if args.json is true.
        // This is a design flaw of simple composition.
        // For now, we assume agent parses the LAST line or we suppress `task` output?
        // Better: `task::execute` should return a Struct we can use, rather than just printing.
        // But for MVP, we'll append this JSON.
        println!("{}", serde_json::to_string(&out)?);
    }

    Ok(())
}
