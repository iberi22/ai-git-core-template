//! Report generation for atomicity check results.
//!
//! Supports multiple output formats:
//! - terminal: Colored terminal output
//! - markdown: GitHub-flavored Markdown
//! - json: Machine-readable JSON

use anyhow::Result;
use colored::*;
use serde::Serialize;
use std::io::Write;

use crate::analyzer::{AtomicityResult, CommitAnalysis};
use crate::config::{Config, Mode};

/// JSON-serializable result structure
#[derive(Debug, Serialize)]
struct JsonResult {
    total_commits: usize,
    atomic_commits: usize,
    non_atomic_commits: usize,
    skipped_commits: usize,
    has_issues: bool,
    mode: String,
    commits: Vec<JsonCommit>,
}

#[derive(Debug, Serialize)]
struct JsonCommit {
    sha: String,
    message: String,
    author: String,
    concerns: Vec<String>,
    is_atomic: bool,
    skipped: bool,
}

/// Print result to terminal/stdout
pub fn print_result(result: &AtomicityResult, format: &str, config: &Config) -> Result<()> {
    match format {
        "json" => print_json(result)?,
        "markdown" => print_markdown(result, config)?,
        _ => print_terminal(result, config),
    }
    Ok(())
}

/// Print analysis of a single commit
pub fn print_commit_analysis(analysis: &CommitAnalysis, format: &str) -> Result<()> {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&JsonCommit {
                sha: analysis.commit.sha.clone(),
                message: analysis.commit.message.clone(),
                author: analysis.commit.author.clone(),
                concerns: analysis.concerns.iter().map(|c| c.to_string()).collect(),
                is_atomic: analysis.is_atomic,
                skipped: analysis.skipped,
            })?;
            println!("{}", json);
        }
        _ => {
            if analysis.skipped {
                println!(
                    "○ {} {} (skipped: {})",
                    analysis.commit.short_sha,
                    analysis.commit.message.dimmed(),
                    analysis.skip_reason.as_deref().unwrap_or("unknown")
                );
            } else if analysis.is_atomic {
                let concerns: Vec<String> = analysis.concerns.iter().map(|c| c.to_string()).collect();
                println!(
                    "{} {} {} ({})",
                    "✅".green(),
                    analysis.commit.short_sha.cyan(),
                    analysis.commit.message,
                    concerns.join(", ").dimmed()
                );
            } else {
                let concerns: Vec<String> = analysis.concerns.iter().map(|c| c.to_string()).collect();
                println!(
                    "{} {} {}",
                    "⚠️".yellow(),
                    analysis.commit.short_sha.cyan(),
                    analysis.commit.message
                );
                println!(
                    "   └─ Mixes {} concerns: {}",
                    analysis.concerns.len().to_string().red(),
                    concerns.join(", ").yellow()
                );
            }
        }
    }
    Ok(())
}

/// Generate a report to a file or stdout
pub fn generate_report(
    result: &AtomicityResult,
    format: &str,
    output_file: Option<&str>,
    config: &Config,
) -> Result<()> {
    let content = match format {
        "json" => generate_json(result)?,
        "markdown" | "md" => generate_markdown(result, config),
        _ => generate_terminal_string(result, config),
    };

    match output_file {
        Some(path) => {
            let mut file = std::fs::File::create(path)?;
            file.write_all(content.as_bytes())?;
            println!("📄 Report written to: {}", path);
        }
        None => {
            print!("{}", content);
        }
    }

    Ok(())
}

fn print_terminal(result: &AtomicityResult, config: &Config) {
    println!();

    if result.has_issues {
        println!("{}", "⚠️  Some commits mix multiple concerns".yellow().bold());
    } else {
        println!("{}", "✅ All commits are atomic".green().bold());
    }

    println!();
    println!("{}", "━".repeat(50));
    println!("{}", "📊 Summary".bold());
    println!("{}", "━".repeat(50));
    println!("   Total commits:    {}", result.total_commits);
    println!("   {} Atomic:         {}", "✅".green(), result.atomic_commits);
    println!("   {} Non-atomic:     {}", "⚠️".yellow(), result.non_atomic_commits);
    println!("   ⏭️  Skipped (bots): {}", result.skipped_commits);
    println!();
    println!(
        "   Mode: {} | Max concerns: {}",
        match config.mode {
            Mode::Error => "error".red(),
            Mode::Warning => "warning".yellow(),
        },
        config.max_concerns
    );
    println!();

    // Print commit details
    if !result.analyses.is_empty() {
        println!("{}", "📝 Commit Details".bold());
        println!("{}", "─".repeat(50));

        for analysis in &result.analyses {
            print_commit_analysis(analysis, "terminal").ok();
        }
        println!();
    }
}

fn print_json(result: &AtomicityResult) -> Result<()> {
    let json = generate_json(result)?;
    println!("{}", json);
    Ok(())
}

fn generate_json(result: &AtomicityResult) -> Result<String> {
    let json_result = JsonResult {
        total_commits: result.total_commits,
        atomic_commits: result.atomic_commits,
        non_atomic_commits: result.non_atomic_commits,
        skipped_commits: result.skipped_commits,
        has_issues: result.has_issues,
        mode: "check".to_string(),
        commits: result
            .analyses
            .iter()
            .map(|a| JsonCommit {
                sha: a.commit.sha.clone(),
                message: a.commit.message.clone(),
                author: a.commit.author.clone(),
                concerns: a.concerns.iter().map(|c| c.to_string()).collect(),
                is_atomic: a.is_atomic,
                skipped: a.skipped,
            })
            .collect(),
    };

    Ok(serde_json::to_string_pretty(&json_result)?)
}

fn print_markdown(result: &AtomicityResult, config: &Config) -> Result<()> {
    let md = generate_markdown(result, config);
    println!("{}", md);
    Ok(())
}

fn generate_markdown(result: &AtomicityResult, config: &Config) -> String {
    let mut md = String::new();

    md.push_str("## 🔍 Commit Atomicity Check\n\n");

    if result.has_issues {
        md.push_str("⚠️ **Some commits mix multiple concerns**\n\n");
    } else {
        md.push_str("✅ **All commits are atomic**\n\n");
    }

    md.push_str("### 📊 Summary\n\n");
    md.push_str("| Metric | Count |\n");
    md.push_str("|--------|-------|\n");
    md.push_str(&format!("| Total commits | {} |\n", result.total_commits));
    md.push_str(&format!("| ✅ Atomic | {} |\n", result.atomic_commits));
    md.push_str(&format!("| ⚠️ Non-atomic | {} |\n", result.non_atomic_commits));
    md.push_str(&format!("| ⏭️ Skipped (bots) | {} |\n", result.skipped_commits));
    md.push_str("\n");

    if !result.analyses.is_empty() {
        md.push_str("### 📝 Commit Details\n\n");
        md.push_str("| Commit | Message | Concerns |\n");
        md.push_str("|--------|---------|----------|\n");

        for analysis in &result.analyses {
            let concerns: Vec<String> = analysis.concerns.iter().map(|c| c.to_string()).collect();
            let status = if analysis.skipped {
                "⏭️"
            } else if analysis.is_atomic {
                "✅"
            } else {
                "⚠️"
            };

            md.push_str(&format!(
                "| `{}` | {} | {} {} |\n",
                analysis.commit.short_sha,
                analysis.commit.message.replace('|', "\\|"),
                status,
                concerns.join(", ")
            ));
        }
        md.push_str("\n");
    }

    md.push_str("---\n\n");
    md.push_str(&format!(
        "**Mode:** `{}` | **Max concerns:** `{}`\n",
        match config.mode {
            Mode::Error => "error",
            Mode::Warning => "warning",
        },
        config.max_concerns
    ));

    md
}

fn generate_terminal_string(result: &AtomicityResult, config: &Config) -> String {
    let mut output = String::new();

    output.push_str(&format!("\n"));

    if result.has_issues {
        output.push_str("⚠️  Some commits mix multiple concerns\n");
    } else {
        output.push_str("✅ All commits are atomic\n");
    }

    output.push_str(&format!("\n{}\n", "━".repeat(50)));
    output.push_str("📊 Summary\n");
    output.push_str(&format!("{}\n", "━".repeat(50)));
    output.push_str(&format!("   Total commits:    {}\n", result.total_commits));
    output.push_str(&format!("   ✅ Atomic:         {}\n", result.atomic_commits));
    output.push_str(&format!("   ⚠️ Non-atomic:     {}\n", result.non_atomic_commits));
    output.push_str(&format!("   ⏭️  Skipped (bots): {}\n", result.skipped_commits));
    output.push_str(&format!(
        "\n   Mode: {:?} | Max concerns: {}\n\n",
        config.mode, config.max_concerns
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Concern;
    use crate::git::CommitInfo;
    use std::collections::HashSet;

    fn sample_result() -> AtomicityResult {
        let mut concerns = HashSet::new();
        concerns.insert(Concern::Source);

        AtomicityResult {
            total_commits: 3,
            atomic_commits: 2,
            non_atomic_commits: 1,
            skipped_commits: 0,
            has_issues: true,
            analyses: vec![CommitAnalysis {
                commit: CommitInfo {
                    sha: "abc12345".to_string(),
                    short_sha: "abc12345".to_string(),
                    author: "developer".to_string(),
                    message: "feat: add feature".to_string(),
                    files: vec!["src/main.rs".to_string()],
                },
                concerns,
                is_atomic: true,
                skipped: false,
                skip_reason: None,
            }],
        }
    }

    #[test]
    fn test_generate_json() {
        let result = sample_result();
        let json = generate_json(&result).unwrap();
        assert!(json.contains("\"total_commits\": 3"));
        assert!(json.contains("\"has_issues\": true"));
    }

    #[test]
    fn test_generate_markdown() {
        let result = sample_result();
        let config = Config::default();
        let md = generate_markdown(&result, &config);
        assert!(md.contains("## 🔍 Commit Atomicity Check"));
        assert!(md.contains("| Total commits | 3 |"));
    }
}
