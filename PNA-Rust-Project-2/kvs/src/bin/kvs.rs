#[macro_use]
extern crate clap;
use clap::App;
use kvs::{KvStore, Result};
use std::{env::current_dir, process::exit};

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();
    if let Some((cmd, args)) = m.subcommand() {
        match cmd {
            "set" => {
                let key = args.value_of("KEY").unwrap();
                let value = args.value_of("VALUE").unwrap();
                let mut store = KvStore::open(current_dir()?)?;
                store.set(key.to_string(), value.to_string())?;
            }
            "get" => {
                let key = args.value_of("KEY").unwrap();
                let mut store = KvStore::open(current_dir()?)?;
                if let Some(res) = store.get(key.to_string())? {
                    println!("{res}");
                } else {
                    println!("Key not found");
                }
            }
            "rm" => {
                let key = args.value_of("KEY").unwrap();
                let mut store = KvStore::open(current_dir()?)?;
                match store.remove(key.to_string()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Key not found");
                        return Err(e);
                    }
                }
            }
            _ => {
                eprintln!("unimplemented");
                exit(1);
            }
        }
    } else {
        exit(1);
    }
    Ok(())
}
