# Atomicity Checker 🔍

High-performance commit atomicity analyzer for Git-Core Protocol, written in Rust.

## What is Commit Atomicity?

An **atomic commit** addresses a single concern (feature, bugfix, docs, etc.). Mixing concerns makes history harder to understand and reverts more difficult.

### Concern Categories

| Category | Examples |
|----------|----------|
| `source` | Code changes in `src/`, `lib/`, `.rs`, `.py`, `.js`, etc. |
| `tests` | Test files in `tests/`, `*.test.*`, `*.spec.*` |
| `docs` | Documentation in `docs/`, `*.md` files |
| `config` | Configuration files: `*.yml`, `*.json`, `*.toml`, dotfiles |
| `infra` | CI/CD in `.github/workflows/`, `scripts/` |

## Installation

### Pre-built Binaries

```bash
# Linux
curl -L https://github.com/iberi22/ai-git-core-template/raw/main/bin/atomicity-checker-linux -o atomicity-checker
chmod +x atomicity-checker

# Windows
curl -L https://github.com/iberi22/ai-git-core-template/raw/main/bin/atomicity-checker.exe -o atomicity-checker.exe
```

### Build from Source

```bash
cd tools/atomicity-checker
cargo build --release
# Binary at: target/release/atomicity-checker
```

## Usage

### Check Commits

```bash
# Check commits between main and current HEAD
atomicity-checker check --base main --head HEAD

# With JSON output
atomicity-checker check --base main --head HEAD --output json

# With custom config
atomicity-checker check --base main --head HEAD --config .github/atomicity-config.yml
```

### Analyze Single Commit

```bash
atomicity-checker analyze --commit abc1234
```

### Generate Report

```bash
# Markdown report to stdout
atomicity-checker report --base main --head HEAD --output markdown

# Save to file
atomicity-checker report --base main --head HEAD --output markdown --file report.md
```

## Configuration

Create `.github/atomicity-config.yml`:

```yaml
# Enable/disable the check
enabled: true

# Mode: "warning" (advisory) or "error" (blocks merge)
mode: warning

# Ignore commits from bots
ignore_bots: true

# Maximum concerns per commit (default: 1 = strict atomicity)
max_concerns: 1

# Bot author patterns (regex)
bot_patterns:
  - github-actions
  - dependabot
  - copilot
  - jules
  - renovate
  - bot$
  - \[bot\]

# Files to ignore (glob patterns)
ignore_files:
  - "*.lock"
  - package-lock.json
  - yarn.lock
  - Cargo.lock
  - .gitignore

# Custom categorization rules (pattern -> concern)
custom_rules:
  - pattern: "^migrations/"
    concern: source
  - pattern: "^fixtures/"
    concern: tests
```

## Output Formats

### Terminal (default)
```
🔍 Atomicity Checker v0.1.0
🔍 Analyzing commits: main..HEAD
📊 Found 5 commits to analyze

✅ abc1234: feat: add login (source)
⚠️ def5678: update docs and code
   └─ Mixes 2 concerns: source, docs
✅ ghi9012: test: add unit tests (tests)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Summary
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Total commits:    5
   ✅ Atomic:         4
   ⚠️ Non-atomic:     1
   ⏭️  Skipped (bots): 0
```

### JSON
```json
{
  "total_commits": 5,
  "atomic_commits": 4,
  "non_atomic_commits": 1,
  "skipped_commits": 0,
  "has_issues": true,
  "commits": [
    {
      "sha": "abc1234...",
      "message": "feat: add login",
      "concerns": ["source"],
      "is_atomic": true
    }
  ]
}
```

### Markdown
GitHub-flavored markdown suitable for PR comments or workflow summaries.

## Performance

| Scenario | Shell Script | Rust |
|----------|-------------|------|
| 10 commits | ~2s | ~0.05s |
| 50 commits | ~8s | ~0.15s |
| 100 commits | ~15s | ~0.25s |

## License

MIT
