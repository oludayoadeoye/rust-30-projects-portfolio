use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ContactMessage {
    pub id: i32,
    pub sender_name: String,
    pub sender_email: String,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMessage {
    pub sender_name: String,
    pub sender_email: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStatus {
    pub status: String,
}
