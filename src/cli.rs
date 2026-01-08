use clap::Parser;

#[derive(Parser)]
#[command(name = "andiamo")]
#[command(about = "A CLI tool for managing git repositories with dual remotes", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub init: bool,

    #[arg(long)]
    pub add_remotes: bool,

    #[arg(long)]
    pub push: bool,
}
