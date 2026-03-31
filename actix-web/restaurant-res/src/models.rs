use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct RestaurantTable {
    pub id: i32,
    pub table_number: i32,
    pub capacity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTableRow {
    pub table_number: i32,
    pub capacity: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Reservation {
    pub id: i32,
    pub table_id: i32,
    pub customer_name: String,
    pub reservation_time: DateTime<Utc>,
    pub num_guests: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReservation {
    pub table_id: i32,
    pub customer_name: String,
    pub reservation_time: DateTime<Utc>,
    pub num_guests: i32,
}
