use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Pattern {
    pub id: i32,
    pub pattern: String,
    pub response: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePattern {
    pub pattern: String,
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ChatLog {
    pub id: i32,
    pub user_message: String,
    pub bot_response: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct MessageRequest {
    pub message: String,
}
