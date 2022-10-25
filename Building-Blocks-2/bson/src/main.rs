use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    x: i32,
    y: i32,
}

fn main() {
    let mv = Move { x: 1, y: 1 };
    println!("{:?}", mv);
    let se = bson::to_bson(&mv).unwrap();
    println!("{:?}", se);
    let de: Move = bson::from_bson(se).unwrap();
    println!("{:?}", de);
}
