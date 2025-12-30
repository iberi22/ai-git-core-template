use mockall::mock;
use gc_core::ports::{FileSystemPort, SystemPort, GitHubPort, GitPort, Result, CoreError};
use async_trait::async_trait;

mock! {
    pub FileSystemPort {}
    #[async_trait]
    impl FileSystemPort for FileSystemPort {
        async fn create_dir(&self, path: &str) -> Result<()>;
        async fn write_file(&self, path: &str, content: &str) -> Result<()>;
        async fn read_file(&self, path: &str) -> Result<String>;
        async fn exists(&self, path: &str) -> Result<bool>;
        async fn move_file(&self, source: &str, dest: &str) -> Result<()>;
        async fn list_files(&self, dir: &str, pattern: Option<&str>) -> Result<Vec<String>>;
    }
}

mock! {
    pub SystemPort {}
    #[async_trait]
    impl SystemPort for SystemPort {
        async fn check_command(&self, name: &str) -> Result<bool>;
        async fn run_command(&self, name: &str, args: &[String]) -> Result<()>;
        async fn run_command_output(&self, name: &str, args: &[String]) -> Result<String>;
    }
}

mock! {
    pub GitHubPort {}
    #[async_trait]
    impl GitHubPort for GitHubPort {
        async fn check_auth(&self) -> Result<String>;
        async fn create_repo(&self, name: &str, private: bool) -> Result<()>;
        async fn create_issue(&self, owner: &str, repo: &str, title: &str, body: &str, labels: &[String]) -> Result<()>;
        async fn create_label(&self, name: &str, color: &str, desc: &str) -> Result<()>;
        async fn get_file_content(&self, owner: &str, repo: &str, branch: &str, path: &str) -> Result<String>;
        async fn get_pr_diff(&self, owner: &str, repo: &str, pr_number: u64) -> Result<String>;
        async fn post_comment(&self, owner: &str, repo: &str, issue_number: u64, body: &str) -> Result<()>;
        async fn list_issues(&self, owner: &str, repo: &str, state: Option<String>, assignee: Option<String>) -> Result<Vec<gc_core::Issue>>;
        async fn list_prs(&self, owner: &str, repo: &str, state: Option<String>) -> Result<Vec<gc_core::PullRequest>>;
    }
}

mock! {
    pub GitPort {}
    #[async_trait]
    impl GitPort for GitPort {
        async fn init(&self) -> Result<()>;
        async fn status(&self) -> Result<bool>;
        async fn remote_url(&self) -> Result<Option<String>>;
        async fn commit(&self, msg: &str) -> Result<()>;
        async fn push(&self) -> Result<()>;
    }
}
