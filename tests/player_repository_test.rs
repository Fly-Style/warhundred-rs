use crate::common::ctx;
use axum::http;
use axum_test::TestServer;
use deadpool_diesel::sqlite::Pool;
use diesel::RunQueryDsl;
use http::header::CONTENT_TYPE;
use rstest::{fixture, rstest};
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::app_state::AppState;
use warhundred_rs::routes::root_routes::root_router;

mod common;

pub const TEST_IN_MEMORY_DB: &str = "sqlite://:memory:";

struct App {
    pool_ref: Arc<Pool>,
    server: TestServer,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sqlite_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| TEST_IN_MEMORY_DB.to_string());
    // std::env::var("SQLITE_URL").unwrap_or_else(|_| "file::memory:?cache=shared".to_string());

    let state = ctx(sqlite_url.as_ref()).await?;
    let pool_ref = Arc::clone(&state.pool);
    let conn = state.pool.get().await?;

    conn.interact(|conn| {
        diesel::sql_query(
            "CREATE TABLE player (\
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, \
            nickname TEXT NOT NULL, \
            email TEXT NOT NULL, \
            password TEXT NOT NULL, \
            registration_time TIMESTAMP NOT NULL, \
            last_login_time TIMESTAMP NOT NULL, \
            guild_id INTEGER NOT NULL);",
        )
        .execute(conn)
    })
    .await
    .map_err(|e| eyre::eyre!("{:?}", e))??;

    let mut server = TestServer::new(root_router().with_state(state)).unwrap();
    server.save_cookies();
    Ok(App { pool_ref, server })
}

pub async fn after_test(pool: Arc<Pool>) -> eyre::Result<()> {
    let conn = pool.get().await?;
    conn.interact(|conn| diesel::sql_query("DROP TABLE player;").execute(conn))
        .await
        .map_err(|e| eyre::eyre!("{:?}", e))??;

    Ok(())
}

#[cfg(all(test, feature = "it_test"))]
#[rstest]
#[tokio::test]
async fn test_register_ok(#[future] app: eyre::Result<App>) -> eyre::Result<()> {
    let App { pool_ref, server } = app.await?;

    let res = server
        .post("/register")
        .add_header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&serde_json::json!({
            "username": "a",
            "email": "a@a.com",
            "password": "pwd"
        }))
        .await;

    res.assert_status_ok();
    println!("{:?}", res.text());
    res.assert_json_contains(&serde_json::json!({
        "nickname": "a",
        "registered": true
    }));

    after_test(pool_ref).await?;

    Ok(())
}
