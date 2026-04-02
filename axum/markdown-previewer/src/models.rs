use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Snippet {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub html_content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSnippet {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PreviewResponse {
    pub html: String,
}
