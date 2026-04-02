use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Conversion {
    pub id: i32,
    pub input_unit: String,
    pub input_value: f64,
    pub output_unit: String,
    pub output_value: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConversionRequest {
    pub from: String,
    pub to: String,
    pub value: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConversionResponse {
    pub result: f64,
}
