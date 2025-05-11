use crate::app::protos::messages::PlayerSession;
use crate::app::redis::RedisConnectionManager;
use crate::error::Result;
use crate::model::cache::PlayerInZone;
use bon::Builder;
use prost::Message;
use redis::AsyncCommands;
use std::sync::Arc;
use strum::AsRefStr;

#[derive(Debug, PartialEq, Eq, Clone, AsRefStr)]
pub enum CacheKey {
    // container: Hash: player_id -> session
    #[strum(serialize = "session")]
    Session,

    // multiple containers, zone_{zone_id} -> player_id
    #[strum(serialize = "zone")]
    ZonePlayers,
}

#[derive(Builder)]
pub struct CacheMiddleware {
    pub cache_pool: Arc<bb8::Pool<RedisConnectionManager>>,
}

impl CacheMiddleware {
    pub async fn get_players_in_zone(&self, zone_id: i64) -> Result<Vec<PlayerInZone>> {
        let mut conn = self.cache_pool.get().await.unwrap();
        let members = conn
            .smembers::<&str, Vec<i64>>(Self::get_zone(zone_id).as_str())
            .await?;

        let mut vec = Vec::with_capacity(members.len());
        for player_id in members {
            let buf = conn
                .hget::<&str, i64, Vec<u8>>(CacheKey::Session.as_ref(), player_id)
                .await?;
            let PlayerSession {
                id,
                nickname,
                level,
                ..
            } = PlayerSession::decode(&buf[..])?;
            vec.push(PlayerInZone {
                id,
                nickname,
                level,
                clan_link: None,
            })
        }

        Ok(vec)
    }

    pub async fn move_player_to_zone(
        &self,
        old_zone_id: i64,
        new_zone_id: i64,
        player_id: i64,
    ) -> eyre::Result<()> {
        let from = Self::get_zone(old_zone_id);
        let to = Self::get_zone(new_zone_id);

        let mut conn = self.cache_pool.get().await?;
        conn.smove::<&str, &str, i64, ()>(from.as_str(), to.as_str(), player_id)
            .await?;

        Ok(())
    }

    fn get_zone(zone_id: i64) -> String {
        format!("{}_{zone_id}", CacheKey::ZonePlayers.as_ref())
    }
}
