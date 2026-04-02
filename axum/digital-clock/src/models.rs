use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{ToSchema, IntoParams};
use chrono::{DateTime, Utc, NaiveTime};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Alarm {
    pub id: i32,
    pub name: String,
    pub alarm_time: NaiveTime,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlarm {
    pub name: String,
    pub alarm_time: NaiveTime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TimeResponse {
    pub current_time: DateTime<Utc>,
    pub timezone: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TimeZoneQuery {
    pub tz: Option<String>,
}
