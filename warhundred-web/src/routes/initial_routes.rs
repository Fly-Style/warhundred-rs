use crate::routes::{RegisterPlayerRequest, RegisterPlayerResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use error::PlayerError;
use tower_http::services::ServeDir;
use warhundred_be::app_state::AppState;
use warhundred_be::domain::player_repository;
use warhundred_be::domain::player_repository::InsertablePlayer;
use warhundred_be::error;
use warhundred_be::utils::json_extractor::JsonExtractor;

pub fn root_router() -> Router<AppState> {
    Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
    // .route("/login", post(login))
}

pub(crate) async fn register(
    State(state): State<AppState>,
    JsonExtractor(new_player): JsonExtractor<RegisterPlayerRequest>,
) -> Result<Json<RegisterPlayerResponse>, PlayerError> {
    println!("Registering player: {:?}", new_player);

    let new_player = InsertablePlayer {
        nickname: new_player.username,
        email: new_player.email,
        password: hash_password(new_player.password.as_bytes()),
        last_login: std::time::SystemTime::now(),
        last_map_location: 0,
        last_town_location: 0,
        guild_id: None,
    };

    match player_repository::register_player(&state.pool, new_player).await {
        Ok(player) => Ok(Json(RegisterPlayerResponse {
            id: player.id,
            nickname: player.nickname,
            registered: true,
        })),
        Err(e) => Err(e),
    }
}

fn hash_password(pwd: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(pwd, &salt);

    assert!(hash.is_ok(), "Failed to hash password");
    hash.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::routes::initial_routes::root_router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use deadpool_diesel::sqlite::{Manager, Pool};
    use serde_json::{json, Value};
    use std::env;
    use axum::http;
    use tower::ServiceExt;
    use warhundred_be::app_state::AppState;

    // #[tokio::test]
    async fn test_register() {
        // TODO: mock the database
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
        let pool = Pool::builder(manager).build().unwrap();

        let state = AppState { pool };

        let app = root_router().with_state(state);

        let response = app
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

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            body,
            json!({
                "data": [1, 2, 3, 4]
            })
        );
    }
}
