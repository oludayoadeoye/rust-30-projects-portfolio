use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Document {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDocument {
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDocument {
    pub title: Option<String>,
    pub content: Option<String>,
    pub version: i32, // For optimistic locking
}
