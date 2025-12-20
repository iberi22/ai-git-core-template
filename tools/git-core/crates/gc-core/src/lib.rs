pub mod ports;
use serde::{Serialize, Deserialize};

// Basic Core setup
pub fn greeting() -> String {
    "Hello from Git-Core!".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub assignees: Vec<String>,
    pub labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
    pub head_ref: String,
    pub base_ref: String,
}
