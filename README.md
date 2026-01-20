---
title: "Git-Core Protocol - README"
type: DOCUMENTATION
id: "doc-readme"
created: 2025-12-01
updated: 2025-12-13
agent: copilot
model: claude-sonnet-4
requested_by: system
summary: |
  Project overview, quick start guide, and core principles of the Git-Core Protocol.
  Optimized with Rust-powered agents and simplified architecture.
keywords: [git-core, protocol, ai-agent, template, llm, copilot, claude, gemini, grok, rust]
tags: ["#documentation", "#readme", "#core", "#v3.5.1"]
project: Git-Core-Protocol
version: 3.5.1
---

# 🧠 Git-Core Protocol

[![Use this template](https://img.shields.io/badge/Use%20this-template-blue?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/generate)
[![Version](https://img.shields.io/badge/Version-3.5.1-green?style=for-the-badge)](https://github.com/iberi22/Git-Core-Protocol/releases/tag/v3.5.1)
[![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg?style=for-the-badge)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![AI Code Review](https://img.shields.io/badge/AI%20Review-CodeRabbit%20%2B%20Gemini-purple?style=for-the-badge)](https://github.com/marketplace/coderabbit)

<div align="center" style="display: flex; align-items: center; justify-content: center; gap: 20px; flex-wrap: wrap;">

<img src="logo.png" alt="Git-Core Protocol Logo" width="200">

<div style="flex: 1; min-width: 500px;">

<<<<<<< Updated upstream
### ⚡ Performance-First AI Development
=======
### 🚀 Active Automated Evolution

**Git-Core Protocol** is a living standard for AI-assisted development. It provides a structured workflow where **Human ↔ AI Agent ↔ GitHub** communicate seamlessly.
>>>>>>> Stashed changes

**Git-Core Protocol** is a living standard for AI-assisted development, now powered by **Rust-native agents** for 10-30x speedup. It provides a structured workflow where **Human ↔ AI Agent ↔ GitHub** communicate seamlessly.

- **How it helps:** Eliminates context loss, enforces architectural consistency, and automates documentation at blazing speed.
- **Workflow:** Issues are state, Commits are atomic, PRs are auto-analyzed, and agents work in sub-second time.
- **Status:** Production-ready with self-healing CI/CD and zero-overhead state management.

| Git-Core Protocol | Compliance | Detail                                      |
|-------------------|------------|---------------------------------------------|
| Defined rules     | ✅          | AGENTS.md, copilot-instructions.md           |
| Syntax            | ✅          | Issue format, conventional commits           |
| Semantics         | ✅          | Labels, states, workflows                    |
| Synchronization   | ✅          | GitHub Issues as state, workflows            |

</div>

</div>

---

## 🌍 Language | Idioma | Língua | Sprache | Langue | 言語 | 语言

<div align="center">

| Flag | Language | Link |
|:----:|----------|------|
| 🇺🇸 | **English** | [Jump to English](#-english) |
| 🇪🇸 | **Español** | [Ir a Español](#-español) |
| 🇧🇷 | **Português** | [Ir para Português](#-português) |
| 🇩🇪 | **Deutsch** | [Zu Deutsch](#-deutsch) |
| 🇫🇷 | **Français** | [Aller au Français](#-français) |
| 🇯🇵 | **日本語** | [日本語へ](#-日本語) |
| 🇨🇳 | **中文** | [跳转到中文](#-中文) |

</div>

---

# 🇺🇸 English

## 🚀 Quick Start - Just Copy This Prompt

> **Copy this prompt to your AI coding agent (Copilot, Cursor, Claude, etc.) to auto-setup:**

Initialize this project with Git-Core Protocol.

# Method 1: Rust CLI (Recommended)

 cargo install --path . # If local

# OR

 gc init

# Method 2: Shell Scripts (Legacy - Use only if CLI fails)

# Linux/macOS

curl -fsSL <https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh> | bash

# Windows

irm <https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1> | iex

After setup, read .gitcore/ARCHITECTURE.md and start with: gc issue list

```

**For AI Agents (auto mode):**

For Windows:

```

$env:GIT_CORE_AUTO = "1"; irm <https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1> | iex
Then execute: gc init --auto

```

$env:GIT_CORE_AUTO = "1"; irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
Then execute: gc init --auto

```

> **✨ Smart Detect:** The installer checks if you have the Rust-based Agents installed globally (in your PATH). If detected, it asks if you want to skip downloading local binaries to save space.

For Linux/Mac:

```

curl -fsSL <https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh> | bash -s -- --auto
Then execute: ./scripts/init_project.sh

```

---

## 📖 CLI Usage Manual

The `gc` (Git-Core) CLI is the primary tool for interacting with the protocol.

<<<<<<< Updated upstream
> **⚠️ IMPORTANT:** Most commands (like `issue`, `pr`, `git`) require you to be inside a **Git repository** to function correctly. They automatically detect the repository context from the `.git` folder.
=======
### Available Recipes (Auto-Downloaded)

The system automatically downloads the latest "recipes" for:

- 🏗️ **Software Engineering** (Patterns, Clean Code)
- 🔬 **Research** (Academic, Technical)
- 🛡️ **Cybersecurity** (Auditing, Hardening)
- 🎨 **UI/UX Design** (Accessibility, Systems)
- ⛓️ **Blockchain** (Smart Contracts, Web3)
- 🤖 **AI Research** (Papers, State of the Art)
- 🎓 **AI Training** (Fine-tuning, Datasets)
>>>>>>> Stashed changes

### 🛠️ Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `gc init` | Initialize Git-Core in a new project | `gc init` |
| `gc info` | Show project info (Team/Solo, contributors) | `gc info` |

### 📋 Workflow Management

| Command | Description | Example |
|---------|-------------|---------|
| `gc issue list` | List issues (default: open) | `gc issue list --limit 5` |
| `gc issue list --assigned-to-me` | List issues assigned to you | `gc issue list --assigned-to-me` |
| `gc issue list --state <STATE>` | Filter by state (open/closed/all) | `gc issue list --state closed` |
| `gc pr list` | List open Pull Requests | `gc pr list` |
| `gc task start` | Start a new task (creates branch & issue) | `gc task start "Fix login bug"` |
| `gc finish` | Finish current task (PR + Report) | `gc finish` |

### 🔍 Context & Git

| Command | Description | Example |
|---------|-------------|---------|
| `gc git status` | Show concise git status | `gc git status` |
| `gc git log` | Show recent git history | `gc git log --limit 5` |
| `gc context list` | List available agent roles | `gc context list` |
| `gc context equip <ROLE>` | Load a specific agent role | `gc context equip security` |

### 🤖 Automation & CI

| Command | Description | Example |
|---------|-------------|---------|
| `gc validate run` | Validate workflow runs | `gc validate run` |
| `gc validate analyze` | Analyze repo (errors, perf, security) | `gc validate analyze` |
| `gc report` | Generate AI Pull Request report | `gc report --pr 42` |
| `gc ci-detect` | Detect CI environment details | `gc ci-detect` |
| `gc telemetry` | Send anonymous usage stats | `gc telemetry` |

---

### 🚨 Important Notes

1. **Repository Context Required:**
   - Commands like `gc issue`, `gc pr`, and `gc git` need to be run **inside a Git repository**.
   - The CLI automatically detects the repository by looking for the `.git` folder.
   - If you run these commands outside a repo, you'll see: `fatal: not a git repository`

2. **GitHub Token:**
   - Set your GitHub token for API access:
     ```bash
     # Windows (PowerShell)
     $env:GITHUB_TOKEN = "ghp_your_token_here"

     # Linux/macOS
     export GITHUB_TOKEN="ghp_your_token_here"
     ```
   - Required for: `gc issue`, `gc pr`, `gc context equip`, `gc report`

3. **Installation:**
   - The CLI binary is named `gc-cli` after installation
   - You can create an alias `gc` for convenience:
     ```bash
     # Add to your shell profile (~/.bashrc, ~/.zshrc, or PowerShell profile)
     Set-Alias gc gc-cli  # PowerShell
     alias gc='gc-cli'    # Bash/Zsh
     ```

---

## ✨ Features

### 🤖 Context Injector

Dynamic system to "equip" your AI agent with specialized roles on demand.

**Available Roles:**
- 🏗️ Software Engineering | 🔬 Research | 🛡️ Cybersecurity
- 🎨 UI/UX Design | ⛓️ Blockchain | 🤖 AI Research | 🎓 AI Training

```powershell
# Load a specialized persona
./scripts/equip-agent.ps1 -Role "security"
```

---

### 🧠 Model-Specific Agents

> ⚠️ **VS Code Only** - Requires GitHub Copilot Chat extension

Custom Copilot agents optimized for different LLM models:

| Agent | Model | Best For |
|-------|-------|----------|
| `@protocol-claude` | Claude Sonnet 4 | Standard tasks |
| `@architect` | Claude Opus 4.5 | Architecture decisions |
| `@quick` | Claude Haiku 4.5 | Fast responses |
| `@protocol-gemini` | Gemini 3 Pro | Large context (1M+) |
| `@protocol-grok` | Grok Code Fast 1 | Massive analysis (2M) |

---

### 🔄 Workflow Agents

> ⚠️ **VS Code Only** - Requires GitHub Copilot Chat extension

| Agent | Purpose |
|-------|---------|
| `@context-loader` | Auto-discovers project state |
| `@workflow-manager` | Orchestrates multi-step workflows |
| `@commit-helper` | Fast atomic commits |
| `@pr-creator` | Creates well-formatted PRs |

---

---

### 📊 AI Report Generation

Automated PR analysis using Gemini or Copilot CLI:

```powershell
./scripts/ai-report.ps1 -PrNumber 42          # Full report
./scripts/ai-report.ps1 -ReportType gemini    # Gemini only
./scripts/ai-report.ps1 -DryRun               # Preview
```

---

### 📤 Session Export

Continue work in a new chat **without losing context**:

```powershell
./scripts/export-session.ps1 -Summary "OAuth implementation" -Topic "oauth"
```

Exports: Git status, recent commits, open issues, and pending tasks.

---

---

### 🎯 Adaptive Workflows

Automatic detection of repository type (public/private) to optimize GitHub Actions:

<<<<<<< Updated upstream
| Mode | Schedules | Cost |
|------|-----------|------|
| 🟢 **Public** | Every 6 hours | $0 (unlimited) |
| 🟡 **Private** | Event-based only | Free tier |
=======
The protocol now **automatically adapts** based on whether your repository is public or private:

```
┌─────────────────────────────────────────────────────────┐
│  DETECTOR  →  IS_PUBLIC?  →  SCHEDULE_MODE              │
│     ↓              ↓              ↓                       │
│  gh repo      true/false    aggressive/moderate/         │
│  view                       conservative                 │
└─────────────────────────────────────────────────────────┘
```

**Three Modes:**

| Mode | When | Schedules | Consumption | Cost |
|------|------|-----------|-------------|------|
| 🟢 **AGGRESSIVE** | Public repos | Every 30 min | ~18,000 min/month | $0 (unlimited) |
| 🟡 **MODERATE** | Private main | Every 6 hours | ~3,000 min/month | Requires Pro |
| 🔴 **CONSERVATIVE** | Other private | Event-based only | ~600 min/month | $0 (Free tier) |

**Key Benefits:**

- ✅ **Zero-configuration**: Automatic detection and adaptation
- ✅ **97% savings** on private repositories
- ✅ **100% functionality** maintained via event-based triggers
- ✅ **Cross-platform**: PowerShell + Bash support

**Usage:**

```bash
# Test detection locally
./scripts/detect-repo-config.ps1  # Windows
./scripts/detect-repo-config.sh   # Linux/macOS

# Workflows automatically use the detection
# No manual configuration needed!
```

📖 **Full documentation:** [docs/ADAPTIVE_WORKFLOWS.md](docs/ADAPTIVE_WORKFLOWS.md)
>>>>>>> Stashed changes

---

### 🧠 Agent State Protocol

<<<<<<< Updated upstream
Agents persist state in GitHub Issues using XML blocks for pausable/resumable workflows:
=======
The latest version integrates advanced patterns from **[12-Factor Agents](https://github.com/humanlayer/12-factor-agents)** and **Agent Control Plane (ACP)**:

### 🧠 Context Protocol (Stateless Reducer)

Agents persist state in GitHub Issues using structured XML blocks. This enables:

- **Pausable/Resumable workflows**: Any agent can pick up where another left off
- **Dynamic Planning**: `<plan>` field with items marked `done`/`in_progress`/`pending`
- **Human-as-Tool**: `<input_request>` for structured data requests (not just approvals)
- **Observability**: `<metrics>` tracks tool calls, errors, and cost estimates
>>>>>>> Stashed changes

```bash
./scripts/agent-state.ps1 read -IssueNumber 42
```

---

### 🛡️ High-Stakes Operations

Critical operations (deletions, deploys) require explicit confirmation:

<<<<<<< Updated upstream
> ⚠️ **HIGH STAKES ACTION DETECTED**. Respond 'Proceder' to continue.
=======
The protocol now **self-improves** through automated weekly analysis:

```
MEDIR → ANALIZAR → PROPONER → IMPLEMENTAR → VALIDAR → ↺
```

**Features:**

- **3-Order Metrics Taxonomy**: Operational (daily), Quality (weekly), Evolution (monthly)
- **Automated Pattern Detection**: Identifies "death loops", low adoption, high friction
- **Weekly Reports**: Auto-generated GitHub Issues with insights

```powershell
# Collect local metrics
./scripts/evolution-metrics.ps1 -OutputFormat markdown

# Trigger evolution cycle (runs every Monday automatically)
gh workflow run evolution-cycle.yml
```

👉 **Full spec:** [docs/agent-docs/EVOLUTION_PROTOCOL.md](docs/agent-docs/EVOLUTION_PROTOCOL.md)

### 📡 Federated Telemetry System

Projects using Git-Core Protocol can **send anonymized metrics back** to the official repo for centralized analysis:

```
┌─────────────────┐    PR with metrics    ┌─────────────────────┐
│  Your Project   │ ─────────────────────▶│ Official Git-Core   │
│  (uses protocol)│                       │ Protocol Repo       │
└─────────────────┘                       │   (analysis)        │
                                          └─────────────────────┘
```

**Usage:**

```powershell
# Preview what would be sent
./scripts/send-telemetry.ps1 -DryRun

# Send anonymized metrics
./scripts/send-telemetry.ps1
```

**Privacy:**

- ✅ Anonymous by default (project names hashed)
- ✅ Only numbers (no code, no content)
- ✅ Opt-in only (you choose when to send)

👉 **Full spec:** [telemetry/README.md](telemetry/README.md)
>>>>>>> Stashed changes

---

### 🧬 Self-Healing CI/CD

Automated error classification and retry for transient failures:

<<<<<<< Updated upstream
- **Transient errors** (timeouts, rate limits): Auto-retry
- **Dependency errors**: Creates issue for review
- **Test failures**: Creates issue with diagnosis

---
=======
### 🛣️ Milestones

- [x] **v1.4.0**: ✅ Model-Specific Agents, Session Export, AI Reports
- [x] **v2.1 (Context Protocol)**: ✅ XML Agent State, Micro-Agents, HumanLayer
- [x] **v1.5.0**: ✅ Evolution Protocol, Federated Telemetry
- [ ] **v2.2**: "Memory Core" - Persistent semantic memory across sessions
- [ ] **v2.3**: Multi-Agent Swarm Protocol (Coordinator + Workers)
- [ ] **v3.0**: Native IDE Integration (VS Code Extension)

### 🤝 We Need Your Feedback

This protocol is in **active automated evolution**. We need you to test it and report:

1. **Friction points:** Where did the agent get stuck?
2. **Missing recipes:** What role did you need that wasn't there?
3. **Workflow bugs:** Did the state get out of sync?
>>>>>>> Stashed changes

### 🤖 Automated Triage Agent

New in v3.5.1! Automatically manages incoming issues:
- **Auto-Labeling**: Detects keywords (bug, feature, ci) and applies labels.
- **Smart Routing**: Assigns issues to relevant teams based on context.
- **CI Integration**: Automatically creates detailed issues for workflow failures with log snippets.

---

### 📦 Dependency Quarantine

14-day quarantine for new dependencies with AI analysis before adoption.

📖 **Full changelog:** [CHANGELOG.md](CHANGELOG.md)

---

## 🤝 Feedback

This protocol evolves through user feedback. Report:

- **Friction points**: Where did the agent get stuck?
- **Missing recipes**: What role did you need?
- **Workflow bugs**: Did state get out of sync?

👉 **[Open a Discussion](https://github.com/iberi22/Git-Core-Protocol/discussions)** or create an Issue with `feedback` label.

---

## Why This Approach?

| Problem | Git-Core Solution |
|---------|-------------------|
| AI "forgets" task state | State in GitHub Issues (persistent) |
| Context grows = more tokens = more cost | Only load current issue + architecture |
| Messy TODO.md files | Organized GitHub board |
| Ecosystem dependency (NPM, etc.) | Language-agnostic bash/PowerShell scripts |

## 📦 Installation Options

**🔐 Trust & Transparency:** Before installing, read [docs/CLI_TRUST.md](docs/CLI_TRUST.md) to understand exactly what each method does.

### Option 1: Shell Scripts (🚀 Transparent - Recommended)

Scripts are **visible code** you can read before running:

```bash
# View the code BEFORE running:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh

# Linux/macOS - If you trust it, run:
curl -fsSL https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.sh | bash

# Windows - View code first:
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1" | Select-Object -ExpandProperty Content

# Windows - Then run:
irm https://raw.githubusercontent.com/iberi22/Git-Core-Protocol/main/install.ps1 | iex
```

### Option 2: Git-Core CLI (🦀 Full Features)

The official CLI provides the best management experience:

```bash
# 🦀 Cargo (compiles from source on YOUR machine)
# Before installing, read: docs/CLI_TRUST.md
# Source code: https://github.com/iberi22/Git-Core-Protocol/tree/main/tools/git-core-cli
cargo install git-core-cli

# 🔨 Or build from source (maximum trust)
git clone https://github.com/iberi22/Git-Core-Protocol
cd Git-Core-Protocol/tools/git-core-cli
cargo build --release
./target/release/git-core install
```

**CLI Commands:**

```bash
# Install protocol in current project
git-core install

# Initialize a new project
git-core init my-project

# Upgrade existing installation
git-core upgrade

# Check protocol integrity
git-core check

# Migrate from .ai/ to .gitcore/
git-core migrate
```

### Option 3: Use as Template

1. Click **"Use this template"** above
2. Clone your new repository
3. Run: `curl -fsSL .../install.sh | bash` or `git-core install`

**Method Comparison:**

<div align="center">

| Method | Trust Level | Speed | Features |
|--------|-------------|-------|----------|
| **Rust CLI** | ⭐⭐⭐⭐ (compiled) | **Fastest** | **Integrated** |
| Shell Scripts | ⭐⭐⭐⭐⭐ (visible code) | Fast | Basic |
| Cargo install | ⭐⭐⭐⭐ (compiles locally) | Medium | Full |
| Build from source | ⭐⭐⭐⭐⭐ (maximum control) | Slow | Full |

</div>

## 📂 Structure

```
/
├── .gitcore/
│   ├── ARCHITECTURE.md       # 📖 System context
│   ├── AGENT_INDEX.md        # 🎭 Agent roles and routing
│   └── CONTEXT_LOG.md        # 📝 Ephemeral session notes
├── .github/
│   ├── agents/               # 🤖 Model-specific agents (NEW!)
│   │   ├── protocol-claude.agent.md
│   │   ├── protocol-gemini.agent.md
│   │   ├── protocol-codex.agent.md
│   │   ├── protocol-grok.agent.md
│   │   ├── architect.agent.md
│   │   ├── quick.agent.md
│   │   ├── router.agent.md
│   │   └── workflow-*.agent.md  # Workflow agents
│   ├── instructions/         # 📋 Model-specific instructions
│   │   ├── claude-tools.instructions.md
│   │   ├── gemini-tools.instructions.md
│   │   ├── codex-tools.instructions.md
│   │   ├── grok-tools.instructions.md
│   │   └── fallback-system.instructions.md
│   ├── copilot-instructions.md  # 🤖 GitHub Copilot rules
│   └── ISSUE_TEMPLATE/       # 📋 Issue templates
├── scripts/
│   ├── init_project.sh       # 🐧 Linux/Mac initializer
│   ├── init_project.ps1      # 🪟 Windows initializer
│   ├── equip-agent.ps1       # 🎭 Recipe loader (Windows)
│   ├── equip-agent.sh        # 🎭 Recipe loader (Linux/Mac)
│   ├── install-cli.sh        # 🛠️ CLI installer (Linux/macOS)
│   └── install-cli.ps1       # 🛠️ CLI installer (Windows)
├── tools/
│   └── git-core-cli/         # 🦀 Official Rust CLI source
├── AGENTS.md                 # 🤖 All AI agents config
├── .cursorrules              # 🎯 Cursor rules
└── .windsurfrules            # 🌊 Windsurf rules
```

## 🔄 The Workflow Loop

```
┌─────────────────────────────────────────────────────────┐
│                    THE LOOP                              │
├─────────────────────────────────────────────────────────┤
│   1. READ: cat .gitcore/ARCHITECTURE.md                      │
│           gh issue list --assignee "@me"                │
│   2. ACT:  gh issue edit <id> --add-assignee "@me"      │
│           git checkout -b feat/issue-<id>               │
│   3. UPDATE: git commit -m "feat: ... (closes #<id>)"   │
│             gh pr create --fill                         │
│   ↺ Repeat                                               │
└─────────────────────────────────────────────────────────┘
```

## 📊 Issue Lifecycle & Progress Tracking

**Issues stay OPEN** while they have pending tasks. They **close automatically** when a commit includes `closes #X`.

```
┌─────────────────────────────────────────────────────────┐
│  OPEN                                                   │
│  ├── 📋 Backlog: No one assigned, waiting               │
│  ├── 🔄 In Progress: Someone assigned, working          │
│  └── ⏸️ Blocked: Waiting for dependency                 │
└─────────────────────────────────────────────────────────┘
                         │
                         │ Commit with "closes #X"
                         ▼
┌─────────────────────────────────────────────────────────┐
│  CLOSED                                                 │
│  └── ✅ Completed: All tasks done                       │
└─────────────────────────────────────────────────────────┘
```

**Progress Tracking:** Use an **EPIC issue** with checkboxes to track overall progress. GitHub automatically calculates the percentage. No local files needed!

```markdown
# Example EPIC Issue
- [x] Task 1 completed
- [x] Task 2 completed
- [ ] Task 3 pending
- [ ] Task 4 pending
# GitHub shows: ██████░░░░ 50%
```

## 🤖 Compatible AI Agents

✅ GitHub Copilot | ✅ Cursor | ✅ Windsurf | ✅ Claude | ✅ ChatGPT | ✅ Any LLM with terminal access

## 🤝 Credits & Inspiration

This protocol is inspired by and builds upon the excellent work of:

- **[HumanLayer](https://github.com/humanlayer/humanlayer)**: For their pioneering work on "12-Factor Agents" and "Context Engineering".
- **[CodeLayer](https://humanlayer.dev/code)**: For demonstrating advanced agent orchestration.
- **Context7**: For the initial concepts of context management.
- **[Git](https://git-scm.com/)**: To be free to use.
- **[GitHub](https://github.com/)**: for shered the infrastructure, and all his community of developers.
- **[anthropic](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents/)**:

We acknowledge their contributions to the field of AI-assisted development.
