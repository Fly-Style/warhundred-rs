use deadpool_diesel::sqlite::Manager;
use deadpool_diesel::Pool;
use std::sync::Arc;
use warhundred_rs::app::middleware::player_middleware::PlayerMiddleware;
use warhundred_rs::app_state::AppState;

pub async fn ctx(sqlite_url: &str) -> eyre::Result<AppState> {
    let manager = Manager::new(sqlite_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Arc::new(Pool::builder(manager).build()?);
    Ok(AppState {
        player_middleware: Arc::new(PlayerMiddleware::builder().pool(pool.clone()).build()),
        pool,
    })
}
