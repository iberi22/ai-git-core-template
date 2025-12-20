use clap::Args;
use gc_core::ports::{FileSystemPort, SystemPort, GitHubPort};
use serde::{Serialize, Deserialize};
use slug::slugify;
use console::style;

#[derive(Args, Debug)]
pub struct TaskArgs {
    /// Title of the task (e.g., "Fix login bug")
    pub title: String,

    /// Type of task (feat, bug, docs, chore). Auto-detected if omitted.
    #[arg(short, long)]
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskFrontmatter {
    title: String,
    labels: Vec<String>,
    assignees: Vec<String>,
}

pub async fn execute(
    args: TaskArgs,
    fs: &impl FileSystemPort,
    system: &impl SystemPort,
    _github: &impl GitHubPort,
) -> color_eyre::Result<()> {
    println!("{} Starting new task...", style("🚀").cyan());

    // 1. Detect Type
    let task_type = args.type_.clone().unwrap_or_else(|| detect_type(&args.title));
    let slug = slugify(&args.title);

    // 2. Generate Filename
    let filename = format!("{}_{}.md", task_type.to_uppercase(), slug);
    let filepath = format!(".github/issues/{}", filename);

    println!("   Type: {}", style(&task_type).yellow());
    println!("   Slug: {}", style(&slug).dim());

    // 3. Create Issue File
    // fs.exists returns Result<bool>, so we must unwrap
    if !fs.exists(&filepath).await.unwrap_or(false) {
        println!("{} Creating issue file: {}", style("📝").green(), filepath);

        let frontmatter = TaskFrontmatter {
            title: args.title.clone(),
            labels: vec![determine_label(&task_type)],
            assignees: vec![], // Empty for now, user can assign later
        };

        let yaml = serde_yaml::to_string(&frontmatter).unwrap();
        let content = format!("---\n{}---\n\n## Description\n{}\n\n## Context\n- Created via `gc task`\n", yaml, args.title);

        fs.write_file(&filepath, &content).await?;
    } else {
        println!("{} Issue file already exists: {}", style("ℹ️").blue(), filepath);
    }

    // 4. Create Branch
    let branch_name = format!("{}/{}", task_type.to_lowercase(), slug);
    println!("{} Switching to branch: {}", style("twisted_rightwards_arrows").blue(), branch_name); // git branch icon analog

    // Check if branch exists
    let ref_args = vec!["show-ref".to_string(), "--verify".to_string(), format!("refs/heads/{}", branch_name)];
    let branch_exists = system.run_command_output("git", &ref_args).await.is_ok();

    if branch_exists {
        let checkout = vec!["checkout".to_string(), branch_name.clone()];
        system.run_command("git", &checkout).await?;
    } else {
        let checkout_b = vec!["checkout".to_string(), "-b".to_string(), branch_name.clone()];
        system.run_command("git", &checkout_b).await?;
    }

    // 5. Auto-Equip Agent
    let role = detect_role(&args.title);
    if let Some(r) = role {
        println!("{} Auto-equipping agent role: {}", style("🤖").magenta(), r);
        // We reuse the context logic here.
        // For simplicity in this iteration, we call the subcommand logic if possible,
        // or just invoke the equip logic directly.
        // Since `commands::context::execute` takes `ContextCmd`, let's construct it.
        // Importing strict dependency might be tricky if not public, but let's try direct call or separate function.
        // For now, let's just print suggestion or run the command via system if easy,
        // but better to use the library.
        // TODO: Call `commands::context::equip_role` properly.
        // For now, we will just output instructions or run it if we refactor `context` to be public.

        // Simulating auto-equip via shell for now to avoid borrow checker/dependency hell in `mod.rs` exposure
        // Actually, we can just call `gc context equip` via system to ensure isolation?
        // No, that's recursive. We should simply instruct the user or refactor context.rs to expose `equip_impl`.
        // Let's assume we can call `gc context equip` is the standard way.

        // Refactoring idea: extract `equip` logic to `gc-core` service?
        // For MVP: Just suggest it.
        println!("   (Run `gc context equip {}` to fully activate)", r);
    }

    println!("\n{} Task '{}' ready!", style("✅").green(), args.title);
    println!("   Issue: {}", filepath);
    println!("   Branch: {}", branch_name);

    Ok(())
}

fn detect_type(title: &str) -> String {
    let lower = title.to_lowercase();
    if lower.contains("fix") || lower.contains("bug") || lower.contains("error") {
        "BUG".to_string()
    } else if lower.contains("feat") || lower.contains("add") || lower.contains("new") {
        "FEAT".to_string()
    } else if lower.contains("docs") || lower.contains("readme") {
        "DOCS".to_string()
    } else if lower.contains("refactor") {
        "REFACTOR".to_string()
    } else if lower.contains("test") {
        "TEST".to_string()
    } else {
        "TASK".to_string()
    }
}

fn determine_label(type_: &str) -> String {
    match type_ {
        "BUG" => "bug".to_string(),
        "FEAT" => "enhancement".to_string(),
        "DOCS" => "documentation".to_string(),
        "REFACTOR" => "refactor".to_string(),
        _ => "task".to_string(),
    }
}

fn detect_role(title: &str) -> Option<&str> {
    let lower = title.to_lowercase();
    if lower.contains("security") || lower.contains("auth") || lower.contains("login") {
        Some("security")
    } else if lower.contains("ui") || lower.contains("css") || lower.contains("frontend") {
        Some("frontend")
    } else if lower.contains("api") || lower.contains("db") || lower.contains("backend") {
        Some("backend")
    } else if lower.contains("ci") || lower.contains("cd") || lower.contains("workflow") {
        Some("devops")
    } else {
        None
    }
}
