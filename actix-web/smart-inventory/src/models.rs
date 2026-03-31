use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub quantity: i32,
    pub price: Decimal,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProduct {
    pub sku: String,
    pub name: String,
    pub quantity: i32,
    pub price: Decimal,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateStock {
    pub amount: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InventoryLog {
    pub id: i32,
    pub product_id: i32,
    pub change_amount: i32,
    pub reason: Option<String>,
    pub recorded_at: DateTime<Utc>,
}
