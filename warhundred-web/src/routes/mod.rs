use serde::{Deserialize, Serialize};

pub mod initial_routes;

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
