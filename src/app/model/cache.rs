use serde::Serialize;
use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct PlayerInZone {
    pub id: i64,
    pub nickname: String,
    pub level: u32,
    pub clan_link: Option<String>,
}
