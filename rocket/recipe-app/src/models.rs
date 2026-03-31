use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub instructions: String,
    pub ingredients: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRecipe {
    pub title: String,
    pub instructions: String,
    pub ingredients: serde_json::Value,
}


