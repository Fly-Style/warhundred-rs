use axum::extract::State;
use axum::Json;
use warhundred_be::domain::player_repository;
use warhundred_be::domain::player_repository::{InsertablePlayer};
use warhundred_be::utils::json_extractor::JsonExtractor;
use crate::routes::{RegisterPlayerRequest, RegisterPlayerResponse};

pub async fn register(
    State(state): State<crate::AppState>,
    JsonExtractor(new_player): JsonExtractor<RegisterPlayerRequest>,
) -> Result<Json<RegisterPlayerResponse>, warhundred_be::error::PlayerError> {
    println!("Registering player: {:?}", new_player);
    let new_player = InsertablePlayer {
        nickname: new_player.username,
        email: new_player.email,
        password: new_player.password,
        last_login: std::time::SystemTime::now(),
        last_map_location: 0,
        last_town_location: 0,
        guild_id: None,
    };

    let wrapped = player_repository::register_player(&state.pool, new_player).await;
    match wrapped {
        Ok(_) => {
            let res = wrapped.unwrap();

            Ok(Json(RegisterPlayerResponse {
                id: res.id,
                nickname: res.nickname,
                registered: true,
            }))
        }
        Err(e) => Err(e),
    }
}