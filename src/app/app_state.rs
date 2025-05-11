use crate::app::middleware::player_middleware::PlayerMiddleware;
use crate::app::redis::RedisConnectionManager;
use crate::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::Algorithm::HS512;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::app::middleware::cache_middleware::CacheMiddleware;

pub const JWT_AUTH_SECRET: &str = "1vTDxVKBx6UMSwvYoRGMokJy3dTPrhSVwsSu5yCoPexukstyMtSjEK3MPUpF9t1";

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<deadpool_diesel::sqlite::Pool>,
    pub cache_pool: Arc<bb8::Pool<RedisConnectionManager>>,
    pub player_middleware: Arc<PlayerMiddleware>,
    pub cache_middleware: Arc<CacheMiddleware>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_AUTH_SECRET.as_bytes()),
            &Validation::new(HS512),
        )
        .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
