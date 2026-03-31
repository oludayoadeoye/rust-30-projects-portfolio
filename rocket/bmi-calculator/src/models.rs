use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BmiRecord {
    pub id: i32,
    pub user_name: String,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub bmi: f64,
    pub category: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBmiRecord {
    pub user_name: String,
    pub height_cm: f64,
    pub weight_kg: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BmiStats {
    pub average_bmi: f64,
    pub total_records: i64,
}
