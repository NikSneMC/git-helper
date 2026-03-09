use anyhow::Context;
use clap::Parser;

use crate::{
    commands::{Command, CommandResult},
    config::{Config, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct RemoveOptions {
    /// Profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for RemoveOptions {
    fn execute(&self, mut config: Config) -> CommandResult {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        if config.profiles.remove_entry(&alias).is_none() {
            println!("Profile with name `{}` does not exist", alias.0);
            return Ok(());
        }
        config.save().context("while saving config")?;
        println!("Profile with name `{}` was successfully removed", alias.0);

        Ok(())
    }
}
