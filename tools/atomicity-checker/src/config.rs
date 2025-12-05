//! Configuration loading and parsing for atomicity-checker.
//!
//! Reads YAML config from `.github/atomicity-config.yml`

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Execution mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Only warn, don't fail the check
    #[default]
    Warning,
    /// Fail the check if non-atomic commits found
    Error,
}

/// File concern category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Concern {
    Source,
    Tests,
    Docs,
    Config,
    Infra,
    Other,
}

impl std::fmt::Display for Concern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Concern::Source => write!(f, "source"),
            Concern::Tests => write!(f, "tests"),
            Concern::Docs => write!(f, "docs"),
            Concern::Config => write!(f, "config"),
            Concern::Infra => write!(f, "infra"),
            Concern::Other => write!(f, "other"),
        }
    }
}

/// Atomicity checker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether the check is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Execution mode (warning or error)
    #[serde(default)]
    pub mode: Mode,

    /// Whether to ignore commits from bots
    #[serde(default = "default_ignore_bots")]
    pub ignore_bots: bool,

    /// Maximum number of concerns per commit
    #[serde(default = "default_max_concerns")]
    pub max_concerns: usize,

    /// Bot author patterns (regex)
    #[serde(default = "default_bot_patterns")]
    pub bot_patterns: Vec<String>,

    /// Files to always ignore (glob patterns)
    #[serde(default = "default_ignore_files")]
    pub ignore_files: Vec<String>,

    /// Custom concern rules
    #[serde(default)]
    pub custom_rules: Vec<ConcernRule>,
}

/// Custom rule for file categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcernRule {
    /// Regex pattern for file paths
    pub pattern: String,
    /// Concern category to assign
    pub concern: Concern,
}

fn default_enabled() -> bool {
    true
}

fn default_ignore_bots() -> bool {
    true
}

fn default_max_concerns() -> usize {
    1
}

fn default_bot_patterns() -> Vec<String> {
    vec![
        r"github-actions".to_string(),
        r"dependabot".to_string(),
        r"copilot".to_string(),
        r"jules".to_string(),
        r"renovate".to_string(),
        r"bot$".to_string(),
        r"\[bot\]".to_string(),
    ]
}

fn default_ignore_files() -> Vec<String> {
    vec![
        "*.lock".to_string(),
        "package-lock.json".to_string(),
        "yarn.lock".to_string(),
        "Cargo.lock".to_string(),
        ".gitignore".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            mode: Mode::default(),
            ignore_bots: default_ignore_bots(),
            max_concerns: default_max_concerns(),
            bot_patterns: default_bot_patterns(),
            ignore_files: default_ignore_files(),
            custom_rules: Vec::new(),
        }
    }
}

impl Config {
    /// Load configuration from a YAML file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            tracing::info!("⚠️ No configuration file found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;

        tracing::info!("📄 Loaded configuration from {:?}", path);
        tracing::debug!("   enabled: {}", config.enabled);
        tracing::debug!("   mode: {:?}", config.mode);
        tracing::debug!("   ignore_bots: {}", config.ignore_bots);
        tracing::debug!("   max_concerns: {}", config.max_concerns);

        Ok(config)
    }

    /// Check if a file should be ignored based on patterns
    pub fn should_ignore_file(&self, path: &str) -> bool {
        for pattern in &self.ignore_files {
            if glob_match(pattern, path) {
                return true;
            }
        }
        false
    }

    /// Check if an author is a bot
    pub fn is_bot_author(&self, author: &str) -> bool {
        if !self.ignore_bots {
            return false;
        }

        for pattern in &self.bot_patterns {
            if let Ok(re) = regex::RegexBuilder::new(pattern)
                .case_insensitive(true)
                .build()
            {
                if re.is_match(author) {
                    return true;
                }
            }
        }
        false
    }
}

/// Simple glob matching (supports * and **)
fn glob_match(pattern: &str, path: &str) -> bool {
    // Simple implementation for common patterns
    if pattern.starts_with("*.") {
        // Extension match: *.lock
        let ext = &pattern[1..];
        return path.ends_with(ext);
    }

    if pattern.starts_with("**/") {
        // Any directory: **/foo
        let suffix = &pattern[3..];
        return path.ends_with(suffix) || path.contains(&format!("/{}", suffix));
    }

    // Exact match
    path == pattern || path.ends_with(&format!("/{}", pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.enabled);
        assert_eq!(config.mode, Mode::Warning);
        assert!(config.ignore_bots);
        assert_eq!(config.max_concerns, 1);
    }

    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.lock", "Cargo.lock"));
        assert!(glob_match("*.lock", "src/Cargo.lock"));
        assert!(!glob_match("*.lock", "Cargo.toml"));

        assert!(glob_match("package-lock.json", "package-lock.json"));
        assert!(glob_match("package-lock.json", "frontend/package-lock.json"));
    }

    #[test]
    fn test_bot_detection() {
        let config = Config::default();

        assert!(config.is_bot_author("dependabot[bot]"));
        assert!(config.is_bot_author("github-actions"));
        assert!(config.is_bot_author("renovate"));
        assert!(config.is_bot_author("my-custom-bot"));

        assert!(!config.is_bot_author("john-doe"));
        assert!(!config.is_bot_author("jane_developer"));
    }
}
