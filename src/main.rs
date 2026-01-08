mod cli;
mod git_helpers;

use clap::Parser;

use cli::Cli;
use git_helpers::{check_git_installed, init_repo, add_remotes, push_to_remotes};

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
        init_repo();
    }

    if cli.add_remotes {
        add_remotes();
    }

    if cli.push {
        push_to_remotes();
    }
}

