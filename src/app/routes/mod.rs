use crate::model::cache::PlayerInZone;
use serde::{Deserialize, Serialize};

pub mod profile_routes;
pub mod root_routes;

#[derive(Debug, Deserialize)]
pub struct RegisterPlayerRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterPlayerResponse {
    nickname: String,
    registered: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginPlayerRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginPlayerResponse {
    access_token: String,
    nickname: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutPlayerRequest {
    nickname: String,
    access_token: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutPlayerResponse {
    ok: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayersInZoneResponse {
    pub list: Vec<PlayerInZone>,
}
