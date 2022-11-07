use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}
use crate::{Request, Response};

impl KvsClient {
    // new
    pub fn new(addr: &str) -> Result<Self, ()> {
        let stream = TcpStream::connect(addr).expect("Tcp connect error!");
        let reader = Deserializer::from_reader(BufReader::new(stream.try_clone().unwrap()));
        let writer = BufWriter::new(stream);
        Ok(KvsClient { reader, writer })
    }
    // request
    pub fn request(&mut self, request: &Request) -> Result<Option<String>, ()> {
        serde_json::to_writer(&mut self.writer, request).unwrap();
        self.writer.flush().unwrap();
        let response = Response::deserialize(&mut self.reader).unwrap();
        match response {
            Response::Ok(value) => Ok(value),
            Response::Err(msg) => Err(()),
        }
    }
}
