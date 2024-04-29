mod blog_routes;

use self::blog_routes::{
    blog_atomic_update, blog_partial_update, create_blog, get_all_blogs, get_blog,
};
use super::config::db_config::db_connetion_config;
use axum::{
    http::Method,
    routing::{get, patch, post, put},
    Extension, Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub async fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let db_connection = db_connetion_config().await.unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        .route("/blog", post(create_blog))
        .route("/blog/:id", get(get_blog))
        .route("/blogs", get(get_all_blogs))
        .route("/blog/:id", put(blog_atomic_update))
        .route("/blog/:id", patch(blog_partial_update))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db_connection))
}
