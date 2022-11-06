use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use serde::Deserializer;
use serde_json::de::IoRead;

struct KvsClient {
    // reader: dyn Deserializer<'a, IoRead<BufReader<TcpStream>>>,
    // writer: BufReader<TcpStream>,
}

impl KvsClient {
    // new 

    // request
}
