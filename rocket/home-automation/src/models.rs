use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub device_type: String,
    pub state: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDevice {
    pub name: String,
    pub device_type: String,
    pub state: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateState {
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeviceLog {
    pub id: i32,
    pub device_id: Uuid,
    pub old_state: Option<String>,
    pub new_state: String,
    pub recorded_at: DateTime<Utc>,
}
