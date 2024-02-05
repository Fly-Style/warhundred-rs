use crate::domain::player_repository::{get_player_by_nick, Credentials, Player};
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
        let user = get_player_by_nick(&self.pool, username).await?;

        match user {
            Some(player) => match password_auth::verify_password(password, &player.password) {
                Ok(_) => Ok(Some(player)),
                Err(_) => Ok(None),
            },
            None => Ok(None),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(get_player_by_nick(&self.pool, user_id.clone()).await?)
    }
}
