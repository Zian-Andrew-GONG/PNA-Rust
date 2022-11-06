use clap::{arg, command, SubCommand};
use env_logger;
use kvs::{KvStore, Result};
use log::{info, warn, LevelFilter};
use std::{env::current_dir, process::exit};

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let matches = command!()
        .name("kvs-client")
        .subcommand(
            SubCommand::with_name("set")
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
            SubCommand::with_name("get")
                .about("Get key-value")
                .arg(arg!(<KEY>))
                .arg(
                    arg!(--addr <IPPORT>)
                        .required(false)
                        .default_value("127.0.0.1:4000"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
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
            }
            "get" => {
                let key = args.get_one::<String>("KEY").unwrap();
                let addr = args.get_one::<String>("addr").unwrap();
                // send request to server
            }
            "rm" => {
                let key = args.get_one::<String>("KEY").unwrap();
                let addr = args.get_one::<String>("addr").unwrap();
                // send request to server
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
