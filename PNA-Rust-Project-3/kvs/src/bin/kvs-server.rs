use clap::{arg, command};
use kvs::{KvStore, Result};
use log::{info, warn, LevelFilter};

fn main() -> Result<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let matches = command!()
        .name("kvs-server")
        .arg(
            arg!(--addr <IPPORT>)
                .required(false)
                .default_value("127.0.0.1:4000"),
        )
        .arg(
            arg!(--engine <ENGINENAME>)
                .required(false)
                .value_parser(["kvs", "sled"]),
        )
        .get_matches();

    let addr = matches.get_one::<String>("addr").unwrap();
    let engine = matches.get_one::<String>("engine").unwrap();
    // choose an engine

    Ok(())
}