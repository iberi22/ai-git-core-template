use async_trait::async_trait;
use gc_core::ports::{JulesPort, CopilotPort, Result, CoreError};
use tokio::process::Command;
use tracing::{debug, info};

pub struct CliJulesAdapter;

#[async_trait]
impl JulesPort for CliJulesAdapter {
    async fn execute_task(&self, task_desc: &str) -> Result<()> {
        info!("Dispatching task to Jules: {}", task_desc);

        let output = Command::new("jules")
            .arg(task_desc)
            .output()
            .await
            .map_err(|e| CoreError::System(format!("Failed to execute jules: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CoreError::System(format!("Jules failed: {}", stderr)));
        }

        Ok(())
    }
}

pub struct CliCopilotAdapter;

#[async_trait]
impl CopilotPort for CliCopilotAdapter {
    async fn suggest(&self, prompt: &str) -> Result<String> {
        debug!("Requesting Copilot suggestion for: {}", prompt);

        let output = Command::new("gh")
            .args(["copilot", "suggest", prompt])
            .output()
            .await
            .map_err(|e| CoreError::System(format!("Failed to execute gh copilot: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CoreError::System(format!("gh copilot failed: {}", stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

pub struct CliGitAdapter;

#[async_trait]
impl gc_core::ports::GitPort for CliGitAdapter {
    async fn init(&self) -> Result<()> {
        let status = Command::new("git")
            .arg("init")
            .status()
            .await
            .map_err(|e| CoreError::Git(format!("Failed to execute git init: {}", e)))?;
        if !status.success() {
            return Err(CoreError::Git("git init failed".into()));
        }
        Ok(())
    }

    async fn status(&self) -> Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .await
            .map_err(|e| CoreError::Git(format!("Failed to execute git status: {}", e)))?;
        Ok(output.stdout.is_empty())
    }

    async fn remote_url(&self) -> Result<Option<String>> {
        let output = Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()
            .await
            .map_err(|e| CoreError::Git(format!("Failed to execute git remote: {}", e)))?;
        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).trim().to_string()))
        } else {
            Ok(None)
        }
    }

    async fn commit(&self, msg: &str) -> Result<()> {
        let status = Command::new("git")
            .args(["commit", "-m", msg])
            .status()
            .await
            .map_err(|e| CoreError::Git(format!("Failed to execute git commit: {}", e)))?;
        if !status.success() {
            return Err(CoreError::Git("git commit failed".into()));
        }
        Ok(())
    }

    async fn push(&self) -> Result<()> {
        let status = Command::new("git")
            .arg("push")
            .status()
            .await
            .map_err(|e| CoreError::Git(format!("Failed to execute git push: {}", e)))?;
        if !status.success() {
            return Err(CoreError::Git("git push failed".into()));
        }
        Ok(())
    }
}
