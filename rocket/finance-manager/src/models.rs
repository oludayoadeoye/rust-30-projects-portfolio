use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub account_type: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccount {
    pub name: String,
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TransactionRecord {
    pub id: i32,
    pub account_id: Uuid,
    pub amount: f64,
    pub description: Option<String>,
    pub category: Option<String>,
    pub transaction_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ForecastResponse {
    pub current_balance: f64,
    pub monthly_average: f64,
    pub projections: Vec<f64>, // Balance for next 6 months
}


#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTransaction {
    pub amount: Decimal,
    pub description: String,
    pub category: String,
}
