use std::{env::current_dir, fs};

use anyhow::Context as _;
use clap::Parser;
use git2::{Repository, WorktreeAddOptions, build::RepoBuilder};

use crate::{
    commands::{Command, CommandResult},
    config::{Config, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct MigrateOptions {
    /// Git user name on the remote server
    #[arg(short, long)]
    pub git_user: Option<String>,
    /// Profile name (alias) to use
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for MigrateOptions {
    fn execute(&self, config: Config) -> CommandResult {
        let current_dir = current_dir().context("while getting the current directory")?;
        let repo = Repository::open(&current_dir)
            .context("while opening a git repository in the current directory")?;

        if let Ok(remote) = repo.find_remote("origin")
            && let Some(mut remote_url) = remote.url()
            && remote_url.starts_with("http")
        {
            if let Some(stripped) = remote_url.strip_prefix("https://") {
                remote_url = stripped;
            }

            let (host, path) = remote_url
                .split_once("/")
                .context("while splitting remote url")?;

            let git_user = self.git_user.clone().unwrap_or_else(|| "git".to_string());

            let remote_url = format!("{git_user}@{host}:{path}.git");
            repo.remote_set_url("origin", &remote_url)
                .context("while setting remote url")?;
        }

        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return Ok(());
            }
            Some(profile) => profile,
        };

        profile.apply().context("while applying profile")?;

        if !repo.is_bare() && !repo.is_worktree() {
            let entries: Vec<_> = current_dir
                .read_dir()
                .context("while reading current directory")?
                .filter_map(|entry| Some(entry.ok()?.path()))
                .collect();

            if entries.contains(&current_dir.join(".git")) {
                let tmp_dir = current_dir
                    .parent()
                    .context("while getting parent directory")?
                    .join("git_helper_tmp_migration_dir");
                fs::rename(&current_dir, &tmp_dir)
                    .context("while moving current branch contents to the tmp directory")?;

                let mut rb = RepoBuilder::new();
                rb.bare(true);

                rb.clone(
                    tmp_dir
                        .to_str()
                        .context("while getting str from the tmp dir path")?,
                    &current_dir.join(".git"),
                )
                .context("while cloning local repository")?;

                let head = repo.head().context("while getting repository head")?;
                let worktree_name = head.shorthand().context("while getting head shorthand")?;

                let new_repo = Repository::open(&current_dir)
                    .context("while opening git repository in the current directory")?;

                let mut opts = WorktreeAddOptions::new();
                opts.checkout_existing(true);
                let worktree_path = current_dir.join(worktree_name);
                new_repo
                    .worktree(worktree_name, &worktree_path, Some(&opts))
                    .context("while creating a new git worktree")?;

                let worktree_git = worktree_path.join(".git");
                let tmp_git = current_dir.join(".git.tmp");

                fs::rename(&worktree_git, &tmp_git)
                    .context("while moving git from the worktree to the tmp path")?;

                fs::remove_dir_all(&worktree_path).context("while deleting worktree path")?;

                fs::rename(tmp_dir, &worktree_path)
                    .context("while moving branch contents from the tmp directory")?;

                fs::remove_dir_all(&worktree_git)
                    .context("while removing .git from the current branch")?;

                fs::rename(&tmp_git, &worktree_git)
                    .context("while moving git from the tmp to the worktree dir")?;
            }
        }

        Ok(())
    }
}
