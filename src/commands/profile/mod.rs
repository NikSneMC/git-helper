use clap::Subcommand;

use crate::{
    commands::{
        Command,
        profile::{
            apply::ApplyOptions, list::ListOptions, remove::RemoveOptions, upsert::UpsertOptions,
        },
    },
    config::Config,
};

pub mod apply;
pub mod list;
pub mod remove;
pub mod upsert;

#[derive(Subcommand)]
pub enum ProfileSubCommands {
    #[command(name = "upsert", alias = "u", about = "Upsert a git profile")]
    Upsert(UpsertOptions),
    #[command(name = "list", alias = "ls", about = "List git profiles")]
    List(ListOptions),
    #[command(name = "remove", alias = "rm", about = "Remove a git profile")]
    Remove(RemoveOptions),
    #[command(
        name = "apply",
        alias = "a",
        about = "Apply selected profile in the current git repository"
    )]
    Apply(ApplyOptions),
}

impl Command for ProfileSubCommands {
    fn execute(&self, config: Config) {
        match self {
            Self::Upsert(upsert_command) => upsert_command.execute(config),
            Self::List(list_command) => list_command.execute(config),
            Self::Remove(remove_command) => remove_command.execute(config),
            Self::Apply(apply_command) => apply_command.execute(config),
        }
    }
}
