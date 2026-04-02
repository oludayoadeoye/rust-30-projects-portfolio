use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Alert {
    pub id: i32,
    pub severity: String,
    pub message: String,
    pub source_ip: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlert {
    pub severity: String,
    pub message: String,
    pub source_ip: String,
}
