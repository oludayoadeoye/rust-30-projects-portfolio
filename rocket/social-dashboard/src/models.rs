use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Platform {
    pub id: i32,
    pub name: String,
    pub handle: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePlatform {
    pub name: String,
    pub handle: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Metric {
    pub id: i32,
    pub platform_id: i32,
    pub followers: i32,
    pub likes: i32,
    pub posts_count: i32,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMetric {
    pub followers: i32,
    pub likes: i32,
    pub posts_count: i32,
}
