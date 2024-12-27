mod routes;

use crate::routes::initial_routes::root_router;
use dotenvy::dotenv;
use std::env;
use deadpool_diesel::Pool;
use deadpool_diesel::sqlite::{Manager};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use warhundred_be::app_state::AppState;
use warhundred_be::error::internal_error;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    let state = AppState { pool };

    let app = root_router().with_state(state);

    // TODO: understand how to host the index file + tree-shacked directory

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(
        listener,
        app.layer(CorsLayer::new().allow_origin(Any))
            .layer(TraceLayer::new_for_http()),
    )
    .await
    .map_err(internal_error)
    .unwrap();
}
