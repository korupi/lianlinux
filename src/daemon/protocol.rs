use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub mode: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}
