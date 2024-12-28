use clap::{value_parser, Arg, ArgMatches, Command};
use crate::settings::Settings;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command{
    Command::new(COMMAND_NAME)
        .about("Start HTTP server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("TCP port to listen on")
                .default_value("8080")
                .value_parser(value_parser!(u16)),
        )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    let port: u16 = *matches.get_one("port").unwrap_or(&8080);

    //println!("TBD: start the webserver on port {}", port);
    start_tokio(port, settings)?;

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {

            let addr = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 
                port
            );
            let router = Router::new();
    
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router.into_make_service()).await?;
            
            Ok::<(), anyhow::Error>(())
        })?;
        
    Ok(())
}