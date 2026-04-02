use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Interaction {
    pub id: i32,
    pub user_query: String,
    pub ai_response: String,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum Intent {
    Schedule,
    Reminder,
    Search,
    Email,
    Other,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInteraction {
    pub user_query: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SuggestionResponse {
    pub intent: Intent,
    pub recommendation: String,
    pub confidence: f64,
}
