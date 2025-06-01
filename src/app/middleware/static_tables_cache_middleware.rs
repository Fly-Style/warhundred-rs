use crate::app::redis::{CacheKey, RedisConnectionManager};
use crate::error::AppError;
use crate::model::r#static::{PlayerClass, PlayerRankTable};
use crate::schema::player_class::dsl::player_class;
use crate::schema::player_rank_table::dsl::player_rank_table;
use bon::Builder;
use diesel::RunQueryDsl;
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::error;

#[derive(Builder)]
pub struct StaticTablesCacheMiddleware {
    pub db_pool: Arc<deadpool_diesel::sqlite::Pool>,
    pub cache_pool: Arc<bb8::Pool<RedisConnectionManager>>,
}

impl StaticTablesCacheMiddleware {
    pub async fn prefetch_rank_table(&self) -> crate::error::Result<()> {
        let conn = self.db_pool.clone().get().await?;

        let ranks = conn
            .interact(|conn| player_rank_table.load::<PlayerRankTable>(conn))
            .await?
            .map_err(|e| AppError::QueryError(e.to_string()))?;

        let mut cache_conn = self.cache_pool.get().await?;

        for rank_table in ranks {
            if let Err(e) = cache_conn
                .hset::<&str, i32, String, ()>(
                    CacheKey::RankTable.as_ref(),
                    rank_table.id,
                    rank_table.rank_name_EN,
                )
                .await
            {
                error!("Failed to cache player class table data: {}", e);
                return Err(AppError::CacheError(e));
            }
        }
        tracing::info!("Player rank table prefetched into the cache successfully.");

        Ok(())
    }

    pub async fn prefetch_class_table(&self) -> crate::error::Result<()> {
        let conn = self.db_pool.clone().get().await?;

        let classes = conn
            .interact(|conn| player_class.load::<PlayerClass>(conn))
            .await?
            .map_err(|e| AppError::QueryError(e.to_string()))?;

        let mut cache_conn = self.cache_pool.get().await?;

        for class in classes {
            if let Err(e) = cache_conn
                .hset::<&str, i32, String, ()>(
                    CacheKey::ClassTable.as_ref(),
                    class.class_id,
                    class.class_name,
                )
                .await
            {
                error!("Failed to cache player class table data: {}", e);
                return Err(AppError::CacheError(e));
            }
        }
        tracing::info!("Player class table prefetched into the cache successfully.");

        Ok(())
    }

    pub async fn get_rank_name_by_id(&self, rank_id: i32) -> crate::error::Result<String> {
        let mut conn = self.cache_pool.get().await.unwrap();

        Ok(conn
            .hget::<&str, i32, String>(CacheKey::RankTable.as_ref(), rank_id)
            .await?)
    }

    pub async fn get_class_name_by_id(&self, class_id: i32) -> crate::error::Result<String> {
        let mut conn = self.cache_pool.get().await.unwrap();

        Ok(conn
            .hget::<&str, i32, String>(CacheKey::ClassTable.as_ref(), class_id)
            .await?)
    }
}
