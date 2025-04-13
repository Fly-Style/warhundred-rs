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
    nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutPlayerRequest {
    nickname: String,
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutPlayerResponse {
    ok: bool,
}
