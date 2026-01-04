use clap::Parser;

use crate::{
    commands::Command,
    config::{Config, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct RemoveOptions {
    /// Profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for RemoveOptions {
    fn execute(&self, mut config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        if config.profiles.remove_entry(&alias).is_none() {
            println!("Profile with name `{}` does not exist", alias.0);
            return;
        }
        config.save().unwrap();
        println!("Profile with name `{}` was successfully removed", alias.0);
    }
}
