use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ChatSession {
    pub id: i32,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSession {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Message {
    pub id: i32,
    pub session_id: i32,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SendMessage {
    pub content: String,
}
