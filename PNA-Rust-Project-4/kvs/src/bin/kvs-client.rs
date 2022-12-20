use clap::{arg, command, Command};
use env_logger;
use kvs::{KvsClient, Request, Result};
use log::{warn, LevelFilter};
use std::{process::exit};
fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let matches = command!()
        .name("kvs-client")
        .subcommand(
            Command::new("set")
                .about("Set key-value")
                .arg(arg!(<KEY>))
                .arg(arg!(<VALUE>))
                .arg(
                    arg!(--addr <IPPORT>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get key-value")
                .arg(arg!(<KEY>))
                .arg(
                    arg!(--addr <IPPORT>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("remove key-value")
                .arg(arg!(<KEY>))
                .arg(
                    arg!(--addr <IPPORT>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .get_matches();

    if let Some((cmd, args)) = matches.subcommand() {
        match cmd {
            "set" => {
                let key = args.get_one::<String>("KEY").unwrap();
                let value = args.get_one::<String>("VALUE").unwrap();
                let addr = args.get_one::<String>("addr").unwrap();
                // send request to server
                let mut client = KvsClient::new(addr)?;
                let request = Request::SET(key.to_owned(), value.to_owned());
                client.request(&request)?;
            }
            "get" => {
                let key = args.get_one::<String>("KEY").unwrap();
                let addr = args.get_one::<String>("addr").unwrap();
                // send request to server
                let mut client = KvsClient::new(addr)?;
                let request = Request::GET(key.to_owned());
                if let Some(res) = client.request(&request)? {
                    println!("{}", res);
                } else {
                    println!("Key not found");
                }
            }
            "rm" => {
                let key = args.get_one::<String>("KEY").unwrap();
                let addr = args.get_one::<String>("addr").unwrap();
                // send request to server
                let mut client = KvsClient::new(addr)?;
                let request = Request::RM(key.to_owned());
                client.request(&request)?;
            }
            _ => {
                warn!("unimplemented");
                exit(1);
            }
        }
    } else {
        exit(1);
    }
    Ok(())
}
