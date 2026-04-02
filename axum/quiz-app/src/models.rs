use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub difficulty: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateQuestion {
    pub question: String,
    pub answer: String,
    pub difficulty: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Attempt {
    pub id: i32,
    pub question_id: i32,
    pub user_answer: String,
    pub is_correct: bool,
    pub attempted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmitAttempt {
    pub user_answer: String,
}
