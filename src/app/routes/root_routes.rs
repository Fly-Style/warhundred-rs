use crate::app_state::AppState;
use crate::domain::player_repository::{Credentials, InsertablePlayer, Player};
use crate::error::PlayerError;
use crate::routes::{
    LoginPlayerRequest, LoginPlayerResponse, RegisterPlayerRequest, RegisterPlayerResponse,
};
use crate::utils::json_extractor::JsonExtractor;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_login::AuthnBackend;
use chrono::prelude::Utc;
use tower_http::services::ServeDir;
use tracing::{debug, warn};

type Result<T, E = PlayerError> = std::result::Result<T, E>;

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
        .route("/login", post(login))
}

pub(crate) async fn register(
    State(state): State<AppState>,
    JsonExtractor(new_player): JsonExtractor<RegisterPlayerRequest>,
) -> Result<Json<RegisterPlayerResponse>> {
    println!("Registering player: {:?}", new_player);

    let new_player = InsertablePlayer {
        nickname: new_player.username,
        email: new_player.email,
        password: password_auth::generate_hash(new_player.password.as_bytes()),
        last_login: Utc::now().to_string(),
        last_map_location: 0,
        last_town_location: 0,
        guild_id: None,
    };

    let player = Player::register_player(&state.pool, new_player).await?;
    Ok(Json(RegisterPlayerResponse {
        id: player.id as i64,
        nickname: player.nickname,
        registered: true,
    }))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    JsonExtractor(extractor): JsonExtractor<LoginPlayerRequest>,
) -> Result<Json<LoginPlayerResponse>> {
    let cred = Credentials {
        username: extractor.username.clone(),
        password: extractor.password,
    };
    let auth_result = state.authenticate(cred).await?;

    match auth_result {
        Some(player) => {
            debug!("Authenticating player: {:?} - success", extractor.username);
            Ok(Json(LoginPlayerResponse {
                nickname: player.nickname,
                logged_in: true,
            }))
        }

        None => {
            warn!(
                "Player {:?} was found, but Option unwrap was not successful.",
                extractor.username.as_str()
            );
            Err(PlayerError::NotFound(extractor.username))
        }
    }
}
