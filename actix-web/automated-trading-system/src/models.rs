use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Trade {
    pub id: i32,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub side: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTrade {
    pub symbol: String,
    pub quantity: f64,
    pub side: String,
}
