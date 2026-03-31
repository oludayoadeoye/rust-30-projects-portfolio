use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ExchangeRate {
    pub id: i32,
    pub currency_code: String,
    pub rate_to_usd: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExchangeRate {
    pub currency_code: String,
    pub rate_to_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Conversion {
    pub id: i32,
    pub from_currency: String,
    pub to_currency: String,
    pub amount: f64,
    pub result: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PerformConversion {
    pub from_currency: String,
    pub to_currency: String,
    pub amount: f64,
}
