use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTopic {
    pub title: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Post {
    pub id: i32,
    pub topic_id: i32,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePost {
    pub author: String,
    pub content: String,
}
