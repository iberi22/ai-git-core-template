---
title: "TEST: Implement E2E Testing Framework for CLI"
labels:
  - testing
  - enhancement
  - cli
assignees: []
---

## Description
Set up a comprehensive E2E testing framework for the `git-core` CLI using Rust's best-in-class testing tools.

## Research Summary

### Recommended Testing Stack

| Crate | Purpose | Usage |
|-------|---------|-------|
| **assert_cmd** | CLI integration testing | Spawn binary, assert on stdout/stderr/exit codes |
| **assert_fs** | Filesystem testing | Create temp directories, files for isolated tests |
| **predicates** | Assertion helpers | Rich matchers for output content |
| **trycmd** | Snapshot testing | Document-driven testing, README example verification |
| **insta** | Snapshot management | Complex output diffs, interactive review |

## Test Categories to Implement

### 1. Command Smoke Tests
Verify each command runs without crashing:
```rust
#[test]
fn gc_help_works() {
    Command::cargo_bin("git-core")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git-Core Protocol CLI"));
}
```

### 2. Check Command Tests
```rust
#[test]
fn gc_check_detects_git_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".git").create_dir_all().unwrap();

    Command::cargo_bin("git-core")
        .unwrap()
        .arg("check")
        .current_dir(&temp)
        .assert()
        .success()
        .stdout(predicate::str::contains("Inside Git Repo"));
}
```

### 3. Init Command Tests
- [ ] Test init in empty directory
- [ ] Test init with existing files
- [ ] Test init with `--force` flag
- [ ] Test init without git installed (should fail gracefully)

### 4. Update Command Tests
- [ ] Test update when already at latest version
- [ ] Test update when new version available
- [ ] Test update with `--force` flag

### 5. Context Command Tests
- [ ] Test `gc context list`
- [ ] Test `gc context equip <agent>`
- [ ] Test context with missing AGENT_INDEX.md

### 6. Workflow Command Tests
- [ ] Test `gc workflow --list`
- [ ] Test `gc workflow <name>` for existing workflow
- [ ] Test `gc workflow <name>` for non-existent workflow

### 7. Snapshot Tests (trycmd)
Create `tests/cmd/*.toml` files:
```toml
# tests/cmd/gc_help.toml
bin.name = "git-core"
args = ["--help"]
status.code = 0
```

## Directory Structure
```
tests/
├── cli_tests.rs          # Main integration test file
├── cmd/                   # trycmd snapshot tests
│   ├── gc_help.toml
│   ├── gc_check.toml
│   ├── gc_version.toml
│   └── README.md
└── fixtures/              # Test data
    ├── minimal_project/
    └── protocol_v3_project/
```

## Dependencies to Add
```toml
[dev-dependencies]
assert_cmd = "2.0"
assert_fs = "1.0"
predicates = "3.0"
trycmd = "0.15"
insta = { version = "1.34", features = ["yaml"] }
```

## Tasks
- [ ] Add testing dependencies to gc-cli/Cargo.toml
- [ ] Create tests/cli_tests.rs with basic structure
- [ ] Implement smoke tests for all commands
- [ ] Create test fixtures
- [ ] Add trycmd snapshot tests
- [ ] Integrate with CI workflow
- [ ] Document testing patterns in CONTRIBUTING.md
