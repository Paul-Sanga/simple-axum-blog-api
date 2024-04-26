use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::database::blog::{self, Entity as Blog};

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

#[derive(Serialize)]
pub struct ResponseBlog {
    id: i32,
    title: String,
    detail: String,
}

pub async fn get_blog(
    Path(blog_id): Path<i32>,
    Extension(database_conection): Extension<DatabaseConnection>,
) -> Result<Json<ResponseBlog>, StatusCode> {
    let blog = Blog::find_by_id(blog_id).one(&database_conection).await;
    if let Ok(blog) = blog {
        if let Some(blog) = blog {
            let response_data = ResponseBlog {
                id: blog.id,
                title: blog.title,
                detail: blog.details,
            };
            Ok(Json(response_data))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_all_blogs(
    Extension(database_connection): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseBlog>>, StatusCode> {
    let blogs = Blog::find()
        .all(&database_connection)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_blog| ResponseBlog {
            id: db_blog.id,
            title: db_blog.title,
            detail: db_blog.details,
        })
        .collect();
    Ok(Json(blogs))
}
