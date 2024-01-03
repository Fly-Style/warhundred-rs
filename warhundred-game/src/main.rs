use axum::Json;
use axum::response::IntoResponse;
use axum::routing::{get, post, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new().allow_origin(Any);

    // TODO: understand how to host the index file + tree-shacked directory
    let root = Router::new()
        .route_service("/", ServeDir::new("public"))
        // .route("/register", post(handler))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, root.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}