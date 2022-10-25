use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;
#[derive(Deserialize, Serialize, Debug)]
struct Move {
    x: i32,
    y: i32,
}

fn main() {
    let path = String::from("json.yaml");
    let mv = Move { x: 1, y: 2 };
    println!("{:?}", mv);
    let serialized = serde_json::to_string(&mv).unwrap();
    println!("{:?}", serialized);
    if let Ok(()) = write_file(&serialized, &path) {
        if let Ok(content) = read_file(&path) {
            let de: Move = serde_json::from_str(&content).unwrap();
            println!("{:?}", de);
        } else {
            panic!("Read error!");
        }
    } else {
        panic!("Write error!")
    }
}

fn write_file(content: &String, path: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_file(path: &String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
