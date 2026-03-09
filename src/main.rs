use anyhow::Context;
use clap::Parser;

mod commands;
mod config;

use crate::{
    commands::{Command as _, CommandResult, Commands},
    config::Config,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> CommandResult {
    let args = Cli::parse();
    let config = Config::load().context("while loading the config file")?;

    args.command.execute(config)
}
