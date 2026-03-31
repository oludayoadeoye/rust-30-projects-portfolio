use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub total_copies: i32,
    pub available_copies: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub total_copies: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMember {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Loan {
    pub id: i32,
    pub book_id: i32,
    pub member_id: Uuid,
    pub loaned_at: DateTime<Utc>,
    pub due_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoanRequest {
    pub book_id: i32,
    pub member_id: Uuid,
    pub days: i64,
}
