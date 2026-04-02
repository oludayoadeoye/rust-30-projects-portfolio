use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAuthor {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlogPost {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBlogPost {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}
