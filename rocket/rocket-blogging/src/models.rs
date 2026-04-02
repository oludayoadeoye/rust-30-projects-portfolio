use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlogPost {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBlogPost {
    pub user_id: i32,
    pub title: String,
    pub content: String,
}
