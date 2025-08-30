use clap::{Parser, Subcommand};
use std::process::Command;

use git_pm::{branch_exists, load_config, sanitize_branch};

#[derive(Parser)]
#[command(name = "git-pm")]
#[command(about = "Project management helper for git", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create or switch to a branch
    Branch {
        /// Task string (e.g. "MINOI-31 Zoom d'images dans galerie")
        name: String,

        /// Branch prefix (feature/, bugfix/, hotfix/, release/, wip/, chore/, epic/, experiment/, docs/)
        #[arg(short, long)]
        prefix: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Branch { name, prefix } => {
            let (allowed_prefixes, default_prefix) = load_config();

            let chosen_prefix = prefix.unwrap_or(default_prefix);

            if !allowed_prefixes.contains(&chosen_prefix) {
                eprintln!(
                    "Invalid prefix '{}'. Allowed: {}",
                    chosen_prefix,
                    allowed_prefixes.join(", ")
                );
                std::process::exit(1);
            }

            let branch_name = sanitize_branch(&name);
            let branch_full = format!("{}/{}", chosen_prefix, branch_name);

            println!("→ Using branch: {}", branch_full);

            if branch_exists(&branch_full) {
                println!("Branch exists, switching...");
                let status = Command::new("git")
                    .args(["checkout", &branch_full])
                    .status()
                    .expect("failed to run git");
                if !status.success() {
                    eprintln!("Failed to switch to branch {}", branch_full);
                }
            } else {
                println!("Branch does not exist, creating...");
                let status = Command::new("git")
                    .args(["checkout", "-b", &branch_full])
                    .status()
                    .expect("failed to run git");
                if !status.success() {
                    eprintln!("Failed to create branch {}", branch_full);
                }
            }
        }
    }
}
