#[macro_use]
extern crate clap;
use clap::App;
use std::env;
#[macro_use]
extern crate dotenv_codegen;
fn main() {
    println!("PORT: {}", dotenv!("PORT"));

    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    }

    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();
    if let Some(config_val) = m.value_of("config") {
        match config_val {
            "c1" => println!("config 1111"),
            "c2" => println!("config 2222"),
            "c3" => println!("config 3333"),
            _ => println!("what did you config?"),
        }
    } else {
        println!("--config is not assigned");
    }
    if let Some(input_val) = m.value_of("INPUT") {
        println!("{:?}", input_val);
    } else {
        panic!("INPUT is not assigned");
    }
}
