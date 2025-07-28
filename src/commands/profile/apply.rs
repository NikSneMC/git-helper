use std::env;

use clap::Parser;
use git2::Config as GitConfig;

use crate::{
    commands::Command,
    config::{
        Config,
        profile::{Profile, alias::ProfileAlias},
    },
};

#[derive(Parser)]
pub struct ApplyOptions {
    /// Profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for ApplyOptions {
    fn execute(&self, config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config, false);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return;
            }
            Some(profile) => profile,
        };

        let current_dir = env::current_dir().expect("Current dir to be valid");
        let mut config = git2::Repository::open(current_dir)
            .expect("Current folder to be a valid git repository")
            .config()
            .expect("Repo config to be available");

        Self::mutate_config(&mut config, profile).expect("Config mutation to be successfull");

        println!("Profile with name `{}` was successfully applied", alias.0);
    }
}

impl ApplyOptions {
    fn mutate_config(config: &mut GitConfig, profile: &Profile) -> Result<(), git2::Error> {
        config.set_str("user.name", &profile.user.name.0)?;
        config.set_str("user.email", &profile.user.email.0)?;

        if let Some(signingkey) = &profile.user.signingkey {
            config.set_str("user.signingkey", &signingkey.0)?;
        }
        if let Some(username) = &profile.credential.username {
            config.set_str("credential.username", &username.0)?;
        }
        Ok(())
    }
}
