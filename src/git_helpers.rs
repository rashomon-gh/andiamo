use std::io;
use std::process::Command;

#[derive(Debug)]
pub enum GitError {
    CommandFailed(String),
    IoError(io::Error),
    NotAGitRepository,
    RemoteNotFound(String),
    RemoteAlreadyExists(String),
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> Self {
        GitError::IoError(err)
    }
}

pub type GitResult<T> = Result<T, GitError>;

pub fn check_git_installed() -> bool {
    let output = Command::new("git").arg("--version").output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn is_git_repo() -> bool {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim() == "true"
        }
        Err(_) => false,
    }
}

pub fn init_repo() -> GitResult<()> {
    if is_git_repo() {
        println!("Git repository already exists in the current directory.");
        Ok(())
    } else {
        let output = Command::new("git").arg("init").current_dir(".").output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Git repository initialized successfully in the current directory.");
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(GitError::CommandFailed(format!(
                        "Error initializing git repository: {}",
                        stderr
                    )))
                }
            }
            Err(e) => Err(GitError::CommandFailed(format!(
                "Error initializing git repository: {}",
                e
            ))),
        }
    }
}

pub fn get_existing_remotes() -> Vec<String> {
    let output = Command::new("git").arg("remote").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect()
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}

pub fn add_remotes_with_urls(
    origin_url: Option<String>,
    mirror_url: Option<String>,
) -> GitResult<()> {
    if !is_git_repo() {
        return Err(GitError::NotAGitRepository);
    }

    let existing_remotes = get_existing_remotes();

    let origin_url = match origin_url {
        Some(url) => url,
        None => {
            return Err(GitError::CommandFailed(
                "origin_url is required".to_string(),
            ));
        }
    };

    let mirror_url = match mirror_url {
        Some(url) => url,
        None => {
            return Err(GitError::CommandFailed(
                "mirror_url is required".to_string(),
            ));
        }
    };

    let mut remotes_to_add = Vec::new();

    if existing_remotes.contains(&"origin".to_string()) {
        println!("Remote 'origin' already exists. Skipping...");
    } else {
        remotes_to_add.push(("origin", &origin_url));
    }

    if existing_remotes.contains(&"mirror".to_string()) {
        println!("Remote 'mirror' already exists. Skipping...");
    } else {
        remotes_to_add.push(("mirror", &mirror_url));
    }

    let remotes_count = remotes_to_add.len();

    for (name, url) in remotes_to_add {
        let output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg(name)
            .arg(url)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Remote '{}' added successfully.", name);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(GitError::CommandFailed(format!(
                        "Error adding remote '{}': {}",
                        name, stderr
                    )));
                }
            }
            Err(e) => {
                return Err(GitError::CommandFailed(format!(
                    "Error adding remote '{}': {}",
                    name, e
                )));
            }
        }
    }

    if remotes_count == 0 {
        println!("Both remotes already exist. No remotes were added.");
    }

    Ok(())
}

pub fn push_to_remotes() -> GitResult<()> {
    if !is_git_repo() {
        return Err(GitError::NotAGitRepository);
    }

    let existing_remotes = get_existing_remotes();

    if !existing_remotes.contains(&"origin".to_string()) {
        return Err(GitError::RemoteNotFound("origin".to_string()));
    }

    if !existing_remotes.contains(&"mirror".to_string()) {
        return Err(GitError::RemoteNotFound("mirror".to_string()));
    }

    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output();

    let branch_name = match branch_output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.trim().to_string()
            } else {
                return Err(GitError::CommandFailed(
                    "Could not determine current branch. Make sure you're on a branch.".to_string(),
                ));
            }
        }
        Err(e) => {
            return Err(GitError::CommandFailed(format!(
                "Error determining current branch: {}",
                e
            )));
        }
    };

    println!("Pushing to origin...");
    let output = Command::new("git")
        .args(["push", "origin", &branch_name])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Successfully pushed to origin.");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(GitError::CommandFailed(format!(
                    "Error pushing to origin: {}",
                    stderr
                )));
            }
        }
        Err(e) => {
            return Err(GitError::CommandFailed(format!(
                "Error pushing to origin: {}",
                e
            )));
        }
    }

    println!("Pushing to mirror...");
    let output = Command::new("git")
        .args(["push", "mirror", &branch_name])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Successfully pushed to mirror.");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(GitError::CommandFailed(format!(
                    "Error pushing to mirror: {}",
                    stderr
                )));
            }
        }
        Err(e) => {
            return Err(GitError::CommandFailed(format!(
                "Error pushing to mirror: {}",
                e
            )));
        }
    }

    println!("All changes pushed successfully to both remotes!");
    Ok(())
}
