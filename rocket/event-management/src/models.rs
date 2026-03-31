use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub capacity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvent {
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub capacity: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Attendee {
    pub id: i32,
    pub event_id: i32,
    pub name: String,
    pub email: String,
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterAttendee {
    pub name: String,
    pub email: String,
}
