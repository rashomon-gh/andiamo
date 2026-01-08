use dialoguer::Input;
use std::process::Command;

pub fn check_git_installed() -> bool {
    let output = Command::new("git")
        .arg("--version")
        .output();

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

pub fn init_repo() {
    if is_git_repo() {
        println!("Git repository already exists in the current directory.");
    } else {
        let output = Command::new("git")
            .arg("init")
            .current_dir(".")
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("Git repository initialized successfully in the current directory.");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Error initializing git repository: {}", stderr);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error initializing git repository: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub fn get_existing_remotes() -> Vec<String> {
    let output = Command::new("git")
        .arg("remote")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.lines()
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

pub fn add_remotes() {
    // Check if we're in a git repo
    if !is_git_repo() {
        eprintln!("Error: Not in a git repository. Please initialize a repository first using --init.");
        std::process::exit(1);
    }

    let existing_remotes = get_existing_remotes();

    // Prompt for origin remote URL
    let origin_url: String = Input::new()
        .with_prompt("Enter the URL for the 'origin' remote")
        .interact_text()
        .expect("Failed to read origin URL");

    // Prompt for mirror remote URL
    let mirror_url: String = Input::new()
        .with_prompt("Enter the URL for the 'mirror' remote")
        .interact_text()
        .expect("Failed to read mirror URL");

    // Determine which remotes need to be added
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

    // Check if any remotes need to be added
    let remotes_count = remotes_to_add.len();

    // Add the remotes that don't exist
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
                    eprintln!("Error adding remote '{}': {}", name, stderr);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Error adding remote '{}': {}", name, e);
                std::process::exit(1);
            }
        }
    }

    if remotes_count == 0 {
        println!("Both remotes already exist. No remotes were added.");
    }
}

pub fn push_to_remotes() {
    // Check if we're in a git repo
    if !is_git_repo() {
        eprintln!("Error: Not in a git repository. Please initialize a repository first using --init.");
        std::process::exit(1);
    }

    let existing_remotes = get_existing_remotes();

    // Check if both remotes exist
    if !existing_remotes.contains(&"origin".to_string()) {
        eprintln!("Error: Remote 'origin' does not exist. Please add remotes using --add-remotes.");
        std::process::exit(1);
    }

    if !existing_remotes.contains(&"mirror".to_string()) {
        eprintln!("Error: Remote 'mirror' does not exist. Please add remotes using --add-remotes.");
        std::process::exit(1);
    }

    // Get current branch name
    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output();

    let branch_name = match branch_output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.trim().to_string()
            } else {
                eprintln!("Error: Could not determine current branch. Make sure you're on a branch.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error determining current branch: {}", e);
            std::process::exit(1);
        }
    };

    // Push to origin
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
                eprintln!("Error pushing to origin: {}", stderr);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error pushing to origin: {}", e);
            std::process::exit(1);
        }
    }

    // Push to mirror
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
                eprintln!("Error pushing to mirror: {}", stderr);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error pushing to mirror: {}", e);
            std::process::exit(1);
        }
    }

    println!("All changes pushed successfully to both remotes!");
}
