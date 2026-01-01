---
title: "FEAT: Robust Init and Dependency Management"
labels:
  - enhancement
  - cli
  - ux
assignees: []
---

## Description
Unlock robust initialization usage in `gc init` to handle existing environments intelligently and ensure strict dependency compliance.

The current `gc init` logic is minimal (MVP) and skips interactive decisions. It needs to be upgraded to:
1.  **Detect and Handle Existing States**: If a directory exists/is a repo, ask the user whether to **Overwrite**, **Backup (Rename)**, or **Cancel**.
2.  **Verify All Dependencies**: Check for the presence of:
    -   `git` (Git CLI)
    -   `gh` (GitHub CLI)
    -   `gemini` (Google Gemini CLI)
    -   `copilot` (GitHub Copilot CLI)
    -   `jules` (Jules CLI - Internal/NPM)
3.  **Interactive Prompts**: asking the user for permission to install/update missing tools or proceed with destructive actions.

## Context
User reported that `gc init` identified an existing repo but only created `.ai-core` without touching existing files (leaving them in a "bad state" or ignoring them). The user wants a robust "clean slate" or "migration" option.

## Tasks
- [x] **CLI Logic Update**: Modify `crates/gc-cli/src/commands/init.rs`:
    - [x] Add dependency checks for `gemini`, `copilot`, `jules`.
    - [x] Add valid detection of existing non-empty directory.
    - [x] Implement interactive prompts (Cancel, Backup/Rename, Overwrite).
    - [x] Logic to actually rename the old folder (e.g., `folder_backup_timestamp`) or delete files if "Overwrite" selected.
- [x] **Documentation**:
    - [x] Create `docs/guides/agents/CLI_GUIDE.md`: Comprehensive guide for agents on how/when to use these CLIs.
    - [x] Create `.ai-core/CLI_CONFIG.md`: Configuration reference for CLI orchestration.
    - [x] Update `.github/copilot-instructions.md`: Add basic commands/rules for using these CLIs.
- [ ] **System Port**: Ensure `SystemPort` can handle interactive input (stdin/stdout) if not already capable.

## Definition of Done
- running `gc init` in an existing folder prompts the user.
- running `gc init` without `gemini` installed warns or prompts to install.
- Agents have clear documentation (`CLI_GUIDE.md`) on tool usage.
