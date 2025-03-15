use deadpool_diesel::sqlite::Manager;
use deadpool_diesel::Pool;
use warhundred_rs::app_state::AppState;

pub async fn ctx(sqlite_url: &str) -> eyre::Result<AppState> {
    let manager = Manager::new(sqlite_url, deadpool_diesel::Runtime::Tokio1);
    Ok(AppState {
        pool: Pool::builder(manager).build()?,
    })
}
