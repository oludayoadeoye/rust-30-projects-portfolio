use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub total_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrder {
    pub customer_id: Uuid,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
}
