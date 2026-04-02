use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Poll {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePoll {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Candidate {
    pub id: i32,
    pub poll_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCandidate {
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CastVote {
    pub candidate_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct PollResult {
    pub candidate_name: String,
    pub vote_count: i64,
}
