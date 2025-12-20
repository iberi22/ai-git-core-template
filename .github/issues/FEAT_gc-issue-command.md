---
github_issue: 111
title: "[CLI] Implement gc issue Command with GitHub Sync"
labels:
  - enhancement
  - cli
  - jules
assignees: []
---
github_issue: 111

## 🎯 Objective

Add a new `gc issue` command to the Git-Core CLI that:
1. Creates issue files locally in `.github/issues/`
2. Syncs them to GitHub automatically
3. Cleans up closed issues from local folder

## 📁 Files to Create/Modify

### 1. Create `tools/git-core/gc-cli/src/commands/issue.rs`

```rust
use clap::{Args, Subcommand};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Args)]
pub struct IssueArgs {
    #[command(subcommand)]
    pub command: IssueCommand,
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// Create a new issue (local + GitHub)
    Create {
        /// Issue title
        #[arg(short, long)]
        title: String,

        /// Issue body
        #[arg(short, long)]
        body: Option<String>,

        /// Labels (comma-separated)
        #[arg(short, long)]
        labels: Option<String>,

        /// Assign to Jules
        #[arg(long)]
        jules: bool,

        /// Only create locally (don't sync to GitHub)
        #[arg(long)]
        local_only: bool,
    },

    /// Sync local issues to GitHub
    Sync {
        /// Push local issues to GitHub
        #[arg(long)]
        push: bool,

        /// Pull closed issues (delete local files)
        #[arg(long)]
        pull: bool,

        /// Dry run - don't make changes
        #[arg(long)]
        dry_run: bool,
    },

    /// List all issues
    List {
        /// Show only open issues
        #[arg(long)]
        open: bool,

        /// Show only closed issues
        #[arg(long)]
        closed: bool,
    },

    /// Clean up closed issues from local folder
    Clean {
        /// Dry run - show what would be deleted
        #[arg(long)]
        dry_run: bool,
    },
}

pub async fn execute(args: IssueArgs) -> Result<()> {
    match args.command {
        IssueCommand::Create { title, body, labels, jules, local_only } => {
            create_issue(title, body, labels, jules, local_only).await
        }
        IssueCommand::Sync { push, pull, dry_run } => {
            sync_issues(push, pull, dry_run).await
        }
        IssueCommand::List { open, closed } => {
            list_issues(open, closed).await
        }
        IssueCommand::Clean { dry_run } => {
            clean_closed_issues(dry_run).await
        }
    }
}

async fn create_issue(
    title: String,
    body: Option<String>,
    labels: Option<String>,
    jules: bool,
    local_only: bool,
) -> Result<()> {
    let issues_dir = PathBuf::from(".github/issues");
    fs::create_dir_all(&issues_dir)?;

    // Generate filename from title
    let safe_title = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();

    let prefix = if title.to_lowercase().contains("bug") { "BUG" }
        else if title.to_lowercase().contains("feat") { "FEAT" }
        else { "TASK" };

    let filename = format!("{}_{}.md", prefix, safe_title);
    let filepath = issues_dir.join(&filename);

    // Build labels
    let mut label_list = labels
        .map(|l| l.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(Vec::new);

    if jules {
        label_list.push("jules".to_string());
    }

    // Create issue content
    let content = format!(r#"---
title: "{}"
labels:
{}
assignees: []
---
github_issue: 111

{}
"#,
        title,
        label_list.iter().map(|l| format!("  - {}", l)).collect::<Vec<_>>().join("\n"),
        body.unwrap_or_else(|| "## Description\n\nTODO: Add description".to_string())
    );

    fs::write(&filepath, &content)?;
    println!("✅ Created: {}", filepath.display());

    // Sync to GitHub unless local_only
    if !local_only {
        sync_single_issue(&filepath, &title, &label_list).await?;
    }

    Ok(())
}

async fn sync_single_issue(
    filepath: &PathBuf,
    title: &str,
    labels: &[String],
) -> Result<()> {
    use std::process::Command;

    let label_args: Vec<String> = labels.iter()
        .flat_map(|l| vec!["--label".to_string(), l.clone()])
        .collect();

    let body = fs::read_to_string(filepath)?;

    let output = Command::new("gh")
        .args(["issue", "create", "--title", title, "--body", &body])
        .args(&label_args)
        .output()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout);
        println!("✅ Synced to GitHub: {}", url.trim());
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        eprintln!("⚠️  GitHub sync failed: {}", err);
    }

    Ok(())
}

async fn sync_issues(push: bool, pull: bool, dry_run: bool) -> Result<()> {
    if push || (!push && !pull) {
        println!("🔄 Pushing local issues to GitHub...");
        // Implementation similar to sync-issues.ps1
    }

    if pull || (!push && !pull) {
        clean_closed_issues(dry_run).await?;
    }

    Ok(())
}

async fn list_issues(open: bool, closed: bool) -> Result<()> {
    use std::process::Command;

    let state = if open { "open" }
        else if closed { "closed" }
        else { "all" };

    let output = Command::new("gh")
        .args(["issue", "list", "--state", state, "--limit", "50"])
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

async fn clean_closed_issues(dry_run: bool) -> Result<()> {
    use std::process::Command;

    let issues_dir = PathBuf::from(".github/issues");

    if !issues_dir.exists() {
        println!("No issues directory found");
        return Ok(());
    }

    // Get mapping file
    let mapping_path = issues_dir.join(".issue-mapping.json");
    let mapping: std::collections::HashMap<String, u64> = if mapping_path.exists() {
        let content = fs::read_to_string(&mapping_path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };

    let mut deleted = 0;

    for entry in fs::read_dir(&issues_dir)? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if !filename.ends_with(".md") || filename.starts_with('_') {
            continue;
        }

        if let Some(issue_num) = mapping.get(&filename) {
            // Check if issue is closed
            let output = Command::new("gh")
                .args(["issue", "view", &issue_num.to_string(), "--json", "state"])
                .output()?;

            let state_json = String::from_utf8_lossy(&output.stdout);
            if state_json.contains("\"CLOSED\"") {
                if dry_run {
                    println!("Would delete: {} (#{} is closed)", filename, issue_num);
                } else {
                    fs::remove_file(entry.path())?;
                    println!("🗑️  Deleted: {} (#{} is closed)", filename, issue_num);
                }
                deleted += 1;
            }
        }
    }

    if deleted == 0 {
        println!("✅ No closed issues to clean up");
    } else if dry_run {
        println!("\n📋 Would delete {} files. Run without --dry-run to execute.", deleted);
    } else {
        println!("\n✅ Cleaned {} closed issues", deleted);
    }

    Ok(())
}
```

### 2. Update `tools/git-core/gc-cli/src/main.rs`

Add issue command to CLI:

```rust
mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gc")]
#[command(about = "Git-Core Protocol CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Manage GitHub Issues
    Issue(commands::issue::IssueArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ... existing matches ...
        Commands::Issue(args) => commands::issue::execute(args).await,
    }
}
```

## ✅ Acceptance Criteria

- [ ] `gc issue create --title "Feature X" --jules` creates local + GitHub issue
- [ ] `gc issue sync` pushes new issues and cleans closed ones
- [ ] `gc issue clean` removes local files for closed GitHub issues
- [ ] `gc issue list` shows all issues
- [ ] `cargo check` passes
- [ ] Integration with existing sync-issues.ps1 mapping

## 🧪 Testing

```bash
cd tools/git-core
cargo build --release

# Test commands
./target/release/gc issue create --title "Test Issue" --body "Test body"
./target/release/gc issue list --open
./target/release/gc issue clean --dry-run
./target/release/gc issue sync
```

## 📋 Usage Examples

```bash
# Create issue and assign to Jules
gc issue create --title "[MVP] Implement Auth" --labels "enhancement" --jules

# Create local-only issue (for drafting)
gc issue create --title "WIP: Research" --local-only

# Sync all issues
gc issue sync

# Clean closed issues from local folder
gc issue clean

# Dry run (see what would be deleted)
gc issue clean --dry-run
```

## ⏱ Estimated Effort
6-8 hours
