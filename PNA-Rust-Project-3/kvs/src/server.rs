use std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use log::{error, info};
use serde::Deserialize;
use serde_json::Deserializer;

use crate::{engine::KvsEngine, Request, Response};

pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<(), ()> {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_connection(stream) {
                        error!("Error on handle connection: {:?}", e);
                    }
                }
                Err(e) => error!("Connection failed:{:?}", e),
            }
        }
        Ok(())
    }

    pub fn handle_connection(&mut self, mut stream: TcpStream) -> Result<(), ()> {
        let request =
            Request::deserialize(&mut Deserializer::from_reader(BufReader::new(&mut stream)))
                .unwrap();
        info!("Request: {:?}", request);
        let response;
        match request {
            Request::SET(key, value) => match self.engine.set(key, value) {
                Ok(_) => response = Response::Ok(None),
                Err(err) => response = Response::Err(format!("{err}")),
            },
            Request::GET(key) => match self.engine.get(key) {
                Ok(value) => response = Response::Ok(value),
                Err(err) => response = Response::Err(format!("{err}")),
            },
            Request::RM(key) => match self.engine.remove(key) {
                Ok(value) => response = Response::Ok(None),
                Err(err) => response = Response::Err(format!("{err}")),
            },
        }
        info!("Response: {:?}", &response);
        serde_json::to_writer(stream, &response).unwrap();
        Ok(())
    }
}
