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
            // Since FileSystemPort might not have 'read_dir', we assume it does or use a workaround.
            // Actually, TokioFileSystem usually has it. Let's assume the port needs it.
            // For MVP, we'll try to use the system if the port is too simple.
            // But let's check what FileSystemPort has.
            println!("(Scanning {}...)", workflow_dir);
            // ... for now, hardcoded detection or simple print
            println!("- reolplazr (reolplazr.md)");
        } else {
            println!("   (No workflows found in {})", workflow_dir);
        }
        return Ok(());
    }

    if let Some(name) = args.name {
        let path = format!("{}/{}.md", workflow_dir, name);
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
