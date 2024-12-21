use axum::{Json, Router};
use axum::extract::State;
use axum::routing::post;
use axum_login::AuthnBackend;
use chrono::prelude::Utc;
use tower_http::services::ServeDir;

use error::PlayerError;
use warhundred_app::app_state::AppState;
use warhundred_app::domain::player_repository;
use warhundred_app::domain::player_repository::{Credentials, InsertablePlayer};
use warhundred_app::error;
use warhundred_app::utils::json_extractor::JsonExtractor;

use crate::routes::{
    LoginPlayerRequest, LoginPlayerResponse, RegisterPlayerRequest, RegisterPlayerResponse,
};

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
        .route("/login", post(login))
}

pub(crate) async fn register(
    State(state): State<AppState>,
    JsonExtractor(new_player): JsonExtractor<RegisterPlayerRequest>,
) -> Result<Json<RegisterPlayerResponse>, PlayerError> {
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

    match player_repository::register_player(&state.pool, new_player).await {
        Ok(player) => Ok(Json(RegisterPlayerResponse {
            id: player.id as i64,
            nickname: player.nickname,
            registered: true,
        })),
        Err(e) => Err(e)
    }
}

pub(crate) async fn login(
    State(state): State<AppState>,
    JsonExtractor(extractor): JsonExtractor<LoginPlayerRequest>,
) -> Result<Json<LoginPlayerResponse>, PlayerError> {
    let cred = Credentials { username: extractor.username.clone(), password: extractor.password };
    let auth_result = state.authenticate(cred).await;

    match auth_result {
        Ok(player_opt) =>
            match player_opt {
                Some(player) => {
                    println!("Authenticating player: {:?} - success", extractor.username);
                    Ok(Json(LoginPlayerResponse {
                        nickname: player.nickname,
                        logged_in: true,
                    }))
                }

                None => {
                    println!("Player was found in database, but cannot unwrap Option.");
                    Err(PlayerError::NotFound(extractor.username))
                }
            }
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use axum::body::Body;
    use axum::http;
    use axum::http::{Request, StatusCode};
    use axum::response::Response;
    use deadpool_diesel::sqlite::{Manager, Pool};
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use warhundred_app::app_state::AppState;

    use crate::routes::initial_routes::root_router;

    // #[tokio::test]
    async fn test_register_ok() {
        // TODO: mock the database
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
        let pool = Pool::builder(manager).build().unwrap();

        let state = AppState { pool };

        let app = root_router().with_state(state);

        let response: Response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/register")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({
                            "username": "a",
                            "email": "a@a.com",
                            "password": "pwd"
                        }))
                            .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX);

        // TODO: use assert_matches!, when become stable.

        let body: Value = serde_json::from_slice(body.await.unwrap().as_ref()).unwrap();
        assert_eq!(
            body,
            json!({
                "data": [1, 2, 3, 4]
            })
        );
    }
}
