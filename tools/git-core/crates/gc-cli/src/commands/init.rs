use clap::Args;
use gc_core::ports::{FileSystemPort, SystemPort, GitHubPort};
use console::style;
use std::io::{self, Write};

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Project name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Create private repository
    #[arg(long)]
    pub private: bool,

    /// Non-interactive mode (Auto-approve)
    #[arg(short, long)]
    pub auto: bool,

    /// Organize existing files
    #[arg(short, long)]
    pub organize: bool,

    /// Force initialization (overwrite existing without prompt)
    #[arg(long)]
    pub force: bool,
}

pub async fn execute(
    args: InitArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    _github: &impl GitHubPort
) -> color_eyre::Result<()> {
    println!("{}", style("🧠 Initializing Git-Core Protocol...").cyan());
    println!("{}", style("==========================================").cyan());

    // 1. Validation Logic
    println!("\n{}", style("📋 Validating environment...").yellow());
    let required_tools = vec![("git", "Git"), ("gh", "GitHub CLI")];
    for (bin, label) in &required_tools {
        if !system.check_command(bin).await? {
            eprintln!("{}", style(format!("❌ Error: {} is not installed.", label)).red());
            return Err(color_eyre::eyre::eyre!("{} missing", label));
        }
    }
    // Validation Passed
    println!("{}", style("✓ Core tools installed").green());

    // 2. Target Resolution
    let target_path = args.name.as_deref().unwrap_or(".").to_string();
    let is_current_dir = target_path == ".";

    // 3. Recommended Tools Check
    let recommended_tools = vec![
        ("gemini", "Gemini CLI"),
        ("copilot", "GitHub Copilot CLI"),
        ("jules", "Jules CLI"),
    ];
    let mut missing_recommended = vec![];
    for (bin, label) in &recommended_tools {
        if !system.check_command(bin).await? {
            println!("{}", style(format!("⚠️  {} is missing (Recommended)", label)).yellow());
            missing_recommended.push(label.to_string());
        } else {
            println!("{}", style(format!("✓ {} installed", label)).green());
        }
    }

    if !missing_recommended.is_empty() {
        println!("{}", style("\nSome agents may not function fully without these tools.").dim());
        if !args.auto {
             println!("Do you want to proceed anyway? [Y/n]");
             if !confirm_user() {
                 println!("{}", style("Aborted by user.").red());
                 return Ok(());
             }
        }
    }

    // 4. Existing State Detection
    let git_check_path = if is_current_dir { ".git".to_string() } else { format!("{}/.git", target_path) };
    let git_exists = fs.exists(&git_check_path).await?;
    let files = fs.list_files(&target_path, None).await.unwrap_or_default();
    let is_empty = files.is_empty();

    if !is_empty && !args.auto && !args.force {
        println!("\n{}", style(format!("⚠️  Target '{}' is not empty.", target_path)).yellow());
        if git_exists {
            println!("{}", style("ℹ️  Existing Git repository detected.").cyan());
        } else {
             println!("{}", style("ℹ️  Existing files detected.").cyan());
        }

        println!("How do you want to proceed?");
        println!("1. [C]ancel");
        println!("2. [B]ackup existing files (Move to ./_backup_TIMESTAMP)");
        println!("3. [O]vwerwrite/Update (Keep files, just add protocol)");

        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_lowercase();

        if choice.starts_with('c') || choice == "1" {
            println!("{}", style("Aborted.").red());
            return Ok(());
        } else if choice.starts_with('b') || choice == "2" {
            // Backup Logic
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
            let backup_dir = if is_current_dir {
                format!("_backup_{}", timestamp)
            } else {
                format!("{}/_backup_{}", target_path, timestamp)
            };

            println!("{}", style(format!("📦 Moving files to {}...", backup_dir)).yellow());
            fs.create_dir(&backup_dir).await?;

            // Move all files except the backup dir itself
            for file in files {
                if file != backup_dir && file != ".git" {
                     let source = if is_current_dir { file.clone() } else { format!("{}/{}", target_path, file) };
                     let dest = if is_current_dir { format!("{}/{}", backup_dir, file) } else { format!("{}/{}/{}", target_path, backup_dir, file) };
                     let _ = fs.move_file(&source, &dest).await;
                }
            }
            println!("{}", style("✓ Backup complete").green());

        } else if choice.starts_with('o') || choice == "3" {
            println!("{}", style("ℹ️  Proceeding with update/overwrite...").blue());
        } else {
            println!("{}", style("Invalid choice. Aborted.").red());
            return Ok(());
        }
    }

    // 5. Initialize Git (If needed)
    if !fs.exists(&git_check_path).await? {
        println!("\n{}", style(format!("🔧 Initializing Git repository in {}...", target_path)).yellow());
        if !is_current_dir {
             let _ = system.run_command("git", &vec!["init".into(), target_path.clone()]).await;
        } else {
             let _ = system.run_command("git", &vec!["init".into()]).await;
        }
        let _ = system.run_command("git", &vec!["branch".into(), "-M".into(), "main".into()]).await;

        // Initial Commit for freshness? Or just leave it.
        // Original logic had commit. Let's add it back for consistency if it's new repo.
        let readme_path = if is_current_dir { "README.md".to_string() } else { format!("{}/README.md", target_path) };
        if !fs.exists(&readme_path).await? {
            fs.write_file(&readme_path, "# Project Initialized by Git-Core").await?;
        }
        let _ = system.run_command("git", &vec!["add".into(), ".".into()]).await;
        let _ = system.run_command("git", &vec!["commit".into(), "-m".into(), "feat: 🚀 Initial commit".into()]).await;
    }

    // 4. Artifact Setup
    setup_artifacts(&target_path, is_current_dir, fs, system, args.force).await?;

    // 5. GitHub Items
    setup_github_items(&target_path, is_current_dir, system).await?;

    // 6. Hooks
    install_hooks(&target_path, is_current_dir, fs).await?;

    println!("\n{}", style("✅ Project initialized successfully!").green());
    Ok(())
}

fn confirm_user() -> bool {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    if io::stdin().read_line(&mut input).is_ok() {
        let t = input.trim().to_lowercase();
        return t == "y" || t == "yes" || t.is_empty();
    }
    false
}

async fn setup_artifacts(
    target_path: &str,
    is_current: bool,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    force: bool
) -> color_eyre::Result<()> {
    let arch_dir = if is_current { ".gitcore".to_string() } else { format!("{}/.gitcore", target_path) };
    let github_dir = if is_current { ".github".to_string() } else { format!("{}/.github", target_path) };

    // Ensure dirs
    if !fs.exists(&arch_dir).await? { fs.create_dir(&arch_dir).await?; }
    if !fs.exists(&github_dir).await? { fs.create_dir(&github_dir).await?; }

    // 1. ARCHITECTURE.md
    let arch_path = format!("{}/ARCHITECTURE.md", arch_dir);
    if force || !fs.exists(&arch_path).await? {
        println!("{}", style("📐 Setting up ARCHITECTURE.md...").yellow());
        let default_content = r#"# 🏗️ Architecture

## Stack
- **Language:** TBD
- **Framework:** TBD

## Key Decisions
_Document architectural decisions here_
"#;

        // Fetch
        let cmd = "gh";
        let args = vec![
            "api".to_string(),
            "-H".to_string(), "Accept: application/vnd.github.v3.raw".to_string(),
            "/repos/iberi22/Git-Core-Protocol/contents/.gitcore/ARCHITECTURE.md?ref=main".to_string()
        ];

        let content = match system.run_command_output(cmd, &args).await {
            Ok(c) if !c.trim().is_empty() => {
                 println!("{}", style("✓ Fetched latest Architecture from remote").green());
                 c
            },
            _ => {
                 println!("{}", style("⚠️  Could not fetch Architecture (CLI), using default").yellow());
                 default_content.to_string()
            }
        };
        fs.write_file(&arch_path, &content).await?;
    }

    // 2. AGENT_INDEX.md
    let agent_index_path = format!("{}/AGENT_INDEX.md", arch_dir);
    if force || !fs.exists(&agent_index_path).await? {
        let default_content = r#"# 🤖 Agent Index

| Agent | Description | Trigger |
|-------|-------------|---------|
| `@copilot` | General assistance | Default |
| `@architect` | Architecture changes | Planning phase |
| `@jules` | Autonomous execution | `jules` label |
"#;
        // Fetch
        let cmd = "gh";
        let args = vec![
            "api".to_string(),
            "-H".to_string(), "Accept: application/vnd.github.v3.raw".to_string(),
            "/repos/iberi22/Git-Core-Protocol/contents/.gitcore/AGENT_INDEX.md?ref=main".to_string()
        ];

        let content = match system.run_command_output(cmd, &args).await {
             Ok(c) if !c.trim().is_empty() => {
                 println!("{}", style("✓ Fetched latest Agent Index from remote").green());
                 c
             },
             _ => {
                 println!("{}", style("⚠️  Could not fetch Agent Index (CLI), using default").yellow());
                 default_content.to_string()
             }
        };
        fs.write_file(&agent_index_path, &content).await?;
    }

    // 3. Agent Rules (copilot-instructions.md)
    let instructions_path = format!("{}/copilot-instructions.md", github_dir);
    if force || !fs.exists(&instructions_path).await? {
         println!("{}", style("📜 Installing Agent Rules...").yellow());
         let default_content = r#"# 🧠 GitHub Copilot Instructions (Offline Fallback)

## Prime Directive
You are operating under the **Git-Core Protocol**.

## 🚀 Quick Commands
| `gc init` | Initialize |
| `gc issue list` | List Tasks |

See [docs/agent-docs/CLI_GUIDE.md](../docs/agent-docs/CLI_GUIDE.md) for full guide.
"#;
         // Fetch
         let cmd = "gh";
         let args = vec![
            "api".to_string(),
            "-H".to_string(), "Accept: application/vnd.github.v3.raw".to_string(),
            "/repos/iberi22/Git-Core-Protocol/contents/.github/copilot-instructions.md?ref=main".to_string()
         ];

         let content = match system.run_command_output(cmd, &args).await {
              Ok(c) if !c.trim().is_empty() => {
                  println!("{}", style("✓ Fetched latest Agent Rules from remote").green());
                  c
              },
              _ => {
                  println!("{}", style("⚠️  Could not fetch Agent Rules (CLI), using default").yellow());
                  default_content.to_string()
              }
         };
         fs.write_file(&instructions_path, &content).await?;
    }

    // Protocol Version
    let version_path = if is_current { ".git-core-protocol-version".to_string() } else { format!("{}/.git-core-protocol-version", target_path) };
    if force || !fs.exists(&version_path).await? {
         // Fetch
         let cmd = "gh";
         let args = vec![
            "api".to_string(),
            "-H".to_string(), "Accept: application/vnd.github.v3.raw".to_string(),
            "/repos/iberi22/Git-Core-Protocol/contents/.git-core-protocol-version?ref=main".to_string()
         ];

         let latest = match system.run_command_output(cmd, &args).await {
             Ok(c) if !c.trim().is_empty() => c.trim().to_string(),
             _ => "3.0.0".to_string()
         };

         fs.write_file(&version_path, &latest).await?;
         println!("{}", style(format!("✓ Installed Protocol Version {}", latest)).green());
    }

    Ok(())
}

async fn setup_github_items(
    _target_path: &str,
    is_current: bool,
    system: &impl SystemPort
) -> color_eyre::Result<()> {
    if !is_current { return Ok(()); }

    println!("\n{}", style("🏷️  Creating semantic labels...").yellow());
    let labels = vec![
        ("ai-plan", "High-level planning tasks", "0E8A16"),
        ("ai-context", "Critical context information", "FBCA04"),
        ("ai-blocked", "Blocked - requires human intervention", "D93F0B"),
        ("in-progress", "Task in progress", "1D76DB"),
        ("needs-review", "Requires review", "5319E7"),
    ];

    for (name, desc, color) in labels {
         let _ = system.run_command("gh", &vec![
            "label".into(), "create".into(), name.into(),
            "--description".into(), desc.into(),
            "--color".into(), color.into(), "--force".into()
         ]).await;
    }
    Ok(())
}

async fn install_hooks(
    target_path: &str,
    is_current: bool,
    fs: &impl FileSystemPort
) -> color_eyre::Result<()> {
    let git_dir = if is_current { ".git".to_string() } else { format!("{}/.git", target_path) };
    if fs.exists(&git_dir).await? {
        println!("\n{}", style("🪝 Installing pre-commit hooks...").yellow());
        let hooks_dir = format!("{}/hooks", git_dir);
        if !fs.exists(&hooks_dir).await? { fs.create_dir(&hooks_dir).await?; }

        let hook_content = r#"#!/bin/bash
# Git-Core Protocol pre-commit hook
REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOK_SCRIPT="$REPO_ROOT/scripts/hooks/pre-commit"
if [ -f "$HOOK_SCRIPT" ]; then exec bash "$HOOK_SCRIPT"; else exit 0; fi
"#;
        fs.write_file(&format!("{}/pre-commit", hooks_dir), hook_content).await?;
         println!("{}", style("✓ Pre-commit hooks installed").green());
    }
    Ok(())
}
