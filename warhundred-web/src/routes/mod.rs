use serde::{Deserialize, Serialize};

pub mod player_routes;

#[derive(Debug, Deserialize)]
pub struct RegisterPlayerRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPlayerResponse {
    id: i32,
    nickname: String,
    registered: bool,
}