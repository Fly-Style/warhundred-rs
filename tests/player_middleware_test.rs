use crate::common::{ctx, STD_SQLITE_TEST_URL};
use axum::http;
use axum_test::TestServer;
use deadpool_diesel::sqlite::Pool;
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use http::header::CONTENT_TYPE;
use rstest::{fixture, rstest};
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::routes::root_routes::root_router;

mod common;

struct App {
    pool_ref: Arc<Pool>,
    server: TestServer,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    #[cfg(not(feature = "local"))]
    let redis = start_containers().await?;

    let sqlite_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| STD_SQLITE_TEST_URL.to_string());

    #[cfg(feature = "local")]
    let redis_url: String = std::env::var("REDIS_URL")?;
    #[cfg(not(feature = "local"))]
    let redis_url: String = redis_conn_uri(&redis).await?;

    let state = ctx(sqlite_url.as_ref(), redis_url.as_str()).await?;
    let pool_ref = Arc::clone(&state.db_pool);
    let conn = state.db_pool.get().await?;

    conn.interact(|conn| {
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS player (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            nickname TEXT NOT NULL, \
            email TEXT NOT NULL, \
            password TEXT NOT NULL, \
            registration_time TIMESTAMP, \
            last_login_time TIMESTAMP, \
            guild_id INTEGER,\
            banned INTEGER);",
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

#[cfg(test)]
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

    after_test(pool_ref.clone()).await?;

    Ok(())
}
