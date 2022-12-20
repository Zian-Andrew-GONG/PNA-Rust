use std::{env::current_dir, process::exit};

use clap::{arg, command};
use kvs::{KvStore, KvsServer, Result, SledKvsEngine};
use log::{LevelFilter};

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
    if engine == "sled" {
        let path = current_dir()?.join("engine");
        if path.exists() && !path.join("sled").exists() {
            exit(1);
        }
        let server = KvsServer::new(SledKvsEngine::new(sled::open(path.join("sled"))?));
        server.run(addr)?;
    } else {
        let path = current_dir()?.join("engine");
        if path.exists() && !path.join("kvs").exists() {
            exit(1);
        }
        let server = KvsServer::new(KvStore::open(path.join("kvs"))?);
        server.run(addr)?;
    }

    Ok(())
}
