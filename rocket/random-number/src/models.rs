use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GeneratedNumber {
    pub id: i32,
    pub value: i32,
    pub min_range: i32,
    pub max_range: i32,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RandomRequest {
    pub min: i32,
    pub max: i32,
}
