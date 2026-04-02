use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub type_name: String,
    pub status: String,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDevice {
    pub name: String,
    pub status: String,
}
