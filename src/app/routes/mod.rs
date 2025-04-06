use serde::{Deserialize, Serialize};

pub mod root_routes;

#[derive(Debug, Deserialize)]
pub struct RegisterPlayerRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPlayerResponse {
    nickname: String,
    registered: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginPlayerRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPlayerResponse {
    access_token: String,
    token_type: String,
}
