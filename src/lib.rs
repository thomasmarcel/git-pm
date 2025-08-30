use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prefixes: Option<Vec<String>>,
    pub default_prefix: Option<String>,
}

pub fn load_config() -> (Vec<String>, String) {
    let mut prefixes = vec![
        "feature".to_string(),
        "bugfix".to_string(),
        "hotfix".to_string(),
        "release".to_string(),
        "wip".to_string(),
        "chore".to_string(),
        "epic".to_string(),
        "experimente".to_string(),
        "docs".to_string(),
    ];
    let mut default_prefix = "feature".to_string();

    // Try repo-local .gitpm.toml first, then $HOME/.gitpm.toml
    let paths = [
        std::path::PathBuf::from(".gitpm.toml"),
        dirs::home_dir().unwrap_or_default().join(".gitpm.toml"),
    ];

    for path in paths.iter() {
        if path.exists() {
            if let Ok(contents) = std::fs::read_to_string(path) {
                if let Ok(config) = toml::from_str::<Config>(&contents) {
                    if let Some(custom_prefixes) = config.prefixes {
                        prefixes = custom_prefixes;
                    }
                    if let Some(dp) = config.default_prefix {
                        default_prefix = dp;
                    }
                    break;
                }
            }
        }
    }

    (prefixes, default_prefix)
}

pub fn sanitize_branch(input: &str) -> String {
    let mut parts = input.splitn(2, ' ');
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

pub fn branch_exists(branch: &str) -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", branch])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
