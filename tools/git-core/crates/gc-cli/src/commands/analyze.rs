use clap::Args;
use std::fs;
use std::path::{Path, PathBuf};
use console::style;
use chrono::Local;
use std::io::Write;

#[derive(Args, Debug)]
pub struct AnalyzeArgs {
    /// Skip files larger than this size in KB
    #[arg(long, default_value = "50")]
    pub max_file_size_kb: u64,

    /// Output directory for the prompt file
    #[arg(long, default_value = "docs/prompts")]
    pub output_dir: String,

    /// Target path to analyze (defaults to current directory)
    #[arg(default_value = ".")]
    pub target_path: PathBuf,
}

pub async fn execute(args: AnalyzeArgs) -> color_eyre::Result<()> {
    println!("{}", style("🧠 Git-Core Architecture Analyzer").cyan().bold());
    println!("{}", style("=================================").cyan().bold());

    let target_root = &args.target_path;
    let output_dir_path = target_root.join(&args.output_dir);
    if !output_dir_path.exists() {
        fs::create_dir_all(&output_dir_path)?;
    }

    let timestamp = Local::now().format("%Y%m%d_%H%M");
    let output_file_name = format!("ARCHITECTURE_REVIEW_{}.md", timestamp);
    let output_path = output_dir_path.join(output_file_name);

    println!("{}", style("🔍 Gathering project context...").yellow());

    let project_tree = get_project_tree(target_root)?;
    let readme = get_file_content(target_root, "README.md", args.max_file_size_kb);

    let architecture_path = if target_root.join(".gitcore/ARCHITECTURE.md").exists() {
        ".gitcore/ARCHITECTURE.md"
    } else {
        ".ai/ARCHITECTURE.md"
    };
    let architecture = get_file_content(target_root, architecture_path, args.max_file_size_kb);
    let agents = get_file_content(target_root, "AGENTS.md", args.max_file_size_kb);

    // We try to grab install.ps1 if it exists, as it's a key script
    let install_script = get_file_content(target_root, "install.ps1", args.max_file_size_kb);

    let prompt_content = format!(
r#"# Architecture Analysis Request

**Context:** I need you to act as a Senior Software Architect and review the following project.
Your goal is to understand the project structure, current architecture, and goals, and then provide a robust architecture assessment.

## Project Structure
```text
{}
```

## Key Documentation

{}

{}

{}

## Core Scripts

{}

## Instructions for AI

1.  **Analyze the Structure**: Does the folder structure make sense for the project type?
2.  **Review the Architecture**: Look for gaps in the `ARCHITECTURE.md`. Are key decisions missing?
3.  **Check Consistency**: Do the `AGENTS.md` rules align with the code structure?
4.  **Recommendations**: Provide concrete steps to improve the robustness of the system.
5.  **Diagram**: Generate a Mermaid diagram representing the high-level system components if possible.
"#,
        project_tree,
        readme,
        architecture,
        agents,
        install_script
    );

    let mut file = fs::File::create(&output_path)?;
    file.write_all(prompt_content.as_bytes())?;

    let absolute_path = output_path.canonicalize()?;

    println!("");
    println!("{}", style("✅ Analysis Prompt Generated!").green().bold());
    println!("   File: {}", style(absolute_path.display()).white());
    println!("");
    println!("{}", style("🚀 HOW TO USE:").yellow().bold());
    println!("   1. Open the file above.");
    println!("   2. Copy the entire content.");
    println!("   3. Paste it into your AI chat (Copilot, ChatGPT, Claude).");
    println!("   4. Ask follow-up questions based on the analysis.");
    println!("");

    match copypasta::ClipboardContext::new() {
        Ok(mut ctx) => {
             use copypasta::ClipboardProvider;
             let file_ref = format!("#file:{}", absolute_path.display().to_string().replace("\\", "/"));
             if let Err(e) = ctx.set_contents(file_ref) {
                 println!("{}", style(format!("⚠️  Could not copy to clipboard: {}", e)).yellow());
             } else {
                 println!("{}", style("📋 File reference copied to clipboard!").dim());
             }
        },
        Err(_) => {
             println!("{}", style("⚠️  Clipboard access not available").dim());
        }
    }

    Ok(())
}

fn get_project_tree(root: &Path) -> color_eyre::Result<String> {
    let exclude = vec![".git", ".vs", "node_modules", "target", "dist", "build", ".gemini", ".history", ".idea"];

    let walker = walkdir::WalkDir::new(root).into_iter();
    let mut tree_out = String::new();

    for entry in walker.filter_entry(|e| !exclude.iter().any(|ex| e.file_name().to_string_lossy().contains(ex))) {
        let entry = entry?;
        let depth = entry.depth();
        if depth == 0 { continue; }

        // Strip root from path for cleaner output if needed, or just print relative path
        let path = entry.path().strip_prefix(root).unwrap_or(entry.path());

        let connector = if entry.file_type().is_dir() { "/" } else { "" };
        let indent = "  ".repeat(depth - 1);
        tree_out.push_str(&format!("{}{} {}\n", indent, path.display(), connector));
    }
    Ok(tree_out)
}

fn get_file_content(root: &Path, rel_path: &str, max_kb: u64) -> String {
    let path = root.join(rel_path);
    if path.exists() {
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.len() < max_kb * 1024 {
                if let Ok(content) = fs::read_to_string(&path) {
                    return format!("\n### File: {}\n```\n{}\n```", rel_path, content);
                }
            } else {
                 return format!("\n(File {} skipped - too large)", rel_path);
            }
        }
    }
    String::new()
}
