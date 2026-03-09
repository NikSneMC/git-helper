use anyhow::Context as _;
use clap::Parser;

use crate::{
    commands::{Command, CommandResult},
    config::{Config, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct ApplyOptions {
    /// Profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for ApplyOptions {
    fn execute(&self, config: Config) -> CommandResult {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return Ok(());
            }
            Some(profile) => profile,
        };

        profile.apply().context("while applying profile")?;

        println!("Profile with name `{}` was successfully applied", alias.0);

        Ok(())
    }
}

impl ApplyOptions {}
