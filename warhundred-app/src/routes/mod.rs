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
    id: i64,
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
    nickname: String,
    logged_in: bool,
}
