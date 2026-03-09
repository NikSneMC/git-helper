use std::{env, path::Path, time::Duration};

use anyhow::Context as _;
use clap::Parser;
use git2::{Cred, FetchOptions, RemoteCallbacks, build::RepoBuilder};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    commands::{Command, CommandResult},
    config::{Config, clone_url::CloneUrl, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct CloneOptions {
    /// Repository url to clone
    #[arg(short, long)]
    pub url: Option<String>,

    /// Profile name (alias) to use
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

        let mut cb = RemoteCallbacks::new();
        cb.credentials(|_url, username_from_url, _allowed_types| {
            let keypath = profile
                .keys
                .auth
                .0
                .replacen("~", &env::var("HOME").unwrap(), 1);
            Cred::ssh_key(username_from_url.unwrap(), None, Path::new(&keypath), None)
        });
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        let mut rb = RepoBuilder::new();
        rb.bare(true);
        rb.fetch_options(fo);

        let git_url = CloneUrl::from_param(self.url.clone()).0;
        let (_, url) = git_url.split_once("@").context("while splitting git url")?;
        let (host, mut url) = url.split_once(":").context("while splitting url")?;
        if let Some(stripped) = url.strip_suffix(".git") {
            url = stripped;
        }

        let current_dir = env::current_dir().context("while getting current directory")?;
        let repo_path = config.base_dir.join(host).join(url).join(".git");
        let repo_path_str = repo_path.to_string_lossy();
        let repo_path_str = repo_path_str
            .strip_suffix("/.git")
            .context("while stripping suffix from repo path string")?;

        let spinner = ProgressBar::new_spinner()
            .with_message(format!("Cloning into {repo_path_str}"))
            .with_style(ProgressStyle::default_spinner().tick_chars("⣾⣽⣻⢿⡿⣟⣯⣷ "));
        spinner.enable_steady_tick(Duration::from_millis(100));

        rb.clone(&git_url, &repo_path)
            .context("while cloning repository")?;

        env::set_current_dir(&repo_path).context("while changing the current directory")?;
        profile.apply().context("while applying profile")?;
        env::set_current_dir(&current_dir).context("while changing the current directory")?;

        spinner.finish_with_message(format!(
            "Repository was cloned to `{repo_path_str}` successfully"
        ));

        Ok(())
    }
}
