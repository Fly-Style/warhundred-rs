use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod player_routes;

#[derive(Debug, Deserialize)]
pub struct RegisterPlayerRequest {
    nickname: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPlayerResponse {
    id: Uuid,
    nickname: String,
    registered: bool,
}