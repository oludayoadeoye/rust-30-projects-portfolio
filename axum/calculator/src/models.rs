use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Calculation {
    pub id: i32,
    pub expression: String,
    pub result: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCalculation {
    pub expression: String,
    pub result: f64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCalculation {
    pub expression: Option<String>,
    pub result: Option<f64>,
}


