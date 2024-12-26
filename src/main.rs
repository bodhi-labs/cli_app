use clap::Command;
use cli_app::commands;
// name of the lib module is equivalent to 
// the name of our crate: cli_app

pub fn main() -> anyhow::Result<()>{
    // make the command instance mutable, 
    // because the `commands::configure`` method creates a new version of it.
    let mut command = Command::new("Simple Cli App");
    command = commands::configure(command);

    // parses the command line 
    let matches = command.get_matches();
    commands::handle(&matches)?;

    Ok(())
}

// One more trick: it's usually useful to arrange the crate 
// to contain both a lib.rs and a main.rs file.
// It will contain both a library and a binary at the same time.
// This can make testing, benchmarking easier later.