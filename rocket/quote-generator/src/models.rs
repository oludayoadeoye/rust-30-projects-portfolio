use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Quote {
    pub id: i32,
    pub text: String,
    pub author: String,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateQuote {
    pub text: String,
    pub author: String,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateQuote {
    pub text: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
}


