use crate::app_state::{AppState, Claims, Keys, JWT_AUTH_SECRET};
use crate::error::{AppError, Result};
use crate::model::player::Player;
use crate::routes::{
    LoginPlayerRequest, LoginPlayerResponse, LogoutPlayerRequest, LogoutPlayerResponse,
    PlayersInZoneResponse, RegisterPlayerRequest, RegisterPlayerResponse,
};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::prelude::Utc;
use jsonwebtoken::Algorithm::HS512;
use jsonwebtoken::Header;
use std::sync::LazyLock;
use tower_http::services::ServeDir;
use tracing::{debug, warn};

pub const TOKEN_EXPIRATION_OFFSET: i64 = 60 * 60 * 24 * 30; // 30 days
pub const AUTH_TOKEN_TYPE: &str = "Bearer ";

static KEYS: LazyLock<Keys> = LazyLock::new(|| Keys::new(JWT_AUTH_SECRET.as_bytes()));

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/zone/players", get(players_in_zone))
}

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(new_player): Json<RegisterPlayerRequest>,
) -> Result<impl IntoResponse> {
    let AppState {
        player_middleware, ..
    } = state;

    debug!("Registering player: {:?}", new_player);

    if new_player.username.is_empty() || new_player.password.is_empty() {
        return Err(AppError::MissedCredentials);
    }

    let nickname = new_player.username.clone();
    let new_player = Player {
        nickname: new_player.username,
        email: new_player.email,
        password: password_auth::generate_hash(new_player.password.as_bytes()),
        registration_time: Some(Utc::now().naive_utc()),
        ..Player::default()
    };

    let user = player_middleware
        .register_player(new_player)
        .await
        .map_err(|_| AppError::PlayerCannotRegister(nickname))?;

    Ok(Json(RegisterPlayerResponse {
        nickname: user.nickname,
        registered: true,
    }))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPlayerRequest>,
) -> Result<Json<LoginPlayerResponse>> {
    let AppState {
        player_middleware, ..
    } = state;
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AppError::MissedCredentials);
    }

    let nickname = payload.username.clone();

    if let Ok(user) = player_middleware.get_player_by_nick(payload.username).await {
        password_auth::verify_password(payload.password, user.password.as_ref())
            .map_err(|_| AppError::WrongCredentials(nickname.clone()))?;

        let claims = Claims {
            sub: "alex.syrotenko.official@gmail.com".to_owned(),
            exp: (Utc::now().timestamp() + TOKEN_EXPIRATION_OFFSET) as usize,
        };
        // Create the authorization token
        let access_token = jsonwebtoken::encode(&Header::new(HS512), &claims, &KEYS.encoding)
            .map_err(|_| AppError::TokenCreation)?;

        player_middleware
            .store_player_session_token(nickname.as_str(), access_token.clone())
            .await?;

        // Send the authorized token
        Ok(Json(LoginPlayerResponse {
            access_token,
            nickname,
        }))
    } else {
        // If the user is not found, return an error
        Err(AppError::PlayerNotFound(nickname))
    }
}

pub(crate) async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutPlayerRequest>,
) -> Result<Json<LogoutPlayerResponse>> {
    let AppState {
        player_middleware, ..
    } = state;
    // Check if the user sent the credentials
    if payload.access_token.is_empty() || payload.nickname.is_empty() {
        return Err(AppError::MissedCredentials);
    }

    let LogoutPlayerRequest {
        nickname,
        access_token,
    } = payload;

    if let Err(_err) = player_middleware
        .remove_player_session_token(nickname.as_str(), access_token.as_str())
        .await
    {
        warn!("Error during player logout: {:?}", _err);
        return Ok(Json(LogoutPlayerResponse { ok: false }));
    }

    Ok(Json(LogoutPlayerResponse { ok: true }))
}

pub(crate) async fn players_in_zone(
    State(state): State<AppState>,
    Path(zone_id): Path<i64>,
) -> Result<Json<PlayersInZoneResponse>> {
    let AppState {
        cache_middleware, ..
    } = state;
    let list = cache_middleware.get_players_in_zone(zone_id).await?;

    Ok(Json(PlayersInZoneResponse { list }))
}
