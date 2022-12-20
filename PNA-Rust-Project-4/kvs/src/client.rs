use crate::{KvStoreError, Result};
use log::info;
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}
use crate::{Request, Response};

impl KvsClient {
    // new
    pub fn new(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).expect("Tcp connect error!");
        let reader = Deserializer::from_reader(BufReader::new(stream.try_clone()?));
        let writer = BufWriter::new(stream);
        Ok(KvsClient { reader, writer })
    }
    // request
    pub fn request(&mut self, request: &Request) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, request)?;
        self.writer.flush()?;
        let response = Response::deserialize(&mut self.reader)?;
        info!("response: {:?}", response);
        match response {
            Response::Ok(value) => Ok(value),
            Response::Err(msg) => Err(KvStoreError::StringError(msg)),
        }
    }
}
