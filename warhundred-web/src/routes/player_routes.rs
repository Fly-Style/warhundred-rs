use axum::extract::State;
use axum::Json;
use warhundred_be::domain::player_repository;
use warhundred_be::domain::player_repository::{InsertablePlayer, PlayerError};
use crate::routes::{RegisterPlayerRequest, RegisterPlayerResponse};
use crate::utils::json_extractor::JsonExtractor;

pub async fn register(
    State(state): State<crate::AppState>,
    JsonExtractor(new_player): JsonExtractor<RegisterPlayerRequest>,
) -> Result<Json<RegisterPlayerResponse>, PlayerError> {

    let new_player = InsertablePlayer {
        nickname: new_player.nickname,
        email: new_player.email,
        password: new_player.password,
        last_login: std::time::SystemTime::now(),
        last_map_location: 0,
        last_town_location: None,
        guild_id: None,
    };

    let res = player_repository::register_player(&state.pool, &new_player).await;

    match res {
        Ok(_) => Ok(Json(RegisterPlayerResponse {
            id: uuid::Uuid::new_v4(),
            nickname: new_player.nickname,
            registered: true,
        })),
        Err(e) => No,
    }
}