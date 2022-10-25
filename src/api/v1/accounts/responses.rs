use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountResponse {
    Success { username: String, id: String },
    Error { error: String },
}
