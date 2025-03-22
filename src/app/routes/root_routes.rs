use crate::app_state::AppState;
use crate::model::player_repository::{Credentials, Player};
use crate::error::{AppError, Result};
use crate::routes::{
    LoginPlayerRequest, LoginPlayerResponse, RegisterPlayerRequest, RegisterPlayerResponse,
};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_login::AuthnBackend;
use chrono::prelude::Utc;
use tower_http::services::ServeDir;
use tracing::debug;

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
        .route("/login", post(login))
}

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(new_player): Json<RegisterPlayerRequest>,
) -> Result<impl IntoResponse> {
    let AppState {
        pool
    } = state;
    
    println!("Registering player: {:?}", new_player);

    let new_player = Player {
        nickname: new_player.username,
        email: new_player.email,
        password: password_auth::generate_hash(new_player.password.as_bytes()),
        registration_time: Some(Utc::now().naive_utc()),
        ..Player::default()
    };

    let player = Player::register_player(pool.clone(), new_player).await?;
    Ok(Json(RegisterPlayerResponse {
        nickname: player.nickname,
        registered: true,
    }))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(extractor): Json<LoginPlayerRequest>,
) -> Result<impl IntoResponse> {
    let cred = Credentials {
        username: extractor.username.clone(),
        password: extractor.password,
    };
    let auth_result = state.authenticate(cred).await?;

    if let Some(player) = auth_result {
        debug!("Authenticating player: {:?} - success", extractor.username);
        Ok(Json(LoginPlayerResponse {
            nickname: player.nickname,
            logged_in: true,
        }))
    } else {
        Err(AppError::PlayerNotFound(extractor.username))
    }
}
