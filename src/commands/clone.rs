use std::{env, process, time::Duration};

use anyhow::Context as _;
use clap::Parser;
use git2::{Repository, WorktreeAddOptions};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    commands::{Command, CommandResult},
    config::{Config, clone_url::CloneUrl, profile::alias::ProfileAlias},
    repo,
};

#[derive(Parser)]
pub struct CloneOptions {
    #[arg(short, long)]
    pub url: Option<String>,

    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for CloneOptions {
    fn execute(&self, config: Config) -> CommandResult {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return Ok(());
            }
            Some(profile) => profile,
        };

        let git_url = CloneUrl::from_param(self.url.clone()).0;
        let mut url = git_url
            .split("/")
            .last()
            .context("while splitting git url")?;
        if let Some(stripped) = url.strip_suffix(".git") {
            url = stripped;
        }

        let current_dir = env::current_dir().context("while getting current directory")?;
        let repo_path = current_dir.join(url);
        let repo_path_str = repo_path.to_string_lossy().to_string();

        let spinner = ProgressBar::new_spinner()
            .with_message(format!("Cloning into {repo_path_str}"))
            .with_style(ProgressStyle::default_spinner().tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷ "));
        spinner.enable_steady_tick(Duration::from_millis(100));

        repo::clone(profile, &git_url, &repo_path, true)
            .context("while cloning repository")?;
        profile
            .apply_at(&repo_path)
            .context("while applying profile")?;

        let repo = Repository::open(&repo_path)
            .context("while opening git repository in the current directory")?;
        let origin = repo
            .find_reference("refs/remotes/origin/HEAD")
            .context("while finding origin reference")?
            .resolve()
            .context("while resolving origin")?;

        let worktree_name = origin
            .shorthand()
            .context("while getting origin shorthand")?
            .strip_prefix("origin/")
            .context("while removing origin prefix")?;

        let mut opts = WorktreeAddOptions::new();
        opts.checkout_existing(true);
        let worktree_path = repo_path.join(worktree_name);
        repo.worktree(worktree_name, &worktree_path, Some(&opts))
            .context("while creating a new git worktree")?;

        let _ = process::Command::new("zoxide")
            .arg("add")
            .arg(&repo_path_str)
            .output();

        spinner.finish_with_message(format!(
            "Repository was cloned to `{repo_path_str}` successfully"
        ));

        Ok(())
    }
}
