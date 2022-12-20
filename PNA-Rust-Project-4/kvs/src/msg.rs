use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Debug)]
pub enum Request {
    SET(String, String),
    RM(String),
    GET(String),
}

#[derive(Serialize, Deserialize,Debug)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}
