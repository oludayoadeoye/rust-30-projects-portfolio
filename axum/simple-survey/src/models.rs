use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Survey {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSurvey {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Response {
    pub id: i32,
    pub survey_id: i32,
    pub respondent_name: Option<String>,
    pub answers: Value,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateResponse {
    pub respondent_name: Option<String>,
    pub answers: Value,
}
