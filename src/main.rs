use clap::Parser;

mod commands;
mod config;

use crate::{
    commands::{Command as _, Commands},
    config::Config,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();
    let config = Config::load().expect("failed to load config file");
    args.command.execute(config);
}
