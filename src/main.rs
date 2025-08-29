use clap::{Parser, Subcommand};
use std::process::Command;

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
    },
}

fn sanitize_branch(input: &str) -> String {
    let mut parts = input.splitn(2, ' '); // split into "MINOI-31" and the rest
    let task_id = parts.next().unwrap_or("").to_string();
    let rest = parts.next().unwrap_or("");

    let mut branch_rest = rest
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();

    while branch_rest.contains("--") {
        branch_rest = branch_rest.replace("--", "-");
    }

    branch_rest = branch_rest.trim_matches('-').to_string();

    if branch_rest.is_empty() {
        task_id
    } else {
        format!("{}-{}", task_id, branch_rest)
    }
}

fn branch_exists(branch: &str) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", branch])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Branch { name } => {
            let branch_name = sanitize_branch(&name);

            println!("→ Using branch: {}", branch_name);

            if branch_exists(&branch_name) {
                println!("Branch exists, switching...");
                let status = Command::new("git")
                    .args(["checkout", &branch_name])
                    .status()
                    .expect("failed to run git");
                if !status.success() {
                    eprintln!("Failed to switch to branch {}", branch_name);
                }
            } else {
                println!("Branch does not exist, creating...");
                let status = Command::new("git")
                    .args(["checkout", "-b", &branch_name])
                    .status()
                    .expect("failed to run git");
                if !status.success() {
                    eprintln!("Failed to create branch {}", branch_name);
                }
            }
        }
    }
}
