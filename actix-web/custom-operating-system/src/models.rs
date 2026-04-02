use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Process {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProcess {
    pub name: String,
    pub status: String,
}
