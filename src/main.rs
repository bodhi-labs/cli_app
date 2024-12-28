use clap::{Arg, Command};
use dotenv::dotenv;
use cli_app::{commands, settings};
// name of the lib module is equivalent to 
// the name of our crate: cli_app

pub fn main() -> anyhow::Result<()>{
    dotenv().ok();
    // make the command instance mutable, 
    // because the `commands::configure`` method creates a new version of it.
    let mut command = Command::new("Simple Cli App")
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .help("Configuration file location")
                    .default_value("config.json"),
            );
    
    command = commands::configure(command);

    // parses the command line 
    let matches = command.get_matches();
    
    let config_location = matches
        .get_one("config")
        .map(|s: &String| Some(s.as_str()))
        .unwrap_or(None);

    let settings = settings::Settings::new(config_location, "APP")?;

    commands::handle(&matches, &settings)?;

    
    


    Ok(())
}

// One more trick: it's usually useful to arrange the crate 
// to contain both a lib.rs and a main.rs file.
// It will contain both a library and a binary at the same time.
// This can make testing, benchmarking easier later.

/*println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing database url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    ); */