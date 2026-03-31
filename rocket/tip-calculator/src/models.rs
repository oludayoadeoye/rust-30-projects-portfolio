use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BillRecord {
    pub id: i32,
    pub total_bill: f64,
    pub tip_percentage: f64,
    pub num_people: i32,
    pub tip_amount: f64,
    pub total_per_person: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBillRecord {
    pub total_bill: f64,
    pub tip_percentage: f64,
    pub num_people: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BillStats {
    pub total_revenue: f64,
    pub total_tips: f64,
    pub record_count: i64,
}
