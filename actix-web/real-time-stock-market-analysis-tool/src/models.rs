use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Stock {
    pub id: i32,
    pub symbol: String,
    pub price: f64,
    pub change_percent: f64,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStock {
    pub symbol: String,
    pub price: f64,
    pub change_percent: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AnalysisResponse {
    pub symbol: String,
    pub current_price: f64,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub recommendation: String,
}
