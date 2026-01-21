use clap::Args;
use color_eyre::Result;
use gc_core::ports::FileSystemPort;
use console::style;

#[derive(Args, Debug)]
pub struct WorkflowArgs {
    /// Workflow to run/view
    pub name: Option<String>,

    /// List available local workflows
    #[arg(long)]
    pub list: bool,
}

pub async fn execute(
    args: WorkflowArgs,
    fs: &impl FileSystemPort,
) -> Result<()> {
    let workflow_dir = ".agent/workflows";

    if args.list || args.name.is_none() {
        println!("{} Local Agent Workflows:", style("📋").cyan());

        if fs.exists(workflow_dir).await? {
            let files = fs.list_files(workflow_dir, Some("*.md".to_string())).await?;
            if files.is_empty() {
                println!("   (No .md workflows found in {})", workflow_dir);
            } else {
                for file in files {
                    let name = file.trim_end_matches(".md");
                    println!("  - {} ({})", style(name).yellow(), file);
                }
            }
        } else {
            println!("   (Directory not found: {})", workflow_dir);
        }
        return Ok(());
    }

    if let Some(name) = args.name {
        let path = if name.ends_with(".md") {
            format!("{}/{}", workflow_dir, name)
        } else {
            format!("{}/{}.md", workflow_dir, name)
        };

        if fs.exists(&path).await? {
            let content = fs.read_file(&path).await?;
            println!("{} Workflow: {}", style("📖").yellow(), name);
            println!("---");
            println!("{}", content);
        } else {
            color_eyre::eyre::bail!("Workflow '{}' not found at {}", name, path);
        }
    }

    Ok(())
}
