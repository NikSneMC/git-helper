use clap::Parser;

use crate::{
    commands::Command,
    config::{Config, profile::alias::ProfileAlias},
};

#[derive(Parser)]
pub struct ApplyOptions {
    /// Profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,
}

impl Command for ApplyOptions {
    fn execute(&self, config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);
        let profile = match config.profiles.get(&alias) {
            None => {
                println!("Profile with name `{}` does not exist", alias.0);
                return;
            }
            Some(profile) => profile,
        };

        profile.apply().expect("Config mutation to be successfull");

        println!("Profile with name `{}` was successfully applied", alias.0);
    }
}

impl ApplyOptions {}
