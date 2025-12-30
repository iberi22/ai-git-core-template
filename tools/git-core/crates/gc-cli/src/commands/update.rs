use clap::Args;
use color_eyre::Result;
use gc_core::ports::{SystemPort, FileSystemPort, GitHubPort};
use console::style;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Force update (overwrites ARCHITECTURE.md)
    #[arg(long)]
    pub force: bool,

    /// Non-interactive mode
    #[arg(short, long)]
    pub auto: bool,
}

pub async fn execute(
    args: UpdateArgs,
    fs: &impl FileSystemPort,
    _system: &impl SystemPort, // SystemPort not strictly needed for native download if we use reqwest directly
    github: &impl GitHubPort,
) -> Result<()> {
    println!("{}", style("🔄 Upgrading Git-Core Protocol...").cyan());

    // 1. Version Check
    let version_file = ".git-core-protocol-version";
    let local_version = if fs.exists(version_file).await.unwrap_or(false) {
        fs.read_file(version_file).await.unwrap_or_else(|_| "0.0.0".to_string()).trim().to_string()
    } else {
        "0.0.0".to_string()
    };

    let latest_version = github.get_file_content(
        "iberi22",
        "Git-Core-Protocol",
        "main",
        ".git-core-protocol-version"
    ).await.unwrap_or_else(|_| "unknown".to_string()).trim().to_string();

    if local_version == latest_version && !args.force {
        println!("{} Protocol is already at version {} (latest).", style("✅").green(), local_version);
        println!("   Use --force if you want to reinstall anyway.");
        return Ok(());
    }

    if latest_version != "unknown" {
        println!("{} Update available: {} → {}", style("ℹ").blue(), local_version, latest_version);
    }

    if args.force {
        println!("{}", style("⚠️  Force mode enabled: Files will be overwritten.").red());
    }

    // 2. Download Zip
    let zip_url = "https://github.com/iberi22/Git-Core-Protocol/archive/refs/heads/main.zip";
    println!("{}", style(format!("📥 Downloading protocol from {}...", zip_url)).yellow());

    // Use async reqwest to avoid blocking the runtime
    let response = reqwest::get(zip_url).await?;
    let bytes = response.bytes().await?;
    let reader = Cursor::new(bytes);

    let mut archive = ZipArchive::new(reader)?;

    println!("{}", style("📦 Extracting files...").yellow());

    // 3. Extract specific folders
    // Archive structure: Git-Core-Protocol-main/FOLDER/...
    // We want to map `Git-Core-Protocol-main/FOLDER` -> `./FOLDER`

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string(); // e.g., Git-Core-Protocol-main/.github/workflows/main.yml

        // Basic filtering to extract only relevant parts
        // Remove top level folder
        let parts: Vec<&str> = name.splitn(2, '/').collect();
        if parts.len() < 2 { continue; }

        let relative_path = parts[1]; // .github/workflows/main.yml
        if relative_path.is_empty() { continue; }

        // Whitelist of paths to sync
        if relative_path.starts_with(".github/") ||
           relative_path.starts_with("scripts/") ||
           relative_path.starts_with("docs/") ||
           relative_path == "AGENTS.md" {

            if file.is_dir() {
                // Directories created implicitly by fs port usually or we need explicit create
                // fs trait has create_dir.
                // BUT fs trait is async and we are inside loop.
                // NOTE: Our FileSystemPort trait abstraction might be too simple for bulk operations if we want strict mocking.
                // However, for update logic, using std::fs directly might be pragmatic if we assume standard env,
                // BUT we want to support mocking for tests.
                // We'll trust fs.create_dir handles "mkdir -p".
                // Our TokioFileSystem uses fs::create_dir_all.

                // Let's continue, we create dirs when we see files or explicitly.
                // TokioFileSystem::create_dir wraps fs::create_dir_all, so passing "path/to/dir" works.
                let _ = fs.create_dir(relative_path).await;
            } else {
                // It's a file
                // Ensure parent dir exists
                if let Some(parent) = Path::new(relative_path).parent() {
                     if let Some(p_str) = parent.to_str() {
                         if !p_str.is_empty() {
                             let _ = fs.create_dir(p_str).await;
                         }
                     }
                }

                // Read content
                // Zip file reader implements Read.
                // Our fs.write_file takes &str (string content).
                // This assumes text files. Most protocol files are text (md, yaml, sh, ps1).
                // If we have binaries this will fail/corrupt.
                // Git-Core Protocol currently contains text.

                // Safety check: is it a text file?
                // For now we assume yes.
                let mut content = String::new();
                if std::io::Read::read_to_string(&mut file, &mut content).is_ok() {
                     fs.write_file(relative_path, &content).await?;
                     // println!("  extracted: {}", relative_path);
                } else {
                    println!("{}", style(format!("  ⚠️ Skipped binary or non-utf8 file: {}", relative_path)).dim());
                }
            }
        }
    }

    // 4. Update version file
    fs.write_file(version_file, &latest_version).await?;
    println!("{}", style(format!("✓ Updated .git-core-protocol-version to {}", latest_version)).green());

    println!("\n{}", style("✅ Protocol upgraded successfully (Native).").green());
    Ok(())
}
