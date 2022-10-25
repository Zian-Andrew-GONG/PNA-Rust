#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();
    if let Some((cmd, _args)) = m.subcommand() {
        match cmd {
            "set" => {
                eprintln!("unimplemented");
                exit(1);
            }
            "get" => {
                eprintln!("unimplemented");
                exit(1);
            }
            "rm" => {
                eprintln!("unimplemented");
                exit(1);
            }
            _ => {
                eprintln!("unimplemented");
                exit(1);
            }
        }
    }
}
