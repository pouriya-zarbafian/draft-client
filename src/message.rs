use serde::{Serialize, Deserialize};
use std::error::Error;

pub fn serialize<T: Serialize>(t: &T) -> Result<String, Box<dyn Error>> {
    match serde_json::to_string(t) {
        Ok(o) => Ok(o),
        Err(e) => Err(e)?,
    }
}

pub fn deserialize<'a, T: Deserialize<'a>>(json: &'a String) -> Result<T, Box<dyn Error>> {
    match serde_json::from_str(json) {
        Ok(o) => Ok(o),
        Err(e) => Err(e)?,
    }
}

pub const QUERY_RESULT_SUCCESS: u8 = 0x00;
pub const QUERY_RESULT_REDIRECT: u8 = 0x01;

pub const MESSAGE_TYPE_CLIENT_REQUEST: u8 = 0x04;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub term: usize,
    pub index: usize,
    pub data: Query,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRequest {
    pub client_id: String,
    pub request_id: String,
    pub query: Query,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientResponse {
    pub server_id: String,
    pub client_id: String,
    pub request_id: String,
    pub result: QueryResult,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Get,
    Save,
    Delete,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    pub action: Action,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub error: u8,
    pub message: String,
    pub value: Option<String>,
}
