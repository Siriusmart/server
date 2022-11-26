use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountResponse {
    Success {
        username: String,
        id: String,
        email: String,
    },
    Error {
        error: String,
    },
}
