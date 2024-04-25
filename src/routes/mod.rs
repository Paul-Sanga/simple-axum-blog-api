use axum::{http::Method, routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        .route("/", get(|| async { String::from("I am Phantom Vasploit") }))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
