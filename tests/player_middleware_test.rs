use crate::common::{ctx, redis_conn_uri, start_containers, STD_SQLITE_TEST_URL};
use axum::http;
use axum_test::TestServer;
use deadpool_diesel::sqlite::Pool;
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use http::header::CONTENT_TYPE;
use rstest::{fixture, rstest};
use std::sync::Arc;
use testcontainers::ContainerAsync;
use testcontainers_modules::redis::Redis;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::app_state::AppState;
use warhundred_rs::routes::root_routes::root_router;

mod common;

pub struct App {
    _redis: ContainerAsync<Redis>,
    server: TestServer,
    state: AppState,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let redis = start_containers().await?;

    let sqlite_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| STD_SQLITE_TEST_URL.to_string());

    #[cfg(feature = "local")]
    let _redis_url: String = std::env::var("REDIS_URL")?;

    let redis_url: String = redis_conn_uri(&redis).await?;

    let state = ctx(sqlite_url.as_ref(), redis_url.as_str()).await?;
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

    let mut server = TestServer::new(root_router().with_state(state.clone())).unwrap();
    server.save_cookies();

    #[cfg(not(feature = "local"))]
    Ok(App {
        _redis: redis,
        server,
        state,
    })
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
    let App {
        _redis,
        server,
        state,
    } = app.await?;

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

    after_test(state.db_pool.clone()).await?;

    Ok(())
}

#[cfg(all(test, feature = "it_test"))]
#[rstest]
#[tokio::test]
async fn test_login_ok(#[future] app: eyre::Result<App>) -> eyre::Result<()> {
    let App {
        _redis,
        server,
        state,
    } = app.await?;

    // First register a user
    let username = "testuser";
    let password = "testpassword";

    let register_res = server
        .post("/register")
        .add_header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&serde_json::json!({
            "username": username,
            "email": "test@example.com",
            "password": password
        }))
        .await;

    register_res.assert_status_ok();

    // Then login with the registered user
    let login_res = server
        .post("/login")
        .add_header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .await;

    login_res.assert_status_ok();

    // Parse the response to get the access token
    let login_response = login_res.json::<serde_json::Value>();
    let access_token = login_response["access_token"].as_str().unwrap().to_string();

    // Verify the session exists in Redis
    let session_exists = state
        .player_middleware
        .check_player_session_token(username, access_token)
        .await?;

    assert!(session_exists, "Session should exist in Redis");

    after_test(state.db_pool.clone()).await?;

    Ok(())
}
