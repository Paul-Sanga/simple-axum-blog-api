use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

use crate::database::blog;

#[derive(Deserialize, Debug)]
pub struct RequestBlog {
    title: String,
    details: String,
}

pub async fn create_blog(
    Extension(database_connection): Extension<DatabaseConnection>,
    Json(request_data): Json<RequestBlog>,
) {
    let new_blog = blog::ActiveModel {
        title: Set(request_data.title),
        details: Set(request_data.details),
        ..Default::default()
    };

    let result = new_blog.save(&database_connection).await.unwrap();
    dbg!(&result);
}

pub async fn get_blog() {}
