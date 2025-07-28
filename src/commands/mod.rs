use clap::Subcommand;

use crate::{
    commands::{clone::CloneOptions, profile::ProfileSubCommands},
    config::Config,
};

pub mod clone;
pub mod profile;

pub trait Command {
    fn execute(&self, config: Config);
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Profile(ProfileSubCommands),
    #[command(name = "clone", about = "Clone a repository to the current directory.")]
    Clone(CloneOptions),
}

impl Command for Commands {
    fn execute(&self, config: Config) {
        match self {
            Self::Profile(profile_command) => profile_command.execute(config),
            Self::Clone(clone_command) => clone_command.execute(config),
        };
    }
}
