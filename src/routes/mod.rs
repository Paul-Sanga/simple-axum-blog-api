use axum::{http::Method, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    

    Router::new()
    .route("/", get(||async{String::from("I am Phantom Vasploit")}))
    .layer(cors)
}
