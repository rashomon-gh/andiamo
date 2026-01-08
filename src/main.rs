mod cli;
mod git_helpers;

use clap::Parser;

use cli::Cli;
use dialoguer::Input;
use git_helpers::{
    add_remotes_with_urls, check_git_installed, init_repo, push_to_remotes,
};

fn main() {
    let cli = Cli::parse();

    // Check if git is installed
    if !check_git_installed() {
        eprintln!("Error: Git is not installed on your system.");
        eprintln!("Please install Git to use andiamo.");
        std::process::exit(1);
    }

    // Execute commands in order
    if cli.init {
        if let Err(e) = init_repo() {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }

    if cli.add_remotes {
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

        if let Err(e) = add_remotes_with_urls(Some(origin_url), Some(mirror_url)) {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }

    if cli.push {
        if let Err(e) = push_to_remotes() {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
