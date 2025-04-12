use deadpool_diesel::sqlite::Manager;
use deadpool_diesel::Pool;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::app::db::{migration_connection, run_migrations};
use warhundred_rs::app::middleware::player_middleware::PlayerMiddleware;
use warhundred_rs::app::redis::RedisConnectionManager;
use warhundred_rs::app_state::AppState;
use warhundred_rs::routes::root_routes::root_router;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("Initializing the server.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_uri = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // Setup database connection pool
    {
        // Run migrations
        let connection = &mut migration_connection(database_url.as_ref());
        run_migrations(connection).unwrap();
        tracing::info!("Migrations applied successfully.");
    }

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let db_pool = Arc::new(Pool::builder(manager).build()?);

    // Setup Redis connection pool
    let cache_pool = {
        let manager = RedisConnectionManager::new(redis_uri).expect("Unable connect to cache");
        Arc::new(
            bb8::Pool::builder()
                .max_size(16)
                .build(manager)
                .await
                .map_err(eyre::Report::from)?,
        )
    };

    // Setup HTTP routes
    let player_middleware = Arc::new(
        PlayerMiddleware::builder()
            .db_pool(db_pool.clone())
            .cache_pool(cache_pool.clone())
            .build(),
    );

    let state = AppState {
        db_pool,
        cache_pool,
        player_middleware,
    };

    // Setup HTTP server
    let app = root_router().with_state(state);

    // TODO: use nginx or similar for production to host static files
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    tracing::info!("Starting server on port 8000");

    axum::serve(
        listener,
        app.layer(CorsLayer::new().allow_origin(Any))
            .layer(TraceLayer::new_for_http()),
    )
    .await?;

    Ok(())
}
