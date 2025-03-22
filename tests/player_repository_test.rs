use crate::common::ctx;
use axum::http;
use axum_test::TestServer;
use http::header::CONTENT_TYPE;
use rstest::{fixture, rstest};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::routes::root_routes::root_router;

mod common;

struct App {
    server: TestServer,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    
    let sqlite_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://:memory:".to_string());
        // std::env::var("SQLITE_URL").unwrap_or_else(|_| "file::memory:?cache=shared".to_string());
    let state = ctx(sqlite_url.as_ref()).await?;

    let mut server = TestServer::new(root_router().with_state(state)).unwrap();
    server.save_cookies();
    Ok(App { server })
}

#[cfg(all(test, feature = "it_test"))]
#[rstest]
#[tokio::test]
async fn test_register_ok(#[future] app: eyre::Result<App>) -> eyre::Result<()> {
    let App { server } = app.await?;

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
    res.assert_json_contains(&serde_json::json!({
         "registered": true,
        "id": 1,
        "nickname": "a"
    }));

    Ok(())
}
