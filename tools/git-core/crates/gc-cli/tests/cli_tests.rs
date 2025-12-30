use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

/// Helper to get the CLI binary
fn git_core() -> Command {
    Command::cargo_bin("git-core").unwrap()
}

// ============================================================================
// SMOKE TESTS - Verify each command runs without panicking
// ============================================================================

#[test]
fn test_help_works() {
    git_core()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git-Core Protocol CLI"));
}

#[test]
fn test_version_works() {
    git_core()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("gc"));
}

#[test]
fn test_check_help() {
    git_core()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Environment"));
}

#[test]
fn test_init_help() {
    git_core()
        .args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("project"));
}

#[test]
fn test_update_help() {
    git_core()
        .args(["update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Protocol"));
}

#[test]
fn test_update_smoke() {
    // We cannot easily test full network download in unit tests without mocking reqwest.
    // However, we can test that the command accepts arguments and fails gracefully or prints help.
    let temp = assert_fs::TempDir::new().unwrap();
    git_core()
        .arg("update")
        .current_dir(&temp)
        .assert()
        // It might fail due to network or return success saying update available
        // We just check it doesn't panic.
        .code(predicate::in_iter(vec![0, 1]));
}

#[test]
fn test_context_help() {
    git_core()
        .args(["context", "--help"])
        .assert()
        .success();
}

#[test]
fn test_workflow_help() {
    git_core()
        .args(["workflow", "--help"])
        .assert()
        .success();
}

#[test]
fn test_dispatch_help() {
    git_core()
        .args(["dispatch", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent"));
}

// ============================================================================
// CHECK COMMAND TESTS
// ============================================================================

#[test]
fn test_check_outside_git_repo() {
    let temp = assert_fs::TempDir::new().unwrap();

    git_core()
        .arg("check")
        .current_dir(&temp)
        .assert()
        .success()  // Should still succeed but report issues
        .stdout(predicate::str::contains("Git Installed"));
}

#[test]
fn test_check_inside_git_repo() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".git").create_dir_all().unwrap();

    git_core()
        .arg("check")
        .current_dir(&temp)
        .assert()
        .success()
        .stdout(predicate::str::contains("Inside Git Repo"));
}

#[test]
fn test_check_json_output() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".git").create_dir_all().unwrap();

    git_core()
        .args(["check", "--json"])
        .current_dir(&temp)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"git_installed\""));
}

#[test]
fn test_check_detects_protocol_version() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".git").create_dir_all().unwrap();
    temp.child(".git-core-protocol-version").write_str("3.0.0").unwrap();

    git_core()
        .arg("check")
        .current_dir(&temp)
        .assert()
        .success()
        .stdout(predicate::str::contains("Protocol Version: 3.0.0"));
}

// ============================================================================
// WORKFLOW COMMAND TESTS
// ============================================================================

#[test]
fn test_workflow_list_empty() {
    let temp = assert_fs::TempDir::new().unwrap();

    git_core()
        .args(["workflow", "--list"])
        .current_dir(&temp)
        .assert()
        .success();
}

#[test]
fn test_workflow_list_with_workflows() {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child(".agent/workflows").create_dir_all().unwrap();
    temp.child(".agent/workflows/deploy.md")
        .write_str("---\ndescription: Deploy to production\n---\n\n1. Run tests\n2. Deploy")
        .unwrap();

    git_core()
        .args(["workflow", "--list"])
        .current_dir(&temp)
        .assert()
        .success()
        // Just verify it runs and outputs something about workflows
        .stdout(predicate::str::contains("Workflow"));
}

#[test]
fn test_workflow_show_nonexistent() {
    let temp = assert_fs::TempDir::new().unwrap();

    git_core()
        .args(["workflow", "nonexistent"])
        .current_dir(&temp)
        .assert()
        .failure();  // Should fail for non-existent workflow
}

// ============================================================================
// INTEGRATION SCENARIOS
// ============================================================================

#[test]
fn test_full_protocol_project_check() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Setup a minimal protocol-compliant project
    temp.child(".git").create_dir_all().unwrap();
    temp.child(".git-core-protocol-version").write_str("3.0.0").unwrap();
    temp.child(".ai-core").create_dir_all().unwrap();
    temp.child(".ai-core/ARCHITECTURE.md").write_str("# Architecture").unwrap();
    temp.child(".github/issues").create_dir_all().unwrap();
    temp.child("AGENTS.md").write_str("# Agents").unwrap();

    git_core()
        .arg("check")
        .current_dir(&temp)
        .assert()
        .success()
        .stdout(predicate::str::contains("Protocol Version: 3.0.0"));
}
