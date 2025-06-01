use deadpool_diesel::sqlite::Manager;
use deadpool_diesel::Pool;
use std::sync::Arc;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::redis::Redis;
use warhundred_rs::app::middleware::cache_middleware::CacheMiddleware;
use warhundred_rs::app::middleware::player_middleware::PlayerMiddleware;
use warhundred_rs::app::middleware::static_tables_cache_middleware::StaticTablesCacheMiddleware;
use warhundred_rs::app::redis::RedisConnectionManager;
use warhundred_rs::app_state::AppState;

pub const STD_SQLITE_TEST_URL: &str = "sqlite://:memory:";

pub async fn ctx(sqlite_url: &str, redis_url: &str) -> eyre::Result<AppState> {
    let manager = Manager::new(sqlite_url, deadpool_diesel::Runtime::Tokio1);

    let db_pool = Arc::new(Pool::builder(manager).build()?);
    let cache_pool = Arc::new({
        let manager = RedisConnectionManager::new(redis_url).expect("Unable connect to cache");
        bb8::Pool::builder().max_size(4).build(manager).await?
    });

    Ok(AppState {
        player_middleware: Arc::new(
            PlayerMiddleware::builder()
                .db_pool(db_pool.clone())
                .cache_pool(cache_pool.clone())
                .build(),
        ),
        cache_middleware: Arc::new(
            CacheMiddleware::builder()
                .cache_pool(cache_pool.clone())
                .build(),
        ),
        static_table_middleware: Arc::new(
            StaticTablesCacheMiddleware::builder()
                .db_pool(db_pool.clone())
                .cache_pool(cache_pool.clone())
                .build(),
        ),
        db_pool,
        cache_pool,
    })
}

pub async fn redis_conn_uri(c: &ContainerAsync<Redis>) -> eyre::Result<String> {
    Ok(format!(
        "redis://{host}:{port}",
        host = c.get_host().await?,
        port = c.get_host_port_ipv4(6379).await?
    ))
}

pub async fn start_containers() -> eyre::Result<ContainerAsync<Redis>> {
    Ok(Redis::default().start().await?)
}
