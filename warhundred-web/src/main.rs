mod routes;
mod utils;

use axum::response::IntoResponse;
use deadpool_diesel::postgres::{Manager, Pool};
use axum::routing::{get, post, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tokio::net::TcpListener;
use routes::player_routes::register;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState { pool };

    let cors = CorsLayer::new().allow_origin(Any);

    // TODO: understand how to host the index file + tree-shacked directory
    let root = Router::new()
        .route_service("/", ServeDir::new("public"))
        .route("/register", post(register))
        // .route("/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, root.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}