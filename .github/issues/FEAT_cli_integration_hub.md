---
title: "FEAT: CLI Integration Hub - Cross-Tool Communication Layer"
labels:
  - enhancement
  - architecture
  - cli
assignees: []
---

## Description
Implement a native integration layer that allows `git-core` CLI to orchestrate and communicate with other CLI tools (`gh`, `git`, `jules`, `copilot`) directly from Rust, without relying on shell scripts.

## Research Findings

### Similar Projects Analyzed

| Project | Technology | Key Features to Extract |
|---------|------------|------------------------|
| **Sapling SCM** | Rust + Python | `sl` prefix, smartlog, undo/redo stack, `sl hide/unhide` |
| **GitHub Copilot CLI** | Node.js | Natural language commands, MCP integration, `/agent` slash commands |
| **Jules Tools CLI** | Node.js | Async task assignment, `jules remote new`, scriptable piping |
| **Lazygit** | Go | Terminal UI for git, visual diffs |
| **just** | Rust | Task runner, Justfile format |

### CLI Communication Patterns (from Rust)

1. **Subprocess Spawning** (`std::process::Command`)
   - Direct execution of `gh`, `git`, `jules` binaries
   - Capture stdout/stderr for parsing
   - Environment variable passing

2. **JSON API Integration**
   - `gh api` returns JSON, parseable with `serde_json`
   - `gh issue list --json` for structured data
   - Jules supports `--json` output

3. **Piping Between CLIs**
   - Example: `gh issue list --assignee @me --limit 1 --json title | jq -r '.[0].title' | jules remote new --repo .`
   - Our CLI can orchestrate this natively

## Proposed Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      git-core CLI (Rust)                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────┐ │
│  │  GitPort    │  │ GitHubPort  │  │  JulesPort  │  │CopilotPt│ │
│  │ (git spawn) │  │ (gh spawn)  │  │ (jules spwn)│  │(copilot)│ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └────┬────┘ │
│         │                │                │              │      │
│  ┌──────▼────────────────▼────────────────▼──────────────▼────┐ │
│  │              Unified Dispatcher / Orchestrator             │ │
│  │  - Route tasks to appropriate agent                        │ │
│  │  - Handle async responses                                  │ │
│  │  - Aggregate results                                       │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Features to Implement

### Phase 1: Native Integrations
- [ ] `gc issue` - Wrapper for `gh issue` with enhanced formatting
- [ ] `gc pr` - Wrapper for `gh pr` with protocol-aware descriptions
- [ ] `gc commit` - Smart commits with issue linking
- [ ] `gc branch` - Branch creation following naming conventions

### Phase 2: Agent Orchestration
- [ ] `gc dispatch jules "<task>"` - Send task to Jules asynchronously
- [ ] `gc dispatch copilot "<task>"` - Delegate to Copilot CLI
- [ ] `gc status` - Check status of all dispatched tasks

### Phase 3: Advanced Pipelines
- [ ] `gc autofix` - Pipe linting errors to agent for fix
- [ ] `gc review` - Request AI review across multiple providers
- [ ] `gc sync` - Sync issues, PRs, and agent states

## Implementation Notes

### Rust Subprocess Best Practices
```rust
use std::process::Command;

// Execute gh and capture JSON output
let output = Command::new("gh")
    .args(["issue", "list", "--json", "number,title,labels"])
    .output()?;

let issues: Vec<Issue> = serde_json::from_slice(&output.stdout)?;
```

### Error Handling
- Check if binary exists in PATH before spawning
- Graceful fallbacks when tools are not installed
- Clear error messages indicating which tool is missing

## Tasks
- [ ] Add `assert_cmd` and `trycmd` for E2E testing
- [ ] Create GitPort implementation using subprocess
- [ ] Create JulesPort trait and implementation
- [ ] Create CopilotPort trait and implementation
- [ ] Implement unified dispatcher
- [ ] Add integration tests for each port
