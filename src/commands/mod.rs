use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command{
    command
        .subcommand(Command::new("welcome")
        .about("Welcome to 🦀 Cli!"))
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some((cmd, _matches)) = matches.subcommand(){
        match cmd{
            "welcome" => { println!("Welcome to 🦀 Cli!");},
            &_ => {}

        }
    }
    Ok(())
}