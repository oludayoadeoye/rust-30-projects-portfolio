use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePage {
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePage {
    pub title: Option<String>,
    pub content: Option<String>,
}
