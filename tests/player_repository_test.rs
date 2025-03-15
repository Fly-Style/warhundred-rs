use crate::common::ctx;
use axum_test::TestServer;
use rstest::{fixture, rstest};
use warhundred_rs::routes::root_routes::root_router;

mod common;

struct App {
    server: TestServer,
}

#[fixture]
pub async fn app() -> eyre::Result<App> {
    let sqlite_url =
        std::env::var("SQLITE_URL").unwrap_or_else(|_| "sqlite://:memory:".to_string());
    let state = ctx(sqlite_url.as_ref()).await?;

    let mut server = TestServer::new(root_router().with_state(state)).unwrap();
    server.save_cookies();
    Ok(App { server })
}

#[tokio::test]
#[rstest]
#[cfg(all(test, feature = "it_test"))]
async fn test_register_ok(#[future] app: App) -> eyre::Result<()> {
    Ok(())
}
