//! GitHub API Wrapper
//!
//! Simplified interface for GitHub Issues operations using octocrab.

use anyhow::{Context, Result};
use octocrab::{models::issues::Issue, Octocrab};
use crate::parser::IssueData;

/// GitHub client wrapper
#[derive(Clone)]
pub struct GitHubClient {
    client: Octocrab,
    owner: String,
    repo: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(client: Octocrab, owner: String, repo: String) -> Self {
        Self {
            client,
            owner,
            repo,
        }
    }

    /// Create a new issue
    pub async fn create_issue(&self, data: &IssueData) -> Result<u64> {
        let issues = self.client.issues(&self.owner, &self.repo);
        
        let mut builder = issues
            .create(&data.title)
            .body(&data.body);

        // Add labels if present
        if !data.labels.is_empty() {
            builder = builder.labels(data.labels.clone());
        }

        // Add assignees if present
        if !data.assignees.is_empty() {
            builder = builder.assignees(data.assignees.clone());
        }

        let issue = builder
            .send()
            .await
            .context("Failed to create GitHub issue")?;

        Ok(issue.number)
    }

    /// Update an existing issue
    pub async fn update_issue(&self, number: u64, data: &IssueData) -> Result<()> {
        let issues = self.client.issues(&self.owner, &self.repo);
        
        let mut builder = issues
            .update(number)
            .title(&data.title)
            .body(&data.body);

        // Update labels
        if !data.labels.is_empty() {
            builder = builder.labels(&data.labels);
        }

        builder
            .send()
            .await
            .context("Failed to update GitHub issue")?;

        Ok(())
    }

    /// Fetch all closed issues
    pub async fn fetch_closed_issues(&self) -> Result<Vec<Issue>> {
        let issues = self
            .client
            .issues(&self.owner, &self.repo)
            .list()
            .state(octocrab::params::State::Closed)
            .per_page(100)
            .send()
            .await
            .context("Failed to fetch closed issues")?;

        Ok(issues.items)
    }

    /// Fetch all open issues
    pub async fn fetch_open_issues(&self) -> Result<Vec<Issue>> {
        let issues = self
            .client
            .issues(&self.owner, &self.repo)
            .list()
            .state(octocrab::params::State::Open)
            .per_page(100)
            .send()
            .await
            .context("Failed to fetch open issues")?;

        Ok(issues.items)
    }

    /// Check if an issue exists and is open
    pub async fn is_issue_open(&self, number: u64) -> Result<bool> {
        let issue = self
            .client
            .issues(&self.owner, &self.repo)
            .get(number)
            .await
            .context("Failed to fetch issue")?;

        Ok(issue.state == octocrab::models::IssueState::Open)
    }

    /// Get issue by number
    pub async fn get_issue(&self, number: u64) -> Result<Issue> {
        self.client
            .issues(&self.owner, &self.repo)
            .get(number)
            .await
            .context("Failed to fetch issue")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These are unit tests - integration tests with mock server
    // are in tests/integration_github.rs

    #[tokio::test]
    async fn test_github_client_creation() {
        let client = Octocrab::builder().build().unwrap();
        let github = GitHubClient::new(
            client,
            "owner".to_string(),
            "repo".to_string(),
        );

        assert_eq!(github.owner, "owner");
        assert_eq!(github.repo, "repo");
    }
}
