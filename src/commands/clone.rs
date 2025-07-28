use std::env;

use clap::Parser;
use git2::{Cred, FetchOptions, RemoteCallbacks, build::RepoBuilder};

use crate::{
    commands::Command,
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
    fn execute(&self, config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config, false);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return;
            }
            Some(profile) => profile,
        };

        let mut cb = RemoteCallbacks::new();
        if let Some(username) = &profile.credential.username {
            cb.credentials(|url, _username_from_url, _allowed_types| {
                let global_conf =
                    git2::Config::open_default().expect("Global config to open successfully");
                Cred::credential_helper(&global_conf, url, Some(&username.0))
            });
        }
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(cb);
        let mut rb = RepoBuilder::new();
        rb.fetch_options(fo);

        let url = CloneUrl::from_param(self.url.clone()).0;
        let current_dir = env::current_dir().expect("Current dir to be valid");
        let repo_path = current_dir.join(url.split("/").last().unwrap());

        rb.clone(&url, &repo_path)
            .expect("Repo to be cloned successfully");
    }
}
