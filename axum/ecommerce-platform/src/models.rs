use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProduct {
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub status: String,
    pub total_price: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PlaceOrder {
    pub items: Vec<OrderItemRequest>,
}
