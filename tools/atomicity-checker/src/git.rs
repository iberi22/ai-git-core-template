//! Git operations module.
//!
//! Executes git commands using tokio::process for async operation.
//! This approach is chosen over git2-rs for:
//! 1. Smaller binary size
//! 2. Guaranteed compatibility with system git
//! 3. Simpler authentication handling

use anyhow::{Context, Result, bail};
use std::path::Path;
use tokio::process::Command;

/// Commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    /// Full commit SHA
    pub sha: String,
    /// Short SHA (first 8 chars)
    pub short_sha: String,
    /// Author name
    pub author: String,
    /// Commit message (first line)
    pub message: String,
    /// Files changed in this commit
    pub files: Vec<String>,
}

/// Get commits between two refs
pub async fn get_commits_between<P: AsRef<Path>>(
    repo_path: P,
    base: &str,
    head: &str,
) -> Result<Vec<CommitInfo>> {
    let repo_path = repo_path.as_ref();

    // Format: SHA AUTHOR_NAME
    // We use \x00 as delimiter to handle spaces in author names
    let output = Command::new("git")
        .current_dir(repo_path)
        .args([
            "log",
            "--format=%H%x00%an%x00%s",
            &format!("{}..{}", base, head),
        ])
        .output()
        .await
        .context("Failed to execute git log")?;

    if !output.status.success() {
        // Try with origin/ prefix
        let output = Command::new("git")
            .current_dir(repo_path)
            .args([
                "log",
                "--format=%H%x00%an%x00%s",
                &format!("origin/{}..{}", base, head),
            ])
            .output()
            .await
            .context("Failed to execute git log with origin/ prefix")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("git log failed: {}", stderr);
        }

        return parse_log_output(&output.stdout, repo_path).await;
    }

    parse_log_output(&output.stdout, repo_path).await
}

/// Parse git log output and fetch files for each commit
async fn parse_log_output(output: &[u8], repo_path: &Path) -> Result<Vec<CommitInfo>> {
    let stdout = String::from_utf8_lossy(output);
    let mut commits = Vec::new();

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\x00').collect();
        if parts.len() < 3 {
            continue;
        }

        let sha = parts[0].to_string();
        let author = parts[1].to_string();
        let message = parts[2].to_string();
        let short_sha = sha.chars().take(8).collect();

        // Get files changed in this commit
        let files = get_commit_files(repo_path, &sha).await?;

        commits.push(CommitInfo {
            sha,
            short_sha,
            author,
            message,
            files,
        });
    }

    Ok(commits)
}

/// Get files changed in a specific commit
pub async fn get_commit_files<P: AsRef<Path>>(repo_path: P, sha: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(repo_path.as_ref())
        .args(["show", "--name-only", "--format=", sha])
        .output()
        .await
        .context("Failed to get commit files")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git show failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(files)
}

/// Get a single commit info
pub async fn get_commit<P: AsRef<Path>>(repo_path: P, sha: &str) -> Result<CommitInfo> {
    let repo_path = repo_path.as_ref();

    let output = Command::new("git")
        .current_dir(repo_path)
        .args(["log", "-1", "--format=%H%x00%an%x00%s", sha])
        .output()
        .await
        .context("Failed to get commit info")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git log failed for {}: {}", sha, stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next().context("No commit found")?;

    let parts: Vec<&str> = line.split('\x00').collect();
    if parts.len() < 3 {
        bail!("Invalid git log format");
    }

    let full_sha = parts[0].to_string();
    let files = get_commit_files(repo_path, &full_sha).await?;

    Ok(CommitInfo {
        sha: full_sha.clone(),
        short_sha: full_sha.chars().take(8).collect(),
        author: parts[1].to_string(),
        message: parts[2].to_string(),
        files,
    })
}

/// Get the default branch name
pub async fn get_default_branch<P: AsRef<Path>>(repo_path: P) -> Result<String> {
    let output = Command::new("git")
        .current_dir(repo_path.as_ref())
        .args(["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])
        .output()
        .await?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("origin/", "");
        return Ok(branch);
    }

    // Fallback: try main, then master
    for branch in ["main", "master"] {
        let output = Command::new("git")
            .current_dir(repo_path.as_ref())
            .args(["rev-parse", "--verify", &format!("origin/{}", branch)])
            .output()
            .await?;

        if output.status.success() {
            return Ok(branch.to_string());
        }
    }

    Ok("main".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_default_branch() {
        // This test will work in any git repo
        let result = get_default_branch(".").await;
        // Should not error, but branch might be main or master
        assert!(result.is_ok());
    }
}
