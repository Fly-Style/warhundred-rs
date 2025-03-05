use crate::domain::player_repository::{Credentials, Player};
use crate::error::PlayerError;
use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use deadpool_diesel::sqlite::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

#[async_trait]
impl AuthnBackend for AppState {
    type User = Player;
    type Credentials = Credentials;
    type Error = PlayerError;

    async fn authenticate(
        &self,
        Credentials { username, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let result = Player::get_player_by_nick(&self.pool, username).await;
        match result {
            Ok(player) => match password_auth::verify_password(password, &*player.password) {
                Ok(_) => Ok(Some(player)),
                Err(_) => Err(PlayerError::NotFound(player.nickname)),
            },
            Err(e) => Err(e),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let result = Player::get_player_by_nick(&self.pool, user_id.to_string()).await;
        match result {
            Ok(player) => Ok(Some(player)),
            Err(e) => Err(e),
        }
    }
}
