use clap::Subcommand;
use gc_core::ports::{FileSystemPort, GitHubPort};
use console::style;

#[derive(Subcommand, Debug)]
pub enum ContextCmd {
    /// Equip a specific agent role
    Equip {
        role: String,
    },
    /// List available agents
    List,
}

pub async fn execute(
    cmd: ContextCmd,
    fs: &impl FileSystemPort,
    github: &impl GitHubPort,
) -> color_eyre::Result<()> {
    let agent_dir = ".github/agents";
    let index_path = ".gitcore/AGENT_INDEX.md";

    match cmd {
        ContextCmd::List => {
            println!("{} Available Agent Personas:", style("🤖").cyan());

            // 1. List Local Agents (Agent v2)
            if fs.exists(agent_dir).await? {
                println!("  {} Local Agents (.github/agents/):", style("📂").blue());
                let local_agents = fs.list_files(agent_dir, Some("*.agent.md".to_string())).await?;
                for agent in local_agents {
                    let name = agent.trim_end_matches(".agent.md");
                    println!("    - {}", style(name).yellow());
                }
            }

            // 2. List Indexed Agents (Legacy/Remote)
            if fs.exists(index_path).await? {
                println!("\n  {} Indexed Roles (.gitcore/AGENT_INDEX.md):", style("📋").blue());
                let content = fs.read_file(index_path).await?;
                for line in content.lines() {
                    if line.starts_with("| **") {
                        if let Some(name) = line.split('|').nth(1) {
                             println!("    - {}", style(name.trim().trim_matches('*')).cyan());
                        }
                    }
                }
            }
        }
        ContextCmd::Equip { role } => {
            println!("{}", style(format!("🔍 Searching for role '{}'...", role)).cyan());

            let mut final_persona_content = String::new();
            let mut found_locally = false;

            // 1. Try Local Agent Definition (Priority)
            let local_path = format!("{}/{}.agent.md", agent_dir, role.to_lowercase());
            if fs.exists(&local_path).await? {
                println!("{}", style(format!("✅ Found local agent: {}", local_path)).green());
                final_persona_content = fs.read_file(&local_path).await?;
                found_locally = true;
            }

            // 2. Fallback to Index + Remote
            if !found_locally {
                if fs.exists(index_path).await? {
                    let content = fs.read_file(index_path).await?;
                    let mut recipe_path = None;

                    for line in content.lines() {
                        if line.to_lowercase().contains(&role.to_lowercase()) {
                            if let Some(start) = line.find('`') {
                                if let Some(end) = line[start+1..].find('`') {
                                     recipe_path = Some(line[start+1..start+1+end].to_string());
                                     break;
                                }
                            }
                        }
                    }

                    if let Some(path) = recipe_path {
                        println!("{}", style(format!("🌐 Found Remote Recipe: {}", path)).green());
                        println!("{}", style(format!("⬇️ Downloading from iberi22/agents-flows-recipes...")).cyan());

                        final_persona_content = github.get_file_content(
                            "iberi22",
                            "agents-flows-recipes",
                            "main",
                            &path
                        ).await?;
                    }
                }
            }

            if final_persona_content.is_empty() {
                color_eyre::eyre::bail!("Role '{}' not found locally or in index.", role);
            }

            let context_path = ".gitcore/CURRENT_CONTEXT.md";
            let header = format!(r#"# 🎭 ACTIVE AGENT PERSONA: {}
> GENERATED CONTEXT - DO NOT EDIT MANUALLY
> Loaded via Git-Core CLI ({})

---
"#, role, if found_locally { "Local Agent v2" } else { "Remote Recipe v1" });

            let protocol_skills = r#"
---
## 🛡️ MANDATORY PROTOCOL SKILLS
1. **Token Economy:** Use GitHub Issues for state. No TODO.md.
2. **Architecture First:** Verify against .gitcore/ARCHITECTURE.md.
3. **Atomic Commits:** One logical change per commit.
"#;

            let final_context = format!("{}{}{}", header, final_persona_content, protocol_skills);

            fs.write_file(context_path, &final_context).await?;

            println!("{}", style(format!("✨ Agent Equipped! Context written to {}", context_path)).yellow());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mocks::{MockFileSystemPort, MockGitHubPort};
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_context_equip_local_success() {
        let cmd = ContextCmd::Equip { role: "Architect".to_string() };
        let mut mock_fs = MockFileSystemPort::new();
        let mock_github = MockGitHubPort::new();

        // 1. Check Local Agent Exists
        mock_fs.expect_exists()
            .with(eq(".github/agents/architect.agent.md"))
            .returning(|_| Ok(true));

        // 2. Read Local Agent
        mock_fs.expect_read_file()
            .with(eq(".github/agents/architect.agent.md"))
            .returning(|_| Ok("# Architect Persona\nYou are local.".to_string()));

        // 3. Write Context
        mock_fs.expect_write_file()
            .with(eq(".gitcore/CURRENT_CONTEXT.md"), always())
            .returning(|_, _| Ok(()));

        let res = execute(cmd, &mock_fs, &mock_github).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_context_equip_remote_fallback_success() {
        let cmd = ContextCmd::Equip { role: "Architect".to_string() };
        let mut mock_fs = MockFileSystemPort::new();
        let mut mock_github = MockGitHubPort::new();

        // 1. Check Local Agent (NOT Found)
        mock_fs.expect_exists()
            .with(eq(".github/agents/architect.agent.md"))
            .returning(|_| Ok(false));

        // 2. Check Index Exists
        mock_fs.expect_exists()
            .with(eq(".gitcore/AGENT_INDEX.md"))
            .returning(|_| Ok(true));

        // 3. Read Index
        let index_content = r#"
# Agent Index
- **Architect**: `roles/architect.md`
"#;
        mock_fs.expect_read_file()
            .with(eq(".gitcore/AGENT_INDEX.md"))
            .returning(move |_| Ok(index_content.to_string()));

        // 4. GitHub Fetch Recipe
        mock_github.expect_get_file_content()
            .with(eq("iberi22"), eq("agents-flows-recipes"), eq("main"), eq("roles/architect.md"))
            .returning(|_, _, _, _| Ok("# Architect Persona\nYou are remote.".to_string()));

        // 5. Write Context
        mock_fs.expect_write_file()
            .with(eq(".gitcore/CURRENT_CONTEXT.md"), always())
            .returning(|_, _| Ok(()));

        let res = execute(cmd, &mock_fs, &mock_github).await;
        assert!(res.is_ok());
    }
}
