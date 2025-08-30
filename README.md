# git-pm

[![Crates.io](https://img.shields.io/crates/v/git-pm.svg)](https://crates.io/crates/git-pm)
[![Docs.rs](https://docs.rs/git-pm/badge.svg)](https://docs.rs/git-pm)
[![License](https://img.shields.io/crates/l/git-pm.svg)](https://choosealicense.com/licenses/)

A **Git CLI plugin** for project management.  
It lets you create or switch to branches from task strings, enforcing consistent names.

```bash
git pm branch "MINOI-31 Zoom d'images dans galerie"
```

Produces and switches to branch: `feature/MINOI-31-zoom-d-images-dans-galerie`.

## branch

### ✨ Features

- Branch sanitization:
- Keeps task IDs in UPPERCASE
- Lowercases the rest, replaces spaces/punctuation with -
- Handles accents and Unicode (e.g. Fixé → fixe, œuf → oeuf)
- Git-aware:
  - If branch exists → switches to it
  - If not → creates it
- Prefix support:
  - Defaults to feature/
  - Allowed: feature, bugfix, hotfix, release, wip, chore, epic
  - Overridable via .gitpm.toml
- Configurable:
  - Project-local config (.gitpm.toml in repo root)
  - Global fallback ($HOME/.gitpm.toml)

### 🛠 Usage

```bash
git pm branch "<TASK-ID> <description>" [--prefix <prefix>]
```

Examples:

```bash
git pm branch "MINOI-31 Zoom d'images dans galerie"
# => feature/MINOI-31-zoom-d-images-dans-galerie

git pm branch "BUG-42 Fix login bug" --prefix bugfix
# => bugfix/BUG-42-fix-login-bug

git pm branch "HOT-77 Production issue" -p hotfix
# => hotfix/HOT-77-production-issue

git pm branch "TMP-99 Experiment" -p wip
# => wip/TMP-99-experiment
```

## 🚀 Installation

Make sure you have Rust installed, then

```bash
cargo install git-pm
``` 

This provides git pm as a Git subcommand.