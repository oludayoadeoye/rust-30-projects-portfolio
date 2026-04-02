use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NlpAnalysis {
    pub id: i32,
    pub input_text: String,
    pub sentiment: String,
    pub language: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AnalyzeRequest {
    pub text: String,
}
