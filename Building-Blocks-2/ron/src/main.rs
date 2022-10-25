use serde::{Deserialize, Serialize};
use std::str;
#[derive(Debug, Deserialize, Serialize)]
struct Move {
    x: i32,
    y: i32,
}

fn main() {
    let mv = Move { x: 1, y: 2 };
    println!("{:?}", mv);
    let ron = ron::to_string(&mv).unwrap();
    let vec = ron.as_bytes().to_vec();
    println!("{:?}", vec);
    let string = str::from_utf8(&vec).unwrap();
    let de: Move = ron::from_str(&string).unwrap();
    println!("{:?}", de);
}
