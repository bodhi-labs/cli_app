mod welcome;
mod serve;

use crate::settings::Settings;
use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command{
    command
        .subcommand(welcome::configure())
        .subcommand(serve::configure())
        .arg_required_else_help(true)
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some((cmd, _matches)) = matches.subcommand(){
        match cmd{
            welcome::COMMAND_NAME => welcome::handle(matches)?,
            serve::COMMAND_NAME => serve::handle(matches)?,
            &_ => {}
        }
    }
    Ok(())
}