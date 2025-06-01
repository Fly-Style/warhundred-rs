use crate::app_state::AppState;
use crate::model::player::PlayerAttributes;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerProfileResponse {
    pub nickname: String,
    pub level: u8,
    pub rank: String,
    pub spec: String,
    pub health: i32,
    pub max_health: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub physique: i32,
    pub luck: i32,
    pub intellect: i32,
    pub days_played: i32,
    pub inventory: Vec<String>,
    pub statistics: Vec<String>,
}

pub fn profile_router() -> Router<AppState> {
    Router::new().route("/profile/{player_nickname}", get(player_profile))
}

pub(crate) async fn player_profile(
    State(state): State<AppState>,
    Path(player_nickname): Path<String>,
) -> crate::error::Result<Json<PlayerProfileResponse>> {
    let AppState {
        player_middleware,
        static_table_middleware,
        ..
    } = state;

    let (player, player_attributes) = player_middleware
        .get_full_player_info_by_nick(player_nickname)
        .await?;

    let PlayerAttributes {
        class_id,
        rank_id,
        strength,
        dexterity,
        physique,
        luck,
        intellect,
        level,
        ..
    } = player_attributes;

    let current_day = Utc::now().timestamp() / 86400; // seconds in a day
    let registration_day = if let Some(time) = player.registration_time {
        time.and_utc().timestamp() / 86400
    } else {
        current_day
    };
    let days_played = (current_day - registration_day) as i32;

    let response = PlayerProfileResponse {
        nickname: player.nickname,
        level: level as u8,
        rank: static_table_middleware.get_rank_name_by_id(rank_id).await?,
        spec: static_table_middleware
            .get_class_name_by_id(class_id)
            .await?,
        health: 0,
        max_health: 0,
        stamina: 0,
        max_stamina: 0,
        strength,
        dexterity,
        physique,
        luck,
        intellect,
        days_played,
        inventory: vec![],  // TODO, NYI
        statistics: vec![], // TODO, NYI
    };

    Ok(Json(response))
}
