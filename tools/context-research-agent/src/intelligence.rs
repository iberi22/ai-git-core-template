use anyhow::Result;
use crate::search::SearchResult;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct Insight {
    pub dependency_name: String,
    pub version: String,
    pub analysis: String,
}

// ============== CONFIGURATION ==============
// Using GitHub Models via `gh models run` - FREE tier
// Model options:
//   - openai/gpt-4o-mini (fast, good quality)
//   - deepseek/deepseek-r1 (excellent reasoning)
//   - openai/o3-mini (reasoning focused)
const MODEL: &str = "openai/gpt-4o-mini";
const RATE_LIMIT_DELAY_MS: u64 = 2000; // 2 seconds between calls
const BATCH_SIZE: usize = 5; // Smaller batches for better analysis

pub async fn analyze_findings(results: Vec<SearchResult>) -> Result<Vec<Insight>> {
    // Check if gh CLI is available
    let gh_check = Command::new("gh")
        .arg("--version")
        .output();
    
    if gh_check.is_err() {
        println!("⚠️ GitHub CLI (gh) not found. Skipping intelligence analysis.");
        return Ok(Vec::new());
    }

    let mut insights = Vec::new();

    // Filter only dependencies with issues (save API calls)
    let relevant: Vec<_> = results.into_iter().filter(|r| !r.issues.is_empty()).collect();
    let total = relevant.len();

    if total == 0 {
        println!("✅ No issues found in dependencies. Skipping analysis.");
        return Ok(Vec::new());
    }

    println!("🧠 Analyzing {} dependencies with issues using GitHub Models ({})...", total, MODEL);

    // Batch dependencies for analysis
    let batches: Vec<Vec<&SearchResult>> = relevant.chunks(BATCH_SIZE).map(|c| c.iter().collect()).collect();
    let total_batches = batches.len();

    println!("📊 Strategy: {} batches of up to {} deps each", total_batches, BATCH_SIZE);

    for (batch_idx, batch) in batches.iter().enumerate() {
        println!("\n📦 Batch {}/{} ({} deps)...", batch_idx + 1, total_batches, batch.len());

        // Build combined prompt for the batch
        let batch_prompt = build_batch_prompt(&batch);

        // Call GitHub Models via gh CLI
        println!("  🔷 Calling GitHub Models ({})...", MODEL);
        let result = call_gh_models(&batch_prompt).await;

        match &result {
            Ok(text) => {
                println!("  ✅ Success! ({} chars)", text.len());
            }
            Err(e) => println!("  ⚠️ Error: {}", e),
        }

        // Store results for each dep in batch
        let analysis_text = result.unwrap_or_else(|e| format!("Analysis failed: {}", e));

        for dep in batch {
            insights.push(Insight {
                dependency_name: dep.dependency.name.clone(),
                version: dep.dependency.version.clone(),
                analysis: analysis_text.clone(),
            });
        }

        // Rate limit pause before next batch (skip on last)
        if batch_idx < total_batches - 1 {
            println!("  ⏳ Rate limit pause ({}ms)...", RATE_LIMIT_DELAY_MS);
            sleep(Duration::from_millis(RATE_LIMIT_DELAY_MS)).await;
        }
    }

    println!("\n✅ Analysis complete! {} insights generated.", insights.len());
    Ok(insights)
}

fn build_batch_prompt(batch: &[&SearchResult]) -> String {
    let mut prompt = String::from(
        "You are a Senior Software Engineer analyzing GitHub issues for multiple libraries. \
        For EACH library below, provide: \
        1. Known Anomalies: Bugs or quirks in THIS SPECIFIC version. \
        2. Anti-patterns to Avoid: Common mistakes found in issues. \
        3. Intelligent Pattern: The recommended way to use this version safely. \
        Be concise but specific. Focus on actionable insights. "
    );

    for (i, res) in batch.iter().enumerate() {
        prompt.push_str(&format!(
            "--- Library {}: {} (version {}) Issues Found: ",
            i + 1, res.dependency.name, res.dependency.version
        ));
        for issue in &res.issues {
            prompt.push_str(&format!("[{}] {}. ", issue.state, issue.title));
        }
    }

    prompt
}

async fn call_gh_models(prompt: &str) -> Result<String> {
    // Use gh models run with the prompt
    let output = Command::new("gh")
        .args([
            "models",
            "run",
            MODEL,
            prompt,
            "--max-tokens", "2048",
        ])
        .output()?;

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        if response.trim().is_empty() {
            return Err(anyhow::anyhow!("Empty response from GitHub Models"));
        }
        Ok(response)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow::anyhow!("GitHub Models error: {}", stderr))
    }
}
