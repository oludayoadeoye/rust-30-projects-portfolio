use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Reminder {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub remind_at: DateTime<Utc>,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReminder {
    pub title: String,
    pub description: Option<String>,
    pub remind_at: DateTime<Utc>,
}
