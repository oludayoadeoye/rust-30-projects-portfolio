use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Expense {
    pub id: i32,
    pub category_id: i32,
    pub amount: Decimal,
    pub description: Option<String>,
    pub expense_date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExpense {
    pub category_id: i32,
    pub amount: Decimal,
    pub description: Option<String>,
    pub expense_date: Option<NaiveDate>,
}
