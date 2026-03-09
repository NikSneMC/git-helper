use clap::Subcommand;

use crate::{
    commands::{clone::CloneOptions, migrate::MigrateOptions, profile::ProfileSubCommands},
    config::Config,
};

pub mod clone;
pub mod migrate;
pub mod profile;

pub type CommandResult = anyhow::Result<()>;

pub trait Command {
    fn execute(&self, config: Config) -> CommandResult;
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Profile(ProfileSubCommands),
    #[command(name = "clone", about = "Clone a repository to the current directory.")]
    Clone(CloneOptions),
    #[command(
        name = "migrate",
        about = "Migrate a repository in the current directory."
    )]
    Migrate(MigrateOptions),
}

impl Command for Commands {
    fn execute(&self, config: Config) -> CommandResult {
        match self {
            Self::Profile(profile_command) => profile_command.execute(config),
            Self::Clone(clone_command) => clone_command.execute(config),
            Self::Migrate(migrate_command) => migrate_command.execute(config),
        }
    }
}
