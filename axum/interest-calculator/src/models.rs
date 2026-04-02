use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InterestCalculation {
    pub id: i32,
    pub principal: Decimal,
    pub rate: f64,
    pub time_years: f64,
    pub compound_frequency: i32,
    pub total_amount: Decimal,
    pub interest_earned: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalculateRequest {
    pub principal: Decimal,
    pub rate: f64, // percentage, e.g., 5.0
    pub time_years: f64,
    pub compound_frequency: Option<i32>, // defaults to 1 (annually)
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InterestResponse {
    pub total_amount: Decimal,
    pub interest_earned: Decimal,
}
