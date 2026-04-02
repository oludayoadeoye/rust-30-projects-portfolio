use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TempConversion {
    pub id: i32,
    pub input_unit: String,
    pub input_value: f64,
    pub output_unit: String,
    pub output_value: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TempRequest {
    pub from: String, // "C", "F", "K"
    pub to: String,
    pub value: f64,
}
