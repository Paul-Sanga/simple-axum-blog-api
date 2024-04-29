use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{
    prelude::Date, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set
};
use serde::{Deserialize, Serialize};

use crate::database::blog::{self, Entity as Blog};

#[derive(Deserialize, Debug)]
pub struct RequestBlog {
    title: String,
    details: String,
    category: Option<String>,
}

pub async fn create_blog(
    Extension(database_connection): Extension<DatabaseConnection>,
    Json(request_data): Json<RequestBlog>,
) {
    let new_blog = blog::ActiveModel {
        title: Set(request_data.title),
        details: Set(request_data.details),
        category: Set(request_data.category),
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
    category: Option<String>,
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
                category: blog.category,
            };
            Ok(Json(response_data))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

// Getting all priorities with a filter
#[derive(Deserialize)]
pub struct GetBlogQueryParams {
    category: Option<String>,
}

pub async fn get_all_blogs(
    Extension(database_connection): Extension<DatabaseConnection>,
    Query(query_param): Query<GetBlogQueryParams>,
) -> Result<Json<Vec<ResponseBlog>>, StatusCode> {
    let mut catergory_filter = Condition::all();
    if let Some(category) = query_param.category {
        catergory_filter = if category.is_empty() {
            catergory_filter.add(blog::Column::Category.is_null())
        } else {
            catergory_filter.add(blog::Column::Category.eq(category))
        };
    }
    let blogs = Blog::find()
        .filter(catergory_filter)
        .all(&database_connection)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_blog| ResponseBlog {
            id: db_blog.id,
            title: db_blog.title,
            detail: db_blog.details,
            category: db_blog.category,
        })
        .collect();
    Ok(Json(blogs))
}

#[derive(Deserialize)]
pub struct RequestUpdateBlog{
    pub id: i32,
    pub title: String,
    pub details: String,
    pub category: Option<String>,
    pub created_at: Option<Date>,
    pub updated_at: Option<Date>,
}

pub async fn blog_atomic_update(
    Extension(database_connection): Extension<DatabaseConnection>,
    Path(blog_id): Path<i32>,
    Json(request_blog): Json<RequestUpdateBlog>
) -> Result<(), StatusCode> {
    let update_blog = blog::ActiveModel{
        id: Set(blog_id),
        title: Set(request_blog.title),
        details: Set(request_blog.details),
        category: Set(request_blog.category),
        created_at: Set(request_blog.created_at),
        updated_at: Set(request_blog.updated_at)
    };

    Blog::update(update_blog)
    .filter(blog::Column::Id.eq(blog_id))
    .exec(&database_connection)
    .await
    .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
