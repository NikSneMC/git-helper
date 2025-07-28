use clap::Parser;

use crate::{commands::Command, config::Config};

#[derive(Parser)]
pub struct ListOptions {}

impl Command for ListOptions {
    fn execute(&self, config: Config) {
        let profiles = config
            .profiles
            .iter()
            .map(|(name, profile)| [name.to_string(), profile.to_string()].join("\n"))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{profiles}");
    }
}
