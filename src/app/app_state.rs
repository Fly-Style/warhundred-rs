use crate::app::middleware::player_middleware::PlayerMiddleware;
use crate::error::AppError;
use crate::error::AppError::PlayerNotFound;
use crate::model::player::{Credentials, Player};
use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use deadpool_diesel::sqlite::Pool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<Pool>,
    pub player_middleware: Arc<PlayerMiddleware>,
}

#[async_trait]
impl AuthnBackend for AppState {
    type User = Player;
    type Credentials = Credentials;
    type Error = AppError;

    async fn authenticate(
        &self,
        Credentials { username, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let result = self.player_middleware.get_player_by_nick(username).await;
        match result {
            Ok(player) => {
                match password_auth::verify_password(password, player.password.as_ref()) {
                    Ok(_) => Ok(Some(player)),
                    Err(_) => Err(PlayerNotFound(player.nickname)),
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let result = self
            .player_middleware
            .get_player_by_nick(user_id.into())
            .await;
        match result {
            Ok(player) => Ok(Some(player)),
            Err(e) => Err(e),
        }
    }
}
