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
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use warhundred_rs::app_state::AppState;
use warhundred_rs::routes::root_routes::root_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Initializing the server.");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Arc::new(Pool::builder(manager).build().unwrap());

    let state = AppState { pool };

    let app = root_router().with_state(state);

    // TODO: understand how to host the index file + tree-shacked directory
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!("Starting server on port 8000");

    axum::serve(
        listener,
        app.layer(CorsLayer::new().allow_origin(Any))
            .layer(TraceLayer::new_for_http()),
    )
    .await
    // .map_err(internal_error)
    .unwrap();
}
