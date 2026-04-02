use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Drone {
    pub id: i32,
    pub drone_id: String,
    pub status: String,
    pub battery_level: f64,
    pub last_telemetry: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterDrone {
    pub drone_id: String,
}
