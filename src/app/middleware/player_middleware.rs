use crate::app::redis::{CacheKey, RedisConnectionManager};
use crate::error::AppError::PlayerNotFound;
use crate::error::{AppError, Result};
use crate::model::player::{Player, PlayerAttributes};
use crate::model::DefaultModel;
use crate::schema::player::dsl::player;
use crate::schema::player::nickname;
use crate::schema::player_attributes::dsl::player_attributes;
use bon::Builder;
use diesel::result::Error::RollbackTransaction;
use diesel::QueryDsl;
use diesel::{Connection, ExpressionMethods, RunQueryDsl, SelectableHelper};
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::error;

pub type PlayerWithAttributes = (Player, PlayerAttributes);

#[derive(Builder)]
pub struct PlayerMiddleware {
    pub db_pool: Arc<deadpool_diesel::sqlite::Pool>,
    pub cache_pool: Arc<bb8::Pool<RedisConnectionManager>>,
}

impl PlayerMiddleware {
    pub async fn register_player(&self, new_player: Player) -> Result<Player> {
        use crate::schema::player::dsl::*;

        let conn = &self.db_pool.clone().get().await?;
        let nick = new_player.nickname.clone();
        let res = conn
            .interact(move |conn| {
                diesel::insert_into(player)
                    .values(new_player)
                    .returning(Player::as_returning())
                    .get_result(conn)
            })
            .await;

        match res? {
            Ok(p) => Ok(p),
            Err(e) => {
                error!("Error during player registration: {:?}", e);
                Err(AppError::PlayerCannotRegister(nick))
            }
        }
    }

    pub async fn register_player_transaction(&self, new_player: Player) -> Result<Player> {
        use crate::schema::player::dsl::*;

        let conn = self.db_pool.clone().get().await?;
        let nick = new_player.nickname.clone();
        let tx_res = conn
            .interact(move |conn| {
                conn.transaction(|conn| {
                    // Insert into `player` table.
                    let p: Player = diesel::insert_into(player)
                        .values(new_player)
                        .returning(Player::as_returning())
                        .get_result(conn)?;

                    let Some(player_id) = p.id else {
                        return Err(RollbackTransaction);
                    };

                    // Insert defaults into `player_attributes`
                    diesel::insert_into(player_attributes)
                        .values(PlayerAttributes::default_model(player_id))
                        .execute(conn)?;

                    Ok(p)
                })
            })
            .await?;

        match tx_res {
            Ok(p) => Ok(p),
            Err(e) => {
                error!("Error during player registration: {:?}", e);
                Err(AppError::PlayerCannotRegister(nick))
            }
        }
    }

    pub async fn get_player_by_nick(&self, nick: String) -> Result<Player> {
        let conn = self.db_pool.clone().get().await?;
        let _nick: String = nick.clone();

        let query_result = conn
            .interact(move |conn| player.filter(nickname.eq(nick)).first::<Player>(conn))
            .await?;

        match query_result {
            Ok(res) => Ok(res),
            Err(_) => Err(PlayerNotFound(_nick)),
        }
    }

    pub async fn get_full_player_info_by_nick(&self, nick: String) -> Result<PlayerWithAttributes> {
        let conn = self.db_pool.clone().get().await?;
        let _nick: String = nick.clone();

        let query_result = conn
            .interact(move |conn| {
                player
                    .filter(nickname.eq(nick))
                    .inner_join(player_attributes)
                    .first::<(Player, PlayerAttributes)>(conn)
            })
            .await?;

        match query_result {
            Ok(res) => Ok(res),
            Err(_) => Err(PlayerNotFound(_nick)),
        }
    }

    pub async fn store_player_session_token(&self, nick: &str, token: String) -> Result<()> {
        let mut conn = self.cache_pool.get().await?;
        let set_member = format!("{}:{}", nick, token);
        conn.sadd::<&str, &str, ()>(CacheKey::PlayerSession.as_ref(), set_member.as_str())
            .await
            .map_err(|e| {
                error!("Error during storing player session token: {:?}", e);
                AppError::CacheError(e)
            })
    }

    pub async fn check_player_session_token(
        &self,
        nick: &str,
        token_to_compare: String,
    ) -> Result<bool> {
        let mut conn = self.cache_pool.get().await?;
        let set_member = format!("{}:{}", nick, token_to_compare);
        Ok(1 == conn
            .sismember::<&str, &str, i64>(CacheKey::PlayerSession.as_ref(), set_member.as_str())
            .await
            .map_err(|e| {
                error!("Error during storing player session token: {:?}", e);
                AppError::CacheError(e)
            })?)
    }

    pub async fn remove_player_session_token(&self, nick: &str, access_token: &str) -> Result<()> {
        let mut conn = self.cache_pool.get().await?;
        let set_member = format!("{}:{}", nick, access_token);
        conn.srem::<&str, &str, ()>(CacheKey::PlayerSession.as_ref(), set_member.as_str())
            .await
            .map_err(|e| {
                error!("Error during removing player session token: {:?}", e);
                AppError::CacheError(e)
            })
    }

    // Note: just an example of inner JOIN and transaction
    async fn inc_valor(&self, p_id: i32, rank_up: bool) -> Result<()> {
        use crate::schema::player_attributes::dsl::*;
        let conn = &self.db_pool.get().await?;
        let _ = conn
            .interact(move |conn| {
                conn.transaction(|conn| {
                    diesel::update(player_attributes)
                        .filter(player_id.eq(p_id))
                        .set(valor.eq(valor + 1))
                        .execute(conn)?;
                    if rank_up {
                        diesel::update(player_attributes)
                            .filter(player_id.eq(p_id))
                            .set(rank_id.eq(rank_id + 1))
                            .execute(conn)?;
                    }
                    Ok::<(), deadpool_diesel::Error>(())
                })
            })
            .await?;

        Ok(())
    }
}
