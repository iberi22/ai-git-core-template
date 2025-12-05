//! Commit atomicity analyzer.
//!
//! Categorizes file changes and determines if a commit is atomic.

use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use tracing::{debug, info};

use crate::config::{Config, Concern};
use crate::git::{self, CommitInfo};

/// Result of analyzing a single commit
#[derive(Debug, Clone)]
pub struct CommitAnalysis {
    /// Commit information
    pub commit: CommitInfo,
    /// Detected concerns in this commit
    pub concerns: HashSet<Concern>,
    /// Whether the commit is atomic
    pub is_atomic: bool,
    /// Whether the commit was skipped (e.g., bot author)
    pub skipped: bool,
    /// Reason for skipping (if skipped)
    pub skip_reason: Option<String>,
}

/// Result of checking atomicity across multiple commits
#[derive(Debug, Clone)]
pub struct AtomicityResult {
    /// Total number of commits analyzed
    pub total_commits: usize,
    /// Number of atomic commits
    pub atomic_commits: usize,
    /// Number of non-atomic commits
    pub non_atomic_commits: usize,
    /// Number of skipped commits (bots, etc.)
    pub skipped_commits: usize,
    /// Whether any issues were found
    pub has_issues: bool,
    /// Individual commit analyses
    pub analyses: Vec<CommitAnalysis>,
}

/// Check atomicity of commits between two refs
pub async fn check_atomicity(
    repo_path: &str,
    base: &str,
    head: &str,
    config: &Config,
) -> Result<AtomicityResult> {
    info!("🔍 Analyzing commits: {}..{}", base, head);

    let commits = git::get_commits_between(repo_path, base, head).await?;

    if commits.is_empty() {
        info!("✅ No commits to analyze");
        return Ok(AtomicityResult {
            total_commits: 0,
            atomic_commits: 0,
            non_atomic_commits: 0,
            skipped_commits: 0,
            has_issues: false,
            analyses: Vec::new(),
        });
    }

    info!("📊 Found {} commits to analyze", commits.len());

    // Analyze commits in parallel using rayon
    let analyses: Vec<CommitAnalysis> = commits
        .into_par_iter()
        .map(|commit| analyze_commit(commit, config))
        .collect();

    // Calculate statistics
    let total_commits = analyses.len();
    let skipped_commits = analyses.iter().filter(|a| a.skipped).count();
    let atomic_commits = analyses.iter().filter(|a| !a.skipped && a.is_atomic).count();
    let non_atomic_commits = analyses.iter().filter(|a| !a.skipped && !a.is_atomic).count();
    let has_issues = non_atomic_commits > 0;

    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 Summary");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("   Total commits: {}", total_commits);
    info!("   Atomic: {}", atomic_commits);
    info!("   Non-atomic: {}", non_atomic_commits);
    info!("   Skipped (bots): {}", skipped_commits);

    Ok(AtomicityResult {
        total_commits,
        atomic_commits,
        non_atomic_commits,
        skipped_commits,
        has_issues,
        analyses,
    })
}

/// Analyze a single commit for atomicity
fn analyze_commit(commit: CommitInfo, config: &Config) -> CommitAnalysis {
    // Check if author is a bot
    if config.is_bot_author(&commit.author) {
        debug!("○ {}: Skipped (bot: {})", commit.short_sha, commit.author);
        return CommitAnalysis {
            commit,
            concerns: HashSet::new(),
            is_atomic: true,
            skipped: true,
            skip_reason: Some("Bot author".to_string()),
        };
    }

    // Categorize files and collect concerns
    let mut concerns = HashSet::new();

    for file in &commit.files {
        // Skip ignored files
        if config.should_ignore_file(file) {
            continue;
        }

        let concern = categorize_file(file, config);
        concerns.insert(concern);
    }

    let is_atomic = concerns.len() <= config.max_concerns;
    let concern_list: Vec<String> = concerns.iter().map(|c| c.to_string()).collect();

    if is_atomic {
        debug!(
            "✅ {}: {} ({})",
            commit.short_sha,
            commit.message,
            concern_list.join(", ")
        );
    } else {
        info!(
            "⚠️ {}: {}",
            commit.short_sha, commit.message
        );
        info!(
            "   └─ Mixes {} concerns: {}",
            concerns.len(),
            concern_list.join(", ")
        );
    }

    CommitAnalysis {
        commit,
        concerns,
        is_atomic,
        skipped: false,
        skip_reason: None,
    }
}

/// Categorize a file path into a concern type
fn categorize_file(path: &str, config: &Config) -> Concern {
    // Check custom rules first
    for rule in &config.custom_rules {
        if let Ok(re) = regex::Regex::new(&rule.pattern) {
            if re.is_match(path) {
                return rule.concern.clone();
            }
        }
    }

    // Built-in categorization rules

    // Tests
    if path.starts_with("tests/") || path.starts_with("test/")
        || path.contains(".test.")
        || path.contains(".spec.")
        || path.contains("_test.")
        || path.starts_with("test_")
    {
        return Concern::Tests;
    }

    // Documentation
    if path.starts_with("docs/") || path.ends_with(".md") {
        return Concern::Docs;
    }

    // CI/Infrastructure
    if path.starts_with(".github/workflows/") || path.starts_with("scripts/") {
        return Concern::Infra;
    }

    // Config files
    if is_config_file(path) {
        return Concern::Config;
    }

    // Source code
    if path.starts_with("src/") || path.starts_with("lib/") || is_source_file(path) {
        return Concern::Source;
    }

    Concern::Other
}

/// Check if a path is a configuration file
fn is_config_file(path: &str) -> bool {
    let config_extensions = [".yml", ".yaml", ".json", ".toml", ".ini", ".cfg"];
    let config_prefixes = [".", "config", "settings"];

    // Check extension
    for ext in config_extensions {
        if path.ends_with(ext) {
            return true;
        }
    }

    // Check if it's a dotfile
    if let Some(filename) = path.split('/').last() {
        if filename.starts_with('.') {
            return true;
        }
        for prefix in config_prefixes {
            if filename.starts_with(prefix) {
                return true;
            }
        }
    }

    false
}

/// Check if a path is a source code file
fn is_source_file(path: &str) -> bool {
    let source_extensions = [
        ".rs", ".py", ".js", ".ts", ".jsx", ".tsx",
        ".go", ".java", ".kt", ".swift", ".c", ".cpp",
        ".h", ".hpp", ".cs", ".rb", ".php", ".scala",
    ];

    for ext in source_extensions {
        if path.ends_with(ext) {
            return true;
        }
    }

    false
}

/// Analyze a single commit (public API for CLI)
pub async fn analyze_single_commit(
    repo_path: &str,
    sha: &str,
    config: &Config,
) -> Result<CommitAnalysis> {
    let commit = git::get_commit(repo_path, sha).await?;
    Ok(analyze_commit(commit, config))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_categorize_source_files() {
        let config = test_config();

        assert_eq!(categorize_file("src/main.rs", &config), Concern::Source);
        assert_eq!(categorize_file("lib/utils.py", &config), Concern::Source);
        assert_eq!(categorize_file("components/Button.tsx", &config), Concern::Source);
    }

    #[test]
    fn test_categorize_test_files() {
        let config = test_config();

        assert_eq!(categorize_file("tests/unit_test.rs", &config), Concern::Tests);
        assert_eq!(categorize_file("src/utils.test.js", &config), Concern::Tests);
        assert_eq!(categorize_file("lib/parser.spec.ts", &config), Concern::Tests);
        assert_eq!(categorize_file("test_main.py", &config), Concern::Tests);
    }

    #[test]
    fn test_categorize_docs() {
        let config = test_config();

        assert_eq!(categorize_file("docs/guide.md", &config), Concern::Docs);
        assert_eq!(categorize_file("README.md", &config), Concern::Docs);
        assert_eq!(categorize_file("CHANGELOG.md", &config), Concern::Docs);
    }

    #[test]
    fn test_categorize_config() {
        let config = test_config();

        assert_eq!(categorize_file("config.yml", &config), Concern::Config);
        assert_eq!(categorize_file(".eslintrc.json", &config), Concern::Config);
        assert_eq!(categorize_file("Cargo.toml", &config), Concern::Config);
        assert_eq!(categorize_file(".gitignore", &config), Concern::Config);
    }

    #[test]
    fn test_categorize_infra() {
        let config = test_config();

        assert_eq!(categorize_file(".github/workflows/ci.yml", &config), Concern::Infra);
        assert_eq!(categorize_file("scripts/deploy.sh", &config), Concern::Infra);
    }
}
