mod routes;

use std::env;
use dotenvy::dotenv;
use axum::response::IntoResponse;
use deadpool_diesel::postgres::{Manager, Pool};
use axum::routing::{post, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tokio::net::TcpListener;
use routes::player_routes::register;
use warhundred_be::error::internal_error;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    // initialize tracing
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

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
        .map_err(internal_error)
        .unwrap();
}