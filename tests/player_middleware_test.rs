use crate::common::{ctx, redis_conn_uri, start_containers, STD_SQLITE_TEST_URL};
use axum::{http, Router};
use axum_test::TestServer;
use deadpool_diesel::sqlite::Pool;
use deadpool_diesel::Error;
use diesel::{sql_types::Integer, QueryableByName, RunQueryDsl};
use dotenvy::dotenv;
use http::header::CONTENT_TYPE;
use redis::AsyncCommands;
use rstest::{fixture, rstest};
use serial_test::serial;
use std::sync::Arc;
use testcontainers::ContainerAsync;
use testcontainers_modules::redis::Redis;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::app::redis::CacheKey;
use warhundred_rs::app_state::AppState;
use warhundred_rs::routes::profile_routes::profile_router;
use warhundred_rs::routes::root_routes::root_router;

#[derive(QueryableByName, Debug)]
struct PlayerId {
    #[diesel(sql_type = Integer)]
    id: i32,
}

mod common;

pub struct App {
    _redis: ContainerAsync<Redis>,
    server: TestServer,
    state: AppState,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    dotenv().ok();

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
        .expect("Player table creation failed");

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS player_attributes (\
                player_id INTEGER NOT NULL PRIMARY KEY,
                class_id INTEGER NOT NULL DEFAULT 1,
                rank_id INTEGER NOT NULL DEFAULT 1,
                strength INTEGER NOT NULL DEFAULT 0,
                dexterity INTEGER NOT NULL DEFAULT 0,
                physique INTEGER NOT NULL DEFAULT 0,
                luck INTEGER NOT NULL DEFAULT 0,
                intellect INTEGER NOT NULL DEFAULT 0,
                experience INTEGER NOT NULL DEFAULT 0,
                level INTEGER NOT NULL DEFAULT 0,
                valor INTEGER NOT NULL DEFAULT 0);",
        )
        .execute(conn)
        .expect("Player table creation failed");

        Ok::<(), Error>(())
    })
    .await
    .map_err(|e| eyre::eyre!("{:?}", e))??;

    let mut server = TestServer::new(
        Router::new()
            .merge(root_router())
            .merge(profile_router())
            .with_state(state.clone()),
    )
    .unwrap();
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

    conn.interact(|conn| diesel::sql_query("; DROP TABLE player_attributes;").execute(conn))
        .await
        .map_err(|e| eyre::eyre!("{:?}", e))??;

    Ok(())
}

#[cfg(test)]
#[rstest]
#[tokio::test]
#[serial]
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
#[serial]
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
        .await
        .unwrap();

    assert!(session_exists, "Session should exist in Redis");

    after_test(state.db_pool.clone()).await?;

    Ok(())
}

#[cfg(all(test, feature = "it_test"))]
#[rstest]
#[tokio::test]
#[ignore]
#[serial]
async fn test_player_profile(#[future] app: eyre::Result<App>) -> eyre::Result<()> {
    let App {
        _redis,
        server,
        state,
    } = app.await?;

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Create the necessary tables
    let conn = state.db_pool.get().await?;

    // Register a test player
    let username = "test1";
    let email = "profile@test.com";
    let password = "password";

    let register_res = server
        .post("/register")
        .add_header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .json(&serde_json::json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .await;

    register_res.assert_status_ok();
    // Ensure the registration was successful
    register_res.assert_json_contains(&serde_json::json!({
        "nickname": username,
        "registered": true,
    }));

    // Get the player ID
    let player_ids = conn
        .interact(|conn| {
            diesel::sql_query("SELECT id FROM player WHERE nickname = 'test1'")
                .load::<PlayerId>(conn)
        })
        .await
        .map_err(|e| eyre::eyre!("{:?}", e))?
        .unwrap();

    assert!(
        !player_ids.is_empty(),
        "Player should exist in the database"
    );
    let player_id = player_ids[0].id;

    // Update player attributes (or insert if not exists)
    conn.interact(move |conn| {
        // In SQLite, we can use the INSERT OR REPLACE statement which is an UPSERT operation
        diesel::sql_query(format!(
            "INSERT OR REPLACE INTO player_attributes
             (player_id, class_id, rank_id, strength, dexterity, physique, luck, intellect, experience, level, valor)
             VALUES ({}, 1, 1, 10, 8, 6, 7, 5, 100, 2, 5)",
            player_id
        ))
        .execute(conn)
    })
    .await
    .map_err(|e| eyre::eyre!("{:?}", e))??;

    // Manually add class and rank data to the cache
    let mut cache_conn = state.cache_pool.get().await?;

    // Add class data to the cache
    cache_conn
        .hset::<&str, i32, String, ()>(CacheKey::ClassTable.as_ref(), 1, "Warrior".to_string())
        .await?;

    // Add rank data to cache
    cache_conn
        .hset::<&str, i32, String, ()>(CacheKey::RankTable.as_ref(), 1, "Recruit".to_string())
        .await?;

    // Make a request to the profile endpoint
    let profile_res = server.get(&format!("/profile/{}", username)).await;

    profile_res.assert_status_ok();
    profile_res.assert_json_contains(&serde_json::json!({
        "nickname": username,
        "level": 2,
        "rank": "Recruit",
        "spec": "Warrior",
        "strength": 10,
        "dexterity": 8,
        "physique": 6,
        "luck": 7,
        "intellect": 5
    }));

    // Clean up
    after_test(state.db_pool.clone()).await?;

    Ok(())
}
