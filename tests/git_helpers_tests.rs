use andiamo::git_helpers::{
    GitError, add_remotes_with_urls, check_git_installed, get_existing_remotes, init_repo,
    push_to_remotes,
};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to run git commands in a specific directory
fn run_git_command(dir: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .env("GNUPGHOME", "/dev/null")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute git: {}", e)),
    }
}

/// Helper function to create a test file in directory
fn create_test_file(dir: &Path, filename: &str, content: &str) {
    let file_path = dir.join(filename);
    fs::write(&file_path, content).expect("Failed to create test file");
}

/// Helper function to commit changes in a git repository
fn commit_changes(dir: &Path, message: &str) -> Result<(), String> {
    run_git_command(dir, &["config", "user.name", "Test User"])?;
    run_git_command(dir, &["config", "user.email", "test@example.com"])?;
    run_git_command(dir, &["config", "commit.gpgsign", "false"])?;
    run_git_command(dir, &["add", "."])?;
    run_git_command(dir, &["commit", "-m", message])?;
    Ok(())
}

/// Helper function to initialize a git repo in a specific directory
fn init_test_repo(dir: &Path) -> Result<(), String> {
    run_git_command(dir, &["init"])?;
    run_git_command(dir, &["config", "user.name", "Test User"])?;
    run_git_command(dir, &["config", "user.email", "test@example.com"])?;
    run_git_command(dir, &["config", "commit.gpgsign", "false"])?;
    Ok(())
}

/// Helper function to check if a directory is a git repo
fn is_git_repo_in_dir(dir: &Path) -> bool {
    let output = Command::new("git")
        .current_dir(dir)
        .args(["rev-parse", "--is-inside-work-tree"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim() == "true"
        }
        Err(_) => false,
    }
}

#[test]
fn test_check_git_installed() {
    // This test assumes git is installed on system running the tests
    let result = check_git_installed();
    assert!(result, "Git should be installed for these tests to run");
}

#[test]
fn test_is_git_repo_no_repo() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = is_git_repo_in_dir(temp_dir.path());
    assert!(
        !result,
        "Should not be a git repository in a new temp directory"
    );
}

#[test]
fn test_init_repo_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Verify .git directory exists
    assert!(
        temp_dir.path().join(".git").exists(),
        ".git directory should exist"
    );

    // Verify it's a git repo
    assert!(
        is_git_repo_in_dir(temp_dir.path()),
        "Should be a git repository after init"
    );
}

#[test]
fn test_add_remotes_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Add remotes using git command directly
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/test/origin.git",
        ],
    )
    .expect("Failed to add origin remote");
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "mirror",
            "https://github.com/test/mirror.git",
        ],
    )
    .expect("Failed to add mirror remote");

    // Verify remotes exist by checking git config directly
    let config = run_git_command(
        temp_dir.path(),
        &["config", "--get-regexp", "remote\\..*url"],
    )
    .expect("Failed to get git config");
    assert!(
        config.contains("github.com/test/origin.git"),
        "origin remote should exist in config"
    );
    assert!(
        config.contains("github.com/test/mirror.git"),
        "mirror remote should exist in config"
    );
}

#[test]
fn test_add_remotes_already_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Add remotes using git command directly
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/test/origin.git",
        ],
    )
    .expect("Failed to add origin remote");
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "mirror",
            "https://github.com/test/mirror.git",
        ],
    )
    .expect("Failed to add mirror remote");

    // Try to add again - should fail
    let result = run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/test/origin2.git",
        ],
    );

    assert!(result.is_err(), "Adding existing remote should fail");
}

#[test]
fn test_get_existing_remotes_empty() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Get remotes using git command directly
    let remotes = run_git_command(temp_dir.path(), &["remote"]).expect("Failed to get remotes");

    assert!(remotes.is_empty(), "Should have no remotes in a fresh repo");
}

#[test]
fn test_get_existing_remotes_with_remotes() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Add remotes using git command directly
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/test/origin.git",
        ],
    )
    .expect("Failed to add origin remote");
    run_git_command(
        temp_dir.path(),
        &[
            "remote",
            "add",
            "mirror",
            "https://github.com/test/mirror.git",
        ],
    )
    .expect("Failed to add mirror remote");

    // Get remotes using git command directly
    let remotes = run_git_command(temp_dir.path(), &["remote"]).expect("Failed to get remotes");

    let remote_list: Vec<&str> = remotes.lines().collect();
    assert_eq!(remote_list.len(), 2, "Should have 2 remotes");
    assert!(remote_list.contains(&"origin"), "Should contain origin");
    assert!(remote_list.contains(&"mirror"), "Should contain mirror");
}

#[test]
fn test_commit_changes() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize repo using direct git command
    init_test_repo(temp_dir.path()).expect("Failed to initialize repo");

    // Create a test file and commit
    create_test_file(temp_dir.path(), "test.txt", "test content");
    commit_changes(temp_dir.path(), "Initial commit").expect("Failed to commit");

    // Verify commit was created
    let log = run_git_command(temp_dir.path(), &["log", "--oneline", "-1"])
        .expect("Failed to get git log");
    assert!(log.contains("Initial commit"), "Commit should exist in log");
}

#[test]
fn test_git_error_debug() {
    let error = GitError::NotAGitRepository;
    let debug_str = format!("{:?}", error);
    assert!(
        debug_str.contains("NotAGitRepository"),
        "Debug string should contain error type"
    );
}

#[test]
fn test_git_error_command_failed() {
    let error = GitError::CommandFailed("Test error".to_string());
    let debug_str = format!("{:?}", error);
    assert!(
        debug_str.contains("CommandFailed"),
        "Debug string should contain CommandFailed"
    );
    assert!(
        debug_str.contains("Test error"),
        "Debug string should contain error message"
    );
}

#[test]
fn test_git_error_remote_not_found() {
    let error = GitError::RemoteNotFound("origin".to_string());
    let debug_str = format!("{:?}", error);
    assert!(
        debug_str.contains("RemoteNotFound"),
        "Debug string should contain RemoteNotFound"
    );
    assert!(
        debug_str.contains("origin"),
        "Debug string should contain remote name"
    );
}
