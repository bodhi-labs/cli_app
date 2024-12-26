use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "welcome";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Welcome to 🦀 Cli!")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("Welcome to 🦀 Cli!");

    Ok(())
}

